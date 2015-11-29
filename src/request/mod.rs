// use nom::IResult;

#[cfg(test)]
mod tests {
    use nom::IResult;

    #[test]
    fn test_parse_method() {
        match super::request_line(b"GET /hello.htm HTTP/1.1\r\n") {
            IResult::Done(_, request) => {
                assert_eq!(b"GET", request.method);
                assert_eq!(b"/hello.htm", request.uri);
                assert_eq!(b"1.1", request.version);
            },
            IResult::Incomplete(i) => panic!(format!("Incomplete: {:?}", i)),
            _ => panic!("Error while parsing request line")
        }
    }

    #[test]
    fn test_parse_headers() {
        match super::message_header(b"Host: www.tutorialspoint.com\r\n") {
            IResult::Done(_, header) => {
                assert_eq!(b"Host", header.name);
                assert_eq!(b"www.tutorialspoint.com", header.value[0]);
            },
            IResult::Incomplete(i) => panic!(format!("Incomplete: {:?}", i)),
            _ => panic!("Error while parsing Header")
        };
    }

    #[test]
    fn test_parse_request(){
        let req = b"GET /hello.htm HTTP/1.1
User-Agent: Mozilla/4.0 (compatible; MSIE5.01; Windows NT)
Host: www.tutorialspoint.com
Accept-Language: en-us
Accept-Encoding: gzip, deflate
Connection: Keep-Alive\r\n\r\n";

        match super::request(req) {
            IResult::Done(_, (request, headers) ) => {
                assert_eq!(b"GET", request.method);
                assert_eq!(b"/hello.htm", request.uri);
                assert_eq!(b"1.1", request.version);

                let header = &headers[0];
                assert_eq!(header.name, b"User-Agent");
                assert_eq!("Mozilla/4.0 (compatible; MSIE5.01; Windows NT)".as_bytes(), header.value[0]);
            },
            IResult::Incomplete(i) => panic!(format!("Incomplete: {:?}", i)),
            _ => panic!("Error while parsing request line")
        }
    }

}

#[derive(Debug)]
pub struct Request<'a> {
    pub method:  &'a [u8],
    pub uri:     &'a [u8],
    pub version: &'a [u8],
}

#[derive(Debug)]
pub struct Header<'a> {
    pub name:  &'a [u8],
    pub value: Vec<&'a [u8]>,
}

fn is_token(c: u8) -> bool {
    // roughly follows the order of ascii chars: "\"(),/:;<=>?@[\\]{} \t"
    c < 128 && c > 32 && c != b'\t' && c != b'"' && c != b'(' && c != b')' &&
        c != b',' && c != b'/' && !(c > 57 && c < 65) && !(c > 90 && c < 94) &&
        c != b'{' && c != b'}'
}

fn not_line_ending(c: u8) -> bool {
    c != b'\r' && c != b'\n'
}

fn is_space(c: u8) -> bool {
    c == b' '
}

fn is_not_space(c: u8)        -> bool { c != b' ' }

fn is_horizontal_space(c: u8) -> bool { c == b' ' || c == b'\t' }

fn is_version(c: u8) -> bool {
    c >= b'0' && c <= b'9' || c == b'.'
}

named!(line_ending, alt!(tag!("\n") | tag!("\r\n")));

named!(request_line(&[u8]) -> Request, chain!(
    method: take_while1!(is_token)     ~
            take_while1!(is_space)     ~
    url:    take_while1!(is_not_space) ~
            take_while1!(is_space)     ~
    version: http_version              ~
    line_ending,
    
    || Request {
        method: method,
        uri:    url,
        version: version,
}));

named!(http_version, chain!(
    tag!("HTTP/")                    ~
    version: take_while1!(is_version),
    
    || version));

named!(message_header_value, chain!(
          take_while1!(is_horizontal_space) ~
    data: take_while1!(not_line_ending)     ~
    line_ending,
    
    || data));

named!(message_header(&[u8]) -> Header, chain!(
    name:   take_while1!(is_token) ~
            char!(':') ~
    values: many1!(message_header_value),
    
    || Header {
        name: name,
        value: values,
    }));

named!(pub request(&[u8]) -> (Request, Vec<Header>), chain!(
    req: request_line           ~
    h:   many1!(message_header) ~
         line_ending,
    
    || (req, h)));