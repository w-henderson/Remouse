/// Convert a `Vec<Key>` into i64 flags.
/// Mapping is as follows:
///
/// ```
/// [A-Z]                    [0..25]
/// [0-9]                    [26..35]
/// [Space]                  [36]
/// [Return]                 [37]
/// [Shift]                  [38]
/// [Ctrl]                   [39]
/// [Down, Left, Right, Up]  [40..43]
/// [Apostrophe]             [44]
/// [Backquote]              [45]
/// [Backslash]              [46]
/// [Comma]                  [47]
/// [Equal]                  [48]
/// [LeftBracket]            [49]
/// [Minus]                  [50]
/// [Period]                 [51]
/// [RightBracket]           [52]
/// [Semicolon]              [53]
/// [Slash]                  [54]
/// [Backspace]              [55]
/// [Delete]                 [56]
/// [End]                    [57]
/// [Home]                   [58]
/// [PageDown]               [59]
/// [PageUp]                 [60]
/// [Tab]                    [61]
/// [CapsLock]               [62]
/// [Alt]                    [63]
/// ```
pub fn pressed_keys_to_flags(pressed_keys: &Vec<minifb::Key>) -> Option<u64> {
    let mut flags: u64 = 0;
    for key in pressed_keys {
        flags = flags
            + match key {
                minifb::Key::A => 1 << 0,
                minifb::Key::B => 1 << 1,
                minifb::Key::C => 1 << 2,
                minifb::Key::D => 1 << 3,
                minifb::Key::E => 1 << 4,
                minifb::Key::F => 1 << 5,
                minifb::Key::G => 1 << 6,
                minifb::Key::H => 1 << 7,
                minifb::Key::I => 1 << 8,
                minifb::Key::J => 1 << 9,
                minifb::Key::K => 1 << 10,
                minifb::Key::L => 1 << 11,
                minifb::Key::M => 1 << 12,
                minifb::Key::N => 1 << 13,
                minifb::Key::O => 1 << 14,
                minifb::Key::P => 1 << 15,
                minifb::Key::Q => 1 << 16,
                minifb::Key::R => 1 << 17,
                minifb::Key::S => 1 << 18,
                minifb::Key::T => 1 << 19,
                minifb::Key::U => 1 << 20,
                minifb::Key::V => 1 << 21,
                minifb::Key::W => 1 << 22,
                minifb::Key::X => 1 << 23,
                minifb::Key::Y => 1 << 24,
                minifb::Key::Z => 1 << 25,
                minifb::Key::Key0 => 1 << 26,
                minifb::Key::Key1 => 1 << 27,
                minifb::Key::Key2 => 1 << 28,
                minifb::Key::Key3 => 1 << 29,
                minifb::Key::Key4 => 1 << 30,
                minifb::Key::Key5 => 1 << 31,
                minifb::Key::Key6 => 1 << 32,
                minifb::Key::Key7 => 1 << 33,
                minifb::Key::Key8 => 1 << 34,
                minifb::Key::Key9 => 1 << 35,
                minifb::Key::Space => 1 << 36,
                minifb::Key::Enter => 1 << 37,
                minifb::Key::LeftShift => 1 << 38,
                minifb::Key::RightShift => 1 << 38,
                minifb::Key::LeftCtrl => 1 << 39,
                minifb::Key::RightCtrl => 1 << 39,
                minifb::Key::Down => 1 << 40,
                minifb::Key::Left => 1 << 41,
                minifb::Key::Right => 1 << 42,
                minifb::Key::Up => 1 << 43,
                minifb::Key::Apostrophe => 1 << 44,
                minifb::Key::Backquote => 1 << 45,
                minifb::Key::Backslash => 1 << 46,
                minifb::Key::Comma => 1 << 47,
                minifb::Key::Equal => 1 << 48,
                minifb::Key::LeftBracket => 1 << 49,
                minifb::Key::Minus => 1 << 50,
                minifb::Key::Period => 1 << 51,
                minifb::Key::RightBracket => 1 << 52,
                minifb::Key::Semicolon => 1 << 53,
                minifb::Key::Slash => 1 << 54,
                minifb::Key::Backspace => 1 << 55,
                minifb::Key::Delete => 1 << 56,
                minifb::Key::End => 1 << 57,
                minifb::Key::Home => 1 << 58,
                minifb::Key::PageDown => 1 << 59,
                minifb::Key::PageUp => 1 << 60,
                minifb::Key::Tab => 1 << 61,
                minifb::Key::CapsLock => 1 << 62,
                minifb::Key::LeftAlt => 1 << 63,
                minifb::Key::RightAlt => 1 << 63,
                minifb::Key::Escape => return None,
                _ => 0,
            };
    }

    Some(flags)
}

