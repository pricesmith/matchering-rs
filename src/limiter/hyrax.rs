use ndarray::Array;
use ndarray_image;
use num_traits::sign;
use more_asserts::{self, assert_ge};

// TODO: lazy implementation -- probably wrong
fn normalize_axis_index(axis: u32, arr: Array) {
    sign::abs(axis)
}

fn maximum_filter1d(
    input: ndarray::Array, 
    size: u32, 
    axis: u32, 
    output: Optional<ndarray::Array>,
    mode: &str,
    cval: f64,
    origin: i32
) -> i64 {
    // TODO: Check and set optionals
    //
    //
    assert_ge!(axis, 0);
    assert!(-size <= axis <= size);
    normalize_axis_index(axis, input.ndim())
    // if size < 1:
    // raise RuntimeError('incorrect filter size')

    
}

// def maximum_filter1d(input, size, axis=-1, output=None,
//     mode="reflect", cval=0.0, origin=0):

// output = _ni_support._get_output(output, input)
// if (size // 2 + origin < 0) or (size // 2 + origin >= size):
// raise ValueError('invalid origin')
// mode = _ni_support._extend_mode_to_code(mode)
// _nd_image.min_or_max_filter1d(input, size, axis, output, mode, cval,
//                  origin, 0)
// return output

// def maximum_filter1d(input, size, axis=-1, output=None,
//     mode="reflect", cval=0.0, origin=0):
// Maximum-filtered array with same shape as input.
// None if `output` is not None
// Notes
// -----
// This function implements the MAXLIST algorithm [1]_, as described by
// Richard Harter [2]_, and has a guaranteed O(n) performance, `n` being
// the `input` length, regardless of filter size.
// References
// ----------
// .. [1] http://citeseerx.ist.psu.edu/viewdoc/summary?doi=10.1.1.42.2777
// .. [2] http://www.richardhartersworld.com/cri/2001/slidingmin.html
// Examples
// --------
// >>> from scipy.ndimage import maximum_filter1d
// >>> maximum_filter1d([2, 8, 0, 4, 1, 9, 9, 0], size=3)
// array([8, 8, 8, 4, 9, 9, 9, 9])
// """
// input = numpy.asarray(input)
// if numpy.iscomplexobj(input):
// raise TypeError('Complex type not supported')
// axis = normalize_axis_index(axis, input.ndim)
// if size < 1:
// raise RuntimeError('incorrect filter size')
// output = _ni_support._get_output(output, input)
// if (size // 2 + origin < 0) or (size // 2 + origin >= size):
// raise ValueError('invalid origin')
// mode = _ni_support._extend_mode_to_code(mode)
// _nd_image.min_or_max_filter1d(input, size, axis, output, mode, cval,
//                  origin, 0)
// return output

// TODO: fix!
fn __sliding_window_fast(s
    arr: ndarray::Array, window_size: int, mode: Option<&str>
) -> ndarray::Array {
    if mode.is_none() {
        let mode = "attack";
    }
    window_size = make_odd(window_size);
    // maximum_filter1d(array, size = (2 * window_size - 1))
}


// import numpy as np
// import math
// from scipy import signal
// from scipy.ndimage.filters import maximum_filter1d // https://stackoverflow.com/questions/66806152/how-does-maximum-filter1d-work-in-scipy-how-does-cval-origin-mode-parameter-a

// from .. import Config
// from ..log import debug
// from ..dsp import rectify, flip, max_mix
// from ..utils import make_odd, ms_to_samples


// def __sliding_window_fast(
//     array: np.ndarray, window_size: int, mode: str = "attack"
// ) -> np.ndarray:
//     if mode == "attack":
//         window_size = make_odd(window_size)
//         return maximum_filter1d(array, size=(2 * window_size - 1))
//     half_window_size = (window_size - 1) // 2
//     array = np.pad(array, (half_window_size, 0))
//     return maximum_filter1d(array, size=window_size)[:-half_window_size]


// def __process_attack(array: np.ndarray, config: Config) -> (np.ndarray, np.ndarray):
//     attack = ms_to_samples(config.limiter.attack, config.internal_sample_rate)

//     slided_input = __sliding_window_fast(array, attack, mode="attack")

//     coef = math.exp(config.limiter.attack_filter_coefficient / attack)
//     b = [1 - coef]
//     a = [1, -coef]
//     output = signal.filtfilt(b, a, slided_input)

//     return output, slided_input


// def __process_release(array: np.ndarray, config: Config) -> np.ndarray:
//     hold = ms_to_samples(config.limiter.hold, config.internal_sample_rate)

//     slided_input = __sliding_window_fast(array, hold, mode="hold")

//     b, a = signal.butter(
//         config.limiter.hold_filter_order,
//         config.limiter.hold_filter_coefficient,
//         fs=config.internal_sample_rate,
//     )
//     hold_output = signal.lfilter(b, a, slided_input)

//     b, a = signal.butter(
//         config.limiter.release_filter_order,
//         config.limiter.release_filter_coefficient / config.limiter.release,
//         fs=config.internal_sample_rate,
//     )
//     release_output = signal.lfilter(b, a, np.maximum(slided_input, hold_output))

//     return np.maximum(hold_output, release_output)


// def limit(array: np.ndarray, config: Config) -> np.ndarray:

//     debug("The limiter is started. Preparing the gain envelope...")
//     rectified = rectify(array, config.threshold)

//     if np.all(np.isclose(rectified, 1.0)):
//         debug("The limiter is not needed!")
//         return array

//     gain_hard_clip = flip(1.0 / rectified)
//     debug("Modifying the gain envelope: attack stage...")
//     gain_attack, gain_hard_clip_slided = __process_attack(
//         np.copy(gain_hard_clip), config
//     )

//     debug("Modifying the gain envelope: hold / release stage...")
//     gain_release = __process_release(np.copy(gain_hard_clip_slided), config)

//     debug("Finalizing the gain envelope...")
//     gain = flip(max_mix(gain_hard_clip, gain_attack, gain_release))

//     return array * gain[:, None]

