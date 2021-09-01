/*
 * Copyright (c) 2021 Thomas Duckworth <tduck973564@gmail.com>.
 * This file is under the `rusnet` project, which is licenced under the GNU GPL v3.0 which you can read here: https://www.gnu.org/licenses/gpl-3.0.en.html
 */

use crate::*;
use std::net;

#[test]
fn read_and_write() {
    let listener = net::TcpListener::bind(("127.0.0.1", 0)).unwrap();
    let mut output = Stream::new(net::TcpStream::connect(listener.local_addr().unwrap()).unwrap()).unwrap();
    for stream in listener.incoming() {
        let mut input = Stream::new(stream.unwrap()).unwrap();

        input.write("tfw".to_string()).unwrap();
        assert_eq!("tfw".to_string(), output.read().unwrap());

        // it strips whitespace because i am bad at coding
        input.write("when the\n\n\n\n".to_string()).unwrap();
        assert_ne!("when the\n\n\n\n", output.read().unwrap());

        input.write("aaaaaaaaaa".to_string()).unwrap();
        assert_eq!("aaaaaaaaaa".to_string(), output.read().unwrap());

        input.write("".to_string()).unwrap();
        assert_eq!("".to_string(), output.read().unwrap());
        break;
    }
}