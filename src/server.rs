use crate::keyboard::flags_to_pressed_keys;
use enigo::*;
use std::{convert::TryInto, net::UdpSocket};

pub struct Server {
    output_manager: Enigo,
    socket: UdpSocket,
}

/// Initialise the server, starting the UDP socket and returning the server instance.
pub fn init() -> Server {
    let manager = Enigo::new();
    let socket = UdpSocket::bind("0.0.0.0:42069").unwrap();

    Server {
        output_manager: manager,
        socket,
    }
}

/// Run the server, listening for events and acting upon them.
/// Recieves UDP packets of length 13 bytes in the following format:
///
/// - bytes 0-1:  x coordinate of mouse as little endian `i32`
/// - bytes 2-3:  y coordinate of mouse as little endian `i32`
/// - byte 4:     mouse button and connection info flags
/// - bytes 5-13: key flags, mapping described in `keyboard.rs`
///
/// The mouse button and connection info flags are one byte.
/// Their meanings are as follows from the least significant bit:
///
/// - left click
/// - right click
/// - middle click
/// - scroll up
/// - scroll down
/// - disconnect bit (1 triggers a clean disconnect)
/// - connect bit (1 signals that the connection is ready)
///
/// The final bit is currently unused.
pub fn run(server: &mut Server) {
    let mut button_flags: u8 = 0;
    let mut key_flags: u64 = 0;
    let mut previous_button_flags: u8;
    let mut previous_key_flags: u64;

    let mut acknowledged_connection = false;
    let button_types: Vec<MouseButton> =
        vec![MouseButton::Left, MouseButton::Right, MouseButton::Middle];

    loop {
        let mut buf = [0; 13];
        let (_, addr) = server.socket.recv_from(&mut buf).unwrap();

        if !acknowledged_connection {
            let mut connect_message: [u8; 13] = [0; 13];
            connect_message[4] = 0b0100_0000;

            if buf == connect_message {
                println!("receiving input from {}", addr.ip());
                server.socket.send_to(&[0xff_u8], addr).unwrap();
                acknowledged_connection = true;
            } else {
                println!("received malformed request from {}, ignoring", addr.ip());
                server.socket.send_to(&[0x00_u8], addr).unwrap();
            };

            continue;
        }

        let x = i16::from_le_bytes(buf[0..2].try_into().unwrap());
        let y = i16::from_le_bytes(buf[2..4].try_into().unwrap());
        previous_button_flags = button_flags;
        previous_key_flags = key_flags;
        button_flags = buf[4];
        key_flags = u64::from_le_bytes(buf[5..13].try_into().unwrap());

        // Check the button flags and update the mouse state accordingly
        let button_states = button_flags.to_bools();
        let previous_button_states = previous_button_flags.to_bools();
        for button_id in 0..button_types.len() {
            if button_states[button_id] ^ previous_button_states[button_id] {
                match button_states[button_id] {
                    true => server.output_manager.mouse_down(button_types[button_id]),
                    false => server.output_manager.mouse_up(button_types[button_id]),
                }
            }
        }

        // Scroll if the user is scrolling
        let scroll_distance = button_states[3] as i32 - button_states[4] as i32;
        if scroll_distance != 0 {
            server.output_manager.mouse_scroll_y(scroll_distance);
        }

        // Check the key flags and update the keys held accordingly
        let key_states = key_flags.to_bools();
        let previous_key_states = previous_key_flags.to_bools();
        let keys_held = flags_to_pressed_keys(&key_states);
        let previous_keys_held = flags_to_pressed_keys(&previous_key_states);
        for key in &previous_keys_held {
            if !keys_held.contains(key) {
                server.output_manager.key_up(*key);
            }
        }
        for key in &keys_held {
            if !previous_keys_held.contains(key) {
                server.output_manager.key_down(*key);
            }
        }

        if button_states[5] == true {
            println!("connection to {} closed", addr.ip());
            acknowledged_connection = false;
        }

        server
            .output_manager
            .mouse_move_relative(x as i32, y as i32);
    }
}

trait Flags {
    fn to_bools(&self) -> Vec<bool>;
}

impl Flags for u8 {
    /// Convert the `u8` into a `Vec<bool>` for each bit.
    /// The least significant bit becomes the start of the array.
    fn to_bools(&self) -> Vec<bool> {
        let mut out: Vec<bool> = Vec::new();
        for bit in 0..8 {
            out.push(self & (1 << bit) != 0);
        }

        return out;
    }
}

impl Flags for u64 {
    /// Convert the `u64` into a `Vec<bool>` for each bit.
    /// The least significant bit becomes the start of the array.
    fn to_bools(&self) -> Vec<bool> {
        let mut out: Vec<bool> = Vec::new();
        for bit in 0..64 {
            out.push(self & (1 << bit) != 0);
        }

        return out;
    }
}
