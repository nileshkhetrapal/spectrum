# Spectrum Spectrogram Generator

This Rust code is part of the Spectrum project, which is hosted on Hugging Face at the following links:

* [Spectrum-Dataset](https://huggingface.co/datasets/nilekhet/Spectrum-Dataset)
* [Spectrum](https://huggingface.co/nilekhet/Spectrum)

The goal of the Spectrum project is to analyze the spectral properties of executable files for malware analysis purposes. This Rust code generates spectrograms from the input executable files and saves them as images.

## Overview

The main functionality of this code is to read an input directory containing executable files, generate spectrograms for each file, and save the resulting images in the specified output directory.

The main functions in this code are:

* `create_spectrogram`: Generates a spectrogram for a given input file and saves it in the specified output path.
* `process_exe_files`: Processes all the executable files in a given input directory and calls `create_spectrogram` for each file.
* `main`: Parses command-line arguments, validates their format, and calls `process_exe_files`.

## Usage

To use this code, compile and run it with the following command-line arguments:

phpCopy code

`$ cargo run <input_file_path> <output_file_path>`

* `<input_file_path>`: The path to the directory containing the executable files to be processed.
* `<output_file_path>`: The path to the directory where the generated spectrograms will be saved.

## Dependencies

This code depends on the following libraries:

* `sonogram`: For generating spectrograms.
* `rayon`: For parallel processing of files.
* `std`: For filesystem operations and input/output.
