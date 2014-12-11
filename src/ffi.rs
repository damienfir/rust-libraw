extern crate libc;


#[link(name = "raw")]
extern {
    // init
    pub fn libraw_init(flags: libc::c_uint) -> *mut libc::c_void;

    // loading
    pub fn libraw_open_file(data: *mut libc::c_void, filename: *const libc::c_char) -> libc::c_int;
    pub fn libraw_unpack(data: *mut libc::c_void) -> libc::c_int;

    // auxiliary
    pub fn libraw_recycle(data: *mut libc::c_void);

    // dcraw emulation
    pub fn libraw_raw2image(data: *mut libc::c_void) -> libc::c_int;
    pub fn libraw_dcraw_ppm_tiff_writer(data: *mut libc::c_void, filename: *const libc::c_char) -> libc::c_int;
}
