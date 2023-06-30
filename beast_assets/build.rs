use std::{path::Path, fs::{create_dir, remove_dir_all, self}, env};
use glob::glob;
fn main() {
    //"ffmpeg -i .\source.mp4 -vf scale=-1:720 'data/video.webp' -i .\source.mp4 -vn data/audio.ogg";
    println!("cargo:rerun-if-changed=source.mp4");
    println!("cargo:rerun-if-changed=build.rs");
    let data_path = Path::new(&env::var_os("OUT_DIR").unwrap()).join("beast_data");
    if !data_path.exists() {
        create_dir(&data_path).unwrap();
    } else {
        remove_dir_all(&data_path).unwrap();
        create_dir(&data_path).unwrap();
    }
    let video_file_path = data_path.join("f%04d.jpg");
    let audio_file_path = data_path.join("audio.ogg");
    let frames_rs = data_path.join("frames.rs");
    let args = ["-i", "source.mp4", "-vf", "scale=-1:480","-qscale:v", "2", video_file_path.as_os_str().to_str().unwrap(), "-i", "source.mp4", "-vn", audio_file_path.as_os_str().to_str().unwrap()];
    let status = std::process::Command::new("ffmpeg")
        .args(args)
        .status()
        .expect("failed to execute process");
    if !status.success() {
        panic!("ffmpeg failed");
    }
    //fs::write(video_out_file, bincode::DefaultOptions::default().serialize(&process_video(&video_file_path)).unwrap()).unwrap();
    let frames: Vec<_> = glob(data_path.join("*.jpg").as_os_str().to_str().unwrap()).expect("failed to read pattern").filter_map(|e| e.ok()).collect();
    let frame_count = frames.len();
    let frame_data: Vec<_> = frames.into_iter().map(|e| {
        format!("include_bytes!(\"{}\")", e.as_os_str().to_str().unwrap().replace("\\", "\\\\"))
    }).collect();
    let framesrs = format!(r#"
        const FRAMES: [&'static [u8]; {}] = [{}];
    "#, frame_count, frame_data.join(",\n"));
    fs::write(frames_rs, &framesrs).expect("Failed to write frames.rs");
}

