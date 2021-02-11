use minifb::Key;

/// Convert a `Vec<Key>` into i64 flags.
/// Mapping is as follows:
///
/// ```
/// [A-Z]:                   [0..25]
/// [0-9]:                   [26..35]
/// [Space]:                 [36]
/// [Return]:                [37]
/// [Shift]:                 [38]
/// [Ctrl]:                  [39]
/// [Down, Left, Right, Up]: [40..43]
/// [Apostrophe]:            [44]
/// [Backquote]:             [45]
/// [Backslash]:             [46]
/// [Comma]:                 [47]
/// [Equal]:                 [48]
/// [LeftBracket]:           [49]
/// [Minus]:                 [50]
/// [Period]:                [51]
/// [RightBracket]:          [52]
/// [Semicolon]:             [53]
/// [Slash]:                 [54]
/// [Backspace]:             [55]
/// [Delete]:                [56]
/// [End]:                   [57]
/// [Home]:                  [58]
/// [PageDown]:              [59]
/// [PageUp]:                [60]
/// [Tab]:                   [61]
/// [CapsLock]:              [62]
/// [Alt]:                   [63]
/// ```
pub fn pressed_keys_to_flags(pressed_keys: &Vec<Key>) -> Option<u64> {
    let mut flags: u64 = 0;
    for key in pressed_keys {
        flags = flags
            + match key {
                Key::A => 1 << 0,
                Key::B => 1 << 1,
                Key::C => 1 << 2,
                Key::D => 1 << 3,
                Key::E => 1 << 4,
                Key::F => 1 << 5,
                Key::G => 1 << 6,
                Key::H => 1 << 7,
                Key::I => 1 << 8,
                Key::J => 1 << 9,
                Key::K => 1 << 10,
                Key::L => 1 << 11,
                Key::M => 1 << 12,
                Key::N => 1 << 13,
                Key::O => 1 << 14,
                Key::P => 1 << 15,
                Key::Q => 1 << 16,
                Key::R => 1 << 17,
                Key::S => 1 << 18,
                Key::T => 1 << 19,
                Key::U => 1 << 20,
                Key::V => 1 << 21,
                Key::W => 1 << 22,
                Key::X => 1 << 23,
                Key::Y => 1 << 24,
                Key::Z => 1 << 25,
                Key::Key0 => 1 << 26,
                Key::Key1 => 1 << 27,
                Key::Key2 => 1 << 28,
                Key::Key3 => 1 << 29,
                Key::Key4 => 1 << 30,
                Key::Key5 => 1 << 31,
                Key::Key6 => 1 << 32,
                Key::Key7 => 1 << 33,
                Key::Key8 => 1 << 34,
                Key::Key9 => 1 << 35,
                Key::Space => 1 << 36,
                Key::Enter => 1 << 37,
                Key::LeftShift => 1 << 38,
                Key::RightShift => 1 << 38,
                Key::LeftCtrl => 1 << 39,
                Key::RightCtrl => 1 << 39,
                Key::Down => 1 << 40,
                Key::Left => 1 << 41,
                Key::Right => 1 << 42,
                Key::Up => 1 << 43,
                Key::Apostrophe => 1 << 44,
                Key::Backquote => 1 << 45,
                Key::Backslash => 1 << 46,
                Key::Comma => 1 << 47,
                Key::Equal => 1 << 48,
                Key::LeftBracket => 1 << 49,
                Key::Minus => 1 << 50,
                Key::Period => 1 << 51,
                Key::RightBracket => 1 << 52,
                Key::Semicolon => 1 << 53,
                Key::Slash => 1 << 54,
                Key::Backspace => 1 << 55,
                Key::Delete => 1 << 56,
                Key::End => 1 << 57,
                Key::Home => 1 << 58,
                Key::PageDown => 1 << 59,
                Key::PageUp => 1 << 60,
                Key::Tab => 1 << 61,
                Key::CapsLock => 1 << 62,
                Key::LeftAlt => 1 << 63,
                Key::RightAlt => 1 << 63,
                Key::Escape => return None,
                _ => 0,
            };
    }

    Some(flags)
}
