//! STM32CubeIDE Minimal System calls implementation in Rust
//! 
//! This file provides minimal system call implementations required for embedded systems.
//! Translated from the original C implementation provided by STM32CubeIDE.
//! 
//! Copyright (c) 2018 STMicroelectronics
//! 
//! License terms remain as per original file.

use core::ffi::c_void;
use core::ptr;

// Global variables
static mut HEAP_END: *mut u8 = ptr::null_mut();
static mut ENV: [*const u8; 1] = [ptr::null()];
static mut ENVIRON: *const *const u8 = unsafe { &ENV as *const _ };

#[no_mangle]
pub extern "C" fn initialise_monitor_handles() {
}

#[no_mangle]
pub extern "C" fn _getpid() -> i32 {
    1
}

#[no_mangle]
pub extern "C" fn _kill(_pid: i32, _sig: i32) -> i32 {
    set_errno(libc::EINVAL);
    -1
}

#[no_mangle]
pub extern "C" fn _exit(status: i32) -> ! {
    let _ = _kill(status, -1);
    loop {} // Make sure we hang here
}

#[no_mangle]
pub extern "C" fn _read(file: i32, ptr: *mut u8, len: i32) -> i32 {
    let mut count = 0;
    while count < len {
        unsafe {
            *ptr.offset(count as isize) = __io_getchar() as u8;
        }
        count += 1;
    }
    len
}

#[no_mangle]
pub extern "C" fn _write(_file: i32, ptr: *const u8, len: i32) -> i32 {
    let mut count = 0;
    while count < len {
        unsafe {
            __io_putchar(*ptr.offset(count as isize) as i32);
        }
        count += 1;
    }
    len
}

#[no_mangle]
pub extern "C" fn _close(_file: i32) -> i32 {
    -1
}

#[no_mangle]
pub extern "C" fn _fstat(_file: i32, st: *mut libc::stat) -> i32 {
    unsafe {
        (*st).st_mode = libc::S_IFCHR;
    }
    0
}

#[no_mangle]
pub extern "C" fn _isatty(_file: i32) -> i32 {
    1
}

#[no_mangle]
pub extern "C" fn _lseek(_file: i32, _ptr: i32, _dir: i32) -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn _open(_path: *const u8, _flags: i32, _mode: i32) -> i32 {
    -1
}

#[no_mangle]
pub extern "C" fn _wait(_status: *mut i32) -> i32 {
    set_errno(libc::ECHILD);
    -1
}

#[no_mangle]
pub extern "C" fn _unlink(_name: *const u8) -> i32 {
    set_errno(libc::ENOENT);
    -1
}

#[no_mangle]
pub extern "C" fn _times(_buf: *mut libc::tms) -> i32 {
    -1
}

#[no_mangle]
pub extern "C" fn _stat(_file: *const u8, st: *mut libc::stat) -> i32 {
    unsafe {
        (*st).st_mode = libc::S_IFCHR;
    }
    0
}

#[no_mangle]
pub extern "C" fn _link(_old: *const u8, _new: *const u8) -> i32 {
    set_errno(libc::EMLINK);
    -1
}

#[no_mangle]
pub extern "C" fn _fork() -> i32 {
    set_errno(libc::EAGAIN);
    -1
}

#[no_mangle]
pub extern "C" fn _execve(_name: *const u8, _argv: *const *const u8, _env: *const *const u8) -> i32 {
    set_errno(libc::ENOMEM);
    -1
}

/// Increase program data space. Malloc and related functions depend on this
#[no_mangle]
pub extern "C" fn _sbrk(incr: i32) -> *mut c_void {
    extern "C" {
        static end: u8;
        static mut stack_ptr: *mut u8;
    }

    unsafe {
        if HEAP_END.is_null() {
            HEAP_END = &end as *const u8 as *mut u8;
        }

        let prev_heap_end = HEAP_END;
        if HEAP_END.offset(incr as isize) > stack_ptr {
            set_errno(libc::ENOMEM);
            return (-1isize) as *mut c_void;
        }

        HEAP_END = HEAP_END.offset(incr as isize);
        prev_heap_end as *mut c_void
    }
}

// Helper function to set errno
fn set_errno(e: i32) {
    extern "C" {
        static mut errno: i32;
    }
    unsafe {
        errno = e;
    }
}

// External function declarations
extern "C" {
    fn __io_putchar(ch: i32) -> i32;
    fn __io_getchar() -> i32;
}
