use ::libc;
use libc::printf;

use crate::GKlib::{
    error::{__jmp_buf_tag, gk_jbufs, gk_sigtrap, gk_siguntrap, GK_CUR_JBUFS},
    memory::{gk_free, gk_malloc, gk_malloc_cleanup, gk_malloc_init},
    timers::gk_CPUSeconds,
};

use super::{
    coarsen::{libmetis__CoarsenGraph, CoarsenGraphNlevels},
    compress::{libmetis__CompressGraph, libmetis__PruneGraph},
    contig::libmetis__FindSepInducedComponents,
    fortran::{libmetis__Change2CNumbering, libmetis__Change2FNumberingOrder},
    gklib::{libmetis__icopy, libmetis__imalloc, libmetis__irandArrayPermute, libmetis__iset},
    graph::{
        libmetis__FreeGraph, libmetis__FreeRData, libmetis__SetupGraph, libmetis__SetupGraph_tvwgt,
        libmetis__SetupSplitGraph,
    },
    initpart::libmetis__InitSeparator,
    mmd::libmetis__genmmd,
    options::{libmetis__FreeCtrl, libmetis__SetupCtrl},
    srefine::{libmetis__Compute2WayNodePartitionParams, libmetis__Refine2WayNode},
    structure::*,
    timing::{libmetis__InitTimers, libmetis__PrintTimers},
    util::libmetis__metis_rcode,
    wspace::{
        libmetis__AllocateWorkSpace, libmetis__iwspacemalloc, libmetis__wspacepop,
        libmetis__wspacepush,
    },
};

