extern crate libc;


pub struct libraw_data_t;
pub enum LibRaw_image_formats {}

#[repr(C)]
pub struct libraw_processed_image_t {
    t: LibRaw_image_formats,
    pub height: u16,
    pub width: u16,
    pub colors: u16,
    bits: u16,
    data_size: uint,
    pub data: *const u8,
}


#[link(name = "raw")]
extern {
    pub fn libraw_init(flags: libc::c_uint) -> *mut libraw_data_t;

    pub fn libraw_open_file(data: *mut libraw_data_t, filename: *const libc::c_char) -> libc::c_int;
    pub fn libraw_open_file_ex(data: *mut libraw_data_t, filename: *const libc::c_char, bigfile_size: libc::int64_t) -> libc::c_int;
    pub fn libraw_open_buffer(data: *mut libraw_data_t, buffer: *mut libc::c_void, size: libc::size_t) -> libc::c_int;
    pub fn libraw_unpack(data: *mut libraw_data_t) -> libc::c_int;
    pub fn libraw_unpack_thumb(data: *mut libraw_data_t) -> libc::c_int;

    pub fn libraw_version() -> *const libc::c_char;
    pub fn libraw_versionNumber() -> *const libc::c_char;
    pub fn libraw_cameraCount() -> libc::c_int;
    pub fn libraw_cameraList() -> libc::c_int;
    pub fn libraw_get_decoder_info(data: *mut libraw_data_t, info: *mut libc::c_void);
    pub fn libraw_unpack_function_name(data: *mut libraw_data_t);
    pub fn libraw_subtract_black(data: *mut libraw_data_t);
    pub fn libraw_recycle_datastream(data: *mut libraw_data_t);
    pub fn libraw_recycle(data: *mut libraw_data_t);
    pub fn libraw_close(data: *mut libraw_data_t);
    pub fn libraw_strerror(errorcode: libc::c_int) -> *const libc::c_char;
    // bool LIBRAW_CHECK_VERSION(major,minor,patch)
    // const char *libraw_strprogress(enum LibRaw_progress);
    // void libraw_set_memerror_handler(*mut libraw_data_t*, memory_callback cb);
    // void libraw_set_dataerror_handler(*mut libraw_data_t*,data_callback func);
    // void libraw_set_progress_handler(*mut libraw_data_t*,progress_callback func);

    pub fn libraw_raw2image(data: *mut libraw_data_t) -> libc::c_int;
    pub fn libraw_free_image(data: *mut libraw_data_t) -> libc::c_int;
    pub fn libraw_adjust_sizes_info_only(data: *mut libraw_data_t) -> libc::c_int;
    pub fn libraw_dcraw_process(data: *mut libraw_data_t) -> libc::c_int;

    pub fn libraw_dcraw_ppm_tiff_writer(data: *mut libraw_data_t, filename: *const libc::c_char) -> libc::c_int;
    pub fn libraw_dcraw_thumb_writer(data: *mut libraw_data_t, filename: *const libc::c_char) -> libc::c_int;

    pub fn libraw_dcraw_make_mem_image(data: *mut libraw_data_t, errcode: *mut libc::c_int) -> *const libraw_processed_image_t;
    pub fn libraw_dcraw_make_mem_thumb(data: *mut libraw_data_t, errcode: *mut libc::c_int) -> *const libraw_processed_image_t;
}
