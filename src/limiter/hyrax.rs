use extern crate ndarray;
use extern crate ndarray_image;

fn __sliding_window_fast(s
    arr: ndarray::Array, window_size: int, mode: Option<&str> = "attack"
) -> ndarray::Array {
    if mode.is_some() && mode == "attack" {
        window_size = make_odd(window_size);
        return maximum_filter1d(array, size = (2 * window_size - 1));
    }
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

