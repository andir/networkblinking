use std::time::{Duration, Instant};
use std::thread::sleep;
use std::net::UdpSocket;
use std::convert::AsRef;
struct Pixel {
    h: f32,
    s: f32,
    v: f32,
}

impl Pixel {
    fn size() -> usize {
        32
    }
    fn serialize(&self) -> [u8; 32] {
        let h: [u8; 4] = unsafe { std::mem::transmute(self.h) };
        let s: [u8; 4] = unsafe { std::mem::transmute(self.s) };
        let v: [u8; 4] = unsafe { std::mem::transmute(self.v) };
        let mut arr = [0u8; 32];
        for (place, element) in arr.iter_mut().zip(
            vec![h, s, v].iter().flat_map(|s| s.iter()),
        )
        {
            *place = *element;
        }
        arr
    }
}

struct Frame {
    pixels: Vec<Pixel>,
}

impl Frame {
    fn size(&self) -> usize {
        return self.pixels.len() * Pixel::size();
    }
    fn serialize(&self) -> std::vec::Vec<u8> {
        let s = self.size();
        let mut v: std::vec::Vec<u8> = vec![0; s];
        let pixelflut: std::vec::Vec<[u8; 32]> =
            self.pixels.iter().map(|p| p.serialize()).collect();
        for (place, element) in v.iter_mut().zip(pixelflut.iter().flat_map(|s| s.iter())) {
            *place = *element
        }
        v
    }
}

fn main() {

    let target_time = Duration::new(0, ((1000 * 1000 * 1000) / 1000));
    let mut i = 0;
    loop {
        let mut pixels: std::vec::Vec<Pixel> = vec![];
        for n in 1..2000 {
            pixels.push(Pixel {
                h: 0.0,
                s: 0.0,
                v: 0.0,
            });
        }        let frame = Frame { pixels: pixels };
        let encoded: std::boxed::Box<[u8]> = frame.serialize().into_boxed_slice();
        let mut socket = UdpSocket::bind("0.0.0.0:0").expect("couldn't bind to address");
        let data: &[u8] = encoded.as_ref();
        let now = Instant::now();
        socket.send_to(data, "127.0.0.1:1337").expect(
            "couldn't send data",
        );
        if i % 30 == 0 {
            println!("frame: {}", i);
            println!("time: {:?}", now);
        }
        i += 1;
        let elapsed = now.elapsed();
//        let diff = target_time - elapsed;
//        std::thread::sleep(diff);
        if i == 5000 {
            break;
        }
    }
}
