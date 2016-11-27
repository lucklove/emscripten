extern crate emscripten;
extern crate libc;

use emscripten::em;
use libc::*;

fn test_run_script() {
    em::run_script(&r#"console.log("Begin of test_run_script")"#);
    em::run_script(&r#"console.log("Running javascript in rust...")"#);
    em::run_script(&r#"console.log("End of test_run_script")"#);
}

fn test_run_script_int() {
    assert_eq!(em::run_script_int("123"), 123i32);
    assert_eq!(em::run_script_int("456"), 456i32);
}

fn test_run_script_string() {
    assert_eq!(em::run_script_string(r#""hello""#), "hello");
    assert_eq!(em::run_script_string(r#""world""#), "world");
}

extern fn main_loop(arg: *mut c_void) {
    println!("{:?}", arg);
    em::cancel_main_loop();
}

fn main() {
    test_run_script();
    test_run_script_int();
    test_run_script_string();
    em::set_main_loop_arg(main_loop, 16 as *mut c_void, 60, true);
}