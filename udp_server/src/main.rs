use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};
use std::io::Result;
use std::str;

const IP: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
const PORT: u16  = 12000;


fn main() -> Result<()> {
    let server_addr = SocketAddrV4::new(IP, PORT);
    let server_socket = UdpSocket::bind(server_addr)?;

    let mut buffer = [0; 2048];

    println!("Server listening on address: {:?}", server_addr);

    loop {
        let (bytes, client_addr) = server_socket.recv_from(&mut buffer)?;
        println!("Message received from: {:?}", client_addr);

        match str::from_utf8(&buffer[..bytes]) {
            Ok(message) => {
                println!("Message: {message}");
                let modified_message = String::from(message).to_uppercase();
                server_socket.send_to(modified_message.as_bytes(), client_addr)?;
            },
            Err(_) => {}
        }
    }
}
