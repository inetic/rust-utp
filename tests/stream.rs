#![feature(macro_rules)]

extern crate utp;

use std::io::test::next_test_ip4;
use std::thread::Thread;
use utp::UtpStream;

macro_rules! iotry {
    ($e:expr) => (match $e { Ok(e) => e, Err(e) => panic!("{}", e) })
}


#[test]
fn test_stream_open_and_close() {
    let server_addr = next_test_ip4();
    let mut server = iotry!(UtpStream::bind(server_addr));

    Thread::spawn(move || {
        let mut client = iotry!(UtpStream::connect(server_addr));
        iotry!(client.close());
        drop(client);
    }).detach();

    iotry!(server.read_to_end());
    iotry!(server.close());
}

#[test]
fn test_stream_small_data() {
    // Fits in a packet
    const LEN: uint = 1024;
    let data: Vec<u8> = range(0, LEN).map(|idx| idx as u8).collect();
    assert_eq!(LEN, data.len());

    let d = data.clone();
    let server_addr = next_test_ip4();
    let mut server = UtpStream::bind(server_addr);

    Thread::spawn(move || {
        let mut client = iotry!(UtpStream::connect(server_addr));
        iotry!(client.write(d.as_slice()));
        iotry!(client.close());
    }).detach();

    let read = iotry!(server.read_to_end());
    assert!(!read.is_empty());
    assert_eq!(read.len(), data.len());
    assert_eq!(read, data);
}

#[test]
fn test_stream_large_data() {
    // Has to be sent over several packets
    const LEN: uint = 1024 * 1024;
    let data: Vec<u8> = range(0, LEN).map(|idx| idx as u8).collect();
    assert_eq!(LEN, data.len());

    let d = data.clone();
    let server_addr = next_test_ip4();
    let mut server = UtpStream::bind(server_addr);

    Thread::spawn(move || {
        let mut client = iotry!(UtpStream::connect(server_addr));
        iotry!(client.write(d.as_slice()));
        iotry!(client.close());
    }).detach();

    let read = iotry!(server.read_to_end());
    assert!(!read.is_empty());
    assert_eq!(read.len(), data.len());
    assert_eq!(read, data);
}

#[test]
fn test_stream_successive_reads() {
    use std::io::EndOfFile;

    const LEN: uint = 1024;
    let data: Vec<u8> = range(0, LEN).map(|idx| idx as u8).collect();
    assert_eq!(LEN, data.len());

    let d = data.clone();
    let server_addr = next_test_ip4();
    let mut server = UtpStream::bind(server_addr);

    Thread::spawn(move || {
        let mut client = iotry!(UtpStream::connect(server_addr));
        iotry!(client.write(d.as_slice()));
        iotry!(client.close());
    }).detach();

    iotry!(server.read_to_end());

    let mut buf = [0u8, ..4096];
    match server.read(&mut buf) {
        Err(ref e) if e.kind == EndOfFile => {},
        e => panic!("should have failed with Closed, got {}", e),
    };
}
