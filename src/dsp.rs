use statsmodel::lowess;
use ndarray::Array;
use num_traits::sign;
use Option;

const MONO: u32 = 1;
const STEREO: u32 = 2;

// ndarray Documentation for numpy users:
//
// https://docs.rs/ndarray/0.12.1/ndarray/doc/ndarray_for_numpy_users/index.html#shape-and-strides

pub fn size(arr: ndarray::Array::<i64>) -> usize {
    arr.len_of(Axis(0))
}

pub fn channel_count(arr: ndarray::Array) -> usize {
    arr.len_of(Axis(1))
}

fn is_mono(arr: ndarray::Array) -> bool {
    arr.len_of(Axis(1)) == MONO
}

fn is_stereo(arr: ndarray::Array) -> bool {
    arr.len_of(Axis(1)) == STEREO
}

fn is_1d(arr: ndarray::Array) -> bool {
    arr.ndim() == 1
}

// TODO: check syntax and method - "ndarray::arr2(arr);"
fn mono_to_stereo(arr: ndarray::Array) -> ndarray::Array {
    ndarray::arr2(arr);
    assert!(is_stereo(arr));

    arr
}

// TODO: Fix and implement
fn get_max(arr: ndarray) {
    let max = 0;

    let d = arr.ndim();
    for d in arr {
        for i in arr[d] {
            if arr[d][i] > max {
                max = arr[d][i]
            }
        }
    }
}
//
fn count_max_peaks(arr: ndarray::Array) -> (f32, i64) {
    let max_value = sign::abs(arr);
}
// def count_max_peaks(array: np.ndarray) -> (float, int):
//     max_value = np.abs(array).max()
//     max_count = np.count_nonzero(
//         np.logical_or(np.isclose(array, max_value), np.isclose(array, -max_value))
//     )
//     return max_value, max_count

fn lr_to_ms(arr: ndarray::Array) -> (ndarray::Array, ndarray::Array) {

}

fn ms_to_lr(arr: ndarray::Array, arr: ndarray::Array) -> ndarray::Array {

}

fn unfold(arr: ndarray::Array, piece_size: i64, divisions: i64) -> ndarray::Array {

}

fn rms(arr: ndarray::Array) -> f32 {

}

fn batch_rms(arr: ndarray::Array) -> ndarray::Array {

}

fn amplify(arr: ndarray::Array, gain: f32) -> ndarray::Array {

}

fn normalize(
    arr: ndarray::Array, threshold: f32, epsilon: f32, normalize_clipped: bool
) -> (ndarray::Array, f32) {

}

fn smooth_lowess(arr: ndarray::Array, frac: f32, it: i64, delta: f32) -> ndarray::Array {
    
}
// def smooth_lowess(array: np.ndarray, frac: float, it: int, delta: float) -> np.ndarray:
//     return sm.nonparametric.lowess(
//         array, np.linspace(0, 1, len(array)), frac=frac, it=it, delta=delta
//     )[:, 1]

fn clip(arr: ndarray::Array, to: Option<f32> = 1) -> ndarray::Array {

}

pub fn flip(arr: ndarray::Array) -> ndarray::Array {

}

pub fn rectify(arr: ndarray::Array, threshold: f32) -> ndarray::Array {

}

pub fn max_mix(*args) -> ndarray::Array {

}

fn strided_app_2d(matrix: ndarray::Array, batch_size: i64, step: i64) -> ndarray::Array {

}

fn batch_rms_2d(arr: ndarray::Array) -> ndarray::Array {

}

fn fade(arr: ndarray::Array, fade_size: i64) -> ndarray::Array {

}

fn maximum_filter1d(arr: ndarray::Array) -> ndarray::Array{
}

// import numpy as np
// import statsmodels.api as sm

// def count_max_peaks(array: np.ndarray) -> (float, int):
//     max_value = np.abs(array).max()
//     max_count = np.count_nonzero(
//         np.logical_or(np.isclose(array, max_value), np.isclose(array, -max_value))
//     )
//     return max_value, max_count


// def lr_to_ms(array: np.ndarray) -> (np.ndarray, np.ndarray):
//     array = np.copy(array)
//     array[:, 0] += array[:, 1]
//     array[:, 0] *= 0.5
//     mid = np.copy(array[:, 0])
//     array[:, 0] -= array[:, 1]
//     side = np.copy(array[:, 0])
//     return mid, side


// def ms_to_lr(mid_array: np.ndarray, side_array: np.ndarray) -> np.ndarray:
//     return np.vstack((mid_array + side_array, mid_array - side_array)).T


// def unfold(array: np.ndarray, piece_size: int, divisions: int) -> np.ndarray:
//     # (len(array),) -> (divisions, piece_size)
//     return array[: piece_size * divisions].reshape(-1, piece_size)


// def rms(array: np.ndarray) -> float:
//     return np.sqrt(array @ array / array.shape[0])


// def batch_rms(array: np.ndarray) -> np.ndarray:
//     piece_size = array.shape[1]
//     # (divisions, piece_size) -> (divisions, 1, piece_size)
//     multiplicand = array[:, None, :]
//     # (divisions, piece_size) -> (divisions, piece_size, 1)
//     multiplier = array[..., None]
//     return np.sqrt(np.squeeze(multiplicand @ multiplier, axis=(1, 2)) / piece_size)


// def amplify(array: np.ndarray, gain: float) -> np.ndarray:
//     return array * gain


// def normalize(
//     array: np.ndarray, threshold: float, epsilon: float, normalize_clipped: bool
// ) -> (np.ndarray, float):
//     coefficient = 1.0
//     max_value = np.abs(array).max()
//     if max_value < threshold or normalize_clipped:
//         coefficient = max(epsilon, max_value / threshold)
//     return array / coefficient, coefficient


// def smooth_lowess(array: np.ndarray, frac: float, it: int, delta: float) -> np.ndarray:
//     return sm.nonparametric.lowess(
//         array, np.linspace(0, 1, len(array)), frac=frac, it=it, delta=delta
//     )[:, 1]


// def clip(array: np.ndarray, to: float = 1) -> np.ndarray:
//     return np.clip(array, -to, to)


// def flip(array: np.ndarray) -> np.ndarray:
//     return 1.0 - array


// def rectify(array: np.ndarray, threshold: float) -> np.ndarray:
//     rectified = np.abs(array).max(1)
//     rectified[rectified <= threshold] = threshold
//     rectified /= threshold
//     return rectified


// def max_mix(*args) -> np.ndarray:
//     return np.maximum.reduce(args)


// def strided_app_2d(matrix: np.ndarray, batch_size: int, step: int) -> np.ndarray:
//     matrix_length = matrix.shape[0]
//     matrix_width = matrix.shape[1]
//     if batch_size > matrix_length:
//         return np.expand_dims(matrix, axis=0)
//     batch_count = ((matrix_length - batch_size) // step) + 1
//     stride_length, stride_width = matrix.strides
//     return np.lib.stride_tricks.as_strided(
//         matrix,
//         shape=(batch_count, batch_size, matrix_width),
//         strides=(step * stride_length, stride_length, stride_width),
//     )


// def batch_rms_2d(array: np.ndarray) -> np.ndarray:
//     return batch_rms(array.reshape(array.shape[0], array.shape[1] * array.shape[2]))


// def fade(array: np.ndarray, fade_size: int) -> np.ndarray:
//     array = np.copy(array)
//     fade_in = np.linspace(0, 1, fade_size)
//     fade_out = fade_in[::-1]
//     array[:fade_size].T[:] *= fade_in
//     array[size(array) - fade_size :].T[:] *= fade_out
//     return array