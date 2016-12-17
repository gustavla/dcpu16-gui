use dcpu16::dcpu::{self, DCPU, Device};
use std::any::Any;
use piston::input::keyboard::Key;

// If the queue (buffer) fills up, it will start to drop old entries
const MAX_BUFFER: usize = 256;

pub fn piston_key_to_code(key: Key) -> u16 {
    match key {
        Key::Backspace => 0x10,
        Key::Return => 0x11,
        Key::Insert => 0x12,
        Key::Delete => 0x13,
        Key::Space => 0x20,
        Key::Exclaim => 0x21,
        Key::Quotedbl => 0x22,
        Key::Hash => 0x23,
        Key::Dollar => 0x24,
        Key::Percent => 0x25,
        Key::Ampersand => 0x26,
        Key::Quote => 0x27,
        Key::LeftParen => 0x28,
        Key::RightParen => 0x29,
        Key::Asterisk => 0x2a,
        Key::Plus => 0x2b,
        Key::Comma => 0x2c,
        Key::Minus => 0x2d,
        Key::Period => 0x2e,
        Key::Slash => 0x2f,
        Key::D0 => 0x30,
        Key::D1 => 0x31,
        Key::D2 => 0x32,
        Key::D3 => 0x33,
        Key::D4 => 0x34,
        Key::D5 => 0x35,
        Key::D6 => 0x36,
        Key::D7 => 0x37,
        Key::D8 => 0x38,
        Key::D9 => 0x39,
        Key::Colon => 0x3a,
        Key::Semicolon => 0x3b,
        Key::Less => 0x3c,
        Key::Equals => 0x3d,
        Key::Greater => 0x3e,
        Key::Question => 0x3f,
        Key::At => 0x40,
        Key::LeftBracket => 0x5b,
        Key::Backslash => 0x5c,
        Key::RightBracket => 0x5d,
        Key::Caret => 0x5e,
        Key::Underscore => 0x5f,
        Key::Backquote => 0x60,
        Key::A => 0x61,
        Key::B => 0x62,
        Key::C => 0x63,
        Key::D => 0x64,
        Key::E => 0x65,
        Key::F => 0x66,
        Key::G => 0x67,
        Key::H => 0x68,
        Key::I => 0x69,
        Key::J => 0x6a,
        Key::K => 0x6b,
        Key::L => 0x6c,
        Key::M => 0x6d,
        Key::N => 0x6e,
        Key::O => 0x6f,
        Key::P => 0x70,
        Key::Q => 0x71,
        Key::R => 0x72,
        Key::S => 0x73,
        Key::T => 0x74,
        Key::U => 0x75,
        Key::V => 0x76,
        Key::W => 0x77,
        Key::X => 0x78,
        Key::Y => 0x79,
        Key::Z => 0x7a,

        Key::Up => 0x80,
        Key::Down => 0x81,
        Key::Left => 0x82,
        Key::Right => 0x83,

        Key::LShift | Key::RShift => 0x90,
        Key::LCtrl | Key::RCtrl => 0x91,

        //Key:: => 0x7a,
        //s => s as u16 + 10,
        _ => 0,
    }
}

fn with_shift(key: u16) -> u16 {
    match key {
        0x30 => 0x29,
        0x31 => 0x21,
        0x32 => 0x40,
        0x33 => 0x23,
        0x34 => 0x24,
        0x35 => 0x25,
        0x36 => 0x5e,
        0x37 => 0x26,
        0x38 => 0x2a,
        0x39 => 0x28,
        0x3b => 0x3a,
        0x2a => 0x3c,
        0x2e => 0x3e,
        0x2f => 0x3f,
        0x2d => 0x5f,
        0x3d => 0x2b,
        a @ 0x61 ... 0x7a => a - 32,
        a @ 0x5b ... 0x5d => a + 32,
        a => a,
    }
}

pub struct DeviceKeyboardGeneric {
    buffer: Vec<u16>,
    interrupt_message: Option<u16>,
    pressed: [bool; 0x92],
}

impl DeviceKeyboardGeneric {
    pub fn new() -> DeviceKeyboardGeneric {
        DeviceKeyboardGeneric {
            buffer: Vec::new(),
            interrupt_message: None,
            pressed: [false; 0x92],
        }
    }

    pub fn register_press(&mut self, cpu: &mut DCPU, key: u16) -> () {
        // Do not add shift/ctrl to queue
        if key != 0x90 && key != 0x91 {
            // Check if buffer has grown too big
            // If it is, we clear the whole buffer (let's pretend it fails)
            // This will likely happen when the user isn't even using the buffers, or if they turn
            // on queueing and let it go too long.
            if self.buffer.len() >= MAX_BUFFER {
                self.buffer.clear();
            }

            if self.pressed[0x90] {
                self.buffer.push(with_shift(key));
            } else {
                self.buffer.push(key);
            }
        }

        // Mark it as pressed
        if key < 0x92 {
            self.pressed[key as usize] = true;
        }

        // Trigger interrupt
        if let Some(m) = self.interrupt_message {
            cpu.interrupt(m);
        }
    }

    pub fn register_release(&mut self, cpu: &mut DCPU, key: u16) -> () {
        // Buffer is unaffected by this operation

        // Unmark it as pressed
        if key < 0x92 {
            self.pressed[key as usize] = false;
        }

        // Trigger interrupt
        if let Some(m) = self.interrupt_message {
            cpu.interrupt(m);
        }
    }
}

impl Device for DeviceKeyboardGeneric {
    fn info_hardware_id_upper(&self) -> u16 { 0x30cf }
    fn info_hardware_id_lower(&self) -> u16 { 0x7406 }
    fn info_manufacturer_id_upper(&self) -> u16 { 0x0 }
    fn info_manufacturer_id_lower(&self) -> u16 { 0x0 }
    fn info_version(&self) -> u16 { 1 }

    fn process_interrupt(&mut self, cpu: &mut DCPU) -> () {
        let reg_a = cpu.reg[dcpu::REG_A];
        let reg_b = cpu.reg[dcpu::REG_B];
        match reg_a {
            0 => { // Clear keyboard buffer
                self.buffer.clear();
            },
            1 => {
                let v = self.buffer.pop().unwrap_or(0);
                cpu.reg[dcpu::REG_C] = v;
            },
            2 => {
                let key = cpu.reg[dcpu::REG_B];
                cpu.reg[dcpu::REG_C] = if key < 0x92 && self.pressed[key as usize] {
                    1
                } else {
                    0
                };
            },
            3 => {
                self.interrupt_message = if reg_b != 0 {
                    Some(reg_b)
                } else {
                    None
                };
            },
            _ => {}
        }
    }

    fn run(&mut self, _: &mut DCPU, _: usize) -> () {}

    fn as_any(&self) -> &Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut Any {
        self
    }
}


