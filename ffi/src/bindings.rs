/* automatically generated by rust-bindgen 0.65.1 */

pub type wchar_t = ::std::os::raw::c_int;
pub type max_align_t = f64;
pub const snappy_status_SNAPPY_OK: snappy_status = 0;
pub const snappy_status_SNAPPY_INVALID_INPUT: snappy_status = 1;
pub const snappy_status_SNAPPY_BUFFER_TOO_SMALL: snappy_status = 2;
pub type snappy_status = ::std::os::raw::c_uint;
extern "C" {
    pub fn snappy_compress(
        input: *const ::std::os::raw::c_char,
        input_length: usize,
        compressed: *mut ::std::os::raw::c_char,
        compressed_length: *mut usize,
    ) -> snappy_status;
}
extern "C" {
    pub fn snappy_uncompress(
        compressed: *const ::std::os::raw::c_char,
        compressed_length: usize,
        uncompressed: *mut ::std::os::raw::c_char,
        uncompressed_length: *mut usize,
    ) -> snappy_status;
}
extern "C" {
    pub fn snappy_max_compressed_length(source_length: usize) -> usize;
}
extern "C" {
    pub fn snappy_uncompressed_length(
        compressed: *const ::std::os::raw::c_char,
        compressed_length: usize,
        result: *mut usize,
    ) -> snappy_status;
}
extern "C" {
    pub fn snappy_validate_compressed_buffer(
        compressed: *const ::std::os::raw::c_char,
        compressed_length: usize,
    ) -> snappy_status;
}