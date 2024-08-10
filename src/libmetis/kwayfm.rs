use ::libc;
extern "C" {
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn gk_errexit(signum: libc::c_int, _: *mut libc::c_char, _: ...);
    fn libmetis__imax(n: size_t, x: *mut idx_t) -> idx_t;
    fn libmetis__imin(n: size_t, x: *mut idx_t) -> idx_t;
    fn libmetis__iargmin(n: size_t, x: *mut idx_t) -> size_t;
    fn libmetis__isum(n: size_t, x: *mut idx_t, incx: size_t) -> idx_t;
    fn libmetis__iaxpy(
        n: size_t,
        alpha: idx_t,
        x: *mut idx_t,
        incx: size_t,
        y: *mut idx_t,
        incy: size_t,
    ) -> *mut idx_t;
    fn libmetis__iset(n: size_t, val: idx_t, x: *mut idx_t) -> *mut idx_t;
    fn libmetis__rcopy(n: size_t, a: *mut real_t, b: *mut real_t) -> *mut real_t;
    fn libmetis__ipqCreate(maxnodes: size_t) -> *mut ipq_t;
    fn libmetis__ipqDestroy(queue: *mut ipq_t);
    fn libmetis__ipqInsert(queue: *mut ipq_t, node: idx_t, key: idx_t) -> libc::c_int;
    fn libmetis__ipqDelete(queue: *mut ipq_t, node: idx_t) -> libc::c_int;
    fn libmetis__ipqUpdate(queue: *mut ipq_t, node: idx_t, newkey: idx_t);
    fn libmetis__ipqGetTop(queue: *mut ipq_t) -> idx_t;
    fn libmetis__rpqCreate(maxnodes: size_t) -> *mut rpq_t;
    fn libmetis__rpqDestroy(queue: *mut rpq_t);
    fn libmetis__rpqInsert(queue: *mut rpq_t, node: idx_t, key: real_t) -> libc::c_int;
    fn libmetis__rpqDelete(queue: *mut rpq_t, node: idx_t) -> libc::c_int;
    fn libmetis__rpqUpdate(queue: *mut rpq_t, node: idx_t, newkey: real_t);
    fn libmetis__rpqGetTop(queue: *mut rpq_t) -> idx_t;
    fn libmetis__irandArrayPermute(
        n: idx_t,
        p: *mut idx_t,
        nshuffles: idx_t,
        flag: libc::c_int,
    );
    fn libmetis__ComputeVolume(_: *mut graph_t, _: *mut idx_t) -> idx_t;
    fn libmetis__wspacepop(ctrl: *mut ctrl_t);
    fn libmetis__ComputeLoadImbalance(
        graph: *mut graph_t,
        nparts: idx_t,
        pijbm: *mut real_t,
    ) -> real_t;
    fn libmetis__vnbrpoolGetNext(ctrl: *mut ctrl_t, nnbrs: idx_t) -> idx_t;
    fn libmetis__UpdateEdgeSubDomainGraph(
        ctrl: *mut ctrl_t,
        u: idx_t,
        v: idx_t,
        ewgt: idx_t,
        r_maxndoms: *mut idx_t,
    );
    fn libmetis__BetterBalanceKWay(
        ncon: idx_t,
        vwgt: *mut idx_t,
        itvwgt: *mut real_t,
        a1: idx_t,
        pt1: *mut idx_t,
        bm1: *mut real_t,
        a2: idx_t,
        pt2: *mut idx_t,
        bm2: *mut real_t,
    ) -> libc::c_int;
    fn libmetis__ivecaxpylez(
        n: idx_t,
        a: idx_t,
        x: *mut idx_t,
        y: *mut idx_t,
        z: *mut idx_t,
    ) -> libc::c_int;
    fn libmetis__ivecaxpygez(
        n: idx_t,
        a: idx_t,
        x: *mut idx_t,
        y: *mut idx_t,
        z: *mut idx_t,
    ) -> libc::c_int;
    fn libmetis__IsBalanced(
        ctrl: *mut ctrl_t,
        graph: *mut graph_t,
        ffactor: real_t,
    ) -> libc::c_int;
    fn libmetis__iwspacemalloc(_: *mut ctrl_t, _: idx_t) -> *mut idx_t;
    fn libmetis__ComputeSubDomainGraph(ctrl: *mut ctrl_t, graph: *mut graph_t);
    fn libmetis__rvecmaxdiff(n: idx_t, x: *mut real_t, y: *mut real_t) -> real_t;
    fn libmetis__ComputeLoadImbalanceVec(
        graph: *mut graph_t,
        nparts: idx_t,
        pijbm: *mut real_t,
        lbvec: *mut real_t,
    );
    fn libmetis__rwspacemalloc(_: *mut ctrl_t, _: idx_t) -> *mut real_t;
    fn libmetis__wspacepush(ctrl: *mut ctrl_t);
    fn libmetis__cnbrpoolGetNext(ctrl: *mut ctrl_t, nnbrs: idx_t) -> idx_t;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
pub type __int32_t = libc::c_int;
pub type __ssize_t = libc::c_long;
pub type int32_t = __int32_t;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
pub type gk_idx_t = ssize_t;
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
pub struct rkv_t {
    pub key: real_t,
    pub val: idx_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ipq_t {
    pub nnodes: gk_idx_t,
    pub maxnodes: gk_idx_t,
    pub heap: *mut ikv_t,
    pub locator: *mut gk_idx_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rpq_t {
    pub nnodes: gk_idx_t,
    pub maxnodes: gk_idx_t,
    pub heap: *mut rkv_t,
    pub locator: *mut gk_idx_t,
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
pub unsafe extern "C" fn libmetis__Greedy_KWayOptimize(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut niter: idx_t,
    mut ffactor: real_t,
    mut omode: idx_t,
) {
    match (*ctrl).objtype as libc::c_uint {
        0 => {
            if (*graph).ncon == 1 as libc::c_int {
                libmetis__Greedy_KWayCutOptimize(ctrl, graph, niter, ffactor, omode);
            } else {
                libmetis__Greedy_McKWayCutOptimize(ctrl, graph, niter, ffactor, omode);
            }
        }
        1 => {
            if (*graph).ncon == 1 as libc::c_int {
                libmetis__Greedy_KWayVolOptimize(ctrl, graph, niter, ffactor, omode);
            } else {
                libmetis__Greedy_McKWayVolOptimize(ctrl, graph, niter, ffactor, omode);
            }
        }
        _ => {
            gk_errexit(
                15 as libc::c_int,
                b"Unknown objtype of %d\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*ctrl).objtype as libc::c_uint,
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__Greedy_KWayCutOptimize(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut niter: idx_t,
    mut ffactor: real_t,
    mut omode: idx_t,
) {
    let mut i: idx_t = 0;
    let mut ii: idx_t = 0;
    let mut iii: idx_t = 0;
    let mut j: idx_t = 0;
    let mut k: idx_t = 0;
    let mut l: idx_t = 0;
    let mut pass: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut nparts: idx_t = 0;
    let mut gain: idx_t = 0;
    let mut from: idx_t = 0;
    let mut me: idx_t = 0;
    let mut to: idx_t = 0;
    let mut oldcut: idx_t = 0;
    let mut vwgt: idx_t = 0;
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut adjwgt: *mut idx_t = 0 as *mut idx_t;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    let mut pwgts: *mut idx_t = 0 as *mut idx_t;
    let mut perm: *mut idx_t = 0 as *mut idx_t;
    let mut bndptr: *mut idx_t = 0 as *mut idx_t;
    let mut bndind: *mut idx_t = 0 as *mut idx_t;
    let mut minwgt: *mut idx_t = 0 as *mut idx_t;
    let mut maxwgt: *mut idx_t = 0 as *mut idx_t;
    let mut itpwgts: *mut idx_t = 0 as *mut idx_t;
    let mut nmoved: idx_t = 0;
    let mut nupd: idx_t = 0;
    let mut vstatus: *mut idx_t = 0 as *mut idx_t;
    let mut updptr: *mut idx_t = 0 as *mut idx_t;
    let mut updind: *mut idx_t = 0 as *mut idx_t;
    let mut maxndoms: idx_t = 0;
    let mut safetos: *mut idx_t = 0 as *mut idx_t;
    let mut nads: *mut idx_t = 0 as *mut idx_t;
    let mut doms: *mut idx_t = 0 as *mut idx_t;
    let mut adids: *mut *mut idx_t = 0 as *mut *mut idx_t;
    let mut adwgts: *mut *mut idx_t = 0 as *mut *mut idx_t;
    let mut bfslvl: *mut idx_t = 0 as *mut idx_t;
    let mut bfsind: *mut idx_t = 0 as *mut idx_t;
    let mut bfsmrk: *mut idx_t = 0 as *mut idx_t;
    let mut bndtype: idx_t = if omode == 1 as libc::c_int {
        1 as libc::c_int
    } else {
        2 as libc::c_int
    };
    let mut nbnd: idx_t = 0;
    let mut oldnnbrs: idx_t = 0;
    let mut queue: *mut rpq_t = 0 as *mut rpq_t;
    let mut rgain: real_t = 0.;
    let mut myrinfo: *mut ckrinfo_t = 0 as *mut ckrinfo_t;
    let mut mynbrs: *mut cnbr_t = 0 as *mut cnbr_t;
    libmetis__wspacepush(ctrl);
    nvtxs = (*graph).nvtxs;
    xadj = (*graph).xadj;
    adjncy = (*graph).adjncy;
    adjwgt = (*graph).adjwgt;
    bndind = (*graph).bndind;
    bndptr = (*graph).bndptr;
    where_0 = (*graph).where_0;
    pwgts = (*graph).pwgts;
    nparts = (*ctrl).nparts;
    minwgt = libmetis__iwspacemalloc(ctrl, nparts);
    maxwgt = libmetis__iwspacemalloc(ctrl, nparts);
    itpwgts = libmetis__iwspacemalloc(ctrl, nparts);
    i = 0 as libc::c_int;
    while i < nparts {
        *itpwgts
            .offset(
                i as isize,
            ) = (*((*ctrl).tpwgts).offset(i as isize)
            * *((*graph).tvwgt).offset(0 as libc::c_int as isize) as libc::c_float)
            as idx_t;
        *maxwgt
            .offset(
                i as isize,
            ) = (*((*ctrl).tpwgts).offset(i as isize)
            * *((*graph).tvwgt).offset(0 as libc::c_int as isize) as libc::c_float
            * *((*ctrl).ubfactors).offset(0 as libc::c_int as isize)) as idx_t;
        *minwgt
            .offset(
                i as isize,
            ) = ((*((*ctrl).tpwgts).offset(i as isize)
            * *((*graph).tvwgt).offset(0 as libc::c_int as isize) as libc::c_float)
            as libc::c_double
            * (1.0f64
                / *((*ctrl).ubfactors).offset(0 as libc::c_int as isize)
                    as libc::c_double)) as idx_t;
        i += 1;
        i;
    }
    perm = libmetis__iwspacemalloc(ctrl, nvtxs);
    safetos = libmetis__iset(
        nparts as size_t,
        2 as libc::c_int,
        libmetis__iwspacemalloc(ctrl, nparts),
    );
    if (*ctrl).minconn != 0 {
        libmetis__ComputeSubDomainGraph(ctrl, graph);
        nads = (*ctrl).nads;
        adids = (*ctrl).adids;
        adwgts = (*ctrl).adwgts;
        doms = libmetis__iset(nparts as size_t, 0 as libc::c_int, (*ctrl).pvec1);
    }
    vstatus = libmetis__iset(
        nvtxs as size_t,
        3 as libc::c_int,
        libmetis__iwspacemalloc(ctrl, nvtxs),
    );
    updptr = libmetis__iset(
        nvtxs as size_t,
        -(1 as libc::c_int),
        libmetis__iwspacemalloc(ctrl, nvtxs),
    );
    updind = libmetis__iwspacemalloc(ctrl, nvtxs);
    if (*ctrl).contig != 0 {
        bfslvl = libmetis__iset(
            nvtxs as size_t,
            0 as libc::c_int,
            libmetis__iwspacemalloc(ctrl, nvtxs),
        );
        bfsind = libmetis__iwspacemalloc(ctrl, nvtxs);
        bfsmrk = libmetis__iset(
            nvtxs as size_t,
            0 as libc::c_int,
            libmetis__iwspacemalloc(ctrl, nvtxs),
        );
    }
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_REFINE as libc::c_int as libc::c_uint
        != 0
    {
        printf(
            b"%s: [%6d %6d]-[%6d %6d], Bal: %5.3f, Nv-Nb[%6d %6d], Cut: %6d\0"
                as *const u8 as *const libc::c_char,
            if omode == 1 as libc::c_int {
                b"GRC\0" as *const u8 as *const libc::c_char
            } else {
                b"GBC\0" as *const u8 as *const libc::c_char
            },
            *pwgts.offset(libmetis__iargmin(nparts as size_t, pwgts) as isize),
            libmetis__imax(nparts as size_t, pwgts),
            *minwgt.offset(0 as libc::c_int as isize),
            *maxwgt.offset(0 as libc::c_int as isize),
            libmetis__ComputeLoadImbalance(graph, nparts, (*ctrl).pijbm)
                as libc::c_double,
            (*graph).nvtxs,
            (*graph).nbnd,
            (*graph).mincut,
        );
        if (*ctrl).minconn != 0 {
            printf(
                b", Doms: [%3d %4d]\0" as *const u8 as *const libc::c_char,
                libmetis__imax(nparts as size_t, nads),
                libmetis__isum(nparts as size_t, nads, 1 as libc::c_int as size_t),
            );
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
    }
    queue = libmetis__rpqCreate(nvtxs as size_t);
    pass = 0 as libc::c_int;
    while pass < niter {
        if omode == 2 as libc::c_int {
            i = 0 as libc::c_int;
            while i < nparts {
                if *pwgts.offset(i as isize) > *maxwgt.offset(i as isize) {
                    break;
                }
                i += 1;
                i;
            }
            if i == nparts {
                break;
            }
        }
        oldcut = (*graph).mincut;
        nbnd = (*graph).nbnd;
        nupd = 0 as libc::c_int;
        if (*ctrl).minconn != 0 {
            maxndoms = libmetis__imax(nparts as size_t, nads);
        }
        libmetis__irandArrayPermute(
            nbnd,
            perm,
            nbnd / 4 as libc::c_int,
            1 as libc::c_int,
        );
        ii = 0 as libc::c_int;
        while ii < nbnd {
            i = *bndind.offset(*perm.offset(ii as isize) as isize);
            rgain = ((if (*((*graph).ckrinfo).offset(i as isize)).nnbrs
                > 0 as libc::c_int
            {
                1.0f64 * (*((*graph).ckrinfo).offset(i as isize)).ed as libc::c_double
                    / sqrt(
                        (*((*graph).ckrinfo).offset(i as isize)).nnbrs as libc::c_double,
                    )
            } else {
                0.0f64
            }) - (*((*graph).ckrinfo).offset(i as isize)).id as libc::c_double)
                as real_t;
            libmetis__rpqInsert(queue, i, rgain);
            *vstatus.offset(i as isize) = 1 as libc::c_int;
            *updind.offset(nupd as isize) = i;
            let fresh0 = nupd;
            nupd = nupd + 1;
            *updptr.offset(i as isize) = fresh0;
            ii += 1;
            ii;
        }
        let mut current_block_317: u64;
        nmoved = 0 as libc::c_int;
        iii = 0 as libc::c_int;
        loop {
            i = libmetis__rpqGetTop(queue);
            if i == -(1 as libc::c_int) {
                break;
            }
            *vstatus.offset(i as isize) = 2 as libc::c_int;
            myrinfo = ((*graph).ckrinfo).offset(i as isize);
            mynbrs = ((*ctrl).cnbrpool).offset((*myrinfo).inbr as isize);
            from = *where_0.offset(i as isize);
            vwgt = *((*graph).vwgt).offset(i as isize);
            if omode == 1 as libc::c_int {
                if (*myrinfo).id > 0 as libc::c_int
                    && *pwgts.offset(from as isize) - vwgt
                        < *minwgt.offset(from as isize)
                {
                    current_block_317 = 5159818223158340697;
                } else {
                    current_block_317 = 15669289850109000831;
                }
            } else if *pwgts.offset(from as isize) - vwgt < *minwgt.offset(from as isize)
            {
                current_block_317 = 5159818223158340697;
            } else {
                current_block_317 = 15669289850109000831;
            }
            match current_block_317 {
                15669289850109000831 => {
                    if !((*ctrl).contig != 0
                        && libmetis__IsArticulationNode(
                            i,
                            xadj,
                            adjncy,
                            where_0,
                            bfslvl,
                            bfsind,
                            bfsmrk,
                        ) != 0)
                    {
                        if (*ctrl).minconn != 0 {
                            let mut j_0: idx_t = 0;
                            let mut k_0: idx_t = 0;
                            let mut l_0: idx_t = 0;
                            let mut nadd: idx_t = 0;
                            let mut to_0: idx_t = 0;
                            j_0 = 0 as libc::c_int;
                            while j_0 < (*myrinfo).nnbrs {
                                to_0 = (*mynbrs.offset(j_0 as isize)).pid;
                                *safetos.offset(to_0 as isize) = 0 as libc::c_int;
                                k_0 = 0 as libc::c_int;
                                while k_0 < *nads.offset(to_0 as isize) {
                                    *doms
                                        .offset(
                                            *(*adids.offset(to_0 as isize)).offset(k_0 as isize)
                                                as isize,
                                        ) = 1 as libc::c_int;
                                    k_0 += 1;
                                    k_0;
                                }
                                nadd = 0 as libc::c_int;
                                k_0 = 0 as libc::c_int;
                                while k_0 < (*myrinfo).nnbrs {
                                    if !(k_0 == j_0) {
                                        l_0 = (*mynbrs.offset(k_0 as isize)).pid;
                                        if *doms.offset(l_0 as isize) == 0 as libc::c_int {
                                            if *nads.offset(l_0 as isize) > maxndoms - 1 as libc::c_int
                                            {
                                                nadd = maxndoms;
                                                break;
                                            } else {
                                                nadd += 1;
                                                nadd;
                                            }
                                        }
                                    }
                                    k_0 += 1;
                                    k_0;
                                }
                                if *nads.offset(to_0 as isize) + nadd <= maxndoms {
                                    *safetos.offset(to_0 as isize) = 1 as libc::c_int;
                                }
                                if nadd == 0 as libc::c_int {
                                    *safetos.offset(to_0 as isize) = 2 as libc::c_int;
                                }
                                k_0 = 0 as libc::c_int;
                                while k_0 < *nads.offset(to_0 as isize) {
                                    *doms
                                        .offset(
                                            *(*adids.offset(to_0 as isize)).offset(k_0 as isize)
                                                as isize,
                                        ) = 0 as libc::c_int;
                                    k_0 += 1;
                                    k_0;
                                }
                                j_0 += 1;
                                j_0;
                            }
                        }
                        if omode == 1 as libc::c_int {
                            k = (*myrinfo).nnbrs - 1 as libc::c_int;
                            while k >= 0 as libc::c_int {
                                to = (*mynbrs.offset(k as isize)).pid;
                                if !(*safetos.offset(to as isize) == 0) {
                                    gain = (*mynbrs.offset(k as isize)).ed - (*myrinfo).id;
                                    if gain >= 0 as libc::c_int
                                        && (*pwgts.offset(to as isize) + vwgt) as libc::c_float
                                            <= *maxwgt.offset(to as isize) as libc::c_float
                                                + ffactor * gain as libc::c_float
                                    {
                                        break;
                                    }
                                }
                                k -= 1;
                                k;
                            }
                            if k < 0 as libc::c_int {
                                current_block_317 = 5159818223158340697;
                            } else {
                                j = k - 1 as libc::c_int;
                                while j >= 0 as libc::c_int {
                                    to = (*mynbrs.offset(j as isize)).pid;
                                    if !(*safetos.offset(to as isize) == 0) {
                                        gain = (*mynbrs.offset(j as isize)).ed - (*myrinfo).id;
                                        if (*mynbrs.offset(j as isize)).ed
                                            > (*mynbrs.offset(k as isize)).ed
                                            && (*pwgts.offset(to as isize) + vwgt) as libc::c_float
                                                <= *maxwgt.offset(to as isize) as libc::c_float
                                                    + ffactor * gain as libc::c_float
                                            || (*mynbrs.offset(j as isize)).ed
                                                == (*mynbrs.offset(k as isize)).ed
                                                && *itpwgts
                                                    .offset((*mynbrs.offset(k as isize)).pid as isize)
                                                    * *pwgts.offset(to as isize)
                                                    < *itpwgts.offset(to as isize)
                                                        * *pwgts.offset((*mynbrs.offset(k as isize)).pid as isize)
                                        {
                                            k = j;
                                        }
                                    }
                                    j -= 1;
                                    j;
                                }
                                to = (*mynbrs.offset(k as isize)).pid;
                                gain = (*mynbrs.offset(k as isize)).ed - (*myrinfo).id;
                                if !(gain > 0 as libc::c_int
                                    || gain == 0 as libc::c_int
                                        && (*pwgts.offset(from as isize)
                                            >= *maxwgt.offset(from as isize)
                                            || *itpwgts.offset(to as isize)
                                                * *pwgts.offset(from as isize)
                                                > *itpwgts.offset(from as isize)
                                                    * (*pwgts.offset(to as isize) + vwgt)
                                            || iii % 2 as libc::c_int == 0 as libc::c_int
                                                && *safetos.offset(to as isize) == 2 as libc::c_int))
                                {
                                    current_block_317 = 5159818223158340697;
                                } else {
                                    current_block_317 = 16314074004867283505;
                                }
                            }
                        } else {
                            k = (*myrinfo).nnbrs - 1 as libc::c_int;
                            while k >= 0 as libc::c_int {
                                to = (*mynbrs.offset(k as isize)).pid;
                                if !(*safetos.offset(to as isize) == 0) {
                                    if *pwgts.offset(to as isize) + vwgt
                                        <= *maxwgt.offset(to as isize)
                                        || *itpwgts.offset(from as isize)
                                            * (*pwgts.offset(to as isize) + vwgt)
                                            <= *itpwgts.offset(to as isize)
                                                * *pwgts.offset(from as isize)
                                    {
                                        break;
                                    }
                                }
                                k -= 1;
                                k;
                            }
                            if k < 0 as libc::c_int {
                                current_block_317 = 5159818223158340697;
                            } else {
                                j = k - 1 as libc::c_int;
                                while j >= 0 as libc::c_int {
                                    to = (*mynbrs.offset(j as isize)).pid;
                                    if !(*safetos.offset(to as isize) == 0) {
                                        if *itpwgts
                                            .offset((*mynbrs.offset(k as isize)).pid as isize)
                                            * *pwgts.offset(to as isize)
                                            < *itpwgts.offset(to as isize)
                                                * *pwgts.offset((*mynbrs.offset(k as isize)).pid as isize)
                                        {
                                            k = j;
                                        }
                                    }
                                    j -= 1;
                                    j;
                                }
                                to = (*mynbrs.offset(k as isize)).pid;
                                if *pwgts.offset(from as isize)
                                    < *maxwgt.offset(from as isize)
                                    && *pwgts.offset(to as isize) > *minwgt.offset(to as isize)
                                    && (*mynbrs.offset(k as isize)).ed - (*myrinfo).id
                                        < 0 as libc::c_int
                                {
                                    current_block_317 = 5159818223158340697;
                                } else {
                                    current_block_317 = 16314074004867283505;
                                }
                            }
                        }
                        match current_block_317 {
                            5159818223158340697 => {}
                            _ => {
                                (*graph).mincut
                                    -= (*mynbrs.offset(k as isize)).ed - (*myrinfo).id;
                                nmoved += 1;
                                nmoved;
                                if (*ctrl).dbglvl as libc::c_uint
                                    & METIS_DBG_MOVEINFO as libc::c_int as libc::c_uint != 0
                                {
                                    printf(
                                        b"\t\tMoving %6d to %3d. Gain: %4d. Cut: %6d\n\0"
                                            as *const u8 as *const libc::c_char,
                                        i,
                                        to,
                                        (*mynbrs.offset(k as isize)).ed - (*myrinfo).id,
                                        (*graph).mincut,
                                    );
                                }
                                if (*ctrl).minconn != 0 {
                                    libmetis__UpdateEdgeSubDomainGraph(
                                        ctrl,
                                        from,
                                        to,
                                        (*myrinfo).id - (*mynbrs.offset(k as isize)).ed,
                                        &mut maxndoms,
                                    );
                                    j = *xadj.offset(i as isize);
                                    while j < *xadj.offset((i + 1 as libc::c_int) as isize) {
                                        me = *where_0.offset(*adjncy.offset(j as isize) as isize);
                                        if me != from && me != to {
                                            libmetis__UpdateEdgeSubDomainGraph(
                                                ctrl,
                                                from,
                                                me,
                                                -*adjwgt.offset(j as isize),
                                                &mut maxndoms,
                                            );
                                            libmetis__UpdateEdgeSubDomainGraph(
                                                ctrl,
                                                to,
                                                me,
                                                *adjwgt.offset(j as isize),
                                                &mut maxndoms,
                                            );
                                        }
                                        j += 1;
                                        j;
                                    }
                                }
                                let ref mut fresh1 = *pwgts.offset(to as isize);
                                *fresh1 += vwgt;
                                let ref mut fresh2 = *pwgts.offset(from as isize);
                                *fresh2 -= vwgt;
                                *where_0.offset(i as isize) = to;
                                (*myrinfo).ed
                                    += (*myrinfo).id - (*mynbrs.offset(k as isize)).ed;
                                j = (*myrinfo).id;
                                (*myrinfo).id = (*mynbrs.offset(k as isize)).ed;
                                (*mynbrs.offset(k as isize)).ed = j;
                                if (*mynbrs.offset(k as isize)).ed == 0 as libc::c_int {
                                    (*myrinfo).nnbrs -= 1;
                                    *mynbrs
                                        .offset(
                                            k as isize,
                                        ) = *mynbrs.offset((*myrinfo).nnbrs as isize);
                                } else {
                                    (*mynbrs.offset(k as isize)).pid = from;
                                }
                                if bndtype == 1 as libc::c_int {
                                    if *bndptr.offset(i as isize) != -(1 as libc::c_int)
                                        && (*myrinfo).ed - (*myrinfo).id < 0 as libc::c_int
                                    {
                                        nbnd -= 1;
                                        *bndind
                                            .offset(
                                                *bndptr.offset(i as isize) as isize,
                                            ) = *bndind.offset(nbnd as isize);
                                        *bndptr
                                            .offset(
                                                *bndind.offset(nbnd as isize) as isize,
                                            ) = *bndptr.offset(i as isize);
                                        *bndptr.offset(i as isize) = -(1 as libc::c_int);
                                    }
                                    if *bndptr.offset(i as isize) == -(1 as libc::c_int)
                                        && (*myrinfo).ed - (*myrinfo).id >= 0 as libc::c_int
                                    {
                                        *bndind.offset(nbnd as isize) = i;
                                        let fresh3 = nbnd;
                                        nbnd = nbnd + 1;
                                        *bndptr.offset(i as isize) = fresh3;
                                    }
                                } else {
                                    if *bndptr.offset(i as isize) != -(1 as libc::c_int)
                                        && (*myrinfo).ed <= 0 as libc::c_int
                                    {
                                        nbnd -= 1;
                                        *bndind
                                            .offset(
                                                *bndptr.offset(i as isize) as isize,
                                            ) = *bndind.offset(nbnd as isize);
                                        *bndptr
                                            .offset(
                                                *bndind.offset(nbnd as isize) as isize,
                                            ) = *bndptr.offset(i as isize);
                                        *bndptr.offset(i as isize) = -(1 as libc::c_int);
                                    }
                                    if *bndptr.offset(i as isize) == -(1 as libc::c_int)
                                        && (*myrinfo).ed > 0 as libc::c_int
                                    {
                                        *bndind.offset(nbnd as isize) = i;
                                        let fresh4 = nbnd;
                                        nbnd = nbnd + 1;
                                        *bndptr.offset(i as isize) = fresh4;
                                    }
                                }
                                j = *xadj.offset(i as isize);
                                while j < *xadj.offset((i + 1 as libc::c_int) as isize) {
                                    ii = *adjncy.offset(j as isize);
                                    me = *where_0.offset(ii as isize);
                                    myrinfo = ((*graph).ckrinfo).offset(ii as isize);
                                    oldnnbrs = (*myrinfo).nnbrs;
                                    let mut k_1: idx_t = 0;
                                    let mut mynbrs_0: *mut cnbr_t = 0 as *mut cnbr_t;
                                    if (*myrinfo).inbr == -(1 as libc::c_int) {
                                        (*myrinfo)
                                            .inbr = libmetis__cnbrpoolGetNext(
                                            ctrl,
                                            *xadj.offset((ii + 1 as libc::c_int) as isize)
                                                - *xadj.offset(ii as isize) + 1 as libc::c_int,
                                        );
                                        (*myrinfo).nnbrs = 0 as libc::c_int;
                                    }
                                    mynbrs_0 = ((*ctrl).cnbrpool)
                                        .offset((*myrinfo).inbr as isize);
                                    if me == from {
                                        (*myrinfo).ed += *adjwgt.offset(j as isize);
                                        (*myrinfo).id -= *adjwgt.offset(j as isize);
                                        if bndtype == 1 as libc::c_int {
                                            if (*myrinfo).ed - (*myrinfo).id >= 0 as libc::c_int
                                                && *bndptr.offset(ii as isize) == -(1 as libc::c_int)
                                            {
                                                *bndind.offset(nbnd as isize) = ii;
                                                let fresh5 = nbnd;
                                                nbnd = nbnd + 1;
                                                *bndptr.offset(ii as isize) = fresh5;
                                            }
                                        } else if (*myrinfo).ed > 0 as libc::c_int
                                            && *bndptr.offset(ii as isize) == -(1 as libc::c_int)
                                        {
                                            *bndind.offset(nbnd as isize) = ii;
                                            let fresh6 = nbnd;
                                            nbnd = nbnd + 1;
                                            *bndptr.offset(ii as isize) = fresh6;
                                        }
                                    } else if me == to {
                                        (*myrinfo).id += *adjwgt.offset(j as isize);
                                        (*myrinfo).ed -= *adjwgt.offset(j as isize);
                                        if bndtype == 1 as libc::c_int {
                                            if (*myrinfo).ed - (*myrinfo).id < 0 as libc::c_int
                                                && *bndptr.offset(ii as isize) != -(1 as libc::c_int)
                                            {
                                                nbnd -= 1;
                                                *bndind
                                                    .offset(
                                                        *bndptr.offset(ii as isize) as isize,
                                                    ) = *bndind.offset(nbnd as isize);
                                                *bndptr
                                                    .offset(
                                                        *bndind.offset(nbnd as isize) as isize,
                                                    ) = *bndptr.offset(ii as isize);
                                                *bndptr.offset(ii as isize) = -(1 as libc::c_int);
                                            }
                                        } else if (*myrinfo).ed <= 0 as libc::c_int
                                            && *bndptr.offset(ii as isize) != -(1 as libc::c_int)
                                        {
                                            nbnd -= 1;
                                            *bndind
                                                .offset(
                                                    *bndptr.offset(ii as isize) as isize,
                                                ) = *bndind.offset(nbnd as isize);
                                            *bndptr
                                                .offset(
                                                    *bndind.offset(nbnd as isize) as isize,
                                                ) = *bndptr.offset(ii as isize);
                                            *bndptr.offset(ii as isize) = -(1 as libc::c_int);
                                        }
                                    }
                                    if me != from {
                                        k_1 = 0 as libc::c_int;
                                        while k_1 < (*myrinfo).nnbrs {
                                            if (*mynbrs_0.offset(k_1 as isize)).pid == from {
                                                if (*mynbrs_0.offset(k_1 as isize)).ed
                                                    == *adjwgt.offset(j as isize)
                                                {
                                                    (*myrinfo).nnbrs -= 1;
                                                    *mynbrs_0
                                                        .offset(
                                                            k_1 as isize,
                                                        ) = *mynbrs_0.offset((*myrinfo).nnbrs as isize);
                                                } else {
                                                    let ref mut fresh7 = (*mynbrs_0.offset(k_1 as isize)).ed;
                                                    *fresh7 -= *adjwgt.offset(j as isize);
                                                }
                                                break;
                                            } else {
                                                k_1 += 1;
                                                k_1;
                                            }
                                        }
                                    }
                                    if me != to {
                                        k_1 = 0 as libc::c_int;
                                        while k_1 < (*myrinfo).nnbrs {
                                            if (*mynbrs_0.offset(k_1 as isize)).pid == to {
                                                let ref mut fresh8 = (*mynbrs_0.offset(k_1 as isize)).ed;
                                                *fresh8 += *adjwgt.offset(j as isize);
                                                break;
                                            } else {
                                                k_1 += 1;
                                                k_1;
                                            }
                                        }
                                        if k_1 == (*myrinfo).nnbrs {
                                            (*mynbrs_0.offset(k_1 as isize)).pid = to;
                                            (*mynbrs_0.offset(k_1 as isize))
                                                .ed = *adjwgt.offset(j as isize);
                                            (*myrinfo).nnbrs += 1;
                                            (*myrinfo).nnbrs;
                                        }
                                    }
                                    let mut rgain_0: real_t = 0.;
                                    if me == to || me == from || oldnnbrs != (*myrinfo).nnbrs {
                                        rgain_0 = ((if (*myrinfo).nnbrs > 0 as libc::c_int {
                                            1.0f64 * (*myrinfo).ed as libc::c_double
                                                / sqrt((*myrinfo).nnbrs as libc::c_double)
                                        } else {
                                            0.0f64
                                        }) - (*myrinfo).id as libc::c_double) as real_t;
                                        if bndtype == 1 as libc::c_int {
                                            if *vstatus.offset(ii as isize) == 1 as libc::c_int {
                                                if (*myrinfo).ed - (*myrinfo).id >= 0 as libc::c_int {
                                                    libmetis__rpqUpdate(queue, ii, rgain_0);
                                                } else {
                                                    libmetis__rpqDelete(queue, ii);
                                                    *vstatus.offset(ii as isize) = 3 as libc::c_int;
                                                    nupd -= 1;
                                                    *updind
                                                        .offset(
                                                            *updptr.offset(ii as isize) as isize,
                                                        ) = *updind.offset(nupd as isize);
                                                    *updptr
                                                        .offset(
                                                            *updind.offset(nupd as isize) as isize,
                                                        ) = *updptr.offset(ii as isize);
                                                    *updptr.offset(ii as isize) = -(1 as libc::c_int);
                                                }
                                            } else if *vstatus.offset(ii as isize) == 3 as libc::c_int
                                                && (*myrinfo).ed - (*myrinfo).id >= 0 as libc::c_int
                                            {
                                                libmetis__rpqInsert(queue, ii, rgain_0);
                                                *vstatus.offset(ii as isize) = 1 as libc::c_int;
                                                *updind.offset(nupd as isize) = ii;
                                                let fresh9 = nupd;
                                                nupd = nupd + 1;
                                                *updptr.offset(ii as isize) = fresh9;
                                            }
                                        } else if *vstatus.offset(ii as isize) == 1 as libc::c_int {
                                            if (*myrinfo).ed > 0 as libc::c_int {
                                                libmetis__rpqUpdate(queue, ii, rgain_0);
                                            } else {
                                                libmetis__rpqDelete(queue, ii);
                                                *vstatus.offset(ii as isize) = 3 as libc::c_int;
                                                nupd -= 1;
                                                *updind
                                                    .offset(
                                                        *updptr.offset(ii as isize) as isize,
                                                    ) = *updind.offset(nupd as isize);
                                                *updptr
                                                    .offset(
                                                        *updind.offset(nupd as isize) as isize,
                                                    ) = *updptr.offset(ii as isize);
                                                *updptr.offset(ii as isize) = -(1 as libc::c_int);
                                            }
                                        } else if *vstatus.offset(ii as isize) == 3 as libc::c_int
                                            && (*myrinfo).ed > 0 as libc::c_int
                                        {
                                            libmetis__rpqInsert(queue, ii, rgain_0);
                                            *vstatus.offset(ii as isize) = 1 as libc::c_int;
                                            *updind.offset(nupd as isize) = ii;
                                            let fresh10 = nupd;
                                            nupd = nupd + 1;
                                            *updptr.offset(ii as isize) = fresh10;
                                        }
                                    }
                                    j += 1;
                                    j;
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
            iii += 1;
            iii;
        }
        (*graph).nbnd = nbnd;
        i = 0 as libc::c_int;
        while i < nupd {
            *vstatus.offset(*updind.offset(i as isize) as isize) = 3 as libc::c_int;
            *updptr.offset(*updind.offset(i as isize) as isize) = -(1 as libc::c_int);
            i += 1;
            i;
        }
        if (*ctrl).dbglvl as libc::c_uint
            & METIS_DBG_REFINE as libc::c_int as libc::c_uint != 0
        {
            printf(
                b"\t[%6d %6d], Bal: %5.3f, Nb: %6d. Nmoves: %5d, Cut: %6d, Vol: %6d\0"
                    as *const u8 as *const libc::c_char,
                *pwgts.offset(libmetis__iargmin(nparts as size_t, pwgts) as isize),
                libmetis__imax(nparts as size_t, pwgts),
                libmetis__ComputeLoadImbalance(graph, nparts, (*ctrl).pijbm)
                    as libc::c_double,
                (*graph).nbnd,
                nmoved,
                (*graph).mincut,
                libmetis__ComputeVolume(graph, where_0),
            );
            if (*ctrl).minconn != 0 {
                printf(
                    b", Doms: [%3d %4d]\0" as *const u8 as *const libc::c_char,
                    libmetis__imax(nparts as size_t, nads),
                    libmetis__isum(nparts as size_t, nads, 1 as libc::c_int as size_t),
                );
            }
            printf(b"\n\0" as *const u8 as *const libc::c_char);
        }
        if nmoved == 0 as libc::c_int
            || omode == 1 as libc::c_int && (*graph).mincut == oldcut
        {
            break;
        }
        pass += 1;
        pass;
    }
    libmetis__rpqDestroy(queue);
    libmetis__wspacepop(ctrl);
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__Greedy_KWayVolOptimize(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut niter: idx_t,
    mut ffactor: real_t,
    mut omode: idx_t,
) {
    let mut i: idx_t = 0;
    let mut ii: idx_t = 0;
    let mut iii: idx_t = 0;
    let mut j: idx_t = 0;
    let mut k: idx_t = 0;
    let mut l: idx_t = 0;
    let mut pass: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut nparts: idx_t = 0;
    let mut gain: idx_t = 0;
    let mut from: idx_t = 0;
    let mut me: idx_t = 0;
    let mut to: idx_t = 0;
    let mut oldcut: idx_t = 0;
    let mut vwgt: idx_t = 0;
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    let mut pwgts: *mut idx_t = 0 as *mut idx_t;
    let mut perm: *mut idx_t = 0 as *mut idx_t;
    let mut bndptr: *mut idx_t = 0 as *mut idx_t;
    let mut bndind: *mut idx_t = 0 as *mut idx_t;
    let mut minwgt: *mut idx_t = 0 as *mut idx_t;
    let mut maxwgt: *mut idx_t = 0 as *mut idx_t;
    let mut itpwgts: *mut idx_t = 0 as *mut idx_t;
    let mut nmoved: idx_t = 0;
    let mut nupd: idx_t = 0;
    let mut vstatus: *mut idx_t = 0 as *mut idx_t;
    let mut updptr: *mut idx_t = 0 as *mut idx_t;
    let mut updind: *mut idx_t = 0 as *mut idx_t;
    let mut maxndoms: idx_t = 0;
    let mut safetos: *mut idx_t = 0 as *mut idx_t;
    let mut nads: *mut idx_t = 0 as *mut idx_t;
    let mut doms: *mut idx_t = 0 as *mut idx_t;
    let mut adids: *mut *mut idx_t = 0 as *mut *mut idx_t;
    let mut adwgts: *mut *mut idx_t = 0 as *mut *mut idx_t;
    let mut bfslvl: *mut idx_t = 0 as *mut idx_t;
    let mut bfsind: *mut idx_t = 0 as *mut idx_t;
    let mut bfsmrk: *mut idx_t = 0 as *mut idx_t;
    let mut bndtype: idx_t = if omode == 1 as libc::c_int {
        1 as libc::c_int
    } else {
        2 as libc::c_int
    };
    let mut queue: *mut ipq_t = 0 as *mut ipq_t;
    let mut oldvol: idx_t = 0;
    let mut xgain: idx_t = 0;
    let mut vmarker: *mut idx_t = 0 as *mut idx_t;
    let mut pmarker: *mut idx_t = 0 as *mut idx_t;
    let mut modind: *mut idx_t = 0 as *mut idx_t;
    let mut myrinfo: *mut vkrinfo_t = 0 as *mut vkrinfo_t;
    let mut mynbrs: *mut vnbr_t = 0 as *mut vnbr_t;
    libmetis__wspacepush(ctrl);
    nvtxs = (*graph).nvtxs;
    xadj = (*graph).xadj;
    adjncy = (*graph).adjncy;
    bndptr = (*graph).bndptr;
    bndind = (*graph).bndind;
    where_0 = (*graph).where_0;
    pwgts = (*graph).pwgts;
    nparts = (*ctrl).nparts;
    minwgt = libmetis__iwspacemalloc(ctrl, nparts);
    maxwgt = libmetis__iwspacemalloc(ctrl, nparts);
    itpwgts = libmetis__iwspacemalloc(ctrl, nparts);
    i = 0 as libc::c_int;
    while i < nparts {
        *itpwgts
            .offset(
                i as isize,
            ) = (*((*ctrl).tpwgts).offset(i as isize)
            * *((*graph).tvwgt).offset(0 as libc::c_int as isize) as libc::c_float)
            as idx_t;
        *maxwgt
            .offset(
                i as isize,
            ) = (*((*ctrl).tpwgts).offset(i as isize)
            * *((*graph).tvwgt).offset(0 as libc::c_int as isize) as libc::c_float
            * *((*ctrl).ubfactors).offset(0 as libc::c_int as isize)) as idx_t;
        *minwgt
            .offset(
                i as isize,
            ) = ((*((*ctrl).tpwgts).offset(i as isize)
            * *((*graph).tvwgt).offset(0 as libc::c_int as isize) as libc::c_float)
            as libc::c_double
            * (1.0f64
                / *((*ctrl).ubfactors).offset(0 as libc::c_int as isize)
                    as libc::c_double)) as idx_t;
        i += 1;
        i;
    }
    perm = libmetis__iwspacemalloc(ctrl, nvtxs);
    safetos = libmetis__iset(
        nparts as size_t,
        2 as libc::c_int,
        libmetis__iwspacemalloc(ctrl, nparts),
    );
    if (*ctrl).minconn != 0 {
        libmetis__ComputeSubDomainGraph(ctrl, graph);
        nads = (*ctrl).nads;
        adids = (*ctrl).adids;
        adwgts = (*ctrl).adwgts;
        doms = libmetis__iset(nparts as size_t, 0 as libc::c_int, (*ctrl).pvec1);
    }
    vstatus = libmetis__iset(
        nvtxs as size_t,
        3 as libc::c_int,
        libmetis__iwspacemalloc(ctrl, nvtxs),
    );
    updptr = libmetis__iset(
        nvtxs as size_t,
        -(1 as libc::c_int),
        libmetis__iwspacemalloc(ctrl, nvtxs),
    );
    updind = libmetis__iwspacemalloc(ctrl, nvtxs);
    if (*ctrl).contig != 0 {
        bfslvl = libmetis__iset(
            nvtxs as size_t,
            0 as libc::c_int,
            libmetis__iwspacemalloc(ctrl, nvtxs),
        );
        bfsind = libmetis__iwspacemalloc(ctrl, nvtxs);
        bfsmrk = libmetis__iset(
            nvtxs as size_t,
            0 as libc::c_int,
            libmetis__iwspacemalloc(ctrl, nvtxs),
        );
    }
    modind = libmetis__iwspacemalloc(ctrl, nvtxs);
    vmarker = libmetis__iset(
        nvtxs as size_t,
        0 as libc::c_int,
        libmetis__iwspacemalloc(ctrl, nvtxs),
    );
    pmarker = libmetis__iset(
        nparts as size_t,
        -(1 as libc::c_int),
        libmetis__iwspacemalloc(ctrl, nparts),
    );
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_REFINE as libc::c_int as libc::c_uint
        != 0
    {
        printf(
            b"%s: [%6d %6d]-[%6d %6d], Bal: %5.3f, Nv-Nb[%6d %6d], Cut: %5d, Vol: %5d\0"
                as *const u8 as *const libc::c_char,
            if omode == 1 as libc::c_int {
                b"GRV\0" as *const u8 as *const libc::c_char
            } else {
                b"GBV\0" as *const u8 as *const libc::c_char
            },
            *pwgts.offset(libmetis__iargmin(nparts as size_t, pwgts) as isize),
            libmetis__imax(nparts as size_t, pwgts),
            *minwgt.offset(0 as libc::c_int as isize),
            *maxwgt.offset(0 as libc::c_int as isize),
            libmetis__ComputeLoadImbalance(graph, nparts, (*ctrl).pijbm)
                as libc::c_double,
            (*graph).nvtxs,
            (*graph).nbnd,
            (*graph).mincut,
            (*graph).minvol,
        );
        if (*ctrl).minconn != 0 {
            printf(
                b", Doms: [%3d %4d]\0" as *const u8 as *const libc::c_char,
                libmetis__imax(nparts as size_t, nads),
                libmetis__isum(nparts as size_t, nads, 1 as libc::c_int as size_t),
            );
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
    }
    queue = libmetis__ipqCreate(nvtxs as size_t);
    pass = 0 as libc::c_int;
    while pass < niter {
        if omode == 2 as libc::c_int {
            i = 0 as libc::c_int;
            while i < nparts {
                if *pwgts.offset(i as isize) > *maxwgt.offset(i as isize) {
                    break;
                }
                i += 1;
                i;
            }
            if i == nparts {
                break;
            }
        }
        oldcut = (*graph).mincut;
        oldvol = (*graph).minvol;
        nupd = 0 as libc::c_int;
        if (*ctrl).minconn != 0 {
            maxndoms = libmetis__imax(nparts as size_t, nads);
        }
        libmetis__irandArrayPermute(
            (*graph).nbnd,
            perm,
            (*graph).nbnd / 4 as libc::c_int,
            1 as libc::c_int,
        );
        ii = 0 as libc::c_int;
        while ii < (*graph).nbnd {
            i = *bndind.offset(*perm.offset(ii as isize) as isize);
            libmetis__ipqInsert(queue, i, (*((*graph).vkrinfo).offset(i as isize)).gv);
            *vstatus.offset(i as isize) = 1 as libc::c_int;
            *updind.offset(nupd as isize) = i;
            let fresh11 = nupd;
            nupd = nupd + 1;
            *updptr.offset(i as isize) = fresh11;
            ii += 1;
            ii;
        }
        let mut current_block_144: u64;
        nmoved = 0 as libc::c_int;
        iii = 0 as libc::c_int;
        loop {
            i = libmetis__ipqGetTop(queue);
            if i == -(1 as libc::c_int) {
                break;
            }
            *vstatus.offset(i as isize) = 2 as libc::c_int;
            myrinfo = ((*graph).vkrinfo).offset(i as isize);
            mynbrs = ((*ctrl).vnbrpool).offset((*myrinfo).inbr as isize);
            from = *where_0.offset(i as isize);
            vwgt = *((*graph).vwgt).offset(i as isize);
            if omode == 1 as libc::c_int {
                if (*myrinfo).nid > 0 as libc::c_int
                    && *pwgts.offset(from as isize) - vwgt
                        < *minwgt.offset(from as isize)
                {
                    current_block_144 = 9859671972921157070;
                } else {
                    current_block_144 = 5028470053297453708;
                }
            } else if *pwgts.offset(from as isize) - vwgt < *minwgt.offset(from as isize)
            {
                current_block_144 = 9859671972921157070;
            } else {
                current_block_144 = 5028470053297453708;
            }
            match current_block_144 {
                5028470053297453708 => {
                    if !((*ctrl).contig != 0
                        && libmetis__IsArticulationNode(
                            i,
                            xadj,
                            adjncy,
                            where_0,
                            bfslvl,
                            bfsind,
                            bfsmrk,
                        ) != 0)
                    {
                        if (*ctrl).minconn != 0 {
                            let mut j_0: idx_t = 0;
                            let mut k_0: idx_t = 0;
                            let mut l_0: idx_t = 0;
                            let mut nadd: idx_t = 0;
                            let mut to_0: idx_t = 0;
                            j_0 = 0 as libc::c_int;
                            while j_0 < (*myrinfo).nnbrs {
                                to_0 = (*mynbrs.offset(j_0 as isize)).pid;
                                *safetos.offset(to_0 as isize) = 0 as libc::c_int;
                                k_0 = 0 as libc::c_int;
                                while k_0 < *nads.offset(to_0 as isize) {
                                    *doms
                                        .offset(
                                            *(*adids.offset(to_0 as isize)).offset(k_0 as isize)
                                                as isize,
                                        ) = 1 as libc::c_int;
                                    k_0 += 1;
                                    k_0;
                                }
                                nadd = 0 as libc::c_int;
                                k_0 = 0 as libc::c_int;
                                while k_0 < (*myrinfo).nnbrs {
                                    if !(k_0 == j_0) {
                                        l_0 = (*mynbrs.offset(k_0 as isize)).pid;
                                        if *doms.offset(l_0 as isize) == 0 as libc::c_int {
                                            if *nads.offset(l_0 as isize) > maxndoms - 1 as libc::c_int
                                            {
                                                nadd = maxndoms;
                                                break;
                                            } else {
                                                nadd += 1;
                                                nadd;
                                            }
                                        }
                                    }
                                    k_0 += 1;
                                    k_0;
                                }
                                if *nads.offset(to_0 as isize) + nadd <= maxndoms {
                                    *safetos.offset(to_0 as isize) = 1 as libc::c_int;
                                }
                                if nadd == 0 as libc::c_int {
                                    *safetos.offset(to_0 as isize) = 2 as libc::c_int;
                                }
                                k_0 = 0 as libc::c_int;
                                while k_0 < *nads.offset(to_0 as isize) {
                                    *doms
                                        .offset(
                                            *(*adids.offset(to_0 as isize)).offset(k_0 as isize)
                                                as isize,
                                        ) = 0 as libc::c_int;
                                    k_0 += 1;
                                    k_0;
                                }
                                j_0 += 1;
                                j_0;
                            }
                        }
                        xgain = if (*myrinfo).nid == 0 as libc::c_int
                            && (*myrinfo).ned > 0 as libc::c_int
                        {
                            *((*graph).vsize).offset(i as isize)
                        } else {
                            0 as libc::c_int
                        };
                        if omode == 1 as libc::c_int {
                            k = (*myrinfo).nnbrs - 1 as libc::c_int;
                            while k >= 0 as libc::c_int {
                                to = (*mynbrs.offset(k as isize)).pid;
                                if !(*safetos.offset(to as isize) == 0) {
                                    gain = (*mynbrs.offset(k as isize)).gv + xgain;
                                    if gain >= 0 as libc::c_int
                                        && (*pwgts.offset(to as isize) + vwgt) as libc::c_float
                                            <= *maxwgt.offset(to as isize) as libc::c_float
                                                + ffactor * gain as libc::c_float
                                    {
                                        break;
                                    }
                                }
                                k -= 1;
                                k;
                            }
                            if k < 0 as libc::c_int {
                                current_block_144 = 9859671972921157070;
                            } else {
                                j = k - 1 as libc::c_int;
                                while j >= 0 as libc::c_int {
                                    to = (*mynbrs.offset(j as isize)).pid;
                                    if !(*safetos.offset(to as isize) == 0) {
                                        gain = (*mynbrs.offset(j as isize)).gv + xgain;
                                        if (*mynbrs.offset(j as isize)).gv
                                            > (*mynbrs.offset(k as isize)).gv
                                            && (*pwgts.offset(to as isize) + vwgt) as libc::c_float
                                                <= *maxwgt.offset(to as isize) as libc::c_float
                                                    + ffactor * gain as libc::c_float
                                            || (*mynbrs.offset(j as isize)).gv
                                                == (*mynbrs.offset(k as isize)).gv
                                                && (*mynbrs.offset(j as isize)).ned
                                                    > (*mynbrs.offset(k as isize)).ned
                                                && *pwgts.offset(to as isize) + vwgt
                                                    <= *maxwgt.offset(to as isize)
                                            || (*mynbrs.offset(j as isize)).gv
                                                == (*mynbrs.offset(k as isize)).gv
                                                && (*mynbrs.offset(j as isize)).ned
                                                    == (*mynbrs.offset(k as isize)).ned
                                                && *itpwgts
                                                    .offset((*mynbrs.offset(k as isize)).pid as isize)
                                                    * *pwgts.offset(to as isize)
                                                    < *itpwgts.offset(to as isize)
                                                        * *pwgts.offset((*mynbrs.offset(k as isize)).pid as isize)
                                        {
                                            k = j;
                                        }
                                    }
                                    j -= 1;
                                    j;
                                }
                                to = (*mynbrs.offset(k as isize)).pid;
                                j = 0 as libc::c_int;
                                if xgain + (*mynbrs.offset(k as isize)).gv
                                    > 0 as libc::c_int
                                    || (*mynbrs.offset(k as isize)).ned - (*myrinfo).nid
                                        > 0 as libc::c_int
                                {
                                    j = 1 as libc::c_int;
                                } else if (*mynbrs.offset(k as isize)).ned - (*myrinfo).nid
                                    == 0 as libc::c_int
                                {
                                    if iii % 2 as libc::c_int == 0 as libc::c_int
                                        && *safetos.offset(to as isize) == 2 as libc::c_int
                                        || *pwgts.offset(from as isize)
                                            >= *maxwgt.offset(from as isize)
                                        || *itpwgts.offset(from as isize)
                                            * (*pwgts.offset(to as isize) + vwgt)
                                            < *itpwgts.offset(to as isize)
                                                * *pwgts.offset(from as isize)
                                    {
                                        j = 1 as libc::c_int;
                                    }
                                }
                                if j == 0 as libc::c_int {
                                    current_block_144 = 9859671972921157070;
                                } else {
                                    current_block_144 = 13932507243822716336;
                                }
                            }
                        } else {
                            k = (*myrinfo).nnbrs - 1 as libc::c_int;
                            while k >= 0 as libc::c_int {
                                to = (*mynbrs.offset(k as isize)).pid;
                                if !(*safetos.offset(to as isize) == 0) {
                                    if *pwgts.offset(to as isize) + vwgt
                                        <= *maxwgt.offset(to as isize)
                                        || *itpwgts.offset(from as isize)
                                            * (*pwgts.offset(to as isize) + vwgt)
                                            <= *itpwgts.offset(to as isize)
                                                * *pwgts.offset(from as isize)
                                    {
                                        break;
                                    }
                                }
                                k -= 1;
                                k;
                            }
                            if k < 0 as libc::c_int {
                                current_block_144 = 9859671972921157070;
                            } else {
                                j = k - 1 as libc::c_int;
                                while j >= 0 as libc::c_int {
                                    to = (*mynbrs.offset(j as isize)).pid;
                                    if !(*safetos.offset(to as isize) == 0) {
                                        if *itpwgts
                                            .offset((*mynbrs.offset(k as isize)).pid as isize)
                                            * *pwgts.offset(to as isize)
                                            < *itpwgts.offset(to as isize)
                                                * *pwgts.offset((*mynbrs.offset(k as isize)).pid as isize)
                                        {
                                            k = j;
                                        }
                                    }
                                    j -= 1;
                                    j;
                                }
                                to = (*mynbrs.offset(k as isize)).pid;
                                if *pwgts.offset(from as isize)
                                    < *maxwgt.offset(from as isize)
                                    && *pwgts.offset(to as isize) > *minwgt.offset(to as isize)
                                    && (xgain + (*mynbrs.offset(k as isize)).gv
                                        < 0 as libc::c_int
                                        || xgain + (*mynbrs.offset(k as isize)).gv
                                            == 0 as libc::c_int
                                            && (*mynbrs.offset(k as isize)).ned - (*myrinfo).nid
                                                < 0 as libc::c_int)
                                {
                                    current_block_144 = 9859671972921157070;
                                } else {
                                    current_block_144 = 13932507243822716336;
                                }
                            }
                        }
                        match current_block_144 {
                            9859671972921157070 => {}
                            _ => {
                                let ref mut fresh12 = *pwgts.offset(to as isize);
                                *fresh12 += vwgt;
                                let ref mut fresh13 = *pwgts.offset(from as isize);
                                *fresh13 -= vwgt;
                                (*graph).mincut
                                    -= (*mynbrs.offset(k as isize)).ned - (*myrinfo).nid;
                                (*graph).minvol -= xgain + (*mynbrs.offset(k as isize)).gv;
                                *where_0.offset(i as isize) = to;
                                nmoved += 1;
                                nmoved;
                                if (*ctrl).dbglvl as libc::c_uint
                                    & METIS_DBG_MOVEINFO as libc::c_int as libc::c_uint != 0
                                {
                                    printf(
                                        b"\t\tMoving %6d from %3d to %3d. Gain: [%4d %4d]. Cut: %6d, Vol: %6d\n\0"
                                            as *const u8 as *const libc::c_char,
                                        i,
                                        from,
                                        to,
                                        xgain + (*mynbrs.offset(k as isize)).gv,
                                        (*mynbrs.offset(k as isize)).ned - (*myrinfo).nid,
                                        (*graph).mincut,
                                        (*graph).minvol,
                                    );
                                }
                                if (*ctrl).minconn != 0 {
                                    libmetis__UpdateEdgeSubDomainGraph(
                                        ctrl,
                                        from,
                                        to,
                                        (*myrinfo).nid - (*mynbrs.offset(k as isize)).ned,
                                        &mut maxndoms,
                                    );
                                    j = *xadj.offset(i as isize);
                                    while j < *xadj.offset((i + 1 as libc::c_int) as isize) {
                                        me = *where_0.offset(*adjncy.offset(j as isize) as isize);
                                        if me != from && me != to {
                                            libmetis__UpdateEdgeSubDomainGraph(
                                                ctrl,
                                                from,
                                                me,
                                                -(1 as libc::c_int),
                                                &mut maxndoms,
                                            );
                                            libmetis__UpdateEdgeSubDomainGraph(
                                                ctrl,
                                                to,
                                                me,
                                                1 as libc::c_int,
                                                &mut maxndoms,
                                            );
                                        }
                                        j += 1;
                                        j;
                                    }
                                }
                                libmetis__KWayVolUpdate(
                                    ctrl,
                                    graph,
                                    i,
                                    from,
                                    to,
                                    queue,
                                    vstatus,
                                    &mut nupd,
                                    updptr,
                                    updind,
                                    bndtype,
                                    vmarker,
                                    pmarker,
                                    modind,
                                );
                            }
                        }
                    }
                }
                _ => {}
            }
            iii += 1;
            iii;
        }
        i = 0 as libc::c_int;
        while i < nupd {
            *vstatus.offset(*updind.offset(i as isize) as isize) = 3 as libc::c_int;
            *updptr.offset(*updind.offset(i as isize) as isize) = -(1 as libc::c_int);
            i += 1;
            i;
        }
        if (*ctrl).dbglvl as libc::c_uint
            & METIS_DBG_REFINE as libc::c_int as libc::c_uint != 0
        {
            printf(
                b"\t[%6d %6d], Bal: %5.3f, Nb: %6d. Nmoves: %5d, Cut: %6d, Vol: %6d\0"
                    as *const u8 as *const libc::c_char,
                *pwgts.offset(libmetis__iargmin(nparts as size_t, pwgts) as isize),
                libmetis__imax(nparts as size_t, pwgts),
                libmetis__ComputeLoadImbalance(graph, nparts, (*ctrl).pijbm)
                    as libc::c_double,
                (*graph).nbnd,
                nmoved,
                (*graph).mincut,
                (*graph).minvol,
            );
            if (*ctrl).minconn != 0 {
                printf(
                    b", Doms: [%3d %4d]\0" as *const u8 as *const libc::c_char,
                    libmetis__imax(nparts as size_t, nads),
                    libmetis__isum(nparts as size_t, nads, 1 as libc::c_int as size_t),
                );
            }
            printf(b"\n\0" as *const u8 as *const libc::c_char);
        }
        if nmoved == 0 as libc::c_int
            || omode == 1 as libc::c_int && (*graph).minvol == oldvol
                && (*graph).mincut == oldcut
        {
            break;
        }
        pass += 1;
        pass;
    }
    libmetis__ipqDestroy(queue);
    libmetis__wspacepop(ctrl);
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__Greedy_McKWayCutOptimize(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut niter: idx_t,
    mut ffactor: real_t,
    mut omode: idx_t,
) {
    let mut i: idx_t = 0;
    let mut ii: idx_t = 0;
    let mut iii: idx_t = 0;
    let mut j: idx_t = 0;
    let mut k: idx_t = 0;
    let mut l: idx_t = 0;
    let mut pass: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut ncon: idx_t = 0;
    let mut nparts: idx_t = 0;
    let mut gain: idx_t = 0;
    let mut from: idx_t = 0;
    let mut me: idx_t = 0;
    let mut to: idx_t = 0;
    let mut cto: idx_t = 0;
    let mut oldcut: idx_t = 0;
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
    let mut vwgt: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut adjwgt: *mut idx_t = 0 as *mut idx_t;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    let mut pwgts: *mut idx_t = 0 as *mut idx_t;
    let mut perm: *mut idx_t = 0 as *mut idx_t;
    let mut bndptr: *mut idx_t = 0 as *mut idx_t;
    let mut bndind: *mut idx_t = 0 as *mut idx_t;
    let mut minwgt: *mut idx_t = 0 as *mut idx_t;
    let mut maxwgt: *mut idx_t = 0 as *mut idx_t;
    let mut nmoved: idx_t = 0;
    let mut nupd: idx_t = 0;
    let mut vstatus: *mut idx_t = 0 as *mut idx_t;
    let mut updptr: *mut idx_t = 0 as *mut idx_t;
    let mut updind: *mut idx_t = 0 as *mut idx_t;
    let mut maxndoms: idx_t = 0;
    let mut safetos: *mut idx_t = 0 as *mut idx_t;
    let mut nads: *mut idx_t = 0 as *mut idx_t;
    let mut doms: *mut idx_t = 0 as *mut idx_t;
    let mut adids: *mut *mut idx_t = 0 as *mut *mut idx_t;
    let mut adwgts: *mut *mut idx_t = 0 as *mut *mut idx_t;
    let mut bfslvl: *mut idx_t = 0 as *mut idx_t;
    let mut bfsind: *mut idx_t = 0 as *mut idx_t;
    let mut bfsmrk: *mut idx_t = 0 as *mut idx_t;
    let mut bndtype: idx_t = if omode == 1 as libc::c_int {
        1 as libc::c_int
    } else {
        2 as libc::c_int
    };
    let mut ubfactors: *mut real_t = 0 as *mut real_t;
    let mut pijbm: *mut real_t = 0 as *mut real_t;
    let mut origbal: real_t = 0.;
    let mut nbnd: idx_t = 0;
    let mut oldnnbrs: idx_t = 0;
    let mut queue: *mut rpq_t = 0 as *mut rpq_t;
    let mut rgain: real_t = 0.;
    let mut myrinfo: *mut ckrinfo_t = 0 as *mut ckrinfo_t;
    let mut mynbrs: *mut cnbr_t = 0 as *mut cnbr_t;
    libmetis__wspacepush(ctrl);
    nvtxs = (*graph).nvtxs;
    ncon = (*graph).ncon;
    xadj = (*graph).xadj;
    vwgt = (*graph).vwgt;
    adjncy = (*graph).adjncy;
    adjwgt = (*graph).adjwgt;
    bndind = (*graph).bndind;
    bndptr = (*graph).bndptr;
    where_0 = (*graph).where_0;
    pwgts = (*graph).pwgts;
    nparts = (*ctrl).nparts;
    pijbm = (*ctrl).pijbm;
    ubfactors = libmetis__rwspacemalloc(ctrl, ncon);
    libmetis__ComputeLoadImbalanceVec(graph, nparts, pijbm, ubfactors);
    origbal = libmetis__rvecmaxdiff(ncon, ubfactors, (*ctrl).ubfactors);
    if omode == 2 as libc::c_int {
        libmetis__rcopy(ncon as size_t, (*ctrl).ubfactors, ubfactors);
    } else {
        i = 0 as libc::c_int;
        while i < ncon {
            *ubfactors
                .offset(
                    i as isize,
                ) = if *ubfactors.offset(i as isize)
                > *((*ctrl).ubfactors).offset(i as isize)
            {
                *ubfactors.offset(i as isize)
            } else {
                *((*ctrl).ubfactors).offset(i as isize)
            };
            i += 1;
            i;
        }
    }
    minwgt = libmetis__iwspacemalloc(ctrl, nparts * ncon);
    maxwgt = libmetis__iwspacemalloc(ctrl, nparts * ncon);
    i = 0 as libc::c_int;
    while i < nparts {
        j = 0 as libc::c_int;
        while j < ncon {
            *maxwgt
                .offset(
                    (i * ncon + j) as isize,
                ) = (*((*ctrl).tpwgts).offset((i * ncon + j) as isize)
                * *((*graph).tvwgt).offset(j as isize) as libc::c_float
                * *ubfactors.offset(j as isize)) as idx_t;
            *minwgt
                .offset(
                    (i * ncon + j) as isize,
                ) = ((*((*ctrl).tpwgts).offset((i * ncon + j) as isize)
                * *((*graph).tvwgt).offset(j as isize) as libc::c_float)
                as libc::c_double * 0.2f64) as idx_t;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    perm = libmetis__iwspacemalloc(ctrl, nvtxs);
    safetos = libmetis__iset(
        nparts as size_t,
        2 as libc::c_int,
        libmetis__iwspacemalloc(ctrl, nparts),
    );
    if (*ctrl).minconn != 0 {
        libmetis__ComputeSubDomainGraph(ctrl, graph);
        nads = (*ctrl).nads;
        adids = (*ctrl).adids;
        adwgts = (*ctrl).adwgts;
        doms = libmetis__iset(nparts as size_t, 0 as libc::c_int, (*ctrl).pvec1);
    }
    vstatus = libmetis__iset(
        nvtxs as size_t,
        3 as libc::c_int,
        libmetis__iwspacemalloc(ctrl, nvtxs),
    );
    updptr = libmetis__iset(
        nvtxs as size_t,
        -(1 as libc::c_int),
        libmetis__iwspacemalloc(ctrl, nvtxs),
    );
    updind = libmetis__iwspacemalloc(ctrl, nvtxs);
    if (*ctrl).contig != 0 {
        bfslvl = libmetis__iset(
            nvtxs as size_t,
            0 as libc::c_int,
            libmetis__iwspacemalloc(ctrl, nvtxs),
        );
        bfsind = libmetis__iwspacemalloc(ctrl, nvtxs);
        bfsmrk = libmetis__iset(
            nvtxs as size_t,
            0 as libc::c_int,
            libmetis__iwspacemalloc(ctrl, nvtxs),
        );
    }
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_REFINE as libc::c_int as libc::c_uint
        != 0
    {
        printf(
            b"%s: [%6d %6d %6d], Bal: %5.3f(%.3f), Nv-Nb[%6d %6d], Cut: %6d, (%d)\0"
                as *const u8 as *const libc::c_char,
            if omode == 1 as libc::c_int {
                b"GRC\0" as *const u8 as *const libc::c_char
            } else {
                b"GBC\0" as *const u8 as *const libc::c_char
            },
            libmetis__imin((nparts * ncon) as size_t, pwgts),
            libmetis__imax((nparts * ncon) as size_t, pwgts),
            libmetis__imax((nparts * ncon) as size_t, maxwgt),
            libmetis__ComputeLoadImbalance(graph, nparts, pijbm) as libc::c_double,
            origbal as libc::c_double,
            (*graph).nvtxs,
            (*graph).nbnd,
            (*graph).mincut,
            niter,
        );
        if (*ctrl).minconn != 0 {
            printf(
                b", Doms: [%3d %4d]\0" as *const u8 as *const libc::c_char,
                libmetis__imax(nparts as size_t, nads),
                libmetis__isum(nparts as size_t, nads, 1 as libc::c_int as size_t),
            );
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
    }
    queue = libmetis__rpqCreate(nvtxs as size_t);
    pass = 0 as libc::c_int;
    while pass < niter {
        if omode == 2 as libc::c_int
            && libmetis__IsBalanced(ctrl, graph, 0 as libc::c_int as real_t) != 0
        {
            break;
        }
        oldcut = (*graph).mincut;
        nbnd = (*graph).nbnd;
        nupd = 0 as libc::c_int;
        if (*ctrl).minconn != 0 {
            maxndoms = libmetis__imax(nparts as size_t, nads);
        }
        libmetis__irandArrayPermute(
            nbnd,
            perm,
            nbnd / 4 as libc::c_int,
            1 as libc::c_int,
        );
        ii = 0 as libc::c_int;
        while ii < nbnd {
            i = *bndind.offset(*perm.offset(ii as isize) as isize);
            rgain = ((if (*((*graph).ckrinfo).offset(i as isize)).nnbrs
                > 0 as libc::c_int
            {
                1.0f64 * (*((*graph).ckrinfo).offset(i as isize)).ed as libc::c_double
                    / sqrt(
                        (*((*graph).ckrinfo).offset(i as isize)).nnbrs as libc::c_double,
                    )
            } else {
                0.0f64
            }) - (*((*graph).ckrinfo).offset(i as isize)).id as libc::c_double)
                as real_t;
            libmetis__rpqInsert(queue, i, rgain);
            *vstatus.offset(i as isize) = 1 as libc::c_int;
            *updind.offset(nupd as isize) = i;
            let fresh14 = nupd;
            nupd = nupd + 1;
            *updptr.offset(i as isize) = fresh14;
            ii += 1;
            ii;
        }
        let mut current_block_331: u64;
        nmoved = 0 as libc::c_int;
        iii = 0 as libc::c_int;
        loop {
            i = libmetis__rpqGetTop(queue);
            if i == -(1 as libc::c_int) {
                break;
            }
            *vstatus.offset(i as isize) = 2 as libc::c_int;
            myrinfo = ((*graph).ckrinfo).offset(i as isize);
            mynbrs = ((*ctrl).cnbrpool).offset((*myrinfo).inbr as isize);
            from = *where_0.offset(i as isize);
            if omode == 1 as libc::c_int {
                if (*myrinfo).id > 0 as libc::c_int
                    && libmetis__ivecaxpygez(
                        ncon,
                        -(1 as libc::c_int),
                        vwgt.offset((i * ncon) as isize),
                        pwgts.offset((from * ncon) as isize),
                        minwgt.offset((from * ncon) as isize),
                    ) == 0
                {
                    current_block_331 = 307447392441238883;
                } else {
                    current_block_331 = 9437375157805982253;
                }
            } else if libmetis__ivecaxpygez(
                ncon,
                -(1 as libc::c_int),
                vwgt.offset((i * ncon) as isize),
                pwgts.offset((from * ncon) as isize),
                minwgt.offset((from * ncon) as isize),
            ) == 0
            {
                current_block_331 = 307447392441238883;
            } else {
                current_block_331 = 9437375157805982253;
            }
            match current_block_331 {
                9437375157805982253 => {
                    if !((*ctrl).contig != 0
                        && libmetis__IsArticulationNode(
                            i,
                            xadj,
                            adjncy,
                            where_0,
                            bfslvl,
                            bfsind,
                            bfsmrk,
                        ) != 0)
                    {
                        if (*ctrl).minconn != 0 {
                            let mut j_0: idx_t = 0;
                            let mut k_0: idx_t = 0;
                            let mut l_0: idx_t = 0;
                            let mut nadd: idx_t = 0;
                            let mut to_0: idx_t = 0;
                            j_0 = 0 as libc::c_int;
                            while j_0 < (*myrinfo).nnbrs {
                                to_0 = (*mynbrs.offset(j_0 as isize)).pid;
                                *safetos.offset(to_0 as isize) = 0 as libc::c_int;
                                k_0 = 0 as libc::c_int;
                                while k_0 < *nads.offset(to_0 as isize) {
                                    *doms
                                        .offset(
                                            *(*adids.offset(to_0 as isize)).offset(k_0 as isize)
                                                as isize,
                                        ) = 1 as libc::c_int;
                                    k_0 += 1;
                                    k_0;
                                }
                                nadd = 0 as libc::c_int;
                                k_0 = 0 as libc::c_int;
                                while k_0 < (*myrinfo).nnbrs {
                                    if !(k_0 == j_0) {
                                        l_0 = (*mynbrs.offset(k_0 as isize)).pid;
                                        if *doms.offset(l_0 as isize) == 0 as libc::c_int {
                                            if *nads.offset(l_0 as isize) > maxndoms - 1 as libc::c_int
                                            {
                                                nadd = maxndoms;
                                                break;
                                            } else {
                                                nadd += 1;
                                                nadd;
                                            }
                                        }
                                    }
                                    k_0 += 1;
                                    k_0;
                                }
                                if *nads.offset(to_0 as isize) + nadd <= maxndoms {
                                    *safetos.offset(to_0 as isize) = 1 as libc::c_int;
                                }
                                if nadd == 0 as libc::c_int {
                                    *safetos.offset(to_0 as isize) = 2 as libc::c_int;
                                }
                                k_0 = 0 as libc::c_int;
                                while k_0 < *nads.offset(to_0 as isize) {
                                    *doms
                                        .offset(
                                            *(*adids.offset(to_0 as isize)).offset(k_0 as isize)
                                                as isize,
                                        ) = 0 as libc::c_int;
                                    k_0 += 1;
                                    k_0;
                                }
                                j_0 += 1;
                                j_0;
                            }
                        }
                        if omode == 1 as libc::c_int {
                            k = (*myrinfo).nnbrs - 1 as libc::c_int;
                            while k >= 0 as libc::c_int {
                                to = (*mynbrs.offset(k as isize)).pid;
                                if !(*safetos.offset(to as isize) == 0) {
                                    gain = (*mynbrs.offset(k as isize)).ed - (*myrinfo).id;
                                    if gain >= 0 as libc::c_int
                                        && libmetis__ivecaxpylez(
                                            ncon,
                                            1 as libc::c_int,
                                            vwgt.offset((i * ncon) as isize),
                                            pwgts.offset((to * ncon) as isize),
                                            maxwgt.offset((to * ncon) as isize),
                                        ) != 0
                                    {
                                        break;
                                    }
                                }
                                k -= 1;
                                k;
                            }
                            if k < 0 as libc::c_int {
                                current_block_331 = 307447392441238883;
                            } else {
                                cto = to;
                                j = k - 1 as libc::c_int;
                                while j >= 0 as libc::c_int {
                                    to = (*mynbrs.offset(j as isize)).pid;
                                    if !(*safetos.offset(to as isize) == 0) {
                                        if (*mynbrs.offset(j as isize)).ed
                                            > (*mynbrs.offset(k as isize)).ed
                                            && libmetis__ivecaxpylez(
                                                ncon,
                                                1 as libc::c_int,
                                                vwgt.offset((i * ncon) as isize),
                                                pwgts.offset((to * ncon) as isize),
                                                maxwgt.offset((to * ncon) as isize),
                                            ) != 0
                                            || (*mynbrs.offset(j as isize)).ed
                                                == (*mynbrs.offset(k as isize)).ed
                                                && libmetis__BetterBalanceKWay(
                                                    ncon,
                                                    vwgt.offset((i * ncon) as isize),
                                                    ubfactors,
                                                    1 as libc::c_int,
                                                    pwgts.offset((cto * ncon) as isize),
                                                    pijbm.offset((cto * ncon) as isize),
                                                    1 as libc::c_int,
                                                    pwgts.offset((to * ncon) as isize),
                                                    pijbm.offset((to * ncon) as isize),
                                                ) != 0
                                        {
                                            k = j;
                                            cto = to;
                                        }
                                    }
                                    j -= 1;
                                    j;
                                }
                                to = cto;
                                gain = (*mynbrs.offset(k as isize)).ed - (*myrinfo).id;
                                if !(gain > 0 as libc::c_int
                                    || gain == 0 as libc::c_int
                                        && (libmetis__BetterBalanceKWay(
                                            ncon,
                                            vwgt.offset((i * ncon) as isize),
                                            ubfactors,
                                            -(1 as libc::c_int),
                                            pwgts.offset((from * ncon) as isize),
                                            pijbm.offset((from * ncon) as isize),
                                            1 as libc::c_int,
                                            pwgts.offset((to * ncon) as isize),
                                            pijbm.offset((to * ncon) as isize),
                                        ) != 0
                                            || iii % 2 as libc::c_int == 0 as libc::c_int
                                                && *safetos.offset(to as isize) == 2 as libc::c_int))
                                {
                                    current_block_331 = 307447392441238883;
                                } else {
                                    current_block_331 = 18201902862271706575;
                                }
                            }
                        } else {
                            k = (*myrinfo).nnbrs - 1 as libc::c_int;
                            while k >= 0 as libc::c_int {
                                to = (*mynbrs.offset(k as isize)).pid;
                                if !(*safetos.offset(to as isize) == 0) {
                                    if libmetis__ivecaxpylez(
                                        ncon,
                                        1 as libc::c_int,
                                        vwgt.offset((i * ncon) as isize),
                                        pwgts.offset((to * ncon) as isize),
                                        maxwgt.offset((to * ncon) as isize),
                                    ) != 0
                                        || libmetis__BetterBalanceKWay(
                                            ncon,
                                            vwgt.offset((i * ncon) as isize),
                                            ubfactors,
                                            -(1 as libc::c_int),
                                            pwgts.offset((from * ncon) as isize),
                                            pijbm.offset((from * ncon) as isize),
                                            1 as libc::c_int,
                                            pwgts.offset((to * ncon) as isize),
                                            pijbm.offset((to * ncon) as isize),
                                        ) != 0
                                    {
                                        break;
                                    }
                                }
                                k -= 1;
                                k;
                            }
                            if k < 0 as libc::c_int {
                                current_block_331 = 307447392441238883;
                            } else {
                                cto = to;
                                j = k - 1 as libc::c_int;
                                while j >= 0 as libc::c_int {
                                    to = (*mynbrs.offset(j as isize)).pid;
                                    if !(*safetos.offset(to as isize) == 0) {
                                        if libmetis__BetterBalanceKWay(
                                            ncon,
                                            vwgt.offset((i * ncon) as isize),
                                            ubfactors,
                                            1 as libc::c_int,
                                            pwgts.offset((cto * ncon) as isize),
                                            pijbm.offset((cto * ncon) as isize),
                                            1 as libc::c_int,
                                            pwgts.offset((to * ncon) as isize),
                                            pijbm.offset((to * ncon) as isize),
                                        ) != 0
                                        {
                                            k = j;
                                            cto = to;
                                        }
                                    }
                                    j -= 1;
                                    j;
                                }
                                to = cto;
                                if (*mynbrs.offset(k as isize)).ed - (*myrinfo).id
                                    < 0 as libc::c_int
                                    && libmetis__BetterBalanceKWay(
                                        ncon,
                                        vwgt.offset((i * ncon) as isize),
                                        ubfactors,
                                        -(1 as libc::c_int),
                                        pwgts.offset((from * ncon) as isize),
                                        pijbm.offset((from * ncon) as isize),
                                        1 as libc::c_int,
                                        pwgts.offset((to * ncon) as isize),
                                        pijbm.offset((to * ncon) as isize),
                                    ) == 0
                                {
                                    current_block_331 = 307447392441238883;
                                } else {
                                    current_block_331 = 18201902862271706575;
                                }
                            }
                        }
                        match current_block_331 {
                            307447392441238883 => {}
                            _ => {
                                (*graph).mincut
                                    -= (*mynbrs.offset(k as isize)).ed - (*myrinfo).id;
                                nmoved += 1;
                                nmoved;
                                if (*ctrl).dbglvl as libc::c_uint
                                    & METIS_DBG_MOVEINFO as libc::c_int as libc::c_uint != 0
                                {
                                    printf(
                                        b"\t\tMoving %6d to %3d. Gain: %4d. Cut: %6d\n\0"
                                            as *const u8 as *const libc::c_char,
                                        i,
                                        to,
                                        (*mynbrs.offset(k as isize)).ed - (*myrinfo).id,
                                        (*graph).mincut,
                                    );
                                }
                                if (*ctrl).minconn != 0 {
                                    libmetis__UpdateEdgeSubDomainGraph(
                                        ctrl,
                                        from,
                                        to,
                                        (*myrinfo).id - (*mynbrs.offset(k as isize)).ed,
                                        &mut maxndoms,
                                    );
                                    j = *xadj.offset(i as isize);
                                    while j < *xadj.offset((i + 1 as libc::c_int) as isize) {
                                        me = *where_0.offset(*adjncy.offset(j as isize) as isize);
                                        if me != from && me != to {
                                            libmetis__UpdateEdgeSubDomainGraph(
                                                ctrl,
                                                from,
                                                me,
                                                -*adjwgt.offset(j as isize),
                                                &mut maxndoms,
                                            );
                                            libmetis__UpdateEdgeSubDomainGraph(
                                                ctrl,
                                                to,
                                                me,
                                                *adjwgt.offset(j as isize),
                                                &mut maxndoms,
                                            );
                                        }
                                        j += 1;
                                        j;
                                    }
                                }
                                libmetis__iaxpy(
                                    ncon as size_t,
                                    1 as libc::c_int,
                                    vwgt.offset((i * ncon) as isize),
                                    1 as libc::c_int as size_t,
                                    pwgts.offset((to * ncon) as isize),
                                    1 as libc::c_int as size_t,
                                );
                                libmetis__iaxpy(
                                    ncon as size_t,
                                    -(1 as libc::c_int),
                                    vwgt.offset((i * ncon) as isize),
                                    1 as libc::c_int as size_t,
                                    pwgts.offset((from * ncon) as isize),
                                    1 as libc::c_int as size_t,
                                );
                                *where_0.offset(i as isize) = to;
                                (*myrinfo).ed
                                    += (*myrinfo).id - (*mynbrs.offset(k as isize)).ed;
                                j = (*myrinfo).id;
                                (*myrinfo).id = (*mynbrs.offset(k as isize)).ed;
                                (*mynbrs.offset(k as isize)).ed = j;
                                if (*mynbrs.offset(k as isize)).ed == 0 as libc::c_int {
                                    (*myrinfo).nnbrs -= 1;
                                    *mynbrs
                                        .offset(
                                            k as isize,
                                        ) = *mynbrs.offset((*myrinfo).nnbrs as isize);
                                } else {
                                    (*mynbrs.offset(k as isize)).pid = from;
                                }
                                if bndtype == 1 as libc::c_int {
                                    if *bndptr.offset(i as isize) != -(1 as libc::c_int)
                                        && (*myrinfo).ed - (*myrinfo).id < 0 as libc::c_int
                                    {
                                        nbnd -= 1;
                                        *bndind
                                            .offset(
                                                *bndptr.offset(i as isize) as isize,
                                            ) = *bndind.offset(nbnd as isize);
                                        *bndptr
                                            .offset(
                                                *bndind.offset(nbnd as isize) as isize,
                                            ) = *bndptr.offset(i as isize);
                                        *bndptr.offset(i as isize) = -(1 as libc::c_int);
                                    }
                                    if *bndptr.offset(i as isize) == -(1 as libc::c_int)
                                        && (*myrinfo).ed - (*myrinfo).id >= 0 as libc::c_int
                                    {
                                        *bndind.offset(nbnd as isize) = i;
                                        let fresh15 = nbnd;
                                        nbnd = nbnd + 1;
                                        *bndptr.offset(i as isize) = fresh15;
                                    }
                                } else {
                                    if *bndptr.offset(i as isize) != -(1 as libc::c_int)
                                        && (*myrinfo).ed <= 0 as libc::c_int
                                    {
                                        nbnd -= 1;
                                        *bndind
                                            .offset(
                                                *bndptr.offset(i as isize) as isize,
                                            ) = *bndind.offset(nbnd as isize);
                                        *bndptr
                                            .offset(
                                                *bndind.offset(nbnd as isize) as isize,
                                            ) = *bndptr.offset(i as isize);
                                        *bndptr.offset(i as isize) = -(1 as libc::c_int);
                                    }
                                    if *bndptr.offset(i as isize) == -(1 as libc::c_int)
                                        && (*myrinfo).ed > 0 as libc::c_int
                                    {
                                        *bndind.offset(nbnd as isize) = i;
                                        let fresh16 = nbnd;
                                        nbnd = nbnd + 1;
                                        *bndptr.offset(i as isize) = fresh16;
                                    }
                                }
                                j = *xadj.offset(i as isize);
                                while j < *xadj.offset((i + 1 as libc::c_int) as isize) {
                                    ii = *adjncy.offset(j as isize);
                                    me = *where_0.offset(ii as isize);
                                    myrinfo = ((*graph).ckrinfo).offset(ii as isize);
                                    oldnnbrs = (*myrinfo).nnbrs;
                                    let mut k_1: idx_t = 0;
                                    let mut mynbrs_0: *mut cnbr_t = 0 as *mut cnbr_t;
                                    if (*myrinfo).inbr == -(1 as libc::c_int) {
                                        (*myrinfo)
                                            .inbr = libmetis__cnbrpoolGetNext(
                                            ctrl,
                                            *xadj.offset((ii + 1 as libc::c_int) as isize)
                                                - *xadj.offset(ii as isize) + 1 as libc::c_int,
                                        );
                                        (*myrinfo).nnbrs = 0 as libc::c_int;
                                    }
                                    mynbrs_0 = ((*ctrl).cnbrpool)
                                        .offset((*myrinfo).inbr as isize);
                                    if me == from {
                                        (*myrinfo).ed += *adjwgt.offset(j as isize);
                                        (*myrinfo).id -= *adjwgt.offset(j as isize);
                                        if bndtype == 1 as libc::c_int {
                                            if (*myrinfo).ed - (*myrinfo).id >= 0 as libc::c_int
                                                && *bndptr.offset(ii as isize) == -(1 as libc::c_int)
                                            {
                                                *bndind.offset(nbnd as isize) = ii;
                                                let fresh17 = nbnd;
                                                nbnd = nbnd + 1;
                                                *bndptr.offset(ii as isize) = fresh17;
                                            }
                                        } else if (*myrinfo).ed > 0 as libc::c_int
                                            && *bndptr.offset(ii as isize) == -(1 as libc::c_int)
                                        {
                                            *bndind.offset(nbnd as isize) = ii;
                                            let fresh18 = nbnd;
                                            nbnd = nbnd + 1;
                                            *bndptr.offset(ii as isize) = fresh18;
                                        }
                                    } else if me == to {
                                        (*myrinfo).id += *adjwgt.offset(j as isize);
                                        (*myrinfo).ed -= *adjwgt.offset(j as isize);
                                        if bndtype == 1 as libc::c_int {
                                            if (*myrinfo).ed - (*myrinfo).id < 0 as libc::c_int
                                                && *bndptr.offset(ii as isize) != -(1 as libc::c_int)
                                            {
                                                nbnd -= 1;
                                                *bndind
                                                    .offset(
                                                        *bndptr.offset(ii as isize) as isize,
                                                    ) = *bndind.offset(nbnd as isize);
                                                *bndptr
                                                    .offset(
                                                        *bndind.offset(nbnd as isize) as isize,
                                                    ) = *bndptr.offset(ii as isize);
                                                *bndptr.offset(ii as isize) = -(1 as libc::c_int);
                                            }
                                        } else if (*myrinfo).ed <= 0 as libc::c_int
                                            && *bndptr.offset(ii as isize) != -(1 as libc::c_int)
                                        {
                                            nbnd -= 1;
                                            *bndind
                                                .offset(
                                                    *bndptr.offset(ii as isize) as isize,
                                                ) = *bndind.offset(nbnd as isize);
                                            *bndptr
                                                .offset(
                                                    *bndind.offset(nbnd as isize) as isize,
                                                ) = *bndptr.offset(ii as isize);
                                            *bndptr.offset(ii as isize) = -(1 as libc::c_int);
                                        }
                                    }
                                    if me != from {
                                        k_1 = 0 as libc::c_int;
                                        while k_1 < (*myrinfo).nnbrs {
                                            if (*mynbrs_0.offset(k_1 as isize)).pid == from {
                                                if (*mynbrs_0.offset(k_1 as isize)).ed
                                                    == *adjwgt.offset(j as isize)
                                                {
                                                    (*myrinfo).nnbrs -= 1;
                                                    *mynbrs_0
                                                        .offset(
                                                            k_1 as isize,
                                                        ) = *mynbrs_0.offset((*myrinfo).nnbrs as isize);
                                                } else {
                                                    let ref mut fresh19 = (*mynbrs_0.offset(k_1 as isize)).ed;
                                                    *fresh19 -= *adjwgt.offset(j as isize);
                                                }
                                                break;
                                            } else {
                                                k_1 += 1;
                                                k_1;
                                            }
                                        }
                                    }
                                    if me != to {
                                        k_1 = 0 as libc::c_int;
                                        while k_1 < (*myrinfo).nnbrs {
                                            if (*mynbrs_0.offset(k_1 as isize)).pid == to {
                                                let ref mut fresh20 = (*mynbrs_0.offset(k_1 as isize)).ed;
                                                *fresh20 += *adjwgt.offset(j as isize);
                                                break;
                                            } else {
                                                k_1 += 1;
                                                k_1;
                                            }
                                        }
                                        if k_1 == (*myrinfo).nnbrs {
                                            (*mynbrs_0.offset(k_1 as isize)).pid = to;
                                            (*mynbrs_0.offset(k_1 as isize))
                                                .ed = *adjwgt.offset(j as isize);
                                            (*myrinfo).nnbrs += 1;
                                            (*myrinfo).nnbrs;
                                        }
                                    }
                                    let mut rgain_0: real_t = 0.;
                                    if me == to || me == from || oldnnbrs != (*myrinfo).nnbrs {
                                        rgain_0 = ((if (*myrinfo).nnbrs > 0 as libc::c_int {
                                            1.0f64 * (*myrinfo).ed as libc::c_double
                                                / sqrt((*myrinfo).nnbrs as libc::c_double)
                                        } else {
                                            0.0f64
                                        }) - (*myrinfo).id as libc::c_double) as real_t;
                                        if bndtype == 1 as libc::c_int {
                                            if *vstatus.offset(ii as isize) == 1 as libc::c_int {
                                                if (*myrinfo).ed - (*myrinfo).id >= 0 as libc::c_int {
                                                    libmetis__rpqUpdate(queue, ii, rgain_0);
                                                } else {
                                                    libmetis__rpqDelete(queue, ii);
                                                    *vstatus.offset(ii as isize) = 3 as libc::c_int;
                                                    nupd -= 1;
                                                    *updind
                                                        .offset(
                                                            *updptr.offset(ii as isize) as isize,
                                                        ) = *updind.offset(nupd as isize);
                                                    *updptr
                                                        .offset(
                                                            *updind.offset(nupd as isize) as isize,
                                                        ) = *updptr.offset(ii as isize);
                                                    *updptr.offset(ii as isize) = -(1 as libc::c_int);
                                                }
                                            } else if *vstatus.offset(ii as isize) == 3 as libc::c_int
                                                && (*myrinfo).ed - (*myrinfo).id >= 0 as libc::c_int
                                            {
                                                libmetis__rpqInsert(queue, ii, rgain_0);
                                                *vstatus.offset(ii as isize) = 1 as libc::c_int;
                                                *updind.offset(nupd as isize) = ii;
                                                let fresh21 = nupd;
                                                nupd = nupd + 1;
                                                *updptr.offset(ii as isize) = fresh21;
                                            }
                                        } else if *vstatus.offset(ii as isize) == 1 as libc::c_int {
                                            if (*myrinfo).ed > 0 as libc::c_int {
                                                libmetis__rpqUpdate(queue, ii, rgain_0);
                                            } else {
                                                libmetis__rpqDelete(queue, ii);
                                                *vstatus.offset(ii as isize) = 3 as libc::c_int;
                                                nupd -= 1;
                                                *updind
                                                    .offset(
                                                        *updptr.offset(ii as isize) as isize,
                                                    ) = *updind.offset(nupd as isize);
                                                *updptr
                                                    .offset(
                                                        *updind.offset(nupd as isize) as isize,
                                                    ) = *updptr.offset(ii as isize);
                                                *updptr.offset(ii as isize) = -(1 as libc::c_int);
                                            }
                                        } else if *vstatus.offset(ii as isize) == 3 as libc::c_int
                                            && (*myrinfo).ed > 0 as libc::c_int
                                        {
                                            libmetis__rpqInsert(queue, ii, rgain_0);
                                            *vstatus.offset(ii as isize) = 1 as libc::c_int;
                                            *updind.offset(nupd as isize) = ii;
                                            let fresh22 = nupd;
                                            nupd = nupd + 1;
                                            *updptr.offset(ii as isize) = fresh22;
                                        }
                                    }
                                    j += 1;
                                    j;
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
            iii += 1;
            iii;
        }
        (*graph).nbnd = nbnd;
        i = 0 as libc::c_int;
        while i < nupd {
            *vstatus.offset(*updind.offset(i as isize) as isize) = 3 as libc::c_int;
            *updptr.offset(*updind.offset(i as isize) as isize) = -(1 as libc::c_int);
            i += 1;
            i;
        }
        if (*ctrl).dbglvl as libc::c_uint
            & METIS_DBG_REFINE as libc::c_int as libc::c_uint != 0
        {
            printf(
                b"\t[%6d %6d], Bal: %5.3f, Nb: %6d. Nmoves: %5d, Cut: %6d, Vol: %6d\0"
                    as *const u8 as *const libc::c_char,
                libmetis__imin((nparts * ncon) as size_t, pwgts),
                libmetis__imax((nparts * ncon) as size_t, pwgts),
                libmetis__ComputeLoadImbalance(graph, nparts, pijbm) as libc::c_double,
                (*graph).nbnd,
                nmoved,
                (*graph).mincut,
                libmetis__ComputeVolume(graph, where_0),
            );
            if (*ctrl).minconn != 0 {
                printf(
                    b", Doms: [%3d %4d]\0" as *const u8 as *const libc::c_char,
                    libmetis__imax(nparts as size_t, nads),
                    libmetis__isum(nparts as size_t, nads, 1 as libc::c_int as size_t),
                );
            }
            printf(b"\n\0" as *const u8 as *const libc::c_char);
        }
        if nmoved == 0 as libc::c_int
            || omode == 1 as libc::c_int && (*graph).mincut == oldcut
        {
            break;
        }
        pass += 1;
        pass;
    }
    libmetis__rpqDestroy(queue);
    libmetis__wspacepop(ctrl);
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__Greedy_McKWayVolOptimize(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut niter: idx_t,
    mut ffactor: real_t,
    mut omode: idx_t,
) {
    let mut i: idx_t = 0;
    let mut ii: idx_t = 0;
    let mut iii: idx_t = 0;
    let mut j: idx_t = 0;
    let mut k: idx_t = 0;
    let mut l: idx_t = 0;
    let mut pass: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut ncon: idx_t = 0;
    let mut nparts: idx_t = 0;
    let mut gain: idx_t = 0;
    let mut from: idx_t = 0;
    let mut me: idx_t = 0;
    let mut to: idx_t = 0;
    let mut cto: idx_t = 0;
    let mut oldcut: idx_t = 0;
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
    let mut vwgt: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    let mut pwgts: *mut idx_t = 0 as *mut idx_t;
    let mut perm: *mut idx_t = 0 as *mut idx_t;
    let mut bndptr: *mut idx_t = 0 as *mut idx_t;
    let mut bndind: *mut idx_t = 0 as *mut idx_t;
    let mut minwgt: *mut idx_t = 0 as *mut idx_t;
    let mut maxwgt: *mut idx_t = 0 as *mut idx_t;
    let mut nmoved: idx_t = 0;
    let mut nupd: idx_t = 0;
    let mut vstatus: *mut idx_t = 0 as *mut idx_t;
    let mut updptr: *mut idx_t = 0 as *mut idx_t;
    let mut updind: *mut idx_t = 0 as *mut idx_t;
    let mut maxndoms: idx_t = 0;
    let mut safetos: *mut idx_t = 0 as *mut idx_t;
    let mut nads: *mut idx_t = 0 as *mut idx_t;
    let mut doms: *mut idx_t = 0 as *mut idx_t;
    let mut adids: *mut *mut idx_t = 0 as *mut *mut idx_t;
    let mut adwgts: *mut *mut idx_t = 0 as *mut *mut idx_t;
    let mut bfslvl: *mut idx_t = 0 as *mut idx_t;
    let mut bfsind: *mut idx_t = 0 as *mut idx_t;
    let mut bfsmrk: *mut idx_t = 0 as *mut idx_t;
    let mut bndtype: idx_t = if omode == 1 as libc::c_int {
        1 as libc::c_int
    } else {
        2 as libc::c_int
    };
    let mut ubfactors: *mut real_t = 0 as *mut real_t;
    let mut pijbm: *mut real_t = 0 as *mut real_t;
    let mut origbal: real_t = 0.;
    let mut queue: *mut ipq_t = 0 as *mut ipq_t;
    let mut oldvol: idx_t = 0;
    let mut xgain: idx_t = 0;
    let mut vmarker: *mut idx_t = 0 as *mut idx_t;
    let mut pmarker: *mut idx_t = 0 as *mut idx_t;
    let mut modind: *mut idx_t = 0 as *mut idx_t;
    let mut myrinfo: *mut vkrinfo_t = 0 as *mut vkrinfo_t;
    let mut mynbrs: *mut vnbr_t = 0 as *mut vnbr_t;
    libmetis__wspacepush(ctrl);
    nvtxs = (*graph).nvtxs;
    ncon = (*graph).ncon;
    xadj = (*graph).xadj;
    vwgt = (*graph).vwgt;
    adjncy = (*graph).adjncy;
    bndptr = (*graph).bndptr;
    bndind = (*graph).bndind;
    where_0 = (*graph).where_0;
    pwgts = (*graph).pwgts;
    nparts = (*ctrl).nparts;
    pijbm = (*ctrl).pijbm;
    ubfactors = libmetis__rwspacemalloc(ctrl, ncon);
    libmetis__ComputeLoadImbalanceVec(graph, nparts, pijbm, ubfactors);
    origbal = libmetis__rvecmaxdiff(ncon, ubfactors, (*ctrl).ubfactors);
    if omode == 2 as libc::c_int {
        libmetis__rcopy(ncon as size_t, (*ctrl).ubfactors, ubfactors);
    } else {
        i = 0 as libc::c_int;
        while i < ncon {
            *ubfactors
                .offset(
                    i as isize,
                ) = if *ubfactors.offset(i as isize)
                > *((*ctrl).ubfactors).offset(i as isize)
            {
                *ubfactors.offset(i as isize)
            } else {
                *((*ctrl).ubfactors).offset(i as isize)
            };
            i += 1;
            i;
        }
    }
    minwgt = libmetis__iwspacemalloc(ctrl, nparts * ncon);
    maxwgt = libmetis__iwspacemalloc(ctrl, nparts * ncon);
    i = 0 as libc::c_int;
    while i < nparts {
        j = 0 as libc::c_int;
        while j < ncon {
            *maxwgt
                .offset(
                    (i * ncon + j) as isize,
                ) = (*((*ctrl).tpwgts).offset((i * ncon + j) as isize)
                * *((*graph).tvwgt).offset(j as isize) as libc::c_float
                * *ubfactors.offset(j as isize)) as idx_t;
            *minwgt
                .offset(
                    (i * ncon + j) as isize,
                ) = ((*((*ctrl).tpwgts).offset((i * ncon + j) as isize)
                * *((*graph).tvwgt).offset(j as isize) as libc::c_float)
                as libc::c_double * 0.2f64) as idx_t;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    perm = libmetis__iwspacemalloc(ctrl, nvtxs);
    safetos = libmetis__iset(
        nparts as size_t,
        2 as libc::c_int,
        libmetis__iwspacemalloc(ctrl, nparts),
    );
    if (*ctrl).minconn != 0 {
        libmetis__ComputeSubDomainGraph(ctrl, graph);
        nads = (*ctrl).nads;
        adids = (*ctrl).adids;
        adwgts = (*ctrl).adwgts;
        doms = libmetis__iset(nparts as size_t, 0 as libc::c_int, (*ctrl).pvec1);
    }
    vstatus = libmetis__iset(
        nvtxs as size_t,
        3 as libc::c_int,
        libmetis__iwspacemalloc(ctrl, nvtxs),
    );
    updptr = libmetis__iset(
        nvtxs as size_t,
        -(1 as libc::c_int),
        libmetis__iwspacemalloc(ctrl, nvtxs),
    );
    updind = libmetis__iwspacemalloc(ctrl, nvtxs);
    if (*ctrl).contig != 0 {
        bfslvl = libmetis__iset(
            nvtxs as size_t,
            0 as libc::c_int,
            libmetis__iwspacemalloc(ctrl, nvtxs),
        );
        bfsind = libmetis__iwspacemalloc(ctrl, nvtxs);
        bfsmrk = libmetis__iset(
            nvtxs as size_t,
            0 as libc::c_int,
            libmetis__iwspacemalloc(ctrl, nvtxs),
        );
    }
    modind = libmetis__iwspacemalloc(ctrl, nvtxs);
    vmarker = libmetis__iset(
        nvtxs as size_t,
        0 as libc::c_int,
        libmetis__iwspacemalloc(ctrl, nvtxs),
    );
    pmarker = libmetis__iset(
        nparts as size_t,
        -(1 as libc::c_int),
        libmetis__iwspacemalloc(ctrl, nparts),
    );
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_REFINE as libc::c_int as libc::c_uint
        != 0
    {
        printf(
            b"%s: [%6d %6d %6d], Bal: %5.3f(%.3f),, Nv-Nb[%6d %6d], Cut: %5d, Vol: %5d, (%d)\0"
                as *const u8 as *const libc::c_char,
            if omode == 1 as libc::c_int {
                b"GRV\0" as *const u8 as *const libc::c_char
            } else {
                b"GBV\0" as *const u8 as *const libc::c_char
            },
            libmetis__imin((nparts * ncon) as size_t, pwgts),
            libmetis__imax((nparts * ncon) as size_t, pwgts),
            libmetis__imax((nparts * ncon) as size_t, maxwgt),
            libmetis__ComputeLoadImbalance(graph, nparts, pijbm) as libc::c_double,
            origbal as libc::c_double,
            (*graph).nvtxs,
            (*graph).nbnd,
            (*graph).mincut,
            (*graph).minvol,
            niter,
        );
        if (*ctrl).minconn != 0 {
            printf(
                b", Doms: [%3d %4d]\0" as *const u8 as *const libc::c_char,
                libmetis__imax(nparts as size_t, nads),
                libmetis__isum(nparts as size_t, nads, 1 as libc::c_int as size_t),
            );
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
    }
    queue = libmetis__ipqCreate(nvtxs as size_t);
    pass = 0 as libc::c_int;
    while pass < niter {
        if omode == 2 as libc::c_int
            && libmetis__IsBalanced(ctrl, graph, 0 as libc::c_int as real_t) != 0
        {
            break;
        }
        oldcut = (*graph).mincut;
        oldvol = (*graph).minvol;
        nupd = 0 as libc::c_int;
        if (*ctrl).minconn != 0 {
            maxndoms = libmetis__imax(nparts as size_t, nads);
        }
        libmetis__irandArrayPermute(
            (*graph).nbnd,
            perm,
            (*graph).nbnd / 4 as libc::c_int,
            1 as libc::c_int,
        );
        ii = 0 as libc::c_int;
        while ii < (*graph).nbnd {
            i = *bndind.offset(*perm.offset(ii as isize) as isize);
            libmetis__ipqInsert(queue, i, (*((*graph).vkrinfo).offset(i as isize)).gv);
            *vstatus.offset(i as isize) = 1 as libc::c_int;
            *updind.offset(nupd as isize) = i;
            let fresh23 = nupd;
            nupd = nupd + 1;
            *updptr.offset(i as isize) = fresh23;
            ii += 1;
            ii;
        }
        let mut current_block_157: u64;
        nmoved = 0 as libc::c_int;
        iii = 0 as libc::c_int;
        loop {
            i = libmetis__ipqGetTop(queue);
            if i == -(1 as libc::c_int) {
                break;
            }
            *vstatus.offset(i as isize) = 2 as libc::c_int;
            myrinfo = ((*graph).vkrinfo).offset(i as isize);
            mynbrs = ((*ctrl).vnbrpool).offset((*myrinfo).inbr as isize);
            from = *where_0.offset(i as isize);
            if omode == 1 as libc::c_int {
                if (*myrinfo).nid > 0 as libc::c_int
                    && libmetis__ivecaxpygez(
                        ncon,
                        -(1 as libc::c_int),
                        vwgt.offset((i * ncon) as isize),
                        pwgts.offset((from * ncon) as isize),
                        minwgt.offset((from * ncon) as isize),
                    ) == 0
                {
                    current_block_157 = 6243635450180130569;
                } else {
                    current_block_157 = 17239133558811367971;
                }
            } else if libmetis__ivecaxpygez(
                ncon,
                -(1 as libc::c_int),
                vwgt.offset((i * ncon) as isize),
                pwgts.offset((from * ncon) as isize),
                minwgt.offset((from * ncon) as isize),
            ) == 0
            {
                current_block_157 = 6243635450180130569;
            } else {
                current_block_157 = 17239133558811367971;
            }
            match current_block_157 {
                17239133558811367971 => {
                    if !((*ctrl).contig != 0
                        && libmetis__IsArticulationNode(
                            i,
                            xadj,
                            adjncy,
                            where_0,
                            bfslvl,
                            bfsind,
                            bfsmrk,
                        ) != 0)
                    {
                        if (*ctrl).minconn != 0 {
                            let mut j_0: idx_t = 0;
                            let mut k_0: idx_t = 0;
                            let mut l_0: idx_t = 0;
                            let mut nadd: idx_t = 0;
                            let mut to_0: idx_t = 0;
                            j_0 = 0 as libc::c_int;
                            while j_0 < (*myrinfo).nnbrs {
                                to_0 = (*mynbrs.offset(j_0 as isize)).pid;
                                *safetos.offset(to_0 as isize) = 0 as libc::c_int;
                                k_0 = 0 as libc::c_int;
                                while k_0 < *nads.offset(to_0 as isize) {
                                    *doms
                                        .offset(
                                            *(*adids.offset(to_0 as isize)).offset(k_0 as isize)
                                                as isize,
                                        ) = 1 as libc::c_int;
                                    k_0 += 1;
                                    k_0;
                                }
                                nadd = 0 as libc::c_int;
                                k_0 = 0 as libc::c_int;
                                while k_0 < (*myrinfo).nnbrs {
                                    if !(k_0 == j_0) {
                                        l_0 = (*mynbrs.offset(k_0 as isize)).pid;
                                        if *doms.offset(l_0 as isize) == 0 as libc::c_int {
                                            if *nads.offset(l_0 as isize) > maxndoms - 1 as libc::c_int
                                            {
                                                nadd = maxndoms;
                                                break;
                                            } else {
                                                nadd += 1;
                                                nadd;
                                            }
                                        }
                                    }
                                    k_0 += 1;
                                    k_0;
                                }
                                if *nads.offset(to_0 as isize) + nadd <= maxndoms {
                                    *safetos.offset(to_0 as isize) = 1 as libc::c_int;
                                }
                                if nadd == 0 as libc::c_int {
                                    *safetos.offset(to_0 as isize) = 2 as libc::c_int;
                                }
                                k_0 = 0 as libc::c_int;
                                while k_0 < *nads.offset(to_0 as isize) {
                                    *doms
                                        .offset(
                                            *(*adids.offset(to_0 as isize)).offset(k_0 as isize)
                                                as isize,
                                        ) = 0 as libc::c_int;
                                    k_0 += 1;
                                    k_0;
                                }
                                j_0 += 1;
                                j_0;
                            }
                        }
                        xgain = if (*myrinfo).nid == 0 as libc::c_int
                            && (*myrinfo).ned > 0 as libc::c_int
                        {
                            *((*graph).vsize).offset(i as isize)
                        } else {
                            0 as libc::c_int
                        };
                        if omode == 1 as libc::c_int {
                            k = (*myrinfo).nnbrs - 1 as libc::c_int;
                            while k >= 0 as libc::c_int {
                                to = (*mynbrs.offset(k as isize)).pid;
                                if !(*safetos.offset(to as isize) == 0) {
                                    gain = (*mynbrs.offset(k as isize)).gv + xgain;
                                    if gain >= 0 as libc::c_int
                                        && libmetis__ivecaxpylez(
                                            ncon,
                                            1 as libc::c_int,
                                            vwgt.offset((i * ncon) as isize),
                                            pwgts.offset((to * ncon) as isize),
                                            maxwgt.offset((to * ncon) as isize),
                                        ) != 0
                                    {
                                        break;
                                    }
                                }
                                k -= 1;
                                k;
                            }
                            if k < 0 as libc::c_int {
                                current_block_157 = 6243635450180130569;
                            } else {
                                cto = to;
                                j = k - 1 as libc::c_int;
                                while j >= 0 as libc::c_int {
                                    to = (*mynbrs.offset(j as isize)).pid;
                                    if !(*safetos.offset(to as isize) == 0) {
                                        gain = (*mynbrs.offset(j as isize)).gv + xgain;
                                        if (*mynbrs.offset(j as isize)).gv
                                            > (*mynbrs.offset(k as isize)).gv
                                            && libmetis__ivecaxpylez(
                                                ncon,
                                                1 as libc::c_int,
                                                vwgt.offset((i * ncon) as isize),
                                                pwgts.offset((to * ncon) as isize),
                                                maxwgt.offset((to * ncon) as isize),
                                            ) != 0
                                            || (*mynbrs.offset(j as isize)).gv
                                                == (*mynbrs.offset(k as isize)).gv
                                                && (*mynbrs.offset(j as isize)).ned
                                                    > (*mynbrs.offset(k as isize)).ned
                                                && libmetis__ivecaxpylez(
                                                    ncon,
                                                    1 as libc::c_int,
                                                    vwgt.offset((i * ncon) as isize),
                                                    pwgts.offset((to * ncon) as isize),
                                                    maxwgt.offset((to * ncon) as isize),
                                                ) != 0
                                            || (*mynbrs.offset(j as isize)).gv
                                                == (*mynbrs.offset(k as isize)).gv
                                                && (*mynbrs.offset(j as isize)).ned
                                                    == (*mynbrs.offset(k as isize)).ned
                                                && libmetis__BetterBalanceKWay(
                                                    ncon,
                                                    vwgt.offset((i * ncon) as isize),
                                                    ubfactors,
                                                    1 as libc::c_int,
                                                    pwgts.offset((cto * ncon) as isize),
                                                    pijbm.offset((cto * ncon) as isize),
                                                    1 as libc::c_int,
                                                    pwgts.offset((to * ncon) as isize),
                                                    pijbm.offset((to * ncon) as isize),
                                                ) != 0
                                        {
                                            k = j;
                                            cto = to;
                                        }
                                    }
                                    j -= 1;
                                    j;
                                }
                                to = cto;
                                j = 0 as libc::c_int;
                                if xgain + (*mynbrs.offset(k as isize)).gv
                                    > 0 as libc::c_int
                                    || (*mynbrs.offset(k as isize)).ned - (*myrinfo).nid
                                        > 0 as libc::c_int
                                {
                                    j = 1 as libc::c_int;
                                } else if (*mynbrs.offset(k as isize)).ned - (*myrinfo).nid
                                    == 0 as libc::c_int
                                {
                                    if iii % 2 as libc::c_int == 0 as libc::c_int
                                        && *safetos.offset(to as isize) == 2 as libc::c_int
                                        || libmetis__BetterBalanceKWay(
                                            ncon,
                                            vwgt.offset((i * ncon) as isize),
                                            ubfactors,
                                            -(1 as libc::c_int),
                                            pwgts.offset((from * ncon) as isize),
                                            pijbm.offset((from * ncon) as isize),
                                            1 as libc::c_int,
                                            pwgts.offset((to * ncon) as isize),
                                            pijbm.offset((to * ncon) as isize),
                                        ) != 0
                                    {
                                        j = 1 as libc::c_int;
                                    }
                                }
                                if j == 0 as libc::c_int {
                                    current_block_157 = 6243635450180130569;
                                } else {
                                    current_block_157 = 13326139174796812312;
                                }
                            }
                        } else {
                            k = (*myrinfo).nnbrs - 1 as libc::c_int;
                            while k >= 0 as libc::c_int {
                                to = (*mynbrs.offset(k as isize)).pid;
                                if !(*safetos.offset(to as isize) == 0) {
                                    if libmetis__ivecaxpylez(
                                        ncon,
                                        1 as libc::c_int,
                                        vwgt.offset((i * ncon) as isize),
                                        pwgts.offset((to * ncon) as isize),
                                        maxwgt.offset((to * ncon) as isize),
                                    ) != 0
                                        || libmetis__BetterBalanceKWay(
                                            ncon,
                                            vwgt.offset((i * ncon) as isize),
                                            ubfactors,
                                            -(1 as libc::c_int),
                                            pwgts.offset((from * ncon) as isize),
                                            pijbm.offset((from * ncon) as isize),
                                            1 as libc::c_int,
                                            pwgts.offset((to * ncon) as isize),
                                            pijbm.offset((to * ncon) as isize),
                                        ) != 0
                                    {
                                        break;
                                    }
                                }
                                k -= 1;
                                k;
                            }
                            if k < 0 as libc::c_int {
                                current_block_157 = 6243635450180130569;
                            } else {
                                cto = to;
                                j = k - 1 as libc::c_int;
                                while j >= 0 as libc::c_int {
                                    to = (*mynbrs.offset(j as isize)).pid;
                                    if !(*safetos.offset(to as isize) == 0) {
                                        if libmetis__BetterBalanceKWay(
                                            ncon,
                                            vwgt.offset((i * ncon) as isize),
                                            ubfactors,
                                            1 as libc::c_int,
                                            pwgts.offset((cto * ncon) as isize),
                                            pijbm.offset((cto * ncon) as isize),
                                            1 as libc::c_int,
                                            pwgts.offset((to * ncon) as isize),
                                            pijbm.offset((to * ncon) as isize),
                                        ) != 0
                                        {
                                            k = j;
                                            cto = to;
                                        }
                                    }
                                    j -= 1;
                                    j;
                                }
                                to = cto;
                                if (xgain + (*mynbrs.offset(k as isize)).gv
                                    < 0 as libc::c_int
                                    || xgain + (*mynbrs.offset(k as isize)).gv
                                        == 0 as libc::c_int
                                        && (*mynbrs.offset(k as isize)).ned - (*myrinfo).nid
                                            < 0 as libc::c_int)
                                    && libmetis__BetterBalanceKWay(
                                        ncon,
                                        vwgt.offset((i * ncon) as isize),
                                        ubfactors,
                                        -(1 as libc::c_int),
                                        pwgts.offset((from * ncon) as isize),
                                        pijbm.offset((from * ncon) as isize),
                                        1 as libc::c_int,
                                        pwgts.offset((to * ncon) as isize),
                                        pijbm.offset((to * ncon) as isize),
                                    ) == 0
                                {
                                    current_block_157 = 6243635450180130569;
                                } else {
                                    current_block_157 = 13326139174796812312;
                                }
                            }
                        }
                        match current_block_157 {
                            6243635450180130569 => {}
                            _ => {
                                (*graph).mincut
                                    -= (*mynbrs.offset(k as isize)).ned - (*myrinfo).nid;
                                (*graph).minvol -= xgain + (*mynbrs.offset(k as isize)).gv;
                                *where_0.offset(i as isize) = to;
                                nmoved += 1;
                                nmoved;
                                if (*ctrl).dbglvl as libc::c_uint
                                    & METIS_DBG_MOVEINFO as libc::c_int as libc::c_uint != 0
                                {
                                    printf(
                                        b"\t\tMoving %6d from %3d to %3d. Gain: [%4d %4d]. Cut: %6d, Vol: %6d\n\0"
                                            as *const u8 as *const libc::c_char,
                                        i,
                                        from,
                                        to,
                                        xgain + (*mynbrs.offset(k as isize)).gv,
                                        (*mynbrs.offset(k as isize)).ned - (*myrinfo).nid,
                                        (*graph).mincut,
                                        (*graph).minvol,
                                    );
                                }
                                if (*ctrl).minconn != 0 {
                                    libmetis__UpdateEdgeSubDomainGraph(
                                        ctrl,
                                        from,
                                        to,
                                        (*myrinfo).nid - (*mynbrs.offset(k as isize)).ned,
                                        &mut maxndoms,
                                    );
                                    j = *xadj.offset(i as isize);
                                    while j < *xadj.offset((i + 1 as libc::c_int) as isize) {
                                        me = *where_0.offset(*adjncy.offset(j as isize) as isize);
                                        if me != from && me != to {
                                            libmetis__UpdateEdgeSubDomainGraph(
                                                ctrl,
                                                from,
                                                me,
                                                -(1 as libc::c_int),
                                                &mut maxndoms,
                                            );
                                            libmetis__UpdateEdgeSubDomainGraph(
                                                ctrl,
                                                to,
                                                me,
                                                1 as libc::c_int,
                                                &mut maxndoms,
                                            );
                                        }
                                        j += 1;
                                        j;
                                    }
                                }
                                libmetis__iaxpy(
                                    ncon as size_t,
                                    1 as libc::c_int,
                                    vwgt.offset((i * ncon) as isize),
                                    1 as libc::c_int as size_t,
                                    pwgts.offset((to * ncon) as isize),
                                    1 as libc::c_int as size_t,
                                );
                                libmetis__iaxpy(
                                    ncon as size_t,
                                    -(1 as libc::c_int),
                                    vwgt.offset((i * ncon) as isize),
                                    1 as libc::c_int as size_t,
                                    pwgts.offset((from * ncon) as isize),
                                    1 as libc::c_int as size_t,
                                );
                                libmetis__KWayVolUpdate(
                                    ctrl,
                                    graph,
                                    i,
                                    from,
                                    to,
                                    queue,
                                    vstatus,
                                    &mut nupd,
                                    updptr,
                                    updind,
                                    bndtype,
                                    vmarker,
                                    pmarker,
                                    modind,
                                );
                            }
                        }
                    }
                }
                _ => {}
            }
            iii += 1;
            iii;
        }
        i = 0 as libc::c_int;
        while i < nupd {
            *vstatus.offset(*updind.offset(i as isize) as isize) = 3 as libc::c_int;
            *updptr.offset(*updind.offset(i as isize) as isize) = -(1 as libc::c_int);
            i += 1;
            i;
        }
        if (*ctrl).dbglvl as libc::c_uint
            & METIS_DBG_REFINE as libc::c_int as libc::c_uint != 0
        {
            printf(
                b"\t[%6d %6d], Bal: %5.3f, Nb: %6d. Nmoves: %5d, Cut: %6d, Vol: %6d\0"
                    as *const u8 as *const libc::c_char,
                libmetis__imin((nparts * ncon) as size_t, pwgts),
                libmetis__imax((nparts * ncon) as size_t, pwgts),
                libmetis__ComputeLoadImbalance(graph, nparts, pijbm) as libc::c_double,
                (*graph).nbnd,
                nmoved,
                (*graph).mincut,
                (*graph).minvol,
            );
            if (*ctrl).minconn != 0 {
                printf(
                    b", Doms: [%3d %4d]\0" as *const u8 as *const libc::c_char,
                    libmetis__imax(nparts as size_t, nads),
                    libmetis__isum(nparts as size_t, nads, 1 as libc::c_int as size_t),
                );
            }
            printf(b"\n\0" as *const u8 as *const libc::c_char);
        }
        if nmoved == 0 as libc::c_int
            || omode == 1 as libc::c_int && (*graph).minvol == oldvol
                && (*graph).mincut == oldcut
        {
            break;
        }
        pass += 1;
        pass;
    }
    libmetis__ipqDestroy(queue);
    libmetis__wspacepop(ctrl);
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__IsArticulationNode(
    mut i: idx_t,
    mut xadj: *mut idx_t,
    mut adjncy: *mut idx_t,
    mut where_0: *mut idx_t,
    mut bfslvl: *mut idx_t,
    mut bfsind: *mut idx_t,
    mut bfsmrk: *mut idx_t,
) -> idx_t {
    let mut ii: idx_t = 0;
    let mut j: idx_t = 0;
    let mut k: idx_t = 0 as libc::c_int;
    let mut head: idx_t = 0;
    let mut tail: idx_t = 0;
    let mut nhits: idx_t = 0;
    let mut tnhits: idx_t = 0;
    let mut from: idx_t = 0;
    let mut BFSDEPTH: idx_t = 5 as libc::c_int;
    from = *where_0.offset(i as isize);
    tnhits = 0 as libc::c_int;
    j = *xadj.offset(i as isize);
    while j < *xadj.offset((i + 1 as libc::c_int) as isize) {
        if *where_0.offset(*adjncy.offset(j as isize) as isize) == from {
            k = *adjncy.offset(j as isize);
            *bfsmrk.offset(k as isize) = 1 as libc::c_int;
            tnhits += 1;
            tnhits;
        }
        j += 1;
        j;
    }
    if tnhits == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if tnhits == 1 as libc::c_int {
        *bfsmrk.offset(k as isize) = 0 as libc::c_int;
        return 0 as libc::c_int;
    }
    *bfslvl.offset(i as isize) = 1 as libc::c_int;
    *bfsind.offset(0 as libc::c_int as isize) = k;
    *bfslvl.offset(k as isize) = 1 as libc::c_int;
    *bfsmrk.offset(k as isize) = 0 as libc::c_int;
    head = 0 as libc::c_int;
    tail = 1 as libc::c_int;
    nhits = 1 as libc::c_int;
    while head < tail {
        let fresh24 = head;
        head = head + 1;
        ii = *bfsind.offset(fresh24 as isize);
        j = *xadj.offset(ii as isize);
        while j < *xadj.offset((ii + 1 as libc::c_int) as isize) {
            k = *adjncy.offset(j as isize);
            if *where_0.offset(k as isize) == from {
                if *bfsmrk.offset(k as isize) != 0 {
                    *bfsmrk.offset(k as isize) = 0 as libc::c_int;
                    nhits += 1;
                    if nhits == tnhits {
                        break;
                    }
                }
                if *bfslvl.offset(k as isize) == 0 as libc::c_int
                    && *bfslvl.offset(ii as isize) < BFSDEPTH
                {
                    let fresh25 = tail;
                    tail = tail + 1;
                    *bfsind.offset(fresh25 as isize) = k;
                    *bfslvl
                        .offset(
                            k as isize,
                        ) = *bfslvl.offset(ii as isize) + 1 as libc::c_int;
                }
            }
            j += 1;
            j;
        }
        if nhits == tnhits {
            break;
        }
    }
    *bfslvl.offset(i as isize) = 0 as libc::c_int;
    j = 0 as libc::c_int;
    while j < tail {
        *bfslvl.offset(*bfsind.offset(j as isize) as isize) = 0 as libc::c_int;
        j += 1;
        j;
    }
    if nhits < tnhits {
        j = *xadj.offset(i as isize);
        while j < *xadj.offset((i + 1 as libc::c_int) as isize) {
            if *where_0.offset(*adjncy.offset(j as isize) as isize) == from {
                *bfsmrk.offset(*adjncy.offset(j as isize) as isize) = 0 as libc::c_int;
            }
            j += 1;
            j;
        }
    }
    return (nhits != tnhits) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__KWayVolUpdate(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut v: idx_t,
    mut from: idx_t,
    mut to: idx_t,
    mut queue: *mut ipq_t,
    mut vstatus: *mut idx_t,
    mut r_nupd: *mut idx_t,
    mut updptr: *mut idx_t,
    mut updind: *mut idx_t,
    mut bndtype: idx_t,
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
    let mut kk: idx_t = 0;
    let mut l: idx_t = 0;
    let mut u: idx_t = 0;
    let mut nmod: idx_t = 0;
    let mut other: idx_t = 0;
    let mut me: idx_t = 0;
    let mut myidx: idx_t = 0;
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
    let mut vsize: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    let mut myrinfo: *mut vkrinfo_t = 0 as *mut vkrinfo_t;
    let mut orinfo: *mut vkrinfo_t = 0 as *mut vkrinfo_t;
    let mut mynbrs: *mut vnbr_t = 0 as *mut vnbr_t;
    let mut onbrs: *mut vnbr_t = 0 as *mut vnbr_t;
    xadj = (*graph).xadj;
    adjncy = (*graph).adjncy;
    vsize = (*graph).vsize;
    where_0 = (*graph).where_0;
    myrinfo = ((*graph).vkrinfo).offset(v as isize);
    mynbrs = ((*ctrl).vnbrpool).offset((*myrinfo).inbr as isize);
    k = 0 as libc::c_int;
    while k < (*myrinfo).nnbrs {
        *pmarker.offset((*mynbrs.offset(k as isize)).pid as isize) = k;
        k += 1;
        k;
    }
    *pmarker.offset(from as isize) = k;
    myidx = *pmarker.offset(to as isize);
    j = *xadj.offset(v as isize);
    while j < *xadj.offset((v + 1 as libc::c_int) as isize) {
        ii = *adjncy.offset(j as isize);
        other = *where_0.offset(ii as isize);
        orinfo = ((*graph).vkrinfo).offset(ii as isize);
        onbrs = ((*ctrl).vnbrpool).offset((*orinfo).inbr as isize);
        if other == from {
            k = 0 as libc::c_int;
            while k < (*orinfo).nnbrs {
                if *pmarker.offset((*onbrs.offset(k as isize)).pid as isize)
                    == -(1 as libc::c_int)
                {
                    let ref mut fresh26 = (*onbrs.offset(k as isize)).gv;
                    *fresh26 += *vsize.offset(v as isize);
                }
                k += 1;
                k;
            }
        } else if (*mynbrs.offset(*pmarker.offset(other as isize) as isize)).ned
            > 1 as libc::c_int
        {
            k = 0 as libc::c_int;
            while k < (*orinfo).nnbrs {
                if *pmarker.offset((*onbrs.offset(k as isize)).pid as isize)
                    == -(1 as libc::c_int)
                {
                    let ref mut fresh27 = (*onbrs.offset(k as isize)).gv;
                    *fresh27 += *vsize.offset(v as isize);
                }
                k += 1;
                k;
            }
        } else {
            k = 0 as libc::c_int;
            while k < (*orinfo).nnbrs {
                if *pmarker.offset((*onbrs.offset(k as isize)).pid as isize)
                    != -(1 as libc::c_int)
                {
                    let ref mut fresh28 = (*onbrs.offset(k as isize)).gv;
                    *fresh28 -= *vsize.offset(v as isize);
                }
                k += 1;
                k;
            }
        }
        j += 1;
        j;
    }
    k = 0 as libc::c_int;
    while k < (*myrinfo).nnbrs {
        *pmarker.offset((*mynbrs.offset(k as isize)).pid as isize) = -(1 as libc::c_int);
        k += 1;
        k;
    }
    *pmarker.offset(from as isize) = -(1 as libc::c_int);
    if myidx == -(1 as libc::c_int) {
        let fresh29 = (*myrinfo).nnbrs;
        (*myrinfo).nnbrs = (*myrinfo).nnbrs + 1;
        myidx = fresh29;
        (*mynbrs.offset(myidx as isize)).ned = 0 as libc::c_int;
    }
    (*myrinfo).ned += (*myrinfo).nid - (*mynbrs.offset(myidx as isize)).ned;
    j = (*myrinfo).nid;
    (*myrinfo).nid = (*mynbrs.offset(myidx as isize)).ned;
    (*mynbrs.offset(myidx as isize)).ned = j;
    if (*mynbrs.offset(myidx as isize)).ned == 0 as libc::c_int {
        (*myrinfo).nnbrs -= 1;
        *mynbrs.offset(myidx as isize) = *mynbrs.offset((*myrinfo).nnbrs as isize);
    } else {
        (*mynbrs.offset(myidx as isize)).pid = from;
    }
    *vmarker.offset(v as isize) = 1 as libc::c_int;
    *modind.offset(0 as libc::c_int as isize) = v;
    nmod = 1 as libc::c_int;
    j = *xadj.offset(v as isize);
    while j < *xadj.offset((v + 1 as libc::c_int) as isize) {
        ii = *adjncy.offset(j as isize);
        me = *where_0.offset(ii as isize);
        if *vmarker.offset(ii as isize) == 0 {
            *vmarker.offset(ii as isize) = 2 as libc::c_int;
            let fresh30 = nmod;
            nmod = nmod + 1;
            *modind.offset(fresh30 as isize) = ii;
        }
        myrinfo = ((*graph).vkrinfo).offset(ii as isize);
        if (*myrinfo).inbr == -(1 as libc::c_int) {
            (*myrinfo)
                .inbr = libmetis__vnbrpoolGetNext(
                ctrl,
                *xadj.offset((ii + 1 as libc::c_int) as isize)
                    - *xadj.offset(ii as isize) + 1 as libc::c_int,
            );
        }
        mynbrs = ((*ctrl).vnbrpool).offset((*myrinfo).inbr as isize);
        if me == from {
            (*myrinfo).ned += 1 as libc::c_int;
            (*myrinfo).nid -= 1 as libc::c_int;
        } else if me == to {
            (*myrinfo).nid += 1 as libc::c_int;
            (*myrinfo).ned -= 1 as libc::c_int;
        }
        if me != from {
            k = 0 as libc::c_int;
            while k < (*myrinfo).nnbrs {
                if (*mynbrs.offset(k as isize)).pid == from {
                    if (*mynbrs.offset(k as isize)).ned == 1 as libc::c_int {
                        (*myrinfo).nnbrs -= 1;
                        *mynbrs
                            .offset(
                                k as isize,
                            ) = *mynbrs.offset((*myrinfo).nnbrs as isize);
                        *vmarker.offset(ii as isize) = 1 as libc::c_int;
                        jj = *xadj.offset(ii as isize);
                        while jj < *xadj.offset((ii + 1 as libc::c_int) as isize) {
                            u = *adjncy.offset(jj as isize);
                            other = *where_0.offset(u as isize);
                            orinfo = ((*graph).vkrinfo).offset(u as isize);
                            onbrs = ((*ctrl).vnbrpool).offset((*orinfo).inbr as isize);
                            kk = 0 as libc::c_int;
                            while kk < (*orinfo).nnbrs {
                                if (*onbrs.offset(kk as isize)).pid == from {
                                    let ref mut fresh31 = (*onbrs.offset(kk as isize)).gv;
                                    *fresh31 -= *vsize.offset(ii as isize);
                                    if *vmarker.offset(u as isize) == 0 {
                                        *vmarker.offset(u as isize) = 2 as libc::c_int;
                                        let fresh32 = nmod;
                                        nmod = nmod + 1;
                                        *modind.offset(fresh32 as isize) = u;
                                    }
                                    break;
                                } else {
                                    kk += 1;
                                    kk;
                                }
                            }
                            jj += 1;
                            jj;
                        }
                    } else {
                        let ref mut fresh33 = (*mynbrs.offset(k as isize)).ned;
                        *fresh33 -= 1;
                        *fresh33;
                        if (*mynbrs.offset(k as isize)).ned == 1 as libc::c_int {
                            jj = *xadj.offset(ii as isize);
                            while jj < *xadj.offset((ii + 1 as libc::c_int) as isize) {
                                u = *adjncy.offset(jj as isize);
                                other = *where_0.offset(u as isize);
                                if other == from {
                                    orinfo = ((*graph).vkrinfo).offset(u as isize);
                                    onbrs = ((*ctrl).vnbrpool).offset((*orinfo).inbr as isize);
                                    kk = 0 as libc::c_int;
                                    while kk < (*orinfo).nnbrs {
                                        let ref mut fresh34 = (*onbrs.offset(kk as isize)).gv;
                                        *fresh34 += *vsize.offset(ii as isize);
                                        kk += 1;
                                        kk;
                                    }
                                    if *vmarker.offset(u as isize) == 0 {
                                        *vmarker.offset(u as isize) = 2 as libc::c_int;
                                        let fresh35 = nmod;
                                        nmod = nmod + 1;
                                        *modind.offset(fresh35 as isize) = u;
                                    }
                                    break;
                                } else {
                                    jj += 1;
                                    jj;
                                }
                            }
                        }
                    }
                    break;
                } else {
                    k += 1;
                    k;
                }
            }
        }
        if me != to {
            k = 0 as libc::c_int;
            while k < (*myrinfo).nnbrs {
                if (*mynbrs.offset(k as isize)).pid == to {
                    let ref mut fresh36 = (*mynbrs.offset(k as isize)).ned;
                    *fresh36 += 1;
                    *fresh36;
                    if (*mynbrs.offset(k as isize)).ned == 2 as libc::c_int {
                        jj = *xadj.offset(ii as isize);
                        while jj < *xadj.offset((ii + 1 as libc::c_int) as isize) {
                            u = *adjncy.offset(jj as isize);
                            other = *where_0.offset(u as isize);
                            if u != v && other == to {
                                orinfo = ((*graph).vkrinfo).offset(u as isize);
                                onbrs = ((*ctrl).vnbrpool).offset((*orinfo).inbr as isize);
                                kk = 0 as libc::c_int;
                                while kk < (*orinfo).nnbrs {
                                    let ref mut fresh37 = (*onbrs.offset(kk as isize)).gv;
                                    *fresh37 -= *vsize.offset(ii as isize);
                                    kk += 1;
                                    kk;
                                }
                                if *vmarker.offset(u as isize) == 0 {
                                    *vmarker.offset(u as isize) = 2 as libc::c_int;
                                    let fresh38 = nmod;
                                    nmod = nmod + 1;
                                    *modind.offset(fresh38 as isize) = u;
                                }
                                break;
                            } else {
                                jj += 1;
                                jj;
                            }
                        }
                    }
                    break;
                } else {
                    k += 1;
                    k;
                }
            }
            if k == (*myrinfo).nnbrs {
                (*mynbrs.offset((*myrinfo).nnbrs as isize)).pid = to;
                let fresh39 = (*myrinfo).nnbrs;
                (*myrinfo).nnbrs = (*myrinfo).nnbrs + 1;
                (*mynbrs.offset(fresh39 as isize)).ned = 1 as libc::c_int;
                *vmarker.offset(ii as isize) = 1 as libc::c_int;
                jj = *xadj.offset(ii as isize);
                while jj < *xadj.offset((ii + 1 as libc::c_int) as isize) {
                    u = *adjncy.offset(jj as isize);
                    other = *where_0.offset(u as isize);
                    orinfo = ((*graph).vkrinfo).offset(u as isize);
                    onbrs = ((*ctrl).vnbrpool).offset((*orinfo).inbr as isize);
                    kk = 0 as libc::c_int;
                    while kk < (*orinfo).nnbrs {
                        if (*onbrs.offset(kk as isize)).pid == to {
                            let ref mut fresh40 = (*onbrs.offset(kk as isize)).gv;
                            *fresh40 += *vsize.offset(ii as isize);
                            if *vmarker.offset(u as isize) == 0 {
                                *vmarker.offset(u as isize) = 2 as libc::c_int;
                                let fresh41 = nmod;
                                nmod = nmod + 1;
                                *modind.offset(fresh41 as isize) = u;
                            }
                            break;
                        } else {
                            kk += 1;
                            kk;
                        }
                    }
                    jj += 1;
                    jj;
                }
            }
        }
        j += 1;
        j;
    }
    myrinfo = ((*graph).vkrinfo).offset(v as isize);
    mynbrs = ((*ctrl).vnbrpool).offset((*myrinfo).inbr as isize);
    k = 0 as libc::c_int;
    while k < (*myrinfo).nnbrs {
        *pmarker.offset((*mynbrs.offset(k as isize)).pid as isize) = k;
        k += 1;
        k;
    }
    *pmarker.offset(to as isize) = k;
    j = *xadj.offset(v as isize);
    while j < *xadj.offset((v + 1 as libc::c_int) as isize) {
        ii = *adjncy.offset(j as isize);
        other = *where_0.offset(ii as isize);
        orinfo = ((*graph).vkrinfo).offset(ii as isize);
        onbrs = ((*ctrl).vnbrpool).offset((*orinfo).inbr as isize);
        if other == to {
            k = 0 as libc::c_int;
            while k < (*orinfo).nnbrs {
                if *pmarker.offset((*onbrs.offset(k as isize)).pid as isize)
                    == -(1 as libc::c_int)
                {
                    let ref mut fresh42 = (*onbrs.offset(k as isize)).gv;
                    *fresh42 -= *vsize.offset(v as isize);
                }
                k += 1;
                k;
            }
        } else if (*mynbrs.offset(*pmarker.offset(other as isize) as isize)).ned
            > 1 as libc::c_int
        {
            k = 0 as libc::c_int;
            while k < (*orinfo).nnbrs {
                if *pmarker.offset((*onbrs.offset(k as isize)).pid as isize)
                    == -(1 as libc::c_int)
                {
                    let ref mut fresh43 = (*onbrs.offset(k as isize)).gv;
                    *fresh43 -= *vsize.offset(v as isize);
                }
                k += 1;
                k;
            }
        } else {
            k = 0 as libc::c_int;
            while k < (*orinfo).nnbrs {
                if *pmarker.offset((*onbrs.offset(k as isize)).pid as isize)
                    != -(1 as libc::c_int)
                {
                    let ref mut fresh44 = (*onbrs.offset(k as isize)).gv;
                    *fresh44 += *vsize.offset(v as isize);
                }
                k += 1;
                k;
            }
        }
        j += 1;
        j;
    }
    k = 0 as libc::c_int;
    while k < (*myrinfo).nnbrs {
        *pmarker.offset((*mynbrs.offset(k as isize)).pid as isize) = -(1 as libc::c_int);
        k += 1;
        k;
    }
    *pmarker.offset(to as isize) = -(1 as libc::c_int);
    iii = 0 as libc::c_int;
    while iii < nmod {
        i = *modind.offset(iii as isize);
        me = *where_0.offset(i as isize);
        myrinfo = ((*graph).vkrinfo).offset(i as isize);
        mynbrs = ((*ctrl).vnbrpool).offset((*myrinfo).inbr as isize);
        if *vmarker.offset(i as isize) == 1 as libc::c_int {
            k = 0 as libc::c_int;
            while k < (*myrinfo).nnbrs {
                (*mynbrs.offset(k as isize)).gv = 0 as libc::c_int;
                k += 1;
                k;
            }
            j = *xadj.offset(i as isize);
            while j < *xadj.offset((i + 1 as libc::c_int) as isize) {
                ii = *adjncy.offset(j as isize);
                other = *where_0.offset(ii as isize);
                orinfo = ((*graph).vkrinfo).offset(ii as isize);
                onbrs = ((*ctrl).vnbrpool).offset((*orinfo).inbr as isize);
                kk = 0 as libc::c_int;
                while kk < (*orinfo).nnbrs {
                    *pmarker.offset((*onbrs.offset(kk as isize)).pid as isize) = kk;
                    kk += 1;
                    kk;
                }
                *pmarker.offset(other as isize) = 1 as libc::c_int;
                if me == other {
                    k = 0 as libc::c_int;
                    while k < (*myrinfo).nnbrs {
                        if *pmarker.offset((*mynbrs.offset(k as isize)).pid as isize)
                            == -(1 as libc::c_int)
                        {
                            let ref mut fresh45 = (*mynbrs.offset(k as isize)).gv;
                            *fresh45 -= *vsize.offset(ii as isize);
                        }
                        k += 1;
                        k;
                    }
                } else if (*onbrs.offset(*pmarker.offset(me as isize) as isize)).ned
                    == 1 as libc::c_int
                {
                    k = 0 as libc::c_int;
                    while k < (*myrinfo).nnbrs {
                        if *pmarker.offset((*mynbrs.offset(k as isize)).pid as isize)
                            != -(1 as libc::c_int)
                        {
                            let ref mut fresh46 = (*mynbrs.offset(k as isize)).gv;
                            *fresh46 += *vsize.offset(ii as isize);
                        }
                        k += 1;
                        k;
                    }
                } else {
                    k = 0 as libc::c_int;
                    while k < (*myrinfo).nnbrs {
                        if *pmarker.offset((*mynbrs.offset(k as isize)).pid as isize)
                            == -(1 as libc::c_int)
                        {
                            let ref mut fresh47 = (*mynbrs.offset(k as isize)).gv;
                            *fresh47 -= *vsize.offset(ii as isize);
                        }
                        k += 1;
                        k;
                    }
                }
                kk = 0 as libc::c_int;
                while kk < (*orinfo).nnbrs {
                    *pmarker
                        .offset(
                            (*onbrs.offset(kk as isize)).pid as isize,
                        ) = -(1 as libc::c_int);
                    kk += 1;
                    kk;
                }
                *pmarker.offset(other as isize) = -(1 as libc::c_int);
                j += 1;
                j;
            }
        }
        (*myrinfo).gv = -(2147483647 as libc::c_int) - 1 as libc::c_int;
        k = 0 as libc::c_int;
        while k < (*myrinfo).nnbrs {
            if (*mynbrs.offset(k as isize)).gv > (*myrinfo).gv {
                (*myrinfo).gv = (*mynbrs.offset(k as isize)).gv;
            }
            k += 1;
            k;
        }
        if (*myrinfo).ned > 0 as libc::c_int && (*myrinfo).nid == 0 as libc::c_int {
            (*myrinfo).gv += *vsize.offset(i as isize);
        }
        if bndtype == 1 as libc::c_int {
            if (*myrinfo).gv >= 0 as libc::c_int
                && *((*graph).bndptr).offset(i as isize) == -(1 as libc::c_int)
            {
                *((*graph).bndind).offset((*graph).nbnd as isize) = i;
                let fresh48 = (*graph).nbnd;
                (*graph).nbnd = (*graph).nbnd + 1;
                *((*graph).bndptr).offset(i as isize) = fresh48;
            }
            if (*myrinfo).gv < 0 as libc::c_int
                && *((*graph).bndptr).offset(i as isize) != -(1 as libc::c_int)
            {
                (*graph).nbnd -= 1;
                *((*graph).bndind)
                    .offset(
                        *((*graph).bndptr).offset(i as isize) as isize,
                    ) = *((*graph).bndind).offset((*graph).nbnd as isize);
                *((*graph).bndptr)
                    .offset(
                        *((*graph).bndind).offset((*graph).nbnd as isize) as isize,
                    ) = *((*graph).bndptr).offset(i as isize);
                *((*graph).bndptr).offset(i as isize) = -(1 as libc::c_int);
            }
        } else {
            if (*myrinfo).ned > 0 as libc::c_int
                && *((*graph).bndptr).offset(i as isize) == -(1 as libc::c_int)
            {
                *((*graph).bndind).offset((*graph).nbnd as isize) = i;
                let fresh49 = (*graph).nbnd;
                (*graph).nbnd = (*graph).nbnd + 1;
                *((*graph).bndptr).offset(i as isize) = fresh49;
            }
            if (*myrinfo).ned == 0 as libc::c_int
                && *((*graph).bndptr).offset(i as isize) != -(1 as libc::c_int)
            {
                (*graph).nbnd -= 1;
                *((*graph).bndind)
                    .offset(
                        *((*graph).bndptr).offset(i as isize) as isize,
                    ) = *((*graph).bndind).offset((*graph).nbnd as isize);
                *((*graph).bndptr)
                    .offset(
                        *((*graph).bndind).offset((*graph).nbnd as isize) as isize,
                    ) = *((*graph).bndptr).offset(i as isize);
                *((*graph).bndptr).offset(i as isize) = -(1 as libc::c_int);
            }
        }
        if !queue.is_null() {
            if *vstatus.offset(i as isize) != 2 as libc::c_int {
                if *((*graph).bndptr).offset(i as isize) != -(1 as libc::c_int) {
                    if *vstatus.offset(i as isize) == 1 as libc::c_int {
                        libmetis__ipqUpdate(queue, i, (*myrinfo).gv);
                    } else {
                        libmetis__ipqInsert(queue, i, (*myrinfo).gv);
                        *vstatus.offset(i as isize) = 1 as libc::c_int;
                        *updind.offset(*r_nupd as isize) = i;
                        let fresh50 = *r_nupd;
                        *r_nupd = *r_nupd + 1;
                        *updptr.offset(i as isize) = fresh50;
                    }
                } else if *vstatus.offset(i as isize) == 1 as libc::c_int {
                    libmetis__ipqDelete(queue, i);
                    *vstatus.offset(i as isize) = 3 as libc::c_int;
                    *r_nupd -= 1;
                    *updind
                        .offset(
                            *updptr.offset(i as isize) as isize,
                        ) = *updind.offset(*r_nupd as isize);
                    *updptr
                        .offset(
                            *updind.offset(*r_nupd as isize) as isize,
                        ) = *updptr.offset(i as isize);
                    *updptr.offset(i as isize) = -(1 as libc::c_int);
                }
            }
        }
        *vmarker.offset(i as isize) = 0 as libc::c_int;
        iii += 1;
        iii;
    }
}
