extern crate libc;

pub type libraw_data_t = *mut libc::c_void;


#[link(name = "raw")]
extern {
    pub fn libraw_init(flags: libc::c_uint) -> libraw_data_t;

    pub fn libraw_open_file(data: libraw_data_t, filename: *const libc::c_char) -> libc::c_int;
    pub fn libraw_open_file_ex(data: libraw_data_t, filename: *const libc::c_char, bigfile_size: libc::int64_t) -> libc::c_int;
    pub fn libraw_open_buffer(data: libraw_data_t, buffer: *mut libc::c_void, size: libc::size_t) -> libc::c_int;
    pub fn libraw_unpack(data: libraw_data_t) -> libc::c_int;
    pub fn libraw_unpack_thumb(data: libraw_data_t) -> libc::c_int;

    pub fn libraw_version() -> *const libc::c_char;
    pub fn libraw_versionNumber() -> *const libc::c_char;
    pub fn libraw_cameraCount() -> libc::c_int;
    pub fn libraw_cameraList() -> libc::c_int;
    pub fn libraw_get_decoder_info(data: libraw_data_t, info: *mut libc::c_void);
    pub fn libraw_unpack_function_name(data: libraw_data_t);
    pub fn libraw_subtract_black(data: libraw_data_t);
    pub fn libraw_recycle_datastream(data: libraw_data_t);
    pub fn libraw_recycle(data: libraw_data_t);
    pub fn libraw_close(data: libraw_data_t);
    pub fn libraw_strerror(errorcode: libc::c_int) -> *const libc::c_char;
    // bool LIBRAW_CHECK_VERSION(major,minor,patch)
    // const char *libraw_strprogress(enum LibRaw_progress);
    // void libraw_set_memerror_handler(libraw_data_t*, memory_callback cb);
    // void libraw_set_dataerror_handler(libraw_data_t*,data_callback func);
    // void libraw_set_progress_handler(libraw_data_t*,progress_callback func);

    pub fn libraw_raw2image(data: libraw_data_t) -> libc::c_int;
    pub fn libraw_free_image(data: libraw_data_t) -> libc::c_int;
    pub fn libraw_adjust_sizes_info_only(data: libraw_data_t) -> libc::c_int;
    pub fn libraw_dcraw_process(data: libraw_data_t) -> libc::c_int;

    pub fn libraw_dcraw_ppm_tiff_writer(data: libraw_data_t, filename: *const libc::c_char) -> libc::c_int;
    pub fn libraw_dcraw_thumb_writer(data: libraw_data_t, filename: *const libc::c_char) -> libc::c_int;

    // libraw_processed_image_t *libraw_dcraw_make_mem_image(libraw_data_t* lr,int * errcode)
    // libraw_processed_image_t *libraw_dcraw_make_mem_thumb(libraw_data_t* lr,int * errcode)
}
