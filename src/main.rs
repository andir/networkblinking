extern crate bytes;
use std::time::{Duration, Instant};
use std::thread::sleep;
use std::net::UdpSocket;
use std::convert::AsRef;
use std::io::Write;
use bytes::{BytesMut, BufMut, BigEndian, Bytes};
use bytes::buf::Iter;

mod blinking;

pub struct Frame {
    buffer: BytesMut,
    capacity: usize,
}


impl Frame {
    fn new(size: usize) -> Frame {
        let capacity = size * (3 * 4);
        println!("capacity: {}", capacity);
        Frame {
            capacity: capacity,
            buffer: BytesMut::with_capacity(capacity),
        }
    }

    fn write(&mut self, p: Pixel) {
        self.write_f32(p.h);
        self.write_f32(p.s);
        self.write_f32(p.v);
    }

    fn write_f32(&mut self, value: f32) {
        self.buffer.put_f32::<BigEndian>(value);
    }

    fn take(&mut self) -> &BytesMut {
        let b = &self.buffer;
        b
    }
    fn reset(&mut self) {
        unsafe {
            self.buffer.set_len(0);
        }
    }
}

pub trait Effect {
    fn at(&mut self, u: usize) -> Pixel;
}

pub struct Pixel {
    h: f32,
    s: f32,
    v: f32,
}

fn main() {
    let pixel_count = 120;
    let mut socket = UdpSocket::bind("0.0.0.0:0").expect("couldn't bind to address");
    let target_time = Duration::new(0, ((1000 * 1000 * 1000) / 30));
    let mut i = 0;

    let mut frame = Frame::new(pixel_count);
    let mut tot = Duration::new(0, 0);
    let mut eff = blinking::Blinking{};
    loop {
        frame.reset();
        for n in 1..pixel_count {
            frame.write(eff.at(n));
        }

        let bytes = frame.take();
        let now = Instant::now();
        socket.send_to(bytes.as_ref(), "172.23.97.151:1337").expect(
            "couldn't send data",
        );

        i += 1;
        let elapsed = now.elapsed();
        let diff = target_time - elapsed;
        std::thread::sleep(diff);
        if (i % 100 == 0) {
            println!("frame time: {:?}", tot / i);
        }
        tot += elapsed;
        //        if i == 5000 {
        //            break;
        //        }
    }
}
