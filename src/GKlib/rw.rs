use ::libc;
extern "C" {
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gk_dsmalloc(n: size_t, ival: libc::c_double, msg: *mut libc::c_char) -> *mut libc::c_double;
    fn gk_dset(n: size_t, val: libc::c_double, x: *mut libc::c_double) -> *mut libc::c_double;
    fn gk_free(ptr1: *mut *mut libc::c_void, _: ...);
}
pub type __int32_t = libc::c_int;
pub type __ssize_t = i64;
pub type int32_t = __int32_t;
pub type ssize_t = __ssize_t;
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gk_csr_t {
    pub nrows: int32_t,
    pub ncols: int32_t,
    pub rowptr: *mut ssize_t,
    pub colptr: *mut ssize_t,
    pub rowind: *mut int32_t,
    pub colind: *mut int32_t,
    pub rowids: *mut int32_t,
    pub colids: *mut int32_t,
    pub rowval: *mut libc::c_float,
    pub colval: *mut libc::c_float,
    pub rnorms: *mut libc::c_float,
    pub cnorms: *mut libc::c_float,
    pub rsums: *mut libc::c_float,
    pub csums: *mut libc::c_float,
    pub rsizes: *mut libc::c_float,
    pub csizes: *mut libc::c_float,
    pub rvols: *mut libc::c_float,
    pub cvols: *mut libc::c_float,
    pub rwgts: *mut libc::c_float,
    pub cwgts: *mut libc::c_float,
}
#[no_mangle]
pub unsafe extern "C" fn gk_rw_PageRank(
    mut mat: *mut gk_csr_t,
    mut lamda: libc::c_float,
    mut eps: libc::c_float,
    mut max_niter: libc::c_int,
    mut pr: *mut libc::c_float,
) -> libc::c_int {
    let mut i: ssize_t = 0;
    let mut j: ssize_t = 0;
    let mut k: ssize_t = 0;
    let mut iter: ssize_t = 0;
    let mut nrows: ssize_t = 0;
    let mut rscale: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut prold: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut prnew: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut prtmp: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut fromsinks: libc::c_double = 0.;
    let mut error: libc::c_double = 0.;
    let mut rowptr: *mut ssize_t = 0 as *mut ssize_t;
    let mut rowind: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut rowval: *mut libc::c_float = 0 as *mut libc::c_float;
    nrows = (*mat).nrows as ssize_t;
    rowptr = (*mat).rowptr;
    rowind = (*mat).rowind;
    rowval = (*mat).rowval;
    prold = gk_dsmalloc(
        nrows as size_t,
        0 as libc::c_int as libc::c_double,
        b"gk_rw_PageRank: prnew\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    prnew = gk_dsmalloc(
        nrows as size_t,
        0 as libc::c_int as libc::c_double,
        b"gk_rw_PageRank: prold\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    rscale = gk_dsmalloc(
        nrows as size_t,
        0 as libc::c_int as libc::c_double,
        b"gk_rw_PageRank: rscale\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as libc::c_int as ssize_t;
    while i < nrows {
        j = *rowptr.offset(i as isize);
        while j < *rowptr.offset((i + 1 as i64) as isize) {
            *rscale.offset(i as isize) += *rowval.offset(j as isize) as libc::c_double;
            j += 1;
            j;
        }
        if *rscale.offset(i as isize) > 0 as libc::c_int as libc::c_double {
            *rscale.offset(i as isize) = 1.0f64 / *rscale.offset(i as isize);
        }
        i += 1;
        i;
    }
    i = 0 as libc::c_int as ssize_t;
    while i < nrows {
        *prnew.offset(i as isize) = *pr.offset(i as isize) as libc::c_double;
        i += 1;
        i;
    }
    iter = 0 as libc::c_int as ssize_t;
    while iter < max_niter as i64 {
        prtmp = prnew;
        prnew = prold;
        prold = prtmp;
        gk_dset(nrows as size_t, 0.0f64, prnew);
        fromsinks = 0.0f64;
        i = 0 as libc::c_int as ssize_t;
        while i < nrows {
            if *rscale.offset(i as isize) == 0 as libc::c_int as libc::c_double {
                fromsinks += *prold.offset(i as isize);
            }
            i += 1;
            i;
        }
        i = 0 as libc::c_int as ssize_t;
        while i < nrows {
            j = *rowptr.offset(i as isize);
            while j < *rowptr.offset((i + 1 as i64) as isize) {
                *prnew.offset(*rowind.offset(j as isize) as isize) += *prold.offset(i as isize)
                    * *rscale.offset(i as isize)
                    * *rowval.offset(j as isize) as libc::c_double;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
        i = 0 as libc::c_int as ssize_t;
        while i < nrows {
            *prnew.offset(i as isize) = lamda as libc::c_double
                * (fromsinks * *pr.offset(i as isize) as libc::c_double
                    + *prnew.offset(i as isize))
                + (1.0f64 - lamda as libc::c_double) * *pr.offset(i as isize) as libc::c_double;
            i += 1;
            i;
        }
        error = 0.0f64;
        i = 0 as libc::c_int as ssize_t;
        while i < nrows {
            error = if fabs(*prnew.offset(i as isize) - *prold.offset(i as isize)) > error {
                fabs(*prnew.offset(i as isize) - *prold.offset(i as isize))
            } else {
                error
            };
            i += 1;
            i;
        }
        if error < eps as libc::c_double {
            break;
        }
        iter += 1;
        iter;
    }
    i = 0 as libc::c_int as ssize_t;
    while i < nrows {
        *pr.offset(i as isize) = *prnew.offset(i as isize) as libc::c_float;
        i += 1;
        i;
    }
    gk_free(
        &mut prnew as *mut *mut libc::c_double as *mut *mut libc::c_void,
        &mut prold as *mut *mut libc::c_double,
        &mut rscale as *mut *mut libc::c_double,
        0 as *mut *mut libc::c_void,
    );
    return (iter + 1 as i64) as libc::c_int;
}
