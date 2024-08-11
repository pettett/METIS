use ::libc;
use libc::free;

use super::{gklib::libmetis__iset, util::METIS_OK};

use super::structure::*;

#[no_mangle]
pub unsafe extern "C" fn METIS_Free(mut ptr: *mut libc::c_void) -> libc::c_int {
    if !ptr.is_null() {
        free(ptr);
    }
    return METIS_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn METIS_SetDefaultOptions(mut options: *mut idx_t) -> libc::c_int {
    libmetis__iset(40 as libc::c_int as size_t, -(1), options);
    return METIS_OK as libc::c_int;
}
