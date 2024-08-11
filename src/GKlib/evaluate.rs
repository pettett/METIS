use ::libc;
extern "C" {
    fn sqrt(_: libc::c_double) -> libc::c_double;
}
pub type __ssize_t = i64;
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gk_fkv_t {
    pub key: libc::c_float,
    pub val: ssize_t,
}
#[no_mangle]
pub unsafe extern "C" fn ComputeAccuracy(
    mut n: libc::c_int,
    mut list: *mut gk_fkv_t,
) -> libc::c_float {
    let mut i: libc::c_int = 0;
    let mut P: libc::c_int = 0;
    let mut N: libc::c_int = 0;
    let mut TP: libc::c_int = 0;
    let mut FN: libc::c_int = 0 as libc::c_int;
    let mut bAccuracy: libc::c_float = 0.0f64 as libc::c_float;
    let mut acc: libc::c_float = 0.;
    P = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < n {
        P += if (*list.offset(i as isize)).val == 1 as i64 {
            1
        } else {
            0 as libc::c_int
        };
        i += 1;
        i;
    }
    N = n - P;
    FN = 0 as libc::c_int;
    TP = FN;
    i = 0 as libc::c_int;
    while i < n {
        if (*list.offset(i as isize)).val == 1 as i64 {
            TP += 1;
            TP;
        } else {
            FN += 1;
            FN;
        }
        acc = ((TP + N - FN) as libc::c_double * 100.0f64 / (P + N) as libc::c_double)
            as libc::c_float;
        if acc > bAccuracy {
            bAccuracy = acc;
        }
        i += 1;
        i;
    }
    return bAccuracy;
}
#[no_mangle]
pub unsafe extern "C" fn ComputeROCn(
    mut n: libc::c_int,
    mut maxN: libc::c_int,
    mut list: *mut gk_fkv_t,
) -> libc::c_float {
    let mut i: libc::c_int = 0;
    let mut P: libc::c_int = 0;
    let mut TP: libc::c_int = 0;
    let mut FP: libc::c_int = 0;
    let mut TPprev: libc::c_int = 0;
    let mut FPprev: libc::c_int = 0;
    let mut AUC: libc::c_int = 0;
    let mut prev: libc::c_float = 0.;
    AUC = 0 as libc::c_int;
    TPprev = AUC;
    FPprev = TPprev;
    TP = FPprev;
    FP = TP;
    prev = (*list.offset(0 as libc::c_int as isize)).key - 1 as libc::c_float;
    P = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < n {
        P += if (*list.offset(i as isize)).val == 1 as i64 {
            1
        } else {
            0 as libc::c_int
        };
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < n && FP < maxN {
        if (*list.offset(i as isize)).key != prev {
            AUC += (TP + TPprev) * (FP - FPprev) / 2 as libc::c_int;
            prev = (*list.offset(i as isize)).key;
            FPprev = FP;
            TPprev = TP;
        }
        if (*list.offset(i as isize)).val == 1 as i64 {
            TP += 1;
            TP;
        } else {
            FP += 1;
            FP;
        }
        i += 1;
        i;
    }
    AUC += (TP + TPprev) * (FP - FPprev) / 2 as libc::c_int;
    return (if TP * FP > 0 as libc::c_int {
        (1.0f64 * AUC as libc::c_double / (P * FP) as libc::c_double) as libc::c_float
            as libc::c_double
    } else {
        0.0f64
    }) as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn ComputeMedianRFP(
    mut n: libc::c_int,
    mut list: *mut gk_fkv_t,
) -> libc::c_float {
    let mut i: libc::c_int = 0;
    let mut P: libc::c_int = 0;
    let mut N: libc::c_int = 0;
    let mut TP: libc::c_int = 0;
    let mut FP: libc::c_int = 0;
    N = 0 as libc::c_int;
    P = N;
    i = 0 as libc::c_int;
    while i < n {
        if (*list.offset(i as isize)).val == 1 as i64 {
            P += 1;
            P;
        } else {
            N += 1;
            N;
        }
        i += 1;
        i;
    }
    TP = 0 as libc::c_int;
    FP = TP;
    i = 0 as libc::c_int;
    while i < n && TP < (P + 1) / 2 as libc::c_int {
        if (*list.offset(i as isize)).val == 1 as i64 {
            TP += 1;
            TP;
        } else {
            FP += 1;
            FP;
        }
        i += 1;
        i;
    }
    return (1.0f64 * FP as libc::c_double / N as libc::c_double) as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn ComputeMean(
    mut n: libc::c_int,
    mut values: *mut libc::c_float,
) -> libc::c_float {
    let mut i: libc::c_int = 0;
    let mut mean: libc::c_float = 0.0f64 as libc::c_float;
    i = 0 as libc::c_int;
    while i < n {
        mean += *values.offset(i as isize);
        i += 1;
        i;
    }
    return (1.0f64 * mean as libc::c_double / n as libc::c_double) as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn ComputeStdDev(
    mut n: libc::c_int,
    mut values: *mut libc::c_float,
) -> libc::c_float {
    let mut i: libc::c_int = 0;
    let mut mean: libc::c_float = ComputeMean(n, values);
    let mut stdDev: libc::c_float = 0 as libc::c_int as libc::c_float;
    i = 0 as libc::c_int;
    while i < n {
        stdDev += (*values.offset(i as isize) - mean) * (*values.offset(i as isize) - mean);
        i += 1;
        i;
    }
    return sqrt(1.0f64 * stdDev as libc::c_double / n as libc::c_double) as libc::c_float;
}
