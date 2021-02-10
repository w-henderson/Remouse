use enigo::*;
use std::{convert::TryInto, net::UdpSocket};

pub struct Server {
    output_manager: Enigo,
    socket: UdpSocket,
}

pub fn init() -> Server {
    let manager = Enigo::new();

    let socket = UdpSocket::bind("0.0.0.0:42069").unwrap();

    Server {
        output_manager: manager,
        socket,
    }
}

pub fn run(server: &mut Server) {
    loop {
        let mut buf = [0; 8];
        server.socket.recv_from(&mut buf).unwrap();

        let x = i32::from_le_bytes(buf[0..4].try_into().unwrap());
        let y = i32::from_le_bytes(buf[4..8].try_into().unwrap());

        server.output_manager.mouse_move_relative(x, y);
    }
}
