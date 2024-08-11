use ::libc;
use libc::{abs, printf};

use crate::GKlib::{memory::gk_free, timers::gk_CPUSeconds};

use super::{
    compress::libmetis__CompressGraph,
    gklib::{
        libmetis__icopy, libmetis__imalloc, libmetis__irandArrayPermute, libmetis__iset,
        libmetis__rpqCreate, libmetis__rpqDelete, libmetis__rpqDestroy, libmetis__rpqGetTop,
        libmetis__rpqInsert, libmetis__rpqLength, libmetis__rpqReset, libmetis__rpqSeeTopVal,
        libmetis__rpqUpdate,
    },
    graph::{libmetis__FreeGraph, libmetis__SetupGraph},
    ometis::{
        libmetis__MMDOrder, libmetis__MlevelNodeBisectionMultiple, libmetis__SplitGraphOrder,
    },
    options::{libmetis__FreeCtrl, libmetis__SetupCtrl},
    srefine::{
        libmetis__Allocate2WayNodePartitionMemory, libmetis__Compute2WayNodePartitionParams,
    },
    structure::*,
    timing::{libmetis__InitTimers, libmetis__PrintTimers},
    util::{libmetis__InitRandom, METIS_ERROR_INPUT, METIS_OK},
    wspace::{
        libmetis__AllocateWorkSpace, libmetis__iwspacemalloc, libmetis__wspacepop,
        libmetis__wspacepush,
    },
};

