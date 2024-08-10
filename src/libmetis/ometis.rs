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
    fn gk_malloc(nbytes: size_t, msg: *mut libc::c_char) -> *mut libc::c_void;
    fn gk_free(ptr1: *mut *mut libc::c_void, _: ...);
    fn gk_sigtrap() -> libc::c_int;
    fn gk_siguntrap() -> libc::c_int;
    fn gk_CPUSeconds() -> libc::c_double;
    fn libmetis__metis_rcode(sigrval: libc::c_int) -> libc::c_int;
    fn libmetis__Change2FNumberingOrder(
        _: idx_t,
        _: *mut idx_t,
        _: *mut idx_t,
        _: *mut idx_t,
        _: *mut idx_t,
    );
    fn libmetis__FreeCtrl(r_ctrl: *mut *mut ctrl_t);
    fn libmetis__PrintTimers(_: *mut ctrl_t);
    fn libmetis__icopy(n: size_t, a: *mut idx_t, b: *mut idx_t) -> *mut idx_t;
    fn libmetis__FreeGraph(graph: *mut *mut graph_t);
    fn libmetis__wspacepop(ctrl: *mut ctrl_t);
    fn libmetis__genmmd(
        _: idx_t,
        _: *mut idx_t,
        _: *mut idx_t,
        _: *mut idx_t,
        _: *mut idx_t,
        _: idx_t,
        _: *mut idx_t,
        _: *mut idx_t,
        _: *mut idx_t,
        _: *mut idx_t,
        _: idx_t,
        _: *mut idx_t,
    );
    fn libmetis__iwspacemalloc(_: *mut ctrl_t, _: idx_t) -> *mut idx_t;
    fn libmetis__wspacepush(ctrl: *mut ctrl_t);
    fn libmetis__SetupGraph_tvwgt(graph: *mut graph_t);
    fn libmetis__iset(n: size_t, val: idx_t, x: *mut idx_t) -> *mut idx_t;
    fn libmetis__SetupSplitGraph(
        graph: *mut graph_t,
        snvtxs: idx_t,
        snedges: idx_t,
    ) -> *mut graph_t;
    fn libmetis__Compute2WayNodePartitionParams(ctrl: *mut ctrl_t, graph: *mut graph_t);
    fn libmetis__FreeRData(graph: *mut graph_t);
    fn libmetis__Refine2WayNode(
        ctrl: *mut ctrl_t,
        orggraph: *mut graph_t,
        graph: *mut graph_t,
    );
    fn libmetis__InitSeparator(ctrl: *mut ctrl_t, graph: *mut graph_t, niparts: idx_t);
    fn libmetis__CoarsenGraph(ctrl: *mut ctrl_t, graph: *mut graph_t) -> *mut graph_t;
    fn CoarsenGraphNlevels(
        ctrl: *mut ctrl_t,
        graph: *mut graph_t,
        nlevels: idx_t,
    ) -> *mut graph_t;
    fn libmetis__irandArrayPermute(
        n: idx_t,
        p: *mut idx_t,
        nshuffles: idx_t,
        flag: libc::c_int,
    );
    fn libmetis__FindSepInducedComponents(
        _: *mut ctrl_t,
        _: *mut graph_t,
        _: *mut idx_t,
        _: *mut idx_t,
    ) -> idx_t;
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
    fn libmetis__CompressGraph(
        ctrl: *mut ctrl_t,
        nvtxs: idx_t,
        xadj: *mut idx_t,
        adjncy: *mut idx_t,
        vwgt: *mut idx_t,
        cptr: *mut idx_t,
        cind: *mut idx_t,
    ) -> *mut graph_t;
    fn libmetis__imalloc(n: size_t, msg: *mut libc::c_char) -> *mut idx_t;
    fn libmetis__PruneGraph(
        ctrl: *mut ctrl_t,
        nvtxs: idx_t,
        xadj: *mut idx_t,
        adjncy: *mut idx_t,
        vwgt: *mut idx_t,
        iperm: *mut idx_t,
        factor: real_t,
    ) -> *mut graph_t;
    fn libmetis__InitTimers(_: *mut ctrl_t);
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
pub unsafe extern "C" fn METIS_NodeND(
    mut nvtxs: *mut idx_t,
    mut xadj: *mut idx_t,
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
    sigrval = _setjmp(
        (*gk_jbufs.as_mut_ptr().offset(gk_cur_jbufs as isize)).as_mut_ptr(),
    );
    if !(sigrval != 0 as libc::c_int) {
        ctrl = libmetis__SetupCtrl(
            METIS_OP_OMETIS,
            options,
            1 as libc::c_int,
            3 as libc::c_int,
            0 as *mut real_t,
            0 as *mut real_t,
        );
        if ctrl.is_null() {
            gk_siguntrap();
            return METIS_ERROR_INPUT as libc::c_int;
        }
        if (*ctrl).numflag == 1 as libc::c_int {
            libmetis__Change2CNumbering(*nvtxs, xadj, adjncy);
            renumber = 1 as libc::c_int;
        }
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
        if (*ctrl).pfactor as libc::c_double > 0.0f64 {
            piperm = libmetis__imalloc(
                *nvtxs as size_t,
                b"OMETIS: piperm\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            graph = libmetis__PruneGraph(
                ctrl,
                *nvtxs,
                xadj,
                adjncy,
                vwgt,
                piperm,
                (*ctrl).pfactor,
            );
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
                (*nvtxs + 1 as libc::c_int) as size_t,
                b"OMETIS: cptr\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            cind = libmetis__imalloc(
                *nvtxs as size_t,
                b"OMETIS: cind\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            graph = libmetis__CompressGraph(
                ctrl,
                *nvtxs,
                xadj,
                adjncy,
                vwgt,
                cptr,
                cind,
            );
            if graph.is_null() {
                gk_free(
                    &mut cptr as *mut *mut idx_t as *mut *mut libc::c_void,
                    &mut cind as *mut *mut idx_t,
                    0 as *mut *mut libc::c_void,
                );
                (*ctrl).compress = 0 as libc::c_int;
            } else {
                nnvtxs = (*graph).nvtxs;
                (*ctrl)
                    .cfactor = (1.0f64 * *nvtxs as libc::c_double
                    / nnvtxs as libc::c_double) as real_t;
                if (*ctrl).cfactor as libc::c_double > 1.5f64
                    && (*ctrl).nseps == 1 as libc::c_int
                {
                    (*ctrl).nseps = 2 as libc::c_int;
                }
            }
        }
        if (*ctrl).pfactor as libc::c_double == 0.0f64
            && (*ctrl).compress == 0 as libc::c_int
        {
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
                *iperm
                    .offset(
                        *piperm.offset(i as isize) as isize,
                    ) = *perm.offset(i as isize);
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
        while i < *nvtxs {
            *perm.offset(*iperm.offset(i as isize) as isize) = i;
            i += 1;
            i;
        }
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
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_SEPINFO as libc::c_int as libc::c_uint
        != 0
    {
        printf(
            b"Nvtxs: %6d, [%6d %6d %6d]\n\0" as *const u8 as *const libc::c_char,
            (*graph).nvtxs,
            *((*graph).pwgts).offset(0 as libc::c_int as isize),
            *((*graph).pwgts).offset(1 as libc::c_int as isize),
            *((*graph).pwgts).offset(2 as libc::c_int as isize),
        );
    }
    nbnd = (*graph).nbnd;
    bndind = (*graph).bndind;
    label = (*graph).label;
    i = 0 as libc::c_int;
    while i < nbnd {
        lastvtx -= 1;
        *order
            .offset(
                *label.offset(*bndind.offset(i as isize) as isize) as isize,
            ) = lastvtx;
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
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_SEPINFO as libc::c_int as libc::c_uint
        != 0
    {
        printf(
            b"Nvtxs: %6d, [%6d %6d %6d]\n\0" as *const u8 as *const libc::c_char,
            (*graph).nvtxs,
            *((*graph).pwgts).offset(0 as libc::c_int as isize),
            *((*graph).pwgts).offset(1 as libc::c_int as isize),
            *((*graph).pwgts).offset(2 as libc::c_int as isize),
        );
    }
    nbnd = (*graph).nbnd;
    bndind = (*graph).bndind;
    label = (*graph).label;
    i = 0 as libc::c_int;
    while i < nbnd {
        lastvtx -= 1;
        *order
            .offset(
                *label.offset(*bndind.offset(i as isize) as isize) as isize,
            ) = lastvtx;
        i += 1;
        i;
    }
    libmetis__wspacepush(ctrl);
    cptr = libmetis__iwspacemalloc(ctrl, nvtxs + 1 as libc::c_int);
    cind = libmetis__iwspacemalloc(ctrl, nvtxs);
    ncmps = libmetis__FindSepInducedComponents(ctrl, graph, cptr, cind);
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_INFO as libc::c_int as libc::c_uint
        != 0
    {
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
            libmetis__MMDOrder(
                ctrl,
                *sgraphs.offset(i as isize),
                order,
                lastvtx - rnvtxs,
            );
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
    if (*ctrl).nseps == 1 as libc::c_int
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
            if i < (*ctrl).nseps - 1 as libc::c_int {
                libmetis__icopy((*graph).nvtxs as size_t, (*graph).where_0, bestwhere);
            }
        }
        if mincut == 0 as libc::c_int {
            break;
        }
        if i < (*ctrl).nseps - 1 as libc::c_int {
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
    (*ctrl)
        .CoarsenTo = if 100 as libc::c_int >= (*graph).nvtxs / 30 as libc::c_int {
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
            if i < nruns - 1 as libc::c_int {
                libmetis__icopy((*cgraph).nvtxs as size_t, (*cgraph).where_0, bestwhere);
            }
        }
        if mincut == 0 as libc::c_int {
            break;
        }
        if i < nruns - 1 as libc::c_int {
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
    niparts = if 1 as libc::c_int
        >= (if (*cgraph).nvtxs <= (*ctrl).CoarsenTo {
            niparts / 2 as libc::c_int
        } else {
            niparts
        })
    {
        1 as libc::c_int
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
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
    let mut vwgt: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut adjwgt: *mut idx_t = 0 as *mut idx_t;
    let mut label: *mut idx_t = 0 as *mut idx_t;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    let mut bndptr: *mut idx_t = 0 as *mut idx_t;
    let mut bndind: *mut idx_t = 0 as *mut idx_t;
    let mut sxadj: [*mut idx_t; 2] = [0 as *mut idx_t; 2];
    let mut svwgt: [*mut idx_t; 2] = [0 as *mut idx_t; 2];
    let mut sadjncy: [*mut idx_t; 2] = [0 as *mut idx_t; 2];
    let mut sadjwgt: [*mut idx_t; 2] = [0 as *mut idx_t; 2];
    let mut slabel: [*mut idx_t; 2] = [0 as *mut idx_t; 2];
    let mut rename: *mut idx_t = 0 as *mut idx_t;
    let mut auxadjncy: *mut idx_t = 0 as *mut idx_t;
    let mut lgraph: *mut graph_t = 0 as *mut graph_t;
    let mut rgraph: *mut graph_t = 0 as *mut graph_t;
    libmetis__wspacepush(ctrl);
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint
        != 0
    {
        (*ctrl).SplitTmr -= gk_CPUSeconds();
    }
    nvtxs = (*graph).nvtxs;
    xadj = (*graph).xadj;
    vwgt = (*graph).vwgt;
    adjncy = (*graph).adjncy;
    adjwgt = (*graph).adjwgt;
    label = (*graph).label;
    where_0 = (*graph).where_0;
    bndptr = (*graph).bndptr;
    bndind = (*graph).bndind;
    rename = libmetis__iwspacemalloc(ctrl, nvtxs);
    snedges[2 as libc::c_int as usize] = 0 as libc::c_int;
    snedges[1 as libc::c_int as usize] = snedges[2 as libc::c_int as usize];
    snedges[0 as libc::c_int as usize] = snedges[1 as libc::c_int as usize];
    snvtxs[2 as libc::c_int as usize] = snedges[0 as libc::c_int as usize];
    snvtxs[1 as libc::c_int as usize] = snvtxs[2 as libc::c_int as usize];
    snvtxs[0 as libc::c_int as usize] = snvtxs[1 as libc::c_int as usize];
    i = 0 as libc::c_int;
    while i < nvtxs {
        k = *where_0.offset(i as isize);
        let fresh1 = snvtxs[k as usize];
        snvtxs[k as usize] = snvtxs[k as usize] + 1;
        *rename.offset(i as isize) = fresh1;
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
    ii = 0 as libc::c_int;
    while ii < (*graph).nbnd {
        i = *bndind.offset(ii as isize);
        j = *xadj.offset(i as isize);
        while j < *xadj.offset((i + 1 as libc::c_int) as isize) {
            *bndptr.offset(*adjncy.offset(j as isize) as isize) = 1 as libc::c_int;
            j += 1;
            j;
        }
        ii += 1;
        ii;
    }
    snedges[1 as libc::c_int as usize] = 0 as libc::c_int;
    snedges[0 as libc::c_int as usize] = snedges[1 as libc::c_int as usize];
    snvtxs[1 as libc::c_int as usize] = snedges[0 as libc::c_int as usize];
    snvtxs[0 as libc::c_int as usize] = snvtxs[1 as libc::c_int as usize];
    let ref mut fresh2 = *(sxadj[1 as libc::c_int as usize])
        .offset(0 as libc::c_int as isize);
    *fresh2 = 0 as libc::c_int;
    *(sxadj[0 as libc::c_int as usize]).offset(0 as libc::c_int as isize) = *fresh2;
    i = 0 as libc::c_int;
    while i < nvtxs {
        mypart = *where_0.offset(i as isize);
        if !(mypart == 2 as libc::c_int) {
            istart = *xadj.offset(i as isize);
            iend = *xadj.offset((i + 1 as libc::c_int) as isize);
            if *bndptr.offset(i as isize) == -(1 as libc::c_int) {
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
            *(svwgt[mypart as usize])
                .offset(snvtxs[mypart as usize] as isize) = *vwgt.offset(i as isize);
            *(slabel[mypart as usize])
                .offset(snvtxs[mypart as usize] as isize) = *label.offset(i as isize);
            snvtxs[mypart as usize] += 1;
            *(sxadj[mypart as usize])
                .offset(snvtxs[mypart as usize] as isize) = snedges[mypart as usize];
        }
        i += 1;
        i;
    }
    mypart = 0 as libc::c_int;
    while mypart < 2 as libc::c_int {
        iend = snedges[mypart as usize];
        libmetis__iset(iend as size_t, 1 as libc::c_int, sadjwgt[mypart as usize]);
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
    (*lgraph).nvtxs = snvtxs[0 as libc::c_int as usize];
    (*lgraph).nedges = snedges[0 as libc::c_int as usize];
    (*rgraph).nvtxs = snvtxs[1 as libc::c_int as usize];
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
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
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
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint
        != 0
    {
        (*ctrl).SplitTmr -= gk_CPUSeconds();
    }
    nvtxs = (*graph).nvtxs;
    xadj = (*graph).xadj;
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
        j = *xadj.offset(i as isize);
        while j < *xadj.offset((i + 1 as libc::c_int) as isize) {
            *bndptr.offset(*adjncy.offset(j as isize) as isize) = 1 as libc::c_int;
            j += 1;
            j;
        }
        ii += 1;
        ii;
    }
    rename = libmetis__iwspacemalloc(ctrl, nvtxs);
    sgraphs = gk_malloc(
        (::core::mem::size_of::<*mut graph_t>() as libc::c_ulong)
            .wrapping_mul(ncmps as libc::c_ulong),
        b"SplitGraphOrderCC: sgraphs\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    ) as *mut *mut graph_t;
    iii = 0 as libc::c_int;
    while iii < ncmps {
        libmetis__irandArrayPermute(
            *cptr.offset((iii + 1 as libc::c_int) as isize) - *cptr.offset(iii as isize),
            cind.offset(*cptr.offset(iii as isize) as isize),
            *cptr.offset((iii + 1 as libc::c_int) as isize) - *cptr.offset(iii as isize),
            0 as libc::c_int,
        );
        snedges = 0 as libc::c_int;
        snvtxs = snedges;
        j = *cptr.offset(iii as isize);
        while j < *cptr.offset((iii + 1 as libc::c_int) as isize) {
            i = *cind.offset(j as isize);
            let fresh4 = snvtxs;
            snvtxs = snvtxs + 1;
            *rename.offset(i as isize) = fresh4;
            snedges
                += *xadj.offset((i + 1 as libc::c_int) as isize)
                    - *xadj.offset(i as isize);
            j += 1;
            j;
        }
        let ref mut fresh5 = *sgraphs.offset(iii as isize);
        *fresh5 = libmetis__SetupSplitGraph(graph, snvtxs, snedges);
        sxadj = (**sgraphs.offset(iii as isize)).xadj;
        svwgt = (**sgraphs.offset(iii as isize)).vwgt;
        sadjncy = (**sgraphs.offset(iii as isize)).adjncy;
        sadjwgt = (**sgraphs.offset(iii as isize)).adjwgt;
        slabel = (**sgraphs.offset(iii as isize)).label;
        let ref mut fresh6 = *sxadj.offset(0 as libc::c_int as isize);
        *fresh6 = 0 as libc::c_int;
        snedges = *fresh6;
        snvtxs = snedges;
        ii = *cptr.offset(iii as isize);
        while ii < *cptr.offset((iii + 1 as libc::c_int) as isize) {
            i = *cind.offset(ii as isize);
            istart = *xadj.offset(i as isize);
            iend = *xadj.offset((i + 1 as libc::c_int) as isize);
            if *bndptr.offset(i as isize) == -(1 as libc::c_int) {
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
            *sxadj.offset(snvtxs as isize) = snedges;
            ii += 1;
            ii;
        }
        libmetis__iset(snedges as size_t, 1 as libc::c_int, sadjwgt);
        i = 0 as libc::c_int;
        while i < snedges {
            *sadjncy
                .offset(
                    i as isize,
                ) = *rename.offset(*sadjncy.offset(i as isize) as isize);
            i += 1;
            i;
        }
        (**sgraphs.offset(iii as isize)).nvtxs = snvtxs;
        (**sgraphs.offset(iii as isize)).nedges = snedges;
        libmetis__SetupGraph_tvwgt(*sgraphs.offset(iii as isize));
        iii += 1;
        iii;
    }
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint
        != 0
    {
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
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
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
    xadj = (*graph).xadj;
    adjncy = (*graph).adjncy;
    k = *xadj.offset(nvtxs as isize);
    i = 0 as libc::c_int;
    while i < k {
        let ref mut fresh8 = *adjncy.offset(i as isize);
        *fresh8 += 1;
        *fresh8;
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < nvtxs + 1 as libc::c_int {
        let ref mut fresh9 = *xadj.offset(i as isize);
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
        1 as libc::c_int,
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
        *order
            .offset(
                *label.offset(i as isize) as isize,
            ) = firstvtx + *iperm.offset(i as isize) - 1 as libc::c_int;
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < nvtxs + 1 as libc::c_int {
        let ref mut fresh10 = *xadj.offset(i as isize);
        *fresh10 -= 1;
        *fresh10;
        i += 1;
        i;
    }
    k = *xadj.offset(nvtxs as isize);
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
