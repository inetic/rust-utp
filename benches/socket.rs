#![feature(test)]

extern crate test;
extern crate utp;

use test::Bencher;
use utp::UtpSocket;
use std::sync::Arc;
use std::thread;

macro_rules! iotry {
    ($e:expr) => (match $e { Ok(e) => e, Err(e) => panic!("{}", e) })
}

fn next_test_port() -> u16 {
    use std::sync::atomic::{AtomicUsize, ATOMIC_USIZE_INIT, Ordering};
    static NEXT_OFFSET: AtomicUsize = ATOMIC_USIZE_INIT;
    const BASE_PORT: u16 = 9600;
    BASE_PORT + NEXT_OFFSET.fetch_add(1, Ordering::Relaxed) as u16
}

fn next_test_ip4<'a>() -> (&'a str, u16) {
    ("127.0.0.1", next_test_port())
}

#[bench]
fn bench_connection_setup_and_teardown(b: &mut Bencher) {
    let server_addr = next_test_ip4();
    let mut buf = [0; 1500];

    b.iter(|| {
        let mut server = iotry!(UtpSocket::bind(server_addr));

        thread::spawn(move || {
            let mut client = iotry!(UtpSocket::connect(server_addr));
            iotry!(client.close());
        });

        loop {
            match server.recv_from(&mut buf) {
                Ok((0, _src)) => break,
                Ok(_) => (),
                Err(e) => panic!("{}", e)
            }
        }
        iotry!(server.close());
    });
}

#[bench]
fn bench_transfer_one_packet(b: &mut Bencher) {
    let len = 1024;
    let server_addr = next_test_ip4();
    let mut buf = [0; 1500];
    let data = (0..len).map(|x| x as u8).collect::<Vec<u8>>();
    let data_arc = Arc::new(data);

    b.iter(|| {
        let data = data_arc.clone();
        let mut server = iotry!(UtpSocket::bind(server_addr));

        thread::spawn(move || {
            let mut client = iotry!(UtpSocket::connect(server_addr));
            iotry!(client.send_to(&data[..]));
            iotry!(client.close());
        });

        loop {
            match server.recv_from(&mut buf) {
                Ok((0, _src)) => break,
                Ok(_) => (),
                Err(e) => panic!("{}", e)
            }
        }
        iotry!(server.close());
    });
    b.bytes = len as u64;
}

#[bench]
fn bench_transfer_one_megabyte(b: &mut Bencher) {
    let len = 1024 * 1024;
    let server_addr = next_test_ip4();
    let mut buf = [0; 1500];
    let data = (0..len).map(|x| x as u8).collect::<Vec<u8>>();
    let data_arc = Arc::new(data);

    b.iter(|| {
        let data = data_arc.clone();
        let mut server = iotry!(UtpSocket::bind(server_addr));

        thread::spawn(move || {
            let mut client = iotry!(UtpSocket::connect(server_addr));
            iotry!(client.send_to(&data[..]));
            iotry!(client.close());
        });

        loop {
            match server.recv_from(&mut buf) {
                Ok((0, _src)) => break,
                Ok(_) => (),
                Err(e) => panic!("{}", e)
            }
        }
        iotry!(server.close());
    });
    b.bytes = len as u64;
}
