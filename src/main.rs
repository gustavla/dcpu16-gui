extern crate dcpu16;
extern crate piston;
extern crate image;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate gl;
extern crate libc;
extern crate getopts;

use libc::c_void;

//use std::path::Path;
use piston::window::WindowSettings;
use piston::event::*;
use std::path::Path;
use std::env;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL, Texture };
use dcpu16::dcpu;
use dcpu16::dcpu::{DCPU, Hardware};
use gl::types::GLuint;
use getopts::Options;

pub struct GUIHWMonitorLEM1802 {
    pub connected: bool,
    pub ram_location: u16,

    pub default_font: [u16; 256],
    pub default_palette: [u32; 16],
}

const ROWS: usize = 12;
const COLS: usize = 32;
const FONT_WIDTH: usize = 4;
const FONT_HEIGHT: usize = 8;

impl GUIHWMonitorLEM1802 {
    pub fn new() -> GUIHWMonitorLEM1802 {
        GUIHWMonitorLEM1802::new_pre_connected(false)
    }

    pub fn new_pre_connected(pre_connected: bool) -> GUIHWMonitorLEM1802 {
        let (connected, ram_location) = if pre_connected {
            (true, 0x8000)
        } else {
            (false, 0)
        };

        GUIHWMonitorLEM1802{
            connected: connected,
            ram_location: ram_location,
            default_font: [ 0xb79e, 0x388e, 0x722c, 0x75f4, 0x19bb, 0x7f8f, 0x85f9, 0xb158, 0x242e,
            0x2400, 0x082a, 0x0800, 0x0008, 0x0000, 0x0808, 0x0808, 0x00ff, 0x0000, 0x00f8, 0x0808,
            0x08f8, 0x0000, 0x080f, 0x0000, 0x000f, 0x0808, 0x00ff, 0x0808, 0x08f8, 0x0808, 0x08ff,
            0x0000, 0x080f, 0x0808, 0x08ff, 0x0808, 0x6633, 0x99cc, 0x9933, 0x66cc, 0xfef8, 0xe080,
            0x7f1f, 0x0701, 0x0107, 0x1f7f, 0x80e0, 0xf8fe, 0x5500, 0xaa00, 0x55aa, 0x55aa, 0xffaa,
            0xff55, 0x0f0f, 0x0f0f, 0xf0f0, 0xf0f0, 0x0000, 0xffff, 0xffff, 0x0000, 0xffff, 0xffff,
            0x0000, 0x0000, 0x005f, 0x0000, 0x0300, 0x0300, 0x3e14, 0x3e00, 0x266b, 0x3200, 0x611c,
            0x4300, 0x3629, 0x7650, 0x0002, 0x0100, 0x1c22, 0x4100, 0x4122, 0x1c00, 0x1408, 0x1400,
            0x081c, 0x0800, 0x4020, 0x0000, 0x0808, 0x0800, 0x0040, 0x0000, 0x601c, 0x0300, 0x3e49,
            0x3e00, 0x427f, 0x4000, 0x6259, 0x4600, 0x2249, 0x3600, 0x0f08, 0x7f00, 0x2745, 0x3900,
            0x3e49, 0x3200, 0x6119, 0x0700, 0x3649, 0x3600, 0x2649, 0x3e00, 0x0024, 0x0000, 0x4024,
            0x0000, 0x0814, 0x2241, 0x1414, 0x1400, 0x4122, 0x1408, 0x0259, 0x0600, 0x3e59, 0x5e00,
            0x7e09, 0x7e00, 0x7f49, 0x3600, 0x3e41, 0x2200, 0x7f41, 0x3e00, 0x7f49, 0x4100, 0x7f09,
            0x0100, 0x3e41, 0x7a00, 0x7f08, 0x7f00, 0x417f, 0x4100, 0x2040, 0x3f00, 0x7f08, 0x7700,
            0x7f40, 0x4000, 0x7f06, 0x7f00, 0x7f01, 0x7e00, 0x3e41, 0x3e00, 0x7f09, 0x0600, 0x3e41,
            0xbe00, 0x7f09, 0x7600, 0x2649, 0x3200, 0x017f, 0x0100, 0x3f40, 0x3f00, 0x1f60, 0x1f00,
            0x7f30, 0x7f00, 0x7708, 0x7700, 0x0778, 0x0700, 0x7149, 0x4700, 0x007f, 0x4100, 0x031c,
            0x6000, 0x0041, 0x7f00, 0x0201, 0x0200, 0x8080, 0x8000, 0x0001, 0x0200, 0x2454, 0x7800,
            0x7f44, 0x3800, 0x3844, 0x2800, 0x3844, 0x7f00, 0x3854, 0x5800, 0x087e, 0x0900, 0x4854,
            0x3c00, 0x7f04, 0x7800, 0x447d, 0x4000, 0x2040, 0x3d00, 0x7f10, 0x6c00, 0x417f, 0x4000,
            0x7c18, 0x7c00, 0x7c04, 0x7800, 0x3844, 0x3800, 0x7c14, 0x0800, 0x0814, 0x7c00, 0x7c04,
            0x0800, 0x4854, 0x2400, 0x043e, 0x4400, 0x3c40, 0x7c00, 0x1c60, 0x1c00, 0x7c30, 0x7c00,
            0x6c10, 0x6c00, 0x4c50, 0x3c00, 0x6454, 0x4c00, 0x0836, 0x4100, 0x0077, 0x0000, 0x4136,
            0x0800, 0x0201, 0x0201, 0x0205, 0x0200 ],
            default_palette: [ 0x000000, 0x0000aa, 0x00aa00, 0x00aaaa, 0xaa0000, 0xaa00aa,
            0xaa5500, 0xaaaaaa, 0x555555, 0x5555ff, 0x55ff55, 0x55ffff, 0xff5555, 0xff55ff,
            0xffff55, 0xffffff],
        }
    }
}


