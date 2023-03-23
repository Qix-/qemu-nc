use etherparse::PacketHeaders;
use std::{
    io::{ErrorKind, Read},
    net::{TcpListener, TcpStream},
};

fn read_single_packet(stream: &mut TcpStream) -> std::io::Result<()> {
    let mut len_buf = [0u8; 4];
    stream.read_exact(&mut len_buf)?;

    let len: u64 = ((len_buf[0] as u64) << 24)
        | ((len_buf[1] as u64) << 16)
        | ((len_buf[2] as u64) << 8)
        | (len_buf[3] as u64);

    eprintln!("len: {}", len);

    let mut bytes = vec![0; len as usize];
    stream.read_exact(&mut bytes)?;

    let packet_headers = PacketHeaders::from_ethernet_slice(&bytes[..]).unwrap();
    println!("{:?}", packet_headers);

    Ok(())
}

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    loop {
        if let Err(err) = read_single_packet(&mut stream) {
            match err.kind() {
                ErrorKind::UnexpectedEof => {
                    return Ok(());
                }
                _ => {
                    return Err(err);
                }
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0")?;

    eprintln!(
        "listening on port {}",
        listener.local_addr().unwrap().port()
    );

    for stream in listener.incoming() {
        handle_client(stream?)?;
    }

    Ok(())
}
