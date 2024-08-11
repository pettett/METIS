use ::libc;
use libc::printf;

use crate::GKlib::error::gk_errexit;
use crate::GKlib::memory::gk_malloc;
use crate::GKlib::timers::gk_CPUSeconds;

use super::balance::*;
use super::fm::libmetis__FM_2WayRefine;
use super::refine::*;
use super::sfm::{libmetis__FM_2WayNodeRefine1Sided, libmetis__FM_2WayNodeRefine2Sided};
use super::srefine::libmetis__Compute2WayNodePartitionParams;
use super::{
    auxapi::*, coarsen::*, contig::*, fortran::*, gklib::*, graph::*, kwayrefine::*, options::*,
    separator::*, structure::*, timing::*, util::*, wspace::*,
};

use super::structure::*;
#[no_mangle]
pub unsafe extern "C" fn libmetis__Init2WayPartition(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut ntpwgts: *mut real_t,
    mut niparts: idx_t,
) {
    let mut dbglvl: mdbglvl_et = 0 as mdbglvl_et;
    dbglvl = (*ctrl).dbglvl;
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_REFINE as libc::c_int as libc::c_uint != 0 {
        (*ctrl).dbglvl = ::core::mem::transmute::<libc::c_uint, mdbglvl_et>(
            ((*ctrl).dbglvl as libc::c_uint)
                .wrapping_sub(METIS_DBG_REFINE as libc::c_int as libc::c_uint),
        ) as mdbglvl_et;
    }
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_MOVEINFO as libc::c_int as libc::c_uint != 0 {
        (*ctrl).dbglvl = ::core::mem::transmute::<libc::c_uint, mdbglvl_et>(
            ((*ctrl).dbglvl as libc::c_uint)
                .wrapping_sub(METIS_DBG_MOVEINFO as libc::c_int as libc::c_uint),
        ) as mdbglvl_et;
    }
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint != 0 {
        (*ctrl).InitPartTmr -= gk_CPUSeconds();
    }
    match (*ctrl).iptype as libc::c_uint {
        1 => {
            if (*graph).ncon == 1 as libc::c_int {
                libmetis__RandomBisection(ctrl, graph, ntpwgts, niparts);
            } else {
                libmetis__McRandomBisection(ctrl, graph, ntpwgts, niparts);
            }
        }
        0 => {
            if (*graph).nedges == 0 as libc::c_int {
                if (*graph).ncon == 1 as libc::c_int {
                    libmetis__RandomBisection(ctrl, graph, ntpwgts, niparts);
                } else {
                    libmetis__McRandomBisection(ctrl, graph, ntpwgts, niparts);
                }
            } else if (*graph).ncon == 1 as libc::c_int {
                libmetis__GrowBisection(ctrl, graph, ntpwgts, niparts);
            } else {
                libmetis__McGrowBisection(ctrl, graph, ntpwgts, niparts);
            }
        }
        _ => {
            gk_errexit(
                15 as libc::c_int,
                b"Unknown initial partition type: %d\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*ctrl).iptype as libc::c_uint,
            );
        }
    }
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_IPART as libc::c_int as libc::c_uint != 0 {
        printf(
            b"Initial Cut: %d\n\0" as *const u8 as *const libc::c_char,
            (*graph).mincut,
        );
    }
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint != 0 {
        (*ctrl).InitPartTmr += gk_CPUSeconds();
    }
    (*ctrl).dbglvl = dbglvl;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__InitSeparator(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut niparts: idx_t,
) {
    let mut ntpwgts: [real_t; 2] = [0.5f64 as real_t, 0.5f64 as real_t];
    let mut dbglvl: mdbglvl_et = 0 as mdbglvl_et;
    dbglvl = (*ctrl).dbglvl;
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_REFINE as libc::c_int as libc::c_uint != 0 {
        (*ctrl).dbglvl = ::core::mem::transmute::<libc::c_uint, mdbglvl_et>(
            ((*ctrl).dbglvl as libc::c_uint)
                .wrapping_sub(METIS_DBG_REFINE as libc::c_int as libc::c_uint),
        ) as mdbglvl_et;
    }
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_MOVEINFO as libc::c_int as libc::c_uint != 0 {
        (*ctrl).dbglvl = ::core::mem::transmute::<libc::c_uint, mdbglvl_et>(
            ((*ctrl).dbglvl as libc::c_uint)
                .wrapping_sub(METIS_DBG_MOVEINFO as libc::c_int as libc::c_uint),
        ) as mdbglvl_et;
    }
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint != 0 {
        (*ctrl).InitPartTmr -= gk_CPUSeconds();
    }
    libmetis__Setup2WayBalMultipliers(ctrl, graph, ntpwgts.as_mut_ptr());
    match (*ctrl).iptype as libc::c_uint {
        2 => {
            if (*graph).nedges == 0 as libc::c_int {
                libmetis__RandomBisection(ctrl, graph, ntpwgts.as_mut_ptr(), niparts);
            } else {
                libmetis__GrowBisection(ctrl, graph, ntpwgts.as_mut_ptr(), niparts);
            }
            libmetis__Compute2WayPartitionParams(ctrl, graph);
            libmetis__ConstructSeparator(ctrl, graph);
        }
        3 => {
            libmetis__GrowBisectionNode(ctrl, graph, ntpwgts.as_mut_ptr(), niparts);
        }
        _ => {
            gk_errexit(
                15 as libc::c_int,
                b"Unkown iptype of %d\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*ctrl).iptype as libc::c_uint,
            );
        }
    }
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_IPART as libc::c_int as libc::c_uint != 0 {
        printf(
            b"Initial Sep: %d\n\0" as *const u8 as *const libc::c_char,
            (*graph).mincut,
        );
    }
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint != 0 {
        (*ctrl).InitPartTmr += gk_CPUSeconds();
    }
    (*ctrl).dbglvl = dbglvl;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__RandomBisection(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut ntpwgts: *mut real_t,
    mut niparts: idx_t,
) {
    let mut i: idx_t = 0;
    let mut ii: idx_t = 0;
    let mut j: idx_t = 0;
    let mut k: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut pwgts: [idx_t; 2] = [0; 2];
    let mut zeromaxpwgt: idx_t = 0;
    let mut from: idx_t = 0;
    let mut me: idx_t = 0;
    let mut bestcut: idx_t = 0 as libc::c_int;
    let mut icut: idx_t = 0;
    let mut mincut: idx_t = 0;
    let mut inbfs: idx_t = 0;
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
    let mut vwgt: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut adjwgt: *mut idx_t = 0 as *mut idx_t;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    let mut perm: *mut idx_t = 0 as *mut idx_t;
    let mut bestwhere: *mut idx_t = 0 as *mut idx_t;
    libmetis__wspacepush(ctrl);
    nvtxs = (*graph).nvtxs;
    xadj = (*graph).xadj;
    vwgt = (*graph).vwgt;
    adjncy = (*graph).adjncy;
    adjwgt = (*graph).adjwgt;
    libmetis__Allocate2WayPartitionMemory(ctrl, graph);
    where_0 = (*graph).where_0;
    bestwhere = libmetis__iwspacemalloc(ctrl, nvtxs);
    perm = libmetis__iwspacemalloc(ctrl, nvtxs);
    zeromaxpwgt = (*((*ctrl).ubfactors).offset(0 as libc::c_int as isize)
        * *((*graph).tvwgt).offset(0 as libc::c_int as isize) as libc::c_float
        * *ntpwgts.offset(0 as libc::c_int as isize)) as idx_t;
    inbfs = 0 as libc::c_int;
    while inbfs < niparts {
        libmetis__iset(nvtxs as size_t, 1 as libc::c_int, where_0);
        if inbfs > 0 as libc::c_int {
            libmetis__irandArrayPermute(nvtxs, perm, nvtxs / 2 as libc::c_int, 1 as libc::c_int);
            pwgts[1 as libc::c_int as usize] = *((*graph).tvwgt).offset(0 as libc::c_int as isize);
            pwgts[0 as libc::c_int as usize] = 0 as libc::c_int;
            ii = 0 as libc::c_int;
            while ii < nvtxs {
                i = *perm.offset(ii as isize);
                if pwgts[0 as libc::c_int as usize] + *vwgt.offset(i as isize) < zeromaxpwgt {
                    *where_0.offset(i as isize) = 0 as libc::c_int;
                    pwgts[0 as libc::c_int as usize] += *vwgt.offset(i as isize);
                    pwgts[1 as libc::c_int as usize] -= *vwgt.offset(i as isize);
                    if pwgts[0 as libc::c_int as usize] > zeromaxpwgt {
                        break;
                    }
                }
                ii += 1;
                ii;
            }
        }
        libmetis__Compute2WayPartitionParams(ctrl, graph);
        libmetis__Balance2Way(ctrl, graph, ntpwgts);
        libmetis__FM_2WayRefine(ctrl, graph, ntpwgts, 4 as libc::c_int);
        if inbfs == 0 as libc::c_int || bestcut > (*graph).mincut {
            bestcut = (*graph).mincut;
            libmetis__icopy(nvtxs as size_t, where_0, bestwhere);
            if bestcut == 0 as libc::c_int {
                break;
            }
        }
        inbfs += 1;
        inbfs;
    }
    (*graph).mincut = bestcut;
    libmetis__icopy(nvtxs as size_t, bestwhere, where_0);
    libmetis__wspacepop(ctrl);
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__GrowBisection(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut ntpwgts: *mut real_t,
    mut niparts: idx_t,
) {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut k: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut drain: idx_t = 0;
    let mut nleft: idx_t = 0;
    let mut first: idx_t = 0;
    let mut last: idx_t = 0;
    let mut pwgts: [idx_t; 2] = [0; 2];
    let mut oneminpwgt: idx_t = 0;
    let mut onemaxpwgt: idx_t = 0;
    let mut from: idx_t = 0;
    let mut me: idx_t = 0;
    let mut bestcut: idx_t = 0 as libc::c_int;
    let mut icut: idx_t = 0;
    let mut mincut: idx_t = 0;
    let mut inbfs: idx_t = 0;
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
    let mut vwgt: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut adjwgt: *mut idx_t = 0 as *mut idx_t;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    let mut queue: *mut idx_t = 0 as *mut idx_t;
    let mut touched: *mut idx_t = 0 as *mut idx_t;
    let mut gain: *mut idx_t = 0 as *mut idx_t;
    let mut bestwhere: *mut idx_t = 0 as *mut idx_t;
    libmetis__wspacepush(ctrl);
    nvtxs = (*graph).nvtxs;
    xadj = (*graph).xadj;
    vwgt = (*graph).vwgt;
    adjncy = (*graph).adjncy;
    adjwgt = (*graph).adjwgt;
    libmetis__Allocate2WayPartitionMemory(ctrl, graph);
    where_0 = (*graph).where_0;
    bestwhere = libmetis__iwspacemalloc(ctrl, nvtxs);
    queue = libmetis__iwspacemalloc(ctrl, nvtxs);
    touched = libmetis__iwspacemalloc(ctrl, nvtxs);
    onemaxpwgt = (*((*ctrl).ubfactors).offset(0 as libc::c_int as isize)
        * *((*graph).tvwgt).offset(0 as libc::c_int as isize) as libc::c_float
        * *ntpwgts.offset(1 as libc::c_int as isize)) as idx_t;
    oneminpwgt = (1.0f64 / *((*ctrl).ubfactors).offset(0 as libc::c_int as isize) as libc::c_double
        * *((*graph).tvwgt).offset(0 as libc::c_int as isize) as libc::c_double
        * *ntpwgts.offset(1 as libc::c_int as isize) as libc::c_double) as idx_t;
    inbfs = 0 as libc::c_int;
    while inbfs < niparts {
        libmetis__iset(nvtxs as size_t, 1 as libc::c_int, where_0);
        libmetis__iset(nvtxs as size_t, 0 as libc::c_int, touched);
        pwgts[1 as libc::c_int as usize] = *((*graph).tvwgt).offset(0 as libc::c_int as isize);
        pwgts[0 as libc::c_int as usize] = 0 as libc::c_int;
        *queue.offset(0 as libc::c_int as isize) = libmetis__irandInRange(nvtxs);
        *touched.offset(*queue.offset(0 as libc::c_int as isize) as isize) = 1 as libc::c_int;
        first = 0 as libc::c_int;
        last = 1 as libc::c_int;
        nleft = nvtxs - 1 as libc::c_int;
        drain = 0 as libc::c_int;
        loop {
            if first == last {
                if nleft == 0 as libc::c_int || drain != 0 {
                    break;
                }
                k = libmetis__irandInRange(nleft);
                i = 0 as libc::c_int;
                while i < nvtxs {
                    if *touched.offset(i as isize) == 0 as libc::c_int {
                        if k == 0 as libc::c_int {
                            break;
                        }
                        k -= 1;
                        k;
                    }
                    i += 1;
                    i;
                }
                *queue.offset(0 as libc::c_int as isize) = i;
                *touched.offset(i as isize) = 1 as libc::c_int;
                first = 0 as libc::c_int;
                last = 1 as libc::c_int;
                nleft -= 1;
                nleft;
            }
            let fresh0 = first;
            first = first + 1;
            i = *queue.offset(fresh0 as isize);
            if pwgts[0 as libc::c_int as usize] > 0 as libc::c_int
                && pwgts[1 as libc::c_int as usize] - *vwgt.offset(i as isize) < oneminpwgt
            {
                drain = 1 as libc::c_int;
            } else {
                *where_0.offset(i as isize) = 0 as libc::c_int;
                pwgts[0 as libc::c_int as usize] += *vwgt.offset(i as isize);
                pwgts[1 as libc::c_int as usize] -= *vwgt.offset(i as isize);
                if pwgts[1 as libc::c_int as usize] <= onemaxpwgt {
                    break;
                }
                drain = 0 as libc::c_int;
                j = *xadj.offset(i as isize);
                while j < *xadj.offset((i + 1 as libc::c_int) as isize) {
                    k = *adjncy.offset(j as isize);
                    if *touched.offset(k as isize) == 0 as libc::c_int {
                        let fresh1 = last;
                        last = last + 1;
                        *queue.offset(fresh1 as isize) = k;
                        *touched.offset(k as isize) = 1 as libc::c_int;
                        nleft -= 1;
                        nleft;
                    }
                    j += 1;
                    j;
                }
            }
        }
        if pwgts[1 as libc::c_int as usize] == 0 as libc::c_int {
            *where_0.offset(libmetis__irandInRange(nvtxs) as isize) = 1 as libc::c_int;
        }
        if pwgts[0 as libc::c_int as usize] == 0 as libc::c_int {
            *where_0.offset(libmetis__irandInRange(nvtxs) as isize) = 0 as libc::c_int;
        }
        libmetis__Compute2WayPartitionParams(ctrl, graph);
        libmetis__Balance2Way(ctrl, graph, ntpwgts);
        libmetis__FM_2WayRefine(ctrl, graph, ntpwgts, (*ctrl).niter);
        if inbfs == 0 as libc::c_int || bestcut > (*graph).mincut {
            bestcut = (*graph).mincut;
            libmetis__icopy(nvtxs as size_t, where_0, bestwhere);
            if bestcut == 0 as libc::c_int {
                break;
            }
        }
        inbfs += 1;
        inbfs;
    }
    (*graph).mincut = bestcut;
    libmetis__icopy(nvtxs as size_t, bestwhere, where_0);
    libmetis__wspacepop(ctrl);
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__McRandomBisection(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut ntpwgts: *mut real_t,
    mut niparts: idx_t,
) {
    let mut i: idx_t = 0;
    let mut ii: idx_t = 0;
    let mut j: idx_t = 0;
    let mut k: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut ncon: idx_t = 0;
    let mut from: idx_t = 0;
    let mut bestcut: idx_t = 0 as libc::c_int;
    let mut mincut: idx_t = 0;
    let mut inbfs: idx_t = 0;
    let mut qnum: idx_t = 0;
    let mut bestwhere: *mut idx_t = 0 as *mut idx_t;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    let mut perm: *mut idx_t = 0 as *mut idx_t;
    let mut counts: *mut idx_t = 0 as *mut idx_t;
    let mut vwgt: *mut idx_t = 0 as *mut idx_t;
    libmetis__wspacepush(ctrl);
    nvtxs = (*graph).nvtxs;
    ncon = (*graph).ncon;
    vwgt = (*graph).vwgt;
    libmetis__Allocate2WayPartitionMemory(ctrl, graph);
    where_0 = (*graph).where_0;
    bestwhere = libmetis__iwspacemalloc(ctrl, nvtxs);
    perm = libmetis__iwspacemalloc(ctrl, nvtxs);
    counts = libmetis__iwspacemalloc(ctrl, ncon);
    inbfs = 0 as libc::c_int;
    while inbfs < 2 as libc::c_int * niparts {
        libmetis__irandArrayPermute(nvtxs, perm, nvtxs / 2 as libc::c_int, 1 as libc::c_int);
        libmetis__iset(ncon as size_t, 0 as libc::c_int, counts);
        ii = 0 as libc::c_int;
        while ii < nvtxs {
            i = *perm.offset(ii as isize);
            qnum = libmetis__iargmax(ncon as size_t, vwgt.offset((i * ncon) as isize)) as idx_t;
            let ref mut fresh2 = *counts.offset(qnum as isize);
            let fresh3 = *fresh2;
            *fresh2 = *fresh2 + 1;
            *where_0.offset(i as isize) = fresh3 % 2 as libc::c_int;
            ii += 1;
            ii;
        }
        libmetis__Compute2WayPartitionParams(ctrl, graph);
        libmetis__FM_2WayRefine(ctrl, graph, ntpwgts, (*ctrl).niter);
        libmetis__Balance2Way(ctrl, graph, ntpwgts);
        libmetis__FM_2WayRefine(ctrl, graph, ntpwgts, (*ctrl).niter);
        libmetis__Balance2Way(ctrl, graph, ntpwgts);
        libmetis__FM_2WayRefine(ctrl, graph, ntpwgts, (*ctrl).niter);
        if inbfs == 0 as libc::c_int || bestcut >= (*graph).mincut {
            bestcut = (*graph).mincut;
            libmetis__icopy(nvtxs as size_t, where_0, bestwhere);
            if bestcut == 0 as libc::c_int {
                break;
            }
        }
        inbfs += 1;
        inbfs;
    }
    (*graph).mincut = bestcut;
    libmetis__icopy(nvtxs as size_t, bestwhere, where_0);
    libmetis__wspacepop(ctrl);
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__McGrowBisection(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut ntpwgts: *mut real_t,
    mut niparts: idx_t,
) {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut k: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut ncon: idx_t = 0;
    let mut from: idx_t = 0;
    let mut bestcut: idx_t = 0 as libc::c_int;
    let mut mincut: idx_t = 0;
    let mut inbfs: idx_t = 0;
    let mut bestwhere: *mut idx_t = 0 as *mut idx_t;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    libmetis__wspacepush(ctrl);
    nvtxs = (*graph).nvtxs;
    libmetis__Allocate2WayPartitionMemory(ctrl, graph);
    where_0 = (*graph).where_0;
    bestwhere = libmetis__iwspacemalloc(ctrl, nvtxs);
    inbfs = 0 as libc::c_int;
    while inbfs < 2 as libc::c_int * niparts {
        libmetis__iset(nvtxs as size_t, 1 as libc::c_int, where_0);
        *where_0.offset(libmetis__irandInRange(nvtxs) as isize) = 0 as libc::c_int;
        libmetis__Compute2WayPartitionParams(ctrl, graph);
        libmetis__Balance2Way(ctrl, graph, ntpwgts);
        libmetis__FM_2WayRefine(ctrl, graph, ntpwgts, (*ctrl).niter);
        libmetis__Balance2Way(ctrl, graph, ntpwgts);
        libmetis__FM_2WayRefine(ctrl, graph, ntpwgts, (*ctrl).niter);
        if inbfs == 0 as libc::c_int || bestcut >= (*graph).mincut {
            bestcut = (*graph).mincut;
            libmetis__icopy(nvtxs as size_t, where_0, bestwhere);
            if bestcut == 0 as libc::c_int {
                break;
            }
        }
        inbfs += 1;
        inbfs;
    }
    (*graph).mincut = bestcut;
    libmetis__icopy(nvtxs as size_t, bestwhere, where_0);
    libmetis__wspacepop(ctrl);
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__GrowBisectionNode(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut ntpwgts: *mut real_t,
    mut niparts: idx_t,
) {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut k: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut drain: idx_t = 0;
    let mut nleft: idx_t = 0;
    let mut first: idx_t = 0;
    let mut last: idx_t = 0;
    let mut pwgts: [idx_t; 2] = [0; 2];
    let mut oneminpwgt: idx_t = 0;
    let mut onemaxpwgt: idx_t = 0;
    let mut from: idx_t = 0;
    let mut me: idx_t = 0;
    let mut bestcut: idx_t = 0 as libc::c_int;
    let mut icut: idx_t = 0;
    let mut mincut: idx_t = 0;
    let mut inbfs: idx_t = 0;
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
    let mut vwgt: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut adjwgt: *mut idx_t = 0 as *mut idx_t;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    let mut bndind: *mut idx_t = 0 as *mut idx_t;
    let mut queue: *mut idx_t = 0 as *mut idx_t;
    let mut touched: *mut idx_t = 0 as *mut idx_t;
    let mut gain: *mut idx_t = 0 as *mut idx_t;
    let mut bestwhere: *mut idx_t = 0 as *mut idx_t;
    libmetis__wspacepush(ctrl);
    nvtxs = (*graph).nvtxs;
    xadj = (*graph).xadj;
    vwgt = (*graph).vwgt;
    adjncy = (*graph).adjncy;
    adjwgt = (*graph).adjwgt;
    bestwhere = libmetis__iwspacemalloc(ctrl, nvtxs);
    queue = libmetis__iwspacemalloc(ctrl, nvtxs);
    touched = libmetis__iwspacemalloc(ctrl, nvtxs);
    onemaxpwgt = ((*((*ctrl).ubfactors).offset(0 as libc::c_int as isize)
        * *((*graph).tvwgt).offset(0 as libc::c_int as isize) as libc::c_float)
        as libc::c_double
        * 0.5f64) as idx_t;
    oneminpwgt = (1.0f64 / *((*ctrl).ubfactors).offset(0 as libc::c_int as isize) as libc::c_double
        * *((*graph).tvwgt).offset(0 as libc::c_int as isize) as libc::c_double
        * 0.5f64) as idx_t;
    (*graph).pwgts = libmetis__imalloc(
        3 as libc::c_int as size_t,
        b"GrowBisectionNode: pwgts\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (*graph).where_0 = libmetis__imalloc(
        nvtxs as size_t,
        b"GrowBisectionNode: where\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (*graph).bndptr = libmetis__imalloc(
        nvtxs as size_t,
        b"GrowBisectionNode: bndptr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (*graph).bndind = libmetis__imalloc(
        nvtxs as size_t,
        b"GrowBisectionNode: bndind\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (*graph).id = libmetis__imalloc(
        nvtxs as size_t,
        b"GrowBisectionNode: id\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (*graph).ed = libmetis__imalloc(
        nvtxs as size_t,
        b"GrowBisectionNode: ed\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (*graph).nrinfo = gk_malloc(
        (nvtxs as u64).wrapping_mul(::core::mem::size_of::<nrinfo_t>() as u64),
        b"GrowBisectionNode: nrinfo\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut nrinfo_t;
    where_0 = (*graph).where_0;
    bndind = (*graph).bndind;
    inbfs = 0 as libc::c_int;
    while inbfs < niparts {
        libmetis__iset(nvtxs as size_t, 1 as libc::c_int, where_0);
        libmetis__iset(nvtxs as size_t, 0 as libc::c_int, touched);
        pwgts[1 as libc::c_int as usize] = *((*graph).tvwgt).offset(0 as libc::c_int as isize);
        pwgts[0 as libc::c_int as usize] = 0 as libc::c_int;
        *queue.offset(0 as libc::c_int as isize) = libmetis__irandInRange(nvtxs);
        *touched.offset(*queue.offset(0 as libc::c_int as isize) as isize) = 1 as libc::c_int;
        first = 0 as libc::c_int;
        last = 1 as libc::c_int;
        nleft = nvtxs - 1 as libc::c_int;
        drain = 0 as libc::c_int;
        loop {
            if first == last {
                if nleft == 0 as libc::c_int || drain != 0 {
                    break;
                }
                k = libmetis__irandInRange(nleft);
                i = 0 as libc::c_int;
                while i < nvtxs {
                    if *touched.offset(i as isize) == 0 as libc::c_int {
                        if k == 0 as libc::c_int {
                            break;
                        }
                        k -= 1;
                        k;
                    }
                    i += 1;
                    i;
                }
                *queue.offset(0 as libc::c_int as isize) = i;
                *touched.offset(i as isize) = 1 as libc::c_int;
                first = 0 as libc::c_int;
                last = 1 as libc::c_int;
                nleft -= 1;
                nleft;
            }
            let fresh4 = first;
            first = first + 1;
            i = *queue.offset(fresh4 as isize);
            if pwgts[1 as libc::c_int as usize] - *vwgt.offset(i as isize) < oneminpwgt {
                drain = 1 as libc::c_int;
            } else {
                *where_0.offset(i as isize) = 0 as libc::c_int;
                pwgts[0 as libc::c_int as usize] += *vwgt.offset(i as isize);
                pwgts[1 as libc::c_int as usize] -= *vwgt.offset(i as isize);
                if pwgts[1 as libc::c_int as usize] <= onemaxpwgt {
                    break;
                }
                drain = 0 as libc::c_int;
                j = *xadj.offset(i as isize);
                while j < *xadj.offset((i + 1 as libc::c_int) as isize) {
                    k = *adjncy.offset(j as isize);
                    if *touched.offset(k as isize) == 0 as libc::c_int {
                        let fresh5 = last;
                        last = last + 1;
                        *queue.offset(fresh5 as isize) = k;
                        *touched.offset(k as isize) = 1 as libc::c_int;
                        nleft -= 1;
                        nleft;
                    }
                    j += 1;
                    j;
                }
            }
        }
        libmetis__Compute2WayPartitionParams(ctrl, graph);
        libmetis__Balance2Way(ctrl, graph, ntpwgts);
        libmetis__FM_2WayRefine(ctrl, graph, ntpwgts, 4 as libc::c_int);
        i = 0 as libc::c_int;
        while i < (*graph).nbnd {
            j = *bndind.offset(i as isize);
            if *xadj.offset((j + 1 as libc::c_int) as isize) - *xadj.offset(j as isize)
                > 0 as libc::c_int
            {
                *where_0.offset(j as isize) = 2 as libc::c_int;
            }
            i += 1;
            i;
        }
        libmetis__Compute2WayNodePartitionParams(ctrl, graph);
        libmetis__FM_2WayNodeRefine2Sided(ctrl, graph, 1 as libc::c_int);
        libmetis__FM_2WayNodeRefine1Sided(ctrl, graph, 4 as libc::c_int);
        if inbfs == 0 as libc::c_int || bestcut > (*graph).mincut {
            bestcut = (*graph).mincut;
            libmetis__icopy(nvtxs as size_t, where_0, bestwhere);
        }
        inbfs += 1;
        inbfs;
    }
    (*graph).mincut = bestcut;
    libmetis__icopy(nvtxs as size_t, bestwhere, where_0);
    libmetis__wspacepop(ctrl);
}
#[no_mangle]
pub unsafe extern "C" fn GrowBisectionNode2(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut ntpwgts: *mut real_t,
    mut niparts: idx_t,
) {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut k: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut bestcut: idx_t = 0 as libc::c_int;
    let mut mincut: idx_t = 0;
    let mut inbfs: idx_t = 0;
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    let mut bndind: *mut idx_t = 0 as *mut idx_t;
    let mut bestwhere: *mut idx_t = 0 as *mut idx_t;
    libmetis__wspacepush(ctrl);
    nvtxs = (*graph).nvtxs;
    xadj = (*graph).xadj;
    (*graph).pwgts = libmetis__imalloc(
        3 as libc::c_int as size_t,
        b"GrowBisectionNode: pwgts\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (*graph).where_0 = libmetis__imalloc(
        nvtxs as size_t,
        b"GrowBisectionNode: where\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (*graph).bndptr = libmetis__imalloc(
        nvtxs as size_t,
        b"GrowBisectionNode: bndptr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (*graph).bndind = libmetis__imalloc(
        nvtxs as size_t,
        b"GrowBisectionNode: bndind\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (*graph).id = libmetis__imalloc(
        nvtxs as size_t,
        b"GrowBisectionNode: id\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (*graph).ed = libmetis__imalloc(
        nvtxs as size_t,
        b"GrowBisectionNode: ed\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (*graph).nrinfo = gk_malloc(
        (nvtxs as u64).wrapping_mul(::core::mem::size_of::<nrinfo_t>() as u64),
        b"GrowBisectionNode: nrinfo\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut nrinfo_t;
    bestwhere = libmetis__iwspacemalloc(ctrl, nvtxs);
    where_0 = (*graph).where_0;
    bndind = (*graph).bndind;
    inbfs = 0 as libc::c_int;
    while inbfs < niparts {
        libmetis__iset(nvtxs as size_t, 1 as libc::c_int, where_0);
        if inbfs > 0 as libc::c_int {
            *where_0.offset(libmetis__irandInRange(nvtxs) as isize) = 0 as libc::c_int;
        }
        libmetis__Compute2WayPartitionParams(ctrl, graph);
        libmetis__General2WayBalance(ctrl, graph, ntpwgts);
        libmetis__FM_2WayRefine(ctrl, graph, ntpwgts, (*ctrl).niter);
        i = 0 as libc::c_int;
        while i < (*graph).nbnd {
            j = *bndind.offset(i as isize);
            if *xadj.offset((j + 1 as libc::c_int) as isize) - *xadj.offset(j as isize)
                > 0 as libc::c_int
            {
                *where_0.offset(j as isize) = 2 as libc::c_int;
            }
            i += 1;
            i;
        }
        libmetis__Compute2WayNodePartitionParams(ctrl, graph);
        libmetis__FM_2WayNodeRefine2Sided(ctrl, graph, 4 as libc::c_int);
        if inbfs == 0 as libc::c_int || bestcut > (*graph).mincut {
            bestcut = (*graph).mincut;
            libmetis__icopy(nvtxs as size_t, where_0, bestwhere);
        }
        inbfs += 1;
        inbfs;
    }
    (*graph).mincut = bestcut;
    libmetis__icopy(nvtxs as size_t, bestwhere, where_0);
    libmetis__wspacepop(ctrl);
}
