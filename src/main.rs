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

#[derive(Debug, PartialEq, Copy, Clone)]
struct Slime {
    pos: Vec2f,
    vel: Vec2f,
}

impl Slime {
    fn updated(&self, time: f32) -> Slime {
        return Slime {
            pos: self.pos + self.vel * time,
            vel: self.vel,
        };
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Colony<const SIZE: usize> {
    colony: [Slime; SIZE],
}

impl<const SIZE: usize> Colony<SIZE> {
    fn updated(&self, time: f32) -> Colony<SIZE> {
        return Colony {
            colony: self.colony.map(|s| s.updated(time)),
        };
    }
}

#[cfg(test)]
mod slime_tests {

    use super::*;

    #[test]
    fn colony_update() {
        let colony = Colony {
            colony: [Slime {
                pos: Vec2f { x: 0.0, y: 0.0 },
                vel: Vec2f { x: 0.0, y: 1.0 },
            }],
        };
        let colony_updated = colony.updated(1.0);
        let expected = Colony {
            colony: [Slime {
                pos: Vec2f { x: 0.0, y: 1.0 },
                vel: Vec2f { x: 0.0, y: 1.0 },
            }],
        };
        assert_eq!(expected, colony_updated);
    }
}
