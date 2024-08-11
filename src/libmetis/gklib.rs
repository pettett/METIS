use ::libc;
use libc::memmove;

use crate::GKlib::{
    memory::{gk_free, gk_idxsmalloc, gk_malloc, gk_realloc},
    random::{gk_randinit, gk_randint32, gk_randint64, uint64_t},
};

use super::structure::*;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub _hi: *mut ikv_t,
    pub _lo: *mut ikv_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub _hi: *mut rkv_t,
    pub _lo: *mut rkv_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub _hi: *mut idx_t,
    pub _lo: *mut idx_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub _hi: *mut idx_t,
    pub _lo: *mut idx_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub _hi: *mut real_t,
    pub _lo: *mut real_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub _hi: *mut real_t,
    pub _lo: *mut real_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub _hi: *mut ikv_t,
    pub _lo: *mut ikv_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub _hi: *mut ikv_t,
    pub _lo: *mut ikv_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub _hi: *mut rkv_t,
    pub _lo: *mut rkv_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub _hi: *mut uvw_t,
    pub _lo: *mut uvw_t,
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__iargmax(mut n: size_t, mut x: *mut idx_t) -> size_t {
    let mut i: size_t = 0;
    let mut max: size_t = 0 as libc::c_int as size_t;
    i = 1 as size_t;
    while i < n {
        max = if *x.offset(i as isize) > *x.offset(max as isize) {
            i
        } else {
            max
        };
        i = i.wrapping_add(1);
        i;
    }
    return max;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__iargmin(mut n: size_t, mut x: *mut idx_t) -> size_t {
    let mut i: size_t = 0;
    let mut min: size_t = 0 as libc::c_int as size_t;
    i = 1 as size_t;
    while i < n {
        min = if *x.offset(i as isize) < *x.offset(min as isize) {
            i
        } else {
            min
        };
        i = i.wrapping_add(1);
        i;
    }
    return min;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__iargmax_n(
    mut n: size_t,
    mut x: *mut idx_t,
    mut k: size_t,
) -> size_t {
    let mut i: size_t = 0;
    let mut max_n: size_t = 0;
    let mut cand: *mut ikv_t = 0 as *mut ikv_t;
    cand = libmetis__ikvmalloc(
        n,
        b"GK_ARGMAX_N: cand\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as libc::c_int as size_t;
    while i < n {
        (*cand.offset(i as isize)).val = i as idx_t;
        (*cand.offset(i as isize)).key = *x.offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    libmetis__ikvsortd(n, cand);
    max_n = (*cand.offset(k.wrapping_sub(1 as u64) as isize)).val as size_t;
    gk_free(
        &mut cand as *mut *mut ikv_t as *mut libc::c_void as *mut *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
    return max_n;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__iscale(
    mut n: size_t,
    mut alpha: idx_t,
    mut x: *mut idx_t,
    mut incx: size_t,
) -> *mut idx_t {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *x *= alpha;
        i = i.wrapping_add(1);
        i;
        x = x.offset(incx as isize);
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__iincset(
    mut n: size_t,
    mut baseval: idx_t,
    mut x: *mut idx_t,
) -> *mut idx_t {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *x.offset(i as isize) = (baseval as u64).wrapping_add(i) as idx_t;
        i = i.wrapping_add(1);
        i;
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__imax(mut n: size_t, mut x: *mut idx_t) -> idx_t {
    let mut i: size_t = 0;
    let mut max: size_t = 0 as libc::c_int as size_t;
    if n <= 0 as libc::c_int as u64 {
        return 0 as libc::c_int;
    }
    i = 1 as size_t;
    while i < n {
        max = if *x.offset(i as isize) > *x.offset(max as isize) {
            i
        } else {
            max
        };
        i = i.wrapping_add(1);
        i;
    }
    return *x.offset(max as isize);
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__imin(mut n: size_t, mut x: *mut idx_t) -> idx_t {
    let mut i: size_t = 0;
    let mut min: size_t = 0 as libc::c_int as size_t;
    if n <= 0 as libc::c_int as u64 {
        return 0 as libc::c_int;
    }
    i = 1 as size_t;
    while i < n {
        min = if *x.offset(i as isize) < *x.offset(min as isize) {
            i
        } else {
            min
        };
        i = i.wrapping_add(1);
        i;
    }
    return *x.offset(min as isize);
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__iaxpy(
    mut n: size_t,
    mut alpha: idx_t,
    mut x: *mut idx_t,
    mut incx: size_t,
    mut y: *mut idx_t,
    mut incy: size_t,
) -> *mut idx_t {
    let mut i: size_t = 0;
    let mut y_in: *mut idx_t = y;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *y += alpha * *x;
        i = i.wrapping_add(1);
        i;
        x = x.offset(incx as isize);
        y = y.offset(incy as isize);
    }
    return y_in;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__inorm2(
    mut n: size_t,
    mut x: *mut idx_t,
    mut incx: size_t,
) -> idx_t {
    let mut i: size_t = 0;
    let mut partial: idx_t = 0 as libc::c_int;
    i = 0 as libc::c_int as size_t;
    while i < n {
        partial += *x * *x;
        i = i.wrapping_add(1);
        i;
        x = x.offset(incx as isize);
    }
    return if partial > 0 as libc::c_int {
        ((partial as libc::c_double).sqrt() as idx_t)
    } else {
        0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__isum(
    mut n: size_t,
    mut x: *mut idx_t,
    mut incx: size_t,
) -> idx_t {
    let mut i: size_t = 0;
    let mut sum: idx_t = 0 as libc::c_int;
    i = 0 as libc::c_int as size_t;
    while i < n {
        sum += *x;
        i = i.wrapping_add(1);
        i;
        x = x.offset(incx as isize);
    }
    return sum;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__idot(
    mut n: size_t,
    mut x: *mut idx_t,
    mut incx: size_t,
    mut y: *mut idx_t,
    mut incy: size_t,
) -> idx_t {
    let mut i: size_t = 0;
    let mut partial: idx_t = 0.0f64 as idx_t;
    i = 0 as libc::c_int as size_t;
    while i < n {
        partial += *x * *y;
        i = i.wrapping_add(1);
        i;
        x = x.offset(incx as isize);
        y = y.offset(incy as isize);
    }
    return partial;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__rargmax_n(
    mut n: size_t,
    mut x: *mut real_t,
    mut k: size_t,
) -> size_t {
    let mut i: size_t = 0;
    let mut max_n: size_t = 0;
    let mut cand: *mut rkv_t = 0 as *mut rkv_t;
    cand = libmetis__rkvmalloc(
        n,
        b"GK_ARGMAX_N: cand\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as libc::c_int as size_t;
    while i < n {
        (*cand.offset(i as isize)).val = i as idx_t;
        (*cand.offset(i as isize)).key = *x.offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    libmetis__rkvsortd(n, cand);
    max_n = (*cand.offset(k.wrapping_sub(1 as u64) as isize)).val as size_t;
    gk_free(
        &mut cand as *mut *mut rkv_t as *mut libc::c_void as *mut *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
    return max_n;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__rargmin(mut n: size_t, mut x: *mut real_t) -> size_t {
    let mut i: size_t = 0;
    let mut min: size_t = 0 as libc::c_int as size_t;
    i = 1 as size_t;
    while i < n {
        min = if *x.offset(i as isize) < *x.offset(min as isize) {
            i
        } else {
            min
        };
        i = i.wrapping_add(1);
        i;
    }
    return min;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__rargmax(mut n: size_t, mut x: *mut real_t) -> size_t {
    let mut i: size_t = 0;
    let mut max: size_t = 0 as libc::c_int as size_t;
    i = 1 as size_t;
    while i < n {
        max = if *x.offset(i as isize) > *x.offset(max as isize) {
            i
        } else {
            max
        };
        i = i.wrapping_add(1);
        i;
    }
    return max;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__rincset(
    mut n: size_t,
    mut baseval: real_t,
    mut x: *mut real_t,
) -> *mut real_t {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *x.offset(i as isize) = baseval + i as libc::c_float;
        i = i.wrapping_add(1);
        i;
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__rmax(mut n: size_t, mut x: *mut real_t) -> real_t {
    let mut i: size_t = 0;
    let mut max: size_t = 0 as libc::c_int as size_t;
    if n <= 0 as libc::c_int as u64 {
        return 0 as libc::c_int as real_t;
    }
    i = 1 as size_t;
    while i < n {
        max = if *x.offset(i as isize) > *x.offset(max as isize) {
            i
        } else {
            max
        };
        i = i.wrapping_add(1);
        i;
    }
    return *x.offset(max as isize);
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__rmin(mut n: size_t, mut x: *mut real_t) -> real_t {
    let mut i: size_t = 0;
    let mut min: size_t = 0 as libc::c_int as size_t;
    if n <= 0 as libc::c_int as u64 {
        return 0 as libc::c_int as real_t;
    }
    i = 1 as size_t;
    while i < n {
        min = if *x.offset(i as isize) < *x.offset(min as isize) {
            i
        } else {
            min
        };
        i = i.wrapping_add(1);
        i;
    }
    return *x.offset(min as isize);
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__raxpy(
    mut n: size_t,
    mut alpha: real_t,
    mut x: *mut real_t,
    mut incx: size_t,
    mut y: *mut real_t,
    mut incy: size_t,
) -> *mut real_t {
    let mut i: size_t = 0;
    let mut y_in: *mut real_t = y;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *y += alpha * *x;
        i = i.wrapping_add(1);
        i;
        x = x.offset(incx as isize);
        y = y.offset(incy as isize);
    }
    return y_in;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__rscale(
    mut n: size_t,
    mut alpha: real_t,
    mut x: *mut real_t,
    mut incx: size_t,
) -> *mut real_t {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *x *= alpha;
        i = i.wrapping_add(1);
        i;
        x = x.offset(incx as isize);
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__rsum(
    mut n: size_t,
    mut x: *mut real_t,
    mut incx: size_t,
) -> real_t {
    let mut i: size_t = 0;
    let mut sum: real_t = 0 as libc::c_int as real_t;
    i = 0 as libc::c_int as size_t;
    while i < n {
        sum += *x;
        i = i.wrapping_add(1);
        i;
        x = x.offset(incx as isize);
    }
    return sum;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__rnorm2(
    mut n: size_t,
    mut x: *mut real_t,
    mut incx: size_t,
) -> real_t {
    let mut i: size_t = 0;
    let mut partial: real_t = 0 as libc::c_int as real_t;
    i = 0 as libc::c_int as size_t;
    while i < n {
        partial += *x * *x;
        i = i.wrapping_add(1);
        i;
        x = x.offset(incx as isize);
    }
    return if partial > 0 as libc::c_int as libc::c_float {
        ((partial as libc::c_double) as real_t).sqrt()
    } else {
        0 as libc::c_int as real_t
    };
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__rdot(
    mut n: size_t,
    mut x: *mut real_t,
    mut incx: size_t,
    mut y: *mut real_t,
    mut incy: size_t,
) -> real_t {
    let mut i: size_t = 0;
    let mut partial: real_t = 0.0f64 as real_t;
    i = 0 as libc::c_int as size_t;
    while i < n {
        partial += *x * *y;
        i = i.wrapping_add(1);
        i;
        x = x.offset(incx as isize);
        y = y.offset(incy as isize);
    }
    return partial;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__iSetMatrix(
    mut matrix: *mut *mut idx_t,
    mut ndim1: size_t,
    mut ndim2: size_t,
    mut value: idx_t,
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
pub unsafe extern "C" fn libmetis__iFreeMatrix(
    mut r_matrix: *mut *mut *mut idx_t,
    mut ndim1: size_t,
    mut ndim2: size_t,
) {
    let mut i: gk_idx_t = 0;
    let mut matrix: *mut *mut idx_t = 0 as *mut *mut idx_t;
    if (*r_matrix).is_null() {
        return;
    }
    matrix = *r_matrix;
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < ndim1 {
        gk_free(
            &mut *matrix.offset(i as isize) as *mut *mut idx_t as *mut *mut libc::c_void,
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
pub unsafe extern "C" fn libmetis__ismalloc(
    mut n: size_t,
    mut ival: idx_t,
    mut msg: *mut libc::c_char,
) -> *mut idx_t {
    let mut ptr: *mut idx_t = 0 as *mut idx_t;
    ptr = gk_malloc(
        (::core::mem::size_of::<idx_t>() as u64).wrapping_mul(n),
        msg,
    ) as *mut idx_t;
    if ptr.is_null() {
        return 0 as *mut idx_t;
    }
    return libmetis__iset(n, ival, ptr);
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__iset(
    mut n: size_t,
    mut val: idx_t,
    mut x: *mut idx_t,
) -> *mut idx_t {
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
pub unsafe extern "C" fn libmetis__icopy(
    mut n: size_t,
    mut a: *mut idx_t,
    mut b: *mut idx_t,
) -> *mut idx_t {
    return memmove(
        b as *mut libc::c_void,
        a as *mut libc::c_void,
        (::core::mem::size_of::<idx_t>() as u64).wrapping_mul(n) as usize,
    ) as *mut idx_t;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__imalloc(
    mut n: size_t,
    mut msg: *mut libc::c_char,
) -> *mut idx_t {
    return gk_malloc(
        (::core::mem::size_of::<idx_t>() as u64).wrapping_mul(n),
        msg,
    ) as *mut idx_t;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__irealloc(
    mut ptr: *mut idx_t,
    mut n: size_t,
    mut msg: *mut libc::c_char,
) -> *mut idx_t {
    return gk_realloc(
        ptr as *mut libc::c_void,
        (::core::mem::size_of::<idx_t>() as u64).wrapping_mul(n),
        msg,
    ) as *mut idx_t;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__iAllocMatrix(
    mut ndim1: size_t,
    mut ndim2: size_t,
    mut value: idx_t,
    mut errmsg: *mut libc::c_char,
) -> *mut *mut idx_t {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut matrix: *mut *mut idx_t = 0 as *mut *mut idx_t;
    matrix = gk_malloc(
        ndim1.wrapping_mul(::core::mem::size_of::<*mut idx_t>() as u64),
        errmsg,
    ) as *mut *mut idx_t;
    if matrix.is_null() {
        return 0 as *mut *mut idx_t;
    }
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < ndim1 {
        let ref mut fresh0 = *matrix.offset(i as isize);
        *fresh0 = libmetis__ismalloc(ndim2, value, errmsg);
        if (*matrix.offset(i as isize)).is_null() {
            j = 0 as libc::c_int as gk_idx_t;
            while j < i {
                gk_free(
                    &mut *matrix.offset(j as isize) as *mut *mut idx_t as *mut *mut libc::c_void,
                    0 as *mut *mut libc::c_void,
                );
                j += 1;
                j;
            }
            return 0 as *mut *mut idx_t;
        }
        i += 1;
        i;
    }
    return matrix;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__rFreeMatrix(
    mut r_matrix: *mut *mut *mut real_t,
    mut ndim1: size_t,
    mut ndim2: size_t,
) {
    let mut i: gk_idx_t = 0;
    let mut matrix: *mut *mut real_t = 0 as *mut *mut real_t;
    if (*r_matrix).is_null() {
        return;
    }
    matrix = *r_matrix;
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < ndim1 {
        gk_free(
            &mut *matrix.offset(i as isize) as *mut *mut real_t as *mut *mut libc::c_void,
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
pub unsafe extern "C" fn libmetis__rSetMatrix(
    mut matrix: *mut *mut real_t,
    mut ndim1: size_t,
    mut ndim2: size_t,
    mut value: real_t,
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
pub unsafe extern "C" fn libmetis__rsmalloc(
    mut n: size_t,
    mut ival: real_t,
    mut msg: *mut libc::c_char,
) -> *mut real_t {
    let mut ptr: *mut real_t = 0 as *mut real_t;
    ptr = gk_malloc(
        (::core::mem::size_of::<real_t>() as u64).wrapping_mul(n),
        msg,
    ) as *mut real_t;
    if ptr.is_null() {
        return 0 as *mut real_t;
    }
    return libmetis__rset(n, ival, ptr);
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__rset(
    mut n: size_t,
    mut val: real_t,
    mut x: *mut real_t,
) -> *mut real_t {
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
pub unsafe extern "C" fn libmetis__rcopy(
    mut n: size_t,
    mut a: *mut real_t,
    mut b: *mut real_t,
) -> *mut real_t {
    return memmove(
        b as *mut libc::c_void,
        a as *mut libc::c_void,
        (::core::mem::size_of::<real_t>() as u64).wrapping_mul(n) as usize,
    ) as *mut real_t;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__rmalloc(
    mut n: size_t,
    mut msg: *mut libc::c_char,
) -> *mut real_t {
    return gk_malloc(
        (::core::mem::size_of::<real_t>() as u64).wrapping_mul(n),
        msg,
    ) as *mut real_t;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__rrealloc(
    mut ptr: *mut real_t,
    mut n: size_t,
    mut msg: *mut libc::c_char,
) -> *mut real_t {
    return gk_realloc(
        ptr as *mut libc::c_void,
        (::core::mem::size_of::<real_t>() as u64).wrapping_mul(n),
        msg,
    ) as *mut real_t;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__rAllocMatrix(
    mut ndim1: size_t,
    mut ndim2: size_t,
    mut value: real_t,
    mut errmsg: *mut libc::c_char,
) -> *mut *mut real_t {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut matrix: *mut *mut real_t = 0 as *mut *mut real_t;
    matrix = gk_malloc(
        ndim1.wrapping_mul(::core::mem::size_of::<*mut real_t>() as u64),
        errmsg,
    ) as *mut *mut real_t;
    if matrix.is_null() {
        return 0 as *mut *mut real_t;
    }
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < ndim1 {
        let ref mut fresh1 = *matrix.offset(i as isize);
        *fresh1 = libmetis__rsmalloc(ndim2, value, errmsg);
        if (*matrix.offset(i as isize)).is_null() {
            j = 0 as libc::c_int as gk_idx_t;
            while j < i {
                gk_free(
                    &mut *matrix.offset(j as isize) as *mut *mut real_t as *mut *mut libc::c_void,
                    0 as *mut *mut libc::c_void,
                );
                j += 1;
                j;
            }
            return 0 as *mut *mut real_t;
        }
        i += 1;
        i;
    }
    return matrix;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__ikvSetMatrix(
    mut matrix: *mut *mut ikv_t,
    mut ndim1: size_t,
    mut ndim2: size_t,
    mut value: ikv_t,
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
pub unsafe extern "C" fn libmetis__ikvFreeMatrix(
    mut r_matrix: *mut *mut *mut ikv_t,
    mut ndim1: size_t,
    mut ndim2: size_t,
) {
    let mut i: gk_idx_t = 0;
    let mut matrix: *mut *mut ikv_t = 0 as *mut *mut ikv_t;
    if (*r_matrix).is_null() {
        return;
    }
    matrix = *r_matrix;
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < ndim1 {
        gk_free(
            &mut *matrix.offset(i as isize) as *mut *mut ikv_t as *mut *mut libc::c_void,
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
pub unsafe extern "C" fn libmetis__ikvcopy(
    mut n: size_t,
    mut a: *mut ikv_t,
    mut b: *mut ikv_t,
) -> *mut ikv_t {
    return memmove(
        b as *mut libc::c_void,
        a as *mut libc::c_void,
        (::core::mem::size_of::<ikv_t>() as u64).wrapping_mul(n) as usize,
    ) as *mut ikv_t;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__ikvsmalloc(
    mut n: size_t,
    mut ival: ikv_t,
    mut msg: *mut libc::c_char,
) -> *mut ikv_t {
    let mut ptr: *mut ikv_t = 0 as *mut ikv_t;
    ptr = gk_malloc(
        (::core::mem::size_of::<ikv_t>() as u64).wrapping_mul(n),
        msg,
    ) as *mut ikv_t;
    if ptr.is_null() {
        return 0 as *mut ikv_t;
    }
    return libmetis__ikvset(n, ival, ptr);
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__ikvrealloc(
    mut ptr: *mut ikv_t,
    mut n: size_t,
    mut msg: *mut libc::c_char,
) -> *mut ikv_t {
    return gk_realloc(
        ptr as *mut libc::c_void,
        (::core::mem::size_of::<ikv_t>() as u64).wrapping_mul(n),
        msg,
    ) as *mut ikv_t;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__ikvset(
    mut n: size_t,
    mut val: ikv_t,
    mut x: *mut ikv_t,
) -> *mut ikv_t {
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
pub unsafe extern "C" fn libmetis__ikvmalloc(
    mut n: size_t,
    mut msg: *mut libc::c_char,
) -> *mut ikv_t {
    return gk_malloc(
        (::core::mem::size_of::<ikv_t>() as u64).wrapping_mul(n),
        msg,
    ) as *mut ikv_t;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__ikvAllocMatrix(
    mut ndim1: size_t,
    mut ndim2: size_t,
    mut value: ikv_t,
    mut errmsg: *mut libc::c_char,
) -> *mut *mut ikv_t {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut matrix: *mut *mut ikv_t = 0 as *mut *mut ikv_t;
    matrix = gk_malloc(
        ndim1.wrapping_mul(::core::mem::size_of::<*mut ikv_t>() as u64),
        errmsg,
    ) as *mut *mut ikv_t;
    if matrix.is_null() {
        return 0 as *mut *mut ikv_t;
    }
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < ndim1 {
        let ref mut fresh2 = *matrix.offset(i as isize);
        *fresh2 = libmetis__ikvsmalloc(ndim2, value, errmsg);
        if (*matrix.offset(i as isize)).is_null() {
            j = 0 as libc::c_int as gk_idx_t;
            while j < i {
                gk_free(
                    &mut *matrix.offset(j as isize) as *mut *mut ikv_t as *mut *mut libc::c_void,
                    0 as *mut *mut libc::c_void,
                );
                j += 1;
                j;
            }
            return 0 as *mut *mut ikv_t;
        }
        i += 1;
        i;
    }
    return matrix;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__rkvFreeMatrix(
    mut r_matrix: *mut *mut *mut rkv_t,
    mut ndim1: size_t,
    mut ndim2: size_t,
) {
    let mut i: gk_idx_t = 0;
    let mut matrix: *mut *mut rkv_t = 0 as *mut *mut rkv_t;
    if (*r_matrix).is_null() {
        return;
    }
    matrix = *r_matrix;
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < ndim1 {
        gk_free(
            &mut *matrix.offset(i as isize) as *mut *mut rkv_t as *mut *mut libc::c_void,
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
pub unsafe extern "C" fn libmetis__rkvSetMatrix(
    mut matrix: *mut *mut rkv_t,
    mut ndim1: size_t,
    mut ndim2: size_t,
    mut value: rkv_t,
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
pub unsafe extern "C" fn libmetis__rkvAllocMatrix(
    mut ndim1: size_t,
    mut ndim2: size_t,
    mut value: rkv_t,
    mut errmsg: *mut libc::c_char,
) -> *mut *mut rkv_t {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut matrix: *mut *mut rkv_t = 0 as *mut *mut rkv_t;
    matrix = gk_malloc(
        ndim1.wrapping_mul(::core::mem::size_of::<*mut rkv_t>() as u64),
        errmsg,
    ) as *mut *mut rkv_t;
    if matrix.is_null() {
        return 0 as *mut *mut rkv_t;
    }
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < ndim1 {
        let ref mut fresh3 = *matrix.offset(i as isize);
        *fresh3 = libmetis__rkvsmalloc(ndim2, value, errmsg);
        if (*matrix.offset(i as isize)).is_null() {
            j = 0 as libc::c_int as gk_idx_t;
            while j < i {
                gk_free(
                    &mut *matrix.offset(j as isize) as *mut *mut rkv_t as *mut *mut libc::c_void,
                    0 as *mut *mut libc::c_void,
                );
                j += 1;
                j;
            }
            return 0 as *mut *mut rkv_t;
        }
        i += 1;
        i;
    }
    return matrix;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__rkvmalloc(
    mut n: size_t,
    mut msg: *mut libc::c_char,
) -> *mut rkv_t {
    return gk_malloc(
        (::core::mem::size_of::<rkv_t>() as u64).wrapping_mul(n),
        msg,
    ) as *mut rkv_t;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__rkvcopy(
    mut n: size_t,
    mut a: *mut rkv_t,
    mut b: *mut rkv_t,
) -> *mut rkv_t {
    return memmove(
        b as *mut libc::c_void,
        a as *mut libc::c_void,
        (::core::mem::size_of::<rkv_t>() as u64).wrapping_mul(n) as usize,
    ) as *mut rkv_t;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__rkvrealloc(
    mut ptr: *mut rkv_t,
    mut n: size_t,
    mut msg: *mut libc::c_char,
) -> *mut rkv_t {
    return gk_realloc(
        ptr as *mut libc::c_void,
        (::core::mem::size_of::<rkv_t>() as u64).wrapping_mul(n),
        msg,
    ) as *mut rkv_t;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__rkvsmalloc(
    mut n: size_t,
    mut ival: rkv_t,
    mut msg: *mut libc::c_char,
) -> *mut rkv_t {
    let mut ptr: *mut rkv_t = 0 as *mut rkv_t;
    ptr = gk_malloc(
        (::core::mem::size_of::<rkv_t>() as u64).wrapping_mul(n),
        msg,
    ) as *mut rkv_t;
    if ptr.is_null() {
        return 0 as *mut rkv_t;
    }
    return libmetis__rkvset(n, ival, ptr);
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__rkvset(
    mut n: size_t,
    mut val: rkv_t,
    mut x: *mut rkv_t,
) -> *mut rkv_t {
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
pub unsafe extern "C" fn libmetis__ipqInit(mut queue: *mut ipq_t, mut maxnodes: size_t) {
    (*queue).nnodes = 0 as libc::c_int as gk_idx_t;
    (*queue).maxnodes = maxnodes as gk_idx_t;
    (*queue).heap = libmetis__ikvmalloc(
        maxnodes,
        b"gk_PQInit: heap\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (*queue).locator = gk_idxsmalloc(
        maxnodes,
        -(1) as gk_idx_t,
        b"gk_PQInit: locator\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__ipqCheckHeap(mut queue: *mut ipq_t) -> libc::c_int {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut nnodes: size_t = 0;
    let mut locator: *mut gk_idx_t = 0 as *mut gk_idx_t;
    let mut heap: *mut ikv_t = 0 as *mut ikv_t;
    heap = (*queue).heap;
    locator = (*queue).locator;
    nnodes = (*queue).nnodes as size_t;
    if nnodes == 0 as libc::c_int as u64 {
        return 1;
    }
    i = 1 as gk_idx_t;
    while (i as u64) < nnodes {
        i += 1;
        i;
    }
    i = 1 as gk_idx_t;
    while (i as u64) < nnodes {
        i += 1;
        i;
    }
    i = 0 as libc::c_int as gk_idx_t;
    j = i;
    while i < (*queue).maxnodes {
        if *locator.offset(i as isize) != -(1) as i64 {
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    return 1;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__ipqReset(mut queue: *mut ipq_t) {
    let mut i: gk_idx_t = 0;
    let mut locator: *mut gk_idx_t = (*queue).locator;
    let mut heap: *mut ikv_t = (*queue).heap;
    i = (*queue).nnodes - 1 as i64;
    while i >= 0 as libc::c_int as i64 {
        *locator.offset((*heap.offset(i as isize)).val as isize) = -(1) as gk_idx_t;
        i -= 1;
        i;
    }
    (*queue).nnodes = 0 as libc::c_int as gk_idx_t;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__ipqFree(mut queue: *mut ipq_t) {
    if queue.is_null() {
        return;
    }
    gk_free(
        &mut (*queue).heap as *mut *mut ikv_t as *mut *mut libc::c_void,
        &mut (*queue).locator as *mut *mut gk_idx_t,
        0 as *mut *mut libc::c_void,
    );
    (*queue).maxnodes = 0 as libc::c_int as gk_idx_t;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__ipqDestroy(mut queue: *mut ipq_t) {
    if queue.is_null() {
        return;
    }
    libmetis__ipqFree(queue);
    gk_free(
        &mut queue as *mut *mut ipq_t as *mut *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__ipqLength(mut queue: *mut ipq_t) -> size_t {
    return (*queue).nnodes as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__ipqInsert(
    mut queue: *mut ipq_t,
    mut node: idx_t,
    mut key: idx_t,
) -> libc::c_int {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut locator: *mut gk_idx_t = (*queue).locator;
    let mut heap: *mut ikv_t = (*queue).heap;
    let fresh4 = (*queue).nnodes;
    (*queue).nnodes = (*queue).nnodes + 1;
    i = fresh4;
    while i > 0 as libc::c_int as i64 {
        j = i - 1 as i64 >> 1;
        if !(key > (*heap.offset(j as isize)).key) {
            break;
        }
        *heap.offset(i as isize) = *heap.offset(j as isize);
        *locator.offset((*heap.offset(i as isize)).val as isize) = i;
        i = j;
    }
    (*heap.offset(i as isize)).key = key;
    (*heap.offset(i as isize)).val = node;
    *locator.offset(node as isize) = i;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__ipqDelete(
    mut queue: *mut ipq_t,
    mut node: idx_t,
) -> libc::c_int {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut nnodes: gk_idx_t = 0;
    let mut newkey: idx_t = 0;
    let mut oldkey: idx_t = 0;
    let mut locator: *mut gk_idx_t = (*queue).locator;
    let mut heap: *mut ikv_t = (*queue).heap;
    i = *locator.offset(node as isize);
    *locator.offset(node as isize) = -(1) as gk_idx_t;
    (*queue).nnodes -= 1;
    if (*queue).nnodes > 0 as libc::c_int as i64
        && (*heap.offset((*queue).nnodes as isize)).val != node
    {
        node = (*heap.offset((*queue).nnodes as isize)).val;
        newkey = (*heap.offset((*queue).nnodes as isize)).key;
        oldkey = (*heap.offset(i as isize)).key;
        if newkey > oldkey {
            while i > 0 as libc::c_int as i64 {
                j = i - 1 as i64 >> 1;
                if !(newkey > (*heap.offset(j as isize)).key) {
                    break;
                }
                *heap.offset(i as isize) = *heap.offset(j as isize);
                *locator.offset((*heap.offset(i as isize)).val as isize) = i;
                i = j;
            }
        } else {
            nnodes = (*queue).nnodes;
            loop {
                j = (i << 1) + 1 as i64;
                if !(j < nnodes) {
                    break;
                }
                if (*heap.offset(j as isize)).key > newkey {
                    if (j + 1 as i64) < nnodes
                        && (*heap.offset((j + 1 as i64) as isize)).key
                            > (*heap.offset(j as isize)).key
                    {
                        j += 1;
                        j;
                    }
                    *heap.offset(i as isize) = *heap.offset(j as isize);
                    *locator.offset((*heap.offset(i as isize)).val as isize) = i;
                    i = j;
                } else {
                    if !((j + 1 as i64) < nnodes
                        && (*heap.offset((j + 1 as i64) as isize)).key > newkey)
                    {
                        break;
                    }
                    j += 1;
                    j;
                    *heap.offset(i as isize) = *heap.offset(j as isize);
                    *locator.offset((*heap.offset(i as isize)).val as isize) = i;
                    i = j;
                }
            }
        }
        (*heap.offset(i as isize)).key = newkey;
        (*heap.offset(i as isize)).val = node;
        *locator.offset(node as isize) = i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__ipqUpdate(
    mut queue: *mut ipq_t,
    mut node: idx_t,
    mut newkey: idx_t,
) {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut nnodes: gk_idx_t = 0;
    let mut oldkey: idx_t = 0;
    let mut locator: *mut gk_idx_t = (*queue).locator;
    let mut heap: *mut ikv_t = (*queue).heap;
    oldkey = (*heap.offset(*locator.offset(node as isize) as isize)).key;
    i = *locator.offset(node as isize);
    if newkey > oldkey {
        while i > 0 as libc::c_int as i64 {
            j = i - 1 as i64 >> 1;
            if !(newkey > (*heap.offset(j as isize)).key) {
                break;
            }
            *heap.offset(i as isize) = *heap.offset(j as isize);
            *locator.offset((*heap.offset(i as isize)).val as isize) = i;
            i = j;
        }
    } else {
        nnodes = (*queue).nnodes;
        loop {
            j = (i << 1) + 1 as i64;
            if !(j < nnodes) {
                break;
            }
            if (*heap.offset(j as isize)).key > newkey {
                if (j + 1 as i64) < nnodes
                    && (*heap.offset((j + 1 as i64) as isize)).key
                        > (*heap.offset(j as isize)).key
                {
                    j += 1;
                    j;
                }
                *heap.offset(i as isize) = *heap.offset(j as isize);
                *locator.offset((*heap.offset(i as isize)).val as isize) = i;
                i = j;
            } else {
                if !((j + 1 as i64) < nnodes
                    && (*heap.offset((j + 1 as i64) as isize)).key > newkey)
                {
                    break;
                }
                j += 1;
                j;
                *heap.offset(i as isize) = *heap.offset(j as isize);
                *locator.offset((*heap.offset(i as isize)).val as isize) = i;
                i = j;
            }
        }
    }
    (*heap.offset(i as isize)).key = newkey;
    (*heap.offset(i as isize)).val = node;
    *locator.offset(node as isize) = i;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__ipqCreate(mut maxnodes: size_t) -> *mut ipq_t {
    let mut queue: *mut ipq_t = 0 as *mut ipq_t;
    queue = gk_malloc(
        ::core::mem::size_of::<ipq_t>() as u64,
        b"gk_pqCreate: queue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut ipq_t;
    libmetis__ipqInit(queue, maxnodes);
    return queue;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__ipqSeeKey(mut queue: *mut ipq_t, mut node: idx_t) -> idx_t {
    let mut locator: *mut gk_idx_t = 0 as *mut gk_idx_t;
    let mut heap: *mut ikv_t = 0 as *mut ikv_t;
    heap = (*queue).heap;
    locator = (*queue).locator;
    return (*heap.offset(*locator.offset(node as isize) as isize)).key;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__ipqSeeTopKey(mut queue: *mut ipq_t) -> idx_t {
    return if (*queue).nnodes == 0 as libc::c_int as i64 {
        2147483647 as libc::c_int
    } else {
        (*((*queue).heap).offset(0 as libc::c_int as isize)).key
    };
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__ipqSeeTopVal(mut queue: *mut ipq_t) -> idx_t {
    return if (*queue).nnodes == 0 as libc::c_int as i64 {
        -(1)
    } else {
        (*((*queue).heap).offset(0 as libc::c_int as isize)).val
    };
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__ipqGetTop(mut queue: *mut ipq_t) -> idx_t {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut locator: *mut gk_idx_t = 0 as *mut gk_idx_t;
    let mut heap: *mut ikv_t = 0 as *mut ikv_t;
    let mut vtx: idx_t = 0;
    let mut node: idx_t = 0;
    let mut key: idx_t = 0;
    if (*queue).nnodes == 0 as libc::c_int as i64 {
        return -(1);
    }
    (*queue).nnodes -= 1;
    (*queue).nnodes;
    heap = (*queue).heap;
    locator = (*queue).locator;
    vtx = (*heap.offset(0 as libc::c_int as isize)).val;
    *locator.offset(vtx as isize) = -(1) as gk_idx_t;
    i = (*queue).nnodes;
    if i > 0 as libc::c_int as i64 {
        key = (*heap.offset(i as isize)).key;
        node = (*heap.offset(i as isize)).val;
        i = 0 as libc::c_int as gk_idx_t;
        loop {
            j = 2 as libc::c_int as i64 * i + 1 as i64;
            if !(j < (*queue).nnodes) {
                break;
            }
            if (*heap.offset(j as isize)).key > key {
                if (j + 1 as i64) < (*queue).nnodes
                    && (*heap.offset((j + 1 as i64) as isize)).key
                        > (*heap.offset(j as isize)).key
                {
                    j = j + 1 as i64;
                }
                *heap.offset(i as isize) = *heap.offset(j as isize);
                *locator.offset((*heap.offset(i as isize)).val as isize) = i;
                i = j;
            } else {
                if !((j + 1 as i64) < (*queue).nnodes
                    && (*heap.offset((j + 1 as i64) as isize)).key > key)
                {
                    break;
                }
                j = j + 1 as i64;
                *heap.offset(i as isize) = *heap.offset(j as isize);
                *locator.offset((*heap.offset(i as isize)).val as isize) = i;
                i = j;
            }
        }
        (*heap.offset(i as isize)).key = key;
        (*heap.offset(i as isize)).val = node;
        *locator.offset(node as isize) = i;
    }
    return vtx;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__rpqUpdate(
    mut queue: *mut rpq_t,
    mut node: idx_t,
    mut newkey: real_t,
) {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut nnodes: gk_idx_t = 0;
    let mut oldkey: real_t = 0.;
    let mut locator: *mut gk_idx_t = (*queue).locator;
    let mut heap: *mut rkv_t = (*queue).heap;
    oldkey = (*heap.offset(*locator.offset(node as isize) as isize)).key;
    i = *locator.offset(node as isize);
    if newkey > oldkey {
        while i > 0 as libc::c_int as i64 {
            j = i - 1 as i64 >> 1;
            if !(newkey > (*heap.offset(j as isize)).key) {
                break;
            }
            *heap.offset(i as isize) = *heap.offset(j as isize);
            *locator.offset((*heap.offset(i as isize)).val as isize) = i;
            i = j;
        }
    } else {
        nnodes = (*queue).nnodes;
        loop {
            j = (i << 1) + 1 as i64;
            if !(j < nnodes) {
                break;
            }
            if (*heap.offset(j as isize)).key > newkey {
                if (j + 1 as i64) < nnodes
                    && (*heap.offset((j + 1 as i64) as isize)).key
                        > (*heap.offset(j as isize)).key
                {
                    j += 1;
                    j;
                }
                *heap.offset(i as isize) = *heap.offset(j as isize);
                *locator.offset((*heap.offset(i as isize)).val as isize) = i;
                i = j;
            } else {
                if !((j + 1 as i64) < nnodes
                    && (*heap.offset((j + 1 as i64) as isize)).key > newkey)
                {
                    break;
                }
                j += 1;
                j;
                *heap.offset(i as isize) = *heap.offset(j as isize);
                *locator.offset((*heap.offset(i as isize)).val as isize) = i;
                i = j;
            }
        }
    }
    (*heap.offset(i as isize)).key = newkey;
    (*heap.offset(i as isize)).val = node;
    *locator.offset(node as isize) = i;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__rpqCheckHeap(mut queue: *mut rpq_t) -> libc::c_int {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut nnodes: size_t = 0;
    let mut locator: *mut gk_idx_t = 0 as *mut gk_idx_t;
    let mut heap: *mut rkv_t = 0 as *mut rkv_t;
    heap = (*queue).heap;
    locator = (*queue).locator;
    nnodes = (*queue).nnodes as size_t;
    if nnodes == 0 as libc::c_int as u64 {
        return 1;
    }
    i = 1 as gk_idx_t;
    while (i as u64) < nnodes {
        i += 1;
        i;
    }
    i = 1 as gk_idx_t;
    while (i as u64) < nnodes {
        i += 1;
        i;
    }
    i = 0 as libc::c_int as gk_idx_t;
    j = i;
    while i < (*queue).maxnodes {
        if *locator.offset(i as isize) != -(1) as i64 {
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    return 1;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__rpqInit(mut queue: *mut rpq_t, mut maxnodes: size_t) {
    (*queue).nnodes = 0 as libc::c_int as gk_idx_t;
    (*queue).maxnodes = maxnodes as gk_idx_t;
    (*queue).heap = libmetis__rkvmalloc(
        maxnodes,
        b"gk_PQInit: heap\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (*queue).locator = gk_idxsmalloc(
        maxnodes,
        -(1) as gk_idx_t,
        b"gk_PQInit: locator\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__rpqReset(mut queue: *mut rpq_t) {
    let mut i: gk_idx_t = 0;
    let mut locator: *mut gk_idx_t = (*queue).locator;
    let mut heap: *mut rkv_t = (*queue).heap;
    i = (*queue).nnodes - 1 as i64;
    while i >= 0 as libc::c_int as i64 {
        *locator.offset((*heap.offset(i as isize)).val as isize) = -(1) as gk_idx_t;
        i -= 1;
        i;
    }
    (*queue).nnodes = 0 as libc::c_int as gk_idx_t;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__rpqFree(mut queue: *mut rpq_t) {
    if queue.is_null() {
        return;
    }
    gk_free(
        &mut (*queue).heap as *mut *mut rkv_t as *mut *mut libc::c_void,
        &mut (*queue).locator as *mut *mut gk_idx_t,
        0 as *mut *mut libc::c_void,
    );
    (*queue).maxnodes = 0 as libc::c_int as gk_idx_t;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__rpqDestroy(mut queue: *mut rpq_t) {
    if queue.is_null() {
        return;
    }
    libmetis__rpqFree(queue);
    gk_free(
        &mut queue as *mut *mut rpq_t as *mut *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__rpqLength(mut queue: *mut rpq_t) -> size_t {
    return (*queue).nnodes as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__rpqInsert(
    mut queue: *mut rpq_t,
    mut node: idx_t,
    mut key: real_t,
) -> libc::c_int {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut locator: *mut gk_idx_t = (*queue).locator;
    let mut heap: *mut rkv_t = (*queue).heap;
    let fresh5 = (*queue).nnodes;
    (*queue).nnodes = (*queue).nnodes + 1;
    i = fresh5;
    while i > 0 as libc::c_int as i64 {
        j = i - 1 as i64 >> 1;
        if !(key > (*heap.offset(j as isize)).key) {
            break;
        }
        *heap.offset(i as isize) = *heap.offset(j as isize);
        *locator.offset((*heap.offset(i as isize)).val as isize) = i;
        i = j;
    }
    (*heap.offset(i as isize)).key = key;
    (*heap.offset(i as isize)).val = node;
    *locator.offset(node as isize) = i;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__rpqDelete(
    mut queue: *mut rpq_t,
    mut node: idx_t,
) -> libc::c_int {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut nnodes: gk_idx_t = 0;
    let mut newkey: real_t = 0.;
    let mut oldkey: real_t = 0.;
    let mut locator: *mut gk_idx_t = (*queue).locator;
    let mut heap: *mut rkv_t = (*queue).heap;
    i = *locator.offset(node as isize);
    *locator.offset(node as isize) = -(1) as gk_idx_t;
    (*queue).nnodes -= 1;
    if (*queue).nnodes > 0 as libc::c_int as i64
        && (*heap.offset((*queue).nnodes as isize)).val != node
    {
        node = (*heap.offset((*queue).nnodes as isize)).val;
        newkey = (*heap.offset((*queue).nnodes as isize)).key;
        oldkey = (*heap.offset(i as isize)).key;
        if newkey > oldkey {
            while i > 0 as libc::c_int as i64 {
                j = i - 1 as i64 >> 1;
                if !(newkey > (*heap.offset(j as isize)).key) {
                    break;
                }
                *heap.offset(i as isize) = *heap.offset(j as isize);
                *locator.offset((*heap.offset(i as isize)).val as isize) = i;
                i = j;
            }
        } else {
            nnodes = (*queue).nnodes;
            loop {
                j = (i << 1) + 1 as i64;
                if !(j < nnodes) {
                    break;
                }
                if (*heap.offset(j as isize)).key > newkey {
                    if (j + 1 as i64) < nnodes
                        && (*heap.offset((j + 1 as i64) as isize)).key
                            > (*heap.offset(j as isize)).key
                    {
                        j += 1;
                        j;
                    }
                    *heap.offset(i as isize) = *heap.offset(j as isize);
                    *locator.offset((*heap.offset(i as isize)).val as isize) = i;
                    i = j;
                } else {
                    if !((j + 1 as i64) < nnodes
                        && (*heap.offset((j + 1 as i64) as isize)).key > newkey)
                    {
                        break;
                    }
                    j += 1;
                    j;
                    *heap.offset(i as isize) = *heap.offset(j as isize);
                    *locator.offset((*heap.offset(i as isize)).val as isize) = i;
                    i = j;
                }
            }
        }
        (*heap.offset(i as isize)).key = newkey;
        (*heap.offset(i as isize)).val = node;
        *locator.offset(node as isize) = i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__rpqCreate(mut maxnodes: size_t) -> *mut rpq_t {
    let mut queue: *mut rpq_t = 0 as *mut rpq_t;
    queue = gk_malloc(
        ::core::mem::size_of::<rpq_t>() as u64,
        b"gk_pqCreate: queue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut rpq_t;
    libmetis__rpqInit(queue, maxnodes);
    return queue;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__rpqSeeKey(mut queue: *mut rpq_t, mut node: idx_t) -> real_t {
    let mut locator: *mut gk_idx_t = 0 as *mut gk_idx_t;
    let mut heap: *mut rkv_t = 0 as *mut rkv_t;
    heap = (*queue).heap;
    locator = (*queue).locator;
    return (*heap.offset(*locator.offset(node as isize) as isize)).key;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__rpqSeeTopKey(mut queue: *mut rpq_t) -> real_t {
    return if (*queue).nnodes == 0 as libc::c_int as i64 {
        3.40282347e+38f32
    } else {
        (*((*queue).heap).offset(0 as libc::c_int as isize)).key
    };
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__rpqSeeTopVal(mut queue: *mut rpq_t) -> idx_t {
    return if (*queue).nnodes == 0 as libc::c_int as i64 {
        -(1)
    } else {
        (*((*queue).heap).offset(0 as libc::c_int as isize)).val
    };
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__rpqGetTop(mut queue: *mut rpq_t) -> idx_t {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut locator: *mut gk_idx_t = 0 as *mut gk_idx_t;
    let mut heap: *mut rkv_t = 0 as *mut rkv_t;
    let mut vtx: idx_t = 0;
    let mut node: idx_t = 0;
    let mut key: real_t = 0.;
    if (*queue).nnodes == 0 as libc::c_int as i64 {
        return -(1);
    }
    (*queue).nnodes -= 1;
    (*queue).nnodes;
    heap = (*queue).heap;
    locator = (*queue).locator;
    vtx = (*heap.offset(0 as libc::c_int as isize)).val;
    *locator.offset(vtx as isize) = -(1) as gk_idx_t;
    i = (*queue).nnodes;
    if i > 0 as libc::c_int as i64 {
        key = (*heap.offset(i as isize)).key;
        node = (*heap.offset(i as isize)).val;
        i = 0 as libc::c_int as gk_idx_t;
        loop {
            j = 2 as libc::c_int as i64 * i + 1 as i64;
            if !(j < (*queue).nnodes) {
                break;
            }
            if (*heap.offset(j as isize)).key > key {
                if (j + 1 as i64) < (*queue).nnodes
                    && (*heap.offset((j + 1 as i64) as isize)).key
                        > (*heap.offset(j as isize)).key
                {
                    j = j + 1 as i64;
                }
                *heap.offset(i as isize) = *heap.offset(j as isize);
                *locator.offset((*heap.offset(i as isize)).val as isize) = i;
                i = j;
            } else {
                if !((j + 1 as i64) < (*queue).nnodes
                    && (*heap.offset((j + 1 as i64) as isize)).key > key)
                {
                    break;
                }
                j = j + 1 as i64;
                *heap.offset(i as isize) = *heap.offset(j as isize);
                *locator.offset((*heap.offset(i as isize)).val as isize) = i;
                i = j;
            }
        }
        (*heap.offset(i as isize)).key = key;
        (*heap.offset(i as isize)).val = node;
        *locator.offset(node as isize) = i;
    }
    return vtx;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__isrand(mut seed: idx_t) {
    gk_randinit(seed as uint64_t);
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__irandArrayPermute(
    mut n: idx_t,
    mut p: *mut idx_t,
    mut nshuffles: idx_t,
    mut flag: libc::c_int,
) {
    let mut i: idx_t = 0;
    let mut u: idx_t = 0;
    let mut v: idx_t = 0;
    let mut tmp: idx_t = 0;
    if flag == 1 {
        i = 0 as libc::c_int;
        while i < n {
            *p.offset(i as isize) = i;
            i += 1;
            i;
        }
    }
    if n < 10 as libc::c_int {
        i = 0 as libc::c_int;
        while i < n {
            v = libmetis__irandInRange(n);
            u = libmetis__irandInRange(n);
            tmp = *p.offset(v as isize);
            *p.offset(v as isize) = *p.offset(u as isize);
            *p.offset(u as isize) = tmp;
            i += 1;
            i;
        }
    } else {
        i = 0 as libc::c_int;
        while i < nshuffles {
            v = libmetis__irandInRange(n - 3 as libc::c_int);
            u = libmetis__irandInRange(n - 3 as libc::c_int);
            tmp = *p.offset((v + 0 as libc::c_int) as isize);
            *p.offset((v + 0 as libc::c_int) as isize) = *p.offset((u + 2 as libc::c_int) as isize);
            *p.offset((u + 2 as libc::c_int) as isize) = tmp;
            tmp = *p.offset((v + 1) as isize);
            *p.offset((v + 1) as isize) = *p.offset((u + 3 as libc::c_int) as isize);
            *p.offset((u + 3 as libc::c_int) as isize) = tmp;
            tmp = *p.offset((v + 2 as libc::c_int) as isize);
            *p.offset((v + 2 as libc::c_int) as isize) = *p.offset((u + 0 as libc::c_int) as isize);
            *p.offset((u + 0 as libc::c_int) as isize) = tmp;
            tmp = *p.offset((v + 3 as libc::c_int) as isize);
            *p.offset((v + 3 as libc::c_int) as isize) = *p.offset((u + 1) as isize);
            *p.offset((u + 1) as isize) = tmp;
            i += 1;
            i;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__irandArrayPermuteFine(
    mut n: idx_t,
    mut p: *mut idx_t,
    mut flag: libc::c_int,
) {
    let mut i: idx_t = 0;
    let mut v: idx_t = 0;
    let mut tmp: idx_t = 0;
    if flag == 1 {
        i = 0 as libc::c_int;
        while i < n {
            *p.offset(i as isize) = i;
            i += 1;
            i;
        }
    }
    i = 0 as libc::c_int;
    while i < n {
        v = libmetis__irandInRange(n);
        tmp = *p.offset(i as isize);
        *p.offset(i as isize) = *p.offset(v as isize);
        *p.offset(v as isize) = tmp;
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__irand() -> idx_t {
    if ::core::mem::size_of::<idx_t>() as u64 <= ::core::mem::size_of::<int32_t>() as u64 {
        return gk_randint32() as idx_t;
    } else {
        return gk_randint64() as idx_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__irandInRange(mut max: idx_t) -> idx_t {
    return libmetis__irand() % max;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__iarray2csr(
    mut n: idx_t,
    mut range: idx_t,
    mut array: *mut idx_t,
    mut ptr: *mut idx_t,
    mut ind: *mut idx_t,
) {
    let mut i: idx_t = 0;
    i = 0 as libc::c_int;
    while i <= range {
        *ptr.offset(i as isize) = 0 as libc::c_int;
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < n {
        let ref mut fresh6 = *ptr.offset(*array.offset(i as isize) as isize);
        *fresh6 += 1;
        *fresh6;
        i += 1;
        i;
    }
    i = 1;
    while i < range {
        let ref mut fresh7 = *ptr.offset(i as isize);
        *fresh7 += *ptr.offset((i - 1) as isize);
        i += 1;
        i;
    }
    i = range;
    while i > 0 as libc::c_int {
        *ptr.offset(i as isize) = *ptr.offset((i - 1) as isize);
        i -= 1;
        i;
    }
    *ptr.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < n {
        let ref mut fresh8 = *ptr.offset(*array.offset(i as isize) as isize);
        let fresh9 = *fresh8;
        *fresh8 = *fresh8 + 1;
        *ind.offset(fresh9 as isize) = i;
        i += 1;
        i;
    }
    i = range;
    while i > 0 as libc::c_int {
        *ptr.offset(i as isize) = *ptr.offset((i - 1) as isize);
        i -= 1;
        i;
    }
    *ptr.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__isorti(mut n: size_t, mut base: *mut idx_t) {
    let _base: *mut idx_t = base;
    let _elems: size_t = n;
    let mut _hold: idx_t = 0;
    if _elems == 0 as libc::c_int as u64 {
        return;
    }
    if _elems > 4 as libc::c_int as u64 {
        let mut _lo: *mut idx_t = _base;
        let mut _hi: *mut idx_t = _lo
            .offset(_elems as isize)
            .offset(-(1 as isize));
        let mut _stack: [C2RustUnnamed_1; 64] = [C2RustUnnamed_1 {
            _hi: 0 as *mut idx_t,
            _lo: 0 as *mut idx_t,
        }; 64];
        let mut _top: *mut C2RustUnnamed_1 = _stack.as_mut_ptr().offset(1 as isize);
        while _stack.as_mut_ptr() < _top {
            let mut _left_ptr: *mut idx_t = 0 as *mut idx_t;
            let mut _right_ptr: *mut idx_t = 0 as *mut idx_t;
            let mut _mid: *mut idx_t =
                _lo.offset((_hi.offset_from(_lo) as i64 >> 1) as isize);
            if *_mid < *_lo {
                _hold = *_mid;
                *_mid = *_lo;
                *_lo = _hold;
            }
            if *_hi < *_mid {
                _hold = *_mid;
                *_mid = *_hi;
                *_hi = _hold;
                if *_mid < *_lo {
                    _hold = *_mid;
                    *_mid = *_lo;
                    *_lo = _hold;
                }
            }
            _left_ptr = _lo.offset(1 as isize);
            _right_ptr = _hi.offset(-(1 as isize));
            loop {
                while *_left_ptr < *_mid {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                }
                while *_mid < *_right_ptr {
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                }
                if _left_ptr < _right_ptr {
                    _hold = *_left_ptr;
                    *_left_ptr = *_right_ptr;
                    *_right_ptr = _hold;
                    if _mid == _left_ptr {
                        _mid = _right_ptr;
                    } else if _mid == _right_ptr {
                        _mid = _left_ptr;
                    }
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                } else if _left_ptr == _right_ptr {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                    break;
                }
                if !(_left_ptr <= _right_ptr) {
                    break;
                }
            }
            if _right_ptr.offset_from(_lo) as i64 <= 4 as libc::c_int as i64 {
                if _hi.offset_from(_left_ptr) as i64 <= 4 as libc::c_int as i64 {
                    _top = _top.offset(-1);
                    _top;
                    _lo = (*_top)._lo;
                    _hi = (*_top)._hi;
                } else {
                    _lo = _left_ptr;
                }
            } else if _hi.offset_from(_left_ptr) as i64 <= 4 as libc::c_int as i64 {
                _hi = _right_ptr;
            } else if _right_ptr.offset_from(_lo) as i64 > _hi.offset_from(_left_ptr) as i64 {
                (*_top)._lo = _lo;
                (*_top)._hi = _right_ptr;
                _top = _top.offset(1);
                _top;
                _lo = _left_ptr;
            } else {
                (*_top)._lo = _left_ptr;
                (*_top)._hi = _hi;
                _top = _top.offset(1);
                _top;
                _hi = _right_ptr;
            }
        }
    }
    let _end_ptr: *mut idx_t = _base
        .offset(_elems as isize)
        .offset(-(1 as isize));
    let mut _tmp_ptr: *mut idx_t = _base;
    let mut _run_ptr: *mut idx_t = 0 as *mut idx_t;
    let mut _thresh: *mut idx_t = 0 as *mut idx_t;
    _thresh = _base.offset(4 as libc::c_int as isize);
    if _thresh > _end_ptr {
        _thresh = _end_ptr;
    }
    _run_ptr = _tmp_ptr.offset(1 as isize);
    while _run_ptr <= _thresh {
        if *_run_ptr < *_tmp_ptr {
            _tmp_ptr = _run_ptr;
        }
        _run_ptr = _run_ptr.offset(1);
        _run_ptr;
    }
    if _tmp_ptr != _base {
        _hold = *_tmp_ptr;
        *_tmp_ptr = *_base;
        *_base = _hold;
    }
    _run_ptr = _base.offset(1 as isize);
    loop {
        _run_ptr = _run_ptr.offset(1);
        if !(_run_ptr <= _end_ptr) {
            break;
        }
        _tmp_ptr = _run_ptr.offset(-(1 as isize));
        while *_run_ptr < *_tmp_ptr {
            _tmp_ptr = _tmp_ptr.offset(-1);
            _tmp_ptr;
        }
        _tmp_ptr = _tmp_ptr.offset(1);
        _tmp_ptr;
        if _tmp_ptr != _run_ptr {
            let mut _trav: *mut idx_t = _run_ptr.offset(1 as isize);
            loop {
                _trav = _trav.offset(-1);
                if !(_trav >= _run_ptr) {
                    break;
                }
                let mut _hi_0: *mut idx_t = 0 as *mut idx_t;
                let mut _lo_0: *mut idx_t = 0 as *mut idx_t;
                _hold = *_trav;
                _lo_0 = _trav;
                _hi_0 = _lo_0;
                loop {
                    _lo_0 = _lo_0.offset(-1);
                    if !(_lo_0 >= _tmp_ptr) {
                        break;
                    }
                    *_hi_0 = *_lo_0;
                    _hi_0 = _lo_0;
                }
                *_hi_0 = _hold;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__isortd(mut n: size_t, mut base: *mut idx_t) {
    let _base: *mut idx_t = base;
    let _elems: size_t = n;
    let mut _hold: idx_t = 0;
    if _elems == 0 as libc::c_int as u64 {
        return;
    }
    if _elems > 4 as libc::c_int as u64 {
        let mut _lo: *mut idx_t = _base;
        let mut _hi: *mut idx_t = _lo
            .offset(_elems as isize)
            .offset(-(1 as isize));
        let mut _stack: [C2RustUnnamed_2; 64] = [C2RustUnnamed_2 {
            _hi: 0 as *mut idx_t,
            _lo: 0 as *mut idx_t,
        }; 64];
        let mut _top: *mut C2RustUnnamed_2 = _stack.as_mut_ptr().offset(1 as isize);
        while _stack.as_mut_ptr() < _top {
            let mut _left_ptr: *mut idx_t = 0 as *mut idx_t;
            let mut _right_ptr: *mut idx_t = 0 as *mut idx_t;
            let mut _mid: *mut idx_t =
                _lo.offset((_hi.offset_from(_lo) as i64 >> 1) as isize);
            if *_mid > *_lo {
                _hold = *_mid;
                *_mid = *_lo;
                *_lo = _hold;
            }
            if *_hi > *_mid {
                _hold = *_mid;
                *_mid = *_hi;
                *_hi = _hold;
                if *_mid > *_lo {
                    _hold = *_mid;
                    *_mid = *_lo;
                    *_lo = _hold;
                }
            }
            _left_ptr = _lo.offset(1 as isize);
            _right_ptr = _hi.offset(-(1 as isize));
            loop {
                while *_left_ptr > *_mid {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                }
                while *_mid > *_right_ptr {
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                }
                if _left_ptr < _right_ptr {
                    _hold = *_left_ptr;
                    *_left_ptr = *_right_ptr;
                    *_right_ptr = _hold;
                    if _mid == _left_ptr {
                        _mid = _right_ptr;
                    } else if _mid == _right_ptr {
                        _mid = _left_ptr;
                    }
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                } else if _left_ptr == _right_ptr {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                    break;
                }
                if !(_left_ptr <= _right_ptr) {
                    break;
                }
            }
            if _right_ptr.offset_from(_lo) as i64 <= 4 as libc::c_int as i64 {
                if _hi.offset_from(_left_ptr) as i64 <= 4 as libc::c_int as i64 {
                    _top = _top.offset(-1);
                    _top;
                    _lo = (*_top)._lo;
                    _hi = (*_top)._hi;
                } else {
                    _lo = _left_ptr;
                }
            } else if _hi.offset_from(_left_ptr) as i64 <= 4 as libc::c_int as i64 {
                _hi = _right_ptr;
            } else if _right_ptr.offset_from(_lo) as i64 > _hi.offset_from(_left_ptr) as i64 {
                (*_top)._lo = _lo;
                (*_top)._hi = _right_ptr;
                _top = _top.offset(1);
                _top;
                _lo = _left_ptr;
            } else {
                (*_top)._lo = _left_ptr;
                (*_top)._hi = _hi;
                _top = _top.offset(1);
                _top;
                _hi = _right_ptr;
            }
        }
    }
    let _end_ptr: *mut idx_t = _base
        .offset(_elems as isize)
        .offset(-(1 as isize));
    let mut _tmp_ptr: *mut idx_t = _base;
    let mut _run_ptr: *mut idx_t = 0 as *mut idx_t;
    let mut _thresh: *mut idx_t = 0 as *mut idx_t;
    _thresh = _base.offset(4 as libc::c_int as isize);
    if _thresh > _end_ptr {
        _thresh = _end_ptr;
    }
    _run_ptr = _tmp_ptr.offset(1 as isize);
    while _run_ptr <= _thresh {
        if *_run_ptr > *_tmp_ptr {
            _tmp_ptr = _run_ptr;
        }
        _run_ptr = _run_ptr.offset(1);
        _run_ptr;
    }
    if _tmp_ptr != _base {
        _hold = *_tmp_ptr;
        *_tmp_ptr = *_base;
        *_base = _hold;
    }
    _run_ptr = _base.offset(1 as isize);
    loop {
        _run_ptr = _run_ptr.offset(1);
        if !(_run_ptr <= _end_ptr) {
            break;
        }
        _tmp_ptr = _run_ptr.offset(-(1 as isize));
        while *_run_ptr > *_tmp_ptr {
            _tmp_ptr = _tmp_ptr.offset(-1);
            _tmp_ptr;
        }
        _tmp_ptr = _tmp_ptr.offset(1);
        _tmp_ptr;
        if _tmp_ptr != _run_ptr {
            let mut _trav: *mut idx_t = _run_ptr.offset(1 as isize);
            loop {
                _trav = _trav.offset(-1);
                if !(_trav >= _run_ptr) {
                    break;
                }
                let mut _hi_0: *mut idx_t = 0 as *mut idx_t;
                let mut _lo_0: *mut idx_t = 0 as *mut idx_t;
                _hold = *_trav;
                _lo_0 = _trav;
                _hi_0 = _lo_0;
                loop {
                    _lo_0 = _lo_0.offset(-1);
                    if !(_lo_0 >= _tmp_ptr) {
                        break;
                    }
                    *_hi_0 = *_lo_0;
                    _hi_0 = _lo_0;
                }
                *_hi_0 = _hold;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__rsorti(mut n: size_t, mut base: *mut real_t) {
    let _base: *mut real_t = base;
    let _elems: size_t = n;
    let mut _hold: real_t = 0.;
    if _elems == 0 as libc::c_int as u64 {
        return;
    }
    if _elems > 4 as libc::c_int as u64 {
        let mut _lo: *mut real_t = _base;
        let mut _hi: *mut real_t = _lo
            .offset(_elems as isize)
            .offset(-(1 as isize));
        let mut _stack: [C2RustUnnamed_3; 64] = [C2RustUnnamed_3 {
            _hi: 0 as *mut real_t,
            _lo: 0 as *mut real_t,
        }; 64];
        let mut _top: *mut C2RustUnnamed_3 = _stack.as_mut_ptr().offset(1 as isize);
        while _stack.as_mut_ptr() < _top {
            let mut _left_ptr: *mut real_t = 0 as *mut real_t;
            let mut _right_ptr: *mut real_t = 0 as *mut real_t;
            let mut _mid: *mut real_t =
                _lo.offset((_hi.offset_from(_lo) as i64 >> 1) as isize);
            if *_mid < *_lo {
                _hold = *_mid;
                *_mid = *_lo;
                *_lo = _hold;
            }
            if *_hi < *_mid {
                _hold = *_mid;
                *_mid = *_hi;
                *_hi = _hold;
                if *_mid < *_lo {
                    _hold = *_mid;
                    *_mid = *_lo;
                    *_lo = _hold;
                }
            }
            _left_ptr = _lo.offset(1 as isize);
            _right_ptr = _hi.offset(-(1 as isize));
            loop {
                while *_left_ptr < *_mid {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                }
                while *_mid < *_right_ptr {
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                }
                if _left_ptr < _right_ptr {
                    _hold = *_left_ptr;
                    *_left_ptr = *_right_ptr;
                    *_right_ptr = _hold;
                    if _mid == _left_ptr {
                        _mid = _right_ptr;
                    } else if _mid == _right_ptr {
                        _mid = _left_ptr;
                    }
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                } else if _left_ptr == _right_ptr {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                    break;
                }
                if !(_left_ptr <= _right_ptr) {
                    break;
                }
            }
            if _right_ptr.offset_from(_lo) as i64 <= 4 as libc::c_int as i64 {
                if _hi.offset_from(_left_ptr) as i64 <= 4 as libc::c_int as i64 {
                    _top = _top.offset(-1);
                    _top;
                    _lo = (*_top)._lo;
                    _hi = (*_top)._hi;
                } else {
                    _lo = _left_ptr;
                }
            } else if _hi.offset_from(_left_ptr) as i64 <= 4 as libc::c_int as i64 {
                _hi = _right_ptr;
            } else if _right_ptr.offset_from(_lo) as i64 > _hi.offset_from(_left_ptr) as i64 {
                (*_top)._lo = _lo;
                (*_top)._hi = _right_ptr;
                _top = _top.offset(1);
                _top;
                _lo = _left_ptr;
            } else {
                (*_top)._lo = _left_ptr;
                (*_top)._hi = _hi;
                _top = _top.offset(1);
                _top;
                _hi = _right_ptr;
            }
        }
    }
    let _end_ptr: *mut real_t = _base
        .offset(_elems as isize)
        .offset(-(1 as isize));
    let mut _tmp_ptr: *mut real_t = _base;
    let mut _run_ptr: *mut real_t = 0 as *mut real_t;
    let mut _thresh: *mut real_t = 0 as *mut real_t;
    _thresh = _base.offset(4 as libc::c_int as isize);
    if _thresh > _end_ptr {
        _thresh = _end_ptr;
    }
    _run_ptr = _tmp_ptr.offset(1 as isize);
    while _run_ptr <= _thresh {
        if *_run_ptr < *_tmp_ptr {
            _tmp_ptr = _run_ptr;
        }
        _run_ptr = _run_ptr.offset(1);
        _run_ptr;
    }
    if _tmp_ptr != _base {
        _hold = *_tmp_ptr;
        *_tmp_ptr = *_base;
        *_base = _hold;
    }
    _run_ptr = _base.offset(1 as isize);
    loop {
        _run_ptr = _run_ptr.offset(1);
        if !(_run_ptr <= _end_ptr) {
            break;
        }
        _tmp_ptr = _run_ptr.offset(-(1 as isize));
        while *_run_ptr < *_tmp_ptr {
            _tmp_ptr = _tmp_ptr.offset(-1);
            _tmp_ptr;
        }
        _tmp_ptr = _tmp_ptr.offset(1);
        _tmp_ptr;
        if _tmp_ptr != _run_ptr {
            let mut _trav: *mut real_t = _run_ptr.offset(1 as isize);
            loop {
                _trav = _trav.offset(-1);
                if !(_trav >= _run_ptr) {
                    break;
                }
                let mut _hi_0: *mut real_t = 0 as *mut real_t;
                let mut _lo_0: *mut real_t = 0 as *mut real_t;
                _hold = *_trav;
                _lo_0 = _trav;
                _hi_0 = _lo_0;
                loop {
                    _lo_0 = _lo_0.offset(-1);
                    if !(_lo_0 >= _tmp_ptr) {
                        break;
                    }
                    *_hi_0 = *_lo_0;
                    _hi_0 = _lo_0;
                }
                *_hi_0 = _hold;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__rsortd(mut n: size_t, mut base: *mut real_t) {
    let _base: *mut real_t = base;
    let _elems: size_t = n;
    let mut _hold: real_t = 0.;
    if _elems == 0 as libc::c_int as u64 {
        return;
    }
    if _elems > 4 as libc::c_int as u64 {
        let mut _lo: *mut real_t = _base;
        let mut _hi: *mut real_t = _lo
            .offset(_elems as isize)
            .offset(-(1 as isize));
        let mut _stack: [C2RustUnnamed_4; 64] = [C2RustUnnamed_4 {
            _hi: 0 as *mut real_t,
            _lo: 0 as *mut real_t,
        }; 64];
        let mut _top: *mut C2RustUnnamed_4 = _stack.as_mut_ptr().offset(1 as isize);
        while _stack.as_mut_ptr() < _top {
            let mut _left_ptr: *mut real_t = 0 as *mut real_t;
            let mut _right_ptr: *mut real_t = 0 as *mut real_t;
            let mut _mid: *mut real_t =
                _lo.offset((_hi.offset_from(_lo) as i64 >> 1) as isize);
            if *_mid > *_lo {
                _hold = *_mid;
                *_mid = *_lo;
                *_lo = _hold;
            }
            if *_hi > *_mid {
                _hold = *_mid;
                *_mid = *_hi;
                *_hi = _hold;
                if *_mid > *_lo {
                    _hold = *_mid;
                    *_mid = *_lo;
                    *_lo = _hold;
                }
            }
            _left_ptr = _lo.offset(1 as isize);
            _right_ptr = _hi.offset(-(1 as isize));
            loop {
                while *_left_ptr > *_mid {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                }
                while *_mid > *_right_ptr {
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                }
                if _left_ptr < _right_ptr {
                    _hold = *_left_ptr;
                    *_left_ptr = *_right_ptr;
                    *_right_ptr = _hold;
                    if _mid == _left_ptr {
                        _mid = _right_ptr;
                    } else if _mid == _right_ptr {
                        _mid = _left_ptr;
                    }
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                } else if _left_ptr == _right_ptr {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                    break;
                }
                if !(_left_ptr <= _right_ptr) {
                    break;
                }
            }
            if _right_ptr.offset_from(_lo) as i64 <= 4 as libc::c_int as i64 {
                if _hi.offset_from(_left_ptr) as i64 <= 4 as libc::c_int as i64 {
                    _top = _top.offset(-1);
                    _top;
                    _lo = (*_top)._lo;
                    _hi = (*_top)._hi;
                } else {
                    _lo = _left_ptr;
                }
            } else if _hi.offset_from(_left_ptr) as i64 <= 4 as libc::c_int as i64 {
                _hi = _right_ptr;
            } else if _right_ptr.offset_from(_lo) as i64 > _hi.offset_from(_left_ptr) as i64 {
                (*_top)._lo = _lo;
                (*_top)._hi = _right_ptr;
                _top = _top.offset(1);
                _top;
                _lo = _left_ptr;
            } else {
                (*_top)._lo = _left_ptr;
                (*_top)._hi = _hi;
                _top = _top.offset(1);
                _top;
                _hi = _right_ptr;
            }
        }
    }
    let _end_ptr: *mut real_t = _base
        .offset(_elems as isize)
        .offset(-(1 as isize));
    let mut _tmp_ptr: *mut real_t = _base;
    let mut _run_ptr: *mut real_t = 0 as *mut real_t;
    let mut _thresh: *mut real_t = 0 as *mut real_t;
    _thresh = _base.offset(4 as libc::c_int as isize);
    if _thresh > _end_ptr {
        _thresh = _end_ptr;
    }
    _run_ptr = _tmp_ptr.offset(1 as isize);
    while _run_ptr <= _thresh {
        if *_run_ptr > *_tmp_ptr {
            _tmp_ptr = _run_ptr;
        }
        _run_ptr = _run_ptr.offset(1);
        _run_ptr;
    }
    if _tmp_ptr != _base {
        _hold = *_tmp_ptr;
        *_tmp_ptr = *_base;
        *_base = _hold;
    }
    _run_ptr = _base.offset(1 as isize);
    loop {
        _run_ptr = _run_ptr.offset(1);
        if !(_run_ptr <= _end_ptr) {
            break;
        }
        _tmp_ptr = _run_ptr.offset(-(1 as isize));
        while *_run_ptr > *_tmp_ptr {
            _tmp_ptr = _tmp_ptr.offset(-1);
            _tmp_ptr;
        }
        _tmp_ptr = _tmp_ptr.offset(1);
        _tmp_ptr;
        if _tmp_ptr != _run_ptr {
            let mut _trav: *mut real_t = _run_ptr.offset(1 as isize);
            loop {
                _trav = _trav.offset(-1);
                if !(_trav >= _run_ptr) {
                    break;
                }
                let mut _hi_0: *mut real_t = 0 as *mut real_t;
                let mut _lo_0: *mut real_t = 0 as *mut real_t;
                _hold = *_trav;
                _lo_0 = _trav;
                _hi_0 = _lo_0;
                loop {
                    _lo_0 = _lo_0.offset(-1);
                    if !(_lo_0 >= _tmp_ptr) {
                        break;
                    }
                    *_hi_0 = *_lo_0;
                    _hi_0 = _lo_0;
                }
                *_hi_0 = _hold;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__ikvsorti(mut n: size_t, mut base: *mut ikv_t) {
    let _base: *mut ikv_t = base;
    let _elems: size_t = n;
    let mut _hold: ikv_t = ikv_t { key: 0, val: 0 };
    if _elems == 0 as libc::c_int as u64 {
        return;
    }
    if _elems > 4 as libc::c_int as u64 {
        let mut _lo: *mut ikv_t = _base;
        let mut _hi: *mut ikv_t = _lo
            .offset(_elems as isize)
            .offset(-(1 as isize));
        let mut _stack: [C2RustUnnamed_5; 64] = [C2RustUnnamed_5 {
            _hi: 0 as *mut ikv_t,
            _lo: 0 as *mut ikv_t,
        }; 64];
        let mut _top: *mut C2RustUnnamed_5 = _stack.as_mut_ptr().offset(1 as isize);
        while _stack.as_mut_ptr() < _top {
            let mut _left_ptr: *mut ikv_t = 0 as *mut ikv_t;
            let mut _right_ptr: *mut ikv_t = 0 as *mut ikv_t;
            let mut _mid: *mut ikv_t =
                _lo.offset((_hi.offset_from(_lo) as i64 >> 1) as isize);
            if (*_mid).key < (*_lo).key {
                _hold = *_mid;
                *_mid = *_lo;
                *_lo = _hold;
            }
            if (*_hi).key < (*_mid).key {
                _hold = *_mid;
                *_mid = *_hi;
                *_hi = _hold;
                if (*_mid).key < (*_lo).key {
                    _hold = *_mid;
                    *_mid = *_lo;
                    *_lo = _hold;
                }
            }
            _left_ptr = _lo.offset(1 as isize);
            _right_ptr = _hi.offset(-(1 as isize));
            loop {
                while (*_left_ptr).key < (*_mid).key {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                }
                while (*_mid).key < (*_right_ptr).key {
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                }
                if _left_ptr < _right_ptr {
                    _hold = *_left_ptr;
                    *_left_ptr = *_right_ptr;
                    *_right_ptr = _hold;
                    if _mid == _left_ptr {
                        _mid = _right_ptr;
                    } else if _mid == _right_ptr {
                        _mid = _left_ptr;
                    }
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                } else if _left_ptr == _right_ptr {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                    break;
                }
                if !(_left_ptr <= _right_ptr) {
                    break;
                }
            }
            if _right_ptr.offset_from(_lo) as i64 <= 4 as libc::c_int as i64 {
                if _hi.offset_from(_left_ptr) as i64 <= 4 as libc::c_int as i64 {
                    _top = _top.offset(-1);
                    _top;
                    _lo = (*_top)._lo;
                    _hi = (*_top)._hi;
                } else {
                    _lo = _left_ptr;
                }
            } else if _hi.offset_from(_left_ptr) as i64 <= 4 as libc::c_int as i64 {
                _hi = _right_ptr;
            } else if _right_ptr.offset_from(_lo) as i64 > _hi.offset_from(_left_ptr) as i64 {
                (*_top)._lo = _lo;
                (*_top)._hi = _right_ptr;
                _top = _top.offset(1);
                _top;
                _lo = _left_ptr;
            } else {
                (*_top)._lo = _left_ptr;
                (*_top)._hi = _hi;
                _top = _top.offset(1);
                _top;
                _hi = _right_ptr;
            }
        }
    }
    let _end_ptr: *mut ikv_t = _base
        .offset(_elems as isize)
        .offset(-(1 as isize));
    let mut _tmp_ptr: *mut ikv_t = _base;
    let mut _run_ptr: *mut ikv_t = 0 as *mut ikv_t;
    let mut _thresh: *mut ikv_t = 0 as *mut ikv_t;
    _thresh = _base.offset(4 as libc::c_int as isize);
    if _thresh > _end_ptr {
        _thresh = _end_ptr;
    }
    _run_ptr = _tmp_ptr.offset(1 as isize);
    while _run_ptr <= _thresh {
        if (*_run_ptr).key < (*_tmp_ptr).key {
            _tmp_ptr = _run_ptr;
        }
        _run_ptr = _run_ptr.offset(1);
        _run_ptr;
    }
    if _tmp_ptr != _base {
        _hold = *_tmp_ptr;
        *_tmp_ptr = *_base;
        *_base = _hold;
    }
    _run_ptr = _base.offset(1 as isize);
    loop {
        _run_ptr = _run_ptr.offset(1);
        if !(_run_ptr <= _end_ptr) {
            break;
        }
        _tmp_ptr = _run_ptr.offset(-(1 as isize));
        while (*_run_ptr).key < (*_tmp_ptr).key {
            _tmp_ptr = _tmp_ptr.offset(-1);
            _tmp_ptr;
        }
        _tmp_ptr = _tmp_ptr.offset(1);
        _tmp_ptr;
        if _tmp_ptr != _run_ptr {
            let mut _trav: *mut ikv_t = _run_ptr.offset(1 as isize);
            loop {
                _trav = _trav.offset(-1);
                if !(_trav >= _run_ptr) {
                    break;
                }
                let mut _hi_0: *mut ikv_t = 0 as *mut ikv_t;
                let mut _lo_0: *mut ikv_t = 0 as *mut ikv_t;
                _hold = *_trav;
                _lo_0 = _trav;
                _hi_0 = _lo_0;
                loop {
                    _lo_0 = _lo_0.offset(-1);
                    if !(_lo_0 >= _tmp_ptr) {
                        break;
                    }
                    *_hi_0 = *_lo_0;
                    _hi_0 = _lo_0;
                }
                *_hi_0 = _hold;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__ikvsortii(mut n: size_t, mut base: *mut ikv_t) {
    let _base: *mut ikv_t = base;
    let _elems: size_t = n;
    let mut _hold: ikv_t = ikv_t { key: 0, val: 0 };
    if _elems == 0 as libc::c_int as u64 {
        return;
    }
    if _elems > 4 as libc::c_int as u64 {
        let mut _lo: *mut ikv_t = _base;
        let mut _hi: *mut ikv_t = _lo
            .offset(_elems as isize)
            .offset(-(1 as isize));
        let mut _stack: [C2RustUnnamed_6; 64] = [C2RustUnnamed_6 {
            _hi: 0 as *mut ikv_t,
            _lo: 0 as *mut ikv_t,
        }; 64];
        let mut _top: *mut C2RustUnnamed_6 = _stack.as_mut_ptr().offset(1 as isize);
        while _stack.as_mut_ptr() < _top {
            let mut _left_ptr: *mut ikv_t = 0 as *mut ikv_t;
            let mut _right_ptr: *mut ikv_t = 0 as *mut ikv_t;
            let mut _mid: *mut ikv_t =
                _lo.offset((_hi.offset_from(_lo) as i64 >> 1) as isize);
            if (*_mid).key < (*_lo).key || (*_mid).key == (*_lo).key && (*_mid).val < (*_lo).val {
                _hold = *_mid;
                *_mid = *_lo;
                *_lo = _hold;
            }
            if (*_hi).key < (*_mid).key || (*_hi).key == (*_mid).key && (*_hi).val < (*_mid).val {
                _hold = *_mid;
                *_mid = *_hi;
                *_hi = _hold;
                if (*_mid).key < (*_lo).key || (*_mid).key == (*_lo).key && (*_mid).val < (*_lo).val
                {
                    _hold = *_mid;
                    *_mid = *_lo;
                    *_lo = _hold;
                }
            }
            _left_ptr = _lo.offset(1 as isize);
            _right_ptr = _hi.offset(-(1 as isize));
            loop {
                while (*_left_ptr).key < (*_mid).key
                    || (*_left_ptr).key == (*_mid).key && (*_left_ptr).val < (*_mid).val
                {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                }
                while (*_mid).key < (*_right_ptr).key
                    || (*_mid).key == (*_right_ptr).key && (*_mid).val < (*_right_ptr).val
                {
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                }
                if _left_ptr < _right_ptr {
                    _hold = *_left_ptr;
                    *_left_ptr = *_right_ptr;
                    *_right_ptr = _hold;
                    if _mid == _left_ptr {
                        _mid = _right_ptr;
                    } else if _mid == _right_ptr {
                        _mid = _left_ptr;
                    }
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                } else if _left_ptr == _right_ptr {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                    break;
                }
                if !(_left_ptr <= _right_ptr) {
                    break;
                }
            }
            if _right_ptr.offset_from(_lo) as i64 <= 4 as libc::c_int as i64 {
                if _hi.offset_from(_left_ptr) as i64 <= 4 as libc::c_int as i64 {
                    _top = _top.offset(-1);
                    _top;
                    _lo = (*_top)._lo;
                    _hi = (*_top)._hi;
                } else {
                    _lo = _left_ptr;
                }
            } else if _hi.offset_from(_left_ptr) as i64 <= 4 as libc::c_int as i64 {
                _hi = _right_ptr;
            } else if _right_ptr.offset_from(_lo) as i64 > _hi.offset_from(_left_ptr) as i64 {
                (*_top)._lo = _lo;
                (*_top)._hi = _right_ptr;
                _top = _top.offset(1);
                _top;
                _lo = _left_ptr;
            } else {
                (*_top)._lo = _left_ptr;
                (*_top)._hi = _hi;
                _top = _top.offset(1);
                _top;
                _hi = _right_ptr;
            }
        }
    }
    let _end_ptr: *mut ikv_t = _base
        .offset(_elems as isize)
        .offset(-(1 as isize));
    let mut _tmp_ptr: *mut ikv_t = _base;
    let mut _run_ptr: *mut ikv_t = 0 as *mut ikv_t;
    let mut _thresh: *mut ikv_t = 0 as *mut ikv_t;
    _thresh = _base.offset(4 as libc::c_int as isize);
    if _thresh > _end_ptr {
        _thresh = _end_ptr;
    }
    _run_ptr = _tmp_ptr.offset(1 as isize);
    while _run_ptr <= _thresh {
        if (*_run_ptr).key < (*_tmp_ptr).key
            || (*_run_ptr).key == (*_tmp_ptr).key && (*_run_ptr).val < (*_tmp_ptr).val
        {
            _tmp_ptr = _run_ptr;
        }
        _run_ptr = _run_ptr.offset(1);
        _run_ptr;
    }
    if _tmp_ptr != _base {
        _hold = *_tmp_ptr;
        *_tmp_ptr = *_base;
        *_base = _hold;
    }
    _run_ptr = _base.offset(1 as isize);
    loop {
        _run_ptr = _run_ptr.offset(1);
        if !(_run_ptr <= _end_ptr) {
            break;
        }
        _tmp_ptr = _run_ptr.offset(-(1 as isize));
        while (*_run_ptr).key < (*_tmp_ptr).key
            || (*_run_ptr).key == (*_tmp_ptr).key && (*_run_ptr).val < (*_tmp_ptr).val
        {
            _tmp_ptr = _tmp_ptr.offset(-1);
            _tmp_ptr;
        }
        _tmp_ptr = _tmp_ptr.offset(1);
        _tmp_ptr;
        if _tmp_ptr != _run_ptr {
            let mut _trav: *mut ikv_t = _run_ptr.offset(1 as isize);
            loop {
                _trav = _trav.offset(-1);
                if !(_trav >= _run_ptr) {
                    break;
                }
                let mut _hi_0: *mut ikv_t = 0 as *mut ikv_t;
                let mut _lo_0: *mut ikv_t = 0 as *mut ikv_t;
                _hold = *_trav;
                _lo_0 = _trav;
                _hi_0 = _lo_0;
                loop {
                    _lo_0 = _lo_0.offset(-1);
                    if !(_lo_0 >= _tmp_ptr) {
                        break;
                    }
                    *_hi_0 = *_lo_0;
                    _hi_0 = _lo_0;
                }
                *_hi_0 = _hold;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__ikvsortd(mut n: size_t, mut base: *mut ikv_t) {
    let _base: *mut ikv_t = base;
    let _elems: size_t = n;
    let mut _hold: ikv_t = ikv_t { key: 0, val: 0 };
    if _elems == 0 as libc::c_int as u64 {
        return;
    }
    if _elems > 4 as libc::c_int as u64 {
        let mut _lo: *mut ikv_t = _base;
        let mut _hi: *mut ikv_t = _lo
            .offset(_elems as isize)
            .offset(-(1 as isize));
        let mut _stack: [C2RustUnnamed; 64] = [C2RustUnnamed {
            _hi: 0 as *mut ikv_t,
            _lo: 0 as *mut ikv_t,
        }; 64];
        let mut _top: *mut C2RustUnnamed = _stack.as_mut_ptr().offset(1 as isize);
        while _stack.as_mut_ptr() < _top {
            let mut _left_ptr: *mut ikv_t = 0 as *mut ikv_t;
            let mut _right_ptr: *mut ikv_t = 0 as *mut ikv_t;
            let mut _mid: *mut ikv_t =
                _lo.offset((_hi.offset_from(_lo) as i64 >> 1) as isize);
            if (*_mid).key > (*_lo).key {
                _hold = *_mid;
                *_mid = *_lo;
                *_lo = _hold;
            }
            if (*_hi).key > (*_mid).key {
                _hold = *_mid;
                *_mid = *_hi;
                *_hi = _hold;
                if (*_mid).key > (*_lo).key {
                    _hold = *_mid;
                    *_mid = *_lo;
                    *_lo = _hold;
                }
            }
            _left_ptr = _lo.offset(1 as isize);
            _right_ptr = _hi.offset(-(1 as isize));
            loop {
                while (*_left_ptr).key > (*_mid).key {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                }
                while (*_mid).key > (*_right_ptr).key {
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                }
                if _left_ptr < _right_ptr {
                    _hold = *_left_ptr;
                    *_left_ptr = *_right_ptr;
                    *_right_ptr = _hold;
                    if _mid == _left_ptr {
                        _mid = _right_ptr;
                    } else if _mid == _right_ptr {
                        _mid = _left_ptr;
                    }
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                } else if _left_ptr == _right_ptr {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                    break;
                }
                if !(_left_ptr <= _right_ptr) {
                    break;
                }
            }
            if _right_ptr.offset_from(_lo) as i64 <= 4 as libc::c_int as i64 {
                if _hi.offset_from(_left_ptr) as i64 <= 4 as libc::c_int as i64 {
                    _top = _top.offset(-1);
                    _top;
                    _lo = (*_top)._lo;
                    _hi = (*_top)._hi;
                } else {
                    _lo = _left_ptr;
                }
            } else if _hi.offset_from(_left_ptr) as i64 <= 4 as libc::c_int as i64 {
                _hi = _right_ptr;
            } else if _right_ptr.offset_from(_lo) as i64 > _hi.offset_from(_left_ptr) as i64 {
                (*_top)._lo = _lo;
                (*_top)._hi = _right_ptr;
                _top = _top.offset(1);
                _top;
                _lo = _left_ptr;
            } else {
                (*_top)._lo = _left_ptr;
                (*_top)._hi = _hi;
                _top = _top.offset(1);
                _top;
                _hi = _right_ptr;
            }
        }
    }
    let _end_ptr: *mut ikv_t = _base
        .offset(_elems as isize)
        .offset(-(1 as isize));
    let mut _tmp_ptr: *mut ikv_t = _base;
    let mut _run_ptr: *mut ikv_t = 0 as *mut ikv_t;
    let mut _thresh: *mut ikv_t = 0 as *mut ikv_t;
    _thresh = _base.offset(4 as libc::c_int as isize);
    if _thresh > _end_ptr {
        _thresh = _end_ptr;
    }
    _run_ptr = _tmp_ptr.offset(1 as isize);
    while _run_ptr <= _thresh {
        if (*_run_ptr).key > (*_tmp_ptr).key {
            _tmp_ptr = _run_ptr;
        }
        _run_ptr = _run_ptr.offset(1);
        _run_ptr;
    }
    if _tmp_ptr != _base {
        _hold = *_tmp_ptr;
        *_tmp_ptr = *_base;
        *_base = _hold;
    }
    _run_ptr = _base.offset(1 as isize);
    loop {
        _run_ptr = _run_ptr.offset(1);
        if !(_run_ptr <= _end_ptr) {
            break;
        }
        _tmp_ptr = _run_ptr.offset(-(1 as isize));
        while (*_run_ptr).key > (*_tmp_ptr).key {
            _tmp_ptr = _tmp_ptr.offset(-1);
            _tmp_ptr;
        }
        _tmp_ptr = _tmp_ptr.offset(1);
        _tmp_ptr;
        if _tmp_ptr != _run_ptr {
            let mut _trav: *mut ikv_t = _run_ptr.offset(1 as isize);
            loop {
                _trav = _trav.offset(-1);
                if !(_trav >= _run_ptr) {
                    break;
                }
                let mut _hi_0: *mut ikv_t = 0 as *mut ikv_t;
                let mut _lo_0: *mut ikv_t = 0 as *mut ikv_t;
                _hold = *_trav;
                _lo_0 = _trav;
                _hi_0 = _lo_0;
                loop {
                    _lo_0 = _lo_0.offset(-1);
                    if !(_lo_0 >= _tmp_ptr) {
                        break;
                    }
                    *_hi_0 = *_lo_0;
                    _hi_0 = _lo_0;
                }
                *_hi_0 = _hold;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__rkvsorti(mut n: size_t, mut base: *mut rkv_t) {
    let _base: *mut rkv_t = base;
    let _elems: size_t = n;
    let mut _hold: rkv_t = rkv_t { key: 0., val: 0 };
    if _elems == 0 as libc::c_int as u64 {
        return;
    }
    if _elems > 4 as libc::c_int as u64 {
        let mut _lo: *mut rkv_t = _base;
        let mut _hi: *mut rkv_t = _lo
            .offset(_elems as isize)
            .offset(-(1 as isize));
        let mut _stack: [C2RustUnnamed_7; 64] = [C2RustUnnamed_7 {
            _hi: 0 as *mut rkv_t,
            _lo: 0 as *mut rkv_t,
        }; 64];
        let mut _top: *mut C2RustUnnamed_7 = _stack.as_mut_ptr().offset(1 as isize);
        while _stack.as_mut_ptr() < _top {
            let mut _left_ptr: *mut rkv_t = 0 as *mut rkv_t;
            let mut _right_ptr: *mut rkv_t = 0 as *mut rkv_t;
            let mut _mid: *mut rkv_t =
                _lo.offset((_hi.offset_from(_lo) as i64 >> 1) as isize);
            if (*_mid).key < (*_lo).key {
                _hold = *_mid;
                *_mid = *_lo;
                *_lo = _hold;
            }
            if (*_hi).key < (*_mid).key {
                _hold = *_mid;
                *_mid = *_hi;
                *_hi = _hold;
                if (*_mid).key < (*_lo).key {
                    _hold = *_mid;
                    *_mid = *_lo;
                    *_lo = _hold;
                }
            }
            _left_ptr = _lo.offset(1 as isize);
            _right_ptr = _hi.offset(-(1 as isize));
            loop {
                while (*_left_ptr).key < (*_mid).key {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                }
                while (*_mid).key < (*_right_ptr).key {
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                }
                if _left_ptr < _right_ptr {
                    _hold = *_left_ptr;
                    *_left_ptr = *_right_ptr;
                    *_right_ptr = _hold;
                    if _mid == _left_ptr {
                        _mid = _right_ptr;
                    } else if _mid == _right_ptr {
                        _mid = _left_ptr;
                    }
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                } else if _left_ptr == _right_ptr {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                    break;
                }
                if !(_left_ptr <= _right_ptr) {
                    break;
                }
            }
            if _right_ptr.offset_from(_lo) as i64 <= 4 as libc::c_int as i64 {
                if _hi.offset_from(_left_ptr) as i64 <= 4 as libc::c_int as i64 {
                    _top = _top.offset(-1);
                    _top;
                    _lo = (*_top)._lo;
                    _hi = (*_top)._hi;
                } else {
                    _lo = _left_ptr;
                }
            } else if _hi.offset_from(_left_ptr) as i64 <= 4 as libc::c_int as i64 {
                _hi = _right_ptr;
            } else if _right_ptr.offset_from(_lo) as i64 > _hi.offset_from(_left_ptr) as i64 {
                (*_top)._lo = _lo;
                (*_top)._hi = _right_ptr;
                _top = _top.offset(1);
                _top;
                _lo = _left_ptr;
            } else {
                (*_top)._lo = _left_ptr;
                (*_top)._hi = _hi;
                _top = _top.offset(1);
                _top;
                _hi = _right_ptr;
            }
        }
    }
    let _end_ptr: *mut rkv_t = _base
        .offset(_elems as isize)
        .offset(-(1 as isize));
    let mut _tmp_ptr: *mut rkv_t = _base;
    let mut _run_ptr: *mut rkv_t = 0 as *mut rkv_t;
    let mut _thresh: *mut rkv_t = 0 as *mut rkv_t;
    _thresh = _base.offset(4 as libc::c_int as isize);
    if _thresh > _end_ptr {
        _thresh = _end_ptr;
    }
    _run_ptr = _tmp_ptr.offset(1 as isize);
    while _run_ptr <= _thresh {
        if (*_run_ptr).key < (*_tmp_ptr).key {
            _tmp_ptr = _run_ptr;
        }
        _run_ptr = _run_ptr.offset(1);
        _run_ptr;
    }
    if _tmp_ptr != _base {
        _hold = *_tmp_ptr;
        *_tmp_ptr = *_base;
        *_base = _hold;
    }
    _run_ptr = _base.offset(1 as isize);
    loop {
        _run_ptr = _run_ptr.offset(1);
        if !(_run_ptr <= _end_ptr) {
            break;
        }
        _tmp_ptr = _run_ptr.offset(-(1 as isize));
        while (*_run_ptr).key < (*_tmp_ptr).key {
            _tmp_ptr = _tmp_ptr.offset(-1);
            _tmp_ptr;
        }
        _tmp_ptr = _tmp_ptr.offset(1);
        _tmp_ptr;
        if _tmp_ptr != _run_ptr {
            let mut _trav: *mut rkv_t = _run_ptr.offset(1 as isize);
            loop {
                _trav = _trav.offset(-1);
                if !(_trav >= _run_ptr) {
                    break;
                }
                let mut _hi_0: *mut rkv_t = 0 as *mut rkv_t;
                let mut _lo_0: *mut rkv_t = 0 as *mut rkv_t;
                _hold = *_trav;
                _lo_0 = _trav;
                _hi_0 = _lo_0;
                loop {
                    _lo_0 = _lo_0.offset(-1);
                    if !(_lo_0 >= _tmp_ptr) {
                        break;
                    }
                    *_hi_0 = *_lo_0;
                    _hi_0 = _lo_0;
                }
                *_hi_0 = _hold;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__rkvsortd(mut n: size_t, mut base: *mut rkv_t) {
    let _base: *mut rkv_t = base;
    let _elems: size_t = n;
    let mut _hold: rkv_t = rkv_t { key: 0., val: 0 };
    if _elems == 0 as libc::c_int as u64 {
        return;
    }
    if _elems > 4 as libc::c_int as u64 {
        let mut _lo: *mut rkv_t = _base;
        let mut _hi: *mut rkv_t = _lo
            .offset(_elems as isize)
            .offset(-(1 as isize));
        let mut _stack: [C2RustUnnamed_0; 64] = [C2RustUnnamed_0 {
            _hi: 0 as *mut rkv_t,
            _lo: 0 as *mut rkv_t,
        }; 64];
        let mut _top: *mut C2RustUnnamed_0 = _stack.as_mut_ptr().offset(1 as isize);
        while _stack.as_mut_ptr() < _top {
            let mut _left_ptr: *mut rkv_t = 0 as *mut rkv_t;
            let mut _right_ptr: *mut rkv_t = 0 as *mut rkv_t;
            let mut _mid: *mut rkv_t =
                _lo.offset((_hi.offset_from(_lo) as i64 >> 1) as isize);
            if (*_mid).key > (*_lo).key {
                _hold = *_mid;
                *_mid = *_lo;
                *_lo = _hold;
            }
            if (*_hi).key > (*_mid).key {
                _hold = *_mid;
                *_mid = *_hi;
                *_hi = _hold;
                if (*_mid).key > (*_lo).key {
                    _hold = *_mid;
                    *_mid = *_lo;
                    *_lo = _hold;
                }
            }
            _left_ptr = _lo.offset(1 as isize);
            _right_ptr = _hi.offset(-(1 as isize));
            loop {
                while (*_left_ptr).key > (*_mid).key {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                }
                while (*_mid).key > (*_right_ptr).key {
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                }
                if _left_ptr < _right_ptr {
                    _hold = *_left_ptr;
                    *_left_ptr = *_right_ptr;
                    *_right_ptr = _hold;
                    if _mid == _left_ptr {
                        _mid = _right_ptr;
                    } else if _mid == _right_ptr {
                        _mid = _left_ptr;
                    }
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                } else if _left_ptr == _right_ptr {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                    break;
                }
                if !(_left_ptr <= _right_ptr) {
                    break;
                }
            }
            if _right_ptr.offset_from(_lo) as i64 <= 4 as libc::c_int as i64 {
                if _hi.offset_from(_left_ptr) as i64 <= 4 as libc::c_int as i64 {
                    _top = _top.offset(-1);
                    _top;
                    _lo = (*_top)._lo;
                    _hi = (*_top)._hi;
                } else {
                    _lo = _left_ptr;
                }
            } else if _hi.offset_from(_left_ptr) as i64 <= 4 as libc::c_int as i64 {
                _hi = _right_ptr;
            } else if _right_ptr.offset_from(_lo) as i64 > _hi.offset_from(_left_ptr) as i64 {
                (*_top)._lo = _lo;
                (*_top)._hi = _right_ptr;
                _top = _top.offset(1);
                _top;
                _lo = _left_ptr;
            } else {
                (*_top)._lo = _left_ptr;
                (*_top)._hi = _hi;
                _top = _top.offset(1);
                _top;
                _hi = _right_ptr;
            }
        }
    }
    let _end_ptr: *mut rkv_t = _base
        .offset(_elems as isize)
        .offset(-(1 as isize));
    let mut _tmp_ptr: *mut rkv_t = _base;
    let mut _run_ptr: *mut rkv_t = 0 as *mut rkv_t;
    let mut _thresh: *mut rkv_t = 0 as *mut rkv_t;
    _thresh = _base.offset(4 as libc::c_int as isize);
    if _thresh > _end_ptr {
        _thresh = _end_ptr;
    }
    _run_ptr = _tmp_ptr.offset(1 as isize);
    while _run_ptr <= _thresh {
        if (*_run_ptr).key > (*_tmp_ptr).key {
            _tmp_ptr = _run_ptr;
        }
        _run_ptr = _run_ptr.offset(1);
        _run_ptr;
    }
    if _tmp_ptr != _base {
        _hold = *_tmp_ptr;
        *_tmp_ptr = *_base;
        *_base = _hold;
    }
    _run_ptr = _base.offset(1 as isize);
    loop {
        _run_ptr = _run_ptr.offset(1);
        if !(_run_ptr <= _end_ptr) {
            break;
        }
        _tmp_ptr = _run_ptr.offset(-(1 as isize));
        while (*_run_ptr).key > (*_tmp_ptr).key {
            _tmp_ptr = _tmp_ptr.offset(-1);
            _tmp_ptr;
        }
        _tmp_ptr = _tmp_ptr.offset(1);
        _tmp_ptr;
        if _tmp_ptr != _run_ptr {
            let mut _trav: *mut rkv_t = _run_ptr.offset(1 as isize);
            loop {
                _trav = _trav.offset(-1);
                if !(_trav >= _run_ptr) {
                    break;
                }
                let mut _hi_0: *mut rkv_t = 0 as *mut rkv_t;
                let mut _lo_0: *mut rkv_t = 0 as *mut rkv_t;
                _hold = *_trav;
                _lo_0 = _trav;
                _hi_0 = _lo_0;
                loop {
                    _lo_0 = _lo_0.offset(-1);
                    if !(_lo_0 >= _tmp_ptr) {
                        break;
                    }
                    *_hi_0 = *_lo_0;
                    _hi_0 = _lo_0;
                }
                *_hi_0 = _hold;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__uvwsorti(mut n: size_t, mut base: *mut uvw_t) {
    let _base: *mut uvw_t = base;
    let _elems: size_t = n;
    let mut _hold: uvw_t = uvw_t { u: 0, v: 0, w: 0 };
    if _elems == 0 as libc::c_int as u64 {
        return;
    }
    if _elems > 4 as libc::c_int as u64 {
        let mut _lo: *mut uvw_t = _base;
        let mut _hi: *mut uvw_t = _lo
            .offset(_elems as isize)
            .offset(-(1 as isize));
        let mut _stack: [C2RustUnnamed_8; 64] = [C2RustUnnamed_8 {
            _hi: 0 as *mut uvw_t,
            _lo: 0 as *mut uvw_t,
        }; 64];
        let mut _top: *mut C2RustUnnamed_8 = _stack.as_mut_ptr().offset(1 as isize);
        while _stack.as_mut_ptr() < _top {
            let mut _left_ptr: *mut uvw_t = 0 as *mut uvw_t;
            let mut _right_ptr: *mut uvw_t = 0 as *mut uvw_t;
            let mut _mid: *mut uvw_t =
                _lo.offset((_hi.offset_from(_lo) as i64 >> 1) as isize);
            if (*_mid).u < (*_lo).u || (*_mid).u == (*_lo).u && (*_mid).v < (*_lo).v {
                _hold = *_mid;
                *_mid = *_lo;
                *_lo = _hold;
            }
            if (*_hi).u < (*_mid).u || (*_hi).u == (*_mid).u && (*_hi).v < (*_mid).v {
                _hold = *_mid;
                *_mid = *_hi;
                *_hi = _hold;
                if (*_mid).u < (*_lo).u || (*_mid).u == (*_lo).u && (*_mid).v < (*_lo).v {
                    _hold = *_mid;
                    *_mid = *_lo;
                    *_lo = _hold;
                }
            }
            _left_ptr = _lo.offset(1 as isize);
            _right_ptr = _hi.offset(-(1 as isize));
            loop {
                while (*_left_ptr).u < (*_mid).u
                    || (*_left_ptr).u == (*_mid).u && (*_left_ptr).v < (*_mid).v
                {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                }
                while (*_mid).u < (*_right_ptr).u
                    || (*_mid).u == (*_right_ptr).u && (*_mid).v < (*_right_ptr).v
                {
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                }
                if _left_ptr < _right_ptr {
                    _hold = *_left_ptr;
                    *_left_ptr = *_right_ptr;
                    *_right_ptr = _hold;
                    if _mid == _left_ptr {
                        _mid = _right_ptr;
                    } else if _mid == _right_ptr {
                        _mid = _left_ptr;
                    }
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                } else if _left_ptr == _right_ptr {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                    break;
                }
                if !(_left_ptr <= _right_ptr) {
                    break;
                }
            }
            if _right_ptr.offset_from(_lo) as i64 <= 4 as libc::c_int as i64 {
                if _hi.offset_from(_left_ptr) as i64 <= 4 as libc::c_int as i64 {
                    _top = _top.offset(-1);
                    _top;
                    _lo = (*_top)._lo;
                    _hi = (*_top)._hi;
                } else {
                    _lo = _left_ptr;
                }
            } else if _hi.offset_from(_left_ptr) as i64 <= 4 as libc::c_int as i64 {
                _hi = _right_ptr;
            } else if _right_ptr.offset_from(_lo) as i64 > _hi.offset_from(_left_ptr) as i64 {
                (*_top)._lo = _lo;
                (*_top)._hi = _right_ptr;
                _top = _top.offset(1);
                _top;
                _lo = _left_ptr;
            } else {
                (*_top)._lo = _left_ptr;
                (*_top)._hi = _hi;
                _top = _top.offset(1);
                _top;
                _hi = _right_ptr;
            }
        }
    }
    let _end_ptr: *mut uvw_t = _base
        .offset(_elems as isize)
        .offset(-(1 as isize));
    let mut _tmp_ptr: *mut uvw_t = _base;
    let mut _run_ptr: *mut uvw_t = 0 as *mut uvw_t;
    let mut _thresh: *mut uvw_t = 0 as *mut uvw_t;
    _thresh = _base.offset(4 as libc::c_int as isize);
    if _thresh > _end_ptr {
        _thresh = _end_ptr;
    }
    _run_ptr = _tmp_ptr.offset(1 as isize);
    while _run_ptr <= _thresh {
        if (*_run_ptr).u < (*_tmp_ptr).u
            || (*_run_ptr).u == (*_tmp_ptr).u && (*_run_ptr).v < (*_tmp_ptr).v
        {
            _tmp_ptr = _run_ptr;
        }
        _run_ptr = _run_ptr.offset(1);
        _run_ptr;
    }
    if _tmp_ptr != _base {
        _hold = *_tmp_ptr;
        *_tmp_ptr = *_base;
        *_base = _hold;
    }
    _run_ptr = _base.offset(1 as isize);
    loop {
        _run_ptr = _run_ptr.offset(1);
        if !(_run_ptr <= _end_ptr) {
            break;
        }
        _tmp_ptr = _run_ptr.offset(-(1 as isize));
        while (*_run_ptr).u < (*_tmp_ptr).u
            || (*_run_ptr).u == (*_tmp_ptr).u && (*_run_ptr).v < (*_tmp_ptr).v
        {
            _tmp_ptr = _tmp_ptr.offset(-1);
            _tmp_ptr;
        }
        _tmp_ptr = _tmp_ptr.offset(1);
        _tmp_ptr;
        if _tmp_ptr != _run_ptr {
            let mut _trav: *mut uvw_t = _run_ptr.offset(1 as isize);
            loop {
                _trav = _trav.offset(-1);
                if !(_trav >= _run_ptr) {
                    break;
                }
                let mut _hi_0: *mut uvw_t = 0 as *mut uvw_t;
                let mut _lo_0: *mut uvw_t = 0 as *mut uvw_t;
                _hold = *_trav;
                _lo_0 = _trav;
                _hi_0 = _lo_0;
                loop {
                    _lo_0 = _lo_0.offset(-1);
                    if !(_lo_0 >= _tmp_ptr) {
                        break;
                    }
                    *_hi_0 = *_lo_0;
                    _hi_0 = _lo_0;
                }
                *_hi_0 = _hold;
            }
        }
    }
}
