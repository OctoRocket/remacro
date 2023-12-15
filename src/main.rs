use anyhow::Result;
use evdev::{uinput::VirtualDeviceBuilder, Key};
use thiserror::Error;

#[derive(Debug, Error)]
enum ServerError {
    #[error("Unable to parse character {0}.")]
    Parse(char),
}

fn to_keys(c: char) -> Result<(Key, bool)> {
    match c {
        'a' => Ok((Key::KEY_A, false)),
        'b' => Ok((Key::KEY_B, false)),
        'c' => Ok((Key::KEY_C, false)),
        'd' => Ok((Key::KEY_D, false)),
        'e' => Ok((Key::KEY_E, false)),
        'f' => Ok((Key::KEY_F, false)),
        'g' => Ok((Key::KEY_G, false)),
        'h' => Ok((Key::KEY_H, false)),
        'i' => Ok((Key::KEY_I, false)),
        'j' => Ok((Key::KEY_J, false)),
        'k' => Ok((Key::KEY_K, false)),
        'l' => Ok((Key::KEY_L, false)),
        'm' => Ok((Key::KEY_M, false)),
        'n' => Ok((Key::KEY_N, false)),
        'o' => Ok((Key::KEY_O, false)),
        'p' => Ok((Key::KEY_P, false)),
        'q' => Ok((Key::KEY_Q, false)),
        'r' => Ok((Key::KEY_R, false)),
        's' => Ok((Key::KEY_S, false)),
        't' => Ok((Key::KEY_T, false)),
        'u' => Ok((Key::KEY_U, false)),
        'v' => Ok((Key::KEY_V, false)),
        'w' => Ok((Key::KEY_W, false)),
        'x' => Ok((Key::KEY_X, false)),
        'y' => Ok((Key::KEY_Y, false)),
        'z' => Ok((Key::KEY_Z, false)),
        '1' => Ok((Key::KEY_1, false)),
        '2' => Ok((Key::KEY_2, false)),
        '3' => Ok((Key::KEY_3, false)),
        '4' => Ok((Key::KEY_4, false)),
        '5' => Ok((Key::KEY_5, false)),
        '6' => Ok((Key::KEY_6, false)),
        '7' => Ok((Key::KEY_7, false)),
        '8' => Ok((Key::KEY_8, false)),
        '9' => Ok((Key::KEY_9, false)),
        '0' => Ok((Key::KEY_0, false)),
        ',' => Ok((Key::KEY_COMMA, false)),
        '.' => Ok((Key::KEY_DOT, false)),
        '/' => Ok((Key::KEY_SLASH, false)),
        ';' => Ok((Key::KEY_SEMICOLON, false)),
        '\'' => Ok((Key::KEY_APOSTROPHE, false)),
        '[' => Ok((Key::KEY_LEFTBRACE, false)),
        ']' => Ok((Key::KEY_RIGHTBRACE, false)),
        '\\' => Ok((Key::KEY_BACKSLASH, false)),
        '`' => Ok((Key::KEY_GRAVE, false)),
        '!' => Ok((Key::KEY_1, true)),
        '@' => Ok((Key::KEY_2, true)),
        '#' => Ok((Key::KEY_3, true)),
        '$' => Ok((Key::KEY_4, true)),
        '%' => Ok((Key::KEY_5, true)),
        '^' => Ok((Key::KEY_6, true)),
        '&' => Ok((Key::KEY_7, true)),
        '*' => Ok((Key::KEY_8, true)),
        '(' => Ok((Key::KEY_9, true)),
        ')' => Ok((Key::KEY_0, true)),
        '-' => Ok((Key::KEY_MINUS, false)),
        '=' => Ok((Key::KEY_EQUAL, false)),
        '_' => Ok((Key::KEY_MINUS, true)),
        '+' => Ok((Key::KEY_EQUAL, true)),
        '<' => Ok((Key::KEY_COMMA, true)),
        '>' => Ok((Key::KEY_DOT, true)),
        '?' => Ok((Key::KEY_SLASH, true)),
        ':' => Ok((Key::KEY_SEMICOLON, true)),
        '"' => Ok((Key::KEY_APOSTROPHE, true)),
        '{' => Ok((Key::KEY_LEFTBRACE, true)),
        '}' => Ok((Key::KEY_RIGHTBRACE, true)),
        '|' => Ok((Key::KEY_BACKSLASH, true)),
        '~' => Ok((Key::KEY_GRAVE, true)),
        e => Err(ServerError::Parse(e).into()),
    }
}

fn main() -> Result<()> {
    Ok(())
}
