use super::{
    auxapi::*,
    balance::*,
    coarsen::*,
    contig::*,
    fortran::*,
    gklib::*,
    graph::*,
    kwayrefine::*,
    mcutil::{
        libmetis__BetterBalance2Way, libmetis__ComputeLoadImbalance,
        libmetis__ComputeLoadImbalanceDiffVec,
    },
    options::*,
    structure::*,
    timing::*,
    util::*,
    wspace::*,
};
use ::libc;
use libc::{abs, printf};

#[no_mangle]
pub unsafe extern "C" fn libmetis__FM_2WayRefine(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut ntpwgts: *mut real_t,
    mut niter: idx_t,
) {
    if (*graph).ncon == 1 as libc::c_int {
        libmetis__FM_2WayCutRefine(ctrl, graph, ntpwgts, niter);
    } else {
        libmetis__FM_Mc2WayCutRefine(ctrl, graph, ntpwgts, niter);
    };
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__FM_2WayCutRefine(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut ntpwgts: *mut real_t,
    mut niter: idx_t,
) {
    let mut i: idx_t = 0;
    let mut ii: idx_t = 0;
    let mut j: idx_t = 0;
    let mut k: idx_t = 0;
    let mut kwgt: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut nbnd: idx_t = 0;
    let mut nswaps: idx_t = 0;
    let mut from: idx_t = 0;
    let mut to: idx_t = 0;
    let mut pass: idx_t = 0;
    let mut me: idx_t = 0;
    let mut limit: idx_t = 0;
    let mut tmp: idx_t = 0;
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
    let mut vwgt: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut adjwgt: *mut idx_t = 0 as *mut idx_t;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    let mut id: *mut idx_t = 0 as *mut idx_t;
    let mut ed: *mut idx_t = 0 as *mut idx_t;
    let mut bndptr: *mut idx_t = 0 as *mut idx_t;
    let mut bndind: *mut idx_t = 0 as *mut idx_t;
    let mut pwgts: *mut idx_t = 0 as *mut idx_t;
    let mut moved: *mut idx_t = 0 as *mut idx_t;
    let mut swaps: *mut idx_t = 0 as *mut idx_t;
    let mut perm: *mut idx_t = 0 as *mut idx_t;
    let mut queues: [*mut rpq_t; 2] = [0 as *mut rpq_t; 2];
    let mut higain: idx_t = 0;
    let mut mincut: idx_t = 0;
    let mut mindiff: idx_t = 0;
    let mut origdiff: idx_t = 0;
    let mut initcut: idx_t = 0;
    let mut newcut: idx_t = 0;
    let mut mincutorder: idx_t = 0;
    let mut avgvwgt: idx_t = 0;
    let mut tpwgts: [idx_t; 2] = [0; 2];
    libmetis__wspacepush(ctrl);
    nvtxs = (*graph).nvtxs;
    xadj = (*graph).xadj;
    vwgt = (*graph).vwgt;
    adjncy = (*graph).adjncy;
    adjwgt = (*graph).adjwgt;
    where_0 = (*graph).where_0;
    id = (*graph).id;
    ed = (*graph).ed;
    pwgts = (*graph).pwgts;
    bndptr = (*graph).bndptr;
    bndind = (*graph).bndind;
    moved = libmetis__iwspacemalloc(ctrl, nvtxs);
    swaps = libmetis__iwspacemalloc(ctrl, nvtxs);
    perm = libmetis__iwspacemalloc(ctrl, nvtxs);
    tpwgts[0 as libc::c_int as usize] =
        (*((*graph).tvwgt).offset(0 as libc::c_int as isize) as libc::c_float
            * *ntpwgts.offset(0 as libc::c_int as isize)) as idx_t;
    tpwgts[1 as libc::c_int as usize] =
        *((*graph).tvwgt).offset(0 as libc::c_int as isize) - tpwgts[0 as libc::c_int as usize];
    limit = (if (if 0.01f64 * nvtxs as libc::c_double >= 15 as libc::c_int as libc::c_double {
        0.01f64 * nvtxs as libc::c_double
    } else {
        15 as libc::c_int as libc::c_double
    }) >= 100 as libc::c_int as libc::c_double
    {
        100 as libc::c_int as libc::c_double
    } else if 0.01f64 * nvtxs as libc::c_double >= 15 as libc::c_int as libc::c_double {
        0.01f64 * nvtxs as libc::c_double
    } else {
        15 as libc::c_int as libc::c_double
    }) as idx_t;
    avgvwgt = if (*pwgts.offset(0 as libc::c_int as isize)
        + *pwgts.offset(1 as libc::c_int as isize))
        / 20 as libc::c_int
        >= 2 as libc::c_int
            * (*pwgts.offset(0 as libc::c_int as isize) + *pwgts.offset(1 as libc::c_int as isize))
            / nvtxs
    {
        2 as libc::c_int
            * (*pwgts.offset(0 as libc::c_int as isize) + *pwgts.offset(1 as libc::c_int as isize))
            / nvtxs
    } else {
        (*pwgts.offset(0 as libc::c_int as isize) + *pwgts.offset(1 as libc::c_int as isize))
            / 20 as libc::c_int
    };
    queues[0 as libc::c_int as usize] = libmetis__rpqCreate(nvtxs as size_t);
    queues[1 as libc::c_int as usize] = libmetis__rpqCreate(nvtxs as size_t);
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_REFINE as libc::c_int as libc::c_uint != 0 {
        libmetis__Print2WayRefineStats(
            ctrl,
            graph,
            ntpwgts,
            0 as libc::c_int as real_t,
            -(2 as libc::c_int),
        );
    }
    origdiff = abs(tpwgts[0 as libc::c_int as usize] - *pwgts.offset(0 as libc::c_int as isize));
    libmetis__iset(nvtxs as size_t, -(1 as libc::c_int), moved);
    pass = 0 as libc::c_int;
    while pass < niter {
        libmetis__rpqReset(queues[0 as libc::c_int as usize]);
        libmetis__rpqReset(queues[1 as libc::c_int as usize]);
        mincutorder = -(1 as libc::c_int);
        initcut = (*graph).mincut;
        mincut = initcut;
        newcut = mincut;
        mindiff = abs(tpwgts[0 as libc::c_int as usize] - *pwgts.offset(0 as libc::c_int as isize));
        nbnd = (*graph).nbnd;
        libmetis__irandArrayPermute(nbnd, perm, nbnd, 1 as libc::c_int);
        ii = 0 as libc::c_int;
        while ii < nbnd {
            i = *perm.offset(ii as isize);
            libmetis__rpqInsert(
                queues[*where_0.offset(*bndind.offset(i as isize) as isize) as usize],
                *bndind.offset(i as isize),
                (*ed.offset(*bndind.offset(i as isize) as isize)
                    - *id.offset(*bndind.offset(i as isize) as isize)) as real_t,
            );
            ii += 1;
            ii;
        }
        nswaps = 0 as libc::c_int;
        while nswaps < nvtxs {
            from = if tpwgts[0 as libc::c_int as usize] - *pwgts.offset(0 as libc::c_int as isize)
                < tpwgts[1 as libc::c_int as usize] - *pwgts.offset(1 as libc::c_int as isize)
            {
                0 as libc::c_int
            } else {
                1 as libc::c_int
            };
            to = (from + 1 as libc::c_int) % 2 as libc::c_int;
            higain = libmetis__rpqGetTop(queues[from as usize]);
            if higain == -(1 as libc::c_int) {
                break;
            }
            newcut -= *ed.offset(higain as isize) - *id.offset(higain as isize);
            let ref mut fresh0 = *pwgts.offset(to as isize);
            *fresh0 += *vwgt.offset(higain as isize);
            let ref mut fresh1 = *pwgts.offset(from as isize);
            *fresh1 -= *vwgt.offset(higain as isize);
            if newcut < mincut
                && abs(tpwgts[0 as libc::c_int as usize] - *pwgts.offset(0 as libc::c_int as isize))
                    <= origdiff + avgvwgt
                || newcut == mincut
                    && abs(tpwgts[0 as libc::c_int as usize]
                        - *pwgts.offset(0 as libc::c_int as isize))
                        < mindiff
            {
                mincut = newcut;
                mindiff =
                    abs(tpwgts[0 as libc::c_int as usize]
                        - *pwgts.offset(0 as libc::c_int as isize));
                mincutorder = nswaps;
            } else if nswaps - mincutorder > limit {
                newcut += *ed.offset(higain as isize) - *id.offset(higain as isize);
                let ref mut fresh2 = *pwgts.offset(from as isize);
                *fresh2 += *vwgt.offset(higain as isize);
                let ref mut fresh3 = *pwgts.offset(to as isize);
                *fresh3 -= *vwgt.offset(higain as isize);
                break;
            }
            *where_0.offset(higain as isize) = to;
            *moved.offset(higain as isize) = nswaps;
            *swaps.offset(nswaps as isize) = higain;
            if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_MOVEINFO as libc::c_int as libc::c_uint
                != 0
            {
                printf(
                    b"Moved %6d from %d. [%3d %3d] %5d [%4d %4d]\n\0" as *const u8
                        as *const libc::c_char,
                    higain,
                    from,
                    *ed.offset(higain as isize) - *id.offset(higain as isize),
                    *vwgt.offset(higain as isize),
                    newcut,
                    *pwgts.offset(0 as libc::c_int as isize),
                    *pwgts.offset(1 as libc::c_int as isize),
                );
            }
            tmp = *id.offset(higain as isize);
            *id.offset(higain as isize) = *ed.offset(higain as isize);
            *ed.offset(higain as isize) = tmp;
            if *ed.offset(higain as isize) == 0 as libc::c_int
                && *xadj.offset(higain as isize)
                    < *xadj.offset((higain + 1 as libc::c_int) as isize)
            {
                nbnd -= 1;
                *bndind.offset(*bndptr.offset(higain as isize) as isize) =
                    *bndind.offset(nbnd as isize);
                *bndptr.offset(*bndind.offset(nbnd as isize) as isize) =
                    *bndptr.offset(higain as isize);
                *bndptr.offset(higain as isize) = -(1 as libc::c_int);
            }
            j = *xadj.offset(higain as isize);
            while j < *xadj.offset((higain + 1 as libc::c_int) as isize) {
                k = *adjncy.offset(j as isize);
                kwgt = if to == *where_0.offset(k as isize) {
                    *adjwgt.offset(j as isize)
                } else {
                    -*adjwgt.offset(j as isize)
                };
                let ref mut fresh4 = *id.offset(k as isize);
                *fresh4 += kwgt;
                let ref mut fresh5 = *ed.offset(k as isize);
                *fresh5 -= kwgt;
                if *bndptr.offset(k as isize) != -(1 as libc::c_int) {
                    if *ed.offset(k as isize) == 0 as libc::c_int {
                        nbnd -= 1;
                        *bndind.offset(*bndptr.offset(k as isize) as isize) =
                            *bndind.offset(nbnd as isize);
                        *bndptr.offset(*bndind.offset(nbnd as isize) as isize) =
                            *bndptr.offset(k as isize);
                        *bndptr.offset(k as isize) = -(1 as libc::c_int);
                        if *moved.offset(k as isize) == -(1 as libc::c_int) {
                            libmetis__rpqDelete(queues[*where_0.offset(k as isize) as usize], k);
                        }
                    } else if *moved.offset(k as isize) == -(1 as libc::c_int) {
                        libmetis__rpqUpdate(
                            queues[*where_0.offset(k as isize) as usize],
                            k,
                            (*ed.offset(k as isize) - *id.offset(k as isize)) as real_t,
                        );
                    }
                } else if *ed.offset(k as isize) > 0 as libc::c_int {
                    *bndind.offset(nbnd as isize) = k;
                    let fresh6 = nbnd;
                    nbnd = nbnd + 1;
                    *bndptr.offset(k as isize) = fresh6;
                    if *moved.offset(k as isize) == -(1 as libc::c_int) {
                        libmetis__rpqInsert(
                            queues[*where_0.offset(k as isize) as usize],
                            k,
                            (*ed.offset(k as isize) - *id.offset(k as isize)) as real_t,
                        );
                    }
                }
                j += 1;
                j;
            }
            nswaps += 1;
            nswaps;
        }
        i = 0 as libc::c_int;
        while i < nswaps {
            *moved.offset(*swaps.offset(i as isize) as isize) = -(1 as libc::c_int);
            i += 1;
            i;
        }
        nswaps -= 1;
        nswaps;
        while nswaps > mincutorder {
            higain = *swaps.offset(nswaps as isize);
            let ref mut fresh7 = *where_0.offset(higain as isize);
            *fresh7 = (*where_0.offset(higain as isize) + 1 as libc::c_int) % 2 as libc::c_int;
            to = *fresh7;
            tmp = *id.offset(higain as isize);
            *id.offset(higain as isize) = *ed.offset(higain as isize);
            *ed.offset(higain as isize) = tmp;
            if *ed.offset(higain as isize) == 0 as libc::c_int
                && *bndptr.offset(higain as isize) != -(1 as libc::c_int)
                && *xadj.offset(higain as isize)
                    < *xadj.offset((higain + 1 as libc::c_int) as isize)
            {
                nbnd -= 1;
                *bndind.offset(*bndptr.offset(higain as isize) as isize) =
                    *bndind.offset(nbnd as isize);
                *bndptr.offset(*bndind.offset(nbnd as isize) as isize) =
                    *bndptr.offset(higain as isize);
                *bndptr.offset(higain as isize) = -(1 as libc::c_int);
            } else if *ed.offset(higain as isize) > 0 as libc::c_int
                && *bndptr.offset(higain as isize) == -(1 as libc::c_int)
            {
                *bndind.offset(nbnd as isize) = higain;
                let fresh8 = nbnd;
                nbnd = nbnd + 1;
                *bndptr.offset(higain as isize) = fresh8;
            }
            let ref mut fresh9 = *pwgts.offset(to as isize);
            *fresh9 += *vwgt.offset(higain as isize);
            let ref mut fresh10 =
                *pwgts.offset(((to + 1 as libc::c_int) % 2 as libc::c_int) as isize);
            *fresh10 -= *vwgt.offset(higain as isize);
            j = *xadj.offset(higain as isize);
            while j < *xadj.offset((higain + 1 as libc::c_int) as isize) {
                k = *adjncy.offset(j as isize);
                kwgt = if to == *where_0.offset(k as isize) {
                    *adjwgt.offset(j as isize)
                } else {
                    -*adjwgt.offset(j as isize)
                };
                let ref mut fresh11 = *id.offset(k as isize);
                *fresh11 += kwgt;
                let ref mut fresh12 = *ed.offset(k as isize);
                *fresh12 -= kwgt;
                if *bndptr.offset(k as isize) != -(1 as libc::c_int)
                    && *ed.offset(k as isize) == 0 as libc::c_int
                {
                    nbnd -= 1;
                    *bndind.offset(*bndptr.offset(k as isize) as isize) =
                        *bndind.offset(nbnd as isize);
                    *bndptr.offset(*bndind.offset(nbnd as isize) as isize) =
                        *bndptr.offset(k as isize);
                    *bndptr.offset(k as isize) = -(1 as libc::c_int);
                }
                if *bndptr.offset(k as isize) == -(1 as libc::c_int)
                    && *ed.offset(k as isize) > 0 as libc::c_int
                {
                    *bndind.offset(nbnd as isize) = k;
                    let fresh13 = nbnd;
                    nbnd = nbnd + 1;
                    *bndptr.offset(k as isize) = fresh13;
                }
                j += 1;
                j;
            }
            nswaps -= 1;
            nswaps;
        }
        (*graph).mincut = mincut;
        (*graph).nbnd = nbnd;
        if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_REFINE as libc::c_int as libc::c_uint != 0 {
            libmetis__Print2WayRefineStats(
                ctrl,
                graph,
                ntpwgts,
                0 as libc::c_int as real_t,
                mincutorder,
            );
        }
        if mincutorder <= 0 as libc::c_int || mincut == initcut {
            break;
        }
        pass += 1;
        pass;
    }
    libmetis__rpqDestroy(queues[0 as libc::c_int as usize]);
    libmetis__rpqDestroy(queues[1 as libc::c_int as usize]);
    libmetis__wspacepop(ctrl);
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__FM_Mc2WayCutRefine(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut ntpwgts: *mut real_t,
    mut niter: idx_t,
) {
    let mut i: idx_t = 0;
    let mut ii: idx_t = 0;
    let mut j: idx_t = 0;
    let mut k: idx_t = 0;
    let mut l: idx_t = 0;
    let mut kwgt: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut ncon: idx_t = 0;
    let mut nbnd: idx_t = 0;
    let mut nswaps: idx_t = 0;
    let mut from: idx_t = 0;
    let mut to: idx_t = 0;
    let mut pass: idx_t = 0;
    let mut me: idx_t = 0;
    let mut limit: idx_t = 0;
    let mut tmp: idx_t = 0;
    let mut cnum: idx_t = 0;
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut vwgt: *mut idx_t = 0 as *mut idx_t;
    let mut adjwgt: *mut idx_t = 0 as *mut idx_t;
    let mut pwgts: *mut idx_t = 0 as *mut idx_t;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    let mut id: *mut idx_t = 0 as *mut idx_t;
    let mut ed: *mut idx_t = 0 as *mut idx_t;
    let mut bndptr: *mut idx_t = 0 as *mut idx_t;
    let mut bndind: *mut idx_t = 0 as *mut idx_t;
    let mut moved: *mut idx_t = 0 as *mut idx_t;
    let mut swaps: *mut idx_t = 0 as *mut idx_t;
    let mut perm: *mut idx_t = 0 as *mut idx_t;
    let mut qnum: *mut idx_t = 0 as *mut idx_t;
    let mut higain: idx_t = 0;
    let mut mincut: idx_t = 0;
    let mut initcut: idx_t = 0;
    let mut newcut: idx_t = 0;
    let mut mincutorder: idx_t = 0;
    let mut invtvwgt: *mut real_t = 0 as *mut real_t;
    let mut ubfactors: *mut real_t = 0 as *mut real_t;
    let mut minbalv: *mut real_t = 0 as *mut real_t;
    let mut newbalv: *mut real_t = 0 as *mut real_t;
    let mut origbal: real_t = 0.;
    let mut minbal: real_t = 0.;
    let mut newbal: real_t = 0.;
    let mut rgain: real_t = 0.;
    let mut ffactor: real_t = 0.;
    let mut queues: *mut *mut rpq_t = 0 as *mut *mut rpq_t;
    libmetis__wspacepush(ctrl);
    nvtxs = (*graph).nvtxs;
    ncon = (*graph).ncon;
    xadj = (*graph).xadj;
    vwgt = (*graph).vwgt;
    adjncy = (*graph).adjncy;
    adjwgt = (*graph).adjwgt;
    invtvwgt = (*graph).invtvwgt;
    where_0 = (*graph).where_0;
    id = (*graph).id;
    ed = (*graph).ed;
    pwgts = (*graph).pwgts;
    bndptr = (*graph).bndptr;
    bndind = (*graph).bndind;
    moved = libmetis__iwspacemalloc(ctrl, nvtxs);
    swaps = libmetis__iwspacemalloc(ctrl, nvtxs);
    perm = libmetis__iwspacemalloc(ctrl, nvtxs);
    qnum = libmetis__iwspacemalloc(ctrl, nvtxs);
    ubfactors = libmetis__rwspacemalloc(ctrl, ncon);
    newbalv = libmetis__rwspacemalloc(ctrl, ncon);
    minbalv = libmetis__rwspacemalloc(ctrl, ncon);
    limit = (if (if 0.01f64 * nvtxs as libc::c_double >= 25 as libc::c_int as libc::c_double {
        0.01f64 * nvtxs as libc::c_double
    } else {
        25 as libc::c_int as libc::c_double
    }) >= 150 as libc::c_int as libc::c_double
    {
        150 as libc::c_int as libc::c_double
    } else if 0.01f64 * nvtxs as libc::c_double >= 25 as libc::c_int as libc::c_double {
        0.01f64 * nvtxs as libc::c_double
    } else {
        25 as libc::c_int as libc::c_double
    }) as idx_t;
    ffactor = (0.5f64
        / (if 20 as libc::c_int >= nvtxs {
            20 as libc::c_int
        } else {
            nvtxs
        }) as libc::c_double) as real_t;
    queues = libmetis__wspacemalloc(
        ctrl,
        ((2 as libc::c_int * ncon) as u64)
            .wrapping_mul(::core::mem::size_of::<*mut rpq_t>() as u64),
    ) as *mut *mut rpq_t;
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int * ncon {
        let ref mut fresh14 = *queues.offset(i as isize);
        *fresh14 = libmetis__rpqCreate(nvtxs as size_t);
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < nvtxs {
        *qnum.offset(i as isize) =
            libmetis__iargmax_nrm(ncon as size_t, vwgt.offset((i * ncon) as isize), invtvwgt);
        i += 1;
        i;
    }
    origbal = libmetis__ComputeLoadImbalanceDiffVec(
        graph,
        2 as libc::c_int,
        (*ctrl).pijbm,
        (*ctrl).ubfactors,
        ubfactors,
    );
    i = 0 as libc::c_int;
    while i < ncon {
        *ubfactors.offset(i as isize) =
            if *ubfactors.offset(i as isize) > 0 as libc::c_int as libc::c_float {
                *((*ctrl).ubfactors).offset(i as isize) + *ubfactors.offset(i as isize)
            } else {
                *((*ctrl).ubfactors).offset(i as isize)
            };
        i += 1;
        i;
    }
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_REFINE as libc::c_int as libc::c_uint != 0 {
        libmetis__Print2WayRefineStats(ctrl, graph, ntpwgts, origbal, -(2 as libc::c_int));
    }
    libmetis__iset(nvtxs as size_t, -(1 as libc::c_int), moved);
    pass = 0 as libc::c_int;
    while pass < niter {
        i = 0 as libc::c_int;
        while i < 2 as libc::c_int * ncon {
            libmetis__rpqReset(*queues.offset(i as isize));
            i += 1;
            i;
        }
        mincutorder = -(1 as libc::c_int);
        initcut = (*graph).mincut;
        mincut = initcut;
        newcut = mincut;
        minbal = libmetis__ComputeLoadImbalanceDiffVec(
            graph,
            2 as libc::c_int,
            (*ctrl).pijbm,
            ubfactors,
            minbalv,
        );
        nbnd = (*graph).nbnd;
        libmetis__irandArrayPermute(nbnd, perm, nbnd / 5 as libc::c_int, 1 as libc::c_int);
        ii = 0 as libc::c_int;
        while ii < nbnd {
            i = *bndind.offset(*perm.offset(ii as isize) as isize);
            rgain = (*ed.offset(i as isize) - *id.offset(i as isize)) as real_t;
            libmetis__rpqInsert(
                *queues.offset(
                    (2 as libc::c_int * *qnum.offset(i as isize) + *where_0.offset(i as isize))
                        as isize,
                ),
                i,
                rgain,
            );
            ii += 1;
            ii;
        }
        nswaps = 0 as libc::c_int;
        while nswaps < nvtxs {
            libmetis__SelectQueue(
                graph,
                (*ctrl).pijbm,
                ubfactors,
                queues,
                &mut from,
                &mut cnum,
            );
            to = (from + 1 as libc::c_int) % 2 as libc::c_int;
            if from == -(1 as libc::c_int) || {
                higain =
                    libmetis__rpqGetTop(*queues.offset((2 as libc::c_int * cnum + from) as isize));
                higain == -(1 as libc::c_int)
            } {
                break;
            }
            newcut -= *ed.offset(higain as isize) - *id.offset(higain as isize);
            libmetis__iaxpy(
                ncon as size_t,
                1 as libc::c_int,
                vwgt.offset((higain * ncon) as isize),
                1 as libc::c_int as size_t,
                pwgts.offset((to * ncon) as isize),
                1 as libc::c_int as size_t,
            );
            libmetis__iaxpy(
                ncon as size_t,
                -(1 as libc::c_int),
                vwgt.offset((higain * ncon) as isize),
                1 as libc::c_int as size_t,
                pwgts.offset((from * ncon) as isize),
                1 as libc::c_int as size_t,
            );
            newbal = libmetis__ComputeLoadImbalanceDiffVec(
                graph,
                2 as libc::c_int,
                (*ctrl).pijbm,
                ubfactors,
                newbalv,
            );
            if newcut < mincut && newbal <= ffactor
                || newcut == mincut
                    && (newbal < minbal
                        || newbal == minbal
                            && libmetis__BetterBalance2Way(ncon, minbalv, newbalv) != 0)
            {
                mincut = newcut;
                minbal = newbal;
                mincutorder = nswaps;
                libmetis__rcopy(ncon as size_t, newbalv, minbalv);
            } else if nswaps - mincutorder > limit {
                newcut += *ed.offset(higain as isize) - *id.offset(higain as isize);
                libmetis__iaxpy(
                    ncon as size_t,
                    1 as libc::c_int,
                    vwgt.offset((higain * ncon) as isize),
                    1 as libc::c_int as size_t,
                    pwgts.offset((from * ncon) as isize),
                    1 as libc::c_int as size_t,
                );
                libmetis__iaxpy(
                    ncon as size_t,
                    -(1 as libc::c_int),
                    vwgt.offset((higain * ncon) as isize),
                    1 as libc::c_int as size_t,
                    pwgts.offset((to * ncon) as isize),
                    1 as libc::c_int as size_t,
                );
                break;
            }
            *where_0.offset(higain as isize) = to;
            *moved.offset(higain as isize) = nswaps;
            *swaps.offset(nswaps as isize) = higain;
            if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_MOVEINFO as libc::c_int as libc::c_uint
                != 0
            {
                printf(
                    b"Moved%6d from %d(%d) Gain:%5d, Cut:%5d, NPwgts:\0" as *const u8
                        as *const libc::c_char,
                    higain,
                    from,
                    cnum,
                    *ed.offset(higain as isize) - *id.offset(higain as isize),
                    newcut,
                );
                l = 0 as libc::c_int;
                while l < ncon {
                    printf(
                        b"(%.3f %.3f)\0" as *const u8 as *const libc::c_char,
                        (*pwgts.offset(l as isize) as libc::c_float * *invtvwgt.offset(l as isize))
                            as libc::c_double,
                        (*pwgts.offset((ncon + l) as isize) as libc::c_float
                            * *invtvwgt.offset(l as isize))
                            as libc::c_double,
                    );
                    l += 1;
                    l;
                }
                printf(
                    b" %+.3f LB: %.3f(%+.3f)\n\0" as *const u8 as *const libc::c_char,
                    minbal as libc::c_double,
                    libmetis__ComputeLoadImbalance(graph, 2 as libc::c_int, (*ctrl).pijbm)
                        as libc::c_double,
                    newbal as libc::c_double,
                );
            }
            tmp = *id.offset(higain as isize);
            *id.offset(higain as isize) = *ed.offset(higain as isize);
            *ed.offset(higain as isize) = tmp;
            if *ed.offset(higain as isize) == 0 as libc::c_int
                && *xadj.offset(higain as isize)
                    < *xadj.offset((higain + 1 as libc::c_int) as isize)
            {
                nbnd -= 1;
                *bndind.offset(*bndptr.offset(higain as isize) as isize) =
                    *bndind.offset(nbnd as isize);
                *bndptr.offset(*bndind.offset(nbnd as isize) as isize) =
                    *bndptr.offset(higain as isize);
                *bndptr.offset(higain as isize) = -(1 as libc::c_int);
            }
            j = *xadj.offset(higain as isize);
            while j < *xadj.offset((higain + 1 as libc::c_int) as isize) {
                k = *adjncy.offset(j as isize);
                kwgt = if to == *where_0.offset(k as isize) {
                    *adjwgt.offset(j as isize)
                } else {
                    -*adjwgt.offset(j as isize)
                };
                let ref mut fresh15 = *id.offset(k as isize);
                *fresh15 += kwgt;
                let ref mut fresh16 = *ed.offset(k as isize);
                *fresh16 -= kwgt;
                if *bndptr.offset(k as isize) != -(1 as libc::c_int) {
                    if *ed.offset(k as isize) == 0 as libc::c_int {
                        nbnd -= 1;
                        *bndind.offset(*bndptr.offset(k as isize) as isize) =
                            *bndind.offset(nbnd as isize);
                        *bndptr.offset(*bndind.offset(nbnd as isize) as isize) =
                            *bndptr.offset(k as isize);
                        *bndptr.offset(k as isize) = -(1 as libc::c_int);
                        if *moved.offset(k as isize) == -(1 as libc::c_int) {
                            libmetis__rpqDelete(
                                *queues.offset(
                                    (2 as libc::c_int * *qnum.offset(k as isize)
                                        + *where_0.offset(k as isize))
                                        as isize,
                                ),
                                k,
                            );
                        }
                    } else if *moved.offset(k as isize) == -(1 as libc::c_int) {
                        rgain = (*ed.offset(k as isize) - *id.offset(k as isize)) as real_t;
                        libmetis__rpqUpdate(
                            *queues.offset(
                                (2 as libc::c_int * *qnum.offset(k as isize)
                                    + *where_0.offset(k as isize))
                                    as isize,
                            ),
                            k,
                            rgain,
                        );
                    }
                } else if *ed.offset(k as isize) > 0 as libc::c_int {
                    *bndind.offset(nbnd as isize) = k;
                    let fresh17 = nbnd;
                    nbnd = nbnd + 1;
                    *bndptr.offset(k as isize) = fresh17;
                    if *moved.offset(k as isize) == -(1 as libc::c_int) {
                        rgain = (*ed.offset(k as isize) - *id.offset(k as isize)) as real_t;
                        libmetis__rpqInsert(
                            *queues.offset(
                                (2 as libc::c_int * *qnum.offset(k as isize)
                                    + *where_0.offset(k as isize))
                                    as isize,
                            ),
                            k,
                            rgain,
                        );
                    }
                }
                j += 1;
                j;
            }
            nswaps += 1;
            nswaps;
        }
        i = 0 as libc::c_int;
        while i < nswaps {
            *moved.offset(*swaps.offset(i as isize) as isize) = -(1 as libc::c_int);
            i += 1;
            i;
        }
        nswaps -= 1;
        nswaps;
        while nswaps > mincutorder {
            higain = *swaps.offset(nswaps as isize);
            let ref mut fresh18 = *where_0.offset(higain as isize);
            *fresh18 = (*where_0.offset(higain as isize) + 1 as libc::c_int) % 2 as libc::c_int;
            to = *fresh18;
            tmp = *id.offset(higain as isize);
            *id.offset(higain as isize) = *ed.offset(higain as isize);
            *ed.offset(higain as isize) = tmp;
            if *ed.offset(higain as isize) == 0 as libc::c_int
                && *bndptr.offset(higain as isize) != -(1 as libc::c_int)
                && *xadj.offset(higain as isize)
                    < *xadj.offset((higain + 1 as libc::c_int) as isize)
            {
                nbnd -= 1;
                *bndind.offset(*bndptr.offset(higain as isize) as isize) =
                    *bndind.offset(nbnd as isize);
                *bndptr.offset(*bndind.offset(nbnd as isize) as isize) =
                    *bndptr.offset(higain as isize);
                *bndptr.offset(higain as isize) = -(1 as libc::c_int);
            } else if *ed.offset(higain as isize) > 0 as libc::c_int
                && *bndptr.offset(higain as isize) == -(1 as libc::c_int)
            {
                *bndind.offset(nbnd as isize) = higain;
                let fresh19 = nbnd;
                nbnd = nbnd + 1;
                *bndptr.offset(higain as isize) = fresh19;
            }
            libmetis__iaxpy(
                ncon as size_t,
                1 as libc::c_int,
                vwgt.offset((higain * ncon) as isize),
                1 as libc::c_int as size_t,
                pwgts.offset((to * ncon) as isize),
                1 as libc::c_int as size_t,
            );
            libmetis__iaxpy(
                ncon as size_t,
                -(1 as libc::c_int),
                vwgt.offset((higain * ncon) as isize),
                1 as libc::c_int as size_t,
                pwgts.offset(((to + 1 as libc::c_int) % 2 as libc::c_int * ncon) as isize),
                1 as libc::c_int as size_t,
            );
            j = *xadj.offset(higain as isize);
            while j < *xadj.offset((higain + 1 as libc::c_int) as isize) {
                k = *adjncy.offset(j as isize);
                kwgt = if to == *where_0.offset(k as isize) {
                    *adjwgt.offset(j as isize)
                } else {
                    -*adjwgt.offset(j as isize)
                };
                let ref mut fresh20 = *id.offset(k as isize);
                *fresh20 += kwgt;
                let ref mut fresh21 = *ed.offset(k as isize);
                *fresh21 -= kwgt;
                if *bndptr.offset(k as isize) != -(1 as libc::c_int)
                    && *ed.offset(k as isize) == 0 as libc::c_int
                {
                    nbnd -= 1;
                    *bndind.offset(*bndptr.offset(k as isize) as isize) =
                        *bndind.offset(nbnd as isize);
                    *bndptr.offset(*bndind.offset(nbnd as isize) as isize) =
                        *bndptr.offset(k as isize);
                    *bndptr.offset(k as isize) = -(1 as libc::c_int);
                }
                if *bndptr.offset(k as isize) == -(1 as libc::c_int)
                    && *ed.offset(k as isize) > 0 as libc::c_int
                {
                    *bndind.offset(nbnd as isize) = k;
                    let fresh22 = nbnd;
                    nbnd = nbnd + 1;
                    *bndptr.offset(k as isize) = fresh22;
                }
                j += 1;
                j;
            }
            nswaps -= 1;
            nswaps;
        }
        (*graph).mincut = mincut;
        (*graph).nbnd = nbnd;
        if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_REFINE as libc::c_int as libc::c_uint != 0 {
            libmetis__Print2WayRefineStats(ctrl, graph, ntpwgts, minbal, mincutorder);
        }
        if mincutorder <= 0 as libc::c_int || mincut == initcut {
            break;
        }
        pass += 1;
        pass;
    }
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int * ncon {
        libmetis__rpqDestroy(*queues.offset(i as isize));
        i += 1;
        i;
    }
    libmetis__wspacepop(ctrl);
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__SelectQueue(
    mut graph: *mut graph_t,
    mut pijbm: *mut real_t,
    mut ubfactors: *mut real_t,
    mut queues: *mut *mut rpq_t,
    mut from: *mut idx_t,
    mut cnum: *mut idx_t,
) {
    let mut ncon: idx_t = 0;
    let mut i: idx_t = 0;
    let mut part: idx_t = 0;
    let mut max: real_t = 0.;
    let mut tmp: real_t = 0.;
    ncon = (*graph).ncon;
    *from = -(1 as libc::c_int);
    *cnum = -(1 as libc::c_int);
    max = 0.0f64 as real_t;
    part = 0 as libc::c_int;
    while part < 2 as libc::c_int {
        i = 0 as libc::c_int;
        while i < ncon {
            tmp = *((*graph).pwgts).offset((part * ncon + i) as isize) as libc::c_float
                * *pijbm.offset((part * ncon + i) as isize)
                - *ubfactors.offset(i as isize);
            if tmp >= max {
                max = tmp;
                *from = part;
                *cnum = i;
            }
            i += 1;
            i;
        }
        part += 1;
        part;
    }
    if *from != -(1 as libc::c_int) {
        if libmetis__rpqLength(*queues.offset((2 as libc::c_int * *cnum + *from) as isize))
            == 0 as libc::c_int as u64
        {
            i = 0 as libc::c_int;
            while i < ncon {
                if libmetis__rpqLength(*queues.offset((2 as libc::c_int * i + *from) as isize))
                    > 0 as libc::c_int as u64
                {
                    max = *((*graph).pwgts).offset((*from * ncon + i) as isize) as libc::c_float
                        * *pijbm.offset((*from * ncon + i) as isize)
                        - *ubfactors.offset(i as isize);
                    *cnum = i;
                    break;
                } else {
                    i += 1;
                    i;
                }
            }
            i += 1;
            i;
            while i < ncon {
                tmp = *((*graph).pwgts).offset((*from * ncon + i) as isize) as libc::c_float
                    * *pijbm.offset((*from * ncon + i) as isize)
                    - *ubfactors.offset(i as isize);
                if tmp > max
                    && libmetis__rpqLength(*queues.offset((2 as libc::c_int * i + *from) as isize))
                        > 0 as libc::c_int as u64
                {
                    max = tmp;
                    *cnum = i;
                }
                i += 1;
                i;
            }
        }
    } else {
        part = 0 as libc::c_int;
        while part < 2 as libc::c_int {
            i = 0 as libc::c_int;
            while i < ncon {
                if libmetis__rpqLength(*queues.offset((2 as libc::c_int * i + part) as isize))
                    > 0 as libc::c_int as u64
                    && (*from == -(1 as libc::c_int)
                        || libmetis__rpqSeeTopKey(
                            *queues.offset((2 as libc::c_int * i + part) as isize),
                        ) > max)
                {
                    max = libmetis__rpqSeeTopKey(
                        *queues.offset((2 as libc::c_int * i + part) as isize),
                    );
                    *from = part;
                    *cnum = i;
                }
                i += 1;
                i;
            }
            part += 1;
            part;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__Print2WayRefineStats(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut ntpwgts: *mut real_t,
    mut deltabal: real_t,
    mut mincutorder: idx_t,
) {
    let mut i: libc::c_int = 0;
    if mincutorder == -(2 as libc::c_int) {
        printf(b"Parts: \0" as *const u8 as *const libc::c_char);
        printf(
            b"Nv-Nb[%5d %5d] ICut: %6d\0" as *const u8 as *const libc::c_char,
            (*graph).nvtxs,
            (*graph).nbnd,
            (*graph).mincut,
        );
        printf(b" [\0" as *const u8 as *const libc::c_char);
        i = 0 as libc::c_int;
        while i < (*graph).ncon {
            printf(
                b"(%.3f %.3f T:%.3f %.3f)\0" as *const u8 as *const libc::c_char,
                (*((*graph).pwgts).offset(i as isize) as libc::c_float
                    * *((*graph).invtvwgt).offset(i as isize)) as libc::c_double,
                (*((*graph).pwgts).offset(((*graph).ncon + i) as isize) as libc::c_float
                    * *((*graph).invtvwgt).offset(i as isize)) as libc::c_double,
                *ntpwgts.offset(i as isize) as libc::c_double,
                *ntpwgts.offset(((*graph).ncon + i) as isize) as libc::c_double,
            );
            i += 1;
            i;
        }
        printf(
            b"] LB: %.3f(%+.3f)\n\0" as *const u8 as *const libc::c_char,
            libmetis__ComputeLoadImbalance(graph, 2 as libc::c_int, (*ctrl).pijbm)
                as libc::c_double,
            deltabal as libc::c_double,
        );
    } else {
        printf(
            b"\tMincut: %6d at %5d NBND %6d NPwgts: [\0" as *const u8 as *const libc::c_char,
            (*graph).mincut,
            mincutorder,
            (*graph).nbnd,
        );
        i = 0 as libc::c_int;
        while i < (*graph).ncon {
            printf(
                b"(%.3f %.3f)\0" as *const u8 as *const libc::c_char,
                (*((*graph).pwgts).offset(i as isize) as libc::c_float
                    * *((*graph).invtvwgt).offset(i as isize)) as libc::c_double,
                (*((*graph).pwgts).offset(((*graph).ncon + i) as isize) as libc::c_float
                    * *((*graph).invtvwgt).offset(i as isize)) as libc::c_double,
            );
            i += 1;
            i;
        }
        printf(
            b"] LB: %.3f(%+.3f)\n\0" as *const u8 as *const libc::c_char,
            libmetis__ComputeLoadImbalance(graph, 2 as libc::c_int, (*ctrl).pijbm)
                as libc::c_double,
            deltabal as libc::c_double,
        );
    };
}
