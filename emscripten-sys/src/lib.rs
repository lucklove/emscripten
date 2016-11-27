#![allow(non_camel_case_types)]

extern crate libc;

use libc::*;

pub const EM_TIMING_SETTIMEOUT:     i32 = 0;
pub const EM_TIMING_RAF:            i32 = 1;
pub const EM_TIMING_SETIMMEDIATE:   i32 = 2;

pub type em_callback_fun = extern fn();
pub type em_arg_callback_fun = extern fn(_: *mut c_void);

extern "C" {
    pub fn emscripten_run_script(script: *const c_char);
    pub fn emscripten_run_script_int(script: *const c_char) -> c_int;
    pub fn emscripten_run_script_string(script: *const c_char) -> *mut c_char;

    pub fn emscripten_set_main_loop(func: em_callback_fun, fps: c_int, simulate_infinite_loop: c_int);
    pub fn emscripten_set_main_loop_arg(func: em_arg_callback_fun, arg: *mut c_void, fps: c_int, simulate_infinite_loop: c_int);
    pub fn emscripten_set_main_loop_timing(mode: c_int, value: c_int) -> c_int;
    pub fn emscripten_cancel_main_loop();
}