pub type __int32_t = libc::c_int;
pub type __ssize_t = i64;
pub type int32_t = __int32_t;
pub type ssize_t = __ssize_t;
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [u64; 16],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct gk_mop_t {
    pub type_0: libc::c_int,
    pub nbytes: ssize_t,
    pub ptr: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gk_mcore_t {
    pub coresize: size_t,
    pub corecpos: size_t,
    pub core: *mut libc::c_void,
    pub nmops: size_t,
    pub cmop: size_t,
    pub mops: *mut gk_mop_t,
    pub num_callocs: size_t,
    pub num_hallocs: size_t,
    pub size_callocs: size_t,
    pub size_hallocs: size_t,
    pub cur_callocs: size_t,
    pub cur_hallocs: size_t,
    pub max_callocs: size_t,
    pub max_hallocs: size_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct vnbr_t {
    pub pid: idx_t,
    pub ned: idx_t,
    pub gv: idx_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cnbr_t {
    pub pid: idx_t,
    pub ed: idx_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct nrinfo_t {
    pub edegrees: [idx_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vkrinfo_t {
    pub nid: idx_t,
    pub ned: idx_t,
    pub gv: idx_t,
    pub nnbrs: idx_t,
    pub inbr: idx_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ckrinfo_t {
    pub id: idx_t,
    pub ed: idx_t,
    pub nnbrs: idx_t,
    pub inbr: idx_t,
}
pub const METIS_ERROR_INPUT: C2RustUnnamed = -2;
pub const METIS_ERROR_MEMORY: C2RustUnnamed = -3;
pub type C2RustUnnamed = libc::c_int;
pub const METIS_ERROR: C2RustUnnamed = -4;
pub const METIS_OK: C2RustUnnamed = 1;
#[no_mangle]
pub unsafe extern "C" fn METIS_NodeND(
    mut nvtxs: *mut idx_t,
    mut xadj: &mut Vec<idx_t>,
    mut adjncy: *mut idx_t,
    mut vwgt: *mut idx_t,
    mut options: *mut idx_t,
    mut perm: *mut idx_t,
    mut iperm: *mut idx_t,
) -> libc::c_int {
    let mut sigrval: libc::c_int = 0 as libc::c_int;
    let mut renumber: libc::c_int = 0 as libc::c_int;
    let mut i: idx_t = 0;
    let mut ii: idx_t = 0;
    let mut j: idx_t = 0;
    let mut l: idx_t = 0;
    let mut nnvtxs: idx_t = 0 as libc::c_int;
    let mut graph: *mut graph_t = 0 as *mut graph_t;
    let mut ctrl: *mut ctrl_t = 0 as *mut ctrl_t;
    let mut cptr: *mut idx_t = 0 as *mut idx_t;
    let mut cind: *mut idx_t = 0 as *mut idx_t;
    let mut piperm: *mut idx_t = 0 as *mut idx_t;
    let mut numflag: libc::c_int = 0 as libc::c_int;
    if gk_malloc_init() == 0 {
        return METIS_ERROR_MEMORY as libc::c_int;
    }
    gk_sigtrap();
    sigrval = 0; // _setjmp((*gk_jbufs.as_mut_ptr().offset(GK_CUR_JBUFS as isize)).as_mut_ptr());
    if !(sigrval != 0 as libc::c_int) {
        ctrl = libmetis__SetupCtrl(
            METIS_OP_OMETIS,
            options,
            1,
            3 as libc::c_int,
            0 as *mut real_t,
            0 as *mut real_t,
        );
        if ctrl.is_null() {
            gk_siguntrap();
            return METIS_ERROR_INPUT as libc::c_int;
        }
        if (*ctrl).numflag == 1 {
            libmetis__Change2CNumbering(*nvtxs, xadj, adjncy);
            renumber = 1;
        }
        if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint != 0 {
            libmetis__InitTimers(ctrl);
        }
        if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint != 0 {
            (*ctrl).TotalTmr -= gk_CPUSeconds();
        }
        if (*ctrl).pfactor as libc::c_double > 0.0f64 {
            piperm = libmetis__imalloc(
                *nvtxs as size_t,
                b"OMETIS: piperm\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            graph = libmetis__PruneGraph(ctrl, *nvtxs, xadj, adjncy, vwgt, piperm, (*ctrl).pfactor);
            if graph.is_null() {
                gk_free(
                    &mut piperm as *mut *mut idx_t as *mut *mut libc::c_void,
                    0 as *mut *mut libc::c_void,
                );
                (*ctrl).pfactor = 0.0f64 as real_t;
            } else {
                nnvtxs = (*graph).nvtxs;
                (*ctrl).compress = 0 as libc::c_int;
            }
        }
        if (*ctrl).compress != 0 {
            cptr = libmetis__imalloc(
                (*nvtxs + 1) as size_t,
                b"OMETIS: cptr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            cind = libmetis__imalloc(
                *nvtxs as size_t,
                b"OMETIS: cind\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            graph = libmetis__CompressGraph(ctrl, *nvtxs, xadj, adjncy, vwgt, cptr, cind);
            if graph.is_null() {
                gk_free(
                    &mut cptr as *mut *mut idx_t as *mut *mut libc::c_void,
                    &mut cind as *mut *mut idx_t,
                    0 as *mut *mut libc::c_void,
                );
                (*ctrl).compress = 0 as libc::c_int;
            } else {
                nnvtxs = (*graph).nvtxs;
                (*ctrl).cfactor =
                    (1.0f64 * *nvtxs as libc::c_double / nnvtxs as libc::c_double) as real_t;
                if (*ctrl).cfactor as libc::c_double > 1.5f64 && (*ctrl).nseps == 1 {
                    (*ctrl).nseps = 2 as libc::c_int;
                }
            }
        }
        if (*ctrl).pfactor as libc::c_double == 0.0f64 && (*ctrl).compress == 0 as libc::c_int {
            graph = libmetis__SetupGraph(
                ctrl,
                *nvtxs,
                1,
                xadj,
                adjncy,
                vwgt,
                0 as *mut idx_t,
                0 as *mut idx_t,
            );
        }
        libmetis__AllocateWorkSpace(ctrl, graph);
        if (*ctrl).ccorder != 0 {
            libmetis__MlevelNestedDissectionCC(ctrl, graph, iperm, (*graph).nvtxs);
        } else {
            libmetis__MlevelNestedDissection(ctrl, graph, iperm, (*graph).nvtxs);
        }
        if (*ctrl).pfactor as libc::c_double > 0.0f64 {
            libmetis__icopy(nnvtxs as size_t, iperm, perm);
            i = 0 as libc::c_int;
            while i < nnvtxs {
                *iperm.offset(*piperm.offset(i as isize) as isize) = *perm.offset(i as isize);
                i += 1;
                i;
            }
            i = nnvtxs;
            while i < *nvtxs {
                *iperm.offset(*piperm.offset(i as isize) as isize) = i;
                i += 1;
                i;
            }
            gk_free(
                &mut piperm as *mut *mut idx_t as *mut *mut libc::c_void,
                0 as *mut *mut libc::c_void,
            );
        } else if (*ctrl).compress != 0 {
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
                while j < *cptr.offset((i + 1) as isize) {
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
        while i < *nvtxs {
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
    }
    if renumber != 0 {
        libmetis__Change2FNumberingOrder(*nvtxs, xadj, adjncy, perm, iperm);
    }
    gk_siguntrap();
    gk_malloc_cleanup(0 as libc::c_int);
    return libmetis__metis_rcode(sigrval);
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__MlevelNestedDissection(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut order: *mut idx_t,
    mut lastvtx: idx_t,
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
    libmetis__MlevelNodeBisectionMultiple(ctrl, graph);
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_SEPINFO as libc::c_int as libc::c_uint != 0 {
        printf(
            b"Nvtxs: %6d, [%6d %6d %6d]\n\0" as *const u8 as *const libc::c_char,
            (*graph).nvtxs,
            *((*graph).pwgts).offset(0 as libc::c_int as isize),
            *((*graph).pwgts).offset(1 as isize),
            *((*graph).pwgts).offset(2 as libc::c_int as isize),
        );
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
    if (*lgraph).nvtxs > 120 as libc::c_int && (*lgraph).nedges > 0 as libc::c_int {
        libmetis__MlevelNestedDissection(ctrl, lgraph, order, lastvtx - (*rgraph).nvtxs);
    } else {
        libmetis__MMDOrder(ctrl, lgraph, order, lastvtx - (*rgraph).nvtxs);
        libmetis__FreeGraph(&mut lgraph);
    }
    if (*rgraph).nvtxs > 120 as libc::c_int && (*rgraph).nedges > 0 as libc::c_int {
        libmetis__MlevelNestedDissection(ctrl, rgraph, order, lastvtx);
    } else {
        libmetis__MMDOrder(ctrl, rgraph, order, lastvtx);
        libmetis__FreeGraph(&mut rgraph);
    };
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__MlevelNestedDissectionCC(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut order: *mut idx_t,
    mut lastvtx: idx_t,
) {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut nbnd: idx_t = 0;
    let mut ncmps: idx_t = 0;
    let mut rnvtxs: idx_t = 0;
    let mut snvtxs: idx_t = 0;
    let mut label: *mut idx_t = 0 as *mut idx_t;
    let mut bndind: *mut idx_t = 0 as *mut idx_t;
    let mut cptr: *mut idx_t = 0 as *mut idx_t;
    let mut cind: *mut idx_t = 0 as *mut idx_t;
    let mut sgraphs: *mut *mut graph_t = 0 as *mut *mut graph_t;
    nvtxs = (*graph).nvtxs;
    libmetis__MlevelNodeBisectionMultiple(ctrl, graph);
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_SEPINFO as libc::c_int as libc::c_uint != 0 {
        printf(
            b"Nvtxs: %6d, [%6d %6d %6d]\n\0" as *const u8 as *const libc::c_char,
            (*graph).nvtxs,
            *((*graph).pwgts).offset(0 as libc::c_int as isize),
            *((*graph).pwgts).offset(1 as isize),
            *((*graph).pwgts).offset(2 as libc::c_int as isize),
        );
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
    libmetis__wspacepush(ctrl);
    cptr = libmetis__iwspacemalloc(ctrl, nvtxs + 1);
    cind = libmetis__iwspacemalloc(ctrl, nvtxs);
    ncmps = libmetis__FindSepInducedComponents(ctrl, graph, cptr, cind);
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_INFO as libc::c_int as libc::c_uint != 0 {
        if ncmps > 2 as libc::c_int {
            printf(
                b"  Bisection resulted in %d connected components\n\0" as *const u8
                    as *const libc::c_char,
                ncmps,
            );
        }
    }
    sgraphs = libmetis__SplitGraphOrderCC(ctrl, graph, ncmps, cptr, cind);
    libmetis__wspacepop(ctrl);
    libmetis__FreeGraph(&mut graph);
    i = 0 as libc::c_int;
    rnvtxs = i;
    while i < ncmps {
        snvtxs = (**sgraphs.offset(i as isize)).nvtxs;
        if (**sgraphs.offset(i as isize)).nvtxs > 120 as libc::c_int
            && (**sgraphs.offset(i as isize)).nedges > 0 as libc::c_int
        {
            libmetis__MlevelNestedDissectionCC(
                ctrl,
                *sgraphs.offset(i as isize),
                order,
                lastvtx - rnvtxs,
            );
        } else {
            libmetis__MMDOrder(ctrl, *sgraphs.offset(i as isize), order, lastvtx - rnvtxs);
            libmetis__FreeGraph(&mut *sgraphs.offset(i as isize));
        }
        rnvtxs += snvtxs;
        i += 1;
        i;
    }
    gk_free(
        &mut sgraphs as *mut *mut *mut graph_t as *mut *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__MlevelNodeBisectionMultiple(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
) {
    let mut i: idx_t = 0;
    let mut mincut: idx_t = 0;
    let mut bestwhere: *mut idx_t = 0 as *mut idx_t;
    if (*ctrl).nseps == 1
        || (*graph).nvtxs
            < (if (*ctrl).compress != 0 {
                1000 as libc::c_int
            } else {
                2000 as libc::c_int
            })
    {
        libmetis__MlevelNodeBisectionL2(ctrl, graph, 7 as libc::c_int);
        return;
    }
    libmetis__wspacepush(ctrl);
    bestwhere = libmetis__iwspacemalloc(ctrl, (*graph).nvtxs);
    mincut = *((*graph).tvwgt).offset(0 as libc::c_int as isize);
    i = 0 as libc::c_int;
    while i < (*ctrl).nseps {
        libmetis__MlevelNodeBisectionL2(ctrl, graph, 7 as libc::c_int);
        if i == 0 as libc::c_int || (*graph).mincut < mincut {
            mincut = (*graph).mincut;
            if i < (*ctrl).nseps - 1 {
                libmetis__icopy((*graph).nvtxs as size_t, (*graph).where_0, bestwhere);
            }
        }
        if mincut == 0 as libc::c_int {
            break;
        }
        if i < (*ctrl).nseps - 1 {
            libmetis__FreeRData(graph);
        }
        i += 1;
        i;
    }
    if mincut != (*graph).mincut {
        libmetis__icopy((*graph).nvtxs as size_t, bestwhere, (*graph).where_0);
        libmetis__Compute2WayNodePartitionParams(ctrl, graph);
    }
    libmetis__wspacepop(ctrl);
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__MlevelNodeBisectionL2(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut niparts: idx_t,
) {
    let mut i: idx_t = 0;
    let mut mincut: idx_t = 0;
    let mut nruns: idx_t = 5 as libc::c_int;
    let mut cgraph: *mut graph_t = 0 as *mut graph_t;
    let mut bestwhere: *mut idx_t = 0 as *mut idx_t;
    if (*graph).nvtxs < 5000 as libc::c_int {
        libmetis__MlevelNodeBisectionL1(ctrl, graph, niparts);
        return;
    }
    libmetis__wspacepush(ctrl);
    (*ctrl).CoarsenTo = if 100 as libc::c_int >= (*graph).nvtxs / 30 as libc::c_int {
        100 as libc::c_int
    } else {
        (*graph).nvtxs / 30 as libc::c_int
    };
    cgraph = CoarsenGraphNlevels(ctrl, graph, 4 as libc::c_int);
    bestwhere = libmetis__iwspacemalloc(ctrl, (*cgraph).nvtxs);
    mincut = *((*graph).tvwgt).offset(0 as libc::c_int as isize);
    i = 0 as libc::c_int;
    while i < nruns {
        libmetis__MlevelNodeBisectionL1(
            ctrl,
            cgraph,
            (0.7f64 * niparts as libc::c_double) as idx_t,
        );
        if i == 0 as libc::c_int || (*cgraph).mincut < mincut {
            mincut = (*cgraph).mincut;
            if i < nruns - 1 {
                libmetis__icopy((*cgraph).nvtxs as size_t, (*cgraph).where_0, bestwhere);
            }
        }
        if mincut == 0 as libc::c_int {
            break;
        }
        if i < nruns - 1 {
            libmetis__FreeRData(cgraph);
        }
        i += 1;
        i;
    }
    if mincut != (*cgraph).mincut {
        libmetis__icopy((*cgraph).nvtxs as size_t, bestwhere, (*cgraph).where_0);
    }
    libmetis__wspacepop(ctrl);
    libmetis__Refine2WayNode(ctrl, graph, cgraph);
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__MlevelNodeBisectionL1(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut niparts: idx_t,
) {
    let mut cgraph: *mut graph_t = 0 as *mut graph_t;
    (*ctrl).CoarsenTo = (*graph).nvtxs / 8 as libc::c_int;
    if (*ctrl).CoarsenTo > 100 as libc::c_int {
        (*ctrl).CoarsenTo = 100 as libc::c_int;
    } else if (*ctrl).CoarsenTo < 40 as libc::c_int {
        (*ctrl).CoarsenTo = 40 as libc::c_int;
    }
    cgraph = libmetis__CoarsenGraph(ctrl, graph);
    niparts = if 1
        >= (if (*cgraph).nvtxs <= (*ctrl).CoarsenTo {
            niparts / 2 as libc::c_int
        } else {
            niparts
        }) {
        1
    } else if (*cgraph).nvtxs <= (*ctrl).CoarsenTo {
        niparts / 2 as libc::c_int
    } else {
        niparts
    };
    libmetis__InitSeparator(ctrl, cgraph, niparts);
    libmetis__Refine2WayNode(ctrl, graph, cgraph);
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__SplitGraphOrder(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut r_lgraph: *mut *mut graph_t,
    mut r_rgraph: *mut *mut graph_t,
) {
    let mut i: idx_t = 0;
    let mut ii: idx_t = 0;
    let mut j: idx_t = 0;
    let mut k: idx_t = 0;
    let mut l: idx_t = 0;
    let mut istart: idx_t = 0;
    let mut iend: idx_t = 0;
    let mut mypart: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut snvtxs: [idx_t; 3] = [0; 3];
    let mut snedges: [idx_t; 3] = [0; 3];

    let mut vwgt: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut adjwgt: *mut idx_t = 0 as *mut idx_t;
    let mut label: *mut idx_t = 0 as *mut idx_t;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    let mut bndptr: *mut idx_t = 0 as *mut idx_t;
    let mut bndind: *mut idx_t = 0 as *mut idx_t;
    let mut sxadj: [&mut Vec<idx_t>; 2];
    let mut svwgt: [*mut idx_t; 2] = [0 as *mut idx_t; 2];
    let mut sadjncy: [*mut idx_t; 2] = [0 as *mut idx_t; 2];
    let mut sadjwgt: [*mut idx_t; 2] = [0 as *mut idx_t; 2];
    let mut slabel: [*mut idx_t; 2] = [0 as *mut idx_t; 2];
    let mut rename: *mut idx_t = 0 as *mut idx_t;
    let mut auxadjncy: *mut idx_t = 0 as *mut idx_t;
    let mut lgraph: *mut graph_t = 0 as *mut graph_t;
    let mut rgraph: *mut graph_t = 0 as *mut graph_t;
    libmetis__wspacepush(ctrl);
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint != 0 {
        (*ctrl).SplitTmr -= gk_CPUSeconds();
    }
    nvtxs = (*graph).nvtxs;
    let mut xadj = &mut (*graph).xadj;
    vwgt = (*graph).vwgt;
    adjncy = (*graph).adjncy;
    adjwgt = (*graph).adjwgt;
    label = (*graph).label;
    where_0 = (*graph).where_0;
    bndptr = (*graph).bndptr;
    bndind = (*graph).bndind;
    rename = libmetis__iwspacemalloc(ctrl, nvtxs);
    snedges[2 as libc::c_int as usize] = 0 as libc::c_int;
    snedges[1] = snedges[2 as libc::c_int as usize];
    snedges[0] = snedges[1];
    snvtxs[2 as libc::c_int as usize] = snedges[0];
    snvtxs[1] = snvtxs[2 as libc::c_int as usize];
    snvtxs[0] = snvtxs[1];
    i = 0 as libc::c_int;
    while i < nvtxs {
        k = *where_0.offset(i as isize);
        let fresh1 = snvtxs[k as usize];
        snvtxs[k as usize] = snvtxs[k as usize] + 1;
        *rename.offset(i as isize) = fresh1;
        snedges[k as usize] += xadj[(i + 1) as usize] - xadj[i as usize];
        i += 1;
        i;
    }
    lgraph = libmetis__SetupSplitGraph(graph, snvtxs[0], snedges[0]);

    rgraph = libmetis__SetupSplitGraph(graph, snvtxs[1], snedges[1]);
    sxadj = [&mut (*lgraph).xadj, &mut (*rgraph).xadj];

    svwgt[0] = (*lgraph).vwgt;
    sadjncy[0] = (*lgraph).adjncy;
    sadjwgt[0] = (*lgraph).adjwgt;
    slabel[0] = (*lgraph).label;

    svwgt[1] = (*rgraph).vwgt;
    sadjncy[1] = (*rgraph).adjncy;
    sadjwgt[1] = (*rgraph).adjwgt;
    slabel[1] = (*rgraph).label;

    ii = 0 as libc::c_int;
    while ii < (*graph).nbnd {
        i = *bndind.offset(ii as isize);
        j = xadj[i as usize];
        while j < xadj[(i + 1) as usize] {
            *bndptr.offset(*adjncy.offset(j as isize) as isize) = 1;
            j += 1;
            j;
        }
        ii += 1;
        ii;
    }
    snedges[1] = 0 as libc::c_int;
    snedges[0] = snedges[1];
    snvtxs[1] = snedges[0];
    snvtxs[0] = snvtxs[1];
    let ref mut fresh2 = (sxadj[1])[0];
    *fresh2 = 0 as libc::c_int;
    (sxadj[0])[0] = *fresh2;
    i = 0 as libc::c_int;
    while i < nvtxs {
        mypart = *where_0.offset(i as isize);
        if !(mypart == 2 as libc::c_int) {
            istart = xadj[i as usize];
            iend = xadj[(i + 1) as usize];
            if *bndptr.offset(i as isize) == -(1) {
                auxadjncy = (sadjncy[mypart as usize])
                    .offset(snedges[mypart as usize] as isize)
                    .offset(-(istart as isize));
                j = istart;
                while j < iend {
                    *auxadjncy.offset(j as isize) = *adjncy.offset(j as isize);
                    j += 1;
                    j;
                }
                snedges[mypart as usize] += iend - istart;
            } else {
                auxadjncy = sadjncy[mypart as usize];
                l = snedges[mypart as usize];
                j = istart;
                while j < iend {
                    k = *adjncy.offset(j as isize);
                    if *where_0.offset(k as isize) == mypart {
                        let fresh3 = l;
                        l = l + 1;
                        *auxadjncy.offset(fresh3 as isize) = k;
                    }
                    j += 1;
                    j;
                }
                snedges[mypart as usize] = l;
            }
            *(svwgt[mypart as usize]).offset(snvtxs[mypart as usize] as isize) =
                *vwgt.offset(i as isize);
            *(slabel[mypart as usize]).offset(snvtxs[mypart as usize] as isize) =
                *label.offset(i as isize);
            snvtxs[mypart as usize] += 1;
            (sxadj[mypart as usize])[(snvtxs[mypart as usize] as usize)] = snedges[mypart as usize];
        }
        i += 1;
        i;
    }
    mypart = 0 as libc::c_int;
    while mypart < 2 as libc::c_int {
        iend = snedges[mypart as usize];
        libmetis__iset(iend as size_t, 1, sadjwgt[mypart as usize]);
        auxadjncy = sadjncy[mypart as usize];
        i = 0 as libc::c_int;
        while i < iend {
            *auxadjncy.offset(i as isize) = *rename.offset(*auxadjncy.offset(i as isize) as isize);
            i += 1;
            i;
        }
        mypart += 1;
        mypart;
    }
    (*lgraph).nvtxs = snvtxs[0];
    (*lgraph).nedges = snedges[0];
    (*rgraph).nvtxs = snvtxs[1];
    (*rgraph).nedges = snedges[1];
    libmetis__SetupGraph_tvwgt(lgraph);
    libmetis__SetupGraph_tvwgt(rgraph);
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint != 0 {
        (*ctrl).SplitTmr += gk_CPUSeconds();
    }
    *r_lgraph = lgraph;
    *r_rgraph = rgraph;
    libmetis__wspacepop(ctrl);
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__SplitGraphOrderCC(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut ncmps: idx_t,
    mut cptr: *mut idx_t,
    mut cind: *mut idx_t,
) -> *mut *mut graph_t {
    let mut i: idx_t = 0;
    let mut ii: idx_t = 0;
    let mut iii: idx_t = 0;
    let mut j: idx_t = 0;
    let mut k: idx_t = 0;
    let mut l: idx_t = 0;
    let mut istart: idx_t = 0;
    let mut iend: idx_t = 0;
    let mut mypart: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut snvtxs: idx_t = 0;
    let mut snedges: idx_t = 0;

    let mut vwgt: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut adjwgt: *mut idx_t = 0 as *mut idx_t;
    let mut label: *mut idx_t = 0 as *mut idx_t;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    let mut bndptr: *mut idx_t = 0 as *mut idx_t;
    let mut bndind: *mut idx_t = 0 as *mut idx_t;
    let mut sxadj: *mut idx_t = 0 as *mut idx_t;
    let mut svwgt: *mut idx_t = 0 as *mut idx_t;
    let mut sadjncy: *mut idx_t = 0 as *mut idx_t;
    let mut sadjwgt: *mut idx_t = 0 as *mut idx_t;
    let mut slabel: *mut idx_t = 0 as *mut idx_t;
    let mut rename: *mut idx_t = 0 as *mut idx_t;
    let mut auxadjncy: *mut idx_t = 0 as *mut idx_t;
    let mut sgraphs: *mut *mut graph_t = 0 as *mut *mut graph_t;
    libmetis__wspacepush(ctrl);
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint != 0 {
        (*ctrl).SplitTmr -= gk_CPUSeconds();
    }
    nvtxs = (*graph).nvtxs;
    let mut xadj = &mut (*graph).xadj;
    vwgt = (*graph).vwgt;
    adjncy = (*graph).adjncy;
    adjwgt = (*graph).adjwgt;
    label = (*graph).label;
    where_0 = (*graph).where_0;
    bndptr = (*graph).bndptr;
    bndind = (*graph).bndind;
    ii = 0 as libc::c_int;
    while ii < (*graph).nbnd {
        i = *bndind.offset(ii as isize);
        j = xadj[i as usize];
        while j < xadj[(i + 1) as usize] {
            *bndptr.offset(*adjncy.offset(j as isize) as isize) = 1;
            j += 1;
            j;
        }
        ii += 1;
        ii;
    }
    rename = libmetis__iwspacemalloc(ctrl, nvtxs);
    sgraphs = gk_malloc(
        (::core::mem::size_of::<*mut graph_t>() as u64).wrapping_mul(ncmps as u64),
        b"SplitGraphOrderCC: sgraphs\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut *mut graph_t;
    iii = 0 as libc::c_int;
    while iii < ncmps {
        libmetis__irandArrayPermute(
            *cptr.offset((iii + 1) as isize) - *cptr.offset(iii as isize),
            cind.offset(*cptr.offset(iii as isize) as isize),
            *cptr.offset((iii + 1) as isize) - *cptr.offset(iii as isize),
            0 as libc::c_int,
        );
        snedges = 0 as libc::c_int;
        snvtxs = snedges;
        j = *cptr.offset(iii as isize);
        while j < *cptr.offset((iii + 1) as isize) {
            i = *cind.offset(j as isize);
            let fresh4 = snvtxs;
            snvtxs = snvtxs + 1;
            *rename.offset(i as isize) = fresh4;
            snedges += xadj[(i + 1) as usize] - xadj[i as usize];
            j += 1;
            j;
        }
        let ref mut fresh5 = *sgraphs.offset(iii as isize);
        *fresh5 = libmetis__SetupSplitGraph(graph, snvtxs, snedges);
        let mut sxadj = &mut (**sgraphs.offset(iii as isize)).xadj;
        svwgt = (**sgraphs.offset(iii as isize)).vwgt;
        sadjncy = (**sgraphs.offset(iii as isize)).adjncy;
        sadjwgt = (**sgraphs.offset(iii as isize)).adjwgt;
        slabel = (**sgraphs.offset(iii as isize)).label;
        let ref mut fresh6 = sxadj[0];
        *fresh6 = 0 as libc::c_int;
        snedges = *fresh6;
        snvtxs = snedges;
        ii = *cptr.offset(iii as isize);
        while ii < *cptr.offset((iii + 1) as isize) {
            i = *cind.offset(ii as isize);
            istart = xadj[i as usize];
            iend = xadj[(i + 1) as usize];
            if *bndptr.offset(i as isize) == -(1) {
                auxadjncy = sadjncy.offset(snedges as isize).offset(-(istart as isize));
                j = istart;
                while j < iend {
                    *auxadjncy.offset(j as isize) = *adjncy.offset(j as isize);
                    j += 1;
                    j;
                }
                snedges += iend - istart;
            } else {
                l = snedges;
                j = istart;
                while j < iend {
                    k = *adjncy.offset(j as isize);
                    if *where_0.offset(k as isize) != 2 as libc::c_int {
                        let fresh7 = l;
                        l = l + 1;
                        *sadjncy.offset(fresh7 as isize) = k;
                    }
                    j += 1;
                    j;
                }
                snedges = l;
            }
            *svwgt.offset(snvtxs as isize) = *vwgt.offset(i as isize);
            *slabel.offset(snvtxs as isize) = *label.offset(i as isize);
            snvtxs += 1;
            sxadj[(snvtxs as usize)] = snedges;
            ii += 1;
            ii;
        }
        libmetis__iset(snedges as size_t, 1, sadjwgt);
        i = 0 as libc::c_int;
        while i < snedges {
            *sadjncy.offset(i as isize) = *rename.offset(*sadjncy.offset(i as isize) as isize);
            i += 1;
            i;
        }
        (**sgraphs.offset(iii as isize)).nvtxs = snvtxs;
        (**sgraphs.offset(iii as isize)).nedges = snedges;
        libmetis__SetupGraph_tvwgt(*sgraphs.offset(iii as isize));
        iii += 1;
        iii;
    }
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint != 0 {
        (*ctrl).SplitTmr += gk_CPUSeconds();
    }
    libmetis__wspacepop(ctrl);
    return sgraphs;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__MMDOrder(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut order: *mut idx_t,
    mut lastvtx: idx_t,
) {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut k: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut nofsub: idx_t = 0;
    let mut firstvtx: idx_t = 0;

    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut label: *mut idx_t = 0 as *mut idx_t;
    let mut perm: *mut idx_t = 0 as *mut idx_t;
    let mut iperm: *mut idx_t = 0 as *mut idx_t;
    let mut head: *mut idx_t = 0 as *mut idx_t;
    let mut qsize: *mut idx_t = 0 as *mut idx_t;
    let mut list: *mut idx_t = 0 as *mut idx_t;
    let mut marker: *mut idx_t = 0 as *mut idx_t;
    libmetis__wspacepush(ctrl);
    nvtxs = (*graph).nvtxs;
    let mut xadj = &mut (*graph).xadj;
    adjncy = (*graph).adjncy;
    k = xadj[nvtxs as usize];
    i = 0 as libc::c_int;
    while i < k {
        let ref mut fresh8 = *adjncy.offset(i as isize);
        *fresh8 += 1;
        *fresh8;
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < nvtxs + 1 {
        let ref mut fresh9 = xadj[i as usize];
        *fresh9 += 1;
        *fresh9;
        i += 1;
        i;
    }
    perm = libmetis__iwspacemalloc(ctrl, nvtxs + 5 as libc::c_int);
    iperm = libmetis__iwspacemalloc(ctrl, nvtxs + 5 as libc::c_int);
    head = libmetis__iwspacemalloc(ctrl, nvtxs + 5 as libc::c_int);
    qsize = libmetis__iwspacemalloc(ctrl, nvtxs + 5 as libc::c_int);
    list = libmetis__iwspacemalloc(ctrl, nvtxs + 5 as libc::c_int);
    marker = libmetis__iwspacemalloc(ctrl, nvtxs + 5 as libc::c_int);
    libmetis__genmmd(
        nvtxs,
        xadj,
        adjncy,
        iperm,
        perm,
        1,
        head,
        qsize,
        list,
        marker,
        2147483647 as libc::c_int,
        &mut nofsub,
    );
    label = (*graph).label;
    firstvtx = lastvtx - nvtxs;
    i = 0 as libc::c_int;
    while i < nvtxs {
        *order.offset(*label.offset(i as isize) as isize) =
            firstvtx + *iperm.offset(i as isize) - 1;
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < nvtxs + 1 {
        let ref mut fresh10 = xadj[i as usize];
        *fresh10 -= 1;
        *fresh10;
        i += 1;
        i;
    }
    k = xadj[nvtxs as usize];
    i = 0 as libc::c_int;
    while i < k {
        let ref mut fresh11 = *adjncy.offset(i as isize);
        *fresh11 -= 1;
        *fresh11;
        i += 1;
        i;
    }
    libmetis__wspacepop(ctrl);
}
