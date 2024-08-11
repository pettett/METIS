use ::libc;
pub type __ssize_t = i64;
pub type ssize_t = __ssize_t;
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gk_fkv_t {
    pub key: libc::c_float,
    pub val: ssize_t,
}
#[no_mangle]
pub unsafe extern "C" fn gk_dfkvkselect(
    mut n: size_t,
    mut topk: libc::c_int,
    mut cand: *mut gk_fkv_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut lo: libc::c_int = 0;
    let mut hi: libc::c_int = 0;
    let mut mid: libc::c_int = 0;
    let mut stmp: gk_fkv_t = gk_fkv_t { key: 0., val: 0 };
    let mut pivot: libc::c_float = 0.;
    if n <= topk as u64 {
        return n as libc::c_int;
    }
    lo = 0 as libc::c_int;
    hi = n.wrapping_sub(1 as libc::c_int as u64) as libc::c_int;
    while lo < hi {
        mid = lo + (hi - lo >> 1 as libc::c_int);
        if (*cand.offset(lo as isize)).key < (*cand.offset(mid as isize)).key {
            mid = lo;
        }
        if (*cand.offset(hi as isize)).key > (*cand.offset(mid as isize)).key {
            mid = hi;
            if (*cand.offset(lo as isize)).key < (*cand.offset(mid as isize)).key {
                mid = lo;
            }
        }
        stmp = *cand.offset(mid as isize);
        *cand.offset(mid as isize) = *cand.offset(hi as isize);
        *cand.offset(hi as isize) = stmp;
        pivot = (*cand.offset(hi as isize)).key;
        i = lo - 1 as libc::c_int;
        j = lo;
        while j < hi {
            if (*cand.offset(j as isize)).key >= pivot {
                i += 1;
                i;
                stmp = *cand.offset(i as isize);
                *cand.offset(i as isize) = *cand.offset(j as isize);
                *cand.offset(j as isize) = stmp;
            }
            j += 1;
            j;
        }
        i += 1;
        i;
        stmp = *cand.offset(i as isize);
        *cand.offset(i as isize) = *cand.offset(hi as isize);
        *cand.offset(hi as isize) = stmp;
        if i > topk {
            hi = i - 1 as libc::c_int;
        } else {
            if !(i < topk) {
                break;
            }
            lo = i + 1 as libc::c_int;
        }
    }
    return topk;
}
#[no_mangle]
pub unsafe extern "C" fn gk_ifkvkselect(
    mut n: size_t,
    mut topk: libc::c_int,
    mut cand: *mut gk_fkv_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut lo: libc::c_int = 0;
    let mut hi: libc::c_int = 0;
    let mut mid: libc::c_int = 0;
    let mut stmp: gk_fkv_t = gk_fkv_t { key: 0., val: 0 };
    let mut pivot: libc::c_float = 0.;
    if n <= topk as u64 {
        return n as libc::c_int;
    }
    lo = 0 as libc::c_int;
    hi = n.wrapping_sub(1 as libc::c_int as u64) as libc::c_int;
    while lo < hi {
        mid = lo + (hi - lo >> 1 as libc::c_int);
        if (*cand.offset(lo as isize)).key > (*cand.offset(mid as isize)).key {
            mid = lo;
        }
        if (*cand.offset(hi as isize)).key < (*cand.offset(mid as isize)).key {
            mid = hi;
            if (*cand.offset(lo as isize)).key > (*cand.offset(mid as isize)).key {
                mid = lo;
            }
        }
        stmp = *cand.offset(mid as isize);
        *cand.offset(mid as isize) = *cand.offset(hi as isize);
        *cand.offset(hi as isize) = stmp;
        pivot = (*cand.offset(hi as isize)).key;
        i = lo - 1 as libc::c_int;
        j = lo;
        while j < hi {
            if (*cand.offset(j as isize)).key <= pivot {
                i += 1;
                i;
                stmp = *cand.offset(i as isize);
                *cand.offset(i as isize) = *cand.offset(j as isize);
                *cand.offset(j as isize) = stmp;
            }
            j += 1;
            j;
        }
        i += 1;
        i;
        stmp = *cand.offset(i as isize);
        *cand.offset(i as isize) = *cand.offset(hi as isize);
        *cand.offset(hi as isize) = stmp;
        if i > topk {
            hi = i - 1 as libc::c_int;
        } else {
            if !(i < topk) {
                break;
            }
            lo = i + 1 as libc::c_int;
        }
    }
    return topk;
}
