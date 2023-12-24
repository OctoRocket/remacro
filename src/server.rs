use anyhow::Result;
use evdev::{
    uinput::VirtualDeviceBuilder,
    Key,
    AttributeSet,
};
use std::{
    path::PathBuf,
    os::unix::net::{
        UnixListener,
        UnixStream
    },
    fs::remove_file,
    io::Read, collections::HashMap,
};
use thiserror::Error;

type KeyShiftPairs = Vec<(Key, bool)>;

#[derive(Debug, Error)]
enum ServerError {
    #[error("Unable to parse character {0}.")]
    Parse(char),
}

static AVAILABLE_KEYS: &[(char, (Key, bool))] = HashMap::from([
    ('a' , (Key::KEY_A,          false)),
    ('b' , (Key::KEY_B,          false)),
    ('c' , (Key::KEY_C,          false)),
    ('d' , (Key::KEY_D,          false)),
    ('e' , (Key::KEY_E,          false)),
    ('f' , (Key::KEY_F,          false)),
    ('g' , (Key::KEY_G,          false)),
    ('h' , (Key::KEY_H,          false)),
    ('i' , (Key::KEY_I,          false)),
    ('j' , (Key::KEY_J,          false)),
    ('k' , (Key::KEY_K,          false)),
    ('l' , (Key::KEY_L,          false)),
    ('m' , (Key::KEY_M,          false)),
    ('n' , (Key::KEY_N,          false)),
    ('o' , (Key::KEY_O,          false)),
    ('p' , (Key::KEY_P,          false)),
    ('q' , (Key::KEY_Q,          false)),
    ('r' , (Key::KEY_R,          false)),
    ('s' , (Key::KEY_S,          false)),
    ('t' , (Key::KEY_T,          false)),
    ('u' , (Key::KEY_U,          false)),
    ('v' , (Key::KEY_V,          false)),
    ('w' , (Key::KEY_W,          false)),
    ('x' , (Key::KEY_X,          false)),
    ('y' , (Key::KEY_Y,          false)),
    ('z' , (Key::KEY_Z,          false)),
    ('1' , (Key::KEY_1,          false)),
    ('2' , (Key::KEY_2,          false)),
    ('3' , (Key::KEY_3,          false)),
    ('4' , (Key::KEY_4,          false)),
    ('5' , (Key::KEY_5,          false)),
    ('6' , (Key::KEY_6,          false)),
    ('7' , (Key::KEY_7,          false)),
    ('8' , (Key::KEY_8,          false)),
    ('9' , (Key::KEY_9,          false)),
    ('0' , (Key::KEY_0,          false)),
    (',' , (Key::KEY_COMMA,      false)),
    ('.' , (Key::KEY_DOT,        false)),
    ('/' , (Key::KEY_SLASH,      false)),
    (';' , (Key::KEY_SEMICOLON,  false)),
    ('\'', (Key::KEY_APOSTROPHE, false)),
    ('[' , (Key::KEY_LEFTBRACE,  false)),
    (']' , (Key::KEY_RIGHTBRACE, false)),
    ('\\', (Key::KEY_BACKSLASH,  false)),
    ('`' , (Key::KEY_GRAVE,      false)),
    ('!' , (Key::KEY_1,          true)),
    ('@' , (Key::KEY_2,          true)),
    ('#' , (Key::KEY_3,          true)),
    ('$' , (Key::KEY_4,          true)),
    ('%' , (Key::KEY_5,          true)),
    ('^' , (Key::KEY_6,          true)),
    ('&' , (Key::KEY_7,          true)),
    ('*' , (Key::KEY_8,          true)),
    ('(' , (Key::KEY_9,          true)),
    (')' , (Key::KEY_0,          true)),
    ('-' , (Key::KEY_MINUS,      false)),
    ('=' , (Key::KEY_EQUAL,      false)),
    ('_' , (Key::KEY_MINUS,      true)),
    ('+' , (Key::KEY_EQUAL,      true)),
    ('<' , (Key::KEY_COMMA,      true)),
    ('>' , (Key::KEY_DOT,        true)),
    ('?' , (Key::KEY_SLASH,      true)),
    (':' , (Key::KEY_SEMICOLON,  true)),
    ('"' , (Key::KEY_APOSTROPHE, true)),
    ('{' , (Key::KEY_LEFTBRACE,  true)),
    ('}' , (Key::KEY_RIGHTBRACE, true)),
    ('|' , (Key::KEY_BACKSLASH,  true)),
    ('~' , (Key::KEY_GRAVE,      true)),
]);

fn to_key(c: char) -> Result<(Key, bool)> {
    AVAILABLE_KEYS.iter();
    todo!()
}

fn to_keys(input: String) -> Result<KeyShiftPairs> {
    Ok(input.chars().map(|c| to_key(c).expect("Unable to convert character into keypress.")).collect::<KeyShiftPairs>())
}

fn type_string(input: KeyShiftPairs) {
    todo!()
}

fn handle_input(mut input: UnixStream) -> Result<()> {
    let mut buf = String::new();
    match input.read_to_string(&mut buf) {
        Ok(_) => (),
        Err(e) => println!("FAILED to read transmitted data with error: {e}."),
    }
    type_string(to_keys(buf)?);

    Ok(())
}

pub fn launch_server(addr: PathBuf) -> Result<()> {
    let keys = AttributeSet::<Key>::new();
    let virt_device = VirtualDeviceBuilder::new()?
        .name("Macro Keyboard")
        .with_keys(&keys);

    let _ = remove_file(&addr);
    let listener = UnixListener::bind(&addr)?;

    for input in listener.incoming() {
        match input {
            Ok(input) => handle_input(input)?,
            Err(e) => println!("Connection failed with error: {e}"),
        }
    }

    println!("done");

    todo!()
}
