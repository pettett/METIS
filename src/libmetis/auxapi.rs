use ::libc;
extern "C" {
    fn free(_: *mut libc::c_void);
    fn libmetis__iset(n: size_t, val: idx_t, x: *mut idx_t) -> *mut idx_t;
}
pub type __int32_t = libc::c_int;
pub type int32_t = __int32_t;
pub type size_t = libc::c_ulong;
pub type idx_t = int32_t;
pub const METIS_OK: C2RustUnnamed = 1;
pub type C2RustUnnamed = libc::c_int;
pub const METIS_ERROR: C2RustUnnamed = -4;
pub const METIS_ERROR_MEMORY: C2RustUnnamed = -3;
pub const METIS_ERROR_INPUT: C2RustUnnamed = -2;
#[no_mangle]
pub unsafe extern "C" fn METIS_Free(mut ptr: *mut libc::c_void) -> libc::c_int {
    if !ptr.is_null() {
        free(ptr);
    }
    return METIS_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn METIS_SetDefaultOptions(
    mut options: *mut idx_t,
) -> libc::c_int {
    libmetis__iset(40 as libc::c_int as size_t, -(1 as libc::c_int), options);
    return METIS_OK as libc::c_int;
}
