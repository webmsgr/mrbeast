use std::{io::Cursor, sync::Mutex};

use lazy_static::lazy_static;
use image::{codecs::jpeg::JpegDecoder, ImageDecoder, ColorType};
use rayon::prelude::*;
use rodio::Decoder;
include!(concat!(env!("OUT_DIR"), "/beast_data/frames.rs"));

lazy_static! {
    pub static ref VIDEO: ((u32,u32), Vec<Vec<u32>>) = process_video();
    pub static ref AUDIO: Mutex<Option<Decoder<Cursor<&'static [u8]>>>> = Mutex::new(Some(Decoder::new_vorbis(Cursor::new(include_bytes!(concat!(env!("OUT_DIR"), "/beast_data/audio.ogg")) as &[u8])).unwrap()));
}


pub fn load() {
    lazy_static::initialize(&VIDEO);
    lazy_static::initialize(&AUDIO);
}

fn process_video() -> ((u32,u32), Vec<Vec<u32>>) {
    let dim = JpegDecoder::new(FRAMES[0]).unwrap().dimensions();
    let frames = FRAMES.par_iter().map(|f| {
        let decoder = JpegDecoder::new(*f).unwrap();
        let mut buf = vec![0u8; decoder.total_bytes() as usize];
        assert_eq!(decoder.color_type(), ColorType::Rgb8);
        decoder.read_image(&mut buf).unwrap();
        let mut pixel_data = Vec::with_capacity(buf.len()/3);
        for pixel in buf.chunks_exact(3) {
            let (red,green,blue): (u32,u32,u32) = (pixel[0].into(), pixel[1].into(), pixel[2].into());
            let pixel = blue | (green << 8) | (red << 16);
            pixel_data.push(pixel);
        }
        pixel_data
    }).collect();
    (dim,frames)
}