/// Convert a `Vec<bool>` into a `Vec<enigo::Key>` with every key currently being held.
/// Mappings are documented on the reverse, `pressed_keys_to_flags()`.
pub fn flags_to_pressed_keys(flags: &Vec<bool>) -> Vec<enigo::Key> {
    let mut held_keys: Vec<enigo::Key> = Vec::new();
    for i in 0..flags.len() {
        if flags[i] {
            held_keys.push(match i {
                0 => enigo::Key::Layout('a'),
                1 => enigo::Key::Layout('b'),
                2 => enigo::Key::Layout('c'),
                3 => enigo::Key::Layout('d'),
                4 => enigo::Key::Layout('e'),
                5 => enigo::Key::Layout('f'),
                6 => enigo::Key::Layout('g'),
                7 => enigo::Key::Layout('h'),
                8 => enigo::Key::Layout('i'),
                9 => enigo::Key::Layout('j'),
                10 => enigo::Key::Layout('k'),
                11 => enigo::Key::Layout('l'),
                12 => enigo::Key::Layout('m'),
                13 => enigo::Key::Layout('n'),
                14 => enigo::Key::Layout('o'),
                15 => enigo::Key::Layout('p'),
                16 => enigo::Key::Layout('q'),
                17 => enigo::Key::Layout('r'),
                18 => enigo::Key::Layout('s'),
                19 => enigo::Key::Layout('t'),
                20 => enigo::Key::Layout('u'),
                21 => enigo::Key::Layout('v'),
                22 => enigo::Key::Layout('w'),
                23 => enigo::Key::Layout('x'),
                24 => enigo::Key::Layout('y'),
                25 => enigo::Key::Layout('z'),
                26 => enigo::Key::Layout('0'),
                27 => enigo::Key::Layout('1'),
                28 => enigo::Key::Layout('2'),
                29 => enigo::Key::Layout('3'),
                30 => enigo::Key::Layout('4'),
                31 => enigo::Key::Layout('5'),
                32 => enigo::Key::Layout('6'),
                33 => enigo::Key::Layout('7'),
                34 => enigo::Key::Layout('8'),
                35 => enigo::Key::Layout('9'),
                36 => enigo::Key::Layout(' '),
                37 => enigo::Key::Return,
                38 => enigo::Key::Shift,
                39 => enigo::Key::Control,
                40 => enigo::Key::DownArrow,
                41 => enigo::Key::LeftArrow,
                42 => enigo::Key::RightArrow,
                43 => enigo::Key::UpArrow,
                44 => enigo::Key::Layout('\''),
                45 => enigo::Key::Layout('`'),
                46 => enigo::Key::Layout('\\'),
                47 => enigo::Key::Layout(','),
                48 => enigo::Key::Layout('='),
                49 => enigo::Key::Layout('['),
                50 => enigo::Key::Layout('-'),
                51 => enigo::Key::Layout('.'),
                52 => enigo::Key::Layout(']'),
                53 => enigo::Key::Layout(';'),
                54 => enigo::Key::Layout('/'),
                55 => enigo::Key::Backspace,
                56 => enigo::Key::Delete,
                57 => enigo::Key::End,
                58 => enigo::Key::Home,
                59 => enigo::Key::PageDown,
                60 => enigo::Key::PageUp,
                61 => enigo::Key::Tab,
                62 => enigo::Key::CapsLock,
                63 => enigo::Key::Alt,
                _ => enigo::Key::Escape, // has to be here because usize, shouldn't ever be reached
            });
        };
    }

    held_keys
}
