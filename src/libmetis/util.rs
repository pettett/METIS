use ::libc;
extern "C" {
    fn libmetis__isrand(seed: idx_t);
}
pub type __int32_t = libc::c_int;
pub type int32_t = __int32_t;
pub type size_t = libc::c_ulong;
pub type idx_t = int32_t;
pub type real_t = libc::c_float;
pub type C2RustUnnamed = libc::c_int;
pub const METIS_ERROR: C2RustUnnamed = -4;
pub const METIS_ERROR_MEMORY: C2RustUnnamed = -3;
pub const METIS_ERROR_INPUT: C2RustUnnamed = -2;
pub const METIS_OK: C2RustUnnamed = 1;
#[no_mangle]
pub unsafe extern "C" fn libmetis__InitRandom(mut seed: idx_t) {
    libmetis__isrand(
        if seed == -(1 as libc::c_int) { 4321 as libc::c_int } else { seed },
    );
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__iargmax_nrm(
    mut n: size_t,
    mut x: *mut idx_t,
    mut y: *mut real_t,
) -> idx_t {
    let mut i: idx_t = 0;
    let mut max: idx_t = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while (i as libc::c_ulong) < n {
        max = if *x.offset(i as isize) as libc::c_float * *y.offset(i as isize)
            > *x.offset(max as isize) as libc::c_float * *y.offset(max as isize)
        {
            i
        } else {
            max
        };
        i += 1;
        i;
    }
    return max;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__iargmax_strd(
    mut n: size_t,
    mut x: *mut idx_t,
    mut incx: idx_t,
) -> idx_t {
    let mut i: size_t = 0;
    let mut max: size_t = 0 as libc::c_int as size_t;
    n = (n as libc::c_ulong).wrapping_mul(incx as libc::c_ulong) as size_t as size_t;
    i = incx as size_t;
    while i < n {
        max = if *x.offset(i as isize) > *x.offset(max as isize) { i } else { max };
        i = (i as libc::c_ulong).wrapping_add(incx as libc::c_ulong) as size_t as size_t;
    }
    return max.wrapping_div(incx as libc::c_ulong) as idx_t;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__rargmax2(mut n: size_t, mut x: *mut real_t) -> idx_t {
    let mut i: size_t = 0;
    let mut max1: size_t = 0;
    let mut max2: size_t = 0;
    if *x.offset(0 as libc::c_int as isize) > *x.offset(1 as libc::c_int as isize) {
        max1 = 0 as libc::c_int as size_t;
        max2 = 1 as libc::c_int as size_t;
    } else {
        max1 = 1 as libc::c_int as size_t;
        max2 = 0 as libc::c_int as size_t;
    }
    i = 2 as libc::c_int as size_t;
    while i < n {
        if *x.offset(i as isize) > *x.offset(max1 as isize) {
            max2 = max1;
            max1 = i;
        } else if *x.offset(i as isize) > *x.offset(max2 as isize) {
            max2 = i;
        }
        i = i.wrapping_add(1);
        i;
    }
    return max2 as idx_t;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__iargmax2_nrm(
    mut n: size_t,
    mut x: *mut idx_t,
    mut y: *mut real_t,
) -> idx_t {
    let mut i: size_t = 0;
    let mut max1: size_t = 0;
    let mut max2: size_t = 0;
    if *x.offset(0 as libc::c_int as isize) as libc::c_float
        * *y.offset(0 as libc::c_int as isize)
        > *x.offset(1 as libc::c_int as isize) as libc::c_float
            * *y.offset(1 as libc::c_int as isize)
    {
        max1 = 0 as libc::c_int as size_t;
        max2 = 1 as libc::c_int as size_t;
    } else {
        max1 = 1 as libc::c_int as size_t;
        max2 = 0 as libc::c_int as size_t;
    }
    i = 2 as libc::c_int as size_t;
    while i < n {
        if *x.offset(i as isize) as libc::c_float * *y.offset(i as isize)
            > *x.offset(max1 as isize) as libc::c_float * *y.offset(max1 as isize)
        {
            max2 = max1;
            max1 = i;
        } else if *x.offset(i as isize) as libc::c_float * *y.offset(i as isize)
            > *x.offset(max2 as isize) as libc::c_float * *y.offset(max2 as isize)
        {
            max2 = i;
        }
        i = i.wrapping_add(1);
        i;
    }
    return max2 as idx_t;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__metis_rcode(mut sigrval: libc::c_int) -> libc::c_int {
    match sigrval {
        0 => return METIS_OK as libc::c_int,
        6 => return METIS_ERROR_MEMORY as libc::c_int,
        _ => return METIS_ERROR as libc::c_int,
    };
}
