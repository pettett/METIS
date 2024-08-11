use ::libc;

use super::structure::*;

#[no_mangle]
pub unsafe extern "C" fn libmetis__rvecle(
    mut n: idx_t,
    mut x: *mut real_t,
    mut y: *mut real_t,
) -> libc::c_int {
    n -= 1;
    n;
    while n >= 0 as libc::c_int {
        if *x.offset(n as isize) > *y.offset(n as isize) {
            return 0 as libc::c_int;
        }
        n -= 1;
        n;
    }
    return 1;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__rvecge(
    mut n: idx_t,
    mut x: *mut real_t,
    mut y: *mut real_t,
) -> libc::c_int {
    n -= 1;
    n;
    while n >= 0 as libc::c_int {
        if *x.offset(n as isize) < *y.offset(n as isize) {
            return 0 as libc::c_int;
        }
        n -= 1;
        n;
    }
    return 1;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__rvecsumle(
    mut n: idx_t,
    mut x1: *mut real_t,
    mut x2: *mut real_t,
    mut y: *mut real_t,
) -> libc::c_int {
    n -= 1;
    n;
    while n >= 0 as libc::c_int {
        if *x1.offset(n as isize) + *x2.offset(n as isize) > *y.offset(n as isize) {
            return 0 as libc::c_int;
        }
        n -= 1;
        n;
    }
    return 1;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__rvecmaxdiff(
    mut n: idx_t,
    mut x: *mut real_t,
    mut y: *mut real_t,
) -> real_t {
    let mut max: real_t = 0.;
    max = *x.offset(0 as libc::c_int as isize) - *y.offset(0 as libc::c_int as isize);
    n -= 1;
    n;
    while n > 0 as libc::c_int {
        if max < *x.offset(n as isize) - *y.offset(n as isize) {
            max = *x.offset(n as isize) - *y.offset(n as isize);
        }
        n -= 1;
        n;
    }
    return max;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__ivecle(
    mut n: idx_t,
    mut x: *mut idx_t,
    mut z: *mut idx_t,
) -> libc::c_int {
    n -= 1;
    n;
    while n >= 0 as libc::c_int {
        if *x.offset(n as isize) > *z.offset(n as isize) {
            return 0 as libc::c_int;
        }
        n -= 1;
        n;
    }
    return 1;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__ivecge(
    mut n: idx_t,
    mut x: *mut idx_t,
    mut z: *mut idx_t,
) -> libc::c_int {
    n -= 1;
    n;
    while n >= 0 as libc::c_int {
        if *x.offset(n as isize) < *z.offset(n as isize) {
            return 0 as libc::c_int;
        }
        n -= 1;
        n;
    }
    return 1;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__ivecaxpylez(
    mut n: idx_t,
    mut a: idx_t,
    mut x: *mut idx_t,
    mut y: *mut idx_t,
    mut z: *mut idx_t,
) -> libc::c_int {
    n -= 1;
    n;
    while n >= 0 as libc::c_int {
        if a * *x.offset(n as isize) + *y.offset(n as isize) > *z.offset(n as isize) {
            return 0 as libc::c_int;
        }
        n -= 1;
        n;
    }
    return 1;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__ivecaxpygez(
    mut n: idx_t,
    mut a: idx_t,
    mut x: *mut idx_t,
    mut y: *mut idx_t,
    mut z: *mut idx_t,
) -> libc::c_int {
    n -= 1;
    n;
    while n >= 0 as libc::c_int {
        if a * *x.offset(n as isize) + *y.offset(n as isize) < *z.offset(n as isize) {
            return 0 as libc::c_int;
        }
        n -= 1;
        n;
    }
    return 1;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__BetterVBalance(
    mut ncon: idx_t,
    mut invtvwgt: *mut real_t,
    mut v_vwgt: *mut idx_t,
    mut u1_vwgt: *mut idx_t,
    mut u2_vwgt: *mut idx_t,
) -> libc::c_int {
    let mut i: idx_t = 0;
    let mut sum1: real_t = 0.0f64 as real_t;
    let mut sum2: real_t = 0.0f64 as real_t;
    let mut diff1: real_t = 0.0f64 as real_t;
    let mut diff2: real_t = 0.0f64 as real_t;
    i = 0 as libc::c_int;
    while i < ncon {
        sum1 += (*v_vwgt.offset(i as isize) + *u1_vwgt.offset(i as isize)) as libc::c_float
            * *invtvwgt.offset(i as isize);
        sum2 += (*v_vwgt.offset(i as isize) + *u2_vwgt.offset(i as isize)) as libc::c_float
            * *invtvwgt.offset(i as isize);
        i += 1;
        i;
    }
    sum1 = sum1 / ncon as libc::c_float;
    sum2 = sum2 / ncon as libc::c_float;
    i = 0 as libc::c_int;
    while i < ncon {
        diff1 += (sum1
            - (*v_vwgt.offset(i as isize) + *u1_vwgt.offset(i as isize)) as libc::c_float
                * *invtvwgt.offset(i as isize))
        .abs();
        diff2 += (sum2
            - (*v_vwgt.offset(i as isize) + *u2_vwgt.offset(i as isize)) as libc::c_float
                * *invtvwgt.offset(i as isize))
        .abs();
        i += 1;
        i;
    }
    return (diff1 - diff2 >= 0 as libc::c_int as libc::c_float) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__BetterBalance2Way(
    mut n: idx_t,
    mut x: *mut real_t,
    mut y: *mut real_t,
) -> libc::c_int {
    let mut nrm1: real_t = 0.0f64 as real_t;
    let mut nrm2: real_t = 0.0f64 as real_t;
    n -= 1;
    n;
    while n >= 0 as libc::c_int {
        if *x.offset(n as isize) > 0 as libc::c_int as libc::c_float {
            nrm1 += *x.offset(n as isize) * *x.offset(n as isize);
        }
        if *y.offset(n as isize) > 0 as libc::c_int as libc::c_float {
            nrm2 += *y.offset(n as isize) * *y.offset(n as isize);
        }
        n -= 1;
        n;
    }
    return (nrm2 < nrm1) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__BetterBalanceKWay(
    mut ncon: idx_t,
    mut vwgt: *mut idx_t,
    mut ubvec: *mut real_t,
    mut a1: idx_t,
    mut pt1: *mut idx_t,
    mut bm1: *mut real_t,
    mut a2: idx_t,
    mut pt2: *mut idx_t,
    mut bm2: *mut real_t,
) -> libc::c_int {
    let mut i: idx_t = 0;
    let mut tmp: real_t = 0.;
    let mut nrm1: real_t = 0.0f64 as real_t;
    let mut nrm2: real_t = 0.0f64 as real_t;
    let mut max1: real_t = 0.0f64 as real_t;
    let mut max2: real_t = 0.0f64 as real_t;
    i = 0 as libc::c_int;
    while i < ncon {
        tmp = *bm1.offset(i as isize)
            * (*pt1.offset(i as isize) + a1 * *vwgt.offset(i as isize)) as libc::c_float
            - *ubvec.offset(i as isize);
        nrm1 += tmp * tmp;
        max1 = if tmp > max1 { tmp } else { max1 };
        tmp = *bm2.offset(i as isize)
            * (*pt2.offset(i as isize) + a2 * *vwgt.offset(i as isize)) as libc::c_float
            - *ubvec.offset(i as isize);
        nrm2 += tmp * tmp;
        max2 = if tmp > max2 { tmp } else { max2 };
        i += 1;
        i;
    }
    if max2 < max1 {
        return 1;
    }
    if max2 == max1 && nrm2 < nrm1 {
        return 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__ComputeLoadImbalance(
    mut graph: *mut graph_t,
    mut nparts: idx_t,
    mut pijbm: *mut real_t,
) -> real_t {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut ncon: idx_t = 0;
    let mut pwgts: *mut idx_t = 0 as *mut idx_t;
    let mut max: real_t = 0.;
    let mut cur: real_t = 0.;
    ncon = (*graph).ncon;
    pwgts = (*graph).pwgts;
    max = 1.0f64 as real_t;
    i = 0 as libc::c_int;
    while i < ncon {
        j = 0 as libc::c_int;
        while j < nparts {
            cur = *pwgts.offset((j * ncon + i) as isize) as libc::c_float
                * *pijbm.offset((j * ncon + i) as isize);
            if cur > max {
                max = cur;
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    return max;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__ComputeLoadImbalanceDiff(
    mut graph: *mut graph_t,
    mut nparts: idx_t,
    mut pijbm: *mut real_t,
    mut ubvec: *mut real_t,
) -> real_t {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut ncon: idx_t = 0;
    let mut pwgts: *mut idx_t = 0 as *mut idx_t;
    let mut max: real_t = 0.;
    let mut cur: real_t = 0.;
    ncon = (*graph).ncon;
    pwgts = (*graph).pwgts;
    max = -1.0f64 as real_t;
    i = 0 as libc::c_int;
    while i < ncon {
        j = 0 as libc::c_int;
        while j < nparts {
            cur = *pwgts.offset((j * ncon + i) as isize) as libc::c_float
                * *pijbm.offset((j * ncon + i) as isize)
                - *ubvec.offset(i as isize);
            if cur > max {
                max = cur;
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    return max;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__ComputeLoadImbalanceDiffVec(
    mut graph: *mut graph_t,
    mut nparts: idx_t,
    mut pijbm: *mut real_t,
    mut ubfactors: *mut real_t,
    mut diffvec: *mut real_t,
) -> real_t {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut ncon: idx_t = 0;
    let mut pwgts: *mut idx_t = 0 as *mut idx_t;
    let mut cur: real_t = 0.;
    let mut max: real_t = 0.;
    ncon = (*graph).ncon;
    pwgts = (*graph).pwgts;
    max = -1.0f64 as real_t;
    i = 0 as libc::c_int;
    while i < ncon {
        *diffvec.offset(i as isize) = *pwgts.offset(i as isize) as libc::c_float
            * *pijbm.offset(i as isize)
            - *ubfactors.offset(i as isize);
        j = 1;
        while j < nparts {
            cur = *pwgts.offset((j * ncon + i) as isize) as libc::c_float
                * *pijbm.offset((j * ncon + i) as isize)
                - *ubfactors.offset(i as isize);
            if cur > *diffvec.offset(i as isize) {
                *diffvec.offset(i as isize) = cur;
            }
            j += 1;
            j;
        }
        if max < *diffvec.offset(i as isize) {
            max = *diffvec.offset(i as isize);
        }
        i += 1;
        i;
    }
    return max;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__ComputeLoadImbalanceVec(
    mut graph: *mut graph_t,
    mut nparts: idx_t,
    mut pijbm: *mut real_t,
    mut lbvec: *mut real_t,
) {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut ncon: idx_t = 0;
    let mut pwgts: *mut idx_t = 0 as *mut idx_t;
    let mut cur: real_t = 0.;
    ncon = (*graph).ncon;
    pwgts = (*graph).pwgts;
    i = 0 as libc::c_int;
    while i < ncon {
        *lbvec.offset(i as isize) =
            *pwgts.offset(i as isize) as libc::c_float * *pijbm.offset(i as isize);
        j = 1;
        while j < nparts {
            cur = *pwgts.offset((j * ncon + i) as isize) as libc::c_float
                * *pijbm.offset((j * ncon + i) as isize);
            if cur > *lbvec.offset(i as isize) {
                *lbvec.offset(i as isize) = cur;
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
}
