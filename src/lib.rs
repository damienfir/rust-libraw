extern crate libc;

use std::ptr;
use std::string::String;

mod ffi;


pub struct Image {
    data: *mut libc::c_void,
    path: String,
}

impl Image {
    pub fn new(path: &str) -> Image {
        let data: *mut libc::c_void;
        unsafe {
            data = ffi::libraw_init(0);
        }
        Image{data: data, path: String::from_str(path)}
    }

    pub fn load(&self) -> bool {
        let c_path = self.path.as_slice().to_c_str();
        unsafe {
            ffi::libraw_open_file(self.data, c_path.as_ptr()) == 0 &&
            ffi::libraw_unpack(self.data) == 0
        }
    }

    pub fn write(&self, path: &str) -> bool {
        let c_out = path.to_c_str();
        unsafe {
            ffi::libraw_raw2image(self.data) == 0 &&
            ffi::libraw_dcraw_ppm_tiff_writer(self.data, c_out.as_ptr()) == 0
        }
    }
}
