/*
 * Copyright (c) 2021 Thomas Duckworth <tduck973564@gmail.com>.
 * This file is under the `rusnet` project, which is licenced under the GNU GPL v3.0 which you can read here: https://www.gnu.org/licenses/gpl-3.0.en.html
 */

//! # Rusnet
//! [![docs](https://docs.rs/rusnet/badge.svg)](https://docs.rs/rusnet)
//! [![dependency status](https://deps.rs/crate/rusnet/0.1.0/status.svg)](https://deps.rs/crate/rusnet/0.1.0)
//! [![build status](https://github.com/tduck973564/rusnet/workflows/Rust/badge.svg)](https://github.com/tduck973564/rusnet/actions)
//! ## An extremely basic network protocol.
//! This network protocol was made by me after I forgot to put a network protocol in my application.
//! ## Examples
//! ```rust
//! use rusnet::*;
//! use std::net::{ TcpListener, TcpStream };
//!
//! fn main() {
//!     let listener = TcpListener::bind(("127.0.0.1", 0)).unwrap();
//!     /* Usually this would be the client,
//!     but it is mocked for the sake of the example */
//!     let mut output = Stream::new(
//!         TcpStream::connect(
//!             listener.local_addr().unwrap()
//!         ).unwrap()
//!     ).unwrap();
//!     for stream in listener.incoming() {
//!         let mut input = Stream::new(stream.unwrap()).unwrap();
//!         input.write("Hello, World!".to_string()).unwrap();
//!         println!("{}", output.read().unwrap()); // This will print "Hello, World!"
//!         break;
//!     }
//! }
//! ```

#![warn(missing_docs)]

use std::error::Error;
use std::time::Duration;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpStream, Shutdown};

#[cfg(test)]
mod tests;

const TIMEOUT_SECS: u64 = 30;

fn wrap(input: String) -> String {
    format!("{}\n{}", input.trim().len(), input)
}

/// The struct containing the read buffer and TcpStream to be read and written to.
pub struct Stream {
    socket: TcpStream,
    read_buffer: BufReader<TcpStream>,
}

impl Stream {
    /// Creates a new Stream. Takes a TcpStream as the argument.
    pub fn new(connection: TcpStream) -> Result<Stream, Box<dyn Error>> {
        connection.set_read_timeout(Some(Duration::from_secs(TIMEOUT_SECS)))?;
        Ok(Stream {
            socket: connection.try_clone()?,
            read_buffer: BufReader::new(connection),
        })
    }
    /// Writes a string to the Stream.
    pub fn write(&mut self, input: String) -> Result<(), Box<dyn Error>> {
        let input = wrap(input.trim().to_string());
        self.socket.write(input.as_ref())?;
        Ok(())
    }
    /// Reads a string from the Stream.
    pub fn read(&mut self) -> Result<String, Box<dyn Error>> {
        // This was painful to write
        loop {
            let mut header_raw = String::new();
            let header_len = self.read_buffer.read_line(&mut header_raw)?;
            let header: usize = header_raw.trim().parse()?;
            let mut contents = vec![0u8; header];
            self.read_buffer.read_exact(&mut contents)?;
            self.read_buffer.consume(header + header_len);
            return Ok(String::from_utf8(contents)?);
        }
    }
    /// Disconnects the TcpStream, and consumes the Stream struct.
    pub fn close(self) -> Result<(), Box<dyn Error>>{
        self.socket.shutdown(Shutdown::Both)?;
        Ok(())
    }
}
