use serde::Deserialize;
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};

#[derive(Deserialize)]
struct Subtitle {
    startMs: f64,
    durationMs: f64,
    text: String,
}

#[derive(Deserialize)]
struct Subtitles {
    subtitles: Vec<Subtitle>,
}

fn convert_to_srt_time_format(seconds: f64) -> String {
    let hours = (seconds / 3600.0).floor() as u32;
    let minutes = ((seconds % 3600.0) / 60.0).floor() as u32;
    let whole_seconds = seconds % 60.0;
    let milliseconds = ((whole_seconds - whole_seconds.floor()) * 1000.0).floor() as u32;

    format!("{:02}:{:02}:{:02},{:03}", hours, minutes, whole_seconds.floor() as u32, milliseconds)
}

fn json_to_srt(json_data: &Subtitles) -> String {
    let average_subtitle_length = 150;
    let initial_capacity = json_data.subtitles.len() * average_subtitle_length;
    let mut srt_content = String::with_capacity(initial_capacity);

    for (index, entry) in json_data.subtitles.iter().enumerate() {
        let start_time = entry.startMs / 1000.0;
        let end_time = (entry.startMs + entry.durationMs) / 1000.0;

        srt_content.push_str(&format!(
            "{}\n{} --> {}\n{}\n\n",
            index + 1,
            convert_to_srt_time_format(start_time),
            convert_to_srt_time_format(end_time),
            entry.text
        ));
    }

    srt_content.trim_end().to_string()
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: json_to_srt <input_file>");
        std::process::exit(1);
    }
    let input_file_path = &args[1];

    let mut file = File::open(input_file_path)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    let json_data: Subtitles = serde_json::from_str(&data)?;

    let srt_data = json_to_srt(&json_data);

    let output_file_path = "output.srt";
    let mut output_file = File::create(output_file_path)?;
    output_file.write_all(srt_data.as_bytes())?;

    println!("SRT content saved to {}", output_file_path);

    Ok(())
}
