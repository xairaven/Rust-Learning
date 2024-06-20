use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap();
}

/*
!!! UNWRAP IS BAD !!!

thread 'main' panicked at src\main.rs:4:49:
called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "The system cannot find the file specified." }
stack backtrace:
   0: rust_begin_unwind
             at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library\std\src/panicking.rs:645:5
   1: core::panicking::panic_fmt
             at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library\core\src/panicking.rs:72:14
   2: core::result::unwrap_failed
             at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library\core\src/result.rs:1654:5
   3: core::result::Result<T,E>::unwrap
             at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src/result.rs:1077:23
   4: project::main
             at .\src\main.rs:4:25
   5: core::ops::function::FnOnce::call_once
             at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
error: process didn't exit successfully: `target\debug\project.exe` (exit code: 101)
 */