use multiinput::*;
use std::net::UdpSocket;

pub struct Client {
    input_manager: RawInputManager,
    socket: UdpSocket,
}

pub fn init(ip: String) -> Client {
    let mut manager = RawInputManager::new().unwrap();
    manager.register_devices(DeviceType::Mice);

    let socket = UdpSocket::bind("0.0.0.0:42069").unwrap();
    socket.connect(&ip).unwrap();

    Client {
        input_manager: manager,
        socket,
    }
}

pub fn run(client: &mut Client) {
    loop {
        if let Some(event) = client.input_manager.get_event() {
            match event {
                RawEvent::MouseMoveEvent(_, x, y) => transmit(client, x, y),
                _ => continue,
            }
        }
    }
}

fn transmit(client: &mut Client, x: i32, y: i32) {
    let to_send = [x.to_le_bytes(), y.to_le_bytes()].concat();
    client.socket.send(&to_send).unwrap();
}
