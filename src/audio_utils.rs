use std::fmt;
use std::f64::consts::E;
use conditional::conditional; 

fn get_temp_folder() {
    // [?] how would this work on ios device?
}

fn random_str(size: Option<i64>) {
    if size == None { // [?] should I use match size { None => size = 16 } here?
        size = 16;
    }
    // [todo] implement - generate random string of ascii lowercase and digits of k len
}

fn random_file(prefix: Option<&str>, extension: Option<&str>) -> &str {
    if prefix.is_none() {
        let prefix = "";
    }
    if extension.is_none {
        let extension = "wav";
    }
    // Link for conditional macro usage: https://docs.rs/conditional/0.1.1/conditional/macro.conditional.html
    let prefix = conditional!(prefix ? fmt!("{}-", prefix) : prefix);
    return fmt!("{}{}.{}", prefix, random_str(), extension); // [todo] create random_str func
}

fn __to_db_int(float: value) -> float {
    return 20 * math.log10(value); // [todo] fix - do this
}

fn to_db(value: float) -> str {
    return fmt!("{} dB", __to_db_int(value):.4f); // [todo] fix/implement - 4 digits of precision
}

fn ms_to_samples(value: f32, sample_rate: int) -> int{
    return int(value as f64 * sample_rate as f64 * 1E-3);
}

fn make_odd(value: int) -> int {
    return conditional!((value / 2 == 0) ? value + 1 : value); // [todo] check
}

fn time_str(length: int, sample_rate: int) -> str {
    return str(timedelta(seconds = (length as float / sample_rate as float).floor())); // [todo] implement/check/fix
}


// import os
// import random
// import string
// import math
// from datetime import timedelta


// def get_temp_folder(results: list) -> str:
//     first_result_file = results[0].file
//     return os.path.dirname(os.path.abspath(first_result_file))


// def random_str(size: int = 16) -> str:
//     return "".join(random.choices(string.ascii_lowercase + string.digits, k=size))


// def random_file(prefix: str = "", extension: str = "wav") -> str:
//     prefix = f"{prefix}-" if prefix else prefix
//     return f"{prefix}{random_str()}.{extension}"


// def __to_db_int(value: float) -> float:
//     return 20 * math.log10(value)


// def to_db(value: float) -> str:
//     return f"{__to_db_int(value):.4f} dB"


// def ms_to_samples(value: float, sample_rate: int) -> int:
//     return int(sample_rate * value * 1e-3)


// def make_odd(value: int) -> int:
//     return value + 1 if not value & 1 else value


// def time_str(length, sample_rate) -> str:
//     return str(timedelta(seconds=length // sample_rate))