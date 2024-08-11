use ::libc;

use crate::GKlib::timers::gk_CPUSeconds;

use super::{
    auxapi::*, balance::*, coarsen::*, contig::*, fm::*, fortran::*, gklib::*, graph::*,
    kwayrefine::*, options::*, structure::*, timing::*, util::*, wspace::*,
};

#[no_mangle]
pub unsafe extern "C" fn libmetis__Refine2Way(
    mut ctrl: *mut ctrl_t,
    mut orggraph: *mut graph_t,
    mut graph: *mut graph_t,
    mut tpwgts: *mut real_t,
) {
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint != 0 {
        (*ctrl).UncoarsenTmr -= gk_CPUSeconds();
    }
    libmetis__Compute2WayPartitionParams(ctrl, graph);
    loop {
        if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint != 0 {
            (*ctrl).RefTmr -= gk_CPUSeconds();
        }
        libmetis__Balance2Way(ctrl, graph, tpwgts);
        libmetis__FM_2WayRefine(ctrl, graph, tpwgts, (*ctrl).niter);
        if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint != 0 {
            (*ctrl).RefTmr += gk_CPUSeconds();
        }
        if graph == orggraph {
            break;
        }
        graph = (*graph).finer;
        if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint != 0 {
            (*ctrl).ProjectTmr -= gk_CPUSeconds();
        }
        libmetis__Project2WayPartition(ctrl, graph);
        if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint != 0 {
            (*ctrl).ProjectTmr += gk_CPUSeconds();
        }
    }
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint != 0 {
        (*ctrl).UncoarsenTmr += gk_CPUSeconds();
    }
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__Allocate2WayPartitionMemory(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
) {
    let mut nvtxs: idx_t = 0;
    let mut ncon: idx_t = 0;
    nvtxs = (*graph).nvtxs;
    ncon = (*graph).ncon;
    (*graph).pwgts = libmetis__imalloc(
        (2 as libc::c_int * ncon) as size_t,
        b"Allocate2WayPartitionMemory: pwgts\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*graph).where_0 = libmetis__imalloc(
        nvtxs as size_t,
        b"Allocate2WayPartitionMemory: where\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*graph).bndptr = libmetis__imalloc(
        nvtxs as size_t,
        b"Allocate2WayPartitionMemory: bndptr\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*graph).bndind = libmetis__imalloc(
        nvtxs as size_t,
        b"Allocate2WayPartitionMemory: bndind\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*graph).id = libmetis__imalloc(
        nvtxs as size_t,
        b"Allocate2WayPartitionMemory: id\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*graph).ed = libmetis__imalloc(
        nvtxs as size_t,
        b"Allocate2WayPartitionMemory: ed\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__Compute2WayPartitionParams(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
) {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut ncon: idx_t = 0;
    let mut nbnd: idx_t = 0;
    let mut mincut: idx_t = 0;
    let mut istart: idx_t = 0;
    let mut iend: idx_t = 0;
    let mut tid: idx_t = 0;
    let mut ted: idx_t = 0;
    let mut me: idx_t = 0;
    let mut vwgt: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut adjwgt: *mut idx_t = 0 as *mut idx_t;
    let mut pwgts: *mut idx_t = 0 as *mut idx_t;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    let mut bndptr: *mut idx_t = 0 as *mut idx_t;
    let mut bndind: *mut idx_t = 0 as *mut idx_t;
    let mut id: *mut idx_t = 0 as *mut idx_t;
    let mut ed: *mut idx_t = 0 as *mut idx_t;
    nvtxs = (*graph).nvtxs;
    ncon = (*graph).ncon;
    let mut xadj = &mut (*graph).xadj;
    vwgt = (*graph).vwgt;
    adjncy = (*graph).adjncy;
    adjwgt = (*graph).adjwgt;
    where_0 = (*graph).where_0;
    id = (*graph).id;
    ed = (*graph).ed;
    pwgts = libmetis__iset(
        (2 as libc::c_int * ncon) as size_t,
        0 as libc::c_int,
        (*graph).pwgts,
    );
    bndptr = libmetis__iset(nvtxs as size_t, -(1), (*graph).bndptr);
    bndind = (*graph).bndind;
    if ncon == 1 {
        i = 0 as libc::c_int;
        while i < nvtxs {
            let ref mut fresh0 = *pwgts.offset(*where_0.offset(i as isize) as isize);
            *fresh0 += *vwgt.offset(i as isize);
            i += 1;
            i;
        }
    } else {
        i = 0 as libc::c_int;
        while i < nvtxs {
            me = *where_0.offset(i as isize);
            j = 0 as libc::c_int;
            while j < ncon {
                let ref mut fresh1 = *pwgts.offset((me * ncon + j) as isize);
                *fresh1 += *vwgt.offset((i * ncon + j) as isize);
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    }
    nbnd = 0 as libc::c_int;
    mincut = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < nvtxs {
        istart = xadj[i as usize];
        iend = xadj[(i + 1) as usize];
        me = *where_0.offset(i as isize);
        ted = 0 as libc::c_int;
        tid = ted;
        j = istart;
        while j < iend {
            if me == *where_0.offset(*adjncy.offset(j as isize) as isize) {
                tid += *adjwgt.offset(j as isize);
            } else {
                ted += *adjwgt.offset(j as isize);
            }
            j += 1;
            j;
        }
        *id.offset(i as isize) = tid;
        *ed.offset(i as isize) = ted;
        if ted > 0 as libc::c_int || istart == iend {
            *bndind.offset(nbnd as isize) = i;
            let fresh2 = nbnd;
            nbnd = nbnd + 1;
            *bndptr.offset(i as isize) = fresh2;
            mincut += ted;
        }
        i += 1;
        i;
    }
    (*graph).mincut = mincut / 2 as libc::c_int;
    (*graph).nbnd = nbnd;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__Project2WayPartition(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
) {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut istart: idx_t = 0;
    let mut iend: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut nbnd: idx_t = 0;
    let mut me: idx_t = 0;
    let mut tid: idx_t = 0;
    let mut ted: idx_t = 0;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut adjwgt: *mut idx_t = 0 as *mut idx_t;
    let mut cmap: *mut idx_t = 0 as *mut idx_t;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    let mut bndptr: *mut idx_t = 0 as *mut idx_t;
    let mut bndind: *mut idx_t = 0 as *mut idx_t;
    let mut cwhere: *mut idx_t = 0 as *mut idx_t;
    let mut cbndptr: *mut idx_t = 0 as *mut idx_t;
    let mut id: *mut idx_t = 0 as *mut idx_t;
    let mut ed: *mut idx_t = 0 as *mut idx_t;
    let mut cgraph: *mut graph_t = 0 as *mut graph_t;
    libmetis__Allocate2WayPartitionMemory(ctrl, graph);
    cgraph = (*graph).coarser;
    cwhere = (*cgraph).where_0;
    cbndptr = (*cgraph).bndptr;
    nvtxs = (*graph).nvtxs;
    cmap = (*graph).cmap;
    let mut xadj = &mut (*graph).xadj;
    adjncy = (*graph).adjncy;
    adjwgt = (*graph).adjwgt;
    where_0 = (*graph).where_0;
    id = (*graph).id;
    ed = (*graph).ed;
    bndptr = libmetis__iset(nvtxs as size_t, -(1), (*graph).bndptr);
    bndind = (*graph).bndind;
    i = 0 as libc::c_int;
    while i < nvtxs {
        j = *cmap.offset(i as isize);
        *where_0.offset(i as isize) = *cwhere.offset(j as isize);
        *cmap.offset(i as isize) = *cbndptr.offset(j as isize);
        i += 1;
        i;
    }
    nbnd = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < nvtxs {
        istart = xadj[i as usize];
        iend = xadj[(i + 1) as usize];
        ted = 0 as libc::c_int;
        tid = ted;
        if *cmap.offset(i as isize) == -(1) {
            j = istart;
            while j < iend {
                tid += *adjwgt.offset(j as isize);
                j += 1;
                j;
            }
        } else {
            me = *where_0.offset(i as isize);
            j = istart;
            while j < iend {
                if me == *where_0.offset(*adjncy.offset(j as isize) as isize) {
                    tid += *adjwgt.offset(j as isize);
                } else {
                    ted += *adjwgt.offset(j as isize);
                }
                j += 1;
                j;
            }
        }
        *id.offset(i as isize) = tid;
        *ed.offset(i as isize) = ted;
        if ted > 0 as libc::c_int || istart == iend {
            *bndind.offset(nbnd as isize) = i;
            let fresh3 = nbnd;
            nbnd = nbnd + 1;
            *bndptr.offset(i as isize) = fresh3;
        }
        i += 1;
        i;
    }
    (*graph).mincut = (*cgraph).mincut;
    (*graph).nbnd = nbnd;
    libmetis__icopy(
        (2 as libc::c_int * (*graph).ncon) as size_t,
        (*cgraph).pwgts,
        (*graph).pwgts,
    );
    libmetis__FreeGraph(&mut (*graph).coarser);
    (*graph).coarser = 0 as *mut graph_t;
}
