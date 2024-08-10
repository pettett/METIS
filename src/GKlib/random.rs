use ::libc;
extern "C" {
    fn rand() -> libc::c_int;
    fn srand(__seed: libc::c_uint);
}
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __ssize_t = libc::c_long;
pub type int32_t = __int32_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
pub type gk_idx_t = ssize_t;
#[no_mangle]
pub unsafe extern "C" fn gk_csrand(mut seed: size_t) {
    gk_randinit(seed);
}
#[no_mangle]
pub unsafe extern "C" fn gk_crandArrayPermuteFine(
    mut n: size_t,
    mut p: *mut libc::c_char,
    mut flag: libc::c_int,
) {
    let mut i: size_t = 0;
    let mut v: size_t = 0;
    let mut tmp: libc::c_char = 0;
    if flag == 1 as libc::c_int {
        i = 0 as libc::c_int as size_t;
        while i < n {
            *p.offset(i as isize) = i as libc::c_char;
            i = i.wrapping_add(1);
            i;
        }
    }
    i = 0 as libc::c_int as size_t;
    while i < n {
        v = gk_crandInRange(n);
        tmp = *p.offset(i as isize);
        *p.offset(i as isize) = *p.offset(v as isize);
        *p.offset(v as isize) = tmp;
        i = i.wrapping_add(1);
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn gk_crandArrayPermute(
    mut n: size_t,
    mut p: *mut libc::c_char,
    mut nshuffles: size_t,
    mut flag: libc::c_int,
) {
    let mut i: size_t = 0;
    let mut u: size_t = 0;
    let mut v: size_t = 0;
    let mut tmp: libc::c_char = 0;
    if flag == 1 as libc::c_int {
        i = 0 as libc::c_int as size_t;
        while i < n {
            *p.offset(i as isize) = i as libc::c_char;
            i = i.wrapping_add(1);
            i;
        }
    }
    if n < 10 as libc::c_int as libc::c_ulong {
        i = 0 as libc::c_int as size_t;
        while i < n {
            v = gk_crandInRange(n);
            u = gk_crandInRange(n);
            tmp = *p.offset(v as isize);
            *p.offset(v as isize) = *p.offset(u as isize);
            *p.offset(u as isize) = tmp;
            i = i.wrapping_add(1);
            i;
        }
    } else {
        i = 0 as libc::c_int as size_t;
        while i < nshuffles {
            v = gk_crandInRange(n.wrapping_sub(3 as libc::c_int as libc::c_ulong));
            u = gk_crandInRange(n.wrapping_sub(3 as libc::c_int as libc::c_ulong));
            tmp = *p.offset(v.wrapping_add(0 as libc::c_int as libc::c_ulong) as isize);
            *p
                .offset(
                    v.wrapping_add(0 as libc::c_int as libc::c_ulong) as isize,
                ) = *p
                .offset(u.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize);
            *p.offset(u.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize) = tmp;
            tmp = *p.offset(v.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
            *p
                .offset(
                    v.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = *p
                .offset(u.wrapping_add(3 as libc::c_int as libc::c_ulong) as isize);
            *p.offset(u.wrapping_add(3 as libc::c_int as libc::c_ulong) as isize) = tmp;
            tmp = *p.offset(v.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize);
            *p
                .offset(
                    v.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize,
                ) = *p
                .offset(u.wrapping_add(0 as libc::c_int as libc::c_ulong) as isize);
            *p.offset(u.wrapping_add(0 as libc::c_int as libc::c_ulong) as isize) = tmp;
            tmp = *p.offset(v.wrapping_add(3 as libc::c_int as libc::c_ulong) as isize);
            *p
                .offset(
                    v.wrapping_add(3 as libc::c_int as libc::c_ulong) as isize,
                ) = *p
                .offset(u.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
            *p.offset(u.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize) = tmp;
            i = i.wrapping_add(1);
            i;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_crand() -> size_t {
    if ::core::mem::size_of::<size_t>() as libc::c_ulong
        <= ::core::mem::size_of::<int32_t>() as libc::c_ulong
    {
        return gk_randint32() as size_t
    } else {
        return gk_randint64()
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_crandInRange(mut max: size_t) -> size_t {
    return (gk_crand()).wrapping_rem(max);
}
#[no_mangle]
pub unsafe extern "C" fn gk_isrand(mut seed: size_t) {
    gk_randinit(seed);
}
#[no_mangle]
pub unsafe extern "C" fn gk_irandArrayPermute(
    mut n: size_t,
    mut p: *mut libc::c_int,
    mut nshuffles: size_t,
    mut flag: libc::c_int,
) {
    let mut i: size_t = 0;
    let mut u: size_t = 0;
    let mut v: size_t = 0;
    let mut tmp: libc::c_int = 0;
    if flag == 1 as libc::c_int {
        i = 0 as libc::c_int as size_t;
        while i < n {
            *p.offset(i as isize) = i as libc::c_int;
            i = i.wrapping_add(1);
            i;
        }
    }
    if n < 10 as libc::c_int as libc::c_ulong {
        i = 0 as libc::c_int as size_t;
        while i < n {
            v = gk_irandInRange(n);
            u = gk_irandInRange(n);
            tmp = *p.offset(v as isize);
            *p.offset(v as isize) = *p.offset(u as isize);
            *p.offset(u as isize) = tmp;
            i = i.wrapping_add(1);
            i;
        }
    } else {
        i = 0 as libc::c_int as size_t;
        while i < nshuffles {
            v = gk_irandInRange(n.wrapping_sub(3 as libc::c_int as libc::c_ulong));
            u = gk_irandInRange(n.wrapping_sub(3 as libc::c_int as libc::c_ulong));
            tmp = *p.offset(v.wrapping_add(0 as libc::c_int as libc::c_ulong) as isize);
            *p
                .offset(
                    v.wrapping_add(0 as libc::c_int as libc::c_ulong) as isize,
                ) = *p
                .offset(u.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize);
            *p.offset(u.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize) = tmp;
            tmp = *p.offset(v.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
            *p
                .offset(
                    v.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = *p
                .offset(u.wrapping_add(3 as libc::c_int as libc::c_ulong) as isize);
            *p.offset(u.wrapping_add(3 as libc::c_int as libc::c_ulong) as isize) = tmp;
            tmp = *p.offset(v.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize);
            *p
                .offset(
                    v.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize,
                ) = *p
                .offset(u.wrapping_add(0 as libc::c_int as libc::c_ulong) as isize);
            *p.offset(u.wrapping_add(0 as libc::c_int as libc::c_ulong) as isize) = tmp;
            tmp = *p.offset(v.wrapping_add(3 as libc::c_int as libc::c_ulong) as isize);
            *p
                .offset(
                    v.wrapping_add(3 as libc::c_int as libc::c_ulong) as isize,
                ) = *p
                .offset(u.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
            *p.offset(u.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize) = tmp;
            i = i.wrapping_add(1);
            i;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_irandArrayPermuteFine(
    mut n: size_t,
    mut p: *mut libc::c_int,
    mut flag: libc::c_int,
) {
    let mut i: size_t = 0;
    let mut v: size_t = 0;
    let mut tmp: libc::c_int = 0;
    if flag == 1 as libc::c_int {
        i = 0 as libc::c_int as size_t;
        while i < n {
            *p.offset(i as isize) = i as libc::c_int;
            i = i.wrapping_add(1);
            i;
        }
    }
    i = 0 as libc::c_int as size_t;
    while i < n {
        v = gk_irandInRange(n);
        tmp = *p.offset(i as isize);
        *p.offset(i as isize) = *p.offset(v as isize);
        *p.offset(v as isize) = tmp;
        i = i.wrapping_add(1);
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn gk_irand() -> size_t {
    if ::core::mem::size_of::<size_t>() as libc::c_ulong
        <= ::core::mem::size_of::<int32_t>() as libc::c_ulong
    {
        return gk_randint32() as size_t
    } else {
        return gk_randint64()
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_irandInRange(mut max: size_t) -> size_t {
    return (gk_irand()).wrapping_rem(max);
}
#[no_mangle]
pub unsafe extern "C" fn gk_frandArrayPermute(
    mut n: size_t,
    mut p: *mut libc::c_float,
    mut nshuffles: size_t,
    mut flag: libc::c_int,
) {
    let mut i: size_t = 0;
    let mut u: size_t = 0;
    let mut v: size_t = 0;
    let mut tmp: libc::c_float = 0.;
    if flag == 1 as libc::c_int {
        i = 0 as libc::c_int as size_t;
        while i < n {
            *p.offset(i as isize) = i as libc::c_float;
            i = i.wrapping_add(1);
            i;
        }
    }
    if n < 10 as libc::c_int as libc::c_ulong {
        i = 0 as libc::c_int as size_t;
        while i < n {
            v = gk_frandInRange(n);
            u = gk_frandInRange(n);
            tmp = *p.offset(v as isize);
            *p.offset(v as isize) = *p.offset(u as isize);
            *p.offset(u as isize) = tmp;
            i = i.wrapping_add(1);
            i;
        }
    } else {
        i = 0 as libc::c_int as size_t;
        while i < nshuffles {
            v = gk_frandInRange(n.wrapping_sub(3 as libc::c_int as libc::c_ulong));
            u = gk_frandInRange(n.wrapping_sub(3 as libc::c_int as libc::c_ulong));
            tmp = *p.offset(v.wrapping_add(0 as libc::c_int as libc::c_ulong) as isize);
            *p
                .offset(
                    v.wrapping_add(0 as libc::c_int as libc::c_ulong) as isize,
                ) = *p
                .offset(u.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize);
            *p.offset(u.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize) = tmp;
            tmp = *p.offset(v.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
            *p
                .offset(
                    v.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = *p
                .offset(u.wrapping_add(3 as libc::c_int as libc::c_ulong) as isize);
            *p.offset(u.wrapping_add(3 as libc::c_int as libc::c_ulong) as isize) = tmp;
            tmp = *p.offset(v.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize);
            *p
                .offset(
                    v.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize,
                ) = *p
                .offset(u.wrapping_add(0 as libc::c_int as libc::c_ulong) as isize);
            *p.offset(u.wrapping_add(0 as libc::c_int as libc::c_ulong) as isize) = tmp;
            tmp = *p.offset(v.wrapping_add(3 as libc::c_int as libc::c_ulong) as isize);
            *p
                .offset(
                    v.wrapping_add(3 as libc::c_int as libc::c_ulong) as isize,
                ) = *p
                .offset(u.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
            *p.offset(u.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize) = tmp;
            i = i.wrapping_add(1);
            i;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_frandArrayPermuteFine(
    mut n: size_t,
    mut p: *mut libc::c_float,
    mut flag: libc::c_int,
) {
    let mut i: size_t = 0;
    let mut v: size_t = 0;
    let mut tmp: libc::c_float = 0.;
    if flag == 1 as libc::c_int {
        i = 0 as libc::c_int as size_t;
        while i < n {
            *p.offset(i as isize) = i as libc::c_float;
            i = i.wrapping_add(1);
            i;
        }
    }
    i = 0 as libc::c_int as size_t;
    while i < n {
        v = gk_frandInRange(n);
        tmp = *p.offset(i as isize);
        *p.offset(i as isize) = *p.offset(v as isize);
        *p.offset(v as isize) = tmp;
        i = i.wrapping_add(1);
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn gk_fsrand(mut seed: size_t) {
    gk_randinit(seed);
}
#[no_mangle]
pub unsafe extern "C" fn gk_frandInRange(mut max: size_t) -> size_t {
    return (gk_frand()).wrapping_rem(max);
}
#[no_mangle]
pub unsafe extern "C" fn gk_frand() -> size_t {
    if ::core::mem::size_of::<size_t>() as libc::c_ulong
        <= ::core::mem::size_of::<int32_t>() as libc::c_ulong
    {
        return gk_randint32() as size_t
    } else {
        return gk_randint64()
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_dsrand(mut seed: size_t) {
    gk_randinit(seed);
}
#[no_mangle]
pub unsafe extern "C" fn gk_drandArrayPermute(
    mut n: size_t,
    mut p: *mut libc::c_double,
    mut nshuffles: size_t,
    mut flag: libc::c_int,
) {
    let mut i: size_t = 0;
    let mut u: size_t = 0;
    let mut v: size_t = 0;
    let mut tmp: libc::c_double = 0.;
    if flag == 1 as libc::c_int {
        i = 0 as libc::c_int as size_t;
        while i < n {
            *p.offset(i as isize) = i as libc::c_double;
            i = i.wrapping_add(1);
            i;
        }
    }
    if n < 10 as libc::c_int as libc::c_ulong {
        i = 0 as libc::c_int as size_t;
        while i < n {
            v = gk_drandInRange(n);
            u = gk_drandInRange(n);
            tmp = *p.offset(v as isize);
            *p.offset(v as isize) = *p.offset(u as isize);
            *p.offset(u as isize) = tmp;
            i = i.wrapping_add(1);
            i;
        }
    } else {
        i = 0 as libc::c_int as size_t;
        while i < nshuffles {
            v = gk_drandInRange(n.wrapping_sub(3 as libc::c_int as libc::c_ulong));
            u = gk_drandInRange(n.wrapping_sub(3 as libc::c_int as libc::c_ulong));
            tmp = *p.offset(v.wrapping_add(0 as libc::c_int as libc::c_ulong) as isize);
            *p
                .offset(
                    v.wrapping_add(0 as libc::c_int as libc::c_ulong) as isize,
                ) = *p
                .offset(u.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize);
            *p.offset(u.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize) = tmp;
            tmp = *p.offset(v.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
            *p
                .offset(
                    v.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = *p
                .offset(u.wrapping_add(3 as libc::c_int as libc::c_ulong) as isize);
            *p.offset(u.wrapping_add(3 as libc::c_int as libc::c_ulong) as isize) = tmp;
            tmp = *p.offset(v.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize);
            *p
                .offset(
                    v.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize,
                ) = *p
                .offset(u.wrapping_add(0 as libc::c_int as libc::c_ulong) as isize);
            *p.offset(u.wrapping_add(0 as libc::c_int as libc::c_ulong) as isize) = tmp;
            tmp = *p.offset(v.wrapping_add(3 as libc::c_int as libc::c_ulong) as isize);
            *p
                .offset(
                    v.wrapping_add(3 as libc::c_int as libc::c_ulong) as isize,
                ) = *p
                .offset(u.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
            *p.offset(u.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize) = tmp;
            i = i.wrapping_add(1);
            i;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_drandArrayPermuteFine(
    mut n: size_t,
    mut p: *mut libc::c_double,
    mut flag: libc::c_int,
) {
    let mut i: size_t = 0;
    let mut v: size_t = 0;
    let mut tmp: libc::c_double = 0.;
    if flag == 1 as libc::c_int {
        i = 0 as libc::c_int as size_t;
        while i < n {
            *p.offset(i as isize) = i as libc::c_double;
            i = i.wrapping_add(1);
            i;
        }
    }
    i = 0 as libc::c_int as size_t;
    while i < n {
        v = gk_drandInRange(n);
        tmp = *p.offset(i as isize);
        *p.offset(i as isize) = *p.offset(v as isize);
        *p.offset(v as isize) = tmp;
        i = i.wrapping_add(1);
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn gk_drand() -> size_t {
    if ::core::mem::size_of::<size_t>() as libc::c_ulong
        <= ::core::mem::size_of::<int32_t>() as libc::c_ulong
    {
        return gk_randint32() as size_t
    } else {
        return gk_randint64()
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_drandInRange(mut max: size_t) -> size_t {
    return (gk_drand()).wrapping_rem(max);
}
#[no_mangle]
pub unsafe extern "C" fn gk_idxsrand(mut seed: size_t) {
    gk_randinit(seed);
}
#[no_mangle]
pub unsafe extern "C" fn gk_idxrandArrayPermute(
    mut n: size_t,
    mut p: *mut gk_idx_t,
    mut nshuffles: size_t,
    mut flag: libc::c_int,
) {
    let mut i: size_t = 0;
    let mut u: size_t = 0;
    let mut v: size_t = 0;
    let mut tmp: gk_idx_t = 0;
    if flag == 1 as libc::c_int {
        i = 0 as libc::c_int as size_t;
        while i < n {
            *p.offset(i as isize) = i as gk_idx_t;
            i = i.wrapping_add(1);
            i;
        }
    }
    if n < 10 as libc::c_int as libc::c_ulong {
        i = 0 as libc::c_int as size_t;
        while i < n {
            v = gk_idxrandInRange(n);
            u = gk_idxrandInRange(n);
            tmp = *p.offset(v as isize);
            *p.offset(v as isize) = *p.offset(u as isize);
            *p.offset(u as isize) = tmp;
            i = i.wrapping_add(1);
            i;
        }
    } else {
        i = 0 as libc::c_int as size_t;
        while i < nshuffles {
            v = gk_idxrandInRange(n.wrapping_sub(3 as libc::c_int as libc::c_ulong));
            u = gk_idxrandInRange(n.wrapping_sub(3 as libc::c_int as libc::c_ulong));
            tmp = *p.offset(v.wrapping_add(0 as libc::c_int as libc::c_ulong) as isize);
            *p
                .offset(
                    v.wrapping_add(0 as libc::c_int as libc::c_ulong) as isize,
                ) = *p
                .offset(u.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize);
            *p.offset(u.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize) = tmp;
            tmp = *p.offset(v.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
            *p
                .offset(
                    v.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = *p
                .offset(u.wrapping_add(3 as libc::c_int as libc::c_ulong) as isize);
            *p.offset(u.wrapping_add(3 as libc::c_int as libc::c_ulong) as isize) = tmp;
            tmp = *p.offset(v.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize);
            *p
                .offset(
                    v.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize,
                ) = *p
                .offset(u.wrapping_add(0 as libc::c_int as libc::c_ulong) as isize);
            *p.offset(u.wrapping_add(0 as libc::c_int as libc::c_ulong) as isize) = tmp;
            tmp = *p.offset(v.wrapping_add(3 as libc::c_int as libc::c_ulong) as isize);
            *p
                .offset(
                    v.wrapping_add(3 as libc::c_int as libc::c_ulong) as isize,
                ) = *p
                .offset(u.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
            *p.offset(u.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize) = tmp;
            i = i.wrapping_add(1);
            i;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_idxrandArrayPermuteFine(
    mut n: size_t,
    mut p: *mut gk_idx_t,
    mut flag: libc::c_int,
) {
    let mut i: size_t = 0;
    let mut v: size_t = 0;
    let mut tmp: gk_idx_t = 0;
    if flag == 1 as libc::c_int {
        i = 0 as libc::c_int as size_t;
        while i < n {
            *p.offset(i as isize) = i as gk_idx_t;
            i = i.wrapping_add(1);
            i;
        }
    }
    i = 0 as libc::c_int as size_t;
    while i < n {
        v = gk_idxrandInRange(n);
        tmp = *p.offset(i as isize);
        *p.offset(i as isize) = *p.offset(v as isize);
        *p.offset(v as isize) = tmp;
        i = i.wrapping_add(1);
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn gk_idxrand() -> size_t {
    if ::core::mem::size_of::<size_t>() as libc::c_ulong
        <= ::core::mem::size_of::<int32_t>() as libc::c_ulong
    {
        return gk_randint32() as size_t
    } else {
        return gk_randint64()
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_idxrandInRange(mut max: size_t) -> size_t {
    return (gk_idxrand()).wrapping_rem(max);
}
#[no_mangle]
pub unsafe extern "C" fn gk_zrandArrayPermute(
    mut n: size_t,
    mut p: *mut ssize_t,
    mut nshuffles: size_t,
    mut flag: libc::c_int,
) {
    let mut i: size_t = 0;
    let mut u: size_t = 0;
    let mut v: size_t = 0;
    let mut tmp: ssize_t = 0;
    if flag == 1 as libc::c_int {
        i = 0 as libc::c_int as size_t;
        while i < n {
            *p.offset(i as isize) = i as ssize_t;
            i = i.wrapping_add(1);
            i;
        }
    }
    if n < 10 as libc::c_int as libc::c_ulong {
        i = 0 as libc::c_int as size_t;
        while i < n {
            v = gk_zrandInRange(n);
            u = gk_zrandInRange(n);
            tmp = *p.offset(v as isize);
            *p.offset(v as isize) = *p.offset(u as isize);
            *p.offset(u as isize) = tmp;
            i = i.wrapping_add(1);
            i;
        }
    } else {
        i = 0 as libc::c_int as size_t;
        while i < nshuffles {
            v = gk_zrandInRange(n.wrapping_sub(3 as libc::c_int as libc::c_ulong));
            u = gk_zrandInRange(n.wrapping_sub(3 as libc::c_int as libc::c_ulong));
            tmp = *p.offset(v.wrapping_add(0 as libc::c_int as libc::c_ulong) as isize);
            *p
                .offset(
                    v.wrapping_add(0 as libc::c_int as libc::c_ulong) as isize,
                ) = *p
                .offset(u.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize);
            *p.offset(u.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize) = tmp;
            tmp = *p.offset(v.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
            *p
                .offset(
                    v.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = *p
                .offset(u.wrapping_add(3 as libc::c_int as libc::c_ulong) as isize);
            *p.offset(u.wrapping_add(3 as libc::c_int as libc::c_ulong) as isize) = tmp;
            tmp = *p.offset(v.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize);
            *p
                .offset(
                    v.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize,
                ) = *p
                .offset(u.wrapping_add(0 as libc::c_int as libc::c_ulong) as isize);
            *p.offset(u.wrapping_add(0 as libc::c_int as libc::c_ulong) as isize) = tmp;
            tmp = *p.offset(v.wrapping_add(3 as libc::c_int as libc::c_ulong) as isize);
            *p
                .offset(
                    v.wrapping_add(3 as libc::c_int as libc::c_ulong) as isize,
                ) = *p
                .offset(u.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
            *p.offset(u.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize) = tmp;
            i = i.wrapping_add(1);
            i;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_zrandArrayPermuteFine(
    mut n: size_t,
    mut p: *mut ssize_t,
    mut flag: libc::c_int,
) {
    let mut i: size_t = 0;
    let mut v: size_t = 0;
    let mut tmp: ssize_t = 0;
    if flag == 1 as libc::c_int {
        i = 0 as libc::c_int as size_t;
        while i < n {
            *p.offset(i as isize) = i as ssize_t;
            i = i.wrapping_add(1);
            i;
        }
    }
    i = 0 as libc::c_int as size_t;
    while i < n {
        v = gk_zrandInRange(n);
        tmp = *p.offset(i as isize);
        *p.offset(i as isize) = *p.offset(v as isize);
        *p.offset(v as isize) = tmp;
        i = i.wrapping_add(1);
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn gk_zsrand(mut seed: size_t) {
    gk_randinit(seed);
}
#[no_mangle]
pub unsafe extern "C" fn gk_zrand() -> size_t {
    if ::core::mem::size_of::<size_t>() as libc::c_ulong
        <= ::core::mem::size_of::<int32_t>() as libc::c_ulong
    {
        return gk_randint32() as size_t
    } else {
        return gk_randint64()
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_zrandInRange(mut max: size_t) -> size_t {
    return (gk_zrand()).wrapping_rem(max);
}
#[no_mangle]
pub unsafe extern "C" fn gk_randinit(mut seed: uint64_t) {
    srand(seed as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn gk_randint64() -> uint64_t {
    return (rand() as uint64_t) << 32 as libc::c_int | rand() as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn gk_randint32() -> uint32_t {
    return rand() as uint32_t;
}
