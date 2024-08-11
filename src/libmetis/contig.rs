use crate::GKlib::{error::gk_errexit, memory::gk_free};

use super::{
    compress::libmetis__CompressGraph,
    gklib::{
        libmetis__iaxpy, libmetis__icopy, libmetis__iincset, libmetis__imalloc,
        libmetis__irandArrayPermute, libmetis__iset, libmetis__ismalloc, libmetis__isum,
        libmetis__rkvsortd, libmetis__rpqCreate, libmetis__rpqDelete, libmetis__rpqDestroy,
        libmetis__rpqGetTop, libmetis__rpqInsert, libmetis__rpqLength, libmetis__rpqReset,
        libmetis__rpqSeeTopVal, libmetis__rpqUpdate,
    },
    graph::{libmetis__FreeGraph, libmetis__SetupGraph},
    kwayfm::libmetis__KWayVolUpdate,
    mcutil::libmetis__BetterBalanceKWay,
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
        libmetis__AllocateWorkSpace, libmetis__cnbrpoolGetNext, libmetis__iwspacemalloc,
        libmetis__vnbrpoolGetNext, libmetis__wspacemalloc, libmetis__wspacepop,
        libmetis__wspacepush,
    },
};
use ::libc;
use libc::printf;

use super::structure::*;
#[no_mangle]
pub unsafe extern "C" fn libmetis__FindPartitionInducedComponents(
    mut graph: *mut graph_t,
    mut where_0: *mut idx_t,
    mut cptr: *mut idx_t,
    mut cind: *mut idx_t,
) -> idx_t {
    let mut i: idx_t = 0;
    let mut ii: idx_t = 0;
    let mut j: idx_t = 0;
    let mut jj: idx_t = 0;
    let mut k: idx_t = 0;
    let mut me: idx_t = 0 as libc::c_int;
    let mut nvtxs: idx_t = 0;
    let mut first: idx_t = 0;
    let mut last: idx_t = 0;
    let mut nleft: idx_t = 0;
    let mut ncmps: idx_t = 0;

    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut touched: *mut idx_t = 0 as *mut idx_t;
    let mut perm: *mut idx_t = 0 as *mut idx_t;
    let mut todo: *mut idx_t = 0 as *mut idx_t;
    let mut mustfree_ccsr: idx_t = 0 as libc::c_int;
    let mut mustfree_where: idx_t = 0 as libc::c_int;
    nvtxs = (*graph).nvtxs;
    let mut xadj = &mut (*graph).xadj;
    adjncy = (*graph).adjncy;
    if cptr.is_null() {
        cptr = libmetis__imalloc(
            (nvtxs + 1) as size_t,
            b"FindPartitionInducedComponents: cptr\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        cind = libmetis__imalloc(
            nvtxs as size_t,
            b"FindPartitionInducedComponents: cind\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        mustfree_ccsr = 1;
    }
    if where_0.is_null() {
        where_0 = libmetis__ismalloc(
            nvtxs as size_t,
            0 as libc::c_int,
            b"FindPartitionInducedComponents: where\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        mustfree_where = 1;
    }
    perm = libmetis__iincset(
        nvtxs as size_t,
        0 as libc::c_int,
        libmetis__imalloc(
            nvtxs as size_t,
            b"FindPartitionInducedComponents: perm\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        ),
    );
    todo = libmetis__iincset(
        nvtxs as size_t,
        0 as libc::c_int,
        libmetis__imalloc(
            nvtxs as size_t,
            b"FindPartitionInducedComponents: todo\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        ),
    );
    touched = libmetis__ismalloc(
        nvtxs as size_t,
        0 as libc::c_int,
        b"FindPartitionInducedComponents: touched\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    ncmps = -(1);
    last = 0 as libc::c_int;
    first = last;
    nleft = nvtxs;
    while nleft > 0 as libc::c_int {
        if first == last {
            ncmps += 1;
            *cptr.offset(ncmps as isize) = first;
            i = *todo.offset(0 as libc::c_int as isize);
            let fresh0 = last;
            last = last + 1;
            *cind.offset(fresh0 as isize) = i;
            *touched.offset(i as isize) = 1;
            me = *where_0.offset(i as isize);
        }
        let fresh1 = first;
        first = first + 1;
        i = *cind.offset(fresh1 as isize);
        k = *perm.offset(i as isize);
        nleft -= 1;
        let ref mut fresh2 = *todo.offset(k as isize);
        *fresh2 = *todo.offset(nleft as isize);
        j = *fresh2;
        *perm.offset(j as isize) = k;
        j = xadj[i as usize];
        while j < xadj[(i + 1) as usize] {
            k = *adjncy.offset(j as isize);
            if *where_0.offset(k as isize) == me && *touched.offset(k as isize) == 0 {
                let fresh3 = last;
                last = last + 1;
                *cind.offset(fresh3 as isize) = k;
                *touched.offset(k as isize) = 1;
            }
            j += 1;
            j;
        }
    }
    ncmps += 1;
    *cptr.offset(ncmps as isize) = first;
    if mustfree_ccsr != 0 {
        gk_free(
            &mut cptr as *mut *mut idx_t as *mut *mut libc::c_void,
            &mut cind as *mut *mut idx_t,
            0 as *mut *mut libc::c_void,
        );
    }
    if mustfree_where != 0 {
        gk_free(
            &mut where_0 as *mut *mut idx_t as *mut *mut libc::c_void,
            0 as *mut *mut libc::c_void,
        );
    }
    gk_free(
        &mut perm as *mut *mut idx_t as *mut *mut libc::c_void,
        &mut todo as *mut *mut idx_t,
        &mut touched as *mut *mut idx_t,
        0 as *mut *mut libc::c_void,
    );
    return ncmps;
}
#[no_mangle]
pub unsafe extern "C" fn ComputeBFSOrdering(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut bfsperm: *mut idx_t,
) {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut k: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut first: idx_t = 0;
    let mut last: idx_t = 0;

    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut perm: *mut idx_t = 0 as *mut idx_t;
    libmetis__wspacepush(ctrl);
    nvtxs = (*graph).nvtxs;
    let mut xadj = &mut (*graph).xadj;
    adjncy = (*graph).adjncy;
    perm = libmetis__iincset(
        nvtxs as size_t,
        0 as libc::c_int,
        libmetis__iwspacemalloc(ctrl, nvtxs),
    );
    libmetis__iincset(nvtxs as size_t, 0 as libc::c_int, bfsperm);
    last = 0 as libc::c_int;
    first = last;
    while first < nvtxs {
        if first == last {
            k = *bfsperm.offset(last as isize);
            *perm.offset(k as isize) = -(1);
            last += 1;
            last;
        }
        let fresh4 = first;
        first = first + 1;
        i = *bfsperm.offset(fresh4 as isize);
        j = xadj[i as usize];
        while j < xadj[(i + 1) as usize] {
            k = *adjncy.offset(j as isize);
            if *perm.offset(k as isize) != -(1) {
                *bfsperm.offset(*perm.offset(k as isize) as isize) = *bfsperm.offset(last as isize);
                *perm.offset(*bfsperm.offset(last as isize) as isize) = *perm.offset(k as isize);
                let fresh5 = last;
                last = last + 1;
                *bfsperm.offset(fresh5 as isize) = k;
                *perm.offset(k as isize) = -(1);
            }
            j += 1;
            j;
        }
    }
    libmetis__wspacepop(ctrl);
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__IsConnected(
    mut graph: *mut graph_t,
    mut report: idx_t,
) -> idx_t {
    let mut ncmps: idx_t = 0;
    ncmps = libmetis__FindPartitionInducedComponents(
        graph,
        0 as *mut idx_t,
        0 as *mut idx_t,
        0 as *mut idx_t,
    );
    if ncmps != 1 && report != 0 {
        printf(
            b"The graph is not connected. It has %d connected components.\n\0" as *const u8
                as *const libc::c_char,
            ncmps,
        );
    }
    return (ncmps == 1) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__IsConnectedSubdomain(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut pid: idx_t,
    mut report: idx_t,
) -> idx_t {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut k: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut first: idx_t = 0;
    let mut last: idx_t = 0;
    let mut nleft: idx_t = 0;
    let mut ncmps: idx_t = 0;
    let mut wgt: idx_t = 0;

    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    let mut touched: *mut idx_t = 0 as *mut idx_t;
    let mut queue: *mut idx_t = 0 as *mut idx_t;
    let mut cptr: *mut idx_t = 0 as *mut idx_t;
    nvtxs = (*graph).nvtxs;
    let mut xadj = &mut (*graph).xadj;
    adjncy = (*graph).adjncy;
    where_0 = (*graph).where_0;
    touched = libmetis__ismalloc(
        nvtxs as size_t,
        0 as libc::c_int,
        b"IsConnected: touched\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    queue = libmetis__imalloc(
        nvtxs as size_t,
        b"IsConnected: queue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    cptr = libmetis__imalloc(
        (nvtxs + 1) as size_t,
        b"IsConnected: cptr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    nleft = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < nvtxs {
        if *where_0.offset(i as isize) == pid {
            nleft += 1;
            nleft;
        }
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < nvtxs {
        if *where_0.offset(i as isize) == pid {
            break;
        }
        i += 1;
        i;
    }
    *touched.offset(i as isize) = 1;
    *queue.offset(0 as libc::c_int as isize) = i;
    first = 0 as libc::c_int;
    last = 1;
    *cptr.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    ncmps = 0 as libc::c_int;
    while first != nleft {
        if first == last {
            ncmps += 1;
            *cptr.offset(ncmps as isize) = first;
            i = 0 as libc::c_int;
            while i < nvtxs {
                if *where_0.offset(i as isize) == pid && *touched.offset(i as isize) == 0 {
                    break;
                }
                i += 1;
                i;
            }
            let fresh6 = last;
            last = last + 1;
            *queue.offset(fresh6 as isize) = i;
            *touched.offset(i as isize) = 1;
        }
        let fresh7 = first;
        first = first + 1;
        i = *queue.offset(fresh7 as isize);
        j = xadj[i as usize];
        while j < xadj[(i + 1) as usize] {
            k = *adjncy.offset(j as isize);
            if *where_0.offset(k as isize) == pid && *touched.offset(k as isize) == 0 {
                let fresh8 = last;
                last = last + 1;
                *queue.offset(fresh8 as isize) = k;
                *touched.offset(k as isize) = 1;
            }
            j += 1;
            j;
        }
    }
    ncmps += 1;
    *cptr.offset(ncmps as isize) = first;
    if ncmps > 1 && report != 0 {
        printf(
            b"The graph has %d connected components in partition %d:\t\0" as *const u8
                as *const libc::c_char,
            ncmps,
            pid,
        );
        i = 0 as libc::c_int;
        while i < ncmps {
            wgt = 0 as libc::c_int;
            j = *cptr.offset(i as isize);
            while j < *cptr.offset((i + 1) as isize) {
                wgt += *((*graph).vwgt).offset(*queue.offset(j as isize) as isize);
                j += 1;
                j;
            }
            printf(
                b"[%5d %5d] \0" as *const u8 as *const libc::c_char,
                *cptr.offset((i + 1) as isize) - *cptr.offset(i as isize),
                wgt,
            );
            i += 1;
            i;
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
    }
    gk_free(
        &mut touched as *mut *mut idx_t as *mut *mut libc::c_void,
        &mut queue as *mut *mut idx_t,
        &mut cptr as *mut *mut idx_t,
        0 as *mut *mut libc::c_void,
    );
    return if ncmps == 1 {
        1
    } else {
        0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__FindSepInducedComponents(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut cptr: *mut idx_t,
    mut cind: *mut idx_t,
) -> idx_t {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut k: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut first: idx_t = 0;
    let mut last: idx_t = 0;
    let mut nleft: idx_t = 0;
    let mut ncmps: idx_t = 0;
    let mut wgt: idx_t = 0;

    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    let mut touched: *mut idx_t = 0 as *mut idx_t;
    let mut queue: *mut idx_t = 0 as *mut idx_t;
    nvtxs = (*graph).nvtxs;
    let mut xadj = &mut (*graph).xadj;
    adjncy = (*graph).adjncy;
    where_0 = (*graph).where_0;
    touched = libmetis__ismalloc(
        nvtxs as size_t,
        0 as libc::c_int,
        b"IsConnected: queue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < (*graph).nbnd {
        *touched.offset(*((*graph).bndind).offset(i as isize) as isize) = 1;
        i += 1;
        i;
    }
    queue = cind;
    nleft = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < nvtxs {
        if *where_0.offset(i as isize) != 2 as libc::c_int {
            nleft += 1;
            nleft;
        }
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < nvtxs {
        if *where_0.offset(i as isize) != 2 as libc::c_int {
            break;
        }
        i += 1;
        i;
    }
    *touched.offset(i as isize) = 1;
    *queue.offset(0 as libc::c_int as isize) = i;
    first = 0 as libc::c_int;
    last = 1;
    *cptr.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    ncmps = 0 as libc::c_int;
    while first != nleft {
        if first == last {
            ncmps += 1;
            *cptr.offset(ncmps as isize) = first;
            i = 0 as libc::c_int;
            while i < nvtxs {
                if *touched.offset(i as isize) == 0 {
                    break;
                }
                i += 1;
                i;
            }
            let fresh9 = last;
            last = last + 1;
            *queue.offset(fresh9 as isize) = i;
            *touched.offset(i as isize) = 1;
        }
        let fresh10 = first;
        first = first + 1;
        i = *queue.offset(fresh10 as isize);
        j = xadj[i as usize];
        while j < xadj[(i + 1) as usize] {
            k = *adjncy.offset(j as isize);
            if *touched.offset(k as isize) == 0 {
                let fresh11 = last;
                last = last + 1;
                *queue.offset(fresh11 as isize) = k;
                *touched.offset(k as isize) = 1;
            }
            j += 1;
            j;
        }
    }
    ncmps += 1;
    *cptr.offset(ncmps as isize) = first;
    gk_free(
        &mut touched as *mut *mut idx_t as *mut *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
    return ncmps;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__EliminateComponents(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
) {
    let mut i: idx_t = 0;
    let mut ii: idx_t = 0;
    let mut j: idx_t = 0;
    let mut jj: idx_t = 0;
    let mut k: idx_t = 0;
    let mut me: idx_t = 0;
    let mut nparts: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut ncon: idx_t = 0;
    let mut ncmps: idx_t = 0;
    let mut other: idx_t = 0;
    let mut ncand: idx_t = 0;
    let mut target: idx_t = 0;

    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut vwgt: *mut idx_t = 0 as *mut idx_t;
    let mut adjwgt: *mut idx_t = 0 as *mut idx_t;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    let mut pwgts: *mut idx_t = 0 as *mut idx_t;
    let mut cptr: *mut idx_t = 0 as *mut idx_t;
    let mut cind: *mut idx_t = 0 as *mut idx_t;
    let mut cpvec: *mut idx_t = 0 as *mut idx_t;
    let mut pcptr: *mut idx_t = 0 as *mut idx_t;
    let mut pcind: *mut idx_t = 0 as *mut idx_t;
    let mut cwhere: *mut idx_t = 0 as *mut idx_t;
    let mut cid: idx_t = 0;
    let mut bestcid: idx_t = 0;
    let mut cwgt: *mut idx_t = 0 as *mut idx_t;
    let mut bestcwgt: *mut idx_t = 0 as *mut idx_t;
    let mut ntodo: idx_t = 0;
    let mut oldntodo: idx_t = 0;
    let mut todo: *mut idx_t = 0 as *mut idx_t;
    let mut cand: *mut rkv_t = 0 as *mut rkv_t;
    let mut tpwgts: *mut real_t = 0 as *mut real_t;
    let mut vmarker: *mut idx_t = 0 as *mut idx_t;
    let mut pmarker: *mut idx_t = 0 as *mut idx_t;
    let mut modind: *mut idx_t = 0 as *mut idx_t;
    libmetis__wspacepush(ctrl);
    nvtxs = (*graph).nvtxs;
    ncon = (*graph).ncon;
    let mut xadj = &mut (*graph).xadj;
    adjncy = (*graph).adjncy;
    vwgt = (*graph).vwgt;
    adjwgt = if (*ctrl).objtype as libc::c_uint == METIS_OBJTYPE_VOL as libc::c_int as libc::c_uint
    {
        0 as *mut idx_t
    } else {
        (*graph).adjwgt
    };
    where_0 = (*graph).where_0;
    pwgts = (*graph).pwgts;
    nparts = (*ctrl).nparts;
    tpwgts = (*ctrl).tpwgts;
    cptr = libmetis__iwspacemalloc(ctrl, nvtxs + 1);
    cind = libmetis__iwspacemalloc(ctrl, nvtxs);
    ncmps = libmetis__FindPartitionInducedComponents(graph, where_0, cptr, cind);
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_CONTIGINFO as libc::c_int as libc::c_uint != 0 {
        printf(
            b"I found %d components, for this %d-way partition\n\0" as *const u8
                as *const libc::c_char,
            ncmps,
            nparts,
        );
    }
    if ncmps > nparts {
        cwgt = libmetis__iwspacemalloc(ctrl, ncon);
        bestcwgt = libmetis__iwspacemalloc(ctrl, ncon);
        cpvec = libmetis__iwspacemalloc(ctrl, nparts);
        pcptr = libmetis__iset(
            (nparts + 1) as size_t,
            0 as libc::c_int,
            libmetis__iwspacemalloc(ctrl, nparts + 1),
        );
        pcind = libmetis__iwspacemalloc(ctrl, ncmps);
        cwhere = libmetis__iset(
            nvtxs as size_t,
            -(1),
            libmetis__iwspacemalloc(ctrl, nvtxs),
        );
        todo = libmetis__iwspacemalloc(ctrl, ncmps);
        cand = libmetis__wspacemalloc(
            ctrl,
            (nparts as u64).wrapping_mul(::core::mem::size_of::<rkv_t>() as u64),
        ) as *mut rkv_t;
        if (*ctrl).objtype as libc::c_uint == METIS_OBJTYPE_VOL as libc::c_int as libc::c_uint {
            modind = libmetis__iwspacemalloc(ctrl, nvtxs);
            vmarker = libmetis__iset(
                nvtxs as size_t,
                0 as libc::c_int,
                libmetis__iwspacemalloc(ctrl, nvtxs),
            );
            pmarker = libmetis__iset(
                nparts as size_t,
                -(1),
                libmetis__iwspacemalloc(ctrl, nparts),
            );
        }
        i = 0 as libc::c_int;
        while i < ncmps {
            let ref mut fresh12 = *pcptr.offset(
                *where_0.offset(*cind.offset(*cptr.offset(i as isize) as isize) as isize) as isize,
            );
            *fresh12 += 1;
            *fresh12;
            i += 1;
            i;
        }
        i = 1;
        while i < nparts {
            let ref mut fresh13 = *pcptr.offset(i as isize);
            *fresh13 += *pcptr.offset((i - 1) as isize);
            i += 1;
            i;
        }
        i = nparts;
        while i > 0 as libc::c_int {
            *pcptr.offset(i as isize) = *pcptr.offset((i - 1) as isize);
            i -= 1;
            i;
        }
        *pcptr.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < ncmps {
            let ref mut fresh14 = *pcptr.offset(
                *where_0.offset(*cind.offset(*cptr.offset(i as isize) as isize) as isize) as isize,
            );
            let fresh15 = *fresh14;
            *fresh14 = *fresh14 + 1;
            *pcind.offset(fresh15 as isize) = i;
            i += 1;
            i;
        }
        i = nparts;
        while i > 0 as libc::c_int {
            *pcptr.offset(i as isize) = *pcptr.offset((i - 1) as isize);
            i -= 1;
            i;
        }
        *pcptr.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
        ntodo = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < nparts {
            if *pcptr.offset((i + 1) as isize) - *pcptr.offset(i as isize)
                == 1
            {
                bestcid = *pcind.offset(*pcptr.offset(i as isize) as isize);
            } else {
                bestcid = -(1);
                j = *pcptr.offset(i as isize);
                while j < *pcptr.offset((i + 1) as isize) {
                    cid = *pcind.offset(j as isize);
                    libmetis__iset(ncon as size_t, 0 as libc::c_int, cwgt);
                    ii = *cptr.offset(cid as isize);
                    while ii < *cptr.offset((cid + 1) as isize) {
                        libmetis__iaxpy(
                            ncon as size_t,
                            1,
                            vwgt.offset((*cind.offset(ii as isize) * ncon) as isize),
                            1 as size_t,
                            cwgt,
                            1 as size_t,
                        );
                        ii += 1;
                        ii;
                    }
                    if bestcid == -(1)
                        || libmetis__isum(ncon as size_t, bestcwgt, 1 as size_t)
                            < libmetis__isum(ncon as size_t, cwgt, 1 as size_t)
                    {
                        bestcid = cid;
                        libmetis__icopy(ncon as size_t, cwgt, bestcwgt);
                    }
                    j += 1;
                    j;
                }
                j = *pcptr.offset(i as isize);
                while j < *pcptr.offset((i + 1) as isize) {
                    if *pcind.offset(j as isize) != bestcid {
                        let fresh16 = ntodo;
                        ntodo = ntodo + 1;
                        *todo.offset(fresh16 as isize) = *pcind.offset(j as isize);
                    }
                    j += 1;
                    j;
                }
            }
            j = *cptr.offset(bestcid as isize);
            while j < *cptr.offset((bestcid + 1) as isize) {
                *cwhere.offset(*cind.offset(j as isize) as isize) = i;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
        while ntodo > 0 as libc::c_int {
            oldntodo = ntodo;
            i = 0 as libc::c_int;
            while i < ntodo {
                cid = *todo.offset(i as isize);
                me = *where_0.offset(*cind.offset(*cptr.offset(cid as isize) as isize) as isize);
                libmetis__iset(ncon as size_t, 0 as libc::c_int, cwgt);
                j = *cptr.offset(cid as isize);
                while j < *cptr.offset((cid + 1) as isize) {
                    libmetis__iaxpy(
                        ncon as size_t,
                        1,
                        vwgt.offset((*cind.offset(j as isize) * ncon) as isize),
                        1 as size_t,
                        cwgt,
                        1 as size_t,
                    );
                    j += 1;
                    j;
                }
                if (*ctrl).dbglvl as libc::c_uint
                    & METIS_DBG_CONTIGINFO as libc::c_int as libc::c_uint
                    != 0
                {
                    printf(
                        b"Trying to move %d [%d] from %d\n\0" as *const u8 as *const libc::c_char,
                        cid,
                        libmetis__isum(ncon as size_t, cwgt, 1 as size_t),
                        me,
                    );
                }
                libmetis__iset(nparts as size_t, 0 as libc::c_int, cpvec);
                j = *cptr.offset(cid as isize);
                while j < *cptr.offset((cid + 1) as isize) {
                    ii = *cind.offset(j as isize);
                    jj = xadj[(ii as usize)];
                    while jj < xadj[((ii + 1) as usize)] {
                        if *cwhere.offset(*adjncy.offset(jj as isize) as isize)
                            != -(1)
                        {
                            let ref mut fresh17 =
                                *cpvec
                                    .offset(*cwhere.offset(*adjncy.offset(jj as isize) as isize)
                                        as isize);
                            *fresh17 += if !adjwgt.is_null() {
                                *adjwgt.offset(jj as isize)
                            } else {
                                1
                            };
                        }
                        jj += 1;
                        jj;
                    }
                    j += 1;
                    j;
                }
                ncand = 0 as libc::c_int;
                j = 0 as libc::c_int;
                while j < nparts {
                    if *cpvec.offset(j as isize) > 0 as libc::c_int {
                        (*cand.offset(ncand as isize)).key = *cpvec.offset(j as isize) as real_t;
                        let fresh18 = ncand;
                        ncand = ncand + 1;
                        (*cand.offset(fresh18 as isize)).val = j;
                    }
                    j += 1;
                    j;
                }
                if !(ncand == 0 as libc::c_int) {
                    libmetis__rkvsortd(ncand as size_t, cand);
                    if ncon == 1 {
                        j = 1;
                        while j < ncand {
                            if ((*cand.offset(j as isize)).key as libc::c_double)
                                < 0.5f64
                                    * (*cand.offset(0 as libc::c_int as isize)).key
                                        as libc::c_double
                            {
                                break;
                            }
                            j += 1;
                            j;
                        }
                        ncand = j;
                    }
                    target = (*cand.offset(0 as libc::c_int as isize)).val;
                    j = 1;
                    while j < ncand {
                        if libmetis__BetterBalanceKWay(
                            ncon,
                            cwgt,
                            (*ctrl).ubfactors,
                            1,
                            pwgts.offset((target * ncon) as isize),
                            ((*ctrl).pijbm).offset((target * ncon) as isize),
                            1,
                            pwgts.offset(((*cand.offset(j as isize)).val * ncon) as isize),
                            ((*ctrl).pijbm)
                                .offset(((*cand.offset(j as isize)).val * ncon) as isize),
                        ) != 0
                        {
                            target = (*cand.offset(j as isize)).val;
                        }
                        j += 1;
                        j;
                    }
                    if (*ctrl).dbglvl as libc::c_uint
                        & METIS_DBG_CONTIGINFO as libc::c_int as libc::c_uint
                        != 0
                    {
                        printf(
                            b"\tMoving it to %d [%d] [%d]\n\0" as *const u8 as *const libc::c_char,
                            target,
                            *cpvec.offset(target as isize),
                            ncand,
                        );
                    }
                    if target != me {
                        match (*ctrl).objtype as libc::c_uint {
                            0 => {
                                libmetis__MoveGroupContigForCut(
                                    ctrl, graph, target, cid, cptr, cind,
                                );
                            }
                            1 => {
                                libmetis__MoveGroupContigForVol(
                                    ctrl, graph, target, cid, cptr, cind, vmarker, pmarker, modind,
                                );
                            }
                            _ => {
                                gk_errexit(
                                    15 as libc::c_int,
                                    b"Unknown objtype %d\n\0" as *const u8 as *const libc::c_char
                                        as *mut libc::c_char,
                                    (*ctrl).objtype as libc::c_uint,
                                );
                            }
                        }
                    }
                    j = *cptr.offset(cid as isize);
                    while j < *cptr.offset((cid + 1) as isize) {
                        *cwhere.offset(*cind.offset(j as isize) as isize) = target;
                        j += 1;
                        j;
                    }
                    ntodo -= 1;
                    *todo.offset(i as isize) = *todo.offset(ntodo as isize);
                }
                i += 1;
                i;
            }
            if !(oldntodo == ntodo) {
                continue;
            }
            if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_CONTIGINFO as libc::c_int as libc::c_uint
                != 0
            {
                printf(
                    b"Stopped at ntodo: %d\n\0" as *const u8 as *const libc::c_char,
                    ntodo,
                );
            }
            break;
        }
        i = 0 as libc::c_int;
        while i < nvtxs {
            i += 1;
            i;
        }
    }
    libmetis__wspacepop(ctrl);
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__MoveGroupContigForCut(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut to: idx_t,
    mut gid: idx_t,
    mut ptr: *mut idx_t,
    mut ind: *mut idx_t,
) {
    let mut i: idx_t = 0;
    let mut ii: idx_t = 0;
    let mut iii: idx_t = 0;
    let mut j: idx_t = 0;
    let mut jj: idx_t = 0;
    let mut k: idx_t = 0;
    let mut l: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut nbnd: idx_t = 0;
    let mut from: idx_t = 0;
    let mut me: idx_t = 0;

    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut adjwgt: *mut idx_t = 0 as *mut idx_t;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    let mut bndptr: *mut idx_t = 0 as *mut idx_t;
    let mut bndind: *mut idx_t = 0 as *mut idx_t;
    let mut myrinfo: *mut ckrinfo_t = 0 as *mut ckrinfo_t;
    let mut mynbrs: *mut cnbr_t = 0 as *mut cnbr_t;
    nvtxs = (*graph).nvtxs;
    let mut xadj = &mut (*graph).xadj;
    adjncy = (*graph).adjncy;
    adjwgt = (*graph).adjwgt;
    where_0 = (*graph).where_0;
    bndptr = (*graph).bndptr;
    bndind = (*graph).bndind;
    nbnd = (*graph).nbnd;
    iii = *ptr.offset(gid as isize);
    while iii < *ptr.offset((gid + 1) as isize) {
        i = *ind.offset(iii as isize);
        from = *where_0.offset(i as isize);
        myrinfo = ((*graph).ckrinfo).offset(i as isize);
        if (*myrinfo).inbr == -(1) {
            (*myrinfo).inbr = libmetis__cnbrpoolGetNext(
                ctrl,
                xadj[(i + 1) as usize] - xadj[i as usize] + 1,
            );
            (*myrinfo).nnbrs = 0 as libc::c_int;
        }
        mynbrs = ((*ctrl).cnbrpool).offset((*myrinfo).inbr as isize);
        k = 0 as libc::c_int;
        while k < (*myrinfo).nnbrs {
            if (*mynbrs.offset(k as isize)).pid == to {
                break;
            }
            k += 1;
            k;
        }
        if k == (*myrinfo).nnbrs {
            (*mynbrs.offset(k as isize)).pid = to;
            (*mynbrs.offset(k as isize)).ed = 0 as libc::c_int;
            (*myrinfo).nnbrs += 1;
            (*myrinfo).nnbrs;
        }
        (*graph).mincut -= (*mynbrs.offset(k as isize)).ed - (*myrinfo).id;
        libmetis__iaxpy(
            (*graph).ncon as size_t,
            1,
            ((*graph).vwgt).offset((i * (*graph).ncon) as isize),
            1 as size_t,
            ((*graph).pwgts).offset((to * (*graph).ncon) as isize),
            1 as size_t,
        );
        libmetis__iaxpy(
            (*graph).ncon as size_t,
            -(1),
            ((*graph).vwgt).offset((i * (*graph).ncon) as isize),
            1 as size_t,
            ((*graph).pwgts).offset((from * (*graph).ncon) as isize),
            1 as size_t,
        );
        *where_0.offset(i as isize) = to;
        (*myrinfo).ed += (*myrinfo).id - (*mynbrs.offset(k as isize)).ed;
        j = (*myrinfo).id;
        (*myrinfo).id = (*mynbrs.offset(k as isize)).ed;
        (*mynbrs.offset(k as isize)).ed = j;
        if (*mynbrs.offset(k as isize)).ed == 0 as libc::c_int {
            (*myrinfo).nnbrs -= 1;
            *mynbrs.offset(k as isize) = *mynbrs.offset((*myrinfo).nnbrs as isize);
        } else {
            (*mynbrs.offset(k as isize)).pid = from;
        }
        if 1 == 1 {
            if *bndptr.offset(i as isize) != -(1)
                && (*myrinfo).ed - (*myrinfo).id < 0 as libc::c_int
            {
                nbnd -= 1;
                *bndind.offset(*bndptr.offset(i as isize) as isize) = *bndind.offset(nbnd as isize);
                *bndptr.offset(*bndind.offset(nbnd as isize) as isize) = *bndptr.offset(i as isize);
                *bndptr.offset(i as isize) = -(1);
            }
            if *bndptr.offset(i as isize) == -(1)
                && (*myrinfo).ed - (*myrinfo).id >= 0 as libc::c_int
            {
                *bndind.offset(nbnd as isize) = i;
                let fresh19 = nbnd;
                nbnd = nbnd + 1;
                *bndptr.offset(i as isize) = fresh19;
            }
        } else {
            if *bndptr.offset(i as isize) != -(1)
                && (*myrinfo).ed <= 0 as libc::c_int
            {
                nbnd -= 1;
                *bndind.offset(*bndptr.offset(i as isize) as isize) = *bndind.offset(nbnd as isize);
                *bndptr.offset(*bndind.offset(nbnd as isize) as isize) = *bndptr.offset(i as isize);
                *bndptr.offset(i as isize) = -(1);
            }
            if *bndptr.offset(i as isize) == -(1) && (*myrinfo).ed > 0 as libc::c_int
            {
                *bndind.offset(nbnd as isize) = i;
                let fresh20 = nbnd;
                nbnd = nbnd + 1;
                *bndptr.offset(i as isize) = fresh20;
            }
        }
        j = xadj[i as usize];
        while j < xadj[(i + 1) as usize] {
            ii = *adjncy.offset(j as isize);
            me = *where_0.offset(ii as isize);
            myrinfo = ((*graph).ckrinfo).offset(ii as isize);
            let mut k_0: idx_t = 0;
            let mut mynbrs_0: *mut cnbr_t = 0 as *mut cnbr_t;
            if (*myrinfo).inbr == -(1) {
                (*myrinfo).inbr = libmetis__cnbrpoolGetNext(
                    ctrl,
                    xadj[((ii + 1) as usize)] - xadj[(ii as usize)]
                        + 1,
                );
                (*myrinfo).nnbrs = 0 as libc::c_int;
            }
            mynbrs_0 = ((*ctrl).cnbrpool).offset((*myrinfo).inbr as isize);
            if me == from {
                (*myrinfo).ed += *adjwgt.offset(j as isize);
                (*myrinfo).id -= *adjwgt.offset(j as isize);
                if 1 == 1 {
                    if (*myrinfo).ed - (*myrinfo).id >= 0 as libc::c_int
                        && *bndptr.offset(ii as isize) == -(1)
                    {
                        *bndind.offset(nbnd as isize) = ii;
                        let fresh21 = nbnd;
                        nbnd = nbnd + 1;
                        *bndptr.offset(ii as isize) = fresh21;
                    }
                } else if (*myrinfo).ed > 0 as libc::c_int
                    && *bndptr.offset(ii as isize) == -(1)
                {
                    *bndind.offset(nbnd as isize) = ii;
                    let fresh22 = nbnd;
                    nbnd = nbnd + 1;
                    *bndptr.offset(ii as isize) = fresh22;
                }
            } else if me == to {
                (*myrinfo).id += *adjwgt.offset(j as isize);
                (*myrinfo).ed -= *adjwgt.offset(j as isize);
                if 1 == 1 {
                    if (*myrinfo).ed - (*myrinfo).id < 0 as libc::c_int
                        && *bndptr.offset(ii as isize) != -(1)
                    {
                        nbnd -= 1;
                        *bndind.offset(*bndptr.offset(ii as isize) as isize) =
                            *bndind.offset(nbnd as isize);
                        *bndptr.offset(*bndind.offset(nbnd as isize) as isize) =
                            *bndptr.offset(ii as isize);
                        *bndptr.offset(ii as isize) = -(1);
                    }
                } else if (*myrinfo).ed <= 0 as libc::c_int
                    && *bndptr.offset(ii as isize) != -(1)
                {
                    nbnd -= 1;
                    *bndind.offset(*bndptr.offset(ii as isize) as isize) =
                        *bndind.offset(nbnd as isize);
                    *bndptr.offset(*bndind.offset(nbnd as isize) as isize) =
                        *bndptr.offset(ii as isize);
                    *bndptr.offset(ii as isize) = -(1);
                }
            }
            if me != from {
                k_0 = 0 as libc::c_int;
                while k_0 < (*myrinfo).nnbrs {
                    if (*mynbrs_0.offset(k_0 as isize)).pid == from {
                        if (*mynbrs_0.offset(k_0 as isize)).ed == *adjwgt.offset(j as isize) {
                            (*myrinfo).nnbrs -= 1;
                            *mynbrs_0.offset(k_0 as isize) =
                                *mynbrs_0.offset((*myrinfo).nnbrs as isize);
                        } else {
                            let ref mut fresh23 = (*mynbrs_0.offset(k_0 as isize)).ed;
                            *fresh23 -= *adjwgt.offset(j as isize);
                        }
                        break;
                    } else {
                        k_0 += 1;
                        k_0;
                    }
                }
            }
            if me != to {
                k_0 = 0 as libc::c_int;
                while k_0 < (*myrinfo).nnbrs {
                    if (*mynbrs_0.offset(k_0 as isize)).pid == to {
                        let ref mut fresh24 = (*mynbrs_0.offset(k_0 as isize)).ed;
                        *fresh24 += *adjwgt.offset(j as isize);
                        break;
                    } else {
                        k_0 += 1;
                        k_0;
                    }
                }
                if k_0 == (*myrinfo).nnbrs {
                    (*mynbrs_0.offset(k_0 as isize)).pid = to;
                    (*mynbrs_0.offset(k_0 as isize)).ed = *adjwgt.offset(j as isize);
                    (*myrinfo).nnbrs += 1;
                    (*myrinfo).nnbrs;
                }
            }
            j += 1;
            j;
        }
        iii += 1;
        iii;
    }
    (*graph).nbnd = nbnd;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__MoveGroupContigForVol(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut to: idx_t,
    mut gid: idx_t,
    mut ptr: *mut idx_t,
    mut ind: *mut idx_t,
    mut vmarker: *mut idx_t,
    mut pmarker: *mut idx_t,
    mut modind: *mut idx_t,
) {
    let mut i: idx_t = 0;
    let mut ii: idx_t = 0;
    let mut iii: idx_t = 0;
    let mut j: idx_t = 0;
    let mut jj: idx_t = 0;
    let mut k: idx_t = 0;
    let mut l: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut from: idx_t = 0;
    let mut me: idx_t = 0;
    let mut other: idx_t = 0;
    let mut xgain: idx_t = 0;

    let mut vsize: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    let mut myrinfo: *mut vkrinfo_t = 0 as *mut vkrinfo_t;
    let mut orinfo: *mut vkrinfo_t = 0 as *mut vkrinfo_t;
    let mut mynbrs: *mut vnbr_t = 0 as *mut vnbr_t;
    let mut onbrs: *mut vnbr_t = 0 as *mut vnbr_t;
    nvtxs = (*graph).nvtxs;
    let mut xadj = &mut (*graph).xadj;
    vsize = (*graph).vsize;
    adjncy = (*graph).adjncy;
    where_0 = (*graph).where_0;
    iii = *ptr.offset(gid as isize);
    while iii < *ptr.offset((gid + 1) as isize) {
        i = *ind.offset(iii as isize);
        from = *where_0.offset(i as isize);
        myrinfo = ((*graph).vkrinfo).offset(i as isize);
        if (*myrinfo).inbr == -(1) {
            (*myrinfo).inbr = libmetis__vnbrpoolGetNext(
                ctrl,
                xadj[(i + 1) as usize] - xadj[i as usize] + 1,
            );
            (*myrinfo).nnbrs = 0 as libc::c_int;
        }
        mynbrs = ((*ctrl).vnbrpool).offset((*myrinfo).inbr as isize);
        xgain = if (*myrinfo).nid == 0 as libc::c_int && (*myrinfo).ned > 0 as libc::c_int {
            *vsize.offset(i as isize)
        } else {
            0 as libc::c_int
        };
        k = 0 as libc::c_int;
        while k < (*myrinfo).nnbrs {
            if (*mynbrs.offset(k as isize)).pid == to {
                break;
            }
            k += 1;
            k;
        }
        if k == (*myrinfo).nnbrs {
            if (*myrinfo).nid > 0 as libc::c_int {
                xgain -= *vsize.offset(i as isize);
            }
            j = xadj[i as usize];
            while j < xadj[(i + 1) as usize] {
                ii = *adjncy.offset(j as isize);
                other = *where_0.offset(ii as isize);
                orinfo = ((*graph).vkrinfo).offset(ii as isize);
                onbrs = ((*ctrl).vnbrpool).offset((*orinfo).inbr as isize);
                if from == other {
                    l = 0 as libc::c_int;
                    while l < (*orinfo).nnbrs {
                        if (*onbrs.offset(l as isize)).pid == to {
                            break;
                        }
                        l += 1;
                        l;
                    }
                    if l == (*orinfo).nnbrs {
                        xgain -= *vsize.offset(ii as isize);
                    }
                } else {
                    l = 0 as libc::c_int;
                    while l < (*orinfo).nnbrs {
                        if (*onbrs.offset(l as isize)).pid == to {
                            break;
                        }
                        l += 1;
                        l;
                    }
                    if l == (*orinfo).nnbrs {
                        xgain -= *vsize.offset(ii as isize);
                    }
                    l = 0 as libc::c_int;
                    while l < (*orinfo).nnbrs {
                        if (*onbrs.offset(l as isize)).pid == from
                            && (*onbrs.offset(l as isize)).ned == 1
                        {
                            xgain += *vsize.offset(ii as isize);
                            break;
                        } else {
                            l += 1;
                            l;
                        }
                    }
                }
                j += 1;
                j;
            }
            (*graph).minvol -= xgain;
            (*graph).mincut -= -(*myrinfo).nid;
        } else {
            (*graph).minvol -= xgain + (*mynbrs.offset(k as isize)).gv;
            (*graph).mincut -= (*mynbrs.offset(k as isize)).ned - (*myrinfo).nid;
        }
        *where_0.offset(i as isize) = to;
        libmetis__iaxpy(
            (*graph).ncon as size_t,
            1,
            ((*graph).vwgt).offset((i * (*graph).ncon) as isize),
            1 as size_t,
            ((*graph).pwgts).offset((to * (*graph).ncon) as isize),
            1 as size_t,
        );
        libmetis__iaxpy(
            (*graph).ncon as size_t,
            -(1),
            ((*graph).vwgt).offset((i * (*graph).ncon) as isize),
            1 as size_t,
            ((*graph).pwgts).offset((from * (*graph).ncon) as isize),
            1 as size_t,
        );
        libmetis__KWayVolUpdate(
            ctrl,
            graph,
            i,
            from,
            to,
            0 as *mut ipq_t,
            0 as *mut idx_t,
            0 as *mut idx_t,
            0 as *mut idx_t,
            0 as *mut idx_t,
            1,
            vmarker,
            pmarker,
            modind,
        );
        iii += 1;
        iii;
    }
}
