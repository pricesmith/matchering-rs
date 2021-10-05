use std::any::type_name;

// Returns type of variable. 
pub fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

// When the `console_error_panic_hook` feature is enabled, we can call the
// `set_panic_hook` function at least once during initialization, and then
// we will get better error messages if our code ever panics.
pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}