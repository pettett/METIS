use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn log(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;
    #[thread_local]
    static mut gk_cur_jbufs: libc::c_int;
    #[thread_local]
    static mut gk_jbufs: [jmp_buf; 0];
    fn libmetis__SetupCtrl(
        optype: moptype_et,
        options: *mut idx_t,
        ncon: idx_t,
        nparts: idx_t,
        tpwgts: *mut real_t,
        ubvec: *mut real_t,
    ) -> *mut ctrl_t;
    fn libmetis__Change2CNumbering(_: idx_t, _: *mut idx_t, _: *mut idx_t);
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
    fn libmetis__SetupKWayBalMultipliers(ctrl: *mut ctrl_t, graph: *mut graph_t);
    fn libmetis__IsConnected(graph: *mut graph_t, report: idx_t) -> idx_t;
    fn libmetis__InitTimers(_: *mut ctrl_t);
    fn libmetis__CoarsenGraph(ctrl: *mut ctrl_t, graph: *mut graph_t) -> *mut graph_t;
    fn libmetis__AllocateKWayPartitionMemory(ctrl: *mut ctrl_t, graph: *mut graph_t);
    fn libmetis__FreeWorkSpace(ctrl: *mut ctrl_t);
    fn METIS_SetDefaultOptions(options: *mut idx_t) -> libc::c_int;
    fn libmetis__rmalloc(n: size_t, msg: *mut libc::c_char) -> *mut real_t;
    fn libmetis__AllocateWorkSpace(ctrl: *mut ctrl_t, graph: *mut graph_t);
    fn libmetis__AllocateRefinementWorkSpace(ctrl: *mut ctrl_t, nbrpoolsize: idx_t);
    fn libmetis__RefineKWay(
        ctrl: *mut ctrl_t,
        orggraph: *mut graph_t,
        graph: *mut graph_t,
    );
    fn libmetis__ComputeLoadImbalanceDiff(
        graph: *mut graph_t,
        nparts: idx_t,
        pijbm: *mut real_t,
        ubvec: *mut real_t,
    ) -> real_t;
    fn libmetis__icopy(n: size_t, a: *mut idx_t, b: *mut idx_t) -> *mut idx_t;
    fn libmetis__FreeRData(graph: *mut graph_t);
    fn libmetis__FreeGraph(graph: *mut *mut graph_t);
    fn libmetis__PrintTimers(_: *mut ctrl_t);
    fn libmetis__FreeCtrl(r_ctrl: *mut *mut ctrl_t);
    fn libmetis__Change2FNumbering(
        _: idx_t,
        _: *mut idx_t,
        _: *mut idx_t,
        _: *mut idx_t,
    );
    fn libmetis__metis_rcode(sigrval: libc::c_int) -> libc::c_int;
    fn METIS_PartGraphRecursive(
        nvtxs: *mut idx_t,
        ncon: *mut idx_t,
        xadj: *mut idx_t,
        adjncy: *mut idx_t,
        vwgt: *mut idx_t,
        vsize: *mut idx_t,
        adjwgt: *mut idx_t,
        nparts: *mut idx_t,
        tpwgts: *mut real_t,
        ubvec: *mut real_t,
        options: *mut idx_t,
        edgecut: *mut idx_t,
        part: *mut idx_t,
    ) -> libc::c_int;
    fn gk_malloc_init() -> libc::c_int;
    fn gk_malloc_cleanup(showstats: libc::c_int);
    fn gk_free(ptr1: *mut *mut libc::c_void, _: ...);
    fn gk_errexit(signum: libc::c_int, _: *mut libc::c_char, _: ...);
    fn gk_sigtrap() -> libc::c_int;
    fn gk_siguntrap() -> libc::c_int;
    fn gk_CPUSeconds() -> libc::c_double;
    fn gk_log2(_: libc::c_int) -> libc::c_int;
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
    sigrval = _setjmp(
        (*gk_jbufs.as_mut_ptr().offset(gk_cur_jbufs as isize)).as_mut_ptr(),
    );
    if !(sigrval != 0 as libc::c_int) {
        ctrl = libmetis__SetupCtrl(
            METIS_OP_KMETIS,
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
        libmetis__SetupKWayBalMultipliers(ctrl, graph);
        (*ctrl)
            .CoarsenTo = if *nvtxs / (20 as libc::c_int * gk_log2(*nparts))
            >= 30 as libc::c_int * *nparts
        {
            *nvtxs / (20 as libc::c_int * gk_log2(*nparts))
        } else {
            30 as libc::c_int * *nparts
        };
        (*ctrl)
            .nIparts = if (*ctrl).CoarsenTo == 30 as libc::c_int * *nparts {
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
        *objval = libmetis__MlevelKWayPartitioning(ctrl, graph, part);
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
        if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint
            != 0
        {
            (*ctrl).InitPartTmr -= gk_CPUSeconds();
        }
        libmetis__AllocateKWayPartitionMemory(ctrl, cgraph);
        libmetis__FreeWorkSpace(ctrl);
        libmetis__InitKWayPartitioning(ctrl, cgraph);
        libmetis__AllocateWorkSpace(ctrl, graph);
        libmetis__AllocateRefinementWorkSpace(ctrl, 2 as libc::c_int * (*cgraph).nedges);
        if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint
            != 0
        {
            (*ctrl).InitPartTmr += gk_CPUSeconds();
        }
        if (*ctrl).dbglvl as libc::c_uint
            & METIS_DBG_IPART as libc::c_int as libc::c_uint != 0
        {
            printf(
                b"Initial %d-way partitioning cut: %d\n\0" as *const u8
                    as *const libc::c_char,
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
    options[METIS_OPTION_OBJTYPE as libc::c_int
        as usize] = METIS_OBJTYPE_CUT as libc::c_int;
    options[METIS_OPTION_NO2HOP as libc::c_int as usize] = (*ctrl).no2hop;
    ubvec = libmetis__rmalloc(
        (*graph).ncon as size_t,
        b"InitKWayPartitioning: ubvec\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < (*graph).ncon {
        *ubvec
            .offset(
                i as isize,
            ) = pow(
            *((*ctrl).ubfactors).offset(i as isize) as libc::c_double,
            1.0f64 / log((*ctrl).nparts as libc::c_double),
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
                    b"Failed during initial partitioning\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
            }
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
    gk_free(
        &mut ubvec as *mut *mut real_t as *mut *mut libc::c_void,
        &mut bestwhere as *mut *mut idx_t,
        0 as *mut *mut libc::c_void,
    );
}
