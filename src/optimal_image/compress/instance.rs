extern crate vips_sys as ffi;

use std::ffi::{CString, CStr};
use std::error::Error;
use std::os::raw::{c_int, c_void, c_char};
use std::ptr::null;
use std::marker::PhantomData;

pub fn current_error() -> String {
    let msg = unsafe {
        CStr::from_ptr(ffi::vips_error_buffer())
    };
    msg.to_str().unwrap().to_string()
}

fn result<'a>(ptr: *mut ffi::VipsImage) -> Result<VipsImage<'a>, Box<Error>> {
    if ptr.is_null() {
        Err(current_error().into())
    } else {
        Ok(VipsImage { c: ptr, marker: PhantomData })
    }
}


pub struct VipsInstance { }

impl VipsInstance {
    pub fn new(name:&str) -> Result<VipsInstance, Box<Error>> {
        let c = CString::new(name)?;
        unsafe {
            ffi::vips_init(c.as_ptr());
        }
        Ok(VipsInstance {})
    }
}

impl Drop for VipsInstance {
    fn drop(&mut self) {
        unsafe {
            ffi::vips_shutdown();
        }
    }
}

