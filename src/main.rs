use std::ops;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use rand::{Rng, rngs};
use std::convert::TryInto;


fn main() -> Result<(), String> {

    // SDL downloaded from: https://www.libsdl.org/download-2.0.php
    // Following instructions from here: https://github.com/PistonDevelopers/piston-examples/issues/391#issuecomment-336219251
    // So copying SDL2.dll and SDL2.lib into C:\Users\james\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\lib
    // and adding sdl2 = "0.32.1" to Cargo.toml

    println!("Hello, world!");

    let mut trail: Trail<1024> = Trail::new();
    // let mut colony: Colony<1024> = Colony::new_random(1024.0);

    // let sdl_context = sdl2::init()?;
    // let video_subsystem = sdl_context.video()?;
    // let window = video_subsystem.window("rust-sdl2 demo", 1024, 1024) 
    //     .position_centered()
    //     .build()
    //     .expect("could not initialize video subsystem");

    // let mut canvas = window.into_canvas()
    //     .build()
    //     .expect("could not make a canvas");

    // canvas.set_draw_color(Color::RGB(0, 255, 255));
    // canvas.clear();
    // canvas.present();
    // let mut event_pump = sdl_context.event_pump()?;
    // let mut i = 0;

    // 'running: loop {
    //     colony = colony.steered(&trail).moved(0.1, &trail);
    //     trail = colony.deposit_on(&trail);

    //     i = (i + 1) % 255;
    //     canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
    //     canvas.clear();
        
    //     for ( x, row ) in trail.trail.iter().enumerate() {
    //         for ( y, value ) in row.iter().enumerate() {
    //             let col = value.min(255.0) as u8;
    //             canvas.set_draw_color(Color::RGB(col, col, col));
    //             canvas.draw_point((x as i32, y as i32)).unwrap();
    //         }
    //     }


    //     for event in event_pump.poll_iter() {
    //         match event {
    //             Event::Quit {..} |
    //             Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
    //                 break 'running;
    //             },
    //             _ => {}
    //         }
    //     }

    //     canvas.present();

    //     ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    // }

    return Ok(());
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Vec2f {
    x: f32,
    y: f32,
}

impl ops::Add for Vec2f {
    type Output = Self;

    fn add(self, other: Vec2f) -> Vec2f {
        return Vec2f {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl ops::Mul<f32> for Vec2f {
    type Output = Self;

    fn mul(self, scale: f32) -> Vec2f {
        return Vec2f {
            x: self.x * scale,
            y: self.y * scale,
        };
    }
}

impl Vec2f {
    fn rotated(&self, theta: f32) -> Vec2f {
        let cs = theta.cos();
        let sn = theta.sin();
        let rx = self.x * cs - self.y * sn;
        let ry = self.x * sn + self.y * cs;
        return Vec2f { x: rx, y: ry };
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Trail<const SIZE: usize> {
    trail: Vec<f32>,
}

impl<const SIZE: usize> Trail<SIZE> {
    fn new() -> Trail<SIZE> {
        return Trail {
            trail: vec![0.0_f32; SIZE*SIZE],
        }
    }

    fn set(mut self, x: usize, y: usize, val: f32) -> Trail<SIZE> {
        self.trail[x+y*SIZE] = val;
        return self;
    }

    fn get_pos_wrapped(&self, pos: Vec2f) -> Vec2f {
        let size_f = SIZE as f32;
        let x = ( pos.x + size_f as f32 ) % size_f;
        let y = ( pos.y + size_f as f32 ) % size_f;
        return Vec2f { x: x, y: y }
    }

    fn get_val_wrapped(&self, pos: Vec2f) -> f32 {
        let x = ( pos.x as usize + SIZE ) % SIZE;
        let y = ( pos.y as usize + SIZE ) % SIZE;
        return self.trail[x+y*SIZE];
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Slime {
    pos: Vec2f,
    vel: Vec2f,
}

impl Slime {
    fn new() -> Slime {
        return Slime {
            pos: Vec2f { x: 0.0, y: 0.0 },
            vel: Vec2f { x: 0.0, y: 0.0 },
        };
    }

    fn at(&self, x: f32, y: f32) -> Slime {
        return Slime {
            pos: Vec2f { x: x, y: y },
            vel: self.vel,
        };
    }

    fn going(&self, vx: f32, vy: f32) -> Slime {
        return Slime {
            pos: self.pos,
            vel: Vec2f { x: vx, y: vy },
        };
    }

    fn updated<const TRAIL_SIZE: usize>(&self, time: f32, trail: &Trail<TRAIL_SIZE>) -> Slime {
        let new_pos = trail.get_pos_wrapped(self.pos + self.vel * time);
        return Slime {
            pos: new_pos,
            vel: self.vel,
        };
    }

    fn steered<const TRAIL_SIZE: usize>(&self, trail: &Trail<TRAIL_SIZE>) -> Slime {
        let l_vel = self.vel.rotated(-45.0_f32.to_radians());
        let r_vel = self.vel.rotated(45.0_f32.to_radians());

        let f_val = trail.get_val_wrapped(self.pos + self.vel);
        let l_val = trail.get_val_wrapped(self.pos + l_vel);
        let r_val = trail.get_val_wrapped(self.pos + r_vel);

        let vel = if f_val >= l_val {
            if f_val >= r_val {
                self.vel
            } else {
                r_vel
            }
        } else {
            if l_val >= r_val {
                l_vel
            } else {
                r_vel
            }
        };

        return Slime {
            pos: self.pos,
            vel: vel,
        };
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Colony<const SIZE: usize> {
    colony: [Slime; SIZE],
}

impl<const SIZE: usize> Colony<SIZE> {
    fn new(colony: [Slime; SIZE]) -> Colony<SIZE> {
        return Colony { colony: colony };
    }

    fn moved<const TRAIL_SIZE: usize>(&self, time: f32, trail: &Trail<TRAIL_SIZE>) -> Colony<SIZE> {
        return Colony {
            colony: self.colony.map(|s| s.updated(time, trail)),
        };
    }

    fn steered<const TRAIL_SIZE: usize>(&self, trail: &Trail<TRAIL_SIZE>) -> Colony<SIZE> {
        return Colony {
            colony: self.colony.map(|s| s.steered(trail)),
        };
    }

    fn deposit_on<const TRAIL_SIZE: usize>(&self, trail: &Trail<TRAIL_SIZE>) -> Trail<TRAIL_SIZE> {
        let mut acc: Trail<TRAIL_SIZE> = (*trail).clone();
        for slime in self.colony {
            let x = slime.pos.x as usize;
            let y = slime.pos.y as usize;
            acc.trail[x+y*TRAIL_SIZE] += 1.0;
        }
        return acc;
    }

    fn new_random(limit_xy: f32) -> Colony<SIZE> {
        let mut vec: Vec<Slime> = vec![];
        for _ in 0 .. SIZE {
            let theta = rand::thread_rng().gen_range(0.0_f32 .. 360.0_f32).to_radians();
            let vel = Vec2f { x: 1.0, y: 0.0 }.rotated(theta);
            let rand_slime = Slime { 
                pos: Vec2f { 
                    x: rand::thread_rng().gen_range(0.0_f32 .. limit_xy),
                    y: rand::thread_rng().gen_range(0.0_f32 .. limit_xy),
                }, 
                vel: vel 
            };
            vec.push(rand_slime);
        }

        let array: [Slime; SIZE] = vec.try_into().unwrap();
        return Colony::new(array);
    }
}

#[cfg(test)]
mod slime_tests {

    use super::*;

    impl Slime {
        fn as_colony(&self) -> Colony<1> {
            let arr: [Slime; 1] = [*self];
            return Colony::new(arr);
        }
    }

    #[test]
    fn colony_update() {
        let colony = Slime::new().going(0.0, 1.0).as_colony();
        let trail = new_3x3_trail();
        let colony_updated = colony.moved(1.0, &trail);
        let expected = Slime::new().at(0.0, 1.0).going(0.0, 1.0).as_colony();
        assert_eq!(expected, colony_updated);
    }

    #[test]
    fn colony_update_half() {
        let colony = Slime::new().going(1.0, 1.0).as_colony();
        let trail = new_3x3_trail();
        let colony_updated = colony.moved(0.5, &trail);
        let expected = Slime::new().at(0.5, 0.5).going(1.0, 1.0).as_colony();
        assert_eq!(expected, colony_updated);
    }

    #[test]
    fn colony_steer_forward() {
        let colony = Slime::new().going(1.0, 1.0).as_colony();
        let trail = new_3x3_trail();
        let colony_updated = colony.steered(&trail);
        let expected = Slime::new().going(1.0, 1.0).as_colony();
        assert_eq!(expected, colony_updated);
    }

    #[test]
    fn colony_steer_right() {
        let colony = Slime::new().going(1.0, 1.0).as_colony();
        let trail: Trail<3> = Trail::new().set(0, 1, 0.1);
        let colony_updated = colony.steered(&trail);
        let expected = Slime::new().going( 0.0, 2.0_f32.sqrt()).as_colony();
        assert_eq!(expected, colony_updated);
    }

    #[test]
    fn colony_steer_left() {
        let colony = Slime::new().going(1.0, 1.0).as_colony();
        let trail: Trail<3> = Trail::new().set(1, 0, 0.1);
        let colony_updated = colony.steered(&trail);
        let expected = Slime::new().going(2.0_f32.sqrt(), 0.0).as_colony();
        assert_eq!(expected, colony_updated);
    }

    #[test]
    fn test_wrap_steer() {
        let colony = Slime::new().going(1.0, 1.0).as_colony();
        let trail: Trail<1> = Trail {
            trail: vec![0.0],
        };
        let _ = colony.steered(&trail);
    }

    #[test]
    fn test_deposit() {
        let colony = Slime::new().at(1.0, 1.0).as_colony();
        let trail = new_3x3_trail();
        let trail_deposited = colony.deposit_on(&trail);
        let expected: Trail<3> = Trail::new().set(1, 1, 1.0);
        assert_eq!(expected, trail_deposited);
    }

    fn new_3x3_trail() -> Trail<3> {
        return Trail::new();
    }

    #[test]
    fn test_wrap_move() {
        let colony = Slime::new().going(1.0, 1.0).as_colony();
        let trail: Trail<1> = Trail {
            trail: vec![0.0],
        };
        let colony_moved = colony.moved(1.0, &trail);
        let _ = colony_moved.deposit_on(&trail);
    }

    #[test]
    fn test_large_trail() {
        let mut trail: Trail<1024> = Trail::new();
    }

}
