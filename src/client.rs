use multiinput::*;
use std::{convert::TryInto, net::UdpSocket};

pub struct Client {
    input_manager: RawInputManager,
    socket: UdpSocket,
}

/// Initialise the client by connecting the UDP socket to the server.
/// This also registers `multiinput` to listen for mouse events.
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

/// Run the client, detecting mouse events and sending them to the server
pub fn run(client: &mut Client) {
    let mut button_flags: i8 = 0;
    let mut button_flags_previous: i8;

    loop {
        let events = client.input_manager.get_events().collect::<Vec<RawEvent>>();

        // Store the previous button flags to know if they changed
        button_flags_previous = button_flags;

        // Update the button flags to match the currently held buttons
        events.iter().for_each(|e| match e {
            RawEvent::MouseButtonEvent(_, button, state) => {
                let add_or_subtract = match state {
                    State::Pressed => 1_i8,
                    State::Released => -1_i8,
                };

                button_flags = match button {
                    MouseButton::Left => button_flags + add_or_subtract,
                    MouseButton::Right => button_flags + 2 * add_or_subtract,
                    MouseButton::Middle => button_flags + 4 * add_or_subtract,
                    _ => button_flags,
                }
            }
            _ => (),
        });

        // If the mouse has moved, send the data and set transmitted to true. Defaults to false
        let transmitted = events
            .iter()
            .find_map(|e| match e {
                RawEvent::MouseMoveEvent(_, x, y) => Some(transmit(client, x, y, button_flags)),
                _ => None,
            })
            .unwrap_or(false);

        // If not already sent the data this iteration and the button states have changed, send it anyway
        if !transmitted && button_flags != button_flags_previous {
            transmit(client, &0, &0, button_flags);
        }
    }
}

/// Transmits the mouse's relative movement as well as the button states
fn transmit(client: &mut Client, x: &i32, y: &i32, button_flags: i8) -> bool {
    let mut to_send = [x.to_le_bytes(), y.to_le_bytes()].concat();
    to_send.push(button_flags.try_into().unwrap());

    client.socket.send(&to_send).unwrap();
    println!("transmitting {:?},{:?},{:?}", x, y, button_flags);

    return true;
}
