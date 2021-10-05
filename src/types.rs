extern crate ndarray;

use ndarray;

pub struct AudioFile {
    ndarray: ndarray::Array,
    sample_rate: u32,
}

pub struct Sys {
    temp_folder: &str
}