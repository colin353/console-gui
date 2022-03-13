use input::{
    event::{
        keyboard::{
            KeyState, {KeyboardEvent, KeyboardEventTrait},
        },
        Event,
    },
    Libinput, LibinputInterface,
};
use nix::{
    fcntl::{open, OFlag},
    sys::stat::Mode,
    unistd::close,
};

pub enum Key {
    LCD1,
    LCD2,
    LCD3,
    LCD4,
    Abort,
    Execute,
    Danger,
    Slider(usize),
}

impl Key {
    fn from_u32(x: u32) -> Option<Self> {
        match x {
            183 => Some(Self::LCD1),
            184 => Some(Self::LCD2),
            185 => Some(Self::LCD3),
            186 => Some(Self::LCD4),
            187 => Some(Self::Abort),
            188 => Some(Self::Execute),
            70 => Some(Self::Danger),
            194 => Some(Self::Slider(0)),
            193 => Some(Self::Slider(1)),
            192 => Some(Self::Slider(2)),
            191 => Some(Self::Slider(3)),
            190 => Some(Self::Slider(4)),
            189 => Some(Self::Slider(5)),
            _ => None,
        }
    }
}

struct LibinputInterfaceRaw;

impl LibinputInterface for LibinputInterfaceRaw {
    fn open_restricted(
        &mut self,
        path: &std::path::Path,
        flags: i32,
    ) -> std::result::Result<std::os::unix::io::RawFd, i32> {
        if let Ok(fd) = open(path, OFlag::from_bits_truncate(flags), Mode::empty()) {
            Ok(fd)
        } else {
            Err(1)
        }
    }

    fn close_restricted(&mut self, fd: std::os::unix::io::RawFd) {
        let _ = close(fd);
    }
}

pub fn handle_input_events<F: Fn(Key) + Send + Sync + 'static>(callback: F) {
    let mut libinput_context = Libinput::new_with_udev(LibinputInterfaceRaw);
    libinput_context.udev_assign_seat("seat0").unwrap();
    loop {
        libinput_context.dispatch().unwrap();
        while let Some(event) = libinput_context.next() {
            if let Event::Keyboard(KeyboardEvent::Key(evt)) = event {
                if evt.key_state() == KeyState::Pressed {
                    if let Some(k) = Key::from_u32(evt.key()) {
                        callback(k)
                    }
                }
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
