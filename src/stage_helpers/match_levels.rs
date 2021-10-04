use extern crate ndarray;


fn __average_fft(
    loudest_pieces: ndarray::Array, sample_rate: i64, fft_size: i64
) -> ndarray::Array {
    *_, specs = stft( // Calculate Short Time Fourier Transform
        loudest_pieces,
        sample_rate,
        window="boxcar", // [todo] implement default values
        nperseg=fft_size,
        noverlap=0,
        boundary=None,
        padded=False,
    )
    return specs.abs().mean((0, 2));
}

// [todo]
fn __smooth_exponentially(matching_fft: ndarray::Array, config: Config) -> ndarray::Array {
    let grid_linear = (
        config.internal_sample_rate * 0.5 * ndarray::linspace(0, 1, config.fft_size // 2 + 1) // [todo] fix
    )

    let grid_logarithmic = (
        config.internal_sample_rate
        * 0.5
        * np.logspace(
            np.log10(4 / config.fft_size),
            0,
            (config.fft_size // 2) * config.lin_log_oversampling + 1,
        )
    )

    let interpolator = interpolate.interp1d(grid_linear, matching_fft, "cubic")
    let matching_fft_log = interpolator(grid_logarithmic)

    let matching_fft_log_filtered = smooth_lowess(
        matching_fft_log, config.lowess_frac, config.lowess_it, config.lowess_delta
    )

    let interpolator = interpolate.interp1d(
        grid_logarithmic, matching_fft_log_filtered, "cubic", fill_value="extrapolate"
    )
    let matching_fft_filtered = interpolator(grid_linear)

    matching_fft_filtered[0] = 0
    matching_fft_filtered[1] = matching_fft[1]

    return matching_fft_filtered
}