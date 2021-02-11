use crate::keyboard::pressed_keys_to_flags;
use enigo::*;
use minifb::{CursorStyle, Window, WindowOptions};
use multiinput::*;
use std::{
    convert::{TryFrom, TryInto},
    net::UdpSocket,
    process::exit,
    thread::sleep,
    time::{Duration, SystemTime},
};

/// Represents a client and holds objects relating to it.
pub struct Client {
    window: Window,
    input_manager: RawInputManager,
    output_manager: Enigo,
    socket: UdpSocket,
}

/// Initialise the client by connecting the UDP socket to the server.
/// This also registers `multiinput` to listen for mouse events.
pub fn init(ip: String) -> Client {
    let mut input_manager = RawInputManager::new().unwrap();
    input_manager.register_devices(DeviceType::Mice);

    let output_manager = Enigo::new();

    let mut window = Window::new("Remouse", 100, 100, WindowOptions::default()).unwrap();
    window.update();
    window.set_position(-50, -50);
    window.set_cursor_style(CursorStyle::Arrow);

    let socket = UdpSocket::bind("0.0.0.0:42069").unwrap();
    socket.connect(&ip).unwrap();

    Client {
        window,
        input_manager,
        output_manager,
        socket,
    }
}

/// Run the client, detecting mouse events and sending them to the server
pub fn run(client: &mut Client, override_movement: bool) {
    let mut scroll: i8;
    let mut button_flags: i8 = 0;
    let mut last_movement_time: SystemTime = SystemTime::now();
    //let mut button_flags_previous: i8;

    loop {
        let events = client.input_manager.get_events().collect::<Vec<RawEvent>>();

        // Store the previous button flags to know if they changed
        //button_flags_previous = button_flags;
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

        // Update the key flags according to which keys have been pressed
        let pressed_keys = client.window.get_keys().unwrap();
        let key_flags_option = pressed_keys_to_flags(&pressed_keys);

        let key_flags: u64 = if key_flags_option == None {
            exit(0);
        } else {
            key_flags_option.unwrap()
        };

        // If the mouse has moved, send the data and set transmitted to true. Defaults to false
        let transmitted = events
            .iter()
            .find_map(|e| match e {
                RawEvent::MouseMoveEvent(_, x, y) => {
                    last_movement_time = SystemTime::now();

                    if override_movement {
                        Some(override_movement_transmit(
                            client,
                            x,
                            y,
                            button_flags,
                            key_flags,
                        ))
                    } else {
                        Some(transmit(client, x, y, button_flags, key_flags))
                    }
                }
                _ => None,
            })
            .unwrap_or(false);

        // If not already sent the data this iteration, send it anyway with no movement
        // Don't do this if the mouse has recently moved to fix issue #1
        if !transmitted {
            if last_movement_time.elapsed().unwrap().as_millis() > 50 {
                transmit(client, &0, &0, button_flags, key_flags);
                sleep(Duration::from_millis(1));
            }
        }

        client.window.update();
    }
}

/// Override the mouse movement by locking the mouse to the top left of the screen.
/// This is inside the key capture box so keys are always captured.
fn override_movement_transmit(
    client: &mut Client,
    x: &i32,
    y: &i32,
    button_flags: i8,
    key_flags: u64,
) -> bool {
    client.output_manager.mouse_move_to(0, 0);
    transmit(client, x, y, button_flags, key_flags)
}

/// Transmits the mouse's relative movement as well as the button states
fn transmit(client: &mut Client, x: &i32, y: &i32, button_flags: i8, key_flags: u64) -> bool {
    let x_le_i16 = i16::try_from(*x).unwrap().to_le_bytes();
    let y_le_i16 = i16::try_from(*y).unwrap().to_le_bytes();

    let mut to_send = [x_le_i16, y_le_i16].concat();
    to_send.push(button_flags.try_into().unwrap());
    to_send.extend_from_slice(&key_flags.to_le_bytes());

    client.socket.send(&to_send).unwrap();

    true
}
