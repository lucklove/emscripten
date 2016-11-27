#![allow(non_camel_case_types)]

extern crate libc;
extern crate emscripten_sys as sys;

mod utils {
    pub fn make_c_style_string(rs_str: &str) -> String {
        // The end of str should be '\0' because C string always end with '\0'
        let mut s = rs_str.to_owned();
        match s.pop() {
            Some('\0') => (),
            Some(c) => s.push(c),
            None => (),
        }
        s.push('\0');
        s
    }
}

pub mod em {
    use libc::*;

    pub use super::sys::{EM_TIMING_SETTIMEOUT, EM_TIMING_RAF, EM_TIMING_SETIMMEDIATE};

    pub fn run_script(script: &str) {
        let script = super::utils::make_c_style_string(script);
        unsafe {
            super::sys::emscripten_run_script(script.as_ptr());
        }
    }

    pub fn run_script_int(script: &str) -> i32 {
        let script = super::utils::make_c_style_string(script);
        unsafe {
            super::sys::emscripten_run_script_int(script.as_ptr())
        }
    }

    pub fn run_script_string(script: &str) -> String {
        let script = super::utils::make_c_style_string(script);
        unsafe {
            let ptr = super::sys::emscripten_run_script_string(script.as_ptr());
            String::from_raw_parts(strdup(ptr), strlen(ptr), strlen(ptr) + 1)
        }
    }

    pub fn set_main_loop(func: super::sys::em_callback_fun, fps: i32, simulate_infinite_loop: bool) {
        unsafe {
            super::sys::emscripten_set_main_loop(func, fps, if simulate_infinite_loop { 1 } else { 0 });
        }
    }

    // XXX: rust style params
    pub fn set_main_loop_arg(func: super::sys::em_arg_callback_fun, arg: *mut c_void, fps: i32, simulate_infinite_loop: bool) {
        unsafe {
            super::sys::emscripten_set_main_loop_arg(func, arg, fps, if simulate_infinite_loop { 1 } else { 0 });
        }
    }

    pub fn set_main_loop_timing(mode: i32, value: i32) -> Result<(), i32> {
        unsafe {
            match super::sys::emscripten_set_main_loop_timing(mode, value) {
                0 => Ok(()),
                code => Err(code),
            }
        }
    }

    pub fn cancel_main_loop() {
        unsafe {
            super::sys::emscripten_cancel_main_loop();
        }
    }
}
