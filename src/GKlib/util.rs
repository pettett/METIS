use ::libc;
extern "C" {
    fn log(_: libc::c_double) -> libc::c_double;
    fn rand() -> libc::c_int;
    fn gk_iset(n: size_t, val: libc::c_int, x: *mut libc::c_int) -> *mut libc::c_int;
}
pub type __ssize_t = i64;
pub type ssize_t = __ssize_t;
pub type size_t = u64;
pub type gk_idx_t = ssize_t;
#[no_mangle]
pub unsafe extern "C" fn gk_RandomPermute(
    mut n: size_t,
    mut p: *mut libc::c_int,
    mut flag: libc::c_int,
) {
    let mut i: gk_idx_t = 0;
    let mut u: gk_idx_t = 0;
    let mut v: gk_idx_t = 0;
    let mut tmp: libc::c_int = 0;
    if flag == 1 {
        i = 0 as libc::c_int as gk_idx_t;
        while (i as u64) < n {
            *p.offset(i as isize) = i as libc::c_int;
            i += 1;
            i;
        }
    }
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < n.wrapping_div(2 as libc::c_int as u64) {
        v = (1.0f64 / (2147483647 as libc::c_int as libc::c_double + 1.0f64)
            * n as libc::c_double
            * rand() as libc::c_double) as libc::c_int as gk_idx_t;
        u = (1.0f64 / (2147483647 as libc::c_int as libc::c_double + 1.0f64)
            * n as libc::c_double
            * rand() as libc::c_double) as libc::c_int as gk_idx_t;
        tmp = *p.offset(v as isize);
        *p.offset(v as isize) = *p.offset(u as isize);
        *p.offset(u as isize) = tmp;
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn gk_array2csr(
    mut n: size_t,
    mut range: size_t,
    mut array: *mut libc::c_int,
    mut ptr: *mut libc::c_int,
    mut ind: *mut libc::c_int,
) {
    let mut i: gk_idx_t = 0;
    gk_iset(range.wrapping_add(1 as u64), 0 as libc::c_int, ptr);
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < n {
        let ref mut fresh0 = *ptr.offset(*array.offset(i as isize) as isize);
        *fresh0 += 1;
        *fresh0;
        i += 1;
        i;
    }
    i = 1 as gk_idx_t;
    while (i as u64) < range {
        *ptr.offset(i as isize) += *ptr.offset((i - 1 as i64) as isize);
        i += 1;
        i;
    }
    i = range as gk_idx_t;
    while i > 0 as libc::c_int as i64 {
        *ptr.offset(i as isize) = *ptr.offset((i - 1 as i64) as isize);
        i -= 1;
        i;
    }
    *ptr.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    i = 0 as libc::c_int as gk_idx_t;
    while (i as u64) < n {
        let ref mut fresh1 = *ptr.offset(*array.offset(i as isize) as isize);
        let fresh2 = *fresh1;
        *fresh1 = *fresh1 + 1;
        *ind.offset(fresh2 as isize) = i as libc::c_int;
        i += 1;
        i;
    }
    i = range as gk_idx_t;
    while i > 0 as libc::c_int as i64 {
        *ptr.offset(i as isize) = *ptr.offset((i - 1 as i64) as isize);
        i -= 1;
        i;
    }
    *ptr.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gk_log2(mut a: libc::c_int) -> libc::c_int {
    let mut i: gk_idx_t = 0;
    i = 1 as gk_idx_t;
    while a > 1 {
        i += 1;
        i;
        a = a >> 1;
    }
    return (i - 1 as i64) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gk_ispow2(mut a: libc::c_int) -> libc::c_int {
    return (a == (1) << gk_log2(a)) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gk_flog2(mut a: libc::c_float) -> libc::c_float {
    return (log(a as libc::c_double) / log(2.0f64)) as libc::c_float;
}
