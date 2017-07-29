use {Effect, Pixel};
extern crate rand;


pub struct Blinking {}

impl Effect for Blinking {
    fn at(&mut self, u: usize) -> Pixel {
        if rand::random::<bool>() {
            Pixel {
                h: 120.0,
                s: 1.0,
                v: 1.0,
            }
        } else {
            Pixel {
                h: 0.0,
                s: 1.0,
                v: 1.0,
            }
        }
    }
}