impl Hardware for GUIHWMonitorLEM1802 {
    fn info_hardware_id_upper(&self) -> u16 { 0x7349 }
    fn info_hardware_id_lower(&self) -> u16 { 0xf615 }
    fn info_manufacturer_id_upper(&self) -> u16 { 0x1c6c }
    fn info_manufacturer_id_lower(&self) -> u16 { 0x8b36 }
    fn info_version(&self) -> u16 { 0x1802 }

    fn process_interrupt(&mut self, cpu: &mut DCPU) -> () {
        let a = cpu.reg[0];
        let b = cpu.reg[1];
        match a {
            0 => { /* MEM_MAP_SCREEN */
                if b > 0 {
                    self.ram_location = b;
                    self.connected = true;
                } else {
                    self.connected = false;
                }
            },
            _ => {}
        }
    }

    fn get_data(&self, cpu: &DCPU) -> Vec<u8> {
        let mut v: Vec<u8> = vec![0; 128 * 96 * 3];
        {
            let mut slice = &mut v[..];
            if self.connected {
                for i in 0..ROWS {
                    for j in 0..COLS {
                        //let (r, g, b) = (0, 0, 0);
                        let mem = cpu.mem[(self.ram_location as usize + i * COLS + j) % dcpu::MEMORY_SIZE];
                        //println!("mem = {}", mem);
                        let c = (mem & 0x7f) as usize;
                        //let blink = (mem >> 7) & 1;
                        let bg_color_index = ((mem >> 8) & 0xf) as usize;
                        let fg_color_index = ((mem >> 12) & 0xf) as usize;

                        let bg_color = self.default_palette[bg_color_index];
                        let fg_color = self.default_palette[fg_color_index];

                        let b0 = ((self.default_font[c*2] as u32)<<16) +
                                  (self.default_font[c*2+1] as u32);

                        for x in 0..FONT_WIDTH {
                            for y in 0..FONT_HEIGHT {
                                let p = (b0 >> ((FONT_WIDTH-1-x) * FONT_HEIGHT + y)) & 1;
                                let color = match p {
                                    1 => fg_color,
                                    _ => bg_color,
                                };

                                let index = i * COLS * FONT_WIDTH * FONT_HEIGHT + y * COLS * FONT_WIDTH + j * FONT_WIDTH + x;
                                slice[index * 3    ] = ((color >> 16) & 0xff) as u8;
                                slice[index * 3 + 1] = ((color >>  8) & 0xff) as u8;
                                slice[index * 3 + 2] = ((color      ) & 0xff) as u8;
                            }
                        }
                        if j > 10 {
                            break;
                        }
                    }
                    break;
                }
            } else {
                // Default screen
                /*
                for i in 0..(128 * 96) {
                    v.push(0);
                    v.push(0);
                    v.push(0);
                }
                */
            }
        }

        v
    }
}


pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
}

impl App {
    fn update(&mut self, _: &UpdateArgs) {

    }
}

fn main() {
    let mut opts = Options::new();

    let args: Vec<String> = env::args().collect();
    opts.optflag("p", "pre-connected", "pre-connected");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m },
        Err(why) => {
            println!("{}", why);
            return;
        },
    };

    if matches.free.len() != 1 {
        println!("Please input file");
        return;
    }
    let pre_connected = matches.opt_present("p");
    let ref filename = matches.free[0];

    let path = Path::new(filename);

    let opengl = OpenGL::_3_2;

    // Create an Glutin window.
    let window = Window::new(
        opengl,
        WindowSettings::new(
            "DCPU-16 LEM1802",
            [640, 480]
        )
        .exit_on_esc(true)
    );

    let mut cpu = DCPU::new();
    if let Err(why) = cpu.load_from_assembly_file(&path) {
        println!("Could load file {}: {}", path.display(), why);
        return;
    }

    // Attach monitor
    cpu.devices.push(Box::new(GUIHWMonitorLEM1802::new_pre_connected(pre_connected)));

    let mut img = image::ImageBuffer::new(128, 96);

    let (width, height) = img.dimensions();

    let mut id: GLuint = 0;
    unsafe {
        gl::GenTextures(1, &mut id);
        gl::BindTexture(gl::TEXTURE_2D, id);
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MIN_FILTER,
            gl::NEAREST as i32
        );
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MAG_FILTER,
            gl::NEAREST as i32
        );
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as i32,
            width as i32,
            height as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            img.as_ptr() as *const c_void
        );
    }

    let mut texture = Texture::new(id, width, height);

    img.put_pixel(10, 10, image::Rgba([200, 0, 0, 255]));



    println!("Here");

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
    };

    for e in window.events().max_fps(30).ups(30) {
        if let Some(args) = e.render_args() {
            //app.render(&args);
            app.gl.draw(args.viewport(), |_, gl| {
                graphics::clear([1.0; 4], gl);
                let mut t = [[0f64; 3]; 2];
                t[0][0] = 0.003125 * 5.0;
                t[1][1] = -0.004167 * 5.0;
                t[0][2] = -1.0;
                t[1][2] = 1.0;

                graphics::image(&texture, t, gl);
            });
        }

        if let Some(u) = e.update_args() {
            cpu.tick();
            let monitor = cpu.devices.get(0).unwrap();
            let v = &monitor.get_data(&cpu)[..];
            for k in 0..(v.len() / 3) {
                let color = image::Rgba([v[3 * k], v[3 * k + 1], v[3 * k + 2], 255]);
                img.put_pixel((k % 128) as u32, (k / 128) as u32, color);
            }

            texture.update(&img);
            app.update(&u);
        }
    }
}
