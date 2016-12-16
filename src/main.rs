extern crate dcpu16;
extern crate piston;
extern crate graphics;
extern crate opengl_graphics;
extern crate gl;
extern crate libc;
extern crate getopts;
extern crate piston_window;
extern crate image;
extern crate event_loop;
extern crate time;

mod monitor_lem1802;
mod keyboard_generic;

use std::path::Path;
use std::env;
use dcpu16::dcpu::DCPU;
use getopts::Options;
use event_loop::{Events, EventLoop};
use graphics::Transformed;
use piston::input::{RenderEvent, Button, PressEvent, ReleaseEvent};
use piston::input::keyboard::Key;

use monitor_lem1802::{DeviceMonitorLEM1802, MONITOR_WIDTH, MONITOR_HEIGHT, SCALE, BORDER};
use keyboard_generic::DeviceKeyboardGeneric;

fn main() {
    let mut opts = Options::new();

    let args: Vec<String> = env::args().collect();
    opts.optopt("m", "monitor", "Pre-connect monitor", "ADDRESS");
    opts.optopt("f", "font", "Pre-connect font address", "ADDRESS");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(why) => {
            println!("{}", why);
            return;
        },
    };

    if matches.free.len() != 1 {
        println!("Please input file");
        return;
    }
    let ref filename = matches.free[0];

    let path = Path::new(filename);
    let window_size = piston_window::Size {width: MONITOR_WIDTH * SCALE + BORDER * 2 as u32, height: (MONITOR_HEIGHT as u32 * SCALE + BORDER * 2)};

    let mut window: piston_window::PistonWindow = piston_window::WindowSettings::new(
            "DCPU16 Monitor",
            window_size
        )
        .exit_on_esc(true)
        .opengl(piston_window::OpenGL::V3_2)
        .build()
        .unwrap();

    let mut cpu = DCPU::new();
    if let Err(why) = cpu.load_from_binary_file(&path) {
        println!("Could load file {}: {}", path.display(), why);
        return;
    }

    let mut monitor = DeviceMonitorLEM1802::new();

    monitor = match matches.opt_str("monitor") {
        Some(s) => {
            if s.len() >= 2 && s.chars().nth(0).unwrap() == '0' && (s.chars().nth(1).unwrap() == 'X' || s.chars().nth(1).unwrap() == 'x') {
                match isize::from_str_radix(&s[2..], 16) {
                    Ok(loc) => monitor.with_pre_connect(loc as u16),
                    Err(e) => {
                        println!("Could not parse argument for -m/--monitor: {}", e);
                        return;
                    },
                }
            } else {
                println!("Could not parse argument for -m/--monitor: must be hexadecimal with leading 0x");
                return;
            }
        },
        None => monitor,
    };
    // TODO Remove this hideous copy-paste from above
    monitor = match matches.opt_str("font") {
        Some(s) => {
            if s.len() >= 2 && s.chars().nth(0).unwrap() == '0' && (s.chars().nth(1).unwrap() == 'X' || s.chars().nth(1).unwrap() == 'x') {
                match isize::from_str_radix(&s[2..], 16) {
                    Ok(loc) => monitor.with_font_location(loc as u16),
                    Err(e) => {
                        println!("Could not parse argument for -f/--font: {}", e);
                        return;
                    },
                }
            } else {
                println!("Could not parse argument for -f/--font: must be hexadecimal with leading 0x");
                return;
            }
        },
        None => monitor,
    };

    let keyboard = DeviceKeyboardGeneric::new();

    cpu.add_device(Box::new(monitor));
    cpu.add_device(Box::new(keyboard));

    let mut img = image::ImageBuffer::new(MONITOR_WIDTH as u32, MONITOR_HEIGHT as u32);

    let mut text_sett = piston_window::TextureSettings::new();
    text_sett.set_filter(piston_window::texture::Filter::Nearest);
    let mut texture = piston_window::Texture::from_image(&mut window.factory, &mut img, &text_sett).unwrap();

    window.set_bench_mode(true);
    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {
        let time = time::get_time();
        let blinkout = time.nsec > 500_000_000;

        if let Some(Button::Keyboard(key)) = e.press_args() {

            //if let Some(devices) = Rc::get_mut(&mut cpu.devices) {
            let devices = cpu.devices.clone();
            {
                let mut dev = devices.get(1).unwrap().borrow_mut();
                if let Some(mut keyboard) = dev.as_any_mut().downcast_mut::<DeviceKeyboardGeneric>() {
                    let v = keyboard_generic::piston_key_to_code(key);
                    if v > 0 {
                        keyboard.register_press(&mut cpu, v);
                    }
                }
            }
        }
        if let Some(Button::Keyboard(key)) = e.release_args() {
            let devices = cpu.devices.clone();
            {
                let mut dev = devices.get(1).unwrap().borrow_mut();
                if let Some(mut keyboard) = dev.as_any_mut().downcast_mut::<DeviceKeyboardGeneric>() {
                    let v = keyboard_generic::piston_key_to_code(key);
                    if v > 0 {
                        keyboard.register_release(&mut cpu, v);
                    }
                }
            }
        }
        if let Some(_) = e.render_args() {
            // TODO: Make the speed a setting
            cpu.run(3000);
            let dev = cpu.devices.get(0).unwrap().borrow();
            if let Some(monitor) = dev.as_any().downcast_ref::<DeviceMonitorLEM1802>() {
                let v = &monitor.data(&cpu, blinkout)[..];
                for k in 0..(v.len() / 3) {
                    let color = image::Rgba([v[3 * k], v[3 * k + 1], v[3 * k + 2], 255]);
                    img.put_pixel((k as u32 % MONITOR_WIDTH), k as u32 / MONITOR_WIDTH as u32, color);
                }

                match texture.update(&mut window.encoder, &img) {
                    Ok(_) => {},
                    Err(_) => {
                        println!("Error");
                    }
                }
            }
        }
        window.draw_2d(&e, |c, g| {
            let dev = cpu.devices.get(0).unwrap().borrow();
            if let Some(monitor) = dev.as_any().downcast_ref::<DeviceMonitorLEM1802>() {
                let (col_r, col_g, col_b) = monitor.get_border_color_rgb(&cpu);
                piston_window::clear([col_r as f32 / 255.0, col_g as f32 / 255.0, col_b as f32 / 255.0, 1.0], g);

                piston_window::image(&texture, c.transform
                    .trans(BORDER as f64, BORDER as f64).zoom(SCALE as f64), g);
            }
        });
    }
}
