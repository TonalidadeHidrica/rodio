use rodio::source::Source;
use std::io::BufReader;

const THRE: i16 = 3000;

fn sgn(a: i16) -> char {
    match () {
        _ if a > THRE => '^',
        _ if a < -THRE => 'v',
        _ => '-',
    }
}

fn main() {
    let hz: usize = 44100;
    let channel: usize = 2;
    let window = 100;

    let file = std::fs::File::open(std::env::args().skip(1).next().unwrap()).unwrap();
    let decoder = rodio::Decoder::new(BufReader::new(file)).unwrap();
    println!(
        "Original sample rate = {}, Output sample rate = {}",
        decoder.sample_rate(),
        hz
    );

    let mut output = rodio::source::UniformSourceIterator::<_, i16>::new(decoder, channel as u16, hz as u32).step_by(channel);
    let output = output.by_ref();

    let _ = output.take(window).collect::<Vec<_>>();
    for i in 0..=10 {
        let _ = output.take(hz as usize - window * 2).collect::<Vec<_>>();
        println!("{:3} {}", i, output.take(window * 2).map(sgn).collect::<String>());
    }
}
