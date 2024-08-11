use ::libc;

use crate::GKlib::error::{__jmp_buf_tag, gk_jbufs, GK_CUR_JBUFS};
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;

    fn gk_malloc_init() -> libc::c_int;
    fn gk_malloc_cleanup(showstats: libc::c_int);
    fn gk_sigtrap() -> libc::c_int;
    fn gk_siguntrap() -> libc::c_int;
    fn gk_CPUSeconds() -> libc::c_double;

}
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
pub type idx_t = int32_t;
pub type real_t = libc::c_float;
use super::{
    coarsen::libmetis__CoarsenGraph,
    fortran::{libmetis__Change2CNumbering, libmetis__Change2FNumbering},
    gklib::{libmetis__icopy, libmetis__rscale, libmetis__rsum},
    graph::{
        libmetis__FreeGraph, libmetis__FreeRData, libmetis__SetupGraph, libmetis__SetupGraph_tvwgt,
        libmetis__SetupSplitGraph,
    },
    initpart::libmetis__Init2WayPartition,
    mcutil::libmetis__ComputeLoadImbalanceDiff,
    options::{libmetis__FreeCtrl, libmetis__Setup2WayBalMultipliers, libmetis__SetupCtrl},
    refine::{libmetis__Compute2WayPartitionParams, libmetis__Refine2Way},
    structure::*,
    timing::{libmetis__InitTimers, libmetis__PrintTimers},
    util::libmetis__metis_rcode,
    wspace::{
        libmetis__AllocateWorkSpace, libmetis__iwspacemalloc, libmetis__rwspacemalloc,
        libmetis__wspacepop, libmetis__wspacepush,
    },
};
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
pub type mrtype_et = libc::c_uint;
pub const METIS_RTYPE_SEP1SIDED: mrtype_et = 3;
pub const METIS_RTYPE_SEP2SIDED: mrtype_et = 2;
pub const METIS_RTYPE_GREEDY: mrtype_et = 1;
pub const METIS_RTYPE_FM: mrtype_et = 0;
pub type miptype_et = libc::c_uint;
pub const METIS_IPTYPE_METISRB: miptype_et = 4;
pub const METIS_IPTYPE_NODE: miptype_et = 3;
pub const METIS_IPTYPE_EDGE: miptype_et = 2;
pub const METIS_IPTYPE_RANDOM: miptype_et = 1;
pub const METIS_IPTYPE_GROW: miptype_et = 0;
pub type mctype_et = libc::c_uint;
pub const METIS_CTYPE_SHEM: mctype_et = 1;
pub const METIS_CTYPE_RM: mctype_et = 0;
pub type mdbglvl_et = libc::c_uint;
pub const METIS_DBG_MEMORY: mdbglvl_et = 2048;
pub const METIS_DBG_CONTIGINFO: mdbglvl_et = 256;
pub const METIS_DBG_CONNINFO: mdbglvl_et = 128;
pub const METIS_DBG_SEPINFO: mdbglvl_et = 64;
pub const METIS_DBG_MOVEINFO: mdbglvl_et = 32;
pub const METIS_DBG_IPART: mdbglvl_et = 16;
pub const METIS_DBG_REFINE: mdbglvl_et = 8;
pub const METIS_DBG_COARSEN: mdbglvl_et = 4;
pub const METIS_DBG_TIME: mdbglvl_et = 2;
pub const METIS_DBG_INFO: mdbglvl_et = 1;
pub type mobjtype_et = libc::c_uint;
pub const METIS_OBJTYPE_NODE: mobjtype_et = 2;
pub const METIS_OBJTYPE_VOL: mobjtype_et = 1;
pub const METIS_OBJTYPE_CUT: mobjtype_et = 0;
pub type moptype_et = libc::c_uint;
pub const METIS_OP_OMETIS: moptype_et = 2;
pub const METIS_OP_KMETIS: moptype_et = 1;
pub const METIS_OP_PMETIS: moptype_et = 0;

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
pub unsafe extern "C" fn METIS_PartGraphRecursive(
    mut nvtxs: *mut idx_t,
    mut ncon: *mut idx_t,
    mut xadj: &mut Vec<idx_t>,
    mut adjncy: *mut idx_t,
    mut vwgt: *mut idx_t,
    mut vsize: *mut idx_t,
    mut adjwgt: *mut idx_t,
    mut nparts: *mut idx_t,
    mut tpwgts: *mut real_t,
    mut ubvec: *mut real_t,
    mut options: *mut idx_t,
    mut objval: *mut idx_t,
    mut part: *mut idx_t,
) -> libc::c_int {
    let mut sigrval: libc::c_int = 0 as libc::c_int;
    let mut renumber: libc::c_int = 0 as libc::c_int;
    let mut graph: *mut graph_t = 0 as *mut graph_t;
    let mut ctrl: *mut ctrl_t = 0 as *mut ctrl_t;
    if gk_malloc_init() == 0 {
        return METIS_ERROR_MEMORY as libc::c_int;
    }
    gk_sigtrap();
    sigrval = _setjmp((*gk_jbufs.as_mut_ptr().offset(GK_CUR_JBUFS as isize)).as_mut_ptr());
    if !(sigrval != 0 as libc::c_int) {
        ctrl = libmetis__SetupCtrl(METIS_OP_PMETIS, options, *ncon, *nparts, tpwgts, ubvec);
        if ctrl.is_null() {
            gk_siguntrap();
            return METIS_ERROR_INPUT as libc::c_int;
        }
        if (*ctrl).numflag == 1 {
            libmetis__Change2CNumbering(*nvtxs, xadj, adjncy);
            renumber = 1;
        }
        graph = libmetis__SetupGraph(ctrl, *nvtxs, *ncon, xadj, adjncy, vwgt, vsize, adjwgt);
        libmetis__AllocateWorkSpace(ctrl, graph);
        if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint != 0 {
            libmetis__InitTimers(ctrl);
        }
        if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint != 0 {
            (*ctrl).TotalTmr -= gk_CPUSeconds();
        }
        *objval = libmetis__MlevelRecursiveBisection(
            ctrl,
            graph,
            *nparts,
            part,
            (*ctrl).tpwgts,
            0 as libc::c_int,
        );
        if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint != 0 {
            (*ctrl).TotalTmr += gk_CPUSeconds();
        }
        if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint != 0 {
            libmetis__PrintTimers(ctrl);
        }
        libmetis__FreeCtrl(&mut ctrl);
    }
    if renumber != 0 {
        libmetis__Change2FNumbering(*nvtxs, xadj, adjncy, part);
    }
    gk_siguntrap();
    gk_malloc_cleanup(0 as libc::c_int);
    return libmetis__metis_rcode(sigrval);
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__MlevelRecursiveBisection(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut nparts: idx_t,
    mut part: *mut idx_t,
    mut tpwgts: *mut real_t,
    mut fpart: idx_t,
) -> idx_t {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut ncon: idx_t = 0;
    let mut objval: idx_t = 0;
    let mut label: *mut idx_t = 0 as *mut idx_t;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    let mut lgraph: *mut graph_t = 0 as *mut graph_t;
    let mut rgraph: *mut graph_t = 0 as *mut graph_t;
    let mut wsum: real_t = 0.;
    let mut tpwgts2: *mut real_t = 0 as *mut real_t;
    nvtxs = (*graph).nvtxs;
    if nvtxs == 0 as libc::c_int {
        printf(
            b"\t***Cannot bisect a graph with 0 vertices!\n\t***You are trying to partition a graph into too many parts!\n\0"
                as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    ncon = (*graph).ncon;
    libmetis__wspacepush(ctrl);
    tpwgts2 = libmetis__rwspacemalloc(ctrl, 2 as libc::c_int * ncon);
    i = 0 as libc::c_int;
    while i < ncon {
        *tpwgts2.offset(i as isize) = libmetis__rsum(
            (nparts >> 1) as size_t,
            tpwgts.offset(i as isize),
            ncon as size_t,
        );
        *tpwgts2.offset((ncon + i) as isize) =
            (1.0f64 - *tpwgts2.offset(i as isize) as libc::c_double) as real_t;
        i += 1;
        i;
    }
    objval = libmetis__MultilevelBisect(ctrl, graph, tpwgts2);
    libmetis__wspacepop(ctrl);
    label = (*graph).label;
    where_0 = (*graph).where_0;
    i = 0 as libc::c_int;
    while i < nvtxs {
        *part.offset(*label.offset(i as isize) as isize) = *where_0.offset(i as isize) + fpart;
        i += 1;
        i;
    }
    if nparts > 2 as libc::c_int {
        libmetis__SplitGraphPart(ctrl, graph, &mut lgraph, &mut rgraph);
    }
    libmetis__FreeGraph(&mut graph);
    i = 0 as libc::c_int;
    while i < ncon {
        wsum = libmetis__rsum(
            (nparts >> 1) as size_t,
            tpwgts.offset(i as isize),
            ncon as size_t,
        );
        libmetis__rscale(
            (nparts >> 1) as size_t,
            (1.0f64 / wsum as libc::c_double) as real_t,
            tpwgts.offset(i as isize),
            ncon as size_t,
        );
        libmetis__rscale(
            (nparts - (nparts >> 1)) as size_t,
            (1.0f64 / (1.0f64 - wsum as libc::c_double)) as real_t,
            tpwgts
                .offset(((nparts >> 1) * ncon) as isize)
                .offset(i as isize),
            ncon as size_t,
        );
        i += 1;
        i;
    }
    if nparts > 3 as libc::c_int {
        objval +=
            libmetis__MlevelRecursiveBisection(ctrl, lgraph, nparts >> 1, part, tpwgts, fpart);
        objval += libmetis__MlevelRecursiveBisection(
            ctrl,
            rgraph,
            nparts - (nparts >> 1),
            part,
            tpwgts.offset(((nparts >> 1) * ncon) as isize),
            fpart + (nparts >> 1),
        );
    } else if nparts == 3 as libc::c_int {
        libmetis__FreeGraph(&mut lgraph);
        objval += libmetis__MlevelRecursiveBisection(
            ctrl,
            rgraph,
            nparts - (nparts >> 1),
            part,
            tpwgts.offset(((nparts >> 1) * ncon) as isize),
            fpart + (nparts >> 1),
        );
    }
    return objval;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__MultilevelBisect(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut tpwgts: *mut real_t,
) -> idx_t {
    let mut i: idx_t = 0;
    let mut niparts: idx_t = 0;
    let mut bestobj: idx_t = 0 as libc::c_int;
    let mut curobj: idx_t = 0 as libc::c_int;
    let mut bestwhere: *mut idx_t = 0 as *mut idx_t;
    let mut cgraph: *mut graph_t = 0 as *mut graph_t;
    let mut bestbal: real_t = 0.0f64 as real_t;
    let mut curbal: real_t = 0.0f64 as real_t;
    libmetis__Setup2WayBalMultipliers(ctrl, graph, tpwgts);
    libmetis__wspacepush(ctrl);
    if (*ctrl).ncuts > 1 {
        bestwhere = libmetis__iwspacemalloc(ctrl, (*graph).nvtxs);
    }
    i = 0 as libc::c_int;
    while i < (*ctrl).ncuts {
        cgraph = libmetis__CoarsenGraph(ctrl, graph);
        niparts = if (*cgraph).nvtxs <= (*ctrl).CoarsenTo {
            5 as libc::c_int
        } else {
            7 as libc::c_int
        };
        libmetis__Init2WayPartition(ctrl, cgraph, tpwgts, niparts);
        libmetis__Refine2Way(ctrl, graph, cgraph, tpwgts);
        curobj = (*graph).mincut;
        curbal = libmetis__ComputeLoadImbalanceDiff(
            graph,
            2 as libc::c_int,
            (*ctrl).pijbm,
            (*ctrl).ubfactors,
        );
        if i == 0 as libc::c_int
            || curbal as libc::c_double <= 0.0005f64 && bestobj > curobj
            || bestbal as libc::c_double > 0.0005f64 && curbal < bestbal
        {
            bestobj = curobj;
            bestbal = curbal;
            if i < (*ctrl).ncuts - 1 {
                libmetis__icopy((*graph).nvtxs as size_t, (*graph).where_0, bestwhere);
            }
        }
        if bestobj == 0 as libc::c_int {
            break;
        }
        if i < (*ctrl).ncuts - 1 {
            libmetis__FreeRData(graph);
        }
        i += 1;
        i;
    }
    if bestobj != curobj {
        libmetis__icopy((*graph).nvtxs as size_t, bestwhere, (*graph).where_0);
        libmetis__Compute2WayPartitionParams(ctrl, graph);
    }
    libmetis__wspacepop(ctrl);
    return bestobj;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__SplitGraphPart(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut r_lgraph: *mut *mut graph_t,
    mut r_rgraph: *mut *mut graph_t,
) {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut k: idx_t = 0;
    let mut l: idx_t = 0;
    let mut istart: idx_t = 0;
    let mut iend: idx_t = 0;
    let mut mypart: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut ncon: idx_t = 0;
    let mut snvtxs: [idx_t; 2] = [0; 2];
    let mut snedges: [idx_t; 2] = [0; 2];

    let mut vwgt: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut adjwgt: *mut idx_t = 0 as *mut idx_t;
    let mut label: *mut idx_t = 0 as *mut idx_t;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    let mut bndptr: *mut idx_t = 0 as *mut idx_t;
    let mut svwgt: [*mut idx_t; 2] = [0 as *mut idx_t; 2];
    let mut sadjncy: [*mut idx_t; 2] = [0 as *mut idx_t; 2];
    let mut sadjwgt: [*mut idx_t; 2] = [0 as *mut idx_t; 2];
    let mut slabel: [*mut idx_t; 2] = [0 as *mut idx_t; 2];
    let mut rename: *mut idx_t = 0 as *mut idx_t;
    let mut auxadjncy: *mut idx_t = 0 as *mut idx_t;
    let mut auxadjwgt: *mut idx_t = 0 as *mut idx_t;
    let mut lgraph: *mut graph_t = 0 as *mut graph_t;
    let mut rgraph: *mut graph_t = 0 as *mut graph_t;
    libmetis__wspacepush(ctrl);
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint != 0 {
        (*ctrl).SplitTmr -= gk_CPUSeconds();
    }
    nvtxs = (*graph).nvtxs;
    ncon = (*graph).ncon;
    let mut xadj = &mut (*graph).xadj;
    vwgt = (*graph).vwgt;
    adjncy = (*graph).adjncy;
    adjwgt = (*graph).adjwgt;
    label = (*graph).label;
    where_0 = (*graph).where_0;
    bndptr = (*graph).bndptr;
    rename = libmetis__iwspacemalloc(ctrl, nvtxs);
    snedges[1] = 0 as libc::c_int;
    snedges[0] = snedges[1];
    snvtxs[1] = snedges[0];
    snvtxs[0] = snvtxs[1];
    i = 0 as libc::c_int;
    while i < nvtxs {
        k = *where_0.offset(i as isize);
        let fresh0 = snvtxs[k as usize];
        snvtxs[k as usize] = snvtxs[k as usize] + 1;
        *rename.offset(i as isize) = fresh0;
        snedges[k as usize] += xadj[(i + 1) as usize] - xadj[i as usize];
        i += 1;
        i;
    }
    lgraph = libmetis__SetupSplitGraph(graph, snvtxs[0], snedges[0]);

    svwgt[0] = (*lgraph).vwgt;
    sadjncy[0] = (*lgraph).adjncy;
    sadjwgt[0] = (*lgraph).adjwgt;
    slabel[0] = (*lgraph).label;
    rgraph = libmetis__SetupSplitGraph(graph, snvtxs[1], snedges[1]);
    let sxadj = [&mut (*lgraph).xadj, &mut (*rgraph).xadj];

    svwgt[1] = (*rgraph).vwgt;
    sadjncy[1] = (*rgraph).adjncy;
    sadjwgt[1] = (*rgraph).adjwgt;
    slabel[1] = (*rgraph).label;
    snedges[1] = 0 as libc::c_int;
    snedges[0] = snedges[1];
    snvtxs[1] = snedges[0];
    snvtxs[0] = snvtxs[1];
    let ref mut fresh1 = (sxadj[1])[0];
    *fresh1 = 0 as libc::c_int;
    (sxadj[0])[0] = *fresh1;
    i = 0 as libc::c_int;
    while i < nvtxs {
        mypart = *where_0.offset(i as isize);
        istart = xadj[i as usize];
        iend = xadj[(i + 1) as usize];
        if *bndptr.offset(i as isize) == -(1) {
            auxadjncy = (sadjncy[mypart as usize])
                .offset(snedges[mypart as usize] as isize)
                .offset(-(istart as isize));
            auxadjwgt = (sadjwgt[mypart as usize])
                .offset(snedges[mypart as usize] as isize)
                .offset(-(istart as isize));
            j = istart;
            while j < iend {
                *auxadjncy.offset(j as isize) = *adjncy.offset(j as isize);
                *auxadjwgt.offset(j as isize) = *adjwgt.offset(j as isize);
                j += 1;
                j;
            }
            snedges[mypart as usize] += iend - istart;
        } else {
            auxadjncy = sadjncy[mypart as usize];
            auxadjwgt = sadjwgt[mypart as usize];
            l = snedges[mypart as usize];
            j = istart;
            while j < iend {
                k = *adjncy.offset(j as isize);
                if *where_0.offset(k as isize) == mypart {
                    *auxadjncy.offset(l as isize) = k;
                    let fresh2 = l;
                    l = l + 1;
                    *auxadjwgt.offset(fresh2 as isize) = *adjwgt.offset(j as isize);
                }
                j += 1;
                j;
            }
            snedges[mypart as usize] = l;
        }
        k = 0 as libc::c_int;
        while k < ncon {
            *(svwgt[mypart as usize]).offset((snvtxs[mypart as usize] * ncon + k) as isize) =
                *vwgt.offset((i * ncon + k) as isize);
            k += 1;
            k;
        }
        *(slabel[mypart as usize]).offset(snvtxs[mypart as usize] as isize) =
            *label.offset(i as isize);
        snvtxs[mypart as usize] += 1;
        (sxadj[mypart as usize])[(snvtxs[mypart as usize] as usize)] = snedges[mypart as usize];
        i += 1;
        i;
    }
    mypart = 0 as libc::c_int;
    while mypart < 2 as libc::c_int {
        iend = (sxadj[mypart as usize])[(snvtxs[mypart as usize] as usize)];
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
    (*lgraph).nedges = snedges[0];
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
