use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn gk_errexit(signum: libc::c_int, _: *mut libc::c_char, _: ...);
    fn gk_CPUSeconds() -> libc::c_double;
    fn libmetis__isum(n: size_t, x: *mut idx_t, incx: size_t) -> idx_t;
    fn libmetis__iaxpy(
        n: size_t,
        alpha: idx_t,
        x: *mut idx_t,
        incx: size_t,
        y: *mut idx_t,
        incy: size_t,
    ) -> *mut idx_t;
    fn libmetis__imalloc(n: size_t, msg: *mut libc::c_char) -> *mut idx_t;
    fn libmetis__irealloc(
        ptr: *mut idx_t,
        n: size_t,
        msg: *mut libc::c_char,
    ) -> *mut idx_t;
    fn libmetis__iset(n: size_t, val: idx_t, x: *mut idx_t) -> *mut idx_t;
    fn libmetis__icopy(n: size_t, a: *mut idx_t, b: *mut idx_t) -> *mut idx_t;
    fn libmetis__rmalloc(n: size_t, msg: *mut libc::c_char) -> *mut real_t;
    fn libmetis__irandArrayPermute(
        n: idx_t,
        p: *mut idx_t,
        nshuffles: idx_t,
        flag: libc::c_int,
    );
    fn libmetis__ikvsorti(n: size_t, base: *mut ikv_t);
    fn libmetis__BucketSortKeysInc(
        ctrl: *mut ctrl_t,
        n: idx_t,
        max: idx_t,
        keys: *mut idx_t,
        tperm: *mut idx_t,
        perm: *mut idx_t,
    );
    fn libmetis__wspacepop(ctrl: *mut ctrl_t);
    fn libmetis__iwspacemalloc(_: *mut ctrl_t, _: idx_t) -> *mut idx_t;
    fn libmetis__CreateGraph() -> *mut graph_t;
    fn libmetis__wspacepush(ctrl: *mut ctrl_t);
    fn libmetis__ikvwspacemalloc(_: *mut ctrl_t, _: idx_t) -> *mut ikv_t;
    fn libmetis__ivecaxpylez(
        n: idx_t,
        a: idx_t,
        x: *mut idx_t,
        y: *mut idx_t,
        z: *mut idx_t,
    ) -> libc::c_int;
    fn libmetis__BetterVBalance(
        ncon: idx_t,
        itvwgt: *mut real_t,
        v_vwgt: *mut idx_t,
        u1_vwgt: *mut idx_t,
        u2_vwgt: *mut idx_t,
    ) -> libc::c_int;
    fn libmetis__ivecle(n: idx_t, x: *mut idx_t, z: *mut idx_t) -> libc::c_int;
}
pub type __int32_t = libc::c_int;
pub type __ssize_t = libc::c_long;
pub type int32_t = __int32_t;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
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
pub type moptype_et = libc::c_uint;
pub const METIS_OP_OMETIS: moptype_et = 2;
pub const METIS_OP_KMETIS: moptype_et = 1;
pub const METIS_OP_PMETIS: moptype_et = 0;
pub type mctype_et = libc::c_uint;
pub const METIS_CTYPE_SHEM: mctype_et = 1;
pub const METIS_CTYPE_RM: mctype_et = 0;
pub type miptype_et = libc::c_uint;
pub const METIS_IPTYPE_METISRB: miptype_et = 4;
pub const METIS_IPTYPE_NODE: miptype_et = 3;
pub const METIS_IPTYPE_EDGE: miptype_et = 2;
pub const METIS_IPTYPE_RANDOM: miptype_et = 1;
pub const METIS_IPTYPE_GROW: miptype_et = 0;
pub type mrtype_et = libc::c_uint;
pub const METIS_RTYPE_SEP1SIDED: mrtype_et = 3;
pub const METIS_RTYPE_SEP2SIDED: mrtype_et = 2;
pub const METIS_RTYPE_GREEDY: mrtype_et = 1;
pub const METIS_RTYPE_FM: mrtype_et = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ikv_t {
    pub key: idx_t,
    pub val: idx_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cnbr_t {
    pub pid: idx_t,
    pub ed: idx_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ckrinfo_t {
    pub id: idx_t,
    pub ed: idx_t,
    pub nnbrs: idx_t,
    pub inbr: idx_t,
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
pub struct vkrinfo_t {
    pub nid: idx_t,
    pub ned: idx_t,
    pub gv: idx_t,
    pub nnbrs: idx_t,
    pub inbr: idx_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nrinfo_t {
    pub edegrees: [idx_t; 2],
}
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
#[no_mangle]
pub unsafe extern "C" fn libmetis__CoarsenGraph(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
) -> *mut graph_t {
    let mut i: idx_t = 0;
    let mut eqewgts: idx_t = 0;
    let mut level: idx_t = 0 as libc::c_int;
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint
        != 0
    {
        (*ctrl).CoarsenTmr -= gk_CPUSeconds();
    }
    eqewgts = 1 as libc::c_int;
    i = 1 as libc::c_int;
    while i < (*graph).nedges {
        if *((*graph).adjwgt).offset(0 as libc::c_int as isize)
            != *((*graph).adjwgt).offset(i as isize)
        {
            eqewgts = 0 as libc::c_int;
            break;
        } else {
            i += 1;
            i;
        }
    }
    i = 0 as libc::c_int;
    while i < (*graph).ncon {
        *((*ctrl).maxvwgt)
            .offset(
                i as isize,
            ) = (1.5f64 * *((*graph).tvwgt).offset(i as isize) as libc::c_double
            / (*ctrl).CoarsenTo as libc::c_double) as idx_t;
        i += 1;
        i;
    }
    loop {
        if (*ctrl).dbglvl as libc::c_uint
            & METIS_DBG_COARSEN as libc::c_int as libc::c_uint != 0
        {
            libmetis__PrintCGraphStats(ctrl, graph);
        }
        if ((*graph).cmap).is_null() {
            (*graph)
                .cmap = libmetis__imalloc(
                (*graph).nvtxs as size_t,
                b"CoarsenGraph: graph->cmap\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        match (*ctrl).ctype as libc::c_uint {
            0 => {
                libmetis__Match_RM(ctrl, graph);
            }
            1 => {
                if eqewgts != 0 || (*graph).nedges == 0 as libc::c_int {
                    libmetis__Match_RM(ctrl, graph);
                } else {
                    libmetis__Match_SHEM(ctrl, graph);
                }
            }
            _ => {
                gk_errexit(
                    15 as libc::c_int,
                    b"Unknown ctype: %d\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    (*ctrl).ctype as libc::c_uint,
                );
            }
        }
        graph = (*graph).coarser;
        eqewgts = 0 as libc::c_int;
        level += 1;
        level;
        if !((*graph).nvtxs > (*ctrl).CoarsenTo
            && ((*graph).nvtxs as libc::c_double)
                < 0.85f64 * (*(*graph).finer).nvtxs as libc::c_double
            && (*graph).nedges > (*graph).nvtxs / 2 as libc::c_int)
        {
            break;
        }
    }
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_COARSEN as libc::c_int as libc::c_uint
        != 0
    {
        libmetis__PrintCGraphStats(ctrl, graph);
    }
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint
        != 0
    {
        (*ctrl).CoarsenTmr += gk_CPUSeconds();
    }
    return graph;
}
#[no_mangle]
pub unsafe extern "C" fn CoarsenGraphNlevels(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut nlevels: idx_t,
) -> *mut graph_t {
    let mut i: idx_t = 0;
    let mut eqewgts: idx_t = 0;
    let mut level: idx_t = 0;
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint
        != 0
    {
        (*ctrl).CoarsenTmr -= gk_CPUSeconds();
    }
    eqewgts = 1 as libc::c_int;
    i = 1 as libc::c_int;
    while i < (*graph).nedges {
        if *((*graph).adjwgt).offset(0 as libc::c_int as isize)
            != *((*graph).adjwgt).offset(i as isize)
        {
            eqewgts = 0 as libc::c_int;
            break;
        } else {
            i += 1;
            i;
        }
    }
    i = 0 as libc::c_int;
    while i < (*graph).ncon {
        *((*ctrl).maxvwgt)
            .offset(
                i as isize,
            ) = (1.5f64 * *((*graph).tvwgt).offset(i as isize) as libc::c_double
            / (*ctrl).CoarsenTo as libc::c_double) as idx_t;
        i += 1;
        i;
    }
    level = 0 as libc::c_int;
    while level < nlevels {
        if (*ctrl).dbglvl as libc::c_uint
            & METIS_DBG_COARSEN as libc::c_int as libc::c_uint != 0
        {
            libmetis__PrintCGraphStats(ctrl, graph);
        }
        if ((*graph).cmap).is_null() {
            (*graph)
                .cmap = libmetis__imalloc(
                (*graph).nvtxs as size_t,
                b"CoarsenGraph: graph->cmap\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        match (*ctrl).ctype as libc::c_uint {
            0 => {
                libmetis__Match_RM(ctrl, graph);
            }
            1 => {
                if eqewgts != 0 || (*graph).nedges == 0 as libc::c_int {
                    libmetis__Match_RM(ctrl, graph);
                } else {
                    libmetis__Match_SHEM(ctrl, graph);
                }
            }
            _ => {
                gk_errexit(
                    15 as libc::c_int,
                    b"Unknown ctype: %d\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    (*ctrl).ctype as libc::c_uint,
                );
            }
        }
        graph = (*graph).coarser;
        eqewgts = 0 as libc::c_int;
        if (*graph).nvtxs < (*ctrl).CoarsenTo
            || (*graph).nvtxs as libc::c_double
                > 0.85f64 * (*(*graph).finer).nvtxs as libc::c_double
            || (*graph).nedges < (*graph).nvtxs / 2 as libc::c_int
        {
            break;
        }
        level += 1;
        level;
    }
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_COARSEN as libc::c_int as libc::c_uint
        != 0
    {
        libmetis__PrintCGraphStats(ctrl, graph);
    }
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint
        != 0
    {
        (*ctrl).CoarsenTmr += gk_CPUSeconds();
    }
    return graph;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__Match_RM(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
) -> idx_t {
    let mut i: idx_t = 0;
    let mut pi: idx_t = 0;
    let mut ii: idx_t = 0;
    let mut j: idx_t = 0;
    let mut jj: idx_t = 0;
    let mut jjinc: idx_t = 0;
    let mut k: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut ncon: idx_t = 0;
    let mut cnvtxs: idx_t = 0;
    let mut maxidx: idx_t = 0;
    let mut last_unmatched: idx_t = 0;
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
    let mut vwgt: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut adjwgt: *mut idx_t = 0 as *mut idx_t;
    let mut maxvwgt: *mut idx_t = 0 as *mut idx_t;
    let mut match_0: *mut idx_t = 0 as *mut idx_t;
    let mut cmap: *mut idx_t = 0 as *mut idx_t;
    let mut perm: *mut idx_t = 0 as *mut idx_t;
    let mut nunmatched: size_t = 0 as libc::c_int as size_t;
    libmetis__wspacepush(ctrl);
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint
        != 0
    {
        (*ctrl).MatchTmr -= gk_CPUSeconds();
    }
    nvtxs = (*graph).nvtxs;
    ncon = (*graph).ncon;
    xadj = (*graph).xadj;
    vwgt = (*graph).vwgt;
    adjncy = (*graph).adjncy;
    adjwgt = (*graph).adjwgt;
    cmap = (*graph).cmap;
    maxvwgt = (*ctrl).maxvwgt;
    match_0 = libmetis__iset(
        nvtxs as size_t,
        -(1 as libc::c_int),
        libmetis__iwspacemalloc(ctrl, nvtxs),
    );
    perm = libmetis__iwspacemalloc(ctrl, nvtxs);
    libmetis__irandArrayPermute(nvtxs, perm, nvtxs / 8 as libc::c_int, 1 as libc::c_int);
    cnvtxs = 0 as libc::c_int;
    last_unmatched = 0 as libc::c_int;
    pi = 0 as libc::c_int;
    while pi < nvtxs {
        i = *perm.offset(pi as isize);
        if *match_0.offset(i as isize) == -(1 as libc::c_int) {
            maxidx = i;
            if if ncon == 1 as libc::c_int {
                (*vwgt.offset(i as isize) < *maxvwgt.offset(0 as libc::c_int as isize))
                    as libc::c_int
            } else {
                libmetis__ivecle(ncon, vwgt.offset((i * ncon) as isize), maxvwgt)
            } != 0
            {
                if *xadj.offset(i as isize)
                    == *xadj.offset((i + 1 as libc::c_int) as isize)
                {
                    last_unmatched = (if pi >= last_unmatched {
                        pi
                    } else {
                        last_unmatched
                    }) + 1 as libc::c_int;
                    while last_unmatched < nvtxs {
                        j = *perm.offset(last_unmatched as isize);
                        if *match_0.offset(j as isize) == -(1 as libc::c_int) {
                            maxidx = j;
                            break;
                        } else {
                            last_unmatched += 1;
                            last_unmatched;
                        }
                    }
                } else if ncon == 1 as libc::c_int {
                    j = *xadj.offset(i as isize);
                    while j < *xadj.offset((i + 1 as libc::c_int) as isize) {
                        k = *adjncy.offset(j as isize);
                        if *match_0.offset(k as isize) == -(1 as libc::c_int)
                            && *vwgt.offset(i as isize) + *vwgt.offset(k as isize)
                                <= *maxvwgt.offset(0 as libc::c_int as isize)
                        {
                            maxidx = k;
                            break;
                        } else {
                            j += 1;
                            j;
                        }
                    }
                    if maxidx == i
                        && 3 as libc::c_int * *vwgt.offset(i as isize)
                            < *maxvwgt.offset(0 as libc::c_int as isize)
                    {
                        nunmatched = nunmatched.wrapping_add(1);
                        nunmatched;
                        maxidx = -(1 as libc::c_int);
                    }
                } else {
                    j = *xadj.offset(i as isize);
                    while j < *xadj.offset((i + 1 as libc::c_int) as isize) {
                        k = *adjncy.offset(j as isize);
                        if *match_0.offset(k as isize) == -(1 as libc::c_int)
                            && libmetis__ivecaxpylez(
                                ncon,
                                1 as libc::c_int,
                                vwgt.offset((i * ncon) as isize),
                                vwgt.offset((k * ncon) as isize),
                                maxvwgt,
                            ) != 0
                        {
                            maxidx = k;
                            break;
                        } else {
                            j += 1;
                            j;
                        }
                    }
                    if maxidx == i
                        && libmetis__ivecaxpylez(
                            ncon,
                            2 as libc::c_int,
                            vwgt.offset((i * ncon) as isize),
                            vwgt.offset((i * ncon) as isize),
                            maxvwgt,
                        ) != 0
                    {
                        nunmatched = nunmatched.wrapping_add(1);
                        nunmatched;
                        maxidx = -(1 as libc::c_int);
                    }
                }
            }
            if maxidx != -(1 as libc::c_int) {
                let fresh0 = cnvtxs;
                cnvtxs = cnvtxs + 1;
                let ref mut fresh1 = *cmap.offset(maxidx as isize);
                *fresh1 = fresh0;
                *cmap.offset(i as isize) = *fresh1;
                *match_0.offset(i as isize) = maxidx;
                *match_0.offset(maxidx as isize) = i;
            }
        }
        pi += 1;
        pi;
    }
    if (*ctrl).no2hop == 0
        && nunmatched as libc::c_double > 0.10f64 * nvtxs as libc::c_double
    {
        cnvtxs = libmetis__Match_2Hop(ctrl, graph, perm, match_0, cnvtxs, nunmatched);
    }
    cnvtxs = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < nvtxs {
        if *match_0.offset(i as isize) == -(1 as libc::c_int) {
            *match_0.offset(i as isize) = i;
            let fresh2 = cnvtxs;
            cnvtxs = cnvtxs + 1;
            *cmap.offset(i as isize) = fresh2;
        } else if i <= *match_0.offset(i as isize) {
            let fresh3 = cnvtxs;
            cnvtxs = cnvtxs + 1;
            let ref mut fresh4 = *cmap.offset(*match_0.offset(i as isize) as isize);
            *fresh4 = fresh3;
            *cmap.offset(i as isize) = *fresh4;
        }
        i += 1;
        i;
    }
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint
        != 0
    {
        (*ctrl).MatchTmr += gk_CPUSeconds();
    }
    libmetis__CreateCoarseGraph(ctrl, graph, cnvtxs, match_0);
    libmetis__wspacepop(ctrl);
    return cnvtxs;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__Match_SHEM(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
) -> idx_t {
    let mut i: idx_t = 0;
    let mut pi: idx_t = 0;
    let mut ii: idx_t = 0;
    let mut j: idx_t = 0;
    let mut jj: idx_t = 0;
    let mut jjinc: idx_t = 0;
    let mut k: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut ncon: idx_t = 0;
    let mut cnvtxs: idx_t = 0;
    let mut maxidx: idx_t = 0;
    let mut maxwgt: idx_t = 0;
    let mut last_unmatched: idx_t = 0;
    let mut avgdegree: idx_t = 0;
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
    let mut vwgt: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut adjwgt: *mut idx_t = 0 as *mut idx_t;
    let mut maxvwgt: *mut idx_t = 0 as *mut idx_t;
    let mut match_0: *mut idx_t = 0 as *mut idx_t;
    let mut cmap: *mut idx_t = 0 as *mut idx_t;
    let mut degrees: *mut idx_t = 0 as *mut idx_t;
    let mut perm: *mut idx_t = 0 as *mut idx_t;
    let mut tperm: *mut idx_t = 0 as *mut idx_t;
    let mut nunmatched: size_t = 0 as libc::c_int as size_t;
    libmetis__wspacepush(ctrl);
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint
        != 0
    {
        (*ctrl).MatchTmr -= gk_CPUSeconds();
    }
    nvtxs = (*graph).nvtxs;
    ncon = (*graph).ncon;
    xadj = (*graph).xadj;
    vwgt = (*graph).vwgt;
    adjncy = (*graph).adjncy;
    adjwgt = (*graph).adjwgt;
    cmap = (*graph).cmap;
    maxvwgt = (*ctrl).maxvwgt;
    match_0 = libmetis__iset(
        nvtxs as size_t,
        -(1 as libc::c_int),
        libmetis__iwspacemalloc(ctrl, nvtxs),
    );
    perm = libmetis__iwspacemalloc(ctrl, nvtxs);
    tperm = libmetis__iwspacemalloc(ctrl, nvtxs);
    degrees = libmetis__iwspacemalloc(ctrl, nvtxs);
    libmetis__irandArrayPermute(
        nvtxs,
        tperm,
        nvtxs / 8 as libc::c_int,
        1 as libc::c_int,
    );
    avgdegree = (0.7f64 * (*xadj.offset(nvtxs as isize) / nvtxs) as libc::c_double)
        as idx_t;
    i = 0 as libc::c_int;
    while i < nvtxs {
        *degrees
            .offset(
                i as isize,
            ) = if *xadj.offset((i + 1 as libc::c_int) as isize)
            - *xadj.offset(i as isize) > avgdegree
        {
            avgdegree
        } else {
            *xadj.offset((i + 1 as libc::c_int) as isize) - *xadj.offset(i as isize)
        };
        i += 1;
        i;
    }
    libmetis__BucketSortKeysInc(ctrl, nvtxs, avgdegree, degrees, tperm, perm);
    cnvtxs = 0 as libc::c_int;
    last_unmatched = 0 as libc::c_int;
    pi = 0 as libc::c_int;
    while pi < nvtxs {
        i = *perm.offset(pi as isize);
        if *match_0.offset(i as isize) == -(1 as libc::c_int) {
            maxidx = i;
            maxwgt = -(1 as libc::c_int);
            if if ncon == 1 as libc::c_int {
                (*vwgt.offset(i as isize) < *maxvwgt.offset(0 as libc::c_int as isize))
                    as libc::c_int
            } else {
                libmetis__ivecle(ncon, vwgt.offset((i * ncon) as isize), maxvwgt)
            } != 0
            {
                if *xadj.offset(i as isize)
                    == *xadj.offset((i + 1 as libc::c_int) as isize)
                {
                    last_unmatched = (if pi >= last_unmatched {
                        pi
                    } else {
                        last_unmatched
                    }) + 1 as libc::c_int;
                    while last_unmatched < nvtxs {
                        j = *perm.offset(last_unmatched as isize);
                        if *match_0.offset(j as isize) == -(1 as libc::c_int) {
                            maxidx = j;
                            break;
                        } else {
                            last_unmatched += 1;
                            last_unmatched;
                        }
                    }
                } else if ncon == 1 as libc::c_int {
                    j = *xadj.offset(i as isize);
                    while j < *xadj.offset((i + 1 as libc::c_int) as isize) {
                        k = *adjncy.offset(j as isize);
                        if *match_0.offset(k as isize) == -(1 as libc::c_int)
                            && maxwgt < *adjwgt.offset(j as isize)
                            && *vwgt.offset(i as isize) + *vwgt.offset(k as isize)
                                <= *maxvwgt.offset(0 as libc::c_int as isize)
                        {
                            maxidx = k;
                            maxwgt = *adjwgt.offset(j as isize);
                        }
                        j += 1;
                        j;
                    }
                    if maxidx == i
                        && 3 as libc::c_int * *vwgt.offset(i as isize)
                            < *maxvwgt.offset(0 as libc::c_int as isize)
                    {
                        nunmatched = nunmatched.wrapping_add(1);
                        nunmatched;
                        maxidx = -(1 as libc::c_int);
                    }
                } else {
                    j = *xadj.offset(i as isize);
                    while j < *xadj.offset((i + 1 as libc::c_int) as isize) {
                        k = *adjncy.offset(j as isize);
                        if *match_0.offset(k as isize) == -(1 as libc::c_int)
                            && libmetis__ivecaxpylez(
                                ncon,
                                1 as libc::c_int,
                                vwgt.offset((i * ncon) as isize),
                                vwgt.offset((k * ncon) as isize),
                                maxvwgt,
                            ) != 0
                            && (maxwgt < *adjwgt.offset(j as isize)
                                || maxwgt == *adjwgt.offset(j as isize)
                                    && libmetis__BetterVBalance(
                                        ncon,
                                        (*graph).invtvwgt,
                                        vwgt.offset((i * ncon) as isize),
                                        vwgt.offset((maxidx * ncon) as isize),
                                        vwgt.offset((k * ncon) as isize),
                                    ) != 0)
                        {
                            maxidx = k;
                            maxwgt = *adjwgt.offset(j as isize);
                        }
                        j += 1;
                        j;
                    }
                    if maxidx == i
                        && libmetis__ivecaxpylez(
                            ncon,
                            2 as libc::c_int,
                            vwgt.offset((i * ncon) as isize),
                            vwgt.offset((i * ncon) as isize),
                            maxvwgt,
                        ) != 0
                    {
                        nunmatched = nunmatched.wrapping_add(1);
                        nunmatched;
                        maxidx = -(1 as libc::c_int);
                    }
                }
            }
            if maxidx != -(1 as libc::c_int) {
                let fresh5 = cnvtxs;
                cnvtxs = cnvtxs + 1;
                let ref mut fresh6 = *cmap.offset(maxidx as isize);
                *fresh6 = fresh5;
                *cmap.offset(i as isize) = *fresh6;
                *match_0.offset(i as isize) = maxidx;
                *match_0.offset(maxidx as isize) = i;
            }
        }
        pi += 1;
        pi;
    }
    if (*ctrl).no2hop == 0
        && nunmatched as libc::c_double > 0.10f64 * nvtxs as libc::c_double
    {
        cnvtxs = libmetis__Match_2Hop(ctrl, graph, perm, match_0, cnvtxs, nunmatched);
    }
    cnvtxs = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < nvtxs {
        if *match_0.offset(i as isize) == -(1 as libc::c_int) {
            *match_0.offset(i as isize) = i;
            let fresh7 = cnvtxs;
            cnvtxs = cnvtxs + 1;
            *cmap.offset(i as isize) = fresh7;
        } else if i <= *match_0.offset(i as isize) {
            let fresh8 = cnvtxs;
            cnvtxs = cnvtxs + 1;
            let ref mut fresh9 = *cmap.offset(*match_0.offset(i as isize) as isize);
            *fresh9 = fresh8;
            *cmap.offset(i as isize) = *fresh9;
        }
        i += 1;
        i;
    }
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint
        != 0
    {
        (*ctrl).MatchTmr += gk_CPUSeconds();
    }
    libmetis__CreateCoarseGraph(ctrl, graph, cnvtxs, match_0);
    libmetis__wspacepop(ctrl);
    return cnvtxs;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__Match_2Hop(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut perm: *mut idx_t,
    mut match_0: *mut idx_t,
    mut cnvtxs: idx_t,
    mut nunmatched: size_t,
) -> idx_t {
    cnvtxs = libmetis__Match_2HopAny(
        ctrl,
        graph,
        perm,
        match_0,
        cnvtxs,
        &mut nunmatched,
        2 as libc::c_int as size_t,
    );
    cnvtxs = libmetis__Match_2HopAll(
        ctrl,
        graph,
        perm,
        match_0,
        cnvtxs,
        &mut nunmatched,
        64 as libc::c_int as size_t,
    );
    if nunmatched as libc::c_double > 1.5f64 * 0.10f64 * (*graph).nvtxs as libc::c_double
    {
        cnvtxs = libmetis__Match_2HopAny(
            ctrl,
            graph,
            perm,
            match_0,
            cnvtxs,
            &mut nunmatched,
            3 as libc::c_int as size_t,
        );
    }
    if nunmatched as libc::c_double > 2.0f64 * 0.10f64 * (*graph).nvtxs as libc::c_double
    {
        cnvtxs = libmetis__Match_2HopAny(
            ctrl,
            graph,
            perm,
            match_0,
            cnvtxs,
            &mut nunmatched,
            (*graph).nvtxs as size_t,
        );
    }
    return cnvtxs;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__Match_2HopAny(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut perm: *mut idx_t,
    mut match_0: *mut idx_t,
    mut cnvtxs: idx_t,
    mut r_nunmatched: *mut size_t,
    mut maxdegree: size_t,
) -> idx_t {
    let mut i: idx_t = 0;
    let mut pi: idx_t = 0;
    let mut ii: idx_t = 0;
    let mut j: idx_t = 0;
    let mut jj: idx_t = 0;
    let mut k: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut colptr: *mut idx_t = 0 as *mut idx_t;
    let mut rowind: *mut idx_t = 0 as *mut idx_t;
    let mut cmap: *mut idx_t = 0 as *mut idx_t;
    let mut nunmatched: size_t = 0;
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint
        != 0
    {
        (*ctrl).Aux3Tmr -= gk_CPUSeconds();
    }
    nvtxs = (*graph).nvtxs;
    xadj = (*graph).xadj;
    adjncy = (*graph).adjncy;
    cmap = (*graph).cmap;
    nunmatched = *r_nunmatched;
    libmetis__wspacepush(ctrl);
    colptr = libmetis__iset(
        nvtxs as size_t,
        0 as libc::c_int,
        libmetis__iwspacemalloc(ctrl, nvtxs + 1 as libc::c_int),
    );
    i = 0 as libc::c_int;
    while i < nvtxs {
        if *match_0.offset(i as isize) == -(1 as libc::c_int)
            && ((*xadj.offset((i + 1 as libc::c_int) as isize)
                - *xadj.offset(i as isize)) as libc::c_ulong) < maxdegree
        {
            j = *xadj.offset(i as isize);
            while j < *xadj.offset((i + 1 as libc::c_int) as isize) {
                let ref mut fresh10 = *colptr
                    .offset(*adjncy.offset(j as isize) as isize);
                *fresh10 += 1;
                *fresh10;
                j += 1;
                j;
            }
        }
        i += 1;
        i;
    }
    i = 1 as libc::c_int;
    while i < nvtxs {
        let ref mut fresh11 = *colptr.offset(i as isize);
        *fresh11 += *colptr.offset((i - 1 as libc::c_int) as isize);
        i += 1;
        i;
    }
    i = nvtxs;
    while i > 0 as libc::c_int {
        *colptr.offset(i as isize) = *colptr.offset((i - 1 as libc::c_int) as isize);
        i -= 1;
        i;
    }
    *colptr.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    rowind = libmetis__iwspacemalloc(ctrl, *colptr.offset(nvtxs as isize));
    pi = 0 as libc::c_int;
    while pi < nvtxs {
        i = *perm.offset(pi as isize);
        if *match_0.offset(i as isize) == -(1 as libc::c_int)
            && ((*xadj.offset((i + 1 as libc::c_int) as isize)
                - *xadj.offset(i as isize)) as libc::c_ulong) < maxdegree
        {
            j = *xadj.offset(i as isize);
            while j < *xadj.offset((i + 1 as libc::c_int) as isize) {
                let ref mut fresh12 = *colptr
                    .offset(*adjncy.offset(j as isize) as isize);
                let fresh13 = *fresh12;
                *fresh12 = *fresh12 + 1;
                *rowind.offset(fresh13 as isize) = i;
                j += 1;
                j;
            }
        }
        pi += 1;
        pi;
    }
    i = nvtxs;
    while i > 0 as libc::c_int {
        *colptr.offset(i as isize) = *colptr.offset((i - 1 as libc::c_int) as isize);
        i -= 1;
        i;
    }
    *colptr.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    pi = 0 as libc::c_int;
    while pi < nvtxs {
        i = *perm.offset(pi as isize);
        if !(*colptr.offset((i + 1 as libc::c_int) as isize) - *colptr.offset(i as isize)
            < 2 as libc::c_int)
        {
            jj = *colptr.offset((i + 1 as libc::c_int) as isize);
            j = *colptr.offset(i as isize);
            while j < jj {
                if *match_0.offset(*rowind.offset(j as isize) as isize)
                    == -(1 as libc::c_int)
                {
                    jj -= 1;
                    jj;
                    while jj > j {
                        if *match_0.offset(*rowind.offset(jj as isize) as isize)
                            == -(1 as libc::c_int)
                        {
                            let fresh14 = cnvtxs;
                            cnvtxs = cnvtxs + 1;
                            let ref mut fresh15 = *cmap
                                .offset(*rowind.offset(jj as isize) as isize);
                            *fresh15 = fresh14;
                            *cmap.offset(*rowind.offset(j as isize) as isize) = *fresh15;
                            *match_0
                                .offset(
                                    *rowind.offset(j as isize) as isize,
                                ) = *rowind.offset(jj as isize);
                            *match_0
                                .offset(
                                    *rowind.offset(jj as isize) as isize,
                                ) = *rowind.offset(j as isize);
                            nunmatched = (nunmatched as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong) as size_t
                                as size_t;
                            break;
                        } else {
                            jj -= 1;
                            jj;
                        }
                    }
                }
                j += 1;
                j;
            }
        }
        pi += 1;
        pi;
    }
    libmetis__wspacepop(ctrl);
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint
        != 0
    {
        (*ctrl).Aux3Tmr += gk_CPUSeconds();
    }
    *r_nunmatched = nunmatched;
    return cnvtxs;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__Match_2HopAll(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut perm: *mut idx_t,
    mut match_0: *mut idx_t,
    mut cnvtxs: idx_t,
    mut r_nunmatched: *mut size_t,
    mut maxdegree: size_t,
) -> idx_t {
    let mut i: idx_t = 0;
    let mut pi: idx_t = 0;
    let mut pk: idx_t = 0;
    let mut ii: idx_t = 0;
    let mut j: idx_t = 0;
    let mut jj: idx_t = 0;
    let mut k: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut mask: idx_t = 0;
    let mut idegree: idx_t = 0;
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut cmap: *mut idx_t = 0 as *mut idx_t;
    let mut mark: *mut idx_t = 0 as *mut idx_t;
    let mut keys: *mut ikv_t = 0 as *mut ikv_t;
    let mut nunmatched: size_t = 0;
    let mut ncand: size_t = 0;
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint
        != 0
    {
        (*ctrl).Aux3Tmr -= gk_CPUSeconds();
    }
    nvtxs = (*graph).nvtxs;
    xadj = (*graph).xadj;
    adjncy = (*graph).adjncy;
    cmap = (*graph).cmap;
    nunmatched = *r_nunmatched;
    mask = (2147483647 as libc::c_int as libc::c_ulong).wrapping_div(maxdegree) as idx_t;
    libmetis__wspacepush(ctrl);
    keys = libmetis__ikvwspacemalloc(ctrl, nunmatched as idx_t);
    ncand = 0 as libc::c_int as size_t;
    pi = 0 as libc::c_int;
    while pi < nvtxs {
        i = *perm.offset(pi as isize);
        idegree = *xadj.offset((i + 1 as libc::c_int) as isize)
            - *xadj.offset(i as isize);
        if *match_0.offset(i as isize) == -(1 as libc::c_int)
            && idegree > 1 as libc::c_int && (idegree as libc::c_ulong) < maxdegree
        {
            k = 0 as libc::c_int;
            j = *xadj.offset(i as isize);
            while j < *xadj.offset((i + 1 as libc::c_int) as isize) {
                k += *adjncy.offset(j as isize) % mask;
                j += 1;
                j;
            }
            (*keys.offset(ncand as isize)).val = i;
            (*keys.offset(ncand as isize))
                .key = ((k % mask) as libc::c_ulong)
                .wrapping_mul(maxdegree)
                .wrapping_add(idegree as libc::c_ulong) as idx_t;
            ncand = ncand.wrapping_add(1);
            ncand;
        }
        pi += 1;
        pi;
    }
    libmetis__ikvsorti(ncand, keys);
    mark = libmetis__iset(
        nvtxs as size_t,
        0 as libc::c_int,
        libmetis__iwspacemalloc(ctrl, nvtxs),
    );
    pi = 0 as libc::c_int;
    while (pi as libc::c_ulong) < ncand {
        i = (*keys.offset(pi as isize)).val;
        if !(*match_0.offset(i as isize) != -(1 as libc::c_int)) {
            j = *xadj.offset(i as isize);
            while j < *xadj.offset((i + 1 as libc::c_int) as isize) {
                *mark.offset(*adjncy.offset(j as isize) as isize) = i;
                j += 1;
                j;
            }
            pk = pi + 1 as libc::c_int;
            while (pk as libc::c_ulong) < ncand {
                k = (*keys.offset(pk as isize)).val;
                if !(*match_0.offset(k as isize) != -(1 as libc::c_int)) {
                    if (*keys.offset(pi as isize)).key != (*keys.offset(pk as isize)).key
                    {
                        break;
                    }
                    if *xadj.offset((i + 1 as libc::c_int) as isize)
                        - *xadj.offset(i as isize)
                        != *xadj.offset((k + 1 as libc::c_int) as isize)
                            - *xadj.offset(k as isize)
                    {
                        break;
                    }
                    jj = *xadj.offset(k as isize);
                    while jj < *xadj.offset((k + 1 as libc::c_int) as isize) {
                        if *mark.offset(*adjncy.offset(jj as isize) as isize) != i {
                            break;
                        }
                        jj += 1;
                        jj;
                    }
                    if jj == *xadj.offset((k + 1 as libc::c_int) as isize) {
                        let fresh16 = cnvtxs;
                        cnvtxs = cnvtxs + 1;
                        let ref mut fresh17 = *cmap.offset(k as isize);
                        *fresh17 = fresh16;
                        *cmap.offset(i as isize) = *fresh17;
                        *match_0.offset(i as isize) = k;
                        *match_0.offset(k as isize) = i;
                        nunmatched = (nunmatched as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong) as size_t
                            as size_t;
                        break;
                    }
                }
                pk += 1;
                pk;
            }
        }
        pi += 1;
        pi;
    }
    libmetis__wspacepop(ctrl);
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint
        != 0
    {
        (*ctrl).Aux3Tmr += gk_CPUSeconds();
    }
    *r_nunmatched = nunmatched;
    return cnvtxs;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__PrintCGraphStats(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
) {
    let mut i: idx_t = 0;
    printf(
        b"%10d %10d %10d [%d] [\0" as *const u8 as *const libc::c_char,
        (*graph).nvtxs,
        (*graph).nedges,
        libmetis__isum(
            (*graph).nedges as size_t,
            (*graph).adjwgt,
            1 as libc::c_int as size_t,
        ),
        (*ctrl).CoarsenTo,
    );
    i = 0 as libc::c_int;
    while i < (*graph).ncon {
        printf(
            b" %8d:%8d\0" as *const u8 as *const libc::c_char,
            *((*ctrl).maxvwgt).offset(i as isize),
            *((*graph).tvwgt).offset(i as isize),
        );
        i += 1;
        i;
    }
    printf(b" ]\n\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__CreateCoarseGraph(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut cnvtxs: idx_t,
    mut match_0: *mut idx_t,
) {
    let mut j: idx_t = 0;
    let mut jj: idx_t = 0;
    let mut k: idx_t = 0;
    let mut kk: idx_t = 0;
    let mut l: idx_t = 0;
    let mut m: idx_t = 0;
    let mut istart: idx_t = 0;
    let mut iend: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut nedges: idx_t = 0;
    let mut ncon: idx_t = 0;
    let mut cnedges: idx_t = 0;
    let mut v: idx_t = 0;
    let mut u: idx_t = 0;
    let mut mask: idx_t = 0;
    let mut dovsize: idx_t = 0;
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
    let mut vwgt: *mut idx_t = 0 as *mut idx_t;
    let mut vsize: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut adjwgt: *mut idx_t = 0 as *mut idx_t;
    let mut cmap: *mut idx_t = 0 as *mut idx_t;
    let mut htable: *mut idx_t = 0 as *mut idx_t;
    let mut cxadj: *mut idx_t = 0 as *mut idx_t;
    let mut cvwgt: *mut idx_t = 0 as *mut idx_t;
    let mut cvsize: *mut idx_t = 0 as *mut idx_t;
    let mut cadjncy: *mut idx_t = 0 as *mut idx_t;
    let mut cadjwgt: *mut idx_t = 0 as *mut idx_t;
    let mut cgraph: *mut graph_t = 0 as *mut graph_t;
    dovsize = if (*ctrl).objtype as libc::c_uint
        == METIS_OBJTYPE_VOL as libc::c_int as libc::c_uint
    {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    mask = ((1 as libc::c_int) << 11 as libc::c_int) - 1 as libc::c_int;
    if cnvtxs < 2 as libc::c_int * mask
        || (*graph).nedges / (*graph).nvtxs > mask / 20 as libc::c_int
    {
        libmetis__CreateCoarseGraphNoMask(ctrl, graph, cnvtxs, match_0);
        return;
    }
    nvtxs = (*graph).nvtxs;
    xadj = (*graph).xadj;
    v = 0 as libc::c_int;
    while v < nvtxs {
        if *xadj.offset((v + 1 as libc::c_int) as isize) - *xadj.offset(v as isize)
            > mask >> 3 as libc::c_int
        {
            libmetis__CreateCoarseGraphNoMask(ctrl, graph, cnvtxs, match_0);
            return;
        }
        v += 1;
        v;
    }
    libmetis__wspacepush(ctrl);
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint
        != 0
    {
        (*ctrl).ContractTmr -= gk_CPUSeconds();
    }
    ncon = (*graph).ncon;
    vwgt = (*graph).vwgt;
    vsize = (*graph).vsize;
    adjncy = (*graph).adjncy;
    adjwgt = (*graph).adjwgt;
    cmap = (*graph).cmap;
    cgraph = libmetis__SetupCoarseGraph(graph, cnvtxs, dovsize);
    cxadj = (*cgraph).xadj;
    cvwgt = (*cgraph).vwgt;
    cvsize = (*cgraph).vsize;
    cadjncy = (*cgraph).adjncy;
    cadjwgt = (*cgraph).adjwgt;
    htable = libmetis__iset(
        (if cnvtxs + 1 as libc::c_int >= mask + 1 as libc::c_int {
            mask + 1 as libc::c_int
        } else {
            cnvtxs + 1 as libc::c_int
        }) as size_t,
        -(1 as libc::c_int),
        libmetis__iwspacemalloc(ctrl, mask + 1 as libc::c_int),
    );
    cnedges = 0 as libc::c_int;
    cnvtxs = cnedges;
    *cxadj.offset(0 as libc::c_int as isize) = cnvtxs;
    v = 0 as libc::c_int;
    while v < nvtxs {
        u = *match_0.offset(v as isize);
        if !(u < v) {
            if ncon == 1 as libc::c_int {
                *cvwgt.offset(cnvtxs as isize) = *vwgt.offset(v as isize);
            } else {
                libmetis__icopy(
                    ncon as size_t,
                    vwgt.offset((v * ncon) as isize),
                    cvwgt.offset((cnvtxs * ncon) as isize),
                );
            }
            if dovsize != 0 {
                *cvsize.offset(cnvtxs as isize) = *vsize.offset(v as isize);
            }
            nedges = 0 as libc::c_int;
            istart = *xadj.offset(v as isize);
            iend = *xadj.offset((v + 1 as libc::c_int) as isize);
            j = istart;
            while j < iend {
                k = *cmap.offset(*adjncy.offset(j as isize) as isize);
                kk = k & mask;
                m = *htable.offset(kk as isize);
                if m == -(1 as libc::c_int) {
                    *cadjncy.offset(nedges as isize) = k;
                    *cadjwgt.offset(nedges as isize) = *adjwgt.offset(j as isize);
                    let fresh18 = nedges;
                    nedges = nedges + 1;
                    *htable.offset(kk as isize) = fresh18;
                } else if *cadjncy.offset(m as isize) == k {
                    let ref mut fresh19 = *cadjwgt.offset(m as isize);
                    *fresh19 += *adjwgt.offset(j as isize);
                } else {
                    jj = 0 as libc::c_int;
                    while jj < nedges {
                        if *cadjncy.offset(jj as isize) == k {
                            let ref mut fresh20 = *cadjwgt.offset(jj as isize);
                            *fresh20 += *adjwgt.offset(j as isize);
                            break;
                        } else {
                            jj += 1;
                            jj;
                        }
                    }
                    if jj == nedges {
                        *cadjncy.offset(nedges as isize) = k;
                        let fresh21 = nedges;
                        nedges = nedges + 1;
                        *cadjwgt.offset(fresh21 as isize) = *adjwgt.offset(j as isize);
                    }
                }
                j += 1;
                j;
            }
            if v != u {
                if ncon == 1 as libc::c_int {
                    let ref mut fresh22 = *cvwgt.offset(cnvtxs as isize);
                    *fresh22 += *vwgt.offset(u as isize);
                } else {
                    libmetis__iaxpy(
                        ncon as size_t,
                        1 as libc::c_int,
                        vwgt.offset((u * ncon) as isize),
                        1 as libc::c_int as size_t,
                        cvwgt.offset((cnvtxs * ncon) as isize),
                        1 as libc::c_int as size_t,
                    );
                }
                if dovsize != 0 {
                    let ref mut fresh23 = *cvsize.offset(cnvtxs as isize);
                    *fresh23 += *vsize.offset(u as isize);
                }
                istart = *xadj.offset(u as isize);
                iend = *xadj.offset((u + 1 as libc::c_int) as isize);
                j = istart;
                while j < iend {
                    k = *cmap.offset(*adjncy.offset(j as isize) as isize);
                    kk = k & mask;
                    m = *htable.offset(kk as isize);
                    if m == -(1 as libc::c_int) {
                        *cadjncy.offset(nedges as isize) = k;
                        *cadjwgt.offset(nedges as isize) = *adjwgt.offset(j as isize);
                        let fresh24 = nedges;
                        nedges = nedges + 1;
                        *htable.offset(kk as isize) = fresh24;
                    } else if *cadjncy.offset(m as isize) == k {
                        let ref mut fresh25 = *cadjwgt.offset(m as isize);
                        *fresh25 += *adjwgt.offset(j as isize);
                    } else {
                        jj = 0 as libc::c_int;
                        while jj < nedges {
                            if *cadjncy.offset(jj as isize) == k {
                                let ref mut fresh26 = *cadjwgt.offset(jj as isize);
                                *fresh26 += *adjwgt.offset(j as isize);
                                break;
                            } else {
                                jj += 1;
                                jj;
                            }
                        }
                        if jj == nedges {
                            *cadjncy.offset(nedges as isize) = k;
                            let fresh27 = nedges;
                            nedges = nedges + 1;
                            *cadjwgt
                                .offset(fresh27 as isize) = *adjwgt.offset(j as isize);
                        }
                    }
                    j += 1;
                    j;
                }
                jj = *htable.offset((cnvtxs & mask) as isize);
                if jj >= 0 as libc::c_int && *cadjncy.offset(jj as isize) != cnvtxs {
                    jj = 0 as libc::c_int;
                    while jj < nedges {
                        if *cadjncy.offset(jj as isize) == cnvtxs {
                            break;
                        }
                        jj += 1;
                        jj;
                    }
                }
                if jj >= 0 as libc::c_int && jj < nedges
                    && *cadjncy.offset(jj as isize) == cnvtxs
                {
                    nedges -= 1;
                    *cadjncy.offset(jj as isize) = *cadjncy.offset(nedges as isize);
                    *cadjwgt.offset(jj as isize) = *cadjwgt.offset(nedges as isize);
                }
            }
            j = 0 as libc::c_int;
            while j < nedges {
                *htable
                    .offset(
                        (*cadjncy.offset(j as isize) & mask) as isize,
                    ) = -(1 as libc::c_int);
                j += 1;
                j;
            }
            *htable.offset((cnvtxs & mask) as isize) = -(1 as libc::c_int);
            cnedges += nedges;
            cnvtxs += 1;
            *cxadj.offset(cnvtxs as isize) = cnedges;
            cadjncy = cadjncy.offset(nedges as isize);
            cadjwgt = cadjwgt.offset(nedges as isize);
        }
        v += 1;
        v;
    }
    (*cgraph).nedges = cnedges;
    j = 0 as libc::c_int;
    while j < ncon {
        *((*cgraph).tvwgt)
            .offset(
                j as isize,
            ) = libmetis__isum(
            (*cgraph).nvtxs as size_t,
            ((*cgraph).vwgt).offset(j as isize),
            ncon as size_t,
        );
        *((*cgraph).invtvwgt)
            .offset(
                j as isize,
            ) = (1.0f64
            / (if *((*cgraph).tvwgt).offset(j as isize) > 0 as libc::c_int {
                *((*cgraph).tvwgt).offset(j as isize)
            } else {
                1 as libc::c_int
            }) as libc::c_double) as real_t;
        j += 1;
        j;
    }
    libmetis__ReAdjustMemory(ctrl, graph, cgraph);
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint
        != 0
    {
        (*ctrl).ContractTmr += gk_CPUSeconds();
    }
    libmetis__wspacepop(ctrl);
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__CreateCoarseGraphNoMask(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut cnvtxs: idx_t,
    mut match_0: *mut idx_t,
) {
    let mut j: idx_t = 0;
    let mut k: idx_t = 0;
    let mut m: idx_t = 0;
    let mut istart: idx_t = 0;
    let mut iend: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut nedges: idx_t = 0;
    let mut ncon: idx_t = 0;
    let mut cnedges: idx_t = 0;
    let mut v: idx_t = 0;
    let mut u: idx_t = 0;
    let mut dovsize: idx_t = 0;
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
    let mut vwgt: *mut idx_t = 0 as *mut idx_t;
    let mut vsize: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut adjwgt: *mut idx_t = 0 as *mut idx_t;
    let mut cmap: *mut idx_t = 0 as *mut idx_t;
    let mut htable: *mut idx_t = 0 as *mut idx_t;
    let mut cxadj: *mut idx_t = 0 as *mut idx_t;
    let mut cvwgt: *mut idx_t = 0 as *mut idx_t;
    let mut cvsize: *mut idx_t = 0 as *mut idx_t;
    let mut cadjncy: *mut idx_t = 0 as *mut idx_t;
    let mut cadjwgt: *mut idx_t = 0 as *mut idx_t;
    let mut cgraph: *mut graph_t = 0 as *mut graph_t;
    libmetis__wspacepush(ctrl);
    dovsize = if (*ctrl).objtype as libc::c_uint
        == METIS_OBJTYPE_VOL as libc::c_int as libc::c_uint
    {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint
        != 0
    {
        (*ctrl).ContractTmr -= gk_CPUSeconds();
    }
    nvtxs = (*graph).nvtxs;
    ncon = (*graph).ncon;
    xadj = (*graph).xadj;
    vwgt = (*graph).vwgt;
    vsize = (*graph).vsize;
    adjncy = (*graph).adjncy;
    adjwgt = (*graph).adjwgt;
    cmap = (*graph).cmap;
    cgraph = libmetis__SetupCoarseGraph(graph, cnvtxs, dovsize);
    cxadj = (*cgraph).xadj;
    cvwgt = (*cgraph).vwgt;
    cvsize = (*cgraph).vsize;
    cadjncy = (*cgraph).adjncy;
    cadjwgt = (*cgraph).adjwgt;
    htable = libmetis__iset(
        cnvtxs as size_t,
        -(1 as libc::c_int),
        libmetis__iwspacemalloc(ctrl, cnvtxs),
    );
    cnedges = 0 as libc::c_int;
    cnvtxs = cnedges;
    *cxadj.offset(0 as libc::c_int as isize) = cnvtxs;
    v = 0 as libc::c_int;
    while v < nvtxs {
        u = *match_0.offset(v as isize);
        if !(u < v) {
            if ncon == 1 as libc::c_int {
                *cvwgt.offset(cnvtxs as isize) = *vwgt.offset(v as isize);
            } else {
                libmetis__icopy(
                    ncon as size_t,
                    vwgt.offset((v * ncon) as isize),
                    cvwgt.offset((cnvtxs * ncon) as isize),
                );
            }
            if dovsize != 0 {
                *cvsize.offset(cnvtxs as isize) = *vsize.offset(v as isize);
            }
            nedges = 0 as libc::c_int;
            istart = *xadj.offset(v as isize);
            iend = *xadj.offset((v + 1 as libc::c_int) as isize);
            j = istart;
            while j < iend {
                k = *cmap.offset(*adjncy.offset(j as isize) as isize);
                m = *htable.offset(k as isize);
                if m == -(1 as libc::c_int) {
                    *cadjncy.offset(nedges as isize) = k;
                    *cadjwgt.offset(nedges as isize) = *adjwgt.offset(j as isize);
                    let fresh28 = nedges;
                    nedges = nedges + 1;
                    *htable.offset(k as isize) = fresh28;
                } else {
                    let ref mut fresh29 = *cadjwgt.offset(m as isize);
                    *fresh29 += *adjwgt.offset(j as isize);
                }
                j += 1;
                j;
            }
            if v != u {
                if ncon == 1 as libc::c_int {
                    let ref mut fresh30 = *cvwgt.offset(cnvtxs as isize);
                    *fresh30 += *vwgt.offset(u as isize);
                } else {
                    libmetis__iaxpy(
                        ncon as size_t,
                        1 as libc::c_int,
                        vwgt.offset((u * ncon) as isize),
                        1 as libc::c_int as size_t,
                        cvwgt.offset((cnvtxs * ncon) as isize),
                        1 as libc::c_int as size_t,
                    );
                }
                if dovsize != 0 {
                    let ref mut fresh31 = *cvsize.offset(cnvtxs as isize);
                    *fresh31 += *vsize.offset(u as isize);
                }
                istart = *xadj.offset(u as isize);
                iend = *xadj.offset((u + 1 as libc::c_int) as isize);
                j = istart;
                while j < iend {
                    k = *cmap.offset(*adjncy.offset(j as isize) as isize);
                    m = *htable.offset(k as isize);
                    if m == -(1 as libc::c_int) {
                        *cadjncy.offset(nedges as isize) = k;
                        *cadjwgt.offset(nedges as isize) = *adjwgt.offset(j as isize);
                        let fresh32 = nedges;
                        nedges = nedges + 1;
                        *htable.offset(k as isize) = fresh32;
                    } else {
                        let ref mut fresh33 = *cadjwgt.offset(m as isize);
                        *fresh33 += *adjwgt.offset(j as isize);
                    }
                    j += 1;
                    j;
                }
                j = *htable.offset(cnvtxs as isize);
                if j != -(1 as libc::c_int) {
                    nedges -= 1;
                    *cadjncy.offset(j as isize) = *cadjncy.offset(nedges as isize);
                    *cadjwgt.offset(j as isize) = *cadjwgt.offset(nedges as isize);
                    *htable.offset(cnvtxs as isize) = -(1 as libc::c_int);
                }
            }
            j = 0 as libc::c_int;
            while j < nedges {
                *htable
                    .offset(*cadjncy.offset(j as isize) as isize) = -(1 as libc::c_int);
                j += 1;
                j;
            }
            cnedges += nedges;
            cnvtxs += 1;
            *cxadj.offset(cnvtxs as isize) = cnedges;
            cadjncy = cadjncy.offset(nedges as isize);
            cadjwgt = cadjwgt.offset(nedges as isize);
        }
        v += 1;
        v;
    }
    (*cgraph).nedges = cnedges;
    j = 0 as libc::c_int;
    while j < ncon {
        *((*cgraph).tvwgt)
            .offset(
                j as isize,
            ) = libmetis__isum(
            (*cgraph).nvtxs as size_t,
            ((*cgraph).vwgt).offset(j as isize),
            ncon as size_t,
        );
        *((*cgraph).invtvwgt)
            .offset(
                j as isize,
            ) = (1.0f64
            / (if *((*cgraph).tvwgt).offset(j as isize) > 0 as libc::c_int {
                *((*cgraph).tvwgt).offset(j as isize)
            } else {
                1 as libc::c_int
            }) as libc::c_double) as real_t;
        j += 1;
        j;
    }
    libmetis__ReAdjustMemory(ctrl, graph, cgraph);
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint
        != 0
    {
        (*ctrl).ContractTmr += gk_CPUSeconds();
    }
    libmetis__wspacepop(ctrl);
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__CreateCoarseGraphPerm(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut cnvtxs: idx_t,
    mut match_0: *mut idx_t,
    mut perm: *mut idx_t,
) {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut jj: idx_t = 0;
    let mut k: idx_t = 0;
    let mut kk: idx_t = 0;
    let mut l: idx_t = 0;
    let mut m: idx_t = 0;
    let mut istart: idx_t = 0;
    let mut iend: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut nedges: idx_t = 0;
    let mut ncon: idx_t = 0;
    let mut cnedges: idx_t = 0;
    let mut v: idx_t = 0;
    let mut u: idx_t = 0;
    let mut mask: idx_t = 0;
    let mut dovsize: idx_t = 0;
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
    let mut vwgt: *mut idx_t = 0 as *mut idx_t;
    let mut vsize: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut adjwgt: *mut idx_t = 0 as *mut idx_t;
    let mut cmap: *mut idx_t = 0 as *mut idx_t;
    let mut htable: *mut idx_t = 0 as *mut idx_t;
    let mut cxadj: *mut idx_t = 0 as *mut idx_t;
    let mut cvwgt: *mut idx_t = 0 as *mut idx_t;
    let mut cvsize: *mut idx_t = 0 as *mut idx_t;
    let mut cadjncy: *mut idx_t = 0 as *mut idx_t;
    let mut cadjwgt: *mut idx_t = 0 as *mut idx_t;
    let mut cgraph: *mut graph_t = 0 as *mut graph_t;
    libmetis__wspacepush(ctrl);
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint
        != 0
    {
        (*ctrl).ContractTmr -= gk_CPUSeconds();
    }
    dovsize = if (*ctrl).objtype as libc::c_uint
        == METIS_OBJTYPE_VOL as libc::c_int as libc::c_uint
    {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    mask = ((1 as libc::c_int) << 11 as libc::c_int) - 1 as libc::c_int;
    nvtxs = (*graph).nvtxs;
    ncon = (*graph).ncon;
    xadj = (*graph).xadj;
    vwgt = (*graph).vwgt;
    vsize = (*graph).vsize;
    adjncy = (*graph).adjncy;
    adjwgt = (*graph).adjwgt;
    cmap = (*graph).cmap;
    cgraph = libmetis__SetupCoarseGraph(graph, cnvtxs, dovsize);
    cxadj = (*cgraph).xadj;
    cvwgt = (*cgraph).vwgt;
    cvsize = (*cgraph).vsize;
    cadjncy = (*cgraph).adjncy;
    cadjwgt = (*cgraph).adjwgt;
    htable = libmetis__iset(
        (mask + 1 as libc::c_int) as size_t,
        -(1 as libc::c_int),
        libmetis__iwspacemalloc(ctrl, mask + 1 as libc::c_int),
    );
    cnedges = 0 as libc::c_int;
    cnvtxs = cnedges;
    *cxadj.offset(0 as libc::c_int as isize) = cnvtxs;
    i = 0 as libc::c_int;
    while i < nvtxs {
        v = *perm.offset(i as isize);
        if !(*cmap.offset(v as isize) != cnvtxs) {
            u = *match_0.offset(v as isize);
            if ncon == 1 as libc::c_int {
                *cvwgt.offset(cnvtxs as isize) = *vwgt.offset(v as isize);
            } else {
                libmetis__icopy(
                    ncon as size_t,
                    vwgt.offset((v * ncon) as isize),
                    cvwgt.offset((cnvtxs * ncon) as isize),
                );
            }
            if dovsize != 0 {
                *cvsize.offset(cnvtxs as isize) = *vsize.offset(v as isize);
            }
            nedges = 0 as libc::c_int;
            istart = *xadj.offset(v as isize);
            iend = *xadj.offset((v + 1 as libc::c_int) as isize);
            j = istart;
            while j < iend {
                k = *cmap.offset(*adjncy.offset(j as isize) as isize);
                kk = k & mask;
                m = *htable.offset(kk as isize);
                if m == -(1 as libc::c_int) {
                    *cadjncy.offset(nedges as isize) = k;
                    *cadjwgt.offset(nedges as isize) = *adjwgt.offset(j as isize);
                    let fresh34 = nedges;
                    nedges = nedges + 1;
                    *htable.offset(kk as isize) = fresh34;
                } else if *cadjncy.offset(m as isize) == k {
                    let ref mut fresh35 = *cadjwgt.offset(m as isize);
                    *fresh35 += *adjwgt.offset(j as isize);
                } else {
                    jj = 0 as libc::c_int;
                    while jj < nedges {
                        if *cadjncy.offset(jj as isize) == k {
                            let ref mut fresh36 = *cadjwgt.offset(jj as isize);
                            *fresh36 += *adjwgt.offset(j as isize);
                            break;
                        } else {
                            jj += 1;
                            jj;
                        }
                    }
                    if jj == nedges {
                        *cadjncy.offset(nedges as isize) = k;
                        let fresh37 = nedges;
                        nedges = nedges + 1;
                        *cadjwgt.offset(fresh37 as isize) = *adjwgt.offset(j as isize);
                    }
                }
                j += 1;
                j;
            }
            if v != u {
                if ncon == 1 as libc::c_int {
                    let ref mut fresh38 = *cvwgt.offset(cnvtxs as isize);
                    *fresh38 += *vwgt.offset(u as isize);
                } else {
                    libmetis__iaxpy(
                        ncon as size_t,
                        1 as libc::c_int,
                        vwgt.offset((u * ncon) as isize),
                        1 as libc::c_int as size_t,
                        cvwgt.offset((cnvtxs * ncon) as isize),
                        1 as libc::c_int as size_t,
                    );
                }
                if dovsize != 0 {
                    let ref mut fresh39 = *cvsize.offset(cnvtxs as isize);
                    *fresh39 += *vsize.offset(u as isize);
                }
                istart = *xadj.offset(u as isize);
                iend = *xadj.offset((u + 1 as libc::c_int) as isize);
                j = istart;
                while j < iend {
                    k = *cmap.offset(*adjncy.offset(j as isize) as isize);
                    kk = k & mask;
                    m = *htable.offset(kk as isize);
                    if m == -(1 as libc::c_int) {
                        *cadjncy.offset(nedges as isize) = k;
                        *cadjwgt.offset(nedges as isize) = *adjwgt.offset(j as isize);
                        let fresh40 = nedges;
                        nedges = nedges + 1;
                        *htable.offset(kk as isize) = fresh40;
                    } else if *cadjncy.offset(m as isize) == k {
                        let ref mut fresh41 = *cadjwgt.offset(m as isize);
                        *fresh41 += *adjwgt.offset(j as isize);
                    } else {
                        jj = 0 as libc::c_int;
                        while jj < nedges {
                            if *cadjncy.offset(jj as isize) == k {
                                let ref mut fresh42 = *cadjwgt.offset(jj as isize);
                                *fresh42 += *adjwgt.offset(j as isize);
                                break;
                            } else {
                                jj += 1;
                                jj;
                            }
                        }
                        if jj == nedges {
                            *cadjncy.offset(nedges as isize) = k;
                            let fresh43 = nedges;
                            nedges = nedges + 1;
                            *cadjwgt
                                .offset(fresh43 as isize) = *adjwgt.offset(j as isize);
                        }
                    }
                    j += 1;
                    j;
                }
                jj = *htable.offset((cnvtxs & mask) as isize);
                if jj >= 0 as libc::c_int && *cadjncy.offset(jj as isize) != cnvtxs {
                    jj = 0 as libc::c_int;
                    while jj < nedges {
                        if *cadjncy.offset(jj as isize) == cnvtxs {
                            break;
                        }
                        jj += 1;
                        jj;
                    }
                }
                if jj >= 0 as libc::c_int && *cadjncy.offset(jj as isize) == cnvtxs {
                    nedges -= 1;
                    *cadjncy.offset(jj as isize) = *cadjncy.offset(nedges as isize);
                    *cadjwgt.offset(jj as isize) = *cadjwgt.offset(nedges as isize);
                }
            }
            j = 0 as libc::c_int;
            while j < nedges {
                *htable
                    .offset(
                        (*cadjncy.offset(j as isize) & mask) as isize,
                    ) = -(1 as libc::c_int);
                j += 1;
                j;
            }
            *htable.offset((cnvtxs & mask) as isize) = -(1 as libc::c_int);
            cnedges += nedges;
            cnvtxs += 1;
            *cxadj.offset(cnvtxs as isize) = cnedges;
            cadjncy = cadjncy.offset(nedges as isize);
            cadjwgt = cadjwgt.offset(nedges as isize);
        }
        i += 1;
        i;
    }
    (*cgraph).nedges = cnedges;
    i = 0 as libc::c_int;
    while i < ncon {
        *((*cgraph).tvwgt)
            .offset(
                i as isize,
            ) = libmetis__isum(
            (*cgraph).nvtxs as size_t,
            ((*cgraph).vwgt).offset(i as isize),
            ncon as size_t,
        );
        *((*cgraph).invtvwgt)
            .offset(
                i as isize,
            ) = (1.0f64
            / (if *((*cgraph).tvwgt).offset(i as isize) > 0 as libc::c_int {
                *((*cgraph).tvwgt).offset(i as isize)
            } else {
                1 as libc::c_int
            }) as libc::c_double) as real_t;
        i += 1;
        i;
    }
    libmetis__ReAdjustMemory(ctrl, graph, cgraph);
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint
        != 0
    {
        (*ctrl).ContractTmr += gk_CPUSeconds();
    }
    libmetis__wspacepop(ctrl);
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__SetupCoarseGraph(
    mut graph: *mut graph_t,
    mut cnvtxs: idx_t,
    mut dovsize: idx_t,
) -> *mut graph_t {
    let mut cgraph: *mut graph_t = 0 as *mut graph_t;
    cgraph = libmetis__CreateGraph();
    (*cgraph).nvtxs = cnvtxs;
    (*cgraph).ncon = (*graph).ncon;
    (*cgraph).finer = graph;
    (*graph).coarser = cgraph;
    (*cgraph)
        .xadj = libmetis__imalloc(
        (cnvtxs + 1 as libc::c_int) as size_t,
        b"SetupCoarseGraph: xadj\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*cgraph)
        .adjncy = libmetis__imalloc(
        (*graph).nedges as size_t,
        b"SetupCoarseGraph: adjncy\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*cgraph)
        .adjwgt = libmetis__imalloc(
        (*graph).nedges as size_t,
        b"SetupCoarseGraph: adjwgt\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*cgraph)
        .vwgt = libmetis__imalloc(
        ((*cgraph).ncon * cnvtxs) as size_t,
        b"SetupCoarseGraph: vwgt\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*cgraph)
        .tvwgt = libmetis__imalloc(
        (*cgraph).ncon as size_t,
        b"SetupCoarseGraph: tvwgt\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*cgraph)
        .invtvwgt = libmetis__rmalloc(
        (*cgraph).ncon as size_t,
        b"SetupCoarseGraph: invtvwgt\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    if dovsize != 0 {
        (*cgraph)
            .vsize = libmetis__imalloc(
            cnvtxs as size_t,
            b"SetupCoarseGraph: vsize\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    return cgraph;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__ReAdjustMemory(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut cgraph: *mut graph_t,
) {
    if (*cgraph).nedges > 10000 as libc::c_int
        && ((*cgraph).nedges as libc::c_double)
            < 0.9f64 * (*graph).nedges as libc::c_double
    {
        (*cgraph)
            .adjncy = libmetis__irealloc(
            (*cgraph).adjncy,
            (*cgraph).nedges as size_t,
            b"ReAdjustMemory: adjncy\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        (*cgraph)
            .adjwgt = libmetis__irealloc(
            (*cgraph).adjwgt,
            (*cgraph).nedges as size_t,
            b"ReAdjustMemory: adjwgt\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
}
