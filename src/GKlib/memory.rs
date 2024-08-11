use super::{
    error::{__va_list_tag, gk_errexit},
    io::{__off64_t, __off_t},
    mcore::{
        gk_gkmcoreAdd, gk_gkmcoreCreate, gk_gkmcoreDel, gk_gkmcoreDestroy, gk_gkmcorePop,
        gk_gkmcorePush,
    },
};
use crate::libmetis::{
    auxapi::*, contig::*, fortran::*, gklib::*, graph::*, kwayrefine::*, options::*, structure::*,
    timing::*, util::*, wspace::*,
};
use ::libc;
use libc::{fprintf, free, malloc, memmove, printf, realloc};
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;

}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type gk_idx_t = ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gk_ckv_t {
    pub key: libc::c_char,
    pub val: ssize_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gk_ikv_t {
    pub key: libc::c_int,
    pub val: ssize_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gk_i32kv_t {
    pub key: int32_t,
    pub val: ssize_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gk_i64kv_t {
    pub key: int64_t,
    pub val: ssize_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gk_zkv_t {
    pub key: ssize_t,
    pub val: ssize_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gk_fkv_t {
    pub key: libc::c_float,
    pub val: ssize_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gk_dkv_t {
    pub key: libc::c_double,
    pub val: ssize_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gk_skv_t {
    pub key: *mut libc::c_char,
    pub val: ssize_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gk_idxkv_t {
    pub key: gk_idx_t,
    pub val: gk_idx_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gk_mop_t {
    pub type_0: libc::c_int,
    pub nbytes: ssize_t,
    pub ptr: *mut libc::c_void,
}
#[thread_local]
static mut gkmcore: *mut gk_mcore_t = 0 as *const gk_mcore_t as *mut gk_mcore_t;
#[no_mangle]
pub unsafe extern "C" fn gk_cFreeMatrix(
    mut r_matrix: *mut *mut *mut libc::c_char,
    mut ndim1: size_t,
    mut ndim2: size_t,
) {
    let mut i: gk_idx_t = 0;
    let mut matrix: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    if (*r_matrix).is_null() {
        return;
    }
    matrix = *r_matrix;
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < ndim1 {
        gk_free(
            &mut *matrix.offset(i as isize) as *mut *mut libc::c_char as *mut *mut libc::c_void,
            0 as *mut *mut libc::c_void,
        );
        i += 1;
        i;
    }
    gk_free(
        r_matrix as *mut *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gk_cSetMatrix(
    mut matrix: *mut *mut libc::c_char,
    mut ndim1: size_t,
    mut ndim2: size_t,
    mut value: libc::c_char,
) {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < ndim1 {
        j = 0 as libc::c_int as gk_idx_t;
        while (j as u64) < ndim2 {
            *(*matrix.offset(i as isize)).offset(j as isize) = value;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn gk_cmalloc(
    mut n: size_t,
    mut msg: *mut libc::c_char,
) -> *mut libc::c_char {
    return gk_malloc(
        (::core::mem::size_of::<libc::c_char>() as u64).wrapping_mul(n),
        msg,
    ) as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn gk_crealloc(
    mut ptr: *mut libc::c_char,
    mut n: size_t,
    mut msg: *mut libc::c_char,
) -> *mut libc::c_char {
    return gk_realloc(
        ptr as *mut libc::c_void,
        (::core::mem::size_of::<libc::c_char>() as u64).wrapping_mul(n),
        msg,
    ) as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn gk_csmalloc(
    mut n: size_t,
    mut ival: libc::c_char,
    mut msg: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    ptr = gk_malloc(
        (::core::mem::size_of::<libc::c_char>() as u64).wrapping_mul(n),
        msg,
    ) as *mut libc::c_char;
    if ptr.is_null() {
        return 0 as *mut libc::c_char;
    }
    return gk_cset(n, ival, ptr);
}
#[no_mangle]
pub unsafe extern "C" fn gk_cset(
    mut n: size_t,
    mut val: libc::c_char,
    mut x: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *x.offset(i as isize) = val;
        i = i.wrapping_add(1);
        i;
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn gk_ccopy(
    mut n: size_t,
    mut a: *mut libc::c_char,
    mut b: *mut libc::c_char,
) -> *mut libc::c_char {
    return memmove(
        b as *mut libc::c_void,
        a as *mut libc::c_void,
        (::core::mem::size_of::<libc::c_char>() as u64).wrapping_mul(n) as usize,
    ) as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn gk_cAllocMatrix(
    mut ndim1: size_t,
    mut ndim2: size_t,
    mut value: libc::c_char,
    mut errmsg: *mut libc::c_char,
) -> *mut *mut libc::c_char {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut matrix: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    matrix = gk_malloc(
        ndim1.wrapping_mul(::core::mem::size_of::<*mut libc::c_char>() as u64),
        errmsg,
    ) as *mut *mut libc::c_char;
    if matrix.is_null() {
        return 0 as *mut *mut libc::c_char;
    }
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < ndim1 {
        let ref mut fresh0 = *matrix.offset(i as isize);
        *fresh0 = gk_csmalloc(ndim2, value, errmsg);
        if (*matrix.offset(i as isize)).is_null() {
            j = 0 as libc::c_int as gk_idx_t;
            while j < i {
                gk_free(
                    &mut *matrix.offset(j as isize) as *mut *mut libc::c_char
                        as *mut *mut libc::c_void,
                    0 as *mut *mut libc::c_void,
                );
                j += 1;
                j;
            }
            return 0 as *mut *mut libc::c_char;
        }
        i += 1;
        i;
    }
    return matrix;
}
#[no_mangle]
pub unsafe extern "C" fn gk_iFreeMatrix(
    mut r_matrix: *mut *mut *mut libc::c_int,
    mut ndim1: size_t,
    mut ndim2: size_t,
) {
    let mut i: gk_idx_t = 0;
    let mut matrix: *mut *mut libc::c_int = 0 as *mut *mut libc::c_int;
    if (*r_matrix).is_null() {
        return;
    }
    matrix = *r_matrix;
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < ndim1 {
        gk_free(
            &mut *matrix.offset(i as isize) as *mut *mut libc::c_int as *mut *mut libc::c_void,
            0 as *mut *mut libc::c_void,
        );
        i += 1;
        i;
    }
    gk_free(
        r_matrix as *mut *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gk_iSetMatrix(
    mut matrix: *mut *mut libc::c_int,
    mut ndim1: size_t,
    mut ndim2: size_t,
    mut value: libc::c_int,
) {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < ndim1 {
        j = 0 as libc::c_int as gk_idx_t;
        while (j as u64) < ndim2 {
            *(*matrix.offset(i as isize)).offset(j as isize) = value;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn gk_imalloc(mut n: size_t, mut msg: *mut libc::c_char) -> *mut libc::c_int {
    return gk_malloc(
        (::core::mem::size_of::<libc::c_int>() as u64).wrapping_mul(n),
        msg,
    ) as *mut libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gk_irealloc(
    mut ptr: *mut libc::c_int,
    mut n: size_t,
    mut msg: *mut libc::c_char,
) -> *mut libc::c_int {
    return gk_realloc(
        ptr as *mut libc::c_void,
        (::core::mem::size_of::<libc::c_int>() as u64).wrapping_mul(n),
        msg,
    ) as *mut libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gk_ismalloc(
    mut n: size_t,
    mut ival: libc::c_int,
    mut msg: *mut libc::c_char,
) -> *mut libc::c_int {
    let mut ptr: *mut libc::c_int = 0 as *mut libc::c_int;
    ptr = gk_malloc(
        (::core::mem::size_of::<libc::c_int>() as u64).wrapping_mul(n),
        msg,
    ) as *mut libc::c_int;
    if ptr.is_null() {
        return 0 as *mut libc::c_int;
    }
    return gk_iset(n, ival, ptr);
}
#[no_mangle]
pub unsafe extern "C" fn gk_iset(
    mut n: size_t,
    mut val: libc::c_int,
    mut x: *mut libc::c_int,
) -> *mut libc::c_int {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *x.offset(i as isize) = val;
        i = i.wrapping_add(1);
        i;
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn gk_icopy(
    mut n: size_t,
    mut a: *mut libc::c_int,
    mut b: *mut libc::c_int,
) -> *mut libc::c_int {
    return memmove(
        b as *mut libc::c_void,
        a as *mut libc::c_void,
        (::core::mem::size_of::<libc::c_int>() as u64).wrapping_mul(n) as usize,
    ) as *mut libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gk_iAllocMatrix(
    mut ndim1: size_t,
    mut ndim2: size_t,
    mut value: libc::c_int,
    mut errmsg: *mut libc::c_char,
) -> *mut *mut libc::c_int {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut matrix: *mut *mut libc::c_int = 0 as *mut *mut libc::c_int;
    matrix = gk_malloc(
        ndim1.wrapping_mul(::core::mem::size_of::<*mut libc::c_int>() as u64),
        errmsg,
    ) as *mut *mut libc::c_int;
    if matrix.is_null() {
        return 0 as *mut *mut libc::c_int;
    }
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < ndim1 {
        let ref mut fresh1 = *matrix.offset(i as isize);
        *fresh1 = gk_ismalloc(ndim2, value, errmsg);
        if (*matrix.offset(i as isize)).is_null() {
            j = 0 as libc::c_int as gk_idx_t;
            while j < i {
                gk_free(
                    &mut *matrix.offset(j as isize) as *mut *mut libc::c_int
                        as *mut *mut libc::c_void,
                    0 as *mut *mut libc::c_void,
                );
                j += 1;
                j;
            }
            return 0 as *mut *mut libc::c_int;
        }
        i += 1;
        i;
    }
    return matrix;
}
#[no_mangle]
pub unsafe extern "C" fn gk_i32FreeMatrix(
    mut r_matrix: *mut *mut *mut int32_t,
    mut ndim1: size_t,
    mut ndim2: size_t,
) {
    let mut i: gk_idx_t = 0;
    let mut matrix: *mut *mut int32_t = 0 as *mut *mut int32_t;
    if (*r_matrix).is_null() {
        return;
    }
    matrix = *r_matrix;
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < ndim1 {
        gk_free(
            &mut *matrix.offset(i as isize) as *mut *mut int32_t as *mut *mut libc::c_void,
            0 as *mut *mut libc::c_void,
        );
        i += 1;
        i;
    }
    gk_free(
        r_matrix as *mut *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gk_i32SetMatrix(
    mut matrix: *mut *mut int32_t,
    mut ndim1: size_t,
    mut ndim2: size_t,
    mut value: int32_t,
) {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < ndim1 {
        j = 0 as libc::c_int as gk_idx_t;
        while (j as u64) < ndim2 {
            *(*matrix.offset(i as isize)).offset(j as isize) = value;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn gk_i32copy(
    mut n: size_t,
    mut a: *mut int32_t,
    mut b: *mut int32_t,
) -> *mut int32_t {
    return memmove(
        b as *mut libc::c_void,
        a as *mut libc::c_void,
        (::core::mem::size_of::<int32_t>() as u64).wrapping_mul(n) as usize,
    ) as *mut int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn gk_i32set(
    mut n: size_t,
    mut val: int32_t,
    mut x: *mut int32_t,
) -> *mut int32_t {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *x.offset(i as isize) = val;
        i = i.wrapping_add(1);
        i;
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn gk_i32AllocMatrix(
    mut ndim1: size_t,
    mut ndim2: size_t,
    mut value: int32_t,
    mut errmsg: *mut libc::c_char,
) -> *mut *mut int32_t {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut matrix: *mut *mut int32_t = 0 as *mut *mut int32_t;
    matrix = gk_malloc(
        ndim1.wrapping_mul(::core::mem::size_of::<*mut int32_t>() as u64),
        errmsg,
    ) as *mut *mut int32_t;
    if matrix.is_null() {
        return 0 as *mut *mut int32_t;
    }
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < ndim1 {
        let ref mut fresh2 = *matrix.offset(i as isize);
        *fresh2 = gk_i32smalloc(ndim2, value, errmsg);
        if (*matrix.offset(i as isize)).is_null() {
            j = 0 as libc::c_int as gk_idx_t;
            while j < i {
                gk_free(
                    &mut *matrix.offset(j as isize) as *mut *mut int32_t as *mut *mut libc::c_void,
                    0 as *mut *mut libc::c_void,
                );
                j += 1;
                j;
            }
            return 0 as *mut *mut int32_t;
        }
        i += 1;
        i;
    }
    return matrix;
}
#[no_mangle]
pub unsafe extern "C" fn gk_i32malloc(mut n: size_t, mut msg: *mut libc::c_char) -> *mut int32_t {
    return gk_malloc(
        (::core::mem::size_of::<int32_t>() as u64).wrapping_mul(n),
        msg,
    ) as *mut int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn gk_i32realloc(
    mut ptr: *mut int32_t,
    mut n: size_t,
    mut msg: *mut libc::c_char,
) -> *mut int32_t {
    return gk_realloc(
        ptr as *mut libc::c_void,
        (::core::mem::size_of::<int32_t>() as u64).wrapping_mul(n),
        msg,
    ) as *mut int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn gk_i32smalloc(
    mut n: size_t,
    mut ival: int32_t,
    mut msg: *mut libc::c_char,
) -> *mut int32_t {
    let mut ptr: *mut int32_t = 0 as *mut int32_t;
    ptr = gk_malloc(
        (::core::mem::size_of::<int32_t>() as u64).wrapping_mul(n),
        msg,
    ) as *mut int32_t;
    if ptr.is_null() {
        return 0 as *mut int32_t;
    }
    return gk_i32set(n, ival, ptr);
}
#[no_mangle]
pub unsafe extern "C" fn gk_i64FreeMatrix(
    mut r_matrix: *mut *mut *mut int64_t,
    mut ndim1: size_t,
    mut ndim2: size_t,
) {
    let mut i: gk_idx_t = 0;
    let mut matrix: *mut *mut int64_t = 0 as *mut *mut int64_t;
    if (*r_matrix).is_null() {
        return;
    }
    matrix = *r_matrix;
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < ndim1 {
        gk_free(
            &mut *matrix.offset(i as isize) as *mut *mut int64_t as *mut *mut libc::c_void,
            0 as *mut *mut libc::c_void,
        );
        i += 1;
        i;
    }
    gk_free(
        r_matrix as *mut *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gk_i64SetMatrix(
    mut matrix: *mut *mut int64_t,
    mut ndim1: size_t,
    mut ndim2: size_t,
    mut value: int64_t,
) {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < ndim1 {
        j = 0 as libc::c_int as gk_idx_t;
        while (j as u64) < ndim2 {
            *(*matrix.offset(i as isize)).offset(j as isize) = value;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn gk_i64malloc(mut n: size_t, mut msg: *mut libc::c_char) -> *mut int64_t {
    return gk_malloc(
        (::core::mem::size_of::<int64_t>() as u64).wrapping_mul(n),
        msg,
    ) as *mut int64_t;
}
#[no_mangle]
pub unsafe extern "C" fn gk_i64realloc(
    mut ptr: *mut int64_t,
    mut n: size_t,
    mut msg: *mut libc::c_char,
) -> *mut int64_t {
    return gk_realloc(
        ptr as *mut libc::c_void,
        (::core::mem::size_of::<int64_t>() as u64).wrapping_mul(n),
        msg,
    ) as *mut int64_t;
}
#[no_mangle]
pub unsafe extern "C" fn gk_i64smalloc(
    mut n: size_t,
    mut ival: int64_t,
    mut msg: *mut libc::c_char,
) -> *mut int64_t {
    let mut ptr: *mut int64_t = 0 as *mut int64_t;
    ptr = gk_malloc(
        (::core::mem::size_of::<int64_t>() as u64).wrapping_mul(n),
        msg,
    ) as *mut int64_t;
    if ptr.is_null() {
        return 0 as *mut int64_t;
    }
    return gk_i64set(n, ival, ptr);
}
#[no_mangle]
pub unsafe extern "C" fn gk_i64set(
    mut n: size_t,
    mut val: int64_t,
    mut x: *mut int64_t,
) -> *mut int64_t {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *x.offset(i as isize) = val;
        i = i.wrapping_add(1);
        i;
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn gk_i64copy(
    mut n: size_t,
    mut a: *mut int64_t,
    mut b: *mut int64_t,
) -> *mut int64_t {
    return memmove(
        b as *mut libc::c_void,
        a as *mut libc::c_void,
        (::core::mem::size_of::<int64_t>() as u64).wrapping_mul(n) as usize,
    ) as *mut int64_t;
}
#[no_mangle]
pub unsafe extern "C" fn gk_i64AllocMatrix(
    mut ndim1: size_t,
    mut ndim2: size_t,
    mut value: int64_t,
    mut errmsg: *mut libc::c_char,
) -> *mut *mut int64_t {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut matrix: *mut *mut int64_t = 0 as *mut *mut int64_t;
    matrix = gk_malloc(
        ndim1.wrapping_mul(::core::mem::size_of::<*mut int64_t>() as u64),
        errmsg,
    ) as *mut *mut int64_t;
    if matrix.is_null() {
        return 0 as *mut *mut int64_t;
    }
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < ndim1 {
        let ref mut fresh3 = *matrix.offset(i as isize);
        *fresh3 = gk_i64smalloc(ndim2, value, errmsg);
        if (*matrix.offset(i as isize)).is_null() {
            j = 0 as libc::c_int as gk_idx_t;
            while j < i {
                gk_free(
                    &mut *matrix.offset(j as isize) as *mut *mut int64_t as *mut *mut libc::c_void,
                    0 as *mut *mut libc::c_void,
                );
                j += 1;
                j;
            }
            return 0 as *mut *mut int64_t;
        }
        i += 1;
        i;
    }
    return matrix;
}
#[no_mangle]
pub unsafe extern "C" fn gk_zFreeMatrix(
    mut r_matrix: *mut *mut *mut ssize_t,
    mut ndim1: size_t,
    mut ndim2: size_t,
) {
    let mut i: gk_idx_t = 0;
    let mut matrix: *mut *mut ssize_t = 0 as *mut *mut ssize_t;
    if (*r_matrix).is_null() {
        return;
    }
    matrix = *r_matrix;
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < ndim1 {
        gk_free(
            &mut *matrix.offset(i as isize) as *mut *mut ssize_t as *mut *mut libc::c_void,
            0 as *mut *mut libc::c_void,
        );
        i += 1;
        i;
    }
    gk_free(
        r_matrix as *mut *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gk_zSetMatrix(
    mut matrix: *mut *mut ssize_t,
    mut ndim1: size_t,
    mut ndim2: size_t,
    mut value: ssize_t,
) {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < ndim1 {
        j = 0 as libc::c_int as gk_idx_t;
        while (j as u64) < ndim2 {
            *(*matrix.offset(i as isize)).offset(j as isize) = value;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn gk_zcopy(
    mut n: size_t,
    mut a: *mut ssize_t,
    mut b: *mut ssize_t,
) -> *mut ssize_t {
    return memmove(
        b as *mut libc::c_void,
        a as *mut libc::c_void,
        (::core::mem::size_of::<ssize_t>() as u64).wrapping_mul(n) as usize,
    ) as *mut ssize_t;
}
#[no_mangle]
pub unsafe extern "C" fn gk_zset(
    mut n: size_t,
    mut val: ssize_t,
    mut x: *mut ssize_t,
) -> *mut ssize_t {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *x.offset(i as isize) = val;
        i = i.wrapping_add(1);
        i;
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn gk_zmalloc(mut n: size_t, mut msg: *mut libc::c_char) -> *mut ssize_t {
    return gk_malloc(
        (::core::mem::size_of::<ssize_t>() as u64).wrapping_mul(n),
        msg,
    ) as *mut ssize_t;
}
#[no_mangle]
pub unsafe extern "C" fn gk_zAllocMatrix(
    mut ndim1: size_t,
    mut ndim2: size_t,
    mut value: ssize_t,
    mut errmsg: *mut libc::c_char,
) -> *mut *mut ssize_t {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut matrix: *mut *mut ssize_t = 0 as *mut *mut ssize_t;
    matrix = gk_malloc(
        ndim1.wrapping_mul(::core::mem::size_of::<*mut ssize_t>() as u64),
        errmsg,
    ) as *mut *mut ssize_t;
    if matrix.is_null() {
        return 0 as *mut *mut ssize_t;
    }
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < ndim1 {
        let ref mut fresh4 = *matrix.offset(i as isize);
        *fresh4 = gk_zsmalloc(ndim2, value, errmsg);
        if (*matrix.offset(i as isize)).is_null() {
            j = 0 as libc::c_int as gk_idx_t;
            while j < i {
                gk_free(
                    &mut *matrix.offset(j as isize) as *mut *mut ssize_t as *mut *mut libc::c_void,
                    0 as *mut *mut libc::c_void,
                );
                j += 1;
                j;
            }
            return 0 as *mut *mut ssize_t;
        }
        i += 1;
        i;
    }
    return matrix;
}
#[no_mangle]
pub unsafe extern "C" fn gk_zrealloc(
    mut ptr: *mut ssize_t,
    mut n: size_t,
    mut msg: *mut libc::c_char,
) -> *mut ssize_t {
    return gk_realloc(
        ptr as *mut libc::c_void,
        (::core::mem::size_of::<ssize_t>() as u64).wrapping_mul(n),
        msg,
    ) as *mut ssize_t;
}
#[no_mangle]
pub unsafe extern "C" fn gk_zsmalloc(
    mut n: size_t,
    mut ival: ssize_t,
    mut msg: *mut libc::c_char,
) -> *mut ssize_t {
    let mut ptr: *mut ssize_t = 0 as *mut ssize_t;
    ptr = gk_malloc(
        (::core::mem::size_of::<ssize_t>() as u64).wrapping_mul(n),
        msg,
    ) as *mut ssize_t;
    if ptr.is_null() {
        return 0 as *mut ssize_t;
    }
    return gk_zset(n, ival, ptr);
}
#[no_mangle]
pub unsafe extern "C" fn gk_fFreeMatrix(
    mut r_matrix: *mut *mut *mut libc::c_float,
    mut ndim1: size_t,
    mut ndim2: size_t,
) {
    let mut i: gk_idx_t = 0;
    let mut matrix: *mut *mut libc::c_float = 0 as *mut *mut libc::c_float;
    if (*r_matrix).is_null() {
        return;
    }
    matrix = *r_matrix;
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < ndim1 {
        gk_free(
            &mut *matrix.offset(i as isize) as *mut *mut libc::c_float as *mut *mut libc::c_void,
            0 as *mut *mut libc::c_void,
        );
        i += 1;
        i;
    }
    gk_free(
        r_matrix as *mut *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gk_fSetMatrix(
    mut matrix: *mut *mut libc::c_float,
    mut ndim1: size_t,
    mut ndim2: size_t,
    mut value: libc::c_float,
) {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < ndim1 {
        j = 0 as libc::c_int as gk_idx_t;
        while (j as u64) < ndim2 {
            *(*matrix.offset(i as isize)).offset(j as isize) = value;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn gk_fmalloc(
    mut n: size_t,
    mut msg: *mut libc::c_char,
) -> *mut libc::c_float {
    return gk_malloc(
        (::core::mem::size_of::<libc::c_float>() as u64).wrapping_mul(n),
        msg,
    ) as *mut libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn gk_frealloc(
    mut ptr: *mut libc::c_float,
    mut n: size_t,
    mut msg: *mut libc::c_char,
) -> *mut libc::c_float {
    return gk_realloc(
        ptr as *mut libc::c_void,
        (::core::mem::size_of::<libc::c_float>() as u64).wrapping_mul(n),
        msg,
    ) as *mut libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn gk_fsmalloc(
    mut n: size_t,
    mut ival: libc::c_float,
    mut msg: *mut libc::c_char,
) -> *mut libc::c_float {
    let mut ptr: *mut libc::c_float = 0 as *mut libc::c_float;
    ptr = gk_malloc(
        (::core::mem::size_of::<libc::c_float>() as u64).wrapping_mul(n),
        msg,
    ) as *mut libc::c_float;
    if ptr.is_null() {
        return 0 as *mut libc::c_float;
    }
    return gk_fset(n, ival, ptr);
}
#[no_mangle]
pub unsafe extern "C" fn gk_fset(
    mut n: size_t,
    mut val: libc::c_float,
    mut x: *mut libc::c_float,
) -> *mut libc::c_float {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *x.offset(i as isize) = val;
        i = i.wrapping_add(1);
        i;
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn gk_fcopy(
    mut n: size_t,
    mut a: *mut libc::c_float,
    mut b: *mut libc::c_float,
) -> *mut libc::c_float {
    return memmove(
        b as *mut libc::c_void,
        a as *mut libc::c_void,
        (::core::mem::size_of::<libc::c_float>() as u64).wrapping_mul(n) as usize,
    ) as *mut libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn gk_fAllocMatrix(
    mut ndim1: size_t,
    mut ndim2: size_t,
    mut value: libc::c_float,
    mut errmsg: *mut libc::c_char,
) -> *mut *mut libc::c_float {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut matrix: *mut *mut libc::c_float = 0 as *mut *mut libc::c_float;
    matrix = gk_malloc(
        ndim1.wrapping_mul(::core::mem::size_of::<*mut libc::c_float>() as u64),
        errmsg,
    ) as *mut *mut libc::c_float;
    if matrix.is_null() {
        return 0 as *mut *mut libc::c_float;
    }
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < ndim1 {
        let ref mut fresh5 = *matrix.offset(i as isize);
        *fresh5 = gk_fsmalloc(ndim2, value, errmsg);
        if (*matrix.offset(i as isize)).is_null() {
            j = 0 as libc::c_int as gk_idx_t;
            while j < i {
                gk_free(
                    &mut *matrix.offset(j as isize) as *mut *mut libc::c_float
                        as *mut *mut libc::c_void,
                    0 as *mut *mut libc::c_void,
                );
                j += 1;
                j;
            }
            return 0 as *mut *mut libc::c_float;
        }
        i += 1;
        i;
    }
    return matrix;
}
#[no_mangle]
pub unsafe extern "C" fn gk_dFreeMatrix(
    mut r_matrix: *mut *mut *mut libc::c_double,
    mut ndim1: size_t,
    mut ndim2: size_t,
) {
    let mut i: gk_idx_t = 0;
    let mut matrix: *mut *mut libc::c_double = 0 as *mut *mut libc::c_double;
    if (*r_matrix).is_null() {
        return;
    }
    matrix = *r_matrix;
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < ndim1 {
        gk_free(
            &mut *matrix.offset(i as isize) as *mut *mut libc::c_double as *mut *mut libc::c_void,
            0 as *mut *mut libc::c_void,
        );
        i += 1;
        i;
    }
    gk_free(
        r_matrix as *mut *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gk_dSetMatrix(
    mut matrix: *mut *mut libc::c_double,
    mut ndim1: size_t,
    mut ndim2: size_t,
    mut value: libc::c_double,
) {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < ndim1 {
        j = 0 as libc::c_int as gk_idx_t;
        while (j as u64) < ndim2 {
            *(*matrix.offset(i as isize)).offset(j as isize) = value;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn gk_drealloc(
    mut ptr: *mut libc::c_double,
    mut n: size_t,
    mut msg: *mut libc::c_char,
) -> *mut libc::c_double {
    return gk_realloc(
        ptr as *mut libc::c_void,
        (::core::mem::size_of::<libc::c_double>() as u64).wrapping_mul(n),
        msg,
    ) as *mut libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn gk_dsmalloc(
    mut n: size_t,
    mut ival: libc::c_double,
    mut msg: *mut libc::c_char,
) -> *mut libc::c_double {
    let mut ptr: *mut libc::c_double = 0 as *mut libc::c_double;
    ptr = gk_malloc(
        (::core::mem::size_of::<libc::c_double>() as u64).wrapping_mul(n),
        msg,
    ) as *mut libc::c_double;
    if ptr.is_null() {
        return 0 as *mut libc::c_double;
    }
    return gk_dset(n, ival, ptr);
}
#[no_mangle]
pub unsafe extern "C" fn gk_dset(
    mut n: size_t,
    mut val: libc::c_double,
    mut x: *mut libc::c_double,
) -> *mut libc::c_double {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *x.offset(i as isize) = val;
        i = i.wrapping_add(1);
        i;
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn gk_dcopy(
    mut n: size_t,
    mut a: *mut libc::c_double,
    mut b: *mut libc::c_double,
) -> *mut libc::c_double {
    return memmove(
        b as *mut libc::c_void,
        a as *mut libc::c_void,
        (::core::mem::size_of::<libc::c_double>() as u64).wrapping_mul(n) as usize,
    ) as *mut libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn gk_dAllocMatrix(
    mut ndim1: size_t,
    mut ndim2: size_t,
    mut value: libc::c_double,
    mut errmsg: *mut libc::c_char,
) -> *mut *mut libc::c_double {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut matrix: *mut *mut libc::c_double = 0 as *mut *mut libc::c_double;
    matrix = gk_malloc(
        ndim1.wrapping_mul(::core::mem::size_of::<*mut libc::c_double>() as u64),
        errmsg,
    ) as *mut *mut libc::c_double;
    if matrix.is_null() {
        return 0 as *mut *mut libc::c_double;
    }
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < ndim1 {
        let ref mut fresh6 = *matrix.offset(i as isize);
        *fresh6 = gk_dsmalloc(ndim2, value, errmsg);
        if (*matrix.offset(i as isize)).is_null() {
            j = 0 as libc::c_int as gk_idx_t;
            while j < i {
                gk_free(
                    &mut *matrix.offset(j as isize) as *mut *mut libc::c_double
                        as *mut *mut libc::c_void,
                    0 as *mut *mut libc::c_void,
                );
                j += 1;
                j;
            }
            return 0 as *mut *mut libc::c_double;
        }
        i += 1;
        i;
    }
    return matrix;
}
#[no_mangle]
pub unsafe extern "C" fn gk_dmalloc(
    mut n: size_t,
    mut msg: *mut libc::c_char,
) -> *mut libc::c_double {
    return gk_malloc(
        (::core::mem::size_of::<libc::c_double>() as u64).wrapping_mul(n),
        msg,
    ) as *mut libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn gk_idxFreeMatrix(
    mut r_matrix: *mut *mut *mut gk_idx_t,
    mut ndim1: size_t,
    mut ndim2: size_t,
) {
    let mut i: gk_idx_t = 0;
    let mut matrix: *mut *mut gk_idx_t = 0 as *mut *mut gk_idx_t;
    if (*r_matrix).is_null() {
        return;
    }
    matrix = *r_matrix;
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < ndim1 {
        gk_free(
            &mut *matrix.offset(i as isize) as *mut *mut gk_idx_t as *mut *mut libc::c_void,
            0 as *mut *mut libc::c_void,
        );
        i += 1;
        i;
    }
    gk_free(
        r_matrix as *mut *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gk_idxSetMatrix(
    mut matrix: *mut *mut gk_idx_t,
    mut ndim1: size_t,
    mut ndim2: size_t,
    mut value: gk_idx_t,
) {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < ndim1 {
        j = 0 as libc::c_int as gk_idx_t;
        while (j as u64) < ndim2 {
            *(*matrix.offset(i as isize)).offset(j as isize) = value;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn gk_idxset(
    mut n: size_t,
    mut val: gk_idx_t,
    mut x: *mut gk_idx_t,
) -> *mut gk_idx_t {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *x.offset(i as isize) = val;
        i = i.wrapping_add(1);
        i;
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn gk_idxAllocMatrix(
    mut ndim1: size_t,
    mut ndim2: size_t,
    mut value: gk_idx_t,
    mut errmsg: *mut libc::c_char,
) -> *mut *mut gk_idx_t {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut matrix: *mut *mut gk_idx_t = 0 as *mut *mut gk_idx_t;
    matrix = gk_malloc(
        ndim1.wrapping_mul(::core::mem::size_of::<*mut gk_idx_t>() as u64),
        errmsg,
    ) as *mut *mut gk_idx_t;
    if matrix.is_null() {
        return 0 as *mut *mut gk_idx_t;
    }
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < ndim1 {
        let ref mut fresh7 = *matrix.offset(i as isize);
        *fresh7 = gk_idxsmalloc(ndim2, value, errmsg);
        if (*matrix.offset(i as isize)).is_null() {
            j = 0 as libc::c_int as gk_idx_t;
            while j < i {
                gk_free(
                    &mut *matrix.offset(j as isize) as *mut *mut gk_idx_t as *mut *mut libc::c_void,
                    0 as *mut *mut libc::c_void,
                );
                j += 1;
                j;
            }
            return 0 as *mut *mut gk_idx_t;
        }
        i += 1;
        i;
    }
    return matrix;
}
#[no_mangle]
pub unsafe extern "C" fn gk_idxcopy(
    mut n: size_t,
    mut a: *mut gk_idx_t,
    mut b: *mut gk_idx_t,
) -> *mut gk_idx_t {
    return memmove(
        b as *mut libc::c_void,
        a as *mut libc::c_void,
        (::core::mem::size_of::<gk_idx_t>() as u64).wrapping_mul(n) as usize,
    ) as *mut gk_idx_t;
}
#[no_mangle]
pub unsafe extern "C" fn gk_idxsmalloc(
    mut n: size_t,
    mut ival: gk_idx_t,
    mut msg: *mut libc::c_char,
) -> *mut gk_idx_t {
    let mut ptr: *mut gk_idx_t = 0 as *mut gk_idx_t;
    ptr = gk_malloc(
        (::core::mem::size_of::<gk_idx_t>() as u64).wrapping_mul(n),
        msg,
    ) as *mut gk_idx_t;
    if ptr.is_null() {
        return 0 as *mut gk_idx_t;
    }
    return gk_idxset(n, ival, ptr);
}
#[no_mangle]
pub unsafe extern "C" fn gk_idxrealloc(
    mut ptr: *mut gk_idx_t,
    mut n: size_t,
    mut msg: *mut libc::c_char,
) -> *mut gk_idx_t {
    return gk_realloc(
        ptr as *mut libc::c_void,
        (::core::mem::size_of::<gk_idx_t>() as u64).wrapping_mul(n),
        msg,
    ) as *mut gk_idx_t;
}
#[no_mangle]
pub unsafe extern "C" fn gk_idxmalloc(mut n: size_t, mut msg: *mut libc::c_char) -> *mut gk_idx_t {
    return gk_malloc(
        (::core::mem::size_of::<gk_idx_t>() as u64).wrapping_mul(n),
        msg,
    ) as *mut gk_idx_t;
}
#[no_mangle]
pub unsafe extern "C" fn gk_ckvFreeMatrix(
    mut r_matrix: *mut *mut *mut gk_ckv_t,
    mut ndim1: size_t,
    mut ndim2: size_t,
) {
    let mut i: gk_idx_t = 0;
    let mut matrix: *mut *mut gk_ckv_t = 0 as *mut *mut gk_ckv_t;
    if (*r_matrix).is_null() {
        return;
    }
    matrix = *r_matrix;
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < ndim1 {
        gk_free(
            &mut *matrix.offset(i as isize) as *mut *mut gk_ckv_t as *mut *mut libc::c_void,
            0 as *mut *mut libc::c_void,
        );
        i += 1;
        i;
    }
    gk_free(
        r_matrix as *mut *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gk_ckvSetMatrix(
    mut matrix: *mut *mut gk_ckv_t,
    mut ndim1: size_t,
    mut ndim2: size_t,
    mut value: gk_ckv_t,
) {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < ndim1 {
        j = 0 as libc::c_int as gk_idx_t;
        while (j as u64) < ndim2 {
            *(*matrix.offset(i as isize)).offset(j as isize) = value;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn gk_ckvset(
    mut n: size_t,
    mut val: gk_ckv_t,
    mut x: *mut gk_ckv_t,
) -> *mut gk_ckv_t {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *x.offset(i as isize) = val;
        i = i.wrapping_add(1);
        i;
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn gk_ckvsmalloc(
    mut n: size_t,
    mut ival: gk_ckv_t,
    mut msg: *mut libc::c_char,
) -> *mut gk_ckv_t {
    let mut ptr: *mut gk_ckv_t = 0 as *mut gk_ckv_t;
    ptr = gk_malloc(
        (::core::mem::size_of::<gk_ckv_t>() as u64).wrapping_mul(n),
        msg,
    ) as *mut gk_ckv_t;
    if ptr.is_null() {
        return 0 as *mut gk_ckv_t;
    }
    return gk_ckvset(n, ival, ptr);
}
#[no_mangle]
pub unsafe extern "C" fn gk_ckvrealloc(
    mut ptr: *mut gk_ckv_t,
    mut n: size_t,
    mut msg: *mut libc::c_char,
) -> *mut gk_ckv_t {
    return gk_realloc(
        ptr as *mut libc::c_void,
        (::core::mem::size_of::<gk_ckv_t>() as u64).wrapping_mul(n),
        msg,
    ) as *mut gk_ckv_t;
}
#[no_mangle]
pub unsafe extern "C" fn gk_ckvmalloc(mut n: size_t, mut msg: *mut libc::c_char) -> *mut gk_ckv_t {
    return gk_malloc(
        (::core::mem::size_of::<gk_ckv_t>() as u64).wrapping_mul(n),
        msg,
    ) as *mut gk_ckv_t;
}
#[no_mangle]
pub unsafe extern "C" fn gk_ckvcopy(
    mut n: size_t,
    mut a: *mut gk_ckv_t,
    mut b: *mut gk_ckv_t,
) -> *mut gk_ckv_t {
    return memmove(
        b as *mut libc::c_void,
        a as *mut libc::c_void,
        (::core::mem::size_of::<gk_ckv_t>() as u64).wrapping_mul(n) as usize,
    ) as *mut gk_ckv_t;
}
#[no_mangle]
pub unsafe extern "C" fn gk_ckvAllocMatrix(
    mut ndim1: size_t,
    mut ndim2: size_t,
    mut value: gk_ckv_t,
    mut errmsg: *mut libc::c_char,
) -> *mut *mut gk_ckv_t {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut matrix: *mut *mut gk_ckv_t = 0 as *mut *mut gk_ckv_t;
    matrix = gk_malloc(
        ndim1.wrapping_mul(::core::mem::size_of::<*mut gk_ckv_t>() as u64),
        errmsg,
    ) as *mut *mut gk_ckv_t;
    if matrix.is_null() {
        return 0 as *mut *mut gk_ckv_t;
    }
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < ndim1 {
        let ref mut fresh8 = *matrix.offset(i as isize);
        *fresh8 = gk_ckvsmalloc(ndim2, value, errmsg);
        if (*matrix.offset(i as isize)).is_null() {
            j = 0 as libc::c_int as gk_idx_t;
            while j < i {
                gk_free(
                    &mut *matrix.offset(j as isize) as *mut *mut gk_ckv_t as *mut *mut libc::c_void,
                    0 as *mut *mut libc::c_void,
                );
                j += 1;
                j;
            }
            return 0 as *mut *mut gk_ckv_t;
        }
        i += 1;
        i;
    }
    return matrix;
}
#[no_mangle]
pub unsafe extern "C" fn gk_ikvSetMatrix(
    mut matrix: *mut *mut gk_ikv_t,
    mut ndim1: size_t,
    mut ndim2: size_t,
    mut value: gk_ikv_t,
) {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < ndim1 {
        j = 0 as libc::c_int as gk_idx_t;
        while (j as u64) < ndim2 {
            *(*matrix.offset(i as isize)).offset(j as isize) = value;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn gk_ikvFreeMatrix(
    mut r_matrix: *mut *mut *mut gk_ikv_t,
    mut ndim1: size_t,
    mut ndim2: size_t,
) {
    let mut i: gk_idx_t = 0;
    let mut matrix: *mut *mut gk_ikv_t = 0 as *mut *mut gk_ikv_t;
    if (*r_matrix).is_null() {
        return;
    }
    matrix = *r_matrix;
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < ndim1 {
        gk_free(
            &mut *matrix.offset(i as isize) as *mut *mut gk_ikv_t as *mut *mut libc::c_void,
            0 as *mut *mut libc::c_void,
        );
        i += 1;
        i;
    }
    gk_free(
        r_matrix as *mut *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gk_ikvset(
    mut n: size_t,
    mut val: gk_ikv_t,
    mut x: *mut gk_ikv_t,
) -> *mut gk_ikv_t {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *x.offset(i as isize) = val;
        i = i.wrapping_add(1);
        i;
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn gk_ikvrealloc(
    mut ptr: *mut gk_ikv_t,
    mut n: size_t,
    mut msg: *mut libc::c_char,
) -> *mut gk_ikv_t {
    return gk_realloc(
        ptr as *mut libc::c_void,
        (::core::mem::size_of::<gk_ikv_t>() as u64).wrapping_mul(n),
        msg,
    ) as *mut gk_ikv_t;
}
#[no_mangle]
pub unsafe extern "C" fn gk_ikvsmalloc(
    mut n: size_t,
    mut ival: gk_ikv_t,
    mut msg: *mut libc::c_char,
) -> *mut gk_ikv_t {
    let mut ptr: *mut gk_ikv_t = 0 as *mut gk_ikv_t;
    ptr = gk_malloc(
        (::core::mem::size_of::<gk_ikv_t>() as u64).wrapping_mul(n),
        msg,
    ) as *mut gk_ikv_t;
    if ptr.is_null() {
        return 0 as *mut gk_ikv_t;
    }
    return gk_ikvset(n, ival, ptr);
}
#[no_mangle]
pub unsafe extern "C" fn gk_ikvmalloc(mut n: size_t, mut msg: *mut libc::c_char) -> *mut gk_ikv_t {
    return gk_malloc(
        (::core::mem::size_of::<gk_ikv_t>() as u64).wrapping_mul(n),
        msg,
    ) as *mut gk_ikv_t;
}
#[no_mangle]
pub unsafe extern "C" fn gk_ikvcopy(
    mut n: size_t,
    mut a: *mut gk_ikv_t,
    mut b: *mut gk_ikv_t,
) -> *mut gk_ikv_t {
    return memmove(
        b as *mut libc::c_void,
        a as *mut libc::c_void,
        (::core::mem::size_of::<gk_ikv_t>() as u64).wrapping_mul(n) as usize,
    ) as *mut gk_ikv_t;
}
#[no_mangle]
pub unsafe extern "C" fn gk_ikvAllocMatrix(
    mut ndim1: size_t,
    mut ndim2: size_t,
    mut value: gk_ikv_t,
    mut errmsg: *mut libc::c_char,
) -> *mut *mut gk_ikv_t {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut matrix: *mut *mut gk_ikv_t = 0 as *mut *mut gk_ikv_t;
    matrix = gk_malloc(
        ndim1.wrapping_mul(::core::mem::size_of::<*mut gk_ikv_t>() as u64),
        errmsg,
    ) as *mut *mut gk_ikv_t;
    if matrix.is_null() {
        return 0 as *mut *mut gk_ikv_t;
    }
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < ndim1 {
        let ref mut fresh9 = *matrix.offset(i as isize);
        *fresh9 = gk_ikvsmalloc(ndim2, value, errmsg);
        if (*matrix.offset(i as isize)).is_null() {
            j = 0 as libc::c_int as gk_idx_t;
            while j < i {
                gk_free(
                    &mut *matrix.offset(j as isize) as *mut *mut gk_ikv_t as *mut *mut libc::c_void,
                    0 as *mut *mut libc::c_void,
                );
                j += 1;
                j;
            }
            return 0 as *mut *mut gk_ikv_t;
        }
        i += 1;
        i;
    }
    return matrix;
}
#[no_mangle]
pub unsafe extern "C" fn gk_i32kvFreeMatrix(
    mut r_matrix: *mut *mut *mut gk_i32kv_t,
    mut ndim1: size_t,
    mut ndim2: size_t,
) {
    let mut i: gk_idx_t = 0;
    let mut matrix: *mut *mut gk_i32kv_t = 0 as *mut *mut gk_i32kv_t;
    if (*r_matrix).is_null() {
        return;
    }
    matrix = *r_matrix;
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < ndim1 {
        gk_free(
            &mut *matrix.offset(i as isize) as *mut *mut gk_i32kv_t as *mut *mut libc::c_void,
            0 as *mut *mut libc::c_void,
        );
        i += 1;
        i;
    }
    gk_free(
        r_matrix as *mut *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gk_i32kvSetMatrix(
    mut matrix: *mut *mut gk_i32kv_t,
    mut ndim1: size_t,
    mut ndim2: size_t,
    mut value: gk_i32kv_t,
) {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < ndim1 {
        j = 0 as libc::c_int as gk_idx_t;
        while (j as u64) < ndim2 {
            *(*matrix.offset(i as isize)).offset(j as isize) = value;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn gk_i32kvmalloc(
    mut n: size_t,
    mut msg: *mut libc::c_char,
) -> *mut gk_i32kv_t {
    return gk_malloc(
        (::core::mem::size_of::<gk_i32kv_t>() as u64).wrapping_mul(n),
        msg,
    ) as *mut gk_i32kv_t;
}
#[no_mangle]
pub unsafe extern "C" fn gk_i32kvrealloc(
    mut ptr: *mut gk_i32kv_t,
    mut n: size_t,
    mut msg: *mut libc::c_char,
) -> *mut gk_i32kv_t {
    return gk_realloc(
        ptr as *mut libc::c_void,
        (::core::mem::size_of::<gk_i32kv_t>() as u64).wrapping_mul(n),
        msg,
    ) as *mut gk_i32kv_t;
}
#[no_mangle]
pub unsafe extern "C" fn gk_i32kvsmalloc(
    mut n: size_t,
    mut ival: gk_i32kv_t,
    mut msg: *mut libc::c_char,
) -> *mut gk_i32kv_t {
    let mut ptr: *mut gk_i32kv_t = 0 as *mut gk_i32kv_t;
    ptr = gk_malloc(
        (::core::mem::size_of::<gk_i32kv_t>() as u64).wrapping_mul(n),
        msg,
    ) as *mut gk_i32kv_t;
    if ptr.is_null() {
        return 0 as *mut gk_i32kv_t;
    }
    return gk_i32kvset(n, ival, ptr);
}
#[no_mangle]
pub unsafe extern "C" fn gk_i32kvset(
    mut n: size_t,
    mut val: gk_i32kv_t,
    mut x: *mut gk_i32kv_t,
) -> *mut gk_i32kv_t {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *x.offset(i as isize) = val;
        i = i.wrapping_add(1);
        i;
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn gk_i32kvcopy(
    mut n: size_t,
    mut a: *mut gk_i32kv_t,
    mut b: *mut gk_i32kv_t,
) -> *mut gk_i32kv_t {
    return memmove(
        b as *mut libc::c_void,
        a as *mut libc::c_void,
        (::core::mem::size_of::<gk_i32kv_t>() as u64).wrapping_mul(n) as usize,
    ) as *mut gk_i32kv_t;
}
#[no_mangle]
pub unsafe extern "C" fn gk_i32kvAllocMatrix(
    mut ndim1: size_t,
    mut ndim2: size_t,
    mut value: gk_i32kv_t,
    mut errmsg: *mut libc::c_char,
) -> *mut *mut gk_i32kv_t {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut matrix: *mut *mut gk_i32kv_t = 0 as *mut *mut gk_i32kv_t;
    matrix = gk_malloc(
        ndim1.wrapping_mul(::core::mem::size_of::<*mut gk_i32kv_t>() as u64),
        errmsg,
    ) as *mut *mut gk_i32kv_t;
    if matrix.is_null() {
        return 0 as *mut *mut gk_i32kv_t;
    }
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < ndim1 {
        let ref mut fresh10 = *matrix.offset(i as isize);
        *fresh10 = gk_i32kvsmalloc(ndim2, value, errmsg);
        if (*matrix.offset(i as isize)).is_null() {
            j = 0 as libc::c_int as gk_idx_t;
            while j < i {
                gk_free(
                    &mut *matrix.offset(j as isize) as *mut *mut gk_i32kv_t
                        as *mut *mut libc::c_void,
                    0 as *mut *mut libc::c_void,
                );
                j += 1;
                j;
            }
            return 0 as *mut *mut gk_i32kv_t;
        }
        i += 1;
        i;
    }
    return matrix;
}
#[no_mangle]
pub unsafe extern "C" fn gk_i64kvSetMatrix(
    mut matrix: *mut *mut gk_i64kv_t,
    mut ndim1: size_t,
    mut ndim2: size_t,
    mut value: gk_i64kv_t,
) {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < ndim1 {
        j = 0 as libc::c_int as gk_idx_t;
        while (j as u64) < ndim2 {
            *(*matrix.offset(i as isize)).offset(j as isize) = value;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn gk_i64kvFreeMatrix(
    mut r_matrix: *mut *mut *mut gk_i64kv_t,
    mut ndim1: size_t,
    mut ndim2: size_t,
) {
    let mut i: gk_idx_t = 0;
    let mut matrix: *mut *mut gk_i64kv_t = 0 as *mut *mut gk_i64kv_t;
    if (*r_matrix).is_null() {
        return;
    }
    matrix = *r_matrix;
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < ndim1 {
        gk_free(
            &mut *matrix.offset(i as isize) as *mut *mut gk_i64kv_t as *mut *mut libc::c_void,
            0 as *mut *mut libc::c_void,
        );
        i += 1;
        i;
    }
    gk_free(
        r_matrix as *mut *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gk_i64kvmalloc(
    mut n: size_t,
    mut msg: *mut libc::c_char,
) -> *mut gk_i64kv_t {
    return gk_malloc(
        (::core::mem::size_of::<gk_i64kv_t>() as u64).wrapping_mul(n),
        msg,
    ) as *mut gk_i64kv_t;
}
#[no_mangle]
pub unsafe extern "C" fn gk_i64kvrealloc(
    mut ptr: *mut gk_i64kv_t,
    mut n: size_t,
    mut msg: *mut libc::c_char,
) -> *mut gk_i64kv_t {
    return gk_realloc(
        ptr as *mut libc::c_void,
        (::core::mem::size_of::<gk_i64kv_t>() as u64).wrapping_mul(n),
        msg,
    ) as *mut gk_i64kv_t;
}
#[no_mangle]
pub unsafe extern "C" fn gk_i64kvsmalloc(
    mut n: size_t,
    mut ival: gk_i64kv_t,
    mut msg: *mut libc::c_char,
) -> *mut gk_i64kv_t {
    let mut ptr: *mut gk_i64kv_t = 0 as *mut gk_i64kv_t;
    ptr = gk_malloc(
        (::core::mem::size_of::<gk_i64kv_t>() as u64).wrapping_mul(n),
        msg,
    ) as *mut gk_i64kv_t;
    if ptr.is_null() {
        return 0 as *mut gk_i64kv_t;
    }
    return gk_i64kvset(n, ival, ptr);
}
#[no_mangle]
pub unsafe extern "C" fn gk_i64kvset(
    mut n: size_t,
    mut val: gk_i64kv_t,
    mut x: *mut gk_i64kv_t,
) -> *mut gk_i64kv_t {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *x.offset(i as isize) = val;
        i = i.wrapping_add(1);
        i;
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn gk_i64kvcopy(
    mut n: size_t,
    mut a: *mut gk_i64kv_t,
    mut b: *mut gk_i64kv_t,
) -> *mut gk_i64kv_t {
    return memmove(
        b as *mut libc::c_void,
        a as *mut libc::c_void,
        (::core::mem::size_of::<gk_i64kv_t>() as u64).wrapping_mul(n) as usize,
    ) as *mut gk_i64kv_t;
}
#[no_mangle]
pub unsafe extern "C" fn gk_i64kvAllocMatrix(
    mut ndim1: size_t,
    mut ndim2: size_t,
    mut value: gk_i64kv_t,
    mut errmsg: *mut libc::c_char,
) -> *mut *mut gk_i64kv_t {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut matrix: *mut *mut gk_i64kv_t = 0 as *mut *mut gk_i64kv_t;
    matrix = gk_malloc(
        ndim1.wrapping_mul(::core::mem::size_of::<*mut gk_i64kv_t>() as u64),
        errmsg,
    ) as *mut *mut gk_i64kv_t;
    if matrix.is_null() {
        return 0 as *mut *mut gk_i64kv_t;
    }
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < ndim1 {
        let ref mut fresh11 = *matrix.offset(i as isize);
        *fresh11 = gk_i64kvsmalloc(ndim2, value, errmsg);
        if (*matrix.offset(i as isize)).is_null() {
            j = 0 as libc::c_int as gk_idx_t;
            while j < i {
                gk_free(
                    &mut *matrix.offset(j as isize) as *mut *mut gk_i64kv_t
                        as *mut *mut libc::c_void,
                    0 as *mut *mut libc::c_void,
                );
                j += 1;
                j;
            }
            return 0 as *mut *mut gk_i64kv_t;
        }
        i += 1;
        i;
    }
    return matrix;
}
#[no_mangle]
pub unsafe extern "C" fn gk_zkvSetMatrix(
    mut matrix: *mut *mut gk_zkv_t,
    mut ndim1: size_t,
    mut ndim2: size_t,
    mut value: gk_zkv_t,
) {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < ndim1 {
        j = 0 as libc::c_int as gk_idx_t;
        while (j as u64) < ndim2 {
            *(*matrix.offset(i as isize)).offset(j as isize) = value;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn gk_zkvFreeMatrix(
    mut r_matrix: *mut *mut *mut gk_zkv_t,
    mut ndim1: size_t,
    mut ndim2: size_t,
) {
    let mut i: gk_idx_t = 0;
    let mut matrix: *mut *mut gk_zkv_t = 0 as *mut *mut gk_zkv_t;
    if (*r_matrix).is_null() {
        return;
    }
    matrix = *r_matrix;
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < ndim1 {
        gk_free(
            &mut *matrix.offset(i as isize) as *mut *mut gk_zkv_t as *mut *mut libc::c_void,
            0 as *mut *mut libc::c_void,
        );
        i += 1;
        i;
    }
    gk_free(
        r_matrix as *mut *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gk_zkvcopy(
    mut n: size_t,
    mut a: *mut gk_zkv_t,
    mut b: *mut gk_zkv_t,
) -> *mut gk_zkv_t {
    return memmove(
        b as *mut libc::c_void,
        a as *mut libc::c_void,
        (::core::mem::size_of::<gk_zkv_t>() as u64).wrapping_mul(n) as usize,
    ) as *mut gk_zkv_t;
}
#[no_mangle]
pub unsafe extern "C" fn gk_zkvsmalloc(
    mut n: size_t,
    mut ival: gk_zkv_t,
    mut msg: *mut libc::c_char,
) -> *mut gk_zkv_t {
    let mut ptr: *mut gk_zkv_t = 0 as *mut gk_zkv_t;
    ptr = gk_malloc(
        (::core::mem::size_of::<gk_zkv_t>() as u64).wrapping_mul(n),
        msg,
    ) as *mut gk_zkv_t;
    if ptr.is_null() {
        return 0 as *mut gk_zkv_t;
    }
    return gk_zkvset(n, ival, ptr);
}
#[no_mangle]
pub unsafe extern "C" fn gk_zkvset(
    mut n: size_t,
    mut val: gk_zkv_t,
    mut x: *mut gk_zkv_t,
) -> *mut gk_zkv_t {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *x.offset(i as isize) = val;
        i = i.wrapping_add(1);
        i;
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn gk_zkvmalloc(mut n: size_t, mut msg: *mut libc::c_char) -> *mut gk_zkv_t {
    return gk_malloc(
        (::core::mem::size_of::<gk_zkv_t>() as u64).wrapping_mul(n),
        msg,
    ) as *mut gk_zkv_t;
}
#[no_mangle]
pub unsafe extern "C" fn gk_zkvAllocMatrix(
    mut ndim1: size_t,
    mut ndim2: size_t,
    mut value: gk_zkv_t,
    mut errmsg: *mut libc::c_char,
) -> *mut *mut gk_zkv_t {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut matrix: *mut *mut gk_zkv_t = 0 as *mut *mut gk_zkv_t;
    matrix = gk_malloc(
        ndim1.wrapping_mul(::core::mem::size_of::<*mut gk_zkv_t>() as u64),
        errmsg,
    ) as *mut *mut gk_zkv_t;
    if matrix.is_null() {
        return 0 as *mut *mut gk_zkv_t;
    }
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < ndim1 {
        let ref mut fresh12 = *matrix.offset(i as isize);
        *fresh12 = gk_zkvsmalloc(ndim2, value, errmsg);
        if (*matrix.offset(i as isize)).is_null() {
            j = 0 as libc::c_int as gk_idx_t;
            while j < i {
                gk_free(
                    &mut *matrix.offset(j as isize) as *mut *mut gk_zkv_t as *mut *mut libc::c_void,
                    0 as *mut *mut libc::c_void,
                );
                j += 1;
                j;
            }
            return 0 as *mut *mut gk_zkv_t;
        }
        i += 1;
        i;
    }
    return matrix;
}
#[no_mangle]
pub unsafe extern "C" fn gk_zkvrealloc(
    mut ptr: *mut gk_zkv_t,
    mut n: size_t,
    mut msg: *mut libc::c_char,
) -> *mut gk_zkv_t {
    return gk_realloc(
        ptr as *mut libc::c_void,
        (::core::mem::size_of::<gk_zkv_t>() as u64).wrapping_mul(n),
        msg,
    ) as *mut gk_zkv_t;
}
#[no_mangle]
pub unsafe extern "C" fn gk_fkvFreeMatrix(
    mut r_matrix: *mut *mut *mut gk_fkv_t,
    mut ndim1: size_t,
    mut ndim2: size_t,
) {
    let mut i: gk_idx_t = 0;
    let mut matrix: *mut *mut gk_fkv_t = 0 as *mut *mut gk_fkv_t;
    if (*r_matrix).is_null() {
        return;
    }
    matrix = *r_matrix;
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < ndim1 {
        gk_free(
            &mut *matrix.offset(i as isize) as *mut *mut gk_fkv_t as *mut *mut libc::c_void,
            0 as *mut *mut libc::c_void,
        );
        i += 1;
        i;
    }
    gk_free(
        r_matrix as *mut *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gk_fkvSetMatrix(
    mut matrix: *mut *mut gk_fkv_t,
    mut ndim1: size_t,
    mut ndim2: size_t,
    mut value: gk_fkv_t,
) {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < ndim1 {
        j = 0 as libc::c_int as gk_idx_t;
        while (j as u64) < ndim2 {
            *(*matrix.offset(i as isize)).offset(j as isize) = value;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn gk_fkvmalloc(mut n: size_t, mut msg: *mut libc::c_char) -> *mut gk_fkv_t {
    return gk_malloc(
        (::core::mem::size_of::<gk_fkv_t>() as u64).wrapping_mul(n),
        msg,
    ) as *mut gk_fkv_t;
}
#[no_mangle]
pub unsafe extern "C" fn gk_fkvrealloc(
    mut ptr: *mut gk_fkv_t,
    mut n: size_t,
    mut msg: *mut libc::c_char,
) -> *mut gk_fkv_t {
    return gk_realloc(
        ptr as *mut libc::c_void,
        (::core::mem::size_of::<gk_fkv_t>() as u64).wrapping_mul(n),
        msg,
    ) as *mut gk_fkv_t;
}
#[no_mangle]
pub unsafe extern "C" fn gk_fkvsmalloc(
    mut n: size_t,
    mut ival: gk_fkv_t,
    mut msg: *mut libc::c_char,
) -> *mut gk_fkv_t {
    let mut ptr: *mut gk_fkv_t = 0 as *mut gk_fkv_t;
    ptr = gk_malloc(
        (::core::mem::size_of::<gk_fkv_t>() as u64).wrapping_mul(n),
        msg,
    ) as *mut gk_fkv_t;
    if ptr.is_null() {
        return 0 as *mut gk_fkv_t;
    }
    return gk_fkvset(n, ival, ptr);
}
#[no_mangle]
pub unsafe extern "C" fn gk_fkvset(
    mut n: size_t,
    mut val: gk_fkv_t,
    mut x: *mut gk_fkv_t,
) -> *mut gk_fkv_t {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *x.offset(i as isize) = val;
        i = i.wrapping_add(1);
        i;
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn gk_fkvcopy(
    mut n: size_t,
    mut a: *mut gk_fkv_t,
    mut b: *mut gk_fkv_t,
) -> *mut gk_fkv_t {
    return memmove(
        b as *mut libc::c_void,
        a as *mut libc::c_void,
        (::core::mem::size_of::<gk_fkv_t>() as u64).wrapping_mul(n) as usize,
    ) as *mut gk_fkv_t;
}
#[no_mangle]
pub unsafe extern "C" fn gk_fkvAllocMatrix(
    mut ndim1: size_t,
    mut ndim2: size_t,
    mut value: gk_fkv_t,
    mut errmsg: *mut libc::c_char,
) -> *mut *mut gk_fkv_t {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut matrix: *mut *mut gk_fkv_t = 0 as *mut *mut gk_fkv_t;
    matrix = gk_malloc(
        ndim1.wrapping_mul(::core::mem::size_of::<*mut gk_fkv_t>() as u64),
        errmsg,
    ) as *mut *mut gk_fkv_t;
    if matrix.is_null() {
        return 0 as *mut *mut gk_fkv_t;
    }
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < ndim1 {
        let ref mut fresh13 = *matrix.offset(i as isize);
        *fresh13 = gk_fkvsmalloc(ndim2, value, errmsg);
        if (*matrix.offset(i as isize)).is_null() {
            j = 0 as libc::c_int as gk_idx_t;
            while j < i {
                gk_free(
                    &mut *matrix.offset(j as isize) as *mut *mut gk_fkv_t as *mut *mut libc::c_void,
                    0 as *mut *mut libc::c_void,
                );
                j += 1;
                j;
            }
            return 0 as *mut *mut gk_fkv_t;
        }
        i += 1;
        i;
    }
    return matrix;
}
#[no_mangle]
pub unsafe extern "C" fn gk_dkvFreeMatrix(
    mut r_matrix: *mut *mut *mut gk_dkv_t,
    mut ndim1: size_t,
    mut ndim2: size_t,
) {
    let mut i: gk_idx_t = 0;
    let mut matrix: *mut *mut gk_dkv_t = 0 as *mut *mut gk_dkv_t;
    if (*r_matrix).is_null() {
        return;
    }
    matrix = *r_matrix;
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < ndim1 {
        gk_free(
            &mut *matrix.offset(i as isize) as *mut *mut gk_dkv_t as *mut *mut libc::c_void,
            0 as *mut *mut libc::c_void,
        );
        i += 1;
        i;
    }
    gk_free(
        r_matrix as *mut *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gk_dkvSetMatrix(
    mut matrix: *mut *mut gk_dkv_t,
    mut ndim1: size_t,
    mut ndim2: size_t,
    mut value: gk_dkv_t,
) {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < ndim1 {
        j = 0 as libc::c_int as gk_idx_t;
        while (j as u64) < ndim2 {
            *(*matrix.offset(i as isize)).offset(j as isize) = value;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn gk_dkvAllocMatrix(
    mut ndim1: size_t,
    mut ndim2: size_t,
    mut value: gk_dkv_t,
    mut errmsg: *mut libc::c_char,
) -> *mut *mut gk_dkv_t {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut matrix: *mut *mut gk_dkv_t = 0 as *mut *mut gk_dkv_t;
    matrix = gk_malloc(
        ndim1.wrapping_mul(::core::mem::size_of::<*mut gk_dkv_t>() as u64),
        errmsg,
    ) as *mut *mut gk_dkv_t;
    if matrix.is_null() {
        return 0 as *mut *mut gk_dkv_t;
    }
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < ndim1 {
        let ref mut fresh14 = *matrix.offset(i as isize);
        *fresh14 = gk_dkvsmalloc(ndim2, value, errmsg);
        if (*matrix.offset(i as isize)).is_null() {
            j = 0 as libc::c_int as gk_idx_t;
            while j < i {
                gk_free(
                    &mut *matrix.offset(j as isize) as *mut *mut gk_dkv_t as *mut *mut libc::c_void,
                    0 as *mut *mut libc::c_void,
                );
                j += 1;
                j;
            }
            return 0 as *mut *mut gk_dkv_t;
        }
        i += 1;
        i;
    }
    return matrix;
}
#[no_mangle]
pub unsafe extern "C" fn gk_dkvmalloc(mut n: size_t, mut msg: *mut libc::c_char) -> *mut gk_dkv_t {
    return gk_malloc(
        (::core::mem::size_of::<gk_dkv_t>() as u64).wrapping_mul(n),
        msg,
    ) as *mut gk_dkv_t;
}
#[no_mangle]
pub unsafe extern "C" fn gk_dkvrealloc(
    mut ptr: *mut gk_dkv_t,
    mut n: size_t,
    mut msg: *mut libc::c_char,
) -> *mut gk_dkv_t {
    return gk_realloc(
        ptr as *mut libc::c_void,
        (::core::mem::size_of::<gk_dkv_t>() as u64).wrapping_mul(n),
        msg,
    ) as *mut gk_dkv_t;
}
#[no_mangle]
pub unsafe extern "C" fn gk_dkvsmalloc(
    mut n: size_t,
    mut ival: gk_dkv_t,
    mut msg: *mut libc::c_char,
) -> *mut gk_dkv_t {
    let mut ptr: *mut gk_dkv_t = 0 as *mut gk_dkv_t;
    ptr = gk_malloc(
        (::core::mem::size_of::<gk_dkv_t>() as u64).wrapping_mul(n),
        msg,
    ) as *mut gk_dkv_t;
    if ptr.is_null() {
        return 0 as *mut gk_dkv_t;
    }
    return gk_dkvset(n, ival, ptr);
}
#[no_mangle]
pub unsafe extern "C" fn gk_dkvset(
    mut n: size_t,
    mut val: gk_dkv_t,
    mut x: *mut gk_dkv_t,
) -> *mut gk_dkv_t {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *x.offset(i as isize) = val;
        i = i.wrapping_add(1);
        i;
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn gk_dkvcopy(
    mut n: size_t,
    mut a: *mut gk_dkv_t,
    mut b: *mut gk_dkv_t,
) -> *mut gk_dkv_t {
    return memmove(
        b as *mut libc::c_void,
        a as *mut libc::c_void,
        (::core::mem::size_of::<gk_dkv_t>() as u64).wrapping_mul(n) as usize,
    ) as *mut gk_dkv_t;
}
#[no_mangle]
pub unsafe extern "C" fn gk_skvSetMatrix(
    mut matrix: *mut *mut gk_skv_t,
    mut ndim1: size_t,
    mut ndim2: size_t,
    mut value: gk_skv_t,
) {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < ndim1 {
        j = 0 as libc::c_int as gk_idx_t;
        while (j as u64) < ndim2 {
            *(*matrix.offset(i as isize)).offset(j as isize) = value;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn gk_skvFreeMatrix(
    mut r_matrix: *mut *mut *mut gk_skv_t,
    mut ndim1: size_t,
    mut ndim2: size_t,
) {
    let mut i: gk_idx_t = 0;
    let mut matrix: *mut *mut gk_skv_t = 0 as *mut *mut gk_skv_t;
    if (*r_matrix).is_null() {
        return;
    }
    matrix = *r_matrix;
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < ndim1 {
        gk_free(
            &mut *matrix.offset(i as isize) as *mut *mut gk_skv_t as *mut *mut libc::c_void,
            0 as *mut *mut libc::c_void,
        );
        i += 1;
        i;
    }
    gk_free(
        r_matrix as *mut *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gk_skvAllocMatrix(
    mut ndim1: size_t,
    mut ndim2: size_t,
    mut value: gk_skv_t,
    mut errmsg: *mut libc::c_char,
) -> *mut *mut gk_skv_t {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut matrix: *mut *mut gk_skv_t = 0 as *mut *mut gk_skv_t;
    matrix = gk_malloc(
        ndim1.wrapping_mul(::core::mem::size_of::<*mut gk_skv_t>() as u64),
        errmsg,
    ) as *mut *mut gk_skv_t;
    if matrix.is_null() {
        return 0 as *mut *mut gk_skv_t;
    }
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < ndim1 {
        let ref mut fresh15 = *matrix.offset(i as isize);
        *fresh15 = gk_skvsmalloc(ndim2, value, errmsg);
        if (*matrix.offset(i as isize)).is_null() {
            j = 0 as libc::c_int as gk_idx_t;
            while j < i {
                gk_free(
                    &mut *matrix.offset(j as isize) as *mut *mut gk_skv_t as *mut *mut libc::c_void,
                    0 as *mut *mut libc::c_void,
                );
                j += 1;
                j;
            }
            return 0 as *mut *mut gk_skv_t;
        }
        i += 1;
        i;
    }
    return matrix;
}
#[no_mangle]
pub unsafe extern "C" fn gk_skvmalloc(mut n: size_t, mut msg: *mut libc::c_char) -> *mut gk_skv_t {
    return gk_malloc(
        (::core::mem::size_of::<gk_skv_t>() as u64).wrapping_mul(n),
        msg,
    ) as *mut gk_skv_t;
}
#[no_mangle]
pub unsafe extern "C" fn gk_skvrealloc(
    mut ptr: *mut gk_skv_t,
    mut n: size_t,
    mut msg: *mut libc::c_char,
) -> *mut gk_skv_t {
    return gk_realloc(
        ptr as *mut libc::c_void,
        (::core::mem::size_of::<gk_skv_t>() as u64).wrapping_mul(n),
        msg,
    ) as *mut gk_skv_t;
}
#[no_mangle]
pub unsafe extern "C" fn gk_skvsmalloc(
    mut n: size_t,
    mut ival: gk_skv_t,
    mut msg: *mut libc::c_char,
) -> *mut gk_skv_t {
    let mut ptr: *mut gk_skv_t = 0 as *mut gk_skv_t;
    ptr = gk_malloc(
        (::core::mem::size_of::<gk_skv_t>() as u64).wrapping_mul(n),
        msg,
    ) as *mut gk_skv_t;
    if ptr.is_null() {
        return 0 as *mut gk_skv_t;
    }
    return gk_skvset(n, ival, ptr);
}
#[no_mangle]
pub unsafe extern "C" fn gk_skvset(
    mut n: size_t,
    mut val: gk_skv_t,
    mut x: *mut gk_skv_t,
) -> *mut gk_skv_t {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *x.offset(i as isize) = val;
        i = i.wrapping_add(1);
        i;
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn gk_skvcopy(
    mut n: size_t,
    mut a: *mut gk_skv_t,
    mut b: *mut gk_skv_t,
) -> *mut gk_skv_t {
    return memmove(
        b as *mut libc::c_void,
        a as *mut libc::c_void,
        (::core::mem::size_of::<gk_skv_t>() as u64).wrapping_mul(n) as usize,
    ) as *mut gk_skv_t;
}
#[no_mangle]
pub unsafe extern "C" fn gk_idxkvSetMatrix(
    mut matrix: *mut *mut gk_idxkv_t,
    mut ndim1: size_t,
    mut ndim2: size_t,
    mut value: gk_idxkv_t,
) {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < ndim1 {
        j = 0 as libc::c_int as gk_idx_t;
        while (j as u64) < ndim2 {
            *(*matrix.offset(i as isize)).offset(j as isize) = value;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn gk_idxkvFreeMatrix(
    mut r_matrix: *mut *mut *mut gk_idxkv_t,
    mut ndim1: size_t,
    mut ndim2: size_t,
) {
    let mut i: gk_idx_t = 0;
    let mut matrix: *mut *mut gk_idxkv_t = 0 as *mut *mut gk_idxkv_t;
    if (*r_matrix).is_null() {
        return;
    }
    matrix = *r_matrix;
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < ndim1 {
        gk_free(
            &mut *matrix.offset(i as isize) as *mut *mut gk_idxkv_t as *mut *mut libc::c_void,
            0 as *mut *mut libc::c_void,
        );
        i += 1;
        i;
    }
    gk_free(
        r_matrix as *mut *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gk_idxkvAllocMatrix(
    mut ndim1: size_t,
    mut ndim2: size_t,
    mut value: gk_idxkv_t,
    mut errmsg: *mut libc::c_char,
) -> *mut *mut gk_idxkv_t {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut matrix: *mut *mut gk_idxkv_t = 0 as *mut *mut gk_idxkv_t;
    matrix = gk_malloc(
        ndim1.wrapping_mul(::core::mem::size_of::<*mut gk_idxkv_t>() as u64),
        errmsg,
    ) as *mut *mut gk_idxkv_t;
    if matrix.is_null() {
        return 0 as *mut *mut gk_idxkv_t;
    }
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < ndim1 {
        let ref mut fresh16 = *matrix.offset(i as isize);
        *fresh16 = gk_idxkvsmalloc(ndim2, value, errmsg);
        if (*matrix.offset(i as isize)).is_null() {
            j = 0 as libc::c_int as gk_idx_t;
            while j < i {
                gk_free(
                    &mut *matrix.offset(j as isize) as *mut *mut gk_idxkv_t
                        as *mut *mut libc::c_void,
                    0 as *mut *mut libc::c_void,
                );
                j += 1;
                j;
            }
            return 0 as *mut *mut gk_idxkv_t;
        }
        i += 1;
        i;
    }
    return matrix;
}
#[no_mangle]
pub unsafe extern "C" fn gk_idxkvmalloc(
    mut n: size_t,
    mut msg: *mut libc::c_char,
) -> *mut gk_idxkv_t {
    return gk_malloc(
        (::core::mem::size_of::<gk_idxkv_t>() as u64).wrapping_mul(n),
        msg,
    ) as *mut gk_idxkv_t;
}
#[no_mangle]
pub unsafe extern "C" fn gk_idxkvrealloc(
    mut ptr: *mut gk_idxkv_t,
    mut n: size_t,
    mut msg: *mut libc::c_char,
) -> *mut gk_idxkv_t {
    return gk_realloc(
        ptr as *mut libc::c_void,
        (::core::mem::size_of::<gk_idxkv_t>() as u64).wrapping_mul(n),
        msg,
    ) as *mut gk_idxkv_t;
}
#[no_mangle]
pub unsafe extern "C" fn gk_idxkvsmalloc(
    mut n: size_t,
    mut ival: gk_idxkv_t,
    mut msg: *mut libc::c_char,
) -> *mut gk_idxkv_t {
    let mut ptr: *mut gk_idxkv_t = 0 as *mut gk_idxkv_t;
    ptr = gk_malloc(
        (::core::mem::size_of::<gk_idxkv_t>() as u64).wrapping_mul(n),
        msg,
    ) as *mut gk_idxkv_t;
    if ptr.is_null() {
        return 0 as *mut gk_idxkv_t;
    }
    return gk_idxkvset(n, ival, ptr);
}
#[no_mangle]
pub unsafe extern "C" fn gk_idxkvset(
    mut n: size_t,
    mut val: gk_idxkv_t,
    mut x: *mut gk_idxkv_t,
) -> *mut gk_idxkv_t {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *x.offset(i as isize) = val;
        i = i.wrapping_add(1);
        i;
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn gk_idxkvcopy(
    mut n: size_t,
    mut a: *mut gk_idxkv_t,
    mut b: *mut gk_idxkv_t,
) -> *mut gk_idxkv_t {
    return memmove(
        b as *mut libc::c_void,
        a as *mut libc::c_void,
        (::core::mem::size_of::<gk_idxkv_t>() as u64).wrapping_mul(n) as usize,
    ) as *mut gk_idxkv_t;
}
#[no_mangle]
pub unsafe extern "C" fn gk_AllocMatrix(
    mut r_matrix: *mut *mut *mut libc::c_void,
    mut elmlen: size_t,
    mut ndim1: size_t,
    mut ndim2: size_t,
) {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut matrix: *mut *mut libc::c_void = 0 as *mut *mut libc::c_void;
    *r_matrix = 0 as *mut *mut libc::c_void;
    matrix = gk_malloc(
        ndim1.wrapping_mul(::core::mem::size_of::<*mut libc::c_void>() as u64),
        b"gk_AllocMatrix: matrix\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut *mut libc::c_void;
    if matrix.is_null() {
        return;
    }
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < ndim1 {
        let ref mut fresh17 = *matrix.offset(i as isize);
        *fresh17 = gk_malloc(
            ndim2.wrapping_mul(elmlen),
            b"gk_AllocMatrix: matrix[i]\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if (*fresh17).is_null() {
            j = 0 as libc::c_int as gk_idx_t;
            while j < i {
                gk_free(
                    &mut *matrix.offset(j as isize) as *mut *mut libc::c_void,
                    0 as *mut *mut libc::c_void,
                );
                j += 1;
                j;
            }
            return;
        }
        i += 1;
        i;
    }
    *r_matrix = matrix;
}
#[no_mangle]
pub unsafe extern "C" fn gk_FreeMatrix(
    mut r_matrix: *mut *mut *mut libc::c_void,
    mut ndim1: size_t,
    mut ndim2: size_t,
) {
    let mut i: gk_idx_t = 0;
    let mut matrix: *mut *mut libc::c_void = 0 as *mut *mut libc::c_void;
    matrix = *r_matrix;
    if matrix.is_null() {
        return;
    }
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < ndim1 {
        gk_free(
            &mut *matrix.offset(i as isize) as *mut *mut libc::c_void,
            0 as *mut *mut libc::c_void,
        );
        i += 1;
        i;
    }
    gk_free(
        r_matrix as *mut *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gk_malloc_init() -> libc::c_int {
    if gkmcore.is_null() {
        gkmcore = gk_gkmcoreCreate();
    }
    if gkmcore.is_null() {
        return 0 as libc::c_int;
    }
    gk_gkmcorePush(gkmcore);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gk_malloc_cleanup(mut showstats: libc::c_int) {
    if !gkmcore.is_null() {
        gk_gkmcorePop(gkmcore);
        if (*gkmcore).cmop == 0 as libc::c_int as u64 {
            gk_gkmcoreDestroy(&mut gkmcore, showstats);
            gkmcore = 0 as *mut gk_mcore_t;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn gk_malloc(
    mut nbytes: size_t,
    mut msg: *mut libc::c_char,
) -> *mut libc::c_void {
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    if nbytes == 0 as libc::c_int as u64 {
        nbytes = nbytes.wrapping_add(1);
        nbytes;
    }
    ptr = malloc(nbytes as usize);
    if ptr.is_null() {
        printf(
            b"   Current memory used:  %10zu bytes\n\0" as *const u8 as *const libc::c_char,
            gk_GetCurMemoryUsed(),
        );
        printf(
            b"   Maximum memory used:  %10zu bytes\n\0" as *const u8 as *const libc::c_char,
            gk_GetMaxMemoryUsed(),
        );
        gk_errexit(
            6 as libc::c_int,
            b"***Memory allocation failed for %s. Requested size: %zu bytes\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            msg,
            nbytes,
        );
        return 0 as *mut libc::c_void;
    }
    if !gkmcore.is_null() {
        gk_gkmcoreAdd(gkmcore, 3 as libc::c_int, nbytes, ptr);
    }
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn gk_realloc(
    mut oldptr: *mut libc::c_void,
    mut nbytes: size_t,
    mut msg: *mut libc::c_char,
) -> *mut libc::c_void {
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    if nbytes == 0 as libc::c_int as u64 {
        nbytes = nbytes.wrapping_add(1);
        nbytes;
    }
    if !gkmcore.is_null() && !oldptr.is_null() {
        gk_gkmcoreDel(gkmcore, oldptr);
    }
    ptr = realloc(oldptr, nbytes as usize);
    if ptr.is_null() {
        printf(
            b"   Maximum memory used: %10zu bytes\n\0" as *const u8 as *const libc::c_char,
            gk_GetMaxMemoryUsed(),
        );
        printf(
            b"   Current memory used: %10zu bytes\n\0" as *const u8 as *const libc::c_char,
            gk_GetCurMemoryUsed(),
        );
        gk_errexit(
            6 as libc::c_int,
            b"***Memory realloc failed for %s. Requested size: %zu bytes\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            msg,
            nbytes,
        );
        return 0 as *mut libc::c_void;
    }
    if !gkmcore.is_null() {
        gk_gkmcoreAdd(gkmcore, 3 as libc::c_int, nbytes, ptr);
    }
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn gk_free(mut ptr1: *mut *mut libc::c_void, mut args: ...) {
    let mut plist: ::core::ffi::VaListImpl;
    let mut ptr: *mut *mut libc::c_void = 0 as *mut *mut libc::c_void;
    if !(*ptr1).is_null() {
        free(*ptr1);
        if !gkmcore.is_null() {
            gk_gkmcoreDel(gkmcore, *ptr1);
        }
    }
    *ptr1 = 0 as *mut libc::c_void;
    plist = args.clone();
    loop {
        ptr = plist.arg::<*mut *mut libc::c_void>();
        if ptr.is_null() {
            break;
        }
        if !(*ptr).is_null() {
            free(*ptr);
            if !gkmcore.is_null() {
                gk_gkmcoreDel(gkmcore, *ptr);
            }
        }
        *ptr = 0 as *mut libc::c_void;
    }
}
#[no_mangle]
pub unsafe extern "C" fn gk_GetCurMemoryUsed() -> size_t {
    if gkmcore.is_null() {
        return 0 as libc::c_int as size_t;
    } else {
        return (*gkmcore).cur_hallocs;
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_GetMaxMemoryUsed() -> size_t {
    if gkmcore.is_null() {
        return 0 as libc::c_int as size_t;
    } else {
        return (*gkmcore).max_hallocs;
    };
}
