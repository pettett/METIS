use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;
    #[thread_local]
    static mut gk_cur_jbufs: libc::c_int;
    #[thread_local]
    static mut gk_jbufs: [jmp_buf; 0];
    fn gk_malloc_init() -> libc::c_int;
    fn gk_malloc_cleanup(showstats: libc::c_int);
    fn gk_sigtrap() -> libc::c_int;
    fn gk_siguntrap() -> libc::c_int;
    fn gk_CPUSeconds() -> libc::c_double;
    fn libmetis__metis_rcode(sigrval: libc::c_int) -> libc::c_int;
    fn libmetis__Change2FNumbering(
        _: idx_t,
        _: *mut idx_t,
        _: *mut idx_t,
        _: *mut idx_t,
    );
    fn libmetis__FreeCtrl(r_ctrl: *mut *mut ctrl_t);
    fn libmetis__PrintTimers(_: *mut ctrl_t);
    fn libmetis__FreeGraph(graph: *mut *mut graph_t);
    fn libmetis__rscale(
        n: size_t,
        alpha: real_t,
        x: *mut real_t,
        incx: size_t,
    ) -> *mut real_t;
    fn libmetis__rsum(n: size_t, x: *mut real_t, incx: size_t) -> real_t;
    fn libmetis__wspacepop(ctrl: *mut ctrl_t);
    fn libmetis__SetupGraph_tvwgt(graph: *mut graph_t);
    fn libmetis__SetupSplitGraph(
        graph: *mut graph_t,
        snvtxs: idx_t,
        snedges: idx_t,
    ) -> *mut graph_t;
    fn libmetis__iwspacemalloc(_: *mut ctrl_t, _: idx_t) -> *mut idx_t;
    fn libmetis__wspacepush(ctrl: *mut ctrl_t);
    fn libmetis__Compute2WayPartitionParams(ctrl: *mut ctrl_t, graph: *mut graph_t);
    fn libmetis__icopy(n: size_t, a: *mut idx_t, b: *mut idx_t) -> *mut idx_t;
    fn libmetis__FreeRData(graph: *mut graph_t);
    fn libmetis__ComputeLoadImbalanceDiff(
        graph: *mut graph_t,
        nparts: idx_t,
        pijbm: *mut real_t,
        ubvec: *mut real_t,
    ) -> real_t;
    fn libmetis__Refine2Way(
        ctrl: *mut ctrl_t,
        orggraph: *mut graph_t,
        graph: *mut graph_t,
        rtpwgts: *mut real_t,
    );
    fn libmetis__Init2WayPartition(
        ctrl: *mut ctrl_t,
        graph: *mut graph_t,
        ntpwgts: *mut real_t,
        niparts: idx_t,
    );
    fn libmetis__CoarsenGraph(ctrl: *mut ctrl_t, graph: *mut graph_t) -> *mut graph_t;
    fn libmetis__Setup2WayBalMultipliers(
        ctrl: *mut ctrl_t,
        graph: *mut graph_t,
        tpwgts: *mut real_t,
    );
    fn libmetis__rwspacemalloc(_: *mut ctrl_t, _: idx_t) -> *mut real_t;
    fn libmetis__InitTimers(_: *mut ctrl_t);
    fn libmetis__AllocateWorkSpace(ctrl: *mut ctrl_t, graph: *mut graph_t);
    fn libmetis__SetupGraph(
        ctrl: *mut ctrl_t,
        nvtxs: idx_t,
        ncon: idx_t,
        xadj: *mut idx_t,
        adjncy: *mut idx_t,
        vwgt: *mut idx_t,
        vsize: *mut idx_t,
        adjwgt: *mut idx_t,
    ) -> *mut graph_t;
    fn libmetis__Change2CNumbering(_: idx_t, _: *mut idx_t, _: *mut idx_t);
    fn libmetis__SetupCtrl(
        optype: moptype_et,
        options: *mut idx_t,
        ncon: idx_t,
        nparts: idx_t,
        tpwgts: *mut real_t,
        ubvec: *mut real_t,
    ) -> *mut ctrl_t;
}
pub type __int32_t = libc::c_int;
pub type __ssize_t = libc::c_long;
pub type int32_t = __int32_t;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type __jmp_buf = [libc::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ctrl_t {
    pub optype: moptype_et,
    pub objtype: mobjtype_et,
    pub dbglvl: mdbglvl_et,
    pub ctype: mctype_et,
    pub iptype: miptype_et,
    pub rtype: mrtype_et,
    pub CoarsenTo: idx_t,
    pub nIparts: idx_t,
    pub no2hop: idx_t,
    pub minconn: idx_t,
    pub contig: idx_t,
    pub nseps: idx_t,
    pub ufactor: idx_t,
    pub compress: idx_t,
    pub ccorder: idx_t,
    pub seed: idx_t,
    pub ncuts: idx_t,
    pub niter: idx_t,
    pub numflag: idx_t,
    pub maxvwgt: *mut idx_t,
    pub ncon: idx_t,
    pub nparts: idx_t,
    pub pfactor: real_t,
    pub ubfactors: *mut real_t,
    pub tpwgts: *mut real_t,
    pub pijbm: *mut real_t,
    pub cfactor: real_t,
    pub TotalTmr: libc::c_double,
    pub InitPartTmr: libc::c_double,
    pub MatchTmr: libc::c_double,
    pub ContractTmr: libc::c_double,
    pub CoarsenTmr: libc::c_double,
    pub UncoarsenTmr: libc::c_double,
    pub RefTmr: libc::c_double,
    pub ProjectTmr: libc::c_double,
    pub SplitTmr: libc::c_double,
    pub Aux1Tmr: libc::c_double,
    pub Aux2Tmr: libc::c_double,
    pub Aux3Tmr: libc::c_double,
    pub mcore: *mut gk_mcore_t,
    pub nbrpoolsize: size_t,
    pub nbrpoolcpos: size_t,
    pub nbrpoolreallocs: size_t,
    pub cnbrpool: *mut cnbr_t,
    pub vnbrpool: *mut vnbr_t,
    pub maxnads: *mut idx_t,
    pub nads: *mut idx_t,
    pub adids: *mut *mut idx_t,
    pub adwgts: *mut *mut idx_t,
    pub pvec1: *mut idx_t,
    pub pvec2: *mut idx_t,
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
pub struct graph_t {
    pub nvtxs: idx_t,
    pub nedges: idx_t,
    pub ncon: idx_t,
    pub xadj: *mut idx_t,
    pub vwgt: *mut idx_t,
    pub vsize: *mut idx_t,
    pub adjncy: *mut idx_t,
    pub adjwgt: *mut idx_t,
    pub tvwgt: *mut idx_t,
    pub invtvwgt: *mut real_t,
    pub free_xadj: libc::c_int,
    pub free_vwgt: libc::c_int,
    pub free_vsize: libc::c_int,
    pub free_adjncy: libc::c_int,
    pub free_adjwgt: libc::c_int,
    pub label: *mut idx_t,
    pub cmap: *mut idx_t,
    pub mincut: idx_t,
    pub minvol: idx_t,
    pub where_0: *mut idx_t,
    pub pwgts: *mut idx_t,
    pub nbnd: idx_t,
    pub bndptr: *mut idx_t,
    pub bndind: *mut idx_t,
    pub id: *mut idx_t,
    pub ed: *mut idx_t,
    pub ckrinfo: *mut ckrinfo_t,
    pub vkrinfo: *mut vkrinfo_t,
    pub nrinfo: *mut nrinfo_t,
    pub coarser: *mut graph_t,
    pub finer: *mut graph_t,
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
pub unsafe extern "C" fn METIS_PartGraphRecursive(
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
    sigrval = _setjmp(
        (*gk_jbufs.as_mut_ptr().offset(gk_cur_jbufs as isize)).as_mut_ptr(),
    );
    if !(sigrval != 0 as libc::c_int) {
        ctrl = libmetis__SetupCtrl(
            METIS_OP_PMETIS,
            options,
            *ncon,
            *nparts,
            tpwgts,
            ubvec,
        );
        if ctrl.is_null() {
            gk_siguntrap();
            return METIS_ERROR_INPUT as libc::c_int;
        }
        if (*ctrl).numflag == 1 as libc::c_int {
            libmetis__Change2CNumbering(*nvtxs, xadj, adjncy);
            renumber = 1 as libc::c_int;
        }
        graph = libmetis__SetupGraph(
            ctrl,
            *nvtxs,
            *ncon,
            xadj,
            adjncy,
            vwgt,
            vsize,
            adjwgt,
        );
        libmetis__AllocateWorkSpace(ctrl, graph);
        if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint
            != 0
        {
            libmetis__InitTimers(ctrl);
        }
        if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint
            != 0
        {
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
        if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint
            != 0
        {
            (*ctrl).TotalTmr += gk_CPUSeconds();
        }
        if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint
            != 0
        {
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
        *tpwgts2
            .offset(
                i as isize,
            ) = libmetis__rsum(
            (nparts >> 1 as libc::c_int) as size_t,
            tpwgts.offset(i as isize),
            ncon as size_t,
        );
        *tpwgts2
            .offset(
                (ncon + i) as isize,
            ) = (1.0f64 - *tpwgts2.offset(i as isize) as libc::c_double) as real_t;
        i += 1;
        i;
    }
    objval = libmetis__MultilevelBisect(ctrl, graph, tpwgts2);
    libmetis__wspacepop(ctrl);
    label = (*graph).label;
    where_0 = (*graph).where_0;
    i = 0 as libc::c_int;
    while i < nvtxs {
        *part
            .offset(
                *label.offset(i as isize) as isize,
            ) = *where_0.offset(i as isize) + fpart;
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
            (nparts >> 1 as libc::c_int) as size_t,
            tpwgts.offset(i as isize),
            ncon as size_t,
        );
        libmetis__rscale(
            (nparts >> 1 as libc::c_int) as size_t,
            (1.0f64 / wsum as libc::c_double) as real_t,
            tpwgts.offset(i as isize),
            ncon as size_t,
        );
        libmetis__rscale(
            (nparts - (nparts >> 1 as libc::c_int)) as size_t,
            (1.0f64 / (1.0f64 - wsum as libc::c_double)) as real_t,
            tpwgts
                .offset(((nparts >> 1 as libc::c_int) * ncon) as isize)
                .offset(i as isize),
            ncon as size_t,
        );
        i += 1;
        i;
    }
    if nparts > 3 as libc::c_int {
        objval
            += libmetis__MlevelRecursiveBisection(
                ctrl,
                lgraph,
                nparts >> 1 as libc::c_int,
                part,
                tpwgts,
                fpart,
            );
        objval
            += libmetis__MlevelRecursiveBisection(
                ctrl,
                rgraph,
                nparts - (nparts >> 1 as libc::c_int),
                part,
                tpwgts.offset(((nparts >> 1 as libc::c_int) * ncon) as isize),
                fpart + (nparts >> 1 as libc::c_int),
            );
    } else if nparts == 3 as libc::c_int {
        libmetis__FreeGraph(&mut lgraph);
        objval
            += libmetis__MlevelRecursiveBisection(
                ctrl,
                rgraph,
                nparts - (nparts >> 1 as libc::c_int),
                part,
                tpwgts.offset(((nparts >> 1 as libc::c_int) * ncon) as isize),
                fpart + (nparts >> 1 as libc::c_int),
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
    if (*ctrl).ncuts > 1 as libc::c_int {
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
            if i < (*ctrl).ncuts - 1 as libc::c_int {
                libmetis__icopy((*graph).nvtxs as size_t, (*graph).where_0, bestwhere);
            }
        }
        if bestobj == 0 as libc::c_int {
            break;
        }
        if i < (*ctrl).ncuts - 1 as libc::c_int {
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
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
    let mut vwgt: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut adjwgt: *mut idx_t = 0 as *mut idx_t;
    let mut label: *mut idx_t = 0 as *mut idx_t;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    let mut bndptr: *mut idx_t = 0 as *mut idx_t;
    let mut sxadj: [*mut idx_t; 2] = [0 as *mut idx_t; 2];
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
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint
        != 0
    {
        (*ctrl).SplitTmr -= gk_CPUSeconds();
    }
    nvtxs = (*graph).nvtxs;
    ncon = (*graph).ncon;
    xadj = (*graph).xadj;
    vwgt = (*graph).vwgt;
    adjncy = (*graph).adjncy;
    adjwgt = (*graph).adjwgt;
    label = (*graph).label;
    where_0 = (*graph).where_0;
    bndptr = (*graph).bndptr;
    rename = libmetis__iwspacemalloc(ctrl, nvtxs);
    snedges[1 as libc::c_int as usize] = 0 as libc::c_int;
    snedges[0 as libc::c_int as usize] = snedges[1 as libc::c_int as usize];
    snvtxs[1 as libc::c_int as usize] = snedges[0 as libc::c_int as usize];
    snvtxs[0 as libc::c_int as usize] = snvtxs[1 as libc::c_int as usize];
    i = 0 as libc::c_int;
    while i < nvtxs {
        k = *where_0.offset(i as isize);
        let fresh0 = snvtxs[k as usize];
        snvtxs[k as usize] = snvtxs[k as usize] + 1;
        *rename.offset(i as isize) = fresh0;
        snedges[k as usize]
            += *xadj.offset((i + 1 as libc::c_int) as isize) - *xadj.offset(i as isize);
        i += 1;
        i;
    }
    lgraph = libmetis__SetupSplitGraph(
        graph,
        snvtxs[0 as libc::c_int as usize],
        snedges[0 as libc::c_int as usize],
    );
    sxadj[0 as libc::c_int as usize] = (*lgraph).xadj;
    svwgt[0 as libc::c_int as usize] = (*lgraph).vwgt;
    sadjncy[0 as libc::c_int as usize] = (*lgraph).adjncy;
    sadjwgt[0 as libc::c_int as usize] = (*lgraph).adjwgt;
    slabel[0 as libc::c_int as usize] = (*lgraph).label;
    rgraph = libmetis__SetupSplitGraph(
        graph,
        snvtxs[1 as libc::c_int as usize],
        snedges[1 as libc::c_int as usize],
    );
    sxadj[1 as libc::c_int as usize] = (*rgraph).xadj;
    svwgt[1 as libc::c_int as usize] = (*rgraph).vwgt;
    sadjncy[1 as libc::c_int as usize] = (*rgraph).adjncy;
    sadjwgt[1 as libc::c_int as usize] = (*rgraph).adjwgt;
    slabel[1 as libc::c_int as usize] = (*rgraph).label;
    snedges[1 as libc::c_int as usize] = 0 as libc::c_int;
    snedges[0 as libc::c_int as usize] = snedges[1 as libc::c_int as usize];
    snvtxs[1 as libc::c_int as usize] = snedges[0 as libc::c_int as usize];
    snvtxs[0 as libc::c_int as usize] = snvtxs[1 as libc::c_int as usize];
    let ref mut fresh1 = *(sxadj[1 as libc::c_int as usize])
        .offset(0 as libc::c_int as isize);
    *fresh1 = 0 as libc::c_int;
    *(sxadj[0 as libc::c_int as usize]).offset(0 as libc::c_int as isize) = *fresh1;
    i = 0 as libc::c_int;
    while i < nvtxs {
        mypart = *where_0.offset(i as isize);
        istart = *xadj.offset(i as isize);
        iend = *xadj.offset((i + 1 as libc::c_int) as isize);
        if *bndptr.offset(i as isize) == -(1 as libc::c_int) {
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
            *(svwgt[mypart as usize])
                .offset(
                    (snvtxs[mypart as usize] * ncon + k) as isize,
                ) = *vwgt.offset((i * ncon + k) as isize);
            k += 1;
            k;
        }
        *(slabel[mypart as usize])
            .offset(snvtxs[mypart as usize] as isize) = *label.offset(i as isize);
        snvtxs[mypart as usize] += 1;
        *(sxadj[mypart as usize])
            .offset(snvtxs[mypart as usize] as isize) = snedges[mypart as usize];
        i += 1;
        i;
    }
    mypart = 0 as libc::c_int;
    while mypart < 2 as libc::c_int {
        iend = *(sxadj[mypart as usize]).offset(snvtxs[mypart as usize] as isize);
        auxadjncy = sadjncy[mypart as usize];
        i = 0 as libc::c_int;
        while i < iend {
            *auxadjncy
                .offset(
                    i as isize,
                ) = *rename.offset(*auxadjncy.offset(i as isize) as isize);
            i += 1;
            i;
        }
        mypart += 1;
        mypart;
    }
    (*lgraph).nedges = snedges[0 as libc::c_int as usize];
    (*rgraph).nedges = snedges[1 as libc::c_int as usize];
    libmetis__SetupGraph_tvwgt(lgraph);
    libmetis__SetupGraph_tvwgt(rgraph);
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint
        != 0
    {
        (*ctrl).SplitTmr += gk_CPUSeconds();
    }
    *r_lgraph = lgraph;
    *r_rgraph = rgraph;
    libmetis__wspacepop(ctrl);
}
