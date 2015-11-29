extern crate rusttp;
extern crate nom;
// use std::*;
use nom::IResult;
fn parse(data:&[u8]) {
    let mut buf = &data[..];
    let mut i   = 0;
    loop {
        match rusttp::parse_request(buf) {
          IResult::Done(b, _) => {
              buf = b;

              i = i + 1;

              if b.is_empty() {
          
          //println!("{}", i);
                  break;
              }
          },
          IResult::Error(_) => return /*panic!("{:?}", e)*/,
          IResult::Incomplete(_) => return /*panic!("Incomplete!")*/,
        }
    }
}

fn main() {
    let data = include_bytes!("../bigger.txt");
    // for _ in 1..100000 {
       parse(data);
    // }
}