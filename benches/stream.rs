#![feature(old_io, test)]

extern crate test;
extern crate utp;

use test::Bencher;
use utp::UtpStream;
use std::io::{Read, Write};
use std::sync::Arc;
use std::thread;

macro_rules! iotry {
    ($e:expr) => (match $e { Ok(e) => e, Err(e) => panic!("{}", e) })
}

fn next_test_ip4<'a>() -> (&'a str, u16) {
    use std::old_io::test::next_test_port;
    ("127.0.0.1", next_test_port())
}

#[bench]
fn bench_connection_setup_and_teardown(b: &mut Bencher) {
    let server_addr = next_test_ip4();
    let mut received = vec!();
    b.iter(|| {
        let mut server = iotry!(UtpStream::bind(server_addr));

        thread::spawn(move || {
            let mut client = iotry!(UtpStream::connect(server_addr));
            iotry!(client.close());
        });

        iotry!(server.read_to_end(&mut received));
        iotry!(server.close());
    });
}

#[bench]
fn bench_transfer_one_packet(b: &mut Bencher) {
    let len = 1024;
    let server_addr = next_test_ip4();
    let data = (0..len).map(|x| x as u8).collect::<Vec<u8>>();
    let data_arc = Arc::new(data);
    let mut received = vec!();

    b.iter(|| {
        let data = data_arc.clone();
        let mut server = iotry!(UtpStream::bind(server_addr));

        thread::spawn(move || {
            let mut client = iotry!(UtpStream::connect(server_addr));
            iotry!(client.write(&data[..]));
            iotry!(client.close());
        });

        iotry!(server.read_to_end(&mut received));
        iotry!(server.close());
    });
    b.bytes = len as u64;
}

#[bench]
fn bench_transfer_one_megabyte(b: &mut Bencher) {
    let len = 1024 * 1024;
    let server_addr = next_test_ip4();
    let data = (0..len).map(|x| x as u8).collect::<Vec<u8>>();
    let data_arc = Arc::new(data);
    let mut received = vec!();

    b.iter(|| {
        let data = data_arc.clone();
        let mut server = iotry!(UtpStream::bind(server_addr));

        thread::spawn(move || {
            let mut client = iotry!(UtpStream::connect(server_addr));
            iotry!(client.write(&data[..]));
            iotry!(client.close());
        });

        iotry!(server.read_to_end(&mut received));
        iotry!(server.close());
    });
    b.bytes = len as u64;
}
