fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

use Option;
use std::any::type_name;
use std::f64::consts::E;
use conditional::conditional; 
use more_asserts;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

// [Todo] Change conditionals checks to assertions
// [Todo] Change conditionals to terinary conditionals

pub struct LimiterConfig {
    attack: f32,
    hold: f32,
    release: f32,
    attack_filter_coefficient: f32,
    hold_filter_order: u32,
    hold_filter_coefficient: f32,
    release_filter_order: u32,
    release_filter_coefficient: f32
}

impl LimiterConfig {
    pub fn new(
        &self, // [Todo] check - &mut self??
        attack: Option<f32>,
        hold: Option<f32>,
        release: Option<f32>,
        attack_filter_coefficient: Option<f32>,
        hold_filter_order: Option<u32>,
        hold_filter_coefficient: Option<f32>,
        release_filter_order: Option<u32>,
        release_filter_coefficient: Option<f32>
    ) {
        // set attack
        if attack.is_none() || attack < 0 {
            attack = 1;
        }
        self.attack = attack;

        // set hold
        if hold.is_none() || hold < 0 {
            attack = 1;
        }
        self.hold = hold;

        // set release
        if release.is_none(release) || release < 0 {
            let release = 3000;
        }
        self.release = release;

        // set attack_filter_coefficient
        if attack_filter_coefficient.is_none() {
            let attack_filter_coefficient = -2;
        }
        self.attack_filter_coefficient = attack_filter_coefficient;

        // set hold_filter_order
        if hold_filter_order.is_none() || hold_filter_order < 0 {
            let hold_filter_order = 1;
        }
        self.hold_filter_order = hold_filter_order;

        // set hold_filter_coeffiicient
        if hold_filter_coefficient.is_none() {
            let hold_filter_coefficient = 1;
        } 
        self.hold_filter_coefficient = hold_filter_coefficient;

        // set release_filter_order
        if release_filter_order.is_none() || release_filter_order < 0 {
            let release_filter_order = 1;
        }
        self.release_filter_order = release_filter_order;

        // set release_filter_coefficient
        if release_filter_coefficient.is_none() {
            let release_filter_coefficient = 800;
        }
        self.release_filter_coefficient = release_filter_coefficient;

        return self;
    }
}

pub struct Config {
        internal_sample_rate: u32,
        max_length: f32,
        max_piece_size: f32,
        threshold: f32,
        min_value: f32,
        fft_size: u32,
        lin_log_oversampling: u32,
        rms_correction_steps: u32,
        clipping_samples_threshold: u32,
        limited_samples_threshold: u32,
        allow_equality: bool,,
        lowess_frac: f32,
        lowess_it: u32,
        lowess_delta: f32,
        preview_size: f32,
        preview_analysis_step: f32,
        preview_fade_size: f32,
        preview_fade_coefficient: f32,
        temp_folder: &str,
        limiter: LimiterConfig,
}

