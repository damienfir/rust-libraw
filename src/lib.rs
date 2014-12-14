extern crate libc;

use std::ptr;
use std::string::String;
use std::slice;

mod ffi;

// TODO: copy LibRaw C++ api

pub struct Image {
    data: *mut ffi::libraw_data_t,
    path: String,
}

impl Image {
    pub fn new(path: &str) -> Image {
        let data: *mut ffi::libraw_data_t;
        unsafe {
            data = ffi::libraw_init(0);
        }
        Image{data: data, path: String::from_str(path)}
    }

    pub fn load(&self) -> bool {
        let c_path = self.path.as_slice().to_c_str();
        unsafe {
            ffi::libraw_open_file(self.data, c_path.as_ptr()) == 0
        }
    }

    pub fn unpack(&self) -> bool {
        unsafe {
            ffi::libraw_unpack(self.data) == 0
        }
    }

    pub fn to_image(&self) -> bool {
        unsafe {
            ffi::libraw_raw2image(self.data) == 0
        }
    }

    pub fn copy(&self) -> (&[u8], int, int, int) {
        let error: &mut libc::c_int;
        let processed: *const ffi::libraw_processed_image_t;
        unsafe {
            processed = ffi::libraw_dcraw_make_mem_image(self.data, error);
            let n = (*processed).height * (*processed).width * (*processed).colors;
            let bitmap = slice::from_raw_buf(&(*processed).data, n as uint);
            (bitmap, (*processed).height as int, (*processed).width as int, (*processed).colors as int)
        }
    }

    pub fn write(&self, path: &str) -> bool {
        let c_out = path.to_c_str();
        unsafe {
            ffi::libraw_dcraw_ppm_tiff_writer(self.data, c_out.as_ptr()) == 0
        }
    }
}
