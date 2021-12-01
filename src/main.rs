use std::ops;

fn main() {
    println!("Hello, world!");
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

struct Trail<const SIZE: usize> {
    trail: [[f32; SIZE]; SIZE],
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

    fn updated(&self, time: f32) -> Slime {
        return Slime {
            pos: self.pos + self.vel * time,
            vel: self.vel,
        };
    }

    fn steered<const TRAIL_SIZE: usize>(&self, trail: &Trail<TRAIL_SIZE>) -> Slime {
        let l_vel = self.vel.rotated(-45.0_f32.to_radians());
        let r_vel = self.vel.rotated(45.0_f32.to_radians());

        let f_coord = self.pos + self.vel;
        let l_coord = self.pos + l_vel;
        let r_coord = self.pos + r_vel;

        let f_val = trail.trail[f_coord.x as usize][f_coord.y as usize];
        let l_val = trail.trail[l_coord.x as usize][l_coord.y as usize];
        let r_val = trail.trail[r_coord.x as usize][r_coord.y as usize];

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

    fn moved(&self, time: f32) -> Colony<SIZE> {
        return Colony {
            colony: self.colony.map(|s| s.updated(time)),
        };
    }

    fn steered<const TRAIL_SIZE: usize>(&self, trail: &Trail<TRAIL_SIZE>) -> Colony<SIZE> {
        return Colony {
            colony: self.colony.map(|s| s.steered(trail)),
        };
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
        let colony_updated = colony.moved(1.0);
        let expected = Slime::new().at(0.0, 1.0).going(0.0, 1.0).as_colony();
        assert_eq!(expected, colony_updated);
    }

    #[test]
    fn colony_update_half() {
        let colony = Slime::new().going(1.0, 1.0).as_colony();
        let colony_updated = colony.moved(0.5);
        let expected = Slime::new().at(0.5, 0.5).going(1.0, 1.0).as_colony();
        assert_eq!(expected, colony_updated);
    }

    #[test]
    fn colony_steer_forward() {
        let colony = Slime::new().going(1.0, 1.0).as_colony();
        let trail = Trail {
            trail: [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]],
        };
        let colony_updated = colony.steered(&trail);
        let expected = Slime::new().going(1.0, 1.0).as_colony();
        assert_eq!(expected, colony_updated);
    }

    #[test]
    fn colony_steer_left() {
        let colony = Slime::new().going(1.0, 1.0).as_colony();
        let trail = Trail {
            trail: [[0.0, 0.0, 0.0], [0.1, 0.0, 0.0], [0.0, 0.0, 0.0]],
        };
        let colony_updated = colony.steered(&trail);
        let expected = Slime::new().going(2.0_f32.sqrt(), 0.0).as_colony();
        assert_eq!(expected, colony_updated);
    }

    #[test]
    fn colony_steer_right() {
        let colony = Slime::new().going(1.0, 1.0).as_colony();
        let trail = Trail {
            trail: [[0.0, 0.1, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]],
        };
        let colony_updated = colony.steered(&trail);
        let expected = Slime::new().going(0.0, 2.0_f32.sqrt()).as_colony();
        assert_eq!(expected, colony_updated);
    }
}
