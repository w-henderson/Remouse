use enigo::*;
use multiinput::*;
use std::{convert::TryInto, net::UdpSocket};

pub struct Client {
    input_manager: RawInputManager,
    output_manager: Enigo,
    socket: UdpSocket,
}

/// Initialise the client by connecting the UDP socket to the server.
/// This also registers `multiinput` to listen for mouse events.
pub fn init(ip: String) -> Client {
    let mut manager = RawInputManager::new().unwrap();
    manager.register_devices(DeviceType::Mice);

    let output_manager = Enigo::new();

    let socket = UdpSocket::bind("0.0.0.0:42069").unwrap();
    socket.connect(&ip).unwrap();

    Client {
        input_manager: manager,
        output_manager,
        socket,
    }
}

/// Run the client, detecting mouse events and sending them to the server
pub fn run(client: &mut Client, override_movement: bool) {
    let mut scroll: i8;
    let mut button_flags: i8 = 0;
    let mut button_flags_previous: i8;

    loop {
        let events = client.input_manager.get_events().collect::<Vec<RawEvent>>();

        // Store the previous button flags to know if they changed
        button_flags_previous = button_flags;
        scroll = 0;

        // Update the button flags to match the currently held buttons
        // Also update `scroll` to be 1 if scrolling up, 0 if not scrolling, and -1 if scrolling down
        events.iter().for_each(|e| match e {
            RawEvent::MouseButtonEvent(_, button, state) => {
                let add_or_subtract = match state {
                    State::Pressed => 1_i8,
                    State::Released => -1_i8,
                };

                button_flags = match button {
                    multiinput::MouseButton::Left => button_flags + add_or_subtract,
                    multiinput::MouseButton::Right => button_flags + 2 * add_or_subtract,
                    multiinput::MouseButton::Middle => button_flags + 4 * add_or_subtract,
                    _ => button_flags,
                }
            }
            RawEvent::MouseWheelEvent(_, direction) => {
                scroll = (*direction).clone() as i8;
                scroll = scroll / scroll.abs();
            }
            _ => (),
        });

        button_flags &= 0b00111; // Reset scroll flags
        button_flags = match scroll {
            1 => button_flags | 0b0000_1000, // If scrolling up, set scroll up flag
            -1 => button_flags | 0b0001_0000, // If scrolling down, set scroll down flag
            _ => button_flags,               // If not scrolling, don't set scroll flags
        };

        // If the mouse has moved, send the data and set transmitted to true. Defaults to false
        let transmitted = events
            .iter()
            .find_map(|e| match e {
                RawEvent::MouseMoveEvent(_, x, y) => {
                    if override_movement {
                        Some(override_movement_transmit(client, x, y, button_flags))
                    } else {
                        Some(transmit(client, x, y, button_flags))
                    }
                }
                _ => None,
            })
            .unwrap_or(false);

        // If not already sent the data this iteration and the button states have changed, send it anyway
        if !transmitted && button_flags != button_flags_previous {
            transmit(client, &0, &0, button_flags);
        }
    }
}

/// Override the mouse movement by locking the mouse to the top left of the screen.
/// Ideally, it would hide the mouse and block events from going to the OS completely.
/// If you know a way to do this, please open an issue or PR!
fn override_movement_transmit(client: &mut Client, x: &i32, y: &i32, button_flags: i8) -> bool {
    client.output_manager.mouse_move_to(0, 0);
    transmit(client, x, y, button_flags)
}

/// Transmits the mouse's relative movement as well as the button states
fn transmit(client: &mut Client, x: &i32, y: &i32, button_flags: i8) -> bool {
    let mut to_send = [x.to_le_bytes(), y.to_le_bytes()].concat();
    to_send.push(button_flags.try_into().unwrap());

    client.socket.send(&to_send).unwrap();

    true
}