pub type C2RustUnnamed = libc::c_int;
pub const METIS_ERROR: C2RustUnnamed = -4;
pub const METIS_ERROR_MEMORY: C2RustUnnamed = -3;
#[no_mangle]
pub unsafe extern "C" fn METIS_NodeNDP(
    mut nvtxs: idx_t,
    mut xadj: *mut idx_t,
    mut adjncy: *mut idx_t,
    mut vwgt: *mut idx_t,
    mut npes: idx_t,
    mut options: *mut idx_t,
    mut perm: *mut idx_t,
    mut iperm: *mut idx_t,
    mut sizes: *mut idx_t,
) -> libc::c_int {
    let mut i: idx_t = 0;
    let mut ii: idx_t = 0;
    let mut j: idx_t = 0;
    let mut l: idx_t = 0;
    let mut nnvtxs: idx_t = 0 as libc::c_int;
    let mut graph: *mut graph_t = 0 as *mut graph_t;
    let mut ctrl: *mut ctrl_t = 0 as *mut ctrl_t;
    let mut cptr: *mut idx_t = 0 as *mut idx_t;
    let mut cind: *mut idx_t = 0 as *mut idx_t;
    ctrl = libmetis__SetupCtrl(
        METIS_OP_OMETIS,
        options,
        1 as libc::c_int,
        3 as libc::c_int,
        0 as *mut real_t,
        0 as *mut real_t,
    );
    if ctrl.is_null() {
        return METIS_ERROR_INPUT as libc::c_int;
    }
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint != 0 {
        libmetis__InitTimers(ctrl);
    }
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint != 0 {
        (*ctrl).TotalTmr -= gk_CPUSeconds();
    }
    if (*ctrl).compress != 0 {
        cptr = libmetis__imalloc(
            (nvtxs + 1 as libc::c_int) as size_t,
            b"OMETIS: cptr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        cind = libmetis__imalloc(
            nvtxs as size_t,
            b"OMETIS: cind\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        graph = libmetis__CompressGraph(ctrl, nvtxs, xadj, adjncy, vwgt, cptr, cind);
        if graph.is_null() {
            gk_free(
                &mut cptr as *mut *mut idx_t as *mut *mut libc::c_void,
                &mut cind as *mut *mut idx_t,
                0 as *mut *mut libc::c_void,
            );
            (*ctrl).compress = 0 as libc::c_int;
        } else {
            nnvtxs = (*graph).nvtxs;
        }
    }
    if (*ctrl).compress == 0 as libc::c_int {
        graph = libmetis__SetupGraph(
            ctrl,
            nvtxs,
            1 as libc::c_int,
            xadj,
            adjncy,
            vwgt,
            0 as *mut idx_t,
            0 as *mut idx_t,
        );
    }
    libmetis__AllocateWorkSpace(ctrl, graph);
    libmetis__iset(
        (2 as libc::c_int * npes - 1 as libc::c_int) as size_t,
        0 as libc::c_int,
        sizes,
    );
    libmetis__MlevelNestedDissectionP(
        ctrl,
        graph,
        iperm,
        (*graph).nvtxs,
        npes,
        0 as libc::c_int,
        sizes,
    );
    if (*ctrl).compress != 0 {
        i = 0 as libc::c_int;
        while i < nnvtxs {
            *perm.offset(*iperm.offset(i as isize) as isize) = i;
            i += 1;
            i;
        }
        ii = 0 as libc::c_int;
        l = ii;
        while ii < nnvtxs {
            i = *perm.offset(ii as isize);
            j = *cptr.offset(i as isize);
            while j < *cptr.offset((i + 1 as libc::c_int) as isize) {
                let fresh0 = l;
                l = l + 1;
                *iperm.offset(*cind.offset(j as isize) as isize) = fresh0;
                j += 1;
                j;
            }
            ii += 1;
            ii;
        }
        gk_free(
            &mut cptr as *mut *mut idx_t as *mut *mut libc::c_void,
            &mut cind as *mut *mut idx_t,
            0 as *mut *mut libc::c_void,
        );
    }
    i = 0 as libc::c_int;
    while i < nvtxs {
        *perm.offset(*iperm.offset(i as isize) as isize) = i;
        i += 1;
        i;
    }
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint != 0 {
        (*ctrl).TotalTmr += gk_CPUSeconds();
    }
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint != 0 {
        libmetis__PrintTimers(ctrl);
    }
    libmetis__FreeCtrl(&mut ctrl);
    return METIS_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__MlevelNestedDissectionP(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut order: *mut idx_t,
    mut lastvtx: idx_t,
    mut npes: idx_t,
    mut cpos: idx_t,
    mut sizes: *mut idx_t,
) {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut nbnd: idx_t = 0;
    let mut label: *mut idx_t = 0 as *mut idx_t;
    let mut bndind: *mut idx_t = 0 as *mut idx_t;
    let mut lgraph: *mut graph_t = 0 as *mut graph_t;
    let mut rgraph: *mut graph_t = 0 as *mut graph_t;
    nvtxs = (*graph).nvtxs;
    if nvtxs == 0 as libc::c_int {
        libmetis__FreeGraph(&mut graph);
        return;
    }
    libmetis__MlevelNodeBisectionMultiple(ctrl, graph);
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_SEPINFO as libc::c_int as libc::c_uint != 0 {
        printf(
            b"Nvtxs: %6d, [%6d %6d %6d]\n\0" as *const u8 as *const libc::c_char,
            (*graph).nvtxs,
            *((*graph).pwgts).offset(0 as libc::c_int as isize),
            *((*graph).pwgts).offset(1 as libc::c_int as isize),
            *((*graph).pwgts).offset(2 as libc::c_int as isize),
        );
    }
    if cpos < npes - 1 as libc::c_int {
        *sizes.offset((2 as libc::c_int * npes - 2 as libc::c_int - cpos) as isize) =
            *((*graph).pwgts).offset(2 as libc::c_int as isize);
        *sizes.offset(
            (2 as libc::c_int * npes
                - 2 as libc::c_int
                - (2 as libc::c_int * cpos + 1 as libc::c_int)) as isize,
        ) = *((*graph).pwgts).offset(1 as libc::c_int as isize);
        *sizes.offset(
            (2 as libc::c_int * npes
                - 2 as libc::c_int
                - (2 as libc::c_int * cpos + 2 as libc::c_int)) as isize,
        ) = *((*graph).pwgts).offset(0 as libc::c_int as isize);
    }
    nbnd = (*graph).nbnd;
    bndind = (*graph).bndind;
    label = (*graph).label;
    i = 0 as libc::c_int;
    while i < nbnd {
        lastvtx -= 1;
        *order.offset(*label.offset(*bndind.offset(i as isize) as isize) as isize) = lastvtx;
        i += 1;
        i;
    }
    libmetis__SplitGraphOrder(ctrl, graph, &mut lgraph, &mut rgraph);
    libmetis__FreeGraph(&mut graph);
    if ((*lgraph).nvtxs > 120 as libc::c_int
        || (2 as libc::c_int * cpos + 2 as libc::c_int) < npes - 1 as libc::c_int)
        && (*lgraph).nedges > 0 as libc::c_int
    {
        libmetis__MlevelNestedDissectionP(
            ctrl,
            lgraph,
            order,
            lastvtx - (*rgraph).nvtxs,
            npes,
            2 as libc::c_int * cpos + 2 as libc::c_int,
            sizes,
        );
    } else {
        libmetis__MMDOrder(ctrl, lgraph, order, lastvtx - (*rgraph).nvtxs);
        libmetis__FreeGraph(&mut lgraph);
    }
    if ((*rgraph).nvtxs > 120 as libc::c_int
        || (2 as libc::c_int * cpos + 1 as libc::c_int) < npes - 1 as libc::c_int)
        && (*rgraph).nedges > 0 as libc::c_int
    {
        libmetis__MlevelNestedDissectionP(
            ctrl,
            rgraph,
            order,
            lastvtx,
            npes,
            2 as libc::c_int * cpos + 1 as libc::c_int,
            sizes,
        );
    } else {
        libmetis__MMDOrder(ctrl, rgraph, order, lastvtx);
        libmetis__FreeGraph(&mut rgraph);
    };
}
#[no_mangle]
pub unsafe extern "C" fn METIS_ComputeVertexSeparator(
    mut nvtxs: *mut idx_t,
    mut xadj: *mut idx_t,
    mut adjncy: *mut idx_t,
    mut vwgt: *mut idx_t,
    mut options: *mut idx_t,
    mut r_sepsize: *mut idx_t,
    mut part: *mut idx_t,
) -> libc::c_int {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut graph: *mut graph_t = 0 as *mut graph_t;
    let mut ctrl: *mut ctrl_t = 0 as *mut ctrl_t;
    ctrl = libmetis__SetupCtrl(
        METIS_OP_OMETIS,
        options,
        1 as libc::c_int,
        3 as libc::c_int,
        0 as *mut real_t,
        0 as *mut real_t,
    );
    if ctrl.is_null() {
        return METIS_ERROR_INPUT as libc::c_int;
    }
    libmetis__InitRandom((*ctrl).seed);
    graph = libmetis__SetupGraph(
        ctrl,
        *nvtxs,
        1 as libc::c_int,
        xadj,
        adjncy,
        vwgt,
        0 as *mut idx_t,
        0 as *mut idx_t,
    );
    libmetis__AllocateWorkSpace(ctrl, graph);
    (*ctrl).CoarsenTo = 100 as libc::c_int;
    libmetis__MlevelNodeBisectionMultiple(ctrl, graph);
    *r_sepsize = *((*graph).pwgts).offset(2 as libc::c_int as isize);
    libmetis__icopy(*nvtxs as size_t, (*graph).where_0, part);
    libmetis__FreeGraph(&mut graph);
    libmetis__FreeCtrl(&mut ctrl);
    return METIS_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn METIS_NodeRefine(
    mut nvtxs: idx_t,
    mut xadj: *mut idx_t,
    mut vwgt: *mut idx_t,
    mut adjncy: *mut idx_t,
    mut where_0: *mut idx_t,
    mut hmarker: *mut idx_t,
    mut ubfactor: real_t,
) -> libc::c_int {
    let mut graph: *mut graph_t = 0 as *mut graph_t;
    let mut ctrl: *mut ctrl_t = 0 as *mut ctrl_t;
    ctrl = libmetis__SetupCtrl(
        METIS_OP_OMETIS,
        0 as *mut idx_t,
        1 as libc::c_int,
        3 as libc::c_int,
        0 as *mut real_t,
        0 as *mut real_t,
    );
    if ctrl.is_null() {
        return METIS_ERROR_INPUT as libc::c_int;
    }
    graph = libmetis__SetupGraph(
        ctrl,
        nvtxs,
        1 as libc::c_int,
        xadj,
        adjncy,
        vwgt,
        0 as *mut idx_t,
        0 as *mut idx_t,
    );
    libmetis__AllocateWorkSpace(ctrl, graph);
    libmetis__Allocate2WayNodePartitionMemory(ctrl, graph);
    libmetis__icopy(nvtxs as size_t, where_0, (*graph).where_0);
    libmetis__Compute2WayNodePartitionParams(ctrl, graph);
    libmetis__FM_2WayNodeRefine1SidedP(ctrl, graph, hmarker, ubfactor, 10 as libc::c_int);
    libmetis__icopy(nvtxs as size_t, (*graph).where_0, where_0);
    libmetis__FreeGraph(&mut graph);
    libmetis__FreeCtrl(&mut ctrl);
    return METIS_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__FM_2WayNodeRefine1SidedP(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut hmarker: *mut idx_t,
    mut ubfactor: real_t,
    mut npasses: idx_t,
) {
    let mut i: idx_t = 0;
    let mut ii: idx_t = 0;
    let mut j: idx_t = 0;
    let mut k: idx_t = 0;
    let mut jj: idx_t = 0;
    let mut kk: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut nbnd: idx_t = 0;
    let mut nswaps: idx_t = 0;
    let mut nmind: idx_t = 0;
    let mut nbad: idx_t = 0;
    let mut qsize: idx_t = 0;
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
    let mut vwgt: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    let mut pwgts: *mut idx_t = 0 as *mut idx_t;
    let mut edegrees: *mut idx_t = 0 as *mut idx_t;
    let mut bndind: *mut idx_t = 0 as *mut idx_t;
    let mut bndptr: *mut idx_t = 0 as *mut idx_t;
    let mut mptr: *mut idx_t = 0 as *mut idx_t;
    let mut mind: *mut idx_t = 0 as *mut idx_t;
    let mut swaps: *mut idx_t = 0 as *mut idx_t;
    let mut inqueue: *mut idx_t = 0 as *mut idx_t;
    let mut queue: *mut rpq_t = 0 as *mut rpq_t;
    let mut rinfo: *mut nrinfo_t = 0 as *mut nrinfo_t;
    let mut higain: idx_t = 0;
    let mut oldgain: idx_t = 0;
    let mut mincut: idx_t = 0;
    let mut initcut: idx_t = 0;
    let mut mincutorder: idx_t = 0;
    let mut pass: idx_t = 0;
    let mut from: idx_t = 0;
    let mut to: idx_t = 0;
    let mut limit: idx_t = 0;
    let mut badmaxpwgt: idx_t = 0;
    let mut mindiff: idx_t = 0;
    let mut newdiff: idx_t = 0;
    libmetis__wspacepush(ctrl);
    nvtxs = (*graph).nvtxs;
    xadj = (*graph).xadj;
    adjncy = (*graph).adjncy;
    vwgt = (*graph).vwgt;
    bndind = (*graph).bndind;
    bndptr = (*graph).bndptr;
    where_0 = (*graph).where_0;
    pwgts = (*graph).pwgts;
    rinfo = (*graph).nrinfo;
    queue = libmetis__rpqCreate(nvtxs as size_t);
    inqueue = libmetis__iset(
        nvtxs as size_t,
        -(1 as libc::c_int),
        libmetis__iwspacemalloc(ctrl, nvtxs),
    );
    swaps = libmetis__iwspacemalloc(ctrl, nvtxs);
    mptr = libmetis__iwspacemalloc(ctrl, nvtxs + 1 as libc::c_int);
    mind = libmetis__iwspacemalloc(ctrl, 2 as libc::c_int * nvtxs);
    badmaxpwgt = (ubfactor
        * (if *pwgts.offset(0 as libc::c_int as isize) >= *pwgts.offset(1 as libc::c_int as isize) {
            *pwgts.offset(0 as libc::c_int as isize)
        } else {
            *pwgts.offset(1 as libc::c_int as isize)
        }) as libc::c_float) as idx_t;
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_REFINE as libc::c_int as libc::c_uint != 0 {
        printf(
            b"Partitions-N1: [%6d %6d] Nv-Nb[%6d %6d] MaxPwgt[%6d]. ISep: %6d\n\0" as *const u8
                as *const libc::c_char,
            *pwgts.offset(0 as libc::c_int as isize),
            *pwgts.offset(1 as libc::c_int as isize),
            (*graph).nvtxs,
            (*graph).nbnd,
            badmaxpwgt,
            (*graph).mincut,
        );
    }
    to = if *pwgts.offset(0 as libc::c_int as isize) < *pwgts.offset(1 as libc::c_int as isize) {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    pass = 0 as libc::c_int;
    while pass < npasses {
        from = to;
        to = (from + 1 as libc::c_int) % 2 as libc::c_int;
        libmetis__rpqReset(queue);
        mincutorder = -(1 as libc::c_int);
        mincut = (*graph).mincut;
        initcut = mincut;
        nbnd = (*graph).nbnd;
        libmetis__irandArrayPermute(nbnd, swaps, nbnd, 1 as libc::c_int);
        ii = 0 as libc::c_int;
        while ii < nbnd {
            i = *bndind.offset(*swaps.offset(ii as isize) as isize);
            if *hmarker.offset(i as isize) == -(1 as libc::c_int)
                || *hmarker.offset(i as isize) == to
            {
                libmetis__rpqInsert(
                    queue,
                    i,
                    (*vwgt.offset(i as isize) - (*rinfo.offset(i as isize)).edegrees[from as usize])
                        as real_t,
                );
                *inqueue.offset(i as isize) = pass;
            }
            ii += 1;
            ii;
        }
        qsize = libmetis__rpqLength(queue) as idx_t;
        limit = nbnd;
        nbad = 0 as libc::c_int;
        nmind = nbad;
        *mptr.offset(0 as libc::c_int as isize) = nmind;
        mindiff = abs(
            *pwgts.offset(0 as libc::c_int as isize) - *pwgts.offset(1 as libc::c_int as isize)
        );
        nswaps = 0 as libc::c_int;
        while nswaps < nvtxs {
            higain = libmetis__rpqGetTop(queue);
            if higain == -(1 as libc::c_int) {
                break;
            }
            if nmind + *xadj.offset((higain + 1 as libc::c_int) as isize)
                - *xadj.offset(higain as isize)
                >= 2 as libc::c_int * nvtxs - 1 as libc::c_int
            {
                break;
            }
            *inqueue.offset(higain as isize) = -(1 as libc::c_int);
            if *pwgts.offset(to as isize) + *vwgt.offset(higain as isize) > badmaxpwgt {
                let fresh1 = nbad;
                nbad = nbad + 1;
                if fresh1 > limit {
                    break;
                }
                nswaps -= 1;
                nswaps;
            } else {
                let ref mut fresh2 = *pwgts.offset(2 as libc::c_int as isize);
                *fresh2 -= *vwgt.offset(higain as isize)
                    - (*rinfo.offset(higain as isize)).edegrees[from as usize];
                newdiff = abs(*pwgts.offset(to as isize) + *vwgt.offset(higain as isize)
                    - (*pwgts.offset(from as isize)
                        - (*rinfo.offset(higain as isize)).edegrees[from as usize]));
                if *pwgts.offset(2 as libc::c_int as isize) < mincut
                    || *pwgts.offset(2 as libc::c_int as isize) == mincut && newdiff < mindiff
                {
                    mincut = *pwgts.offset(2 as libc::c_int as isize);
                    mincutorder = nswaps;
                    mindiff = newdiff;
                    nbad = 0 as libc::c_int;
                } else {
                    let fresh3 = nbad;
                    nbad = nbad + 1;
                    if fresh3 > limit {
                        let ref mut fresh4 = *pwgts.offset(2 as libc::c_int as isize);
                        *fresh4 += *vwgt.offset(higain as isize)
                            - (*rinfo.offset(higain as isize)).edegrees[from as usize];
                        break;
                    }
                }
                nbnd -= 1;
                *bndind.offset(*bndptr.offset(higain as isize) as isize) =
                    *bndind.offset(nbnd as isize);
                *bndptr.offset(*bndind.offset(nbnd as isize) as isize) =
                    *bndptr.offset(higain as isize);
                *bndptr.offset(higain as isize) = -(1 as libc::c_int);
                let ref mut fresh5 = *pwgts.offset(to as isize);
                *fresh5 += *vwgt.offset(higain as isize);
                *where_0.offset(higain as isize) = to;
                *swaps.offset(nswaps as isize) = higain;
                j = *xadj.offset(higain as isize);
                while j < *xadj.offset((higain + 1 as libc::c_int) as isize) {
                    k = *adjncy.offset(j as isize);
                    if *where_0.offset(k as isize) == 2 as libc::c_int {
                        let ref mut fresh6 = (*rinfo.offset(k as isize)).edegrees[to as usize];
                        *fresh6 += *vwgt.offset(higain as isize);
                    } else if *where_0.offset(k as isize) == from {
                        *bndind.offset(nbnd as isize) = k;
                        let fresh7 = nbnd;
                        nbnd = nbnd + 1;
                        *bndptr.offset(k as isize) = fresh7;
                        let fresh8 = nmind;
                        nmind = nmind + 1;
                        *mind.offset(fresh8 as isize) = k;
                        *where_0.offset(k as isize) = 2 as libc::c_int;
                        let ref mut fresh9 = *pwgts.offset(from as isize);
                        *fresh9 -= *vwgt.offset(k as isize);
                        edegrees = ((*rinfo.offset(k as isize)).edegrees).as_mut_ptr();
                        let ref mut fresh10 = *edegrees.offset(1 as libc::c_int as isize);
                        *fresh10 = 0 as libc::c_int;
                        *edegrees.offset(0 as libc::c_int as isize) = *fresh10;
                        jj = *xadj.offset(k as isize);
                        while jj < *xadj.offset((k + 1 as libc::c_int) as isize) {
                            kk = *adjncy.offset(jj as isize);
                            if *where_0.offset(kk as isize) != 2 as libc::c_int {
                                let ref mut fresh11 =
                                    *edegrees.offset(*where_0.offset(kk as isize) as isize);
                                *fresh11 += *vwgt.offset(kk as isize);
                            } else {
                                oldgain = *vwgt.offset(kk as isize)
                                    - (*rinfo.offset(kk as isize)).edegrees[from as usize];
                                let ref mut fresh12 =
                                    (*rinfo.offset(kk as isize)).edegrees[from as usize];
                                *fresh12 -= *vwgt.offset(k as isize);
                                if *inqueue.offset(kk as isize) == pass {
                                    libmetis__rpqUpdate(
                                        queue,
                                        kk,
                                        (oldgain + *vwgt.offset(k as isize)) as real_t,
                                    );
                                }
                            }
                            jj += 1;
                            jj;
                        }
                        if *hmarker.offset(k as isize) == -(1 as libc::c_int)
                            || *hmarker.offset(k as isize) == to
                        {
                            libmetis__rpqInsert(
                                queue,
                                k,
                                (*vwgt.offset(k as isize) - *edegrees.offset(from as isize))
                                    as real_t,
                            );
                            *inqueue.offset(k as isize) = pass;
                        }
                    }
                    j += 1;
                    j;
                }
                *mptr.offset((nswaps + 1 as libc::c_int) as isize) = nmind;
                if (*ctrl).dbglvl as libc::c_uint
                    & METIS_DBG_MOVEINFO as libc::c_int as libc::c_uint
                    != 0
                {
                    printf(
                        b"Moved %6d to %3d, Gain: %5d [%5d] \t[%5d %5d %5d] [%3d %2d]\n\0"
                            as *const u8 as *const libc::c_char,
                        higain,
                        to,
                        *vwgt.offset(higain as isize)
                            - (*rinfo.offset(higain as isize)).edegrees[from as usize],
                        *vwgt.offset(higain as isize),
                        *pwgts.offset(0 as libc::c_int as isize),
                        *pwgts.offset(1 as libc::c_int as isize),
                        *pwgts.offset(2 as libc::c_int as isize),
                        nswaps,
                        limit,
                    );
                }
            }
            nswaps += 1;
            nswaps;
        }
        nswaps -= 1;
        nswaps;
        while nswaps > mincutorder {
            higain = *swaps.offset(nswaps as isize);
            let ref mut fresh13 = *pwgts.offset(2 as libc::c_int as isize);
            *fresh13 += *vwgt.offset(higain as isize);
            let ref mut fresh14 = *pwgts.offset(to as isize);
            *fresh14 -= *vwgt.offset(higain as isize);
            *where_0.offset(higain as isize) = 2 as libc::c_int;
            *bndind.offset(nbnd as isize) = higain;
            let fresh15 = nbnd;
            nbnd = nbnd + 1;
            *bndptr.offset(higain as isize) = fresh15;
            edegrees = ((*rinfo.offset(higain as isize)).edegrees).as_mut_ptr();
            let ref mut fresh16 = *edegrees.offset(1 as libc::c_int as isize);
            *fresh16 = 0 as libc::c_int;
            *edegrees.offset(0 as libc::c_int as isize) = *fresh16;
            j = *xadj.offset(higain as isize);
            while j < *xadj.offset((higain + 1 as libc::c_int) as isize) {
                k = *adjncy.offset(j as isize);
                if *where_0.offset(k as isize) == 2 as libc::c_int {
                    let ref mut fresh17 = (*rinfo.offset(k as isize)).edegrees[to as usize];
                    *fresh17 -= *vwgt.offset(higain as isize);
                } else {
                    let ref mut fresh18 = *edegrees.offset(*where_0.offset(k as isize) as isize);
                    *fresh18 += *vwgt.offset(k as isize);
                }
                j += 1;
                j;
            }
            j = *mptr.offset(nswaps as isize);
            while j < *mptr.offset((nswaps + 1 as libc::c_int) as isize) {
                k = *mind.offset(j as isize);
                *where_0.offset(k as isize) = from;
                let ref mut fresh19 = *pwgts.offset(from as isize);
                *fresh19 += *vwgt.offset(k as isize);
                let ref mut fresh20 = *pwgts.offset(2 as libc::c_int as isize);
                *fresh20 -= *vwgt.offset(k as isize);
                nbnd -= 1;
                *bndind.offset(*bndptr.offset(k as isize) as isize) = *bndind.offset(nbnd as isize);
                *bndptr.offset(*bndind.offset(nbnd as isize) as isize) = *bndptr.offset(k as isize);
                *bndptr.offset(k as isize) = -(1 as libc::c_int);
                jj = *xadj.offset(k as isize);
                while jj < *xadj.offset((k + 1 as libc::c_int) as isize) {
                    kk = *adjncy.offset(jj as isize);
                    if *where_0.offset(kk as isize) == 2 as libc::c_int {
                        let ref mut fresh21 = (*rinfo.offset(kk as isize)).edegrees[from as usize];
                        *fresh21 += *vwgt.offset(k as isize);
                    }
                    jj += 1;
                    jj;
                }
                j += 1;
                j;
            }
            nswaps -= 1;
            nswaps;
        }
        if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_REFINE as libc::c_int as libc::c_uint != 0 {
            printf(
                b"\tMinimum sep: %6d at %5d, PWGTS: [%6d %6d], NBND: %6d, QSIZE: %6d\n\0"
                    as *const u8 as *const libc::c_char,
                mincut,
                mincutorder,
                *pwgts.offset(0 as libc::c_int as isize),
                *pwgts.offset(1 as libc::c_int as isize),
                nbnd,
                qsize,
            );
        }
        (*graph).mincut = mincut;
        (*graph).nbnd = nbnd;
        if pass % 2 as libc::c_int == 1 as libc::c_int
            && (mincutorder == -(1 as libc::c_int) || mincut >= initcut)
        {
            break;
        }
        pass += 1;
        pass;
    }
    libmetis__rpqDestroy(queue);
    libmetis__wspacepop(ctrl);
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__FM_2WayNodeRefine2SidedP(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut hmarker: *mut idx_t,
    mut ubfactor: real_t,
    mut npasses: idx_t,
) {
    let mut i: idx_t = 0;
    let mut ii: idx_t = 0;
    let mut j: idx_t = 0;
    let mut k: idx_t = 0;
    let mut jj: idx_t = 0;
    let mut kk: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut nbnd: idx_t = 0;
    let mut nswaps: idx_t = 0;
    let mut nmind: idx_t = 0;
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
    let mut vwgt: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    let mut pwgts: *mut idx_t = 0 as *mut idx_t;
    let mut edegrees: *mut idx_t = 0 as *mut idx_t;
    let mut bndind: *mut idx_t = 0 as *mut idx_t;
    let mut bndptr: *mut idx_t = 0 as *mut idx_t;
    let mut mptr: *mut idx_t = 0 as *mut idx_t;
    let mut mind: *mut idx_t = 0 as *mut idx_t;
    let mut moved: *mut idx_t = 0 as *mut idx_t;
    let mut swaps: *mut idx_t = 0 as *mut idx_t;
    let mut queues: [*mut rpq_t; 2] = [0 as *mut rpq_t; 2];
    let mut rinfo: *mut nrinfo_t = 0 as *mut nrinfo_t;
    let mut higain: idx_t = 0;
    let mut oldgain: idx_t = 0;
    let mut mincut: idx_t = 0;
    let mut initcut: idx_t = 0;
    let mut mincutorder: idx_t = 0;
    let mut pass: idx_t = 0;
    let mut to: idx_t = 0;
    let mut other: idx_t = 0;
    let mut limit: idx_t = 0;
    let mut badmaxpwgt: idx_t = 0;
    let mut mindiff: idx_t = 0;
    let mut newdiff: idx_t = 0;
    let mut u: [idx_t; 2] = [0; 2];
    let mut g: [idx_t; 2] = [0; 2];
    libmetis__wspacepush(ctrl);
    nvtxs = (*graph).nvtxs;
    xadj = (*graph).xadj;
    adjncy = (*graph).adjncy;
    vwgt = (*graph).vwgt;
    bndind = (*graph).bndind;
    bndptr = (*graph).bndptr;
    where_0 = (*graph).where_0;
    pwgts = (*graph).pwgts;
    rinfo = (*graph).nrinfo;
    queues[0 as libc::c_int as usize] = libmetis__rpqCreate(nvtxs as size_t);
    queues[1 as libc::c_int as usize] = libmetis__rpqCreate(nvtxs as size_t);
    moved = libmetis__iwspacemalloc(ctrl, nvtxs);
    swaps = libmetis__iwspacemalloc(ctrl, nvtxs);
    mptr = libmetis__iwspacemalloc(ctrl, nvtxs + 1 as libc::c_int);
    mind = libmetis__iwspacemalloc(ctrl, 2 as libc::c_int * nvtxs);
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_REFINE as libc::c_int as libc::c_uint != 0 {
        printf(
            b"Partitions: [%6d %6d] Nv-Nb[%6d %6d]. ISep: %6d\n\0" as *const u8
                as *const libc::c_char,
            *pwgts.offset(0 as libc::c_int as isize),
            *pwgts.offset(1 as libc::c_int as isize),
            (*graph).nvtxs,
            (*graph).nbnd,
            (*graph).mincut,
        );
    }
    badmaxpwgt = (ubfactor
        * (if *pwgts.offset(0 as libc::c_int as isize) >= *pwgts.offset(1 as libc::c_int as isize) {
            *pwgts.offset(0 as libc::c_int as isize)
        } else {
            *pwgts.offset(1 as libc::c_int as isize)
        }) as libc::c_float) as idx_t;
    pass = 0 as libc::c_int;
    while pass < npasses {
        libmetis__iset(nvtxs as size_t, -(1 as libc::c_int), moved);
        libmetis__rpqReset(queues[0 as libc::c_int as usize]);
        libmetis__rpqReset(queues[1 as libc::c_int as usize]);
        mincutorder = -(1 as libc::c_int);
        mincut = (*graph).mincut;
        initcut = mincut;
        nbnd = (*graph).nbnd;
        libmetis__irandArrayPermute(nbnd, swaps, nbnd, 1 as libc::c_int);
        ii = 0 as libc::c_int;
        while ii < nbnd {
            i = *bndind.offset(*swaps.offset(ii as isize) as isize);
            if *hmarker.offset(i as isize) == -(1 as libc::c_int) {
                libmetis__rpqInsert(
                    queues[0 as libc::c_int as usize],
                    i,
                    (*vwgt.offset(i as isize)
                        - (*rinfo.offset(i as isize)).edegrees[1 as libc::c_int as usize])
                        as real_t,
                );
                libmetis__rpqInsert(
                    queues[1 as libc::c_int as usize],
                    i,
                    (*vwgt.offset(i as isize)
                        - (*rinfo.offset(i as isize)).edegrees[0 as libc::c_int as usize])
                        as real_t,
                );
                *moved.offset(i as isize) = -(5 as libc::c_int);
            } else if *hmarker.offset(i as isize) != 2 as libc::c_int {
                libmetis__rpqInsert(
                    queues[*hmarker.offset(i as isize) as usize],
                    i,
                    (*vwgt.offset(i as isize)
                        - (*rinfo.offset(i as isize)).edegrees[((*hmarker.offset(i as isize)
                            + 1 as libc::c_int)
                            % 2 as libc::c_int)
                            as usize]) as real_t,
                );
                *moved.offset(i as isize) = -(10 as libc::c_int + *hmarker.offset(i as isize));
            }
            ii += 1;
            ii;
        }
        limit = nbnd;
        nmind = 0 as libc::c_int;
        *mptr.offset(0 as libc::c_int as isize) = nmind;
        mindiff = abs(
            *pwgts.offset(0 as libc::c_int as isize) - *pwgts.offset(1 as libc::c_int as isize)
        );
        to = if *pwgts.offset(0 as libc::c_int as isize) < *pwgts.offset(1 as libc::c_int as isize)
        {
            0 as libc::c_int
        } else {
            1 as libc::c_int
        };
        nswaps = 0 as libc::c_int;
        while nswaps < nvtxs {
            u[0 as libc::c_int as usize] =
                libmetis__rpqSeeTopVal(queues[0 as libc::c_int as usize]);
            u[1 as libc::c_int as usize] =
                libmetis__rpqSeeTopVal(queues[1 as libc::c_int as usize]);
            if u[0 as libc::c_int as usize] != -(1 as libc::c_int)
                && u[1 as libc::c_int as usize] != -(1 as libc::c_int)
            {
                g[0 as libc::c_int as usize] = *vwgt.offset(u[0 as libc::c_int as usize] as isize)
                    - (*rinfo.offset(u[0 as libc::c_int as usize] as isize)).edegrees
                        [1 as libc::c_int as usize];
                g[1 as libc::c_int as usize] = *vwgt.offset(u[1 as libc::c_int as usize] as isize)
                    - (*rinfo.offset(u[1 as libc::c_int as usize] as isize)).edegrees
                        [0 as libc::c_int as usize];
                to = if g[0 as libc::c_int as usize] > g[1 as libc::c_int as usize] {
                    0 as libc::c_int
                } else if g[0 as libc::c_int as usize] < g[1 as libc::c_int as usize] {
                    1 as libc::c_int
                } else {
                    pass % 2 as libc::c_int
                };
                if *pwgts.offset(to as isize) + *vwgt.offset(u[to as usize] as isize) > badmaxpwgt {
                    to = (to + 1 as libc::c_int) % 2 as libc::c_int;
                }
            } else {
                if u[0 as libc::c_int as usize] == -(1 as libc::c_int)
                    && u[1 as libc::c_int as usize] == -(1 as libc::c_int)
                {
                    break;
                }
                if u[0 as libc::c_int as usize] != -(1 as libc::c_int)
                    && *pwgts.offset(0 as libc::c_int as isize)
                        + *vwgt.offset(u[0 as libc::c_int as usize] as isize)
                        <= badmaxpwgt
                {
                    to = 0 as libc::c_int;
                } else {
                    if !(u[1 as libc::c_int as usize] != -(1 as libc::c_int)
                        && *pwgts.offset(1 as libc::c_int as isize)
                            + *vwgt.offset(u[1 as libc::c_int as usize] as isize)
                            <= badmaxpwgt)
                    {
                        break;
                    }
                    to = 1 as libc::c_int;
                }
            }
            other = (to + 1 as libc::c_int) % 2 as libc::c_int;
            higain = libmetis__rpqGetTop(queues[to as usize]);
            if *moved.offset(higain as isize) == -(5 as libc::c_int) {
                libmetis__rpqDelete(queues[other as usize], higain);
            }
            if nmind + *xadj.offset((higain + 1 as libc::c_int) as isize)
                - *xadj.offset(higain as isize)
                >= 2 as libc::c_int * nvtxs - 1 as libc::c_int
            {
                break;
            }
            let ref mut fresh22 = *pwgts.offset(2 as libc::c_int as isize);
            *fresh22 -= *vwgt.offset(higain as isize)
                - (*rinfo.offset(higain as isize)).edegrees[other as usize];
            newdiff = abs(*pwgts.offset(to as isize) + *vwgt.offset(higain as isize)
                - (*pwgts.offset(other as isize)
                    - (*rinfo.offset(higain as isize)).edegrees[other as usize]));
            if *pwgts.offset(2 as libc::c_int as isize) < mincut
                || *pwgts.offset(2 as libc::c_int as isize) == mincut && newdiff < mindiff
            {
                mincut = *pwgts.offset(2 as libc::c_int as isize);
                mincutorder = nswaps;
                mindiff = newdiff;
            } else if nswaps - mincutorder > limit {
                let ref mut fresh23 = *pwgts.offset(2 as libc::c_int as isize);
                *fresh23 += *vwgt.offset(higain as isize)
                    - (*rinfo.offset(higain as isize)).edegrees[other as usize];
                break;
            }
            nbnd -= 1;
            *bndind.offset(*bndptr.offset(higain as isize) as isize) =
                *bndind.offset(nbnd as isize);
            *bndptr.offset(*bndind.offset(nbnd as isize) as isize) =
                *bndptr.offset(higain as isize);
            *bndptr.offset(higain as isize) = -(1 as libc::c_int);
            let ref mut fresh24 = *pwgts.offset(to as isize);
            *fresh24 += *vwgt.offset(higain as isize);
            *where_0.offset(higain as isize) = to;
            *moved.offset(higain as isize) = nswaps;
            *swaps.offset(nswaps as isize) = higain;
            j = *xadj.offset(higain as isize);
            while j < *xadj.offset((higain + 1 as libc::c_int) as isize) {
                k = *adjncy.offset(j as isize);
                if *where_0.offset(k as isize) == 2 as libc::c_int {
                    oldgain = *vwgt.offset(k as isize)
                        - (*rinfo.offset(k as isize)).edegrees[to as usize];
                    let ref mut fresh25 = (*rinfo.offset(k as isize)).edegrees[to as usize];
                    *fresh25 += *vwgt.offset(higain as isize);
                    if *moved.offset(k as isize) == -(5 as libc::c_int)
                        || *moved.offset(k as isize) == -(10 as libc::c_int + other)
                    {
                        libmetis__rpqUpdate(
                            queues[other as usize],
                            k,
                            (oldgain - *vwgt.offset(higain as isize)) as real_t,
                        );
                    }
                } else if *where_0.offset(k as isize) == other {
                    *bndind.offset(nbnd as isize) = k;
                    let fresh26 = nbnd;
                    nbnd = nbnd + 1;
                    *bndptr.offset(k as isize) = fresh26;
                    let fresh27 = nmind;
                    nmind = nmind + 1;
                    *mind.offset(fresh27 as isize) = k;
                    *where_0.offset(k as isize) = 2 as libc::c_int;
                    let ref mut fresh28 = *pwgts.offset(other as isize);
                    *fresh28 -= *vwgt.offset(k as isize);
                    edegrees = ((*rinfo.offset(k as isize)).edegrees).as_mut_ptr();
                    let ref mut fresh29 = *edegrees.offset(1 as libc::c_int as isize);
                    *fresh29 = 0 as libc::c_int;
                    *edegrees.offset(0 as libc::c_int as isize) = *fresh29;
                    jj = *xadj.offset(k as isize);
                    while jj < *xadj.offset((k + 1 as libc::c_int) as isize) {
                        kk = *adjncy.offset(jj as isize);
                        if *where_0.offset(kk as isize) != 2 as libc::c_int {
                            let ref mut fresh30 =
                                *edegrees.offset(*where_0.offset(kk as isize) as isize);
                            *fresh30 += *vwgt.offset(kk as isize);
                        } else {
                            oldgain = *vwgt.offset(kk as isize)
                                - (*rinfo.offset(kk as isize)).edegrees[other as usize];
                            let ref mut fresh31 =
                                (*rinfo.offset(kk as isize)).edegrees[other as usize];
                            *fresh31 -= *vwgt.offset(k as isize);
                            if *moved.offset(kk as isize) == -(5 as libc::c_int)
                                || *moved.offset(kk as isize) == -(10 as libc::c_int + to)
                            {
                                libmetis__rpqUpdate(
                                    queues[to as usize],
                                    kk,
                                    (oldgain + *vwgt.offset(k as isize)) as real_t,
                                );
                            }
                        }
                        jj += 1;
                        jj;
                    }
                    if *moved.offset(k as isize) == -(1 as libc::c_int)
                        && (*hmarker.offset(k as isize) == -(1 as libc::c_int)
                            || *hmarker.offset(k as isize) == to)
                    {
                        libmetis__rpqInsert(
                            queues[to as usize],
                            k,
                            (*vwgt.offset(k as isize) - *edegrees.offset(other as isize)) as real_t,
                        );
                        *moved.offset(k as isize) = -(10 as libc::c_int + to);
                    }
                }
                j += 1;
                j;
            }
            *mptr.offset((nswaps + 1 as libc::c_int) as isize) = nmind;
            if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_MOVEINFO as libc::c_int as libc::c_uint
                != 0
            {
                printf(
                    b"Moved %6d to %3d, Gain: %5d [%5d] [%4d %4d] \t[%5d %5d %5d]\n\0" as *const u8
                        as *const libc::c_char,
                    higain,
                    to,
                    g[to as usize],
                    g[other as usize],
                    *vwgt.offset(u[to as usize] as isize),
                    *vwgt.offset(u[other as usize] as isize),
                    *pwgts.offset(0 as libc::c_int as isize),
                    *pwgts.offset(1 as libc::c_int as isize),
                    *pwgts.offset(2 as libc::c_int as isize),
                );
            }
            nswaps += 1;
        }
        nswaps -= 1;
        while nswaps > mincutorder {
            higain = *swaps.offset(nswaps as isize);
            to = *where_0.offset(higain as isize);
            other = (to + 1 as libc::c_int) % 2 as libc::c_int;
            let ref mut fresh32 = *pwgts.offset(2 as libc::c_int as isize);
            *fresh32 += *vwgt.offset(higain as isize);
            let ref mut fresh33 = *pwgts.offset(to as isize);
            *fresh33 -= *vwgt.offset(higain as isize);
            *where_0.offset(higain as isize) = 2 as libc::c_int;
            *bndind.offset(nbnd as isize) = higain;
            let fresh34 = nbnd;
            nbnd = nbnd + 1;
            *bndptr.offset(higain as isize) = fresh34;
            edegrees = ((*rinfo.offset(higain as isize)).edegrees).as_mut_ptr();
            let ref mut fresh35 = *edegrees.offset(1 as libc::c_int as isize);
            *fresh35 = 0 as libc::c_int;
            *edegrees.offset(0 as libc::c_int as isize) = *fresh35;
            j = *xadj.offset(higain as isize);
            while j < *xadj.offset((higain + 1 as libc::c_int) as isize) {
                k = *adjncy.offset(j as isize);
                if *where_0.offset(k as isize) == 2 as libc::c_int {
                    let ref mut fresh36 = (*rinfo.offset(k as isize)).edegrees[to as usize];
                    *fresh36 -= *vwgt.offset(higain as isize);
                } else {
                    let ref mut fresh37 = *edegrees.offset(*where_0.offset(k as isize) as isize);
                    *fresh37 += *vwgt.offset(k as isize);
                }
                j += 1;
                j;
            }
            j = *mptr.offset(nswaps as isize);
            while j < *mptr.offset((nswaps + 1 as libc::c_int) as isize) {
                k = *mind.offset(j as isize);
                *where_0.offset(k as isize) = other;
                let ref mut fresh38 = *pwgts.offset(other as isize);
                *fresh38 += *vwgt.offset(k as isize);
                let ref mut fresh39 = *pwgts.offset(2 as libc::c_int as isize);
                *fresh39 -= *vwgt.offset(k as isize);
                nbnd -= 1;
                *bndind.offset(*bndptr.offset(k as isize) as isize) = *bndind.offset(nbnd as isize);
                *bndptr.offset(*bndind.offset(nbnd as isize) as isize) = *bndptr.offset(k as isize);
                *bndptr.offset(k as isize) = -(1 as libc::c_int);
                jj = *xadj.offset(k as isize);
                while jj < *xadj.offset((k + 1 as libc::c_int) as isize) {
                    kk = *adjncy.offset(jj as isize);
                    if *where_0.offset(kk as isize) == 2 as libc::c_int {
                        let ref mut fresh40 = (*rinfo.offset(kk as isize)).edegrees[other as usize];
                        *fresh40 += *vwgt.offset(k as isize);
                    }
                    jj += 1;
                    jj;
                }
                j += 1;
                j;
            }
            nswaps -= 1;
            nswaps;
        }
        if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_REFINE as libc::c_int as libc::c_uint != 0 {
            printf(
                b"\tMinimum sep: %6d at %5d, PWGTS: [%6d %6d], NBND: %6d\n\0" as *const u8
                    as *const libc::c_char,
                mincut,
                mincutorder,
                *pwgts.offset(0 as libc::c_int as isize),
                *pwgts.offset(1 as libc::c_int as isize),
                nbnd,
            );
        }
        (*graph).mincut = mincut;
        (*graph).nbnd = nbnd;
        if mincutorder == -(1 as libc::c_int) || mincut >= initcut {
            break;
        }
        pass += 1;
        pass;
    }
    libmetis__rpqDestroy(queues[0 as libc::c_int as usize]);
    libmetis__rpqDestroy(queues[1 as libc::c_int as usize]);
    libmetis__wspacepop(ctrl);
}
