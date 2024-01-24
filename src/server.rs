use anyhow::Result;
use evdev::{
    uinput::{
        VirtualDeviceBuilder,
        VirtualDevice
    },
    Key,
    AttributeSet,
    InputEvent,
    EventType,
};
use std::{
    path::PathBuf,
    os::unix::net::{
        UnixListener,
        UnixStream
    },
    fs::remove_file,
    io::Read,
    collections::HashMap,
    time::Duration,
    thread::sleep,
};
use thiserror::Error;
use lazy_static::lazy_static;

#[derive(Debug, Error)]
enum ServerError {
    #[error("Invalid character in string.")]
    Parse(),
}

lazy_static! {
    static ref AVAILABLE_KEYS: HashMap<char, (Key, bool)> = HashMap::from([
        ('a' , (Key::KEY_A          , false)),
        ('b' , (Key::KEY_B          , false)),
        ('c' , (Key::KEY_C          , false)),
        ('d' , (Key::KEY_D          , false)),
        ('e' , (Key::KEY_E          , false)),
        ('f' , (Key::KEY_F          , false)),
        ('g' , (Key::KEY_G          , false)),
        ('h' , (Key::KEY_H          , false)),
        ('i' , (Key::KEY_I          , false)),
        ('j' , (Key::KEY_J          , false)),
        ('k' , (Key::KEY_K          , false)),
        ('l' , (Key::KEY_L          , false)),
        ('m' , (Key::KEY_M          , false)),
        ('n' , (Key::KEY_N          , false)),
        ('o' , (Key::KEY_O          , false)),
        ('p' , (Key::KEY_P          , false)),
        ('q' , (Key::KEY_Q          , false)),
        ('r' , (Key::KEY_R          , false)),
        ('s' , (Key::KEY_S          , false)),
        ('t' , (Key::KEY_T          , false)),
        ('u' , (Key::KEY_U          , false)),
        ('v' , (Key::KEY_V          , false)),
        ('w' , (Key::KEY_W          , false)),
        ('x' , (Key::KEY_X          , false)),
        ('y' , (Key::KEY_Y          , false)),
        ('z' , (Key::KEY_Z          , false)),
        ('1' , (Key::KEY_1          , false)),
        ('2' , (Key::KEY_2          , false)),
        ('3' , (Key::KEY_3          , false)),
        ('4' , (Key::KEY_4          , false)),
        ('5' , (Key::KEY_5          , false)),
        ('6' , (Key::KEY_6          , false)),
        ('7' , (Key::KEY_7          , false)),
        ('8' , (Key::KEY_8          , false)),
        ('9' , (Key::KEY_9          , false)),
        ('0' , (Key::KEY_0          , false)),
        (',' , (Key::KEY_COMMA      , false)),
        ('.' , (Key::KEY_DOT        , false)),
        ('/' , (Key::KEY_SLASH      , false)),
        (';' , (Key::KEY_SEMICOLON  , false)),
        ('\'', (Key::KEY_APOSTROPHE , false)),
        ('[' , (Key::KEY_LEFTBRACE  , false)),
        (']' , (Key::KEY_RIGHTBRACE , false)),
        ('\\', (Key::KEY_BACKSLASH  , false)),
        ('`' , (Key::KEY_GRAVE      , false)),
        ('-' , (Key::KEY_MINUS      , false)),
        ('=' , (Key::KEY_EQUAL      , false)),
        ('!' , (Key::KEY_1          , true )),
        ('@' , (Key::KEY_2          , true )),
        ('#' , (Key::KEY_3          , true )),
        ('$' , (Key::KEY_4          , true )),
        ('%' , (Key::KEY_5          , true )),
        ('^' , (Key::KEY_6          , true )),
        ('&' , (Key::KEY_7          , true )),
        ('*' , (Key::KEY_8          , true )),
        ('(' , (Key::KEY_9          , true )),
        (')' , (Key::KEY_0          , true )),
        ('_' , (Key::KEY_MINUS      , true )),
        ('+' , (Key::KEY_EQUAL      , true )),
        ('<' , (Key::KEY_COMMA      , true )),
        ('>' , (Key::KEY_DOT        , true )),
        ('?' , (Key::KEY_SLASH      , true )),
        (':' , (Key::KEY_SEMICOLON  , true )),
        ('"' , (Key::KEY_APOSTROPHE , true )),
        ('{' , (Key::KEY_LEFTBRACE  , true )),
        ('}' , (Key::KEY_RIGHTBRACE , true )),
        ('|' , (Key::KEY_BACKSLASH  , true )),
        ('~' , (Key::KEY_GRAVE      , true )),
        ('\n', (Key::KEY_ENTER      , false)),
    ]);
}

const DELAY: Duration = Duration::from_millis(50);

fn to_keys(text: &str) -> Result<Vec<(Key, bool)>> {
    let mut keys = text.chars().map(|c| AVAILABLE_KEYS.get(&c));
    if keys.any(|k| k.is_none()) {
        return Err(ServerError::Parse().into());
    }

    Ok(keys.map(|c| *c.unwrap()).collect())
}

fn press_key(keycode: u16, device: &mut VirtualDevice) {
    let down_event = InputEvent::new(EventType::KEY, keycode, 1);
    device.emit(&[down_event]).unwrap();
}

fn type_string(key_string: &[(Key, bool)], device: &mut VirtualDevice) {
    for key_combo in key_string {
        let (key, shift) = *key_combo;
        let keycode = key.code();

        if shift {
            press_key(Key::KEY_LEFTSHIFT.code(), device);
            sleep(DELAY);
        }

        press_key(keycode, device);
        sleep(DELAY);
    }
}

fn handle_input(mut input: UnixStream, device: &mut VirtualDevice) -> Result<()> {
    let mut buf = String::new();
    match input.read_to_string(&mut buf) {
        Ok(_) => (),
        Err(e) => println!("FAILED to read transmitted data with error: {e}."),
    }
    type_string(&to_keys(&buf)?, device);

    Ok(())
}

pub fn launch(addr: &PathBuf) -> Result<()> {
    let mut keys: AttributeSet<Key> = AVAILABLE_KEYS.values().map(|p| p.0).collect();
    keys.insert(Key::KEY_LEFTSHIFT);

    let mut virt_device = VirtualDeviceBuilder::new()?
        .name("Macro Keyboard")
        .with_keys(&keys)?
        .build()?;

    let _ = remove_file(addr);
    let listener = UnixListener::bind(addr)?;

    for input in listener.incoming() {
        match input {
            Ok(input) => handle_input(input, &mut virt_device)?,
            Err(e) => println!("Connection failed with error: {e}"),
        }
    }

    println!("done");

    Ok(())
}
