use crate::GKlib::{error::gk_errexit, timers::gk_CPUSeconds};

use super::{
    auxapi::*,
    bucketsort::libmetis__BucketSortKeysInc,
    contig::*,
    fortran::*,
    gklib::*,
    graph::*,
    kwayrefine::*,
    mcutil::{libmetis__BetterVBalance, libmetis__ivecaxpylez, libmetis__ivecle},
    options::*,
    structure::*,
    timing::*,
    util::*,
    wspace::*,
};
use ::libc;
use libc::printf;

use super::structure::*;
#[no_mangle]
pub unsafe extern "C" fn libmetis__CoarsenGraph(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
) -> *mut graph_t {
    let mut i: idx_t = 0;
    let mut eqewgts: idx_t = 0;
    let mut level: idx_t = 0 as libc::c_int;
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint != 0 {
        (*ctrl).CoarsenTmr -= gk_CPUSeconds();
    }
    eqewgts = 1;
    i = 1;
    while i < (*graph).nedges {
        if *((*graph).adjwgt).offset(0 as libc::c_int as isize)
            != *((*graph).adjwgt).offset(i as isize)
        {
            eqewgts = 0 as libc::c_int;
            break;
        } else {
            i += 1;
            i;
        }
    }
    i = 0 as libc::c_int;
    while i < (*graph).ncon {
        *((*ctrl).maxvwgt).offset(i as isize) =
            (1.5f64 * *((*graph).tvwgt).offset(i as isize) as libc::c_double
                / (*ctrl).CoarsenTo as libc::c_double) as idx_t;
        i += 1;
        i;
    }
    loop {
        if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_COARSEN as libc::c_int as libc::c_uint != 0 {
            libmetis__PrintCGraphStats(ctrl, graph);
        }
        if ((*graph).cmap).is_null() {
            (*graph).cmap = libmetis__imalloc(
                (*graph).nvtxs as size_t,
                b"CoarsenGraph: graph->cmap\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        match (*ctrl).ctype as libc::c_uint {
            0 => {
                libmetis__Match_RM(ctrl, graph);
            }
            1 => {
                if eqewgts != 0 || (*graph).nedges == 0 as libc::c_int {
                    libmetis__Match_RM(ctrl, graph);
                } else {
                    libmetis__Match_SHEM(ctrl, graph);
                }
            }
            _ => {
                gk_errexit(
                    15 as libc::c_int,
                    b"Unknown ctype: %d\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    (*ctrl).ctype as libc::c_uint,
                );
            }
        }
        graph = (*graph).coarser;
        eqewgts = 0 as libc::c_int;
        level += 1;
        level;
        if !((*graph).nvtxs > (*ctrl).CoarsenTo
            && ((*graph).nvtxs as libc::c_double)
                < 0.85f64 * (*(*graph).finer).nvtxs as libc::c_double
            && (*graph).nedges > (*graph).nvtxs / 2 as libc::c_int)
        {
            break;
        }
    }
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_COARSEN as libc::c_int as libc::c_uint != 0 {
        libmetis__PrintCGraphStats(ctrl, graph);
    }
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint != 0 {
        (*ctrl).CoarsenTmr += gk_CPUSeconds();
    }
    return graph;
}
#[no_mangle]
pub unsafe extern "C" fn CoarsenGraphNlevels(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut nlevels: idx_t,
) -> *mut graph_t {
    let mut i: idx_t = 0;
    let mut eqewgts: idx_t = 0;
    let mut level: idx_t = 0;
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint != 0 {
        (*ctrl).CoarsenTmr -= gk_CPUSeconds();
    }
    eqewgts = 1;
    i = 1;
    while i < (*graph).nedges {
        if *((*graph).adjwgt).offset(0 as libc::c_int as isize)
            != *((*graph).adjwgt).offset(i as isize)
        {
            eqewgts = 0 as libc::c_int;
            break;
        } else {
            i += 1;
            i;
        }
    }
    i = 0 as libc::c_int;
    while i < (*graph).ncon {
        *((*ctrl).maxvwgt).offset(i as isize) =
            (1.5f64 * *((*graph).tvwgt).offset(i as isize) as libc::c_double
                / (*ctrl).CoarsenTo as libc::c_double) as idx_t;
        i += 1;
        i;
    }
    level = 0 as libc::c_int;
    while level < nlevels {
        if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_COARSEN as libc::c_int as libc::c_uint != 0 {
            libmetis__PrintCGraphStats(ctrl, graph);
        }
        if ((*graph).cmap).is_null() {
            (*graph).cmap = libmetis__imalloc(
                (*graph).nvtxs as size_t,
                b"CoarsenGraph: graph->cmap\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        match (*ctrl).ctype as libc::c_uint {
            0 => {
                libmetis__Match_RM(ctrl, graph);
            }
            1 => {
                if eqewgts != 0 || (*graph).nedges == 0 as libc::c_int {
                    libmetis__Match_RM(ctrl, graph);
                } else {
                    libmetis__Match_SHEM(ctrl, graph);
                }
            }
            _ => {
                gk_errexit(
                    15 as libc::c_int,
                    b"Unknown ctype: %d\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    (*ctrl).ctype as libc::c_uint,
                );
            }
        }
        graph = (*graph).coarser;
        eqewgts = 0 as libc::c_int;
        if (*graph).nvtxs < (*ctrl).CoarsenTo
            || (*graph).nvtxs as libc::c_double
                > 0.85f64 * (*(*graph).finer).nvtxs as libc::c_double
            || (*graph).nedges < (*graph).nvtxs / 2 as libc::c_int
        {
            break;
        }
        level += 1;
        level;
    }
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_COARSEN as libc::c_int as libc::c_uint != 0 {
        libmetis__PrintCGraphStats(ctrl, graph);
    }
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint != 0 {
        (*ctrl).CoarsenTmr += gk_CPUSeconds();
    }
    return graph;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__Match_RM(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
) -> idx_t {
    let mut i: idx_t = 0;
    let mut pi: idx_t = 0;
    let mut ii: idx_t = 0;
    let mut j: idx_t = 0;
    let mut jj: idx_t = 0;
    let mut jjinc: idx_t = 0;
    let mut k: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut ncon: idx_t = 0;
    let mut cnvtxs: idx_t = 0;
    let mut maxidx: idx_t = 0;
    let mut last_unmatched: idx_t = 0;

    let mut vwgt: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut adjwgt: *mut idx_t = 0 as *mut idx_t;
    let mut maxvwgt: *mut idx_t = 0 as *mut idx_t;
    let mut match_0: *mut idx_t = 0 as *mut idx_t;
    let mut cmap: *mut idx_t = 0 as *mut idx_t;
    let mut perm: *mut idx_t = 0 as *mut idx_t;
    let mut nunmatched: size_t = 0 as libc::c_int as size_t;
    libmetis__wspacepush(ctrl);
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint != 0 {
        (*ctrl).MatchTmr -= gk_CPUSeconds();
    }
    nvtxs = (*graph).nvtxs;
    ncon = (*graph).ncon;
    let mut xadj = &mut (*graph).xadj;
    vwgt = (*graph).vwgt;
    adjncy = (*graph).adjncy;
    adjwgt = (*graph).adjwgt;
    cmap = (*graph).cmap;
    maxvwgt = (*ctrl).maxvwgt;
    match_0 = libmetis__iset(
        nvtxs as size_t,
        -(1),
        libmetis__iwspacemalloc(ctrl, nvtxs),
    );
    perm = libmetis__iwspacemalloc(ctrl, nvtxs);
    libmetis__irandArrayPermute(nvtxs, perm, nvtxs / 8 as libc::c_int, 1);
    cnvtxs = 0 as libc::c_int;
    last_unmatched = 0 as libc::c_int;
    pi = 0 as libc::c_int;
    while pi < nvtxs {
        i = *perm.offset(pi as isize);
        if *match_0.offset(i as isize) == -(1) {
            maxidx = i;
            if if ncon == 1 {
                (*vwgt.offset(i as isize) < *maxvwgt.offset(0 as libc::c_int as isize))
                    as libc::c_int
            } else {
                libmetis__ivecle(ncon, vwgt.offset((i * ncon) as isize), maxvwgt)
            } != 0
            {
                if xadj[i as usize] == xadj[(i + 1) as usize] {
                    last_unmatched = (if pi >= last_unmatched {
                        pi
                    } else {
                        last_unmatched
                    }) + 1;
                    while last_unmatched < nvtxs {
                        j = *perm.offset(last_unmatched as isize);
                        if *match_0.offset(j as isize) == -(1) {
                            maxidx = j;
                            break;
                        } else {
                            last_unmatched += 1;
                            last_unmatched;
                        }
                    }
                } else if ncon == 1 {
                    j = xadj[i as usize];
                    while j < xadj[(i + 1) as usize] {
                        k = *adjncy.offset(j as isize);
                        if *match_0.offset(k as isize) == -(1)
                            && *vwgt.offset(i as isize) + *vwgt.offset(k as isize)
                                <= *maxvwgt.offset(0 as libc::c_int as isize)
                        {
                            maxidx = k;
                            break;
                        } else {
                            j += 1;
                            j;
                        }
                    }
                    if maxidx == i
                        && 3 as libc::c_int * *vwgt.offset(i as isize)
                            < *maxvwgt.offset(0 as libc::c_int as isize)
                    {
                        nunmatched = nunmatched.wrapping_add(1);
                        nunmatched;
                        maxidx = -(1);
                    }
                } else {
                    j = xadj[i as usize];
                    while j < xadj[(i + 1) as usize] {
                        k = *adjncy.offset(j as isize);
                        if *match_0.offset(k as isize) == -(1)
                            && libmetis__ivecaxpylez(
                                ncon,
                                1,
                                vwgt.offset((i * ncon) as isize),
                                vwgt.offset((k * ncon) as isize),
                                maxvwgt,
                            ) != 0
                        {
                            maxidx = k;
                            break;
                        } else {
                            j += 1;
                            j;
                        }
                    }
                    if maxidx == i
                        && libmetis__ivecaxpylez(
                            ncon,
                            2 as libc::c_int,
                            vwgt.offset((i * ncon) as isize),
                            vwgt.offset((i * ncon) as isize),
                            maxvwgt,
                        ) != 0
                    {
                        nunmatched = nunmatched.wrapping_add(1);
                        nunmatched;
                        maxidx = -(1);
                    }
                }
            }
            if maxidx != -(1) {
                let fresh0 = cnvtxs;
                cnvtxs = cnvtxs + 1;
                let ref mut fresh1 = *cmap.offset(maxidx as isize);
                *fresh1 = fresh0;
                *cmap.offset(i as isize) = *fresh1;
                *match_0.offset(i as isize) = maxidx;
                *match_0.offset(maxidx as isize) = i;
            }
        }
        pi += 1;
        pi;
    }
    if (*ctrl).no2hop == 0 && nunmatched as libc::c_double > 0.10f64 * nvtxs as libc::c_double {
        cnvtxs = libmetis__Match_2Hop(ctrl, graph, perm, match_0, cnvtxs, nunmatched);
    }
    cnvtxs = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < nvtxs {
        if *match_0.offset(i as isize) == -(1) {
            *match_0.offset(i as isize) = i;
            let fresh2 = cnvtxs;
            cnvtxs = cnvtxs + 1;
            *cmap.offset(i as isize) = fresh2;
        } else if i <= *match_0.offset(i as isize) {
            let fresh3 = cnvtxs;
            cnvtxs = cnvtxs + 1;
            let ref mut fresh4 = *cmap.offset(*match_0.offset(i as isize) as isize);
            *fresh4 = fresh3;
            *cmap.offset(i as isize) = *fresh4;
        }
        i += 1;
        i;
    }
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint != 0 {
        (*ctrl).MatchTmr += gk_CPUSeconds();
    }
    libmetis__CreateCoarseGraph(ctrl, graph, cnvtxs, match_0);
    libmetis__wspacepop(ctrl);
    return cnvtxs;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__Match_SHEM(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
) -> idx_t {
    let mut i: idx_t = 0;
    let mut pi: idx_t = 0;
    let mut ii: idx_t = 0;
    let mut j: idx_t = 0;
    let mut jj: idx_t = 0;
    let mut jjinc: idx_t = 0;
    let mut k: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut ncon: idx_t = 0;
    let mut cnvtxs: idx_t = 0;
    let mut maxidx: idx_t = 0;
    let mut maxwgt: idx_t = 0;
    let mut last_unmatched: idx_t = 0;
    let mut avgdegree: idx_t = 0;

    let mut vwgt: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut adjwgt: *mut idx_t = 0 as *mut idx_t;
    let mut maxvwgt: *mut idx_t = 0 as *mut idx_t;
    let mut match_0: *mut idx_t = 0 as *mut idx_t;
    let mut cmap: *mut idx_t = 0 as *mut idx_t;
    let mut degrees: *mut idx_t = 0 as *mut idx_t;
    let mut perm: *mut idx_t = 0 as *mut idx_t;
    let mut tperm: *mut idx_t = 0 as *mut idx_t;
    let mut nunmatched: size_t = 0 as libc::c_int as size_t;
    libmetis__wspacepush(ctrl);
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint != 0 {
        (*ctrl).MatchTmr -= gk_CPUSeconds();
    }
    nvtxs = (*graph).nvtxs;
    ncon = (*graph).ncon;
    let mut xadj = &mut (*graph).xadj;
    vwgt = (*graph).vwgt;
    adjncy = (*graph).adjncy;
    adjwgt = (*graph).adjwgt;
    cmap = (*graph).cmap;
    maxvwgt = (*ctrl).maxvwgt;
    match_0 = libmetis__iset(
        nvtxs as size_t,
        -(1),
        libmetis__iwspacemalloc(ctrl, nvtxs),
    );
    perm = libmetis__iwspacemalloc(ctrl, nvtxs);
    tperm = libmetis__iwspacemalloc(ctrl, nvtxs);
    degrees = libmetis__iwspacemalloc(ctrl, nvtxs);
    libmetis__irandArrayPermute(nvtxs, tperm, nvtxs / 8 as libc::c_int, 1);
    avgdegree = (0.7f64 * (xadj[nvtxs as usize] / nvtxs) as libc::c_double) as idx_t;
    i = 0 as libc::c_int;
    while i < nvtxs {
        *degrees.offset(i as isize) = if xadj[(i + 1) as usize] - xadj[i as usize] > avgdegree {
            avgdegree
        } else {
            xadj[(i + 1) as usize] - xadj[i as usize]
        };
        i += 1;
        i;
    }
    libmetis__BucketSortKeysInc(ctrl, nvtxs, avgdegree, degrees, tperm, perm);
    cnvtxs = 0 as libc::c_int;
    last_unmatched = 0 as libc::c_int;
    pi = 0 as libc::c_int;
    while pi < nvtxs {
        i = *perm.offset(pi as isize);
        if *match_0.offset(i as isize) == -(1) {
            maxidx = i;
            maxwgt = -(1);
            if if ncon == 1 {
                (*vwgt.offset(i as isize) < *maxvwgt.offset(0 as libc::c_int as isize))
                    as libc::c_int
            } else {
                libmetis__ivecle(ncon, vwgt.offset((i * ncon) as isize), maxvwgt)
            } != 0
            {
                if xadj[i as usize] == xadj[(i + 1) as usize] {
                    last_unmatched = (if pi >= last_unmatched {
                        pi
                    } else {
                        last_unmatched
                    }) + 1;
                    while last_unmatched < nvtxs {
                        j = *perm.offset(last_unmatched as isize);
                        if *match_0.offset(j as isize) == -(1) {
                            maxidx = j;
                            break;
                        } else {
                            last_unmatched += 1;
                            last_unmatched;
                        }
                    }
                } else if ncon == 1 {
                    j = xadj[i as usize];
                    while j < xadj[(i + 1) as usize] {
                        k = *adjncy.offset(j as isize);
                        if *match_0.offset(k as isize) == -(1)
                            && maxwgt < *adjwgt.offset(j as isize)
                            && *vwgt.offset(i as isize) + *vwgt.offset(k as isize)
                                <= *maxvwgt.offset(0 as libc::c_int as isize)
                        {
                            maxidx = k;
                            maxwgt = *adjwgt.offset(j as isize);
                        }
                        j += 1;
                        j;
                    }
                    if maxidx == i
                        && 3 as libc::c_int * *vwgt.offset(i as isize)
                            < *maxvwgt.offset(0 as libc::c_int as isize)
                    {
                        nunmatched = nunmatched.wrapping_add(1);
                        nunmatched;
                        maxidx = -(1);
                    }
                } else {
                    j = xadj[i as usize];
                    while j < xadj[(i + 1) as usize] {
                        k = *adjncy.offset(j as isize);
                        if *match_0.offset(k as isize) == -(1)
                            && libmetis__ivecaxpylez(
                                ncon,
                                1,
                                vwgt.offset((i * ncon) as isize),
                                vwgt.offset((k * ncon) as isize),
                                maxvwgt,
                            ) != 0
                            && (maxwgt < *adjwgt.offset(j as isize)
                                || maxwgt == *adjwgt.offset(j as isize)
                                    && libmetis__BetterVBalance(
                                        ncon,
                                        (*graph).invtvwgt,
                                        vwgt.offset((i * ncon) as isize),
                                        vwgt.offset((maxidx * ncon) as isize),
                                        vwgt.offset((k * ncon) as isize),
                                    ) != 0)
                        {
                            maxidx = k;
                            maxwgt = *adjwgt.offset(j as isize);
                        }
                        j += 1;
                        j;
                    }
                    if maxidx == i
                        && libmetis__ivecaxpylez(
                            ncon,
                            2 as libc::c_int,
                            vwgt.offset((i * ncon) as isize),
                            vwgt.offset((i * ncon) as isize),
                            maxvwgt,
                        ) != 0
                    {
                        nunmatched = nunmatched.wrapping_add(1);
                        nunmatched;
                        maxidx = -(1);
                    }
                }
            }
            if maxidx != -(1) {
                let fresh5 = cnvtxs;
                cnvtxs = cnvtxs + 1;
                let ref mut fresh6 = *cmap.offset(maxidx as isize);
                *fresh6 = fresh5;
                *cmap.offset(i as isize) = *fresh6;
                *match_0.offset(i as isize) = maxidx;
                *match_0.offset(maxidx as isize) = i;
            }
        }
        pi += 1;
        pi;
    }
    if (*ctrl).no2hop == 0 && nunmatched as libc::c_double > 0.10f64 * nvtxs as libc::c_double {
        cnvtxs = libmetis__Match_2Hop(ctrl, graph, perm, match_0, cnvtxs, nunmatched);
    }
    cnvtxs = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < nvtxs {
        if *match_0.offset(i as isize) == -(1) {
            *match_0.offset(i as isize) = i;
            let fresh7 = cnvtxs;
            cnvtxs = cnvtxs + 1;
            *cmap.offset(i as isize) = fresh7;
        } else if i <= *match_0.offset(i as isize) {
            let fresh8 = cnvtxs;
            cnvtxs = cnvtxs + 1;
            let ref mut fresh9 = *cmap.offset(*match_0.offset(i as isize) as isize);
            *fresh9 = fresh8;
            *cmap.offset(i as isize) = *fresh9;
        }
        i += 1;
        i;
    }
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint != 0 {
        (*ctrl).MatchTmr += gk_CPUSeconds();
    }
    libmetis__CreateCoarseGraph(ctrl, graph, cnvtxs, match_0);
    libmetis__wspacepop(ctrl);
    return cnvtxs;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__Match_2Hop(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut perm: *mut idx_t,
    mut match_0: *mut idx_t,
    mut cnvtxs: idx_t,
    mut nunmatched: size_t,
) -> idx_t {
    cnvtxs = libmetis__Match_2HopAny(
        ctrl,
        graph,
        perm,
        match_0,
        cnvtxs,
        &mut nunmatched,
        2 as libc::c_int as size_t,
    );
    cnvtxs = libmetis__Match_2HopAll(
        ctrl,
        graph,
        perm,
        match_0,
        cnvtxs,
        &mut nunmatched,
        64 as libc::c_int as size_t,
    );
    if nunmatched as libc::c_double > 1.5f64 * 0.10f64 * (*graph).nvtxs as libc::c_double {
        cnvtxs = libmetis__Match_2HopAny(
            ctrl,
            graph,
            perm,
            match_0,
            cnvtxs,
            &mut nunmatched,
            3 as libc::c_int as size_t,
        );
    }
    if nunmatched as libc::c_double > 2.0f64 * 0.10f64 * (*graph).nvtxs as libc::c_double {
        cnvtxs = libmetis__Match_2HopAny(
            ctrl,
            graph,
            perm,
            match_0,
            cnvtxs,
            &mut nunmatched,
            (*graph).nvtxs as size_t,
        );
    }
    return cnvtxs;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__Match_2HopAny(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut perm: *mut idx_t,
    mut match_0: *mut idx_t,
    mut cnvtxs: idx_t,
    mut r_nunmatched: *mut size_t,
    mut maxdegree: size_t,
) -> idx_t {
    let mut i: idx_t = 0;
    let mut pi: idx_t = 0;
    let mut ii: idx_t = 0;
    let mut j: idx_t = 0;
    let mut jj: idx_t = 0;
    let mut k: idx_t = 0;
    let mut nvtxs: idx_t = 0;

    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut colptr: *mut idx_t = 0 as *mut idx_t;
    let mut rowind: *mut idx_t = 0 as *mut idx_t;
    let mut cmap: *mut idx_t = 0 as *mut idx_t;
    let mut nunmatched: size_t = 0;
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint != 0 {
        (*ctrl).Aux3Tmr -= gk_CPUSeconds();
    }
    nvtxs = (*graph).nvtxs;
    let mut xadj = &mut (*graph).xadj;
    adjncy = (*graph).adjncy;
    cmap = (*graph).cmap;
    nunmatched = *r_nunmatched;
    libmetis__wspacepush(ctrl);
    colptr = libmetis__iset(
        nvtxs as size_t,
        0 as libc::c_int,
        libmetis__iwspacemalloc(ctrl, nvtxs + 1),
    );
    i = 0 as libc::c_int;
    while i < nvtxs {
        if *match_0.offset(i as isize) == -(1)
            && ((xadj[(i + 1) as usize] - xadj[i as usize]) as u64) < maxdegree
        {
            j = xadj[i as usize];
            while j < xadj[(i + 1) as usize] {
                let ref mut fresh10 = *colptr.offset(*adjncy.offset(j as isize) as isize);
                *fresh10 += 1;
                *fresh10;
                j += 1;
                j;
            }
        }
        i += 1;
        i;
    }
    i = 1;
    while i < nvtxs {
        let ref mut fresh11 = *colptr.offset(i as isize);
        *fresh11 += *colptr.offset((i - 1) as isize);
        i += 1;
        i;
    }
    i = nvtxs;
    while i > 0 as libc::c_int {
        *colptr.offset(i as isize) = *colptr.offset((i - 1) as isize);
        i -= 1;
        i;
    }
    *colptr.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    rowind = libmetis__iwspacemalloc(ctrl, *colptr.offset(nvtxs as isize));
    pi = 0 as libc::c_int;
    while pi < nvtxs {
        i = *perm.offset(pi as isize);
        if *match_0.offset(i as isize) == -(1)
            && ((xadj[(i + 1) as usize] - xadj[i as usize]) as u64) < maxdegree
        {
            j = xadj[i as usize];
            while j < xadj[(i + 1) as usize] {
                let ref mut fresh12 = *colptr.offset(*adjncy.offset(j as isize) as isize);
                let fresh13 = *fresh12;
                *fresh12 = *fresh12 + 1;
                *rowind.offset(fresh13 as isize) = i;
                j += 1;
                j;
            }
        }
        pi += 1;
        pi;
    }
    i = nvtxs;
    while i > 0 as libc::c_int {
        *colptr.offset(i as isize) = *colptr.offset((i - 1) as isize);
        i -= 1;
        i;
    }
    *colptr.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    pi = 0 as libc::c_int;
    while pi < nvtxs {
        i = *perm.offset(pi as isize);
        if !(*colptr.offset((i + 1) as isize) - *colptr.offset(i as isize)
            < 2 as libc::c_int)
        {
            jj = *colptr.offset((i + 1) as isize);
            j = *colptr.offset(i as isize);
            while j < jj {
                if *match_0.offset(*rowind.offset(j as isize) as isize) == -(1) {
                    jj -= 1;
                    jj;
                    while jj > j {
                        if *match_0.offset(*rowind.offset(jj as isize) as isize)
                            == -(1)
                        {
                            let fresh14 = cnvtxs;
                            cnvtxs = cnvtxs + 1;
                            let ref mut fresh15 =
                                *cmap.offset(*rowind.offset(jj as isize) as isize);
                            *fresh15 = fresh14;
                            *cmap.offset(*rowind.offset(j as isize) as isize) = *fresh15;
                            *match_0.offset(*rowind.offset(j as isize) as isize) =
                                *rowind.offset(jj as isize);
                            *match_0.offset(*rowind.offset(jj as isize) as isize) =
                                *rowind.offset(j as isize);
                            nunmatched = (nunmatched as u64).wrapping_sub(2 as libc::c_int as u64)
                                as size_t as size_t;
                            break;
                        } else {
                            jj -= 1;
                            jj;
                        }
                    }
                }
                j += 1;
                j;
            }
        }
        pi += 1;
        pi;
    }
    libmetis__wspacepop(ctrl);
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint != 0 {
        (*ctrl).Aux3Tmr += gk_CPUSeconds();
    }
    *r_nunmatched = nunmatched;
    return cnvtxs;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__Match_2HopAll(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut perm: *mut idx_t,
    mut match_0: *mut idx_t,
    mut cnvtxs: idx_t,
    mut r_nunmatched: *mut size_t,
    mut maxdegree: size_t,
) -> idx_t {
    let mut i: idx_t = 0;
    let mut pi: idx_t = 0;
    let mut pk: idx_t = 0;
    let mut ii: idx_t = 0;
    let mut j: idx_t = 0;
    let mut jj: idx_t = 0;
    let mut k: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut mask: idx_t = 0;
    let mut idegree: idx_t = 0;

    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut cmap: *mut idx_t = 0 as *mut idx_t;
    let mut mark: *mut idx_t = 0 as *mut idx_t;
    let mut keys: *mut ikv_t = 0 as *mut ikv_t;
    let mut nunmatched: size_t = 0;
    let mut ncand: size_t = 0;
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint != 0 {
        (*ctrl).Aux3Tmr -= gk_CPUSeconds();
    }
    nvtxs = (*graph).nvtxs;
    let mut xadj = &mut (*graph).xadj;
    adjncy = (*graph).adjncy;
    cmap = (*graph).cmap;
    nunmatched = *r_nunmatched;
    mask = (2147483647 as libc::c_int as u64).wrapping_div(maxdegree) as idx_t;
    libmetis__wspacepush(ctrl);
    keys = libmetis__ikvwspacemalloc(ctrl, nunmatched as idx_t);
    ncand = 0 as libc::c_int as size_t;
    pi = 0 as libc::c_int;
    while pi < nvtxs {
        i = *perm.offset(pi as isize);
        idegree = xadj[(i + 1) as usize] - xadj[i as usize];
        if *match_0.offset(i as isize) == -(1)
            && idegree > 1
            && (idegree as u64) < maxdegree
        {
            k = 0 as libc::c_int;
            j = xadj[i as usize];
            while j < xadj[(i + 1) as usize] {
                k += *adjncy.offset(j as isize) % mask;
                j += 1;
                j;
            }
            (*keys.offset(ncand as isize)).val = i;
            (*keys.offset(ncand as isize)).key = ((k % mask) as u64)
                .wrapping_mul(maxdegree)
                .wrapping_add(idegree as u64)
                as idx_t;
            ncand = ncand.wrapping_add(1);
            ncand;
        }
        pi += 1;
        pi;
    }
    libmetis__ikvsorti(ncand, keys);
    mark = libmetis__iset(
        nvtxs as size_t,
        0 as libc::c_int,
        libmetis__iwspacemalloc(ctrl, nvtxs),
    );
    pi = 0 as libc::c_int;
    while (pi as u64) < ncand {
        i = (*keys.offset(pi as isize)).val;
        if !(*match_0.offset(i as isize) != -(1)) {
            j = xadj[i as usize];
            while j < xadj[(i + 1) as usize] {
                *mark.offset(*adjncy.offset(j as isize) as isize) = i;
                j += 1;
                j;
            }
            pk = pi + 1;
            while (pk as u64) < ncand {
                k = (*keys.offset(pk as isize)).val;
                if !(*match_0.offset(k as isize) != -(1)) {
                    if (*keys.offset(pi as isize)).key != (*keys.offset(pk as isize)).key {
                        break;
                    }
                    if xadj[(i + 1) as usize] - xadj[i as usize]
                        != xadj[(k + 1) as usize] - xadj[k as usize]
                    {
                        break;
                    }
                    jj = xadj[k as usize];
                    while jj < xadj[(k + 1) as usize] {
                        if *mark.offset(*adjncy.offset(jj as isize) as isize) != i {
                            break;
                        }
                        jj += 1;
                        jj;
                    }
                    if jj == xadj[(k + 1) as usize] {
                        let fresh16 = cnvtxs;
                        cnvtxs = cnvtxs + 1;
                        let ref mut fresh17 = *cmap.offset(k as isize);
                        *fresh17 = fresh16;
                        *cmap.offset(i as isize) = *fresh17;
                        *match_0.offset(i as isize) = k;
                        *match_0.offset(k as isize) = i;
                        nunmatched = (nunmatched as u64).wrapping_sub(2 as libc::c_int as u64)
                            as size_t as size_t;
                        break;
                    }
                }
                pk += 1;
                pk;
            }
        }
        pi += 1;
        pi;
    }
    libmetis__wspacepop(ctrl);
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint != 0 {
        (*ctrl).Aux3Tmr += gk_CPUSeconds();
    }
    *r_nunmatched = nunmatched;
    return cnvtxs;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__PrintCGraphStats(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
) {
    let mut i: idx_t = 0;
    printf(
        b"%10d %10d %10d [%d] [\0" as *const u8 as *const libc::c_char,
        (*graph).nvtxs,
        (*graph).nedges,
        libmetis__isum(
            (*graph).nedges as size_t,
            (*graph).adjwgt,
            1 as size_t,
        ),
        (*ctrl).CoarsenTo,
    );
    i = 0 as libc::c_int;
    while i < (*graph).ncon {
        printf(
            b" %8d:%8d\0" as *const u8 as *const libc::c_char,
            *((*ctrl).maxvwgt).offset(i as isize),
            *((*graph).tvwgt).offset(i as isize),
        );
        i += 1;
        i;
    }
    printf(b" ]\n\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__CreateCoarseGraph(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut cnvtxs: idx_t,
    mut match_0: *mut idx_t,
) {
    let mut j: idx_t = 0;
    let mut jj: idx_t = 0;
    let mut k: idx_t = 0;
    let mut kk: idx_t = 0;
    let mut l: idx_t = 0;
    let mut m: idx_t = 0;
    let mut istart: idx_t = 0;
    let mut iend: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut nedges: idx_t = 0;
    let mut ncon: idx_t = 0;
    let mut cnedges: idx_t = 0;
    let mut v: idx_t = 0;
    let mut u: idx_t = 0;
    let mut mask: idx_t = 0;
    let mut dovsize: idx_t = 0;

    let mut vwgt: *mut idx_t = 0 as *mut idx_t;
    let mut vsize: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut adjwgt: *mut idx_t = 0 as *mut idx_t;
    let mut cmap: *mut idx_t = 0 as *mut idx_t;
    let mut htable: *mut idx_t = 0 as *mut idx_t;
    let mut cxadj;
    let mut cvwgt: *mut idx_t = 0 as *mut idx_t;
    let mut cvsize: *mut idx_t = 0 as *mut idx_t;
    let mut cadjncy: *mut idx_t = 0 as *mut idx_t;
    let mut cadjwgt: *mut idx_t = 0 as *mut idx_t;
    let mut cgraph: *mut graph_t = 0 as *mut graph_t;
    dovsize = if (*ctrl).objtype as libc::c_uint == METIS_OBJTYPE_VOL as libc::c_int as libc::c_uint
    {
        1
    } else {
        0 as libc::c_int
    };
    mask = ((1) << 11) - 1;
    if cnvtxs < 2 as libc::c_int * mask
        || (*graph).nedges / (*graph).nvtxs > mask / 20 as libc::c_int
    {
        libmetis__CreateCoarseGraphNoMask(ctrl, graph, cnvtxs, match_0);
        return;
    }
    nvtxs = (*graph).nvtxs;
    let mut xadj = &mut (*graph).xadj;
    v = 0 as libc::c_int;
    while v < nvtxs {
        if xadj[((v + 1) as usize)] - xadj[(v as usize)] > mask >> 3 as libc::c_int {
            libmetis__CreateCoarseGraphNoMask(ctrl, graph, cnvtxs, match_0);
            return;
        }
        v += 1;
        v;
    }
    libmetis__wspacepush(ctrl);
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint != 0 {
        (*ctrl).ContractTmr -= gk_CPUSeconds();
    }
    ncon = (*graph).ncon;
    vwgt = (*graph).vwgt;
    vsize = (*graph).vsize;
    adjncy = (*graph).adjncy;
    adjwgt = (*graph).adjwgt;
    cmap = (*graph).cmap;
    cgraph = libmetis__SetupCoarseGraph(graph, cnvtxs, dovsize);
    cxadj = (*cgraph).xadj.clone();
    cvwgt = (*cgraph).vwgt;
    cvsize = (*cgraph).vsize;
    cadjncy = (*cgraph).adjncy;
    cadjwgt = (*cgraph).adjwgt;
    htable = libmetis__iset(
        (if cnvtxs + 1 >= mask + 1 {
            mask + 1
        } else {
            cnvtxs + 1
        }) as size_t,
        -(1),
        libmetis__iwspacemalloc(ctrl, mask + 1),
    );
    cnedges = 0 as libc::c_int;
    cnvtxs = cnedges;
    cxadj[0] = cnvtxs;
    v = 0 as libc::c_int;
    while v < nvtxs {
        u = *match_0.offset(v as isize);
        if !(u < v) {
            if ncon == 1 {
                *cvwgt.offset(cnvtxs as isize) = *vwgt.offset(v as isize);
            } else {
                libmetis__icopy(
                    ncon as size_t,
                    vwgt.offset((v * ncon) as isize),
                    cvwgt.offset((cnvtxs * ncon) as isize),
                );
            }
            if dovsize != 0 {
                *cvsize.offset(cnvtxs as isize) = *vsize.offset(v as isize);
            }
            nedges = 0 as libc::c_int;
            istart = xadj[(v as usize)];
            iend = xadj[((v + 1) as usize)];
            j = istart;
            while j < iend {
                k = *cmap.offset(*adjncy.offset(j as isize) as isize);
                kk = k & mask;
                m = *htable.offset(kk as isize);
                if m == -(1) {
                    *cadjncy.offset(nedges as isize) = k;
                    *cadjwgt.offset(nedges as isize) = *adjwgt.offset(j as isize);
                    let fresh18 = nedges;
                    nedges = nedges + 1;
                    *htable.offset(kk as isize) = fresh18;
                } else if *cadjncy.offset(m as isize) == k {
                    let ref mut fresh19 = *cadjwgt.offset(m as isize);
                    *fresh19 += *adjwgt.offset(j as isize);
                } else {
                    jj = 0 as libc::c_int;
                    while jj < nedges {
                        if *cadjncy.offset(jj as isize) == k {
                            let ref mut fresh20 = *cadjwgt.offset(jj as isize);
                            *fresh20 += *adjwgt.offset(j as isize);
                            break;
                        } else {
                            jj += 1;
                            jj;
                        }
                    }
                    if jj == nedges {
                        *cadjncy.offset(nedges as isize) = k;
                        let fresh21 = nedges;
                        nedges = nedges + 1;
                        *cadjwgt.offset(fresh21 as isize) = *adjwgt.offset(j as isize);
                    }
                }
                j += 1;
                j;
            }
            if v != u {
                if ncon == 1 {
                    let ref mut fresh22 = *cvwgt.offset(cnvtxs as isize);
                    *fresh22 += *vwgt.offset(u as isize);
                } else {
                    libmetis__iaxpy(
                        ncon as size_t,
                        1,
                        vwgt.offset((u * ncon) as isize),
                        1 as size_t,
                        cvwgt.offset((cnvtxs * ncon) as isize),
                        1 as size_t,
                    );
                }
                if dovsize != 0 {
                    let ref mut fresh23 = *cvsize.offset(cnvtxs as isize);
                    *fresh23 += *vsize.offset(u as isize);
                }
                istart = xadj[(u as usize)];
                iend = xadj[(u + 1) as usize];
                j = istart;
                while j < iend {
                    k = *cmap.offset(*adjncy.offset(j as isize) as isize);
                    kk = k & mask;
                    m = *htable.offset(kk as isize);
                    if m == -(1) {
                        *cadjncy.offset(nedges as isize) = k;
                        *cadjwgt.offset(nedges as isize) = *adjwgt.offset(j as isize);
                        let fresh24 = nedges;
                        nedges = nedges + 1;
                        *htable.offset(kk as isize) = fresh24;
                    } else if *cadjncy.offset(m as isize) == k {
                        let ref mut fresh25 = *cadjwgt.offset(m as isize);
                        *fresh25 += *adjwgt.offset(j as isize);
                    } else {
                        jj = 0 as libc::c_int;
                        while jj < nedges {
                            if *cadjncy.offset(jj as isize) == k {
                                let ref mut fresh26 = *cadjwgt.offset(jj as isize);
                                *fresh26 += *adjwgt.offset(j as isize);
                                break;
                            } else {
                                jj += 1;
                                jj;
                            }
                        }
                        if jj == nedges {
                            *cadjncy.offset(nedges as isize) = k;
                            let fresh27 = nedges;
                            nedges = nedges + 1;
                            *cadjwgt.offset(fresh27 as isize) = *adjwgt.offset(j as isize);
                        }
                    }
                    j += 1;
                    j;
                }
                jj = *htable.offset((cnvtxs & mask) as isize);
                if jj >= 0 as libc::c_int && *cadjncy.offset(jj as isize) != cnvtxs {
                    jj = 0 as libc::c_int;
                    while jj < nedges {
                        if *cadjncy.offset(jj as isize) == cnvtxs {
                            break;
                        }
                        jj += 1;
                        jj;
                    }
                }
                if jj >= 0 as libc::c_int && jj < nedges && *cadjncy.offset(jj as isize) == cnvtxs {
                    nedges -= 1;
                    *cadjncy.offset(jj as isize) = *cadjncy.offset(nedges as isize);
                    *cadjwgt.offset(jj as isize) = *cadjwgt.offset(nedges as isize);
                }
            }
            j = 0 as libc::c_int;
            while j < nedges {
                *htable.offset((*cadjncy.offset(j as isize) & mask) as isize) = -(1);
                j += 1;
                j;
            }
            *htable.offset((cnvtxs & mask) as isize) = -(1);
            cnedges += nedges;
            cnvtxs += 1;
            cxadj[(cnvtxs as usize)] = cnedges;
            cadjncy = cadjncy.offset(nedges as isize);
            cadjwgt = cadjwgt.offset(nedges as isize);
        }
        v += 1;
        v;
    }
    (*cgraph).nedges = cnedges;
    j = 0 as libc::c_int;
    while j < ncon {
        *((*cgraph).tvwgt).offset(j as isize) = libmetis__isum(
            (*cgraph).nvtxs as size_t,
            ((*cgraph).vwgt).offset(j as isize),
            ncon as size_t,
        );
        *((*cgraph).invtvwgt).offset(j as isize) = (1.0f64
            / (if *((*cgraph).tvwgt).offset(j as isize) > 0 as libc::c_int {
                *((*cgraph).tvwgt).offset(j as isize)
            } else {
                1
            }) as libc::c_double) as real_t;
        j += 1;
        j;
    }
    libmetis__ReAdjustMemory(ctrl, graph, cgraph);
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint != 0 {
        (*ctrl).ContractTmr += gk_CPUSeconds();
    }
    libmetis__wspacepop(ctrl);
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__CreateCoarseGraphNoMask(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut cnvtxs: idx_t,
    mut match_0: *mut idx_t,
) {
    let mut j: idx_t = 0;
    let mut k: idx_t = 0;
    let mut m: idx_t = 0;
    let mut istart: idx_t = 0;
    let mut iend: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut nedges: idx_t = 0;
    let mut ncon: idx_t = 0;
    let mut cnedges: idx_t = 0;
    let mut v: idx_t = 0;
    let mut u: idx_t = 0;
    let mut dovsize: idx_t = 0;

    let mut vwgt: *mut idx_t = 0 as *mut idx_t;
    let mut vsize: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut adjwgt: *mut idx_t = 0 as *mut idx_t;
    let mut cmap: *mut idx_t = 0 as *mut idx_t;
    let mut htable: *mut idx_t = 0 as *mut idx_t;
    let mut cxadj;
    let mut cvwgt: *mut idx_t = 0 as *mut idx_t;
    let mut cvsize: *mut idx_t = 0 as *mut idx_t;
    let mut cadjncy: *mut idx_t = 0 as *mut idx_t;
    let mut cadjwgt: *mut idx_t = 0 as *mut idx_t;
    let mut cgraph: *mut graph_t = 0 as *mut graph_t;
    libmetis__wspacepush(ctrl);
    dovsize = if (*ctrl).objtype as libc::c_uint == METIS_OBJTYPE_VOL as libc::c_int as libc::c_uint
    {
        1
    } else {
        0 as libc::c_int
    };
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint != 0 {
        (*ctrl).ContractTmr -= gk_CPUSeconds();
    }
    nvtxs = (*graph).nvtxs;
    ncon = (*graph).ncon;
    let mut xadj = &mut (*graph).xadj;
    vwgt = (*graph).vwgt;
    vsize = (*graph).vsize;
    adjncy = (*graph).adjncy;
    adjwgt = (*graph).adjwgt;
    cmap = (*graph).cmap;
    cgraph = libmetis__SetupCoarseGraph(graph, cnvtxs, dovsize);
    cxadj = (*cgraph).xadj.clone();
    cvwgt = (*cgraph).vwgt;
    cvsize = (*cgraph).vsize;
    cadjncy = (*cgraph).adjncy;
    cadjwgt = (*cgraph).adjwgt;
    htable = libmetis__iset(
        cnvtxs as size_t,
        -(1),
        libmetis__iwspacemalloc(ctrl, cnvtxs),
    );
    cnedges = 0 as libc::c_int;
    cnvtxs = cnedges;
    cxadj[0] = cnvtxs;
    v = 0 as libc::c_int;
    while v < nvtxs {
        u = *match_0.offset(v as isize);
        if !(u < v) {
            if ncon == 1 {
                *cvwgt.offset(cnvtxs as isize) = *vwgt.offset(v as isize);
            } else {
                libmetis__icopy(
                    ncon as size_t,
                    vwgt.offset((v * ncon) as isize),
                    cvwgt.offset((cnvtxs * ncon) as isize),
                );
            }
            if dovsize != 0 {
                *cvsize.offset(cnvtxs as isize) = *vsize.offset(v as isize);
            }
            nedges = 0 as libc::c_int;
            istart = xadj[(v as usize)];
            iend = xadj[((v + 1) as usize)];
            j = istart;
            while j < iend {
                k = *cmap.offset(*adjncy.offset(j as isize) as isize);
                m = *htable.offset(k as isize);
                if m == -(1) {
                    *cadjncy.offset(nedges as isize) = k;
                    *cadjwgt.offset(nedges as isize) = *adjwgt.offset(j as isize);
                    let fresh28 = nedges;
                    nedges = nedges + 1;
                    *htable.offset(k as isize) = fresh28;
                } else {
                    let ref mut fresh29 = *cadjwgt.offset(m as isize);
                    *fresh29 += *adjwgt.offset(j as isize);
                }
                j += 1;
                j;
            }
            if v != u {
                if ncon == 1 {
                    let ref mut fresh30 = *cvwgt.offset(cnvtxs as isize);
                    *fresh30 += *vwgt.offset(u as isize);
                } else {
                    libmetis__iaxpy(
                        ncon as size_t,
                        1,
                        vwgt.offset((u * ncon) as isize),
                        1 as size_t,
                        cvwgt.offset((cnvtxs * ncon) as isize),
                        1 as size_t,
                    );
                }
                if dovsize != 0 {
                    let ref mut fresh31 = *cvsize.offset(cnvtxs as isize);
                    *fresh31 += *vsize.offset(u as isize);
                }
                istart = xadj[(u as usize)];
                iend = xadj[(u + 1) as usize];
                j = istart;
                while j < iend {
                    k = *cmap.offset(*adjncy.offset(j as isize) as isize);
                    m = *htable.offset(k as isize);
                    if m == -(1) {
                        *cadjncy.offset(nedges as isize) = k;
                        *cadjwgt.offset(nedges as isize) = *adjwgt.offset(j as isize);
                        let fresh32 = nedges;
                        nedges = nedges + 1;
                        *htable.offset(k as isize) = fresh32;
                    } else {
                        let ref mut fresh33 = *cadjwgt.offset(m as isize);
                        *fresh33 += *adjwgt.offset(j as isize);
                    }
                    j += 1;
                    j;
                }
                j = *htable.offset(cnvtxs as isize);
                if j != -(1) {
                    nedges -= 1;
                    *cadjncy.offset(j as isize) = *cadjncy.offset(nedges as isize);
                    *cadjwgt.offset(j as isize) = *cadjwgt.offset(nedges as isize);
                    *htable.offset(cnvtxs as isize) = -(1);
                }
            }
            j = 0 as libc::c_int;
            while j < nedges {
                *htable.offset(*cadjncy.offset(j as isize) as isize) = -(1);
                j += 1;
                j;
            }
            cnedges += nedges;
            cnvtxs += 1;
            cxadj[(cnvtxs as usize)] = cnedges;
            cadjncy = cadjncy.offset(nedges as isize);
            cadjwgt = cadjwgt.offset(nedges as isize);
        }
        v += 1;
        v;
    }
    (*cgraph).nedges = cnedges;
    j = 0 as libc::c_int;
    while j < ncon {
        *((*cgraph).tvwgt).offset(j as isize) = libmetis__isum(
            (*cgraph).nvtxs as size_t,
            ((*cgraph).vwgt).offset(j as isize),
            ncon as size_t,
        );
        *((*cgraph).invtvwgt).offset(j as isize) = (1.0f64
            / (if *((*cgraph).tvwgt).offset(j as isize) > 0 as libc::c_int {
                *((*cgraph).tvwgt).offset(j as isize)
            } else {
                1
            }) as libc::c_double) as real_t;
        j += 1;
        j;
    }
    libmetis__ReAdjustMemory(ctrl, graph, cgraph);
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint != 0 {
        (*ctrl).ContractTmr += gk_CPUSeconds();
    }
    libmetis__wspacepop(ctrl);
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__CreateCoarseGraphPerm(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut cnvtxs: idx_t,
    mut match_0: *mut idx_t,
    mut perm: *mut idx_t,
) {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut jj: idx_t = 0;
    let mut k: idx_t = 0;
    let mut kk: idx_t = 0;
    let mut l: idx_t = 0;
    let mut m: idx_t = 0;
    let mut istart: idx_t = 0;
    let mut iend: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut nedges: idx_t = 0;
    let mut ncon: idx_t = 0;
    let mut cnedges: idx_t = 0;
    let mut v: idx_t = 0;
    let mut u: idx_t = 0;
    let mut mask: idx_t = 0;
    let mut dovsize: idx_t = 0;

    let mut vwgt: *mut idx_t = 0 as *mut idx_t;
    let mut vsize: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut adjwgt: *mut idx_t = 0 as *mut idx_t;
    let mut cmap: *mut idx_t = 0 as *mut idx_t;
    let mut htable: *mut idx_t = 0 as *mut idx_t;
    let mut cxadj;
    let mut cvwgt: *mut idx_t = 0 as *mut idx_t;
    let mut cvsize: *mut idx_t = 0 as *mut idx_t;
    let mut cadjncy: *mut idx_t = 0 as *mut idx_t;
    let mut cadjwgt: *mut idx_t = 0 as *mut idx_t;
    let mut cgraph: *mut graph_t = 0 as *mut graph_t;
    libmetis__wspacepush(ctrl);
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint != 0 {
        (*ctrl).ContractTmr -= gk_CPUSeconds();
    }
    dovsize = if (*ctrl).objtype as libc::c_uint == METIS_OBJTYPE_VOL as libc::c_int as libc::c_uint
    {
        1
    } else {
        0 as libc::c_int
    };
    mask = ((1) << 11) - 1;
    nvtxs = (*graph).nvtxs;
    ncon = (*graph).ncon;
    let mut xadj = &mut (*graph).xadj;
    vwgt = (*graph).vwgt;
    vsize = (*graph).vsize;
    adjncy = (*graph).adjncy;
    adjwgt = (*graph).adjwgt;
    cmap = (*graph).cmap;
    cgraph = libmetis__SetupCoarseGraph(graph, cnvtxs, dovsize);
    cxadj = (*cgraph).xadj.clone();
    cvwgt = (*cgraph).vwgt;
    cvsize = (*cgraph).vsize;
    cadjncy = (*cgraph).adjncy;
    cadjwgt = (*cgraph).adjwgt;
    htable = libmetis__iset(
        (mask + 1) as size_t,
        -(1),
        libmetis__iwspacemalloc(ctrl, mask + 1),
    );
    cnedges = 0 as libc::c_int;
    cnvtxs = cnedges;
    cxadj[0] = cnvtxs;
    i = 0 as libc::c_int;
    while i < nvtxs {
        v = *perm.offset(i as isize);
        if !(*cmap.offset(v as isize) != cnvtxs) {
            u = *match_0.offset(v as isize);
            if ncon == 1 {
                *cvwgt.offset(cnvtxs as isize) = *vwgt.offset(v as isize);
            } else {
                libmetis__icopy(
                    ncon as size_t,
                    vwgt.offset((v * ncon) as isize),
                    cvwgt.offset((cnvtxs * ncon) as isize),
                );
            }
            if dovsize != 0 {
                *cvsize.offset(cnvtxs as isize) = *vsize.offset(v as isize);
            }
            nedges = 0 as libc::c_int;
            istart = xadj[(v as usize)];
            iend = xadj[((v + 1) as usize)];
            j = istart;
            while j < iend {
                k = *cmap.offset(*adjncy.offset(j as isize) as isize);
                kk = k & mask;
                m = *htable.offset(kk as isize);
                if m == -(1) {
                    *cadjncy.offset(nedges as isize) = k;
                    *cadjwgt.offset(nedges as isize) = *adjwgt.offset(j as isize);
                    let fresh34 = nedges;
                    nedges = nedges + 1;
                    *htable.offset(kk as isize) = fresh34;
                } else if *cadjncy.offset(m as isize) == k {
                    let ref mut fresh35 = *cadjwgt.offset(m as isize);
                    *fresh35 += *adjwgt.offset(j as isize);
                } else {
                    jj = 0 as libc::c_int;
                    while jj < nedges {
                        if *cadjncy.offset(jj as isize) == k {
                            let ref mut fresh36 = *cadjwgt.offset(jj as isize);
                            *fresh36 += *adjwgt.offset(j as isize);
                            break;
                        } else {
                            jj += 1;
                            jj;
                        }
                    }
                    if jj == nedges {
                        *cadjncy.offset(nedges as isize) = k;
                        let fresh37 = nedges;
                        nedges = nedges + 1;
                        *cadjwgt.offset(fresh37 as isize) = *adjwgt.offset(j as isize);
                    }
                }
                j += 1;
                j;
            }
            if v != u {
                if ncon == 1 {
                    let ref mut fresh38 = *cvwgt.offset(cnvtxs as isize);
                    *fresh38 += *vwgt.offset(u as isize);
                } else {
                    libmetis__iaxpy(
                        ncon as size_t,
                        1,
                        vwgt.offset((u * ncon) as isize),
                        1 as size_t,
                        cvwgt.offset((cnvtxs * ncon) as isize),
                        1 as size_t,
                    );
                }
                if dovsize != 0 {
                    let ref mut fresh39 = *cvsize.offset(cnvtxs as isize);
                    *fresh39 += *vsize.offset(u as isize);
                }
                istart = xadj[(u as usize)];
                iend = xadj[(u + 1) as usize];
                j = istart;
                while j < iend {
                    k = *cmap.offset(*adjncy.offset(j as isize) as isize);
                    kk = k & mask;
                    m = *htable.offset(kk as isize);
                    if m == -(1) {
                        *cadjncy.offset(nedges as isize) = k;
                        *cadjwgt.offset(nedges as isize) = *adjwgt.offset(j as isize);
                        let fresh40 = nedges;
                        nedges = nedges + 1;
                        *htable.offset(kk as isize) = fresh40;
                    } else if *cadjncy.offset(m as isize) == k {
                        let ref mut fresh41 = *cadjwgt.offset(m as isize);
                        *fresh41 += *adjwgt.offset(j as isize);
                    } else {
                        jj = 0 as libc::c_int;
                        while jj < nedges {
                            if *cadjncy.offset(jj as isize) == k {
                                let ref mut fresh42 = *cadjwgt.offset(jj as isize);
                                *fresh42 += *adjwgt.offset(j as isize);
                                break;
                            } else {
                                jj += 1;
                                jj;
                            }
                        }
                        if jj == nedges {
                            *cadjncy.offset(nedges as isize) = k;
                            let fresh43 = nedges;
                            nedges = nedges + 1;
                            *cadjwgt.offset(fresh43 as isize) = *adjwgt.offset(j as isize);
                        }
                    }
                    j += 1;
                    j;
                }
                jj = *htable.offset((cnvtxs & mask) as isize);
                if jj >= 0 as libc::c_int && *cadjncy.offset(jj as isize) != cnvtxs {
                    jj = 0 as libc::c_int;
                    while jj < nedges {
                        if *cadjncy.offset(jj as isize) == cnvtxs {
                            break;
                        }
                        jj += 1;
                        jj;
                    }
                }
                if jj >= 0 as libc::c_int && *cadjncy.offset(jj as isize) == cnvtxs {
                    nedges -= 1;
                    *cadjncy.offset(jj as isize) = *cadjncy.offset(nedges as isize);
                    *cadjwgt.offset(jj as isize) = *cadjwgt.offset(nedges as isize);
                }
            }
            j = 0 as libc::c_int;
            while j < nedges {
                *htable.offset((*cadjncy.offset(j as isize) & mask) as isize) = -(1);
                j += 1;
                j;
            }
            *htable.offset((cnvtxs & mask) as isize) = -(1);
            cnedges += nedges;
            cnvtxs += 1;
            cxadj[(cnvtxs as usize)] = cnedges;
            cadjncy = cadjncy.offset(nedges as isize);
            cadjwgt = cadjwgt.offset(nedges as isize);
        }
        i += 1;
        i;
    }
    (*cgraph).nedges = cnedges;
    i = 0 as libc::c_int;
    while i < ncon {
        *((*cgraph).tvwgt).offset(i as isize) = libmetis__isum(
            (*cgraph).nvtxs as size_t,
            ((*cgraph).vwgt).offset(i as isize),
            ncon as size_t,
        );
        *((*cgraph).invtvwgt).offset(i as isize) = (1.0f64
            / (if *((*cgraph).tvwgt).offset(i as isize) > 0 as libc::c_int {
                *((*cgraph).tvwgt).offset(i as isize)
            } else {
                1
            }) as libc::c_double) as real_t;
        i += 1;
        i;
    }
    libmetis__ReAdjustMemory(ctrl, graph, cgraph);
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint != 0 {
        (*ctrl).ContractTmr += gk_CPUSeconds();
    }
    libmetis__wspacepop(ctrl);
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__SetupCoarseGraph(
    mut graph: *mut graph_t,
    mut cnvtxs: idx_t,
    mut dovsize: idx_t,
) -> *mut graph_t {
    let mut cgraph: *mut graph_t = 0 as *mut graph_t;
    cgraph = libmetis__CreateGraph();
    (*cgraph).nvtxs = cnvtxs;
    (*cgraph).ncon = (*graph).ncon;
    (*cgraph).finer = graph;
    (*graph).coarser = cgraph;
    (*cgraph).xadj = vec![0; (cnvtxs + 1) as usize];

    (*cgraph).adjncy = libmetis__imalloc(
        (*graph).nedges as size_t,
        b"SetupCoarseGraph: adjncy\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (*cgraph).adjwgt = libmetis__imalloc(
        (*graph).nedges as size_t,
        b"SetupCoarseGraph: adjwgt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (*cgraph).vwgt = libmetis__imalloc(
        ((*cgraph).ncon * cnvtxs) as size_t,
        b"SetupCoarseGraph: vwgt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (*cgraph).tvwgt = libmetis__imalloc(
        (*cgraph).ncon as size_t,
        b"SetupCoarseGraph: tvwgt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (*cgraph).invtvwgt = libmetis__rmalloc(
        (*cgraph).ncon as size_t,
        b"SetupCoarseGraph: invtvwgt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if dovsize != 0 {
        (*cgraph).vsize = libmetis__imalloc(
            cnvtxs as size_t,
            b"SetupCoarseGraph: vsize\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    return cgraph;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__ReAdjustMemory(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut cgraph: *mut graph_t,
) {
    if (*cgraph).nedges > 10000 as libc::c_int
        && ((*cgraph).nedges as libc::c_double) < 0.9f64 * (*graph).nedges as libc::c_double
    {
        (*cgraph).adjncy = libmetis__irealloc(
            (*cgraph).adjncy,
            (*cgraph).nedges as size_t,
            b"ReAdjustMemory: adjncy\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        (*cgraph).adjwgt = libmetis__irealloc(
            (*cgraph).adjwgt,
            (*cgraph).nedges as size_t,
            b"ReAdjustMemory: adjwgt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
}
