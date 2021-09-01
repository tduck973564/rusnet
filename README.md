# Rusnet
[![docs](https://docs.rs/rusnet/badge.svg)](https://docs.rs/rusnet)
[![dependency status](https://deps.rs/crate/rusnet/0.1.0/status.svg)](https://deps.rs/crate/rusnet/0.1.0)
[![build status](https://github.com/tduck973564/rusnet/workflows/Rust/badge.svg)](https://github.com/tduck973564/rusnet/actions)
## An extremely basic network protocol.
This network protocol was made by me after I forgot to put a network protocol in my application.
## Examples
```rust
use rusnet::*;
use std::net::{ TcpListener, TcpStream };

fn main() {
    let listener = TcpListener::bind(("127.0.0.1", 0)).unwrap();
    /* Usually this would be the client,
    but it is mocked for the sake of the example */
    let mut output = Stream::new(
        TcpStream::connect(
            listener.local_addr().unwrap()
        ).unwrap()
    ).unwrap();
    for stream in listener.incoming() {
        let mut input = Stream::new(stream.unwrap()).unwrap();
        input.write("Hello, World!".to_string()).unwrap();
        println!("{}", output.read().unwrap()); // This will print "Hello, World!"
        break;
    }
}
```