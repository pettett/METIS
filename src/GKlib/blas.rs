use ::libc;
extern "C" {
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn gk_free(ptr1: *mut *mut libc::c_void, _: ...);
    fn gk_ckvsortd(_: size_t, _: *mut gk_ckv_t);
    fn gk_ckvmalloc(n: size_t, msg: *mut libc::c_char) -> *mut gk_ckv_t;
    fn gk_ikvsortd(_: size_t, _: *mut gk_ikv_t);
    fn gk_ikvmalloc(n: size_t, msg: *mut libc::c_char) -> *mut gk_ikv_t;
    fn gk_i32kvsortd(_: size_t, _: *mut gk_i32kv_t);
    fn gk_i32kvmalloc(n: size_t, msg: *mut libc::c_char) -> *mut gk_i32kv_t;
    fn gk_i64kvsortd(_: size_t, _: *mut gk_i64kv_t);
    fn gk_i64kvmalloc(n: size_t, msg: *mut libc::c_char) -> *mut gk_i64kv_t;
    fn gk_zkvsortd(_: size_t, _: *mut gk_zkv_t);
    fn gk_zkvmalloc(n: size_t, msg: *mut libc::c_char) -> *mut gk_zkv_t;
    fn gk_fkvsortd(_: size_t, _: *mut gk_fkv_t);
    fn gk_fkvmalloc(n: size_t, msg: *mut libc::c_char) -> *mut gk_fkv_t;
    fn gk_dkvsortd(_: size_t, _: *mut gk_dkv_t);
    fn gk_dkvmalloc(n: size_t, msg: *mut libc::c_char) -> *mut gk_dkv_t;
    fn gk_idxkvsortd(_: size_t, _: *mut gk_idxkv_t);
    fn gk_idxkvmalloc(n: size_t, msg: *mut libc::c_char) -> *mut gk_idxkv_t;
}
pub type __int32_t = libc::c_int;
pub type __int64_t = i64;
pub type __ssize_t = i64;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type ssize_t = __ssize_t;
pub type size_t = u64;
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
pub struct gk_idxkv_t {
    pub key: gk_idx_t,
    pub val: gk_idx_t,
}
#[no_mangle]
pub unsafe extern "C" fn gk_cargmax(mut n: size_t, mut x: *mut libc::c_char) -> size_t {
    let mut i: size_t = 0;
    let mut max: size_t = 0 as libc::c_int as size_t;
    i = 1 as size_t;
    while i < n {
        max = if *x.offset(i as isize) as libc::c_int > *x.offset(max as isize) as libc::c_int {
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
pub unsafe extern "C" fn gk_cargmin(mut n: size_t, mut x: *mut libc::c_char) -> size_t {
    let mut i: size_t = 0;
    let mut min: size_t = 0 as libc::c_int as size_t;
    i = 1 as size_t;
    while i < n {
        min = if (*x.offset(i as isize) as libc::c_int) < *x.offset(min as isize) as libc::c_int {
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
pub unsafe extern "C" fn gk_cargmax_n(
    mut n: size_t,
    mut x: *mut libc::c_char,
    mut k: size_t,
) -> size_t {
    let mut i: size_t = 0;
    let mut max_n: size_t = 0;
    let mut cand: *mut gk_ckv_t = 0 as *mut gk_ckv_t;
    cand = gk_ckvmalloc(
        n,
        b"GK_ARGMAX_N: cand\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as libc::c_int as size_t;
    while i < n {
        (*cand.offset(i as isize)).val = i as ssize_t;
        (*cand.offset(i as isize)).key = *x.offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    gk_ckvsortd(n, cand);
    max_n = (*cand.offset(k.wrapping_sub(1 as u64) as isize)).val as size_t;
    gk_free(
        &mut cand as *mut *mut gk_ckv_t as *mut libc::c_void as *mut *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
    return max_n;
}
#[no_mangle]
pub unsafe extern "C" fn gk_cincset(
    mut n: size_t,
    mut baseval: libc::c_char,
    mut x: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *x.offset(i as isize) = (baseval as u64).wrapping_add(i) as libc::c_char;
        i = i.wrapping_add(1);
        i;
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn gk_cmax(mut n: size_t, mut x: *mut libc::c_char) -> libc::c_char {
    let mut i: size_t = 0;
    let mut max: size_t = 0 as libc::c_int as size_t;
    if n <= 0 as libc::c_int as u64 {
        return 0 as libc::c_int as libc::c_char;
    }
    i = 1 as size_t;
    while i < n {
        max = if *x.offset(i as isize) as libc::c_int > *x.offset(max as isize) as libc::c_int {
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
pub unsafe extern "C" fn gk_cmin(mut n: size_t, mut x: *mut libc::c_char) -> libc::c_char {
    let mut i: size_t = 0;
    let mut min: size_t = 0 as libc::c_int as size_t;
    if n <= 0 as libc::c_int as u64 {
        return 0 as libc::c_int as libc::c_char;
    }
    i = 1 as size_t;
    while i < n {
        min = if (*x.offset(i as isize) as libc::c_int) < *x.offset(min as isize) as libc::c_int {
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
pub unsafe extern "C" fn gk_cscale(
    mut n: size_t,
    mut alpha: libc::c_char,
    mut x: *mut libc::c_char,
    mut incx: size_t,
) -> *mut libc::c_char {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *x = (*x as libc::c_int * alpha as libc::c_int) as libc::c_char;
        i = i.wrapping_add(1);
        i;
        x = x.offset(incx as isize);
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn gk_caxpy(
    mut n: size_t,
    mut alpha: libc::c_char,
    mut x: *mut libc::c_char,
    mut incx: size_t,
    mut y: *mut libc::c_char,
    mut incy: size_t,
) -> *mut libc::c_char {
    let mut i: size_t = 0;
    let mut y_in: *mut libc::c_char = y;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *y = (*y as libc::c_int + alpha as libc::c_int * *x as libc::c_int) as libc::c_char;
        i = i.wrapping_add(1);
        i;
        x = x.offset(incx as isize);
        y = y.offset(incy as isize);
    }
    return y_in;
}
#[no_mangle]
pub unsafe extern "C" fn gk_csum(
    mut n: size_t,
    mut x: *mut libc::c_char,
    mut incx: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut sum: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int as size_t;
    while i < n {
        sum += *x as libc::c_int;
        i = i.wrapping_add(1);
        i;
        x = x.offset(incx as isize);
    }
    return sum;
}
#[no_mangle]
pub unsafe extern "C" fn gk_cnorm2(
    mut n: size_t,
    mut x: *mut libc::c_char,
    mut incx: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut partial: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int as size_t;
    while i < n {
        partial += *x as libc::c_int * *x as libc::c_int;
        i = i.wrapping_add(1);
        i;
        x = x.offset(incx as isize);
    }
    return if partial > 0 as libc::c_int {
        sqrt(partial as libc::c_double) as libc::c_int
    } else {
        0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_cdot(
    mut n: size_t,
    mut x: *mut libc::c_char,
    mut incx: size_t,
    mut y: *mut libc::c_char,
    mut incy: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut partial: libc::c_int = 0.0f64 as libc::c_int;
    i = 0 as libc::c_int as size_t;
    while i < n {
        partial += *x as libc::c_int * *y as libc::c_int;
        i = i.wrapping_add(1);
        i;
        x = x.offset(incx as isize);
        y = y.offset(incy as isize);
    }
    return partial;
}
#[no_mangle]
pub unsafe extern "C" fn gk_iargmax(mut n: size_t, mut x: *mut libc::c_int) -> size_t {
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
pub unsafe extern "C" fn gk_iargmax_n(
    mut n: size_t,
    mut x: *mut libc::c_int,
    mut k: size_t,
) -> size_t {
    let mut i: size_t = 0;
    let mut max_n: size_t = 0;
    let mut cand: *mut gk_ikv_t = 0 as *mut gk_ikv_t;
    cand = gk_ikvmalloc(
        n,
        b"GK_ARGMAX_N: cand\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as libc::c_int as size_t;
    while i < n {
        (*cand.offset(i as isize)).val = i as ssize_t;
        (*cand.offset(i as isize)).key = *x.offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    gk_ikvsortd(n, cand);
    max_n = (*cand.offset(k.wrapping_sub(1 as u64) as isize)).val as size_t;
    gk_free(
        &mut cand as *mut *mut gk_ikv_t as *mut libc::c_void as *mut *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
    return max_n;
}
#[no_mangle]
pub unsafe extern "C" fn gk_iargmin(mut n: size_t, mut x: *mut libc::c_int) -> size_t {
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
pub unsafe extern "C" fn gk_iscale(
    mut n: size_t,
    mut alpha: libc::c_int,
    mut x: *mut libc::c_int,
    mut incx: size_t,
) -> *mut libc::c_int {
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
pub unsafe extern "C" fn gk_iaxpy(
    mut n: size_t,
    mut alpha: libc::c_int,
    mut x: *mut libc::c_int,
    mut incx: size_t,
    mut y: *mut libc::c_int,
    mut incy: size_t,
) -> *mut libc::c_int {
    let mut i: size_t = 0;
    let mut y_in: *mut libc::c_int = y;
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
pub unsafe extern "C" fn gk_imin(mut n: size_t, mut x: *mut libc::c_int) -> libc::c_int {
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
pub unsafe extern "C" fn gk_imax(mut n: size_t, mut x: *mut libc::c_int) -> libc::c_int {
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
pub unsafe extern "C" fn gk_iincset(
    mut n: size_t,
    mut baseval: libc::c_int,
    mut x: *mut libc::c_int,
) -> *mut libc::c_int {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *x.offset(i as isize) = (baseval as u64).wrapping_add(i) as libc::c_int;
        i = i.wrapping_add(1);
        i;
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn gk_isum(
    mut n: size_t,
    mut x: *mut libc::c_int,
    mut incx: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut sum: libc::c_int = 0 as libc::c_int;
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
pub unsafe extern "C" fn gk_inorm2(
    mut n: size_t,
    mut x: *mut libc::c_int,
    mut incx: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut partial: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int as size_t;
    while i < n {
        partial += *x * *x;
        i = i.wrapping_add(1);
        i;
        x = x.offset(incx as isize);
    }
    return if partial > 0 as libc::c_int {
        sqrt(partial as libc::c_double) as libc::c_int
    } else {
        0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_idot(
    mut n: size_t,
    mut x: *mut libc::c_int,
    mut incx: size_t,
    mut y: *mut libc::c_int,
    mut incy: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut partial: libc::c_int = 0.0f64 as libc::c_int;
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
pub unsafe extern "C" fn gk_i32argmin(mut n: size_t, mut x: *mut int32_t) -> size_t {
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
pub unsafe extern "C" fn gk_i32argmax(mut n: size_t, mut x: *mut int32_t) -> size_t {
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
pub unsafe extern "C" fn gk_i32argmax_n(
    mut n: size_t,
    mut x: *mut int32_t,
    mut k: size_t,
) -> size_t {
    let mut i: size_t = 0;
    let mut max_n: size_t = 0;
    let mut cand: *mut gk_i32kv_t = 0 as *mut gk_i32kv_t;
    cand = gk_i32kvmalloc(
        n,
        b"GK_ARGMAX_N: cand\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as libc::c_int as size_t;
    while i < n {
        (*cand.offset(i as isize)).val = i as ssize_t;
        (*cand.offset(i as isize)).key = *x.offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    gk_i32kvsortd(n, cand);
    max_n = (*cand.offset(k.wrapping_sub(1 as u64) as isize)).val as size_t;
    gk_free(
        &mut cand as *mut *mut gk_i32kv_t as *mut libc::c_void as *mut *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
    return max_n;
}
#[no_mangle]
pub unsafe extern "C" fn gk_i32incset(
    mut n: size_t,
    mut baseval: int32_t,
    mut x: *mut int32_t,
) -> *mut int32_t {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *x.offset(i as isize) = (baseval as u64).wrapping_add(i) as int32_t;
        i = i.wrapping_add(1);
        i;
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn gk_i32max(mut n: size_t, mut x: *mut int32_t) -> int32_t {
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
pub unsafe extern "C" fn gk_i32min(mut n: size_t, mut x: *mut int32_t) -> int32_t {
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
pub unsafe extern "C" fn gk_i32scale(
    mut n: size_t,
    mut alpha: int32_t,
    mut x: *mut int32_t,
    mut incx: size_t,
) -> *mut int32_t {
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
pub unsafe extern "C" fn gk_i32axpy(
    mut n: size_t,
    mut alpha: int32_t,
    mut x: *mut int32_t,
    mut incx: size_t,
    mut y: *mut int32_t,
    mut incy: size_t,
) -> *mut int32_t {
    let mut i: size_t = 0;
    let mut y_in: *mut int32_t = y;
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
pub unsafe extern "C" fn gk_i32sum(
    mut n: size_t,
    mut x: *mut int32_t,
    mut incx: size_t,
) -> int32_t {
    let mut i: size_t = 0;
    let mut sum: int32_t = 0 as libc::c_int;
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
pub unsafe extern "C" fn gk_i32norm2(
    mut n: size_t,
    mut x: *mut int32_t,
    mut incx: size_t,
) -> int32_t {
    let mut i: size_t = 0;
    let mut partial: int32_t = 0 as libc::c_int;
    i = 0 as libc::c_int as size_t;
    while i < n {
        partial += *x * *x;
        i = i.wrapping_add(1);
        i;
        x = x.offset(incx as isize);
    }
    return if partial > 0 as libc::c_int {
        sqrt(partial as libc::c_double) as int32_t
    } else {
        0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_i32dot(
    mut n: size_t,
    mut x: *mut int32_t,
    mut incx: size_t,
    mut y: *mut int32_t,
    mut incy: size_t,
) -> int32_t {
    let mut i: size_t = 0;
    let mut partial: int32_t = 0.0f64 as int32_t;
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
pub unsafe extern "C" fn gk_i64argmax(mut n: size_t, mut x: *mut int64_t) -> size_t {
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
pub unsafe extern "C" fn gk_i64argmin(mut n: size_t, mut x: *mut int64_t) -> size_t {
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
pub unsafe extern "C" fn gk_i64argmax_n(
    mut n: size_t,
    mut x: *mut int64_t,
    mut k: size_t,
) -> size_t {
    let mut i: size_t = 0;
    let mut max_n: size_t = 0;
    let mut cand: *mut gk_i64kv_t = 0 as *mut gk_i64kv_t;
    cand = gk_i64kvmalloc(
        n,
        b"GK_ARGMAX_N: cand\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as libc::c_int as size_t;
    while i < n {
        (*cand.offset(i as isize)).val = i as ssize_t;
        (*cand.offset(i as isize)).key = *x.offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    gk_i64kvsortd(n, cand);
    max_n = (*cand.offset(k.wrapping_sub(1 as u64) as isize)).val as size_t;
    gk_free(
        &mut cand as *mut *mut gk_i64kv_t as *mut libc::c_void as *mut *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
    return max_n;
}
#[no_mangle]
pub unsafe extern "C" fn gk_i64axpy(
    mut n: size_t,
    mut alpha: int64_t,
    mut x: *mut int64_t,
    mut incx: size_t,
    mut y: *mut int64_t,
    mut incy: size_t,
) -> *mut int64_t {
    let mut i: size_t = 0;
    let mut y_in: *mut int64_t = y;
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
pub unsafe extern "C" fn gk_i64incset(
    mut n: size_t,
    mut baseval: int64_t,
    mut x: *mut int64_t,
) -> *mut int64_t {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *x.offset(i as isize) = (baseval as u64).wrapping_add(i) as int64_t;
        i = i.wrapping_add(1);
        i;
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn gk_i64max(mut n: size_t, mut x: *mut int64_t) -> int64_t {
    let mut i: size_t = 0;
    let mut max: size_t = 0 as libc::c_int as size_t;
    if n <= 0 as libc::c_int as u64 {
        return 0 as libc::c_int as int64_t;
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
pub unsafe extern "C" fn gk_i64min(mut n: size_t, mut x: *mut int64_t) -> int64_t {
    let mut i: size_t = 0;
    let mut min: size_t = 0 as libc::c_int as size_t;
    if n <= 0 as libc::c_int as u64 {
        return 0 as libc::c_int as int64_t;
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
pub unsafe extern "C" fn gk_i64scale(
    mut n: size_t,
    mut alpha: int64_t,
    mut x: *mut int64_t,
    mut incx: size_t,
) -> *mut int64_t {
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
pub unsafe extern "C" fn gk_i64dot(
    mut n: size_t,
    mut x: *mut int64_t,
    mut incx: size_t,
    mut y: *mut int64_t,
    mut incy: size_t,
) -> int64_t {
    let mut i: size_t = 0;
    let mut partial: int64_t = 0.0f64 as int64_t;
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
pub unsafe extern "C" fn gk_i64norm2(
    mut n: size_t,
    mut x: *mut int64_t,
    mut incx: size_t,
) -> int64_t {
    let mut i: size_t = 0;
    let mut partial: int64_t = 0 as libc::c_int as int64_t;
    i = 0 as libc::c_int as size_t;
    while i < n {
        partial += *x * *x;
        i = i.wrapping_add(1);
        i;
        x = x.offset(incx as isize);
    }
    return if partial > 0 as libc::c_int as i64 {
        sqrt(partial as libc::c_double) as int64_t
    } else {
        0 as libc::c_int as int64_t
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_i64sum(
    mut n: size_t,
    mut x: *mut int64_t,
    mut incx: size_t,
) -> int64_t {
    let mut i: size_t = 0;
    let mut sum: int64_t = 0 as libc::c_int as int64_t;
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
pub unsafe extern "C" fn gk_zargmax(mut n: size_t, mut x: *mut ssize_t) -> size_t {
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
pub unsafe extern "C" fn gk_zargmin(mut n: size_t, mut x: *mut ssize_t) -> size_t {
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
pub unsafe extern "C" fn gk_zargmax_n(mut n: size_t, mut x: *mut ssize_t, mut k: size_t) -> size_t {
    let mut i: size_t = 0;
    let mut max_n: size_t = 0;
    let mut cand: *mut gk_zkv_t = 0 as *mut gk_zkv_t;
    cand = gk_zkvmalloc(
        n,
        b"GK_ARGMAX_N: cand\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as libc::c_int as size_t;
    while i < n {
        (*cand.offset(i as isize)).val = i as ssize_t;
        (*cand.offset(i as isize)).key = *x.offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    gk_zkvsortd(n, cand);
    max_n = (*cand.offset(k.wrapping_sub(1 as u64) as isize)).val as size_t;
    gk_free(
        &mut cand as *mut *mut gk_zkv_t as *mut libc::c_void as *mut *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
    return max_n;
}
#[no_mangle]
pub unsafe extern "C" fn gk_zincset(
    mut n: size_t,
    mut baseval: ssize_t,
    mut x: *mut ssize_t,
) -> *mut ssize_t {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *x.offset(i as isize) = (baseval as u64).wrapping_add(i) as ssize_t;
        i = i.wrapping_add(1);
        i;
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn gk_zmax(mut n: size_t, mut x: *mut ssize_t) -> ssize_t {
    let mut i: size_t = 0;
    let mut max: size_t = 0 as libc::c_int as size_t;
    if n <= 0 as libc::c_int as u64 {
        return 0 as libc::c_int as ssize_t;
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
pub unsafe extern "C" fn gk_zmin(mut n: size_t, mut x: *mut ssize_t) -> ssize_t {
    let mut i: size_t = 0;
    let mut min: size_t = 0 as libc::c_int as size_t;
    if n <= 0 as libc::c_int as u64 {
        return 0 as libc::c_int as ssize_t;
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
pub unsafe extern "C" fn gk_zscale(
    mut n: size_t,
    mut alpha: ssize_t,
    mut x: *mut ssize_t,
    mut incx: size_t,
) -> *mut ssize_t {
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
pub unsafe extern "C" fn gk_zaxpy(
    mut n: size_t,
    mut alpha: ssize_t,
    mut x: *mut ssize_t,
    mut incx: size_t,
    mut y: *mut ssize_t,
    mut incy: size_t,
) -> *mut ssize_t {
    let mut i: size_t = 0;
    let mut y_in: *mut ssize_t = y;
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
pub unsafe extern "C" fn gk_zsum(mut n: size_t, mut x: *mut ssize_t, mut incx: size_t) -> ssize_t {
    let mut i: size_t = 0;
    let mut sum: ssize_t = 0 as libc::c_int as ssize_t;
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
pub unsafe extern "C" fn gk_znorm2(
    mut n: size_t,
    mut x: *mut ssize_t,
    mut incx: size_t,
) -> ssize_t {
    let mut i: size_t = 0;
    let mut partial: ssize_t = 0 as libc::c_int as ssize_t;
    i = 0 as libc::c_int as size_t;
    while i < n {
        partial += *x * *x;
        i = i.wrapping_add(1);
        i;
        x = x.offset(incx as isize);
    }
    return if partial > 0 as libc::c_int as i64 {
        sqrt(partial as libc::c_double) as ssize_t
    } else {
        0 as libc::c_int as ssize_t
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_zdot(
    mut n: size_t,
    mut x: *mut ssize_t,
    mut incx: size_t,
    mut y: *mut ssize_t,
    mut incy: size_t,
) -> ssize_t {
    let mut i: size_t = 0;
    let mut partial: ssize_t = 0.0f64 as ssize_t;
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
pub unsafe extern "C" fn gk_fargmin(mut n: size_t, mut x: *mut libc::c_float) -> size_t {
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
pub unsafe extern "C" fn gk_fargmax_n(
    mut n: size_t,
    mut x: *mut libc::c_float,
    mut k: size_t,
) -> size_t {
    let mut i: size_t = 0;
    let mut max_n: size_t = 0;
    let mut cand: *mut gk_fkv_t = 0 as *mut gk_fkv_t;
    cand = gk_fkvmalloc(
        n,
        b"GK_ARGMAX_N: cand\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as libc::c_int as size_t;
    while i < n {
        (*cand.offset(i as isize)).val = i as ssize_t;
        (*cand.offset(i as isize)).key = *x.offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    gk_fkvsortd(n, cand);
    max_n = (*cand.offset(k.wrapping_sub(1 as u64) as isize)).val as size_t;
    gk_free(
        &mut cand as *mut *mut gk_fkv_t as *mut libc::c_void as *mut *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
    return max_n;
}
#[no_mangle]
pub unsafe extern "C" fn gk_fargmax(mut n: size_t, mut x: *mut libc::c_float) -> size_t {
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
pub unsafe extern "C" fn gk_fscale(
    mut n: size_t,
    mut alpha: libc::c_float,
    mut x: *mut libc::c_float,
    mut incx: size_t,
) -> *mut libc::c_float {
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
pub unsafe extern "C" fn gk_fincset(
    mut n: size_t,
    mut baseval: libc::c_float,
    mut x: *mut libc::c_float,
) -> *mut libc::c_float {
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
pub unsafe extern "C" fn gk_fmax(mut n: size_t, mut x: *mut libc::c_float) -> libc::c_float {
    let mut i: size_t = 0;
    let mut max: size_t = 0 as libc::c_int as size_t;
    if n <= 0 as libc::c_int as u64 {
        return 0 as libc::c_int as libc::c_float;
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
pub unsafe extern "C" fn gk_fmin(mut n: size_t, mut x: *mut libc::c_float) -> libc::c_float {
    let mut i: size_t = 0;
    let mut min: size_t = 0 as libc::c_int as size_t;
    if n <= 0 as libc::c_int as u64 {
        return 0 as libc::c_int as libc::c_float;
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
pub unsafe extern "C" fn gk_faxpy(
    mut n: size_t,
    mut alpha: libc::c_float,
    mut x: *mut libc::c_float,
    mut incx: size_t,
    mut y: *mut libc::c_float,
    mut incy: size_t,
) -> *mut libc::c_float {
    let mut i: size_t = 0;
    let mut y_in: *mut libc::c_float = y;
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
pub unsafe extern "C" fn gk_fdot(
    mut n: size_t,
    mut x: *mut libc::c_float,
    mut incx: size_t,
    mut y: *mut libc::c_float,
    mut incy: size_t,
) -> libc::c_float {
    let mut i: size_t = 0;
    let mut partial: libc::c_float = 0.0f64 as libc::c_float;
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
pub unsafe extern "C" fn gk_fsum(
    mut n: size_t,
    mut x: *mut libc::c_float,
    mut incx: size_t,
) -> libc::c_float {
    let mut i: size_t = 0;
    let mut sum: libc::c_float = 0 as libc::c_int as libc::c_float;
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
pub unsafe extern "C" fn gk_fnorm2(
    mut n: size_t,
    mut x: *mut libc::c_float,
    mut incx: size_t,
) -> libc::c_float {
    let mut i: size_t = 0;
    let mut partial: libc::c_float = 0 as libc::c_int as libc::c_float;
    i = 0 as libc::c_int as size_t;
    while i < n {
        partial += *x * *x;
        i = i.wrapping_add(1);
        i;
        x = x.offset(incx as isize);
    }
    return if partial > 0 as libc::c_int as libc::c_float {
        sqrt(partial as libc::c_double) as libc::c_float
    } else {
        0 as libc::c_int as libc::c_float
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_dargmax_n(
    mut n: size_t,
    mut x: *mut libc::c_double,
    mut k: size_t,
) -> size_t {
    let mut i: size_t = 0;
    let mut max_n: size_t = 0;
    let mut cand: *mut gk_dkv_t = 0 as *mut gk_dkv_t;
    cand = gk_dkvmalloc(
        n,
        b"GK_ARGMAX_N: cand\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as libc::c_int as size_t;
    while i < n {
        (*cand.offset(i as isize)).val = i as ssize_t;
        (*cand.offset(i as isize)).key = *x.offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    gk_dkvsortd(n, cand);
    max_n = (*cand.offset(k.wrapping_sub(1 as u64) as isize)).val as size_t;
    gk_free(
        &mut cand as *mut *mut gk_dkv_t as *mut libc::c_void as *mut *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
    return max_n;
}
#[no_mangle]
pub unsafe extern "C" fn gk_dargmax(mut n: size_t, mut x: *mut libc::c_double) -> size_t {
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
pub unsafe extern "C" fn gk_dargmin(mut n: size_t, mut x: *mut libc::c_double) -> size_t {
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
pub unsafe extern "C" fn gk_dscale(
    mut n: size_t,
    mut alpha: libc::c_double,
    mut x: *mut libc::c_double,
    mut incx: size_t,
) -> *mut libc::c_double {
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
pub unsafe extern "C" fn gk_dincset(
    mut n: size_t,
    mut baseval: libc::c_double,
    mut x: *mut libc::c_double,
) -> *mut libc::c_double {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *x.offset(i as isize) = baseval + i as libc::c_double;
        i = i.wrapping_add(1);
        i;
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn gk_dmax(mut n: size_t, mut x: *mut libc::c_double) -> libc::c_double {
    let mut i: size_t = 0;
    let mut max: size_t = 0 as libc::c_int as size_t;
    if n <= 0 as libc::c_int as u64 {
        return 0 as libc::c_int as libc::c_double;
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
pub unsafe extern "C" fn gk_dmin(mut n: size_t, mut x: *mut libc::c_double) -> libc::c_double {
    let mut i: size_t = 0;
    let mut min: size_t = 0 as libc::c_int as size_t;
    if n <= 0 as libc::c_int as u64 {
        return 0 as libc::c_int as libc::c_double;
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
pub unsafe extern "C" fn gk_daxpy(
    mut n: size_t,
    mut alpha: libc::c_double,
    mut x: *mut libc::c_double,
    mut incx: size_t,
    mut y: *mut libc::c_double,
    mut incy: size_t,
) -> *mut libc::c_double {
    let mut i: size_t = 0;
    let mut y_in: *mut libc::c_double = y;
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
pub unsafe extern "C" fn gk_ddot(
    mut n: size_t,
    mut x: *mut libc::c_double,
    mut incx: size_t,
    mut y: *mut libc::c_double,
    mut incy: size_t,
) -> libc::c_double {
    let mut i: size_t = 0;
    let mut partial: libc::c_double = 0.0f64;
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
pub unsafe extern "C" fn gk_dsum(
    mut n: size_t,
    mut x: *mut libc::c_double,
    mut incx: size_t,
) -> libc::c_double {
    let mut i: size_t = 0;
    let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
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
pub unsafe extern "C" fn gk_dnorm2(
    mut n: size_t,
    mut x: *mut libc::c_double,
    mut incx: size_t,
) -> libc::c_double {
    let mut i: size_t = 0;
    let mut partial: libc::c_double = 0 as libc::c_int as libc::c_double;
    i = 0 as libc::c_int as size_t;
    while i < n {
        partial += *x * *x;
        i = i.wrapping_add(1);
        i;
        x = x.offset(incx as isize);
    }
    return if partial > 0 as libc::c_int as libc::c_double {
        sqrt(partial)
    } else {
        0 as libc::c_int as libc::c_double
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_idxargmax(mut n: size_t, mut x: *mut gk_idx_t) -> size_t {
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
pub unsafe extern "C" fn gk_idxargmin(mut n: size_t, mut x: *mut gk_idx_t) -> size_t {
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
pub unsafe extern "C" fn gk_idxargmax_n(
    mut n: size_t,
    mut x: *mut gk_idx_t,
    mut k: size_t,
) -> size_t {
    let mut i: size_t = 0;
    let mut max_n: size_t = 0;
    let mut cand: *mut gk_idxkv_t = 0 as *mut gk_idxkv_t;
    cand = gk_idxkvmalloc(
        n,
        b"GK_ARGMAX_N: cand\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as libc::c_int as size_t;
    while i < n {
        (*cand.offset(i as isize)).val = i as gk_idx_t;
        (*cand.offset(i as isize)).key = *x.offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    gk_idxkvsortd(n, cand);
    max_n = (*cand.offset(k.wrapping_sub(1 as u64) as isize)).val as size_t;
    gk_free(
        &mut cand as *mut *mut gk_idxkv_t as *mut libc::c_void as *mut *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
    return max_n;
}
#[no_mangle]
pub unsafe extern "C" fn gk_idxaxpy(
    mut n: size_t,
    mut alpha: gk_idx_t,
    mut x: *mut gk_idx_t,
    mut incx: size_t,
    mut y: *mut gk_idx_t,
    mut incy: size_t,
) -> *mut gk_idx_t {
    let mut i: size_t = 0;
    let mut y_in: *mut gk_idx_t = y;
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
pub unsafe extern "C" fn gk_idxincset(
    mut n: size_t,
    mut baseval: gk_idx_t,
    mut x: *mut gk_idx_t,
) -> *mut gk_idx_t {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *x.offset(i as isize) = (baseval as u64).wrapping_add(i) as gk_idx_t;
        i = i.wrapping_add(1);
        i;
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn gk_idxmax(mut n: size_t, mut x: *mut gk_idx_t) -> gk_idx_t {
    let mut i: size_t = 0;
    let mut max: size_t = 0 as libc::c_int as size_t;
    if n <= 0 as libc::c_int as u64 {
        return 0 as libc::c_int as gk_idx_t;
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
pub unsafe extern "C" fn gk_idxmin(mut n: size_t, mut x: *mut gk_idx_t) -> gk_idx_t {
    let mut i: size_t = 0;
    let mut min: size_t = 0 as libc::c_int as size_t;
    if n <= 0 as libc::c_int as u64 {
        return 0 as libc::c_int as gk_idx_t;
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
pub unsafe extern "C" fn gk_idxscale(
    mut n: size_t,
    mut alpha: gk_idx_t,
    mut x: *mut gk_idx_t,
    mut incx: size_t,
) -> *mut gk_idx_t {
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
pub unsafe extern "C" fn gk_idxdot(
    mut n: size_t,
    mut x: *mut gk_idx_t,
    mut incx: size_t,
    mut y: *mut gk_idx_t,
    mut incy: size_t,
) -> gk_idx_t {
    let mut i: size_t = 0;
    let mut partial: gk_idx_t = 0.0f64 as gk_idx_t;
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
pub unsafe extern "C" fn gk_idxsum(
    mut n: size_t,
    mut x: *mut gk_idx_t,
    mut incx: size_t,
) -> gk_idx_t {
    let mut i: size_t = 0;
    let mut sum: gk_idx_t = 0 as libc::c_int as gk_idx_t;
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
pub unsafe extern "C" fn gk_idxnorm2(
    mut n: size_t,
    mut x: *mut gk_idx_t,
    mut incx: size_t,
) -> gk_idx_t {
    let mut i: size_t = 0;
    let mut partial: gk_idx_t = 0 as libc::c_int as gk_idx_t;
    i = 0 as libc::c_int as size_t;
    while i < n {
        partial += *x * *x;
        i = i.wrapping_add(1);
        i;
        x = x.offset(incx as isize);
    }
    return if partial > 0 as libc::c_int as i64 {
        sqrt(partial as libc::c_double) as gk_idx_t
    } else {
        0 as libc::c_int as gk_idx_t
    };
}