// [todo] fix - reorder methods as variables are called vs initialized
impl Config {
    pub fn new(
        &self, // [todo] check - &mut self???
        internal_sample_rate: Option<u32>,
        max_length: Option<f32>,
        max_piece_size: Option<f32>,
        threshold: Option<f32>,
        min_value: Option<f32>,
        fft_size: Option<u32>,
        lin_log_oversampling: Option<u32>,
        rms_correction_steps: Option<u32>,
        clipping_samples_threshold: Option<u32>,
        limited_samples_threshold: Option<u32>,
        allow_equality: Option<bool>,
        lowess_frac: Option<f32>,
        lowess_it: Option<u32>,
        lowess_delta: Option<f32>,
        preview_size: Option<f32>,
        preview_analysis_step: Option<f32>,
        preview_fade_size: Option<f32>,
        preview_fade_coefficient: Option<f32>,
        temp_folder: Option<&str>,
        limiter: LimiterConfig
    ) {
        // check and set internal_sample_rate
        if internal_sample_rate.isNone() {
            let internal_sample_rate = 4100;
        } else if internal_sample_rate != 4100 {
            // [todo] add - console warning about other internal sample rates not being tested
        }
        self.internal_sample_rate = internal_sample_rate;

        // check and set max_length
        if max_length.isNone() || max_length < 0 {
            let max_length = 15 * 60;
        }
        assert_gt!(max_length > (fft_size / internal_sample_rate));
        self.max_length = max_length;

        // check and set max_piece_size
        if max_piece_size.isNone() || max_piece_size < 0 {
            let max_piece_size = 15;
        }
        assert_gt!(max_piece_size > (fft_size / internal_sample_rate));
        assert_gt!(max_piece_size, max_length);
        self.max_piece_size = max_piece_size * internal_sample_rate;

        // check and set threshold
        if threshold.isNone() || threshold < 0 {
            let threshold = ((2 ** 15 - 61) / 2 ** 15);
        }
        assert_gt!(threshold > min_value);
        assert_lt!(threshold < 1);
        self.threshold = threshold;

        // check and set min_value
        if min_value.isNone() || min_value < 0 {
            let min_value = 1E-6;
        }
        assert_lt!(min_value < 0.1);
        self.min_value = min_value;

        // check and set fft_size
        if fft_size.isNone() || fft_size < 1 {
            let fft_size = 4096;
        }
        self.fft_size = fft_size;

        // check and set lin_log_oversampling
        if lin_log_oversampling.isNone() {
            let lin_log_oversampling = 4;
        }
        assert_gt!(lin_log_oversampling, 0);
        self.lin_log_oversampling = lin_log_oversampling;

        // check and set rms_correction_steps
        if rms_correction_steps.isNone() {
            assert_gt!(rms_correction_steps, 0);
            let rms_correction_steps = 4;
        }
        self.rms_correction_steps = rms_correction_steps;

        // 44. check limited_samples_threshold
        if limited_samples_threshold.isNone() || limited_samples_threshold < 0 {
            let limited_samples_threshold = 128;
        }
        assert_gt!(limited_samples_threshold, 0);

        // 45. check and set clipping_samples_threshold
        if clipping_samples_threshold.isNone() || clipping_samples_threshold <= 0 {
            let clipping_samples_threshold = 8;
        }
        assert_gt!(limited_samples_threshold, clipping_samples_threshold);

        // set clipping_samples_threshold and limited_samples_threshold
        self.clipping_samples_threshold = clipping_samples_threshold;
        self.limited_samples_threshold = limited_samples_threshold;

        // check and set allow_equality
        

        // allow_equality: bool = False,
        // lowess_frac: float = 0.0375,
        // lowess_it: int = 0,
        // lowess_delta: float = 0.001,
        // preview_size: float = 30,
        // preview_analysis_step: float = 5,
        // preview_fade_size: float = 1,
        // preview_fade_coefficient: float = 8,
        // temp_folder: str = None,
        // limiter: LimiterConfig = LimiterConfig(),

    }
}


// import math
// from .log import debug

// class Config:

//         assert isinstance(allow_equality, bool)
//         self.allow_equality = allow_equality

//         assert lowess_frac > 0
//         assert lowess_it >= 0
//         assert lowess_delta >= 0
//         assert isinstance(lowess_it, int)
//         self.lowess_frac = lowess_frac
//         self.lowess_it = lowess_it
//         self.lowess_delta = lowess_delta

//         assert preview_size > 5
//         assert preview_analysis_step > 1
//         assert preview_fade_size > 0
//         assert preview_fade_coefficient >= 2
//         self.preview_size = preview_size * internal_sample_rate
//         self.preview_analysis_step = preview_analysis_step * internal_sample_rate
//         self.preview_fade_size = preview_fade_size * internal_sample_rate
//         self.preview_fade_coefficient = preview_fade_coefficient

//         assert temp_folder is None or isinstance(temp_folder, str)
//         self.temp_folder = temp_folder

//         assert isinstance(limiter, LimiterConfig)
//         self.limiter = limiter