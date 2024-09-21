use std::net::{UdpSocket, Ipv4Addr, SocketAddrV4};
use std::{io, io::Write};
use std::str;

fn main() -> io::Result<()> {
    let server_ip = Ipv4Addr::new(127,0 ,0, 1);
    let server_port = 12000;
    let server_addr = SocketAddrV4::new(server_ip, server_port);

    let client_socket = UdpSocket::bind("127.0.0.1:8081")?;
    let mut buffer: [u8; 2048] = [0; 2048];

    loop {
        let message = get_input("Input lowercase sentence: ")?;

        if message.eq("exit") {
            return Ok(());
        }

        client_socket.send_to(message.as_bytes(), server_addr)?;

        let (bytes, _) = client_socket.recv_from(&mut buffer)?;

        match str::from_utf8(&buffer[..bytes]) {
            Ok(message) => println!("Modified message: {message}"),
            Err(_) => {}
        }
    }
}

fn get_input(console_message: &str) -> io::Result<String> {
    let mut input = String::new();

    print!("{console_message}");

    let _= io::stdout().flush();
    io::stdin().read_line(&mut input)?;

    if let Some('\n') = input.chars().next_back() {
        input.pop();
    }

    Ok(input)
}
