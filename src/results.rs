// import os
// import soundfile as sf

pub struct Result {
    file: &str,
    subtype: &str,
    use_limiter: Option<bool>,
    normalize: Option<bool>
}

impl Result {
    pub fn new(file: &str, subtype: &str, use_limiter: Option<bool>, normalize: Option<bool>) {
        // 1. get file extension
        //

        // 2. check if format is supported
        //

        // 3. assign internal state values
        Result.file = file;
        Result.subtype = subtype;

        // [todo] change/fix - add options and change to ternary conditionals
        if use_limiter.is_some() {
            Result.use_limiter = use_limiter;
        } else {
            Result.use_limiter = true;
        }

        if normalize.is_some() {
            Result.normalize = normalize;
        } else {
            Result.normalize = true;
        }
    }

    pub fn pcm16(file: &str) -> Result {
        return Result.new(file, "PCM_16");
    }

    pub fn pcm24(file: &str) -> Result {
        return Result.new(file, "PCM_24");
    }
}


// class Result:
//     def __init__(
//         self, file: str, subtype: str, use_limiter: bool = True, normalize: bool = True
//     ):
//         _, file_ext = os.path.splitext(file)
//         file_ext = file_ext[1:].upper()
//         if not sf.check_format(file_ext):
//             raise TypeError(f"{file_ext} format is not supported")
//         if not sf.check_format(file_ext, subtype):
//             raise TypeError(f"{file_ext} format does not have {subtype} subtype")
//         self.file = file
//         self.subtype = subtype
//         self.use_limiter = use_limiter
//         self.normalize = normalize