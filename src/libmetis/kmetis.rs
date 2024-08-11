use ::libc;
use libc::printf;

use crate::GKlib::{error::*, memory::*, timers::*, util::gk_log2};

use super::{
    auxapi::*, coarsen::libmetis__CoarsenGraph, contig::*, fortran::*, gklib::*, graph::*,
    kwayrefine::*, mcutil::libmetis__ComputeLoadImbalanceDiff, options::*,
    pmetis::METIS_PartGraphRecursive, structure::*, timing::*, util::*, wspace::*,
};

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
pub const METIS_OK: C2RustUnnamed = 1;
pub const METIS_OPTION_NCUTS: C2RustUnnamed_0 = 7;
pub const METIS_OPTION_NO2HOP: C2RustUnnamed_0 = 9;
pub const METIS_OPTION_OBJTYPE: C2RustUnnamed_0 = 1;
pub const METIS_OPTION_NITER: C2RustUnnamed_0 = 6;
pub const METIS_ERROR_INPUT: C2RustUnnamed = -2;
pub const METIS_ERROR_MEMORY: C2RustUnnamed = -3;
pub type C2RustUnnamed = libc::c_int;
pub const METIS_ERROR: C2RustUnnamed = -4;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const METIS_OPTION_UBVEC: C2RustUnnamed_0 = 24;
pub const METIS_OPTION_GTYPE: C2RustUnnamed_0 = 23;
pub const METIS_OPTION_BALANCE: C2RustUnnamed_0 = 22;
pub const METIS_OPTION_NOOUTPUT: C2RustUnnamed_0 = 21;
pub const METIS_OPTION_NCOMMON: C2RustUnnamed_0 = 20;
pub const METIS_OPTION_TPWGTS: C2RustUnnamed_0 = 19;
pub const METIS_OPTION_HELP: C2RustUnnamed_0 = 18;
pub const METIS_OPTION_NUMBERING: C2RustUnnamed_0 = 17;
pub const METIS_OPTION_UFACTOR: C2RustUnnamed_0 = 16;
pub const METIS_OPTION_NSEPS: C2RustUnnamed_0 = 15;
pub const METIS_OPTION_PFACTOR: C2RustUnnamed_0 = 14;
pub const METIS_OPTION_CCORDER: C2RustUnnamed_0 = 13;
pub const METIS_OPTION_COMPRESS: C2RustUnnamed_0 = 12;
pub const METIS_OPTION_CONTIG: C2RustUnnamed_0 = 11;
pub const METIS_OPTION_MINCONN: C2RustUnnamed_0 = 10;
pub const METIS_OPTION_SEED: C2RustUnnamed_0 = 8;
pub const METIS_OPTION_DBGLVL: C2RustUnnamed_0 = 5;
pub const METIS_OPTION_RTYPE: C2RustUnnamed_0 = 4;
pub const METIS_OPTION_IPTYPE: C2RustUnnamed_0 = 3;
pub const METIS_OPTION_CTYPE: C2RustUnnamed_0 = 2;
pub const METIS_OPTION_PTYPE: C2RustUnnamed_0 = 0;
#[no_mangle]
pub unsafe extern "C" fn METIS_PartGraphKway(
    mut nvtxs: *mut idx_t,
    mut ncon: *mut idx_t,
    mut xadj: *mut idx_t,
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
    sigrval = 0; // _setjmp((*gk_jbufs.as_mut_ptr().offset(GK_CUR_JBUFS as isize)).as_mut_ptr());
    if !(sigrval != 0 as libc::c_int) {
        ctrl = libmetis__SetupCtrl(METIS_OP_KMETIS, options, *ncon, *nparts, tpwgts, ubvec);
        if ctrl.is_null() {
            gk_siguntrap();
            return METIS_ERROR_INPUT as libc::c_int;
        }
        if (*ctrl).numflag == 1 as libc::c_int {
            libmetis__Change2CNumbering(*nvtxs, xadj, adjncy);
            renumber = 1 as libc::c_int;
        }
        graph = libmetis__SetupGraph(ctrl, *nvtxs, *ncon, xadj, adjncy, vwgt, vsize, adjwgt);
        libmetis__SetupKWayBalMultipliers(ctrl, graph);
        (*ctrl).CoarsenTo =
            if *nvtxs / (20 as libc::c_int * gk_log2(*nparts)) >= 30 as libc::c_int * *nparts {
                *nvtxs / (20 as libc::c_int * gk_log2(*nparts))
            } else {
                30 as libc::c_int * *nparts
            };
        (*ctrl).nIparts = if (*ctrl).CoarsenTo == 30 as libc::c_int * *nparts {
            4 as libc::c_int
        } else {
            5 as libc::c_int
        };
        if (*ctrl).contig != 0 && libmetis__IsConnected(graph, 0 as libc::c_int) == 0 {
            gk_errexit(
                15 as libc::c_int,
                b"METIS Error: A contiguous partition is requested for a non-contiguous input graph.\n\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
        libmetis__AllocateWorkSpace(ctrl, graph);
        if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint != 0 {
            libmetis__InitTimers(ctrl);
        }
        if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint != 0 {
            (*ctrl).TotalTmr -= gk_CPUSeconds();
        }
        *objval = libmetis__MlevelKWayPartitioning(ctrl, graph, part);
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
pub unsafe extern "C" fn libmetis__MlevelKWayPartitioning(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut part: *mut idx_t,
) -> idx_t {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut objval: idx_t = 0 as libc::c_int;
    let mut curobj: idx_t = 0 as libc::c_int;
    let mut bestobj: idx_t = 0 as libc::c_int;
    let mut curbal: real_t = 0.0f64 as real_t;
    let mut bestbal: real_t = 0.0f64 as real_t;
    let mut cgraph: *mut graph_t = 0 as *mut graph_t;
    let mut status: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*ctrl).ncuts {
        cgraph = libmetis__CoarsenGraph(ctrl, graph);
        if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint != 0 {
            (*ctrl).InitPartTmr -= gk_CPUSeconds();
        }
        libmetis__AllocateKWayPartitionMemory(ctrl, cgraph);
        libmetis__FreeWorkSpace(ctrl);
        libmetis__InitKWayPartitioning(ctrl, cgraph);
        libmetis__AllocateWorkSpace(ctrl, graph);
        libmetis__AllocateRefinementWorkSpace(ctrl, 2 as libc::c_int * (*cgraph).nedges);
        if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint != 0 {
            (*ctrl).InitPartTmr += gk_CPUSeconds();
        }
        if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_IPART as libc::c_int as libc::c_uint != 0 {
            printf(
                b"Initial %d-way partitioning cut: %d\n\0" as *const u8 as *const libc::c_char,
                (*ctrl).nparts,
                objval,
            );
        }
        libmetis__RefineKWay(ctrl, graph, cgraph);
        match (*ctrl).objtype as libc::c_uint {
            0 => {
                curobj = (*graph).mincut;
            }
            1 => {
                curobj = (*graph).minvol;
            }
            _ => {
                gk_errexit(
                    15 as libc::c_int,
                    b"Unknown objtype: %d\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    (*ctrl).objtype as libc::c_uint,
                );
            }
        }
        curbal = libmetis__ComputeLoadImbalanceDiff(
            graph,
            (*ctrl).nparts,
            (*ctrl).pijbm,
            (*ctrl).ubfactors,
        );
        if i == 0 as libc::c_int
            || curbal as libc::c_double <= 0.0005f64 && bestobj > curobj
            || bestbal as libc::c_double > 0.0005f64 && curbal < bestbal
        {
            libmetis__icopy((*graph).nvtxs as size_t, (*graph).where_0, part);
            bestobj = curobj;
            bestbal = curbal;
        }
        libmetis__FreeRData(graph);
        if bestobj == 0 as libc::c_int {
            break;
        }
        i += 1;
        i;
    }
    libmetis__FreeGraph(&mut graph);
    return bestobj;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__InitKWayPartitioning(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
) {
    let mut i: idx_t = 0;
    let mut ntrials: idx_t = 0;
    let mut options: [idx_t; 40] = [0; 40];
    let mut curobj: idx_t = 0 as libc::c_int;
    let mut bestobj: idx_t = 0 as libc::c_int;
    let mut bestwhere: *mut idx_t = 0 as *mut idx_t;
    let mut ubvec: *mut real_t = 0 as *mut real_t;
    let mut status: libc::c_int = 0;
    METIS_SetDefaultOptions(options.as_mut_ptr());
    options[METIS_OPTION_NITER as libc::c_int as usize] = 10 as libc::c_int;
    options[METIS_OPTION_OBJTYPE as libc::c_int as usize] = METIS_OBJTYPE_CUT as libc::c_int;
    options[METIS_OPTION_NO2HOP as libc::c_int as usize] = (*ctrl).no2hop;
    ubvec = libmetis__rmalloc(
        (*graph).ncon as size_t,
        b"InitKWayPartitioning: ubvec\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < (*graph).ncon {
        *ubvec.offset(i as isize) = f64::powf(
            *((*ctrl).ubfactors).offset(i as isize) as libc::c_double,
            1.0f64 / ((*ctrl).nparts as libc::c_double).ln(),
        ) as real_t;
        i += 1;
        i;
    }
    match (*ctrl).objtype as libc::c_uint {
        0 | 1 => {
            options[METIS_OPTION_NCUTS as libc::c_int as usize] = (*ctrl).nIparts;
            status = METIS_PartGraphRecursive(
                &mut (*graph).nvtxs,
                &mut (*graph).ncon,
                (*graph).xadj,
                (*graph).adjncy,
                (*graph).vwgt,
                (*graph).vsize,
                (*graph).adjwgt,
                &mut (*ctrl).nparts,
                (*ctrl).tpwgts,
                ubvec,
                options.as_mut_ptr(),
                &mut curobj,
                (*graph).where_0,
            );
            if status != METIS_OK as libc::c_int {
                gk_errexit(
                    15 as libc::c_int,
                    b"Failed during initial partitioning\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
        }
        _ => {
            gk_errexit(
                15 as libc::c_int,
                b"Unknown objtype: %d\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*ctrl).objtype as libc::c_uint,
            );
        }
    }
    gk_free(
        &mut ubvec as *mut *mut real_t as *mut *mut libc::c_void,
        &mut bestwhere as *mut *mut idx_t,
        0 as *mut *mut libc::c_void,
    );
}
