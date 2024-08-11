use libc::printf;

use crate::GKlib::memory::gk_free;

use super::{
    gklib::{
        libmetis__icopy, libmetis__ikvmalloc, libmetis__ikvsorti, libmetis__imalloc,
        libmetis__irandArrayPermute, libmetis__iset, libmetis__ismalloc, libmetis__rpqCreate,
        libmetis__rpqDelete, libmetis__rpqDestroy, libmetis__rpqGetTop, libmetis__rpqInsert,
        libmetis__rpqLength, libmetis__rpqReset, libmetis__rpqSeeTopVal, libmetis__rpqUpdate,
    },
    graph::{
        libmetis__CreateGraph, libmetis__FreeGraph, libmetis__SetupGraph,
        libmetis__SetupGraph_label, libmetis__SetupGraph_tvwgt,
    },
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

use super::structure::*;
#[no_mangle]
pub unsafe extern "C" fn libmetis__CompressGraph(
    mut ctrl: *mut ctrl_t,
    mut nvtxs: idx_t,
    mut xadj: &mut Vec<idx_t>,
    mut adjncy: *mut idx_t,
    mut vwgt: *mut idx_t,
    mut cptr: *mut idx_t,
    mut cind: *mut idx_t,
) -> *mut graph_t {
    let mut i: idx_t = 0;
    let mut ii: idx_t = 0;
    let mut iii: idx_t = 0;
    let mut j: idx_t = 0;
    let mut jj: idx_t = 0;
    let mut k: idx_t = 0;
    let mut l: idx_t = 0;
    let mut cnvtxs: idx_t = 0;
    let mut cnedges: idx_t = 0;
    let mut cxadj: *mut idx_t = 0 as *mut idx_t;
    let mut cadjncy: *mut idx_t = 0 as *mut idx_t;
    let mut cvwgt: *mut idx_t = 0 as *mut idx_t;
    let mut mark: *mut idx_t = 0 as *mut idx_t;
    let mut map: *mut idx_t = 0 as *mut idx_t;
    let mut keys: *mut ikv_t = 0 as *mut ikv_t;
    let mut graph: *mut graph_t = 0 as *mut graph_t;
    mark = libmetis__ismalloc(
        nvtxs as size_t,
        -(1),
        b"CompressGraph: mark\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    map = libmetis__ismalloc(
        nvtxs as size_t,
        -(1),
        b"CompressGraph: map\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    keys = libmetis__ikvmalloc(
        nvtxs as size_t,
        b"CompressGraph: keys\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < nvtxs {
        k = 0 as libc::c_int;
        j = xadj[i as usize];
        while j < xadj[(i + 1) as usize] {
            k += *adjncy.offset(j as isize);
            j += 1;
            j;
        }
        (*keys.offset(i as isize)).key = k + i;
        (*keys.offset(i as isize)).val = i;
        i += 1;
        i;
    }
    libmetis__ikvsorti(nvtxs as size_t, keys);
    let ref mut fresh0 = *cptr.offset(0 as libc::c_int as isize);
    *fresh0 = 0 as libc::c_int;
    l = *fresh0;
    i = 0 as libc::c_int;
    cnvtxs = i;
    while i < nvtxs {
        ii = (*keys.offset(i as isize)).val;
        if *map.offset(ii as isize) == -(1) {
            *mark.offset(ii as isize) = i;
            j = xadj[(ii as usize)];
            while j < xadj[((ii + 1) as usize)] {
                *mark.offset(*adjncy.offset(j as isize) as isize) = i;
                j += 1;
                j;
            }
            *map.offset(ii as isize) = cnvtxs;
            let fresh1 = l;
            l = l + 1;
            *cind.offset(fresh1 as isize) = ii;
            j = i + 1;
            while j < nvtxs {
                iii = (*keys.offset(j as isize)).val;
                if (*keys.offset(i as isize)).key != (*keys.offset(j as isize)).key
                    || xadj[((ii + 1) as usize)] - xadj[(ii as usize)]
                        != xadj[((iii + 1) as usize)] - xadj[(iii as usize)]
                {
                    break;
                }
                if *map.offset(iii as isize) == -(1) {
                    jj = xadj[(iii as usize)];
                    while jj < xadj[((iii + 1) as usize)] {
                        if *mark.offset(*adjncy.offset(jj as isize) as isize) != i {
                            break;
                        }
                        jj += 1;
                        jj;
                    }
                    if jj == xadj[((iii + 1) as usize)] {
                        *map.offset(iii as isize) = cnvtxs;
                        let fresh2 = l;
                        l = l + 1;
                        *cind.offset(fresh2 as isize) = iii;
                    }
                }
                j += 1;
                j;
            }
            cnvtxs += 1;
            *cptr.offset(cnvtxs as isize) = l;
        }
        i += 1;
        i;
    }
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_INFO as libc::c_int as libc::c_uint != 0 {
        printf(
            b"  Compression: reduction in # of vertices: %d.\n\0" as *const u8
                as *const libc::c_char,
            nvtxs - cnvtxs,
        );
    }
    if (cnvtxs as libc::c_double) < 0.85f64 * nvtxs as libc::c_double {
        graph = libmetis__CreateGraph();
        cnedges = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < cnvtxs {
            ii = *cind.offset(*cptr.offset(i as isize) as isize);
            cnedges += xadj[((ii + 1) as usize)] - xadj[(ii as usize)];
            i += 1;
            i;
        }
        (*graph).xadj = vec![0; (cnvtxs + 1) as usize];

        let mut cxadj = &mut (*graph).xadj;
        (*graph).vwgt = libmetis__ismalloc(
            cnvtxs as size_t,
            0 as libc::c_int,
            b"CompressGraph: vwgt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        cvwgt = (*graph).vwgt;
        (*graph).adjncy = libmetis__imalloc(
            cnedges as size_t,
            b"CompressGraph: adjncy\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        cadjncy = (*graph).adjncy;
        (*graph).adjwgt = libmetis__ismalloc(
            cnedges as size_t,
            1,
            b"CompressGraph: adjwgt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        libmetis__iset(nvtxs as size_t, -(1), mark);
        let ref mut fresh3 = cxadj[(0 as libc::c_int as usize)];
        *fresh3 = 0 as libc::c_int;
        l = *fresh3;
        i = 0 as libc::c_int;
        while i < cnvtxs {
            *mark.offset(i as isize) = i;
            j = *cptr.offset(i as isize);
            while j < *cptr.offset((i + 1) as isize) {
                ii = *cind.offset(j as isize);
                let ref mut fresh4 = *cvwgt.offset(i as isize);
                *fresh4 += if vwgt.is_null() {
                    1
                } else {
                    *vwgt.offset(ii as isize)
                };
                jj = xadj[(ii as usize)];
                while jj < xadj[((ii + 1) as usize)] {
                    k = *map.offset(*adjncy.offset(jj as isize) as isize);
                    if *mark.offset(k as isize) != i {
                        *mark.offset(k as isize) = i;
                        let fresh5 = l;
                        l = l + 1;
                        *cadjncy.offset(fresh5 as isize) = k;
                    }
                    jj += 1;
                    jj;
                }
                j += 1;
                j;
            }
            cxadj[((i + 1) as usize)] = l;
            i += 1;
            i;
        }
        (*graph).nvtxs = cnvtxs;
        (*graph).nedges = l;
        (*graph).ncon = 1;
        libmetis__SetupGraph_tvwgt(graph);
        libmetis__SetupGraph_label(graph);
    }
    gk_free(
        &mut keys as *mut *mut ikv_t as *mut *mut libc::c_void,
        &mut map as *mut *mut idx_t,
        &mut mark as *mut *mut idx_t,
        0 as *mut *mut libc::c_void,
    );
    return graph;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__PruneGraph(
    mut ctrl: *mut ctrl_t,
    mut nvtxs: idx_t,
    mut xadj: &mut Vec<idx_t>,
    mut adjncy: *mut idx_t,
    mut vwgt: *mut idx_t,
    mut iperm: *mut idx_t,
    mut factor: real_t,
) -> *mut graph_t {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut k: idx_t = 0;
    let mut l: idx_t = 0;
    let mut nlarge: idx_t = 0;
    let mut pnvtxs: idx_t = 0;
    let mut pnedges: idx_t = 0;
    let mut pxadj: *mut idx_t = 0 as *mut idx_t;
    let mut padjncy: *mut idx_t = 0 as *mut idx_t;
    let mut padjwgt: *mut idx_t = 0 as *mut idx_t;
    let mut pvwgt: *mut idx_t = 0 as *mut idx_t;
    let mut perm: *mut idx_t = 0 as *mut idx_t;
    let mut graph: *mut graph_t = 0 as *mut graph_t;
    perm = libmetis__imalloc(
        nvtxs as size_t,
        b"PruneGraph: perm\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    factor = factor * xadj[(nvtxs as usize)] as libc::c_float / nvtxs as libc::c_float;
    nlarge = 0 as libc::c_int;
    pnedges = nlarge;
    pnvtxs = pnedges;
    i = 0 as libc::c_int;
    while i < nvtxs {
        if ((xadj[(i + 1) as usize] - xadj[i as usize]) as libc::c_float) < factor {
            *perm.offset(i as isize) = pnvtxs;
            let fresh6 = pnvtxs;
            pnvtxs = pnvtxs + 1;
            *iperm.offset(fresh6 as isize) = i;
            pnedges += xadj[(i + 1) as usize] - xadj[i as usize];
        } else {
            nlarge += 1;
            *perm.offset(i as isize) = nvtxs - nlarge;
            *iperm.offset((nvtxs - nlarge) as isize) = i;
        }
        i += 1;
        i;
    }
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_INFO as libc::c_int as libc::c_uint != 0 {
        printf(
            b"  Pruned %d of %d vertices.\n\0" as *const u8 as *const libc::c_char,
            nlarge,
            nvtxs,
        );
    }
    if nlarge > 0 as libc::c_int && nlarge < nvtxs {
        graph = libmetis__CreateGraph();
        (*graph).xadj = vec![0; (pnvtxs + 1) as usize];

        let mut pxadj = &mut (*graph).xadj;
        (*graph).vwgt = libmetis__imalloc(
            pnvtxs as size_t,
            b"PruneGraph: vwgt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        pvwgt = (*graph).vwgt;
        (*graph).adjncy = libmetis__imalloc(
            pnedges as size_t,
            b"PruneGraph: adjncy\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        padjncy = (*graph).adjncy;
        (*graph).adjwgt = libmetis__ismalloc(
            pnedges as size_t,
            1,
            b"PruneGraph: adjwgt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        l = 0 as libc::c_int;
        pnedges = l;
        pxadj[(0 as libc::c_int as usize)] = pnedges;
        i = 0 as libc::c_int;
        while i < nvtxs {
            if ((xadj[(i + 1) as usize] - xadj[i as usize]) as libc::c_float) < factor {
                *pvwgt.offset(l as isize) = if vwgt.is_null() {
                    1
                } else {
                    *vwgt.offset(i as isize)
                };
                j = xadj[i as usize];
                while j < xadj[(i + 1) as usize] {
                    k = *perm.offset(*adjncy.offset(j as isize) as isize);
                    if k < pnvtxs {
                        let fresh7 = pnedges;
                        pnedges = pnedges + 1;
                        *padjncy.offset(fresh7 as isize) = k;
                    }
                    j += 1;
                    j;
                }
                l += 1;
                pxadj[(l as usize)] = pnedges;
            }
            i += 1;
            i;
        }
        (*graph).nvtxs = pnvtxs;
        (*graph).nedges = pnedges;
        (*graph).ncon = 1;
        libmetis__SetupGraph_tvwgt(graph);
        libmetis__SetupGraph_label(graph);
    } else if nlarge > 0 as libc::c_int && nlarge == nvtxs {
        if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_INFO as libc::c_int as libc::c_uint != 0 {
            printf(
                b"  Pruning is ignored as it removes all vertices.\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        nlarge = 0 as libc::c_int;
    }
    gk_free(
        &mut perm as *mut *mut idx_t as *mut *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
    return graph;
}
