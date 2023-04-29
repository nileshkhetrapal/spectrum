use sonogram::{rectangular, ColourGradient, SpecOptionsBuilder};
use std::io::Read;
use std::path::Path;

use rayon::iter::IntoParallelIterator;
use rayon::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::{env, fs};

fn create_spectrogram(input_file_path: &str, output_file_path: &str) {
    let mut data: Vec<u8> = Vec::new();
    let e_path = Path::new(output_file_path);
    if e_path.exists() {
        return;
    }
    let Ok(mut file) = std::fs::File::open(input_file_path) else {return};
    file.read_to_end(&mut data).unwrap();
    let freq: Vec<i16> = data
        .chunks(2)
        .map(|c| {
            if c.len() == 1 {
                // If the chunk is only one byte, pad it with a zero byte.
                i16::from_le_bytes([c[0], 0])
            } else {
                // Otherwise, convert the two bytes to an i16.
                i16::from_le_bytes([c[0], c[1]])
            }
        })
        .collect::<Vec<i16>>();

    // Set up spectrogram options using the `new` function.
    std::panic::catch_unwind(|| {
        let spec = SpecOptionsBuilder::new(512)
            .load_data_from_memory(freq, 44000)
            .set_window_fn(rectangular)
            .build()
            .map_err(|e| println!("{:?}", e));
        let Ok(mut spec) = spec else {return};
        let mut spec = spec.compute();
        spec.to_png(
            e_path,
            sonogram::FrequencyScale::Linear,
            &mut ColourGradient::audacity_theme(),
            500,
            500,
        )
        .unwrap();
        println!("Generated spectrogram {e_path:?}");
    })
    .map_err(|e| println!("{:?}", e));
}

fn process_exe_files(path: &str, output_path: &str) {
    let files = fs::read_dir(path).expect("Failed to read path");
    let files: Vec<_> = files.collect();
    files.into_par_iter().for_each(|file| {
        let file_path = file.unwrap().path();
        let file_name = file_path.file_name().unwrap().to_str().unwrap().to_owned();

        if file_path.extension().map(|s| s.to_str().unwrap()) == Some("exe") {
            let filename = file_name.split(".").next().unwrap().to_owned();
            let png_file_path = Path::new(&output_path).join(&format!("{}.png", filename));

            if !png_file_path.exists() {
                create_spectrogram(
                    file_path.as_os_str().to_str().unwrap(),
                    png_file_path.as_os_str().to_str().unwrap(),
                );
            }
        }
    });
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <input_file_path> <output_file_path>", args[0]);
        std::process::exit(1);
    }
    let path = &args[1];
    let output_path = &args[2];
    process_exe_files(path, output_path);
}
