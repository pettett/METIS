use ::libc;
use libc::printf;

use crate::GKlib::{
    error::gk_errexit,
    mcore::{gk_mcoreCreate, gk_mcoreDestroy, gk_mcoreMalloc, gk_mcorePop, gk_mcorePush},
    memory::{gk_free, gk_malloc, gk_realloc},
};

use super::{
    gklib::{libmetis__iAllocMatrix, libmetis__iFreeMatrix, libmetis__imalloc, libmetis__ismalloc},
    structure::*,
};

#[no_mangle]
pub unsafe extern "C" fn libmetis__AllocateWorkSpace(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
) {
    let mut coresize: size_t = 0;
    match (*ctrl).optype as libc::c_uint {
        0 => {
            coresize = ((3 as libc::c_int * ((*graph).nvtxs + 1)) as u64)
                .wrapping_mul(::core::mem::size_of::<idx_t>() as u64)
                .wrapping_add(
                    ((5 as libc::c_int * ((*ctrl).nparts + 1) * (*graph).ncon)
                        as u64)
                        .wrapping_mul(::core::mem::size_of::<idx_t>() as u64),
                )
                .wrapping_add(
                    ((5 as libc::c_int * ((*ctrl).nparts + 1) * (*graph).ncon)
                        as u64)
                        .wrapping_mul(::core::mem::size_of::<real_t>() as u64),
                );
        }
        _ => {
            coresize = ((4 as libc::c_int * ((*graph).nvtxs + 1)) as u64)
                .wrapping_mul(::core::mem::size_of::<idx_t>() as u64)
                .wrapping_add(
                    ((5 as libc::c_int * ((*ctrl).nparts + 1) * (*graph).ncon)
                        as u64)
                        .wrapping_mul(::core::mem::size_of::<idx_t>() as u64),
                )
                .wrapping_add(
                    ((5 as libc::c_int * ((*ctrl).nparts + 1) * (*graph).ncon)
                        as u64)
                        .wrapping_mul(::core::mem::size_of::<real_t>() as u64),
                );
        }
    }
    (*ctrl).mcore = gk_mcoreCreate(coresize);
    (*ctrl).nbrpoolsize = 0 as libc::c_int as size_t;
    (*ctrl).nbrpoolcpos = 0 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__AllocateRefinementWorkSpace(
    mut ctrl: *mut ctrl_t,
    mut nbrpoolsize: idx_t,
) {
    (*ctrl).nbrpoolsize = nbrpoolsize as size_t;
    (*ctrl).nbrpoolcpos = 0 as libc::c_int as size_t;
    (*ctrl).nbrpoolreallocs = 0 as libc::c_int as size_t;
    match (*ctrl).objtype as libc::c_uint {
        0 => {
            (*ctrl).cnbrpool = gk_malloc(
                ((*ctrl).nbrpoolsize).wrapping_mul(::core::mem::size_of::<cnbr_t>() as u64),
                b"AllocateRefinementWorkSpace: cnbrpool\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ) as *mut cnbr_t;
        }
        1 => {
            (*ctrl).vnbrpool = gk_malloc(
                ((*ctrl).nbrpoolsize).wrapping_mul(::core::mem::size_of::<vnbr_t>() as u64),
                b"AllocateRefinementWorkSpace: vnbrpool\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ) as *mut vnbr_t;
        }
        _ => {
            gk_errexit(
                15 as libc::c_int,
                b"Unknown objtype of %d\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*ctrl).objtype as libc::c_uint,
            );
        }
    }
    if (*ctrl).minconn != 0 {
        (*ctrl).pvec1 = libmetis__imalloc(
            ((*ctrl).nparts + 1) as size_t,
            b"AllocateRefinementWorkSpace: pvec1\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        (*ctrl).pvec2 = libmetis__imalloc(
            ((*ctrl).nparts + 1) as size_t,
            b"AllocateRefinementWorkSpace: pvec2\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        (*ctrl).maxnads = libmetis__ismalloc(
            (*ctrl).nparts as size_t,
            200 as libc::c_int,
            b"AllocateRefinementWorkSpace: maxnads\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        (*ctrl).nads = libmetis__imalloc(
            (*ctrl).nparts as size_t,
            b"AllocateRefinementWorkSpace: nads\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        (*ctrl).adids = libmetis__iAllocMatrix(
            (*ctrl).nparts as size_t,
            200 as libc::c_int as size_t,
            0 as libc::c_int,
            b"AllocateRefinementWorkSpace: adids\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        (*ctrl).adwgts = libmetis__iAllocMatrix(
            (*ctrl).nparts as size_t,
            200 as libc::c_int as size_t,
            0 as libc::c_int,
            b"AllocateRefinementWorkSpace: adwgts\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__FreeWorkSpace(mut ctrl: *mut ctrl_t) {
    gk_mcoreDestroy(
        &mut (*ctrl).mcore,
        ((*ctrl).dbglvl as libc::c_uint & METIS_DBG_INFO as libc::c_int as libc::c_uint)
            as libc::c_int,
    );
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_INFO as libc::c_int as libc::c_uint != 0 {
        printf(
            b" nbrpool statistics\n        nbrpoolsize: %12zu   nbrpoolcpos: %12zu\n    nbrpoolreallocs: %12zu\n\n\0"
                as *const u8 as *const libc::c_char,
            (*ctrl).nbrpoolsize,
            (*ctrl).nbrpoolcpos,
            (*ctrl).nbrpoolreallocs,
        );
    }
    gk_free(
        &mut (*ctrl).cnbrpool as *mut *mut cnbr_t as *mut *mut libc::c_void,
        &mut (*ctrl).vnbrpool as *mut *mut vnbr_t,
        0 as *mut *mut libc::c_void,
    );
    (*ctrl).nbrpoolsize = 0 as libc::c_int as size_t;
    (*ctrl).nbrpoolcpos = 0 as libc::c_int as size_t;
    if (*ctrl).minconn != 0 {
        libmetis__iFreeMatrix(
            &mut (*ctrl).adids,
            (*ctrl).nparts as size_t,
            200 as libc::c_int as size_t,
        );
        libmetis__iFreeMatrix(
            &mut (*ctrl).adwgts,
            (*ctrl).nparts as size_t,
            200 as libc::c_int as size_t,
        );
        gk_free(
            &mut (*ctrl).pvec1 as *mut *mut idx_t as *mut *mut libc::c_void,
            &mut (*ctrl).pvec2 as *mut *mut idx_t,
            &mut (*ctrl).maxnads as *mut *mut idx_t,
            &mut (*ctrl).nads as *mut *mut idx_t,
            0 as *mut *mut libc::c_void,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__wspacemalloc(
    mut ctrl: *mut ctrl_t,
    mut nbytes: size_t,
) -> *mut libc::c_void {
    return gk_mcoreMalloc((*ctrl).mcore, nbytes);
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__wspacepush(mut ctrl: *mut ctrl_t) {
    gk_mcorePush((*ctrl).mcore);
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__wspacepop(mut ctrl: *mut ctrl_t) {
    gk_mcorePop((*ctrl).mcore);
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__iwspacemalloc(
    mut ctrl: *mut ctrl_t,
    mut n: idx_t,
) -> *mut idx_t {
    return libmetis__wspacemalloc(
        ctrl,
        (n as u64).wrapping_mul(::core::mem::size_of::<idx_t>() as u64),
    ) as *mut idx_t;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__rwspacemalloc(
    mut ctrl: *mut ctrl_t,
    mut n: idx_t,
) -> *mut real_t {
    return libmetis__wspacemalloc(
        ctrl,
        (n as u64).wrapping_mul(::core::mem::size_of::<real_t>() as u64),
    ) as *mut real_t;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__ikvwspacemalloc(
    mut ctrl: *mut ctrl_t,
    mut n: idx_t,
) -> *mut ikv_t {
    return libmetis__wspacemalloc(
        ctrl,
        (n as u64).wrapping_mul(::core::mem::size_of::<ikv_t>() as u64),
    ) as *mut ikv_t;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__cnbrpoolReset(mut ctrl: *mut ctrl_t) {
    (*ctrl).nbrpoolcpos = 0 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__cnbrpoolGetNext(
    mut ctrl: *mut ctrl_t,
    mut nnbrs: idx_t,
) -> idx_t {
    (*ctrl).nbrpoolcpos =
        ((*ctrl).nbrpoolcpos as u64).wrapping_add(nnbrs as u64) as size_t as size_t;
    if (*ctrl).nbrpoolcpos > (*ctrl).nbrpoolsize {
        (*ctrl).nbrpoolsize = ((*ctrl).nbrpoolsize as u64).wrapping_add(
            if (10 as libc::c_int * nnbrs) as u64
                >= ((*ctrl).nbrpoolsize).wrapping_div(2 as libc::c_int as u64)
            {
                (10 as libc::c_int * nnbrs) as u64
            } else {
                ((*ctrl).nbrpoolsize).wrapping_div(2 as libc::c_int as u64)
            },
        ) as size_t as size_t;
        (*ctrl).cnbrpool = gk_realloc(
            (*ctrl).cnbrpool as *mut libc::c_void,
            ((*ctrl).nbrpoolsize).wrapping_mul(::core::mem::size_of::<cnbr_t>() as u64),
            b"cnbrpoolGet: cnbrpool\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) as *mut cnbr_t;
        (*ctrl).nbrpoolreallocs = ((*ctrl).nbrpoolreallocs).wrapping_add(1);
        (*ctrl).nbrpoolreallocs;
    }
    return ((*ctrl).nbrpoolcpos).wrapping_sub(nnbrs as u64) as idx_t;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__vnbrpoolReset(mut ctrl: *mut ctrl_t) {
    (*ctrl).nbrpoolcpos = 0 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__vnbrpoolGetNext(
    mut ctrl: *mut ctrl_t,
    mut nnbrs: idx_t,
) -> idx_t {
    (*ctrl).nbrpoolcpos =
        ((*ctrl).nbrpoolcpos as u64).wrapping_add(nnbrs as u64) as size_t as size_t;
    if (*ctrl).nbrpoolcpos > (*ctrl).nbrpoolsize {
        (*ctrl).nbrpoolsize = ((*ctrl).nbrpoolsize as u64).wrapping_add(
            if (10 as libc::c_int * nnbrs) as u64
                >= ((*ctrl).nbrpoolsize).wrapping_div(2 as libc::c_int as u64)
            {
                (10 as libc::c_int * nnbrs) as u64
            } else {
                ((*ctrl).nbrpoolsize).wrapping_div(2 as libc::c_int as u64)
            },
        ) as size_t as size_t;
        (*ctrl).vnbrpool = gk_realloc(
            (*ctrl).vnbrpool as *mut libc::c_void,
            ((*ctrl).nbrpoolsize).wrapping_mul(::core::mem::size_of::<vnbr_t>() as u64),
            b"vnbrpoolGet: vnbrpool\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) as *mut vnbr_t;
        (*ctrl).nbrpoolreallocs = ((*ctrl).nbrpoolreallocs).wrapping_add(1);
        (*ctrl).nbrpoolreallocs;
    }
    return ((*ctrl).nbrpoolcpos).wrapping_sub(nnbrs as u64) as idx_t;
}
