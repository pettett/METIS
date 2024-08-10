use ::libc;
extern "C" {
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn libmetis__EliminateSubDomainEdges(ctrl: *mut ctrl_t, graph: *mut graph_t);
    fn libmetis__cnbrpoolReset(ctrl: *mut ctrl_t);
    fn libmetis__cnbrpoolGetNext(ctrl: *mut ctrl_t, nnbrs: idx_t) -> idx_t;
    fn libmetis__vnbrpoolReset(ctrl: *mut ctrl_t);
    fn libmetis__vnbrpoolGetNext(ctrl: *mut ctrl_t, nnbrs: idx_t) -> idx_t;
    fn libmetis__wspacepush(ctrl: *mut ctrl_t);
    fn libmetis__iwspacemalloc(_: *mut ctrl_t, _: idx_t) -> *mut idx_t;
    fn libmetis__wspacepop(ctrl: *mut ctrl_t);
    fn libmetis__ComputeLoadImbalanceDiff(
        graph: *mut graph_t,
        nparts: idx_t,
        pijbm: *mut real_t,
        ubvec: *mut real_t,
    ) -> real_t;
    fn libmetis__Greedy_KWayOptimize(
        ctrl: *mut ctrl_t,
        graph: *mut graph_t,
        niter: idx_t,
        ffactor: real_t,
        omode: idx_t,
    );
    fn libmetis__FreeGraph(graph: *mut *mut graph_t);
    fn libmetis__EliminateComponents(ctrl: *mut ctrl_t, graph: *mut graph_t);
    fn libmetis__FindPartitionInducedComponents(
        graph: *mut graph_t,
        where_0: *mut idx_t,
        cptr: *mut idx_t,
        cind: *mut idx_t,
    ) -> idx_t;
    fn gk_malloc(nbytes: size_t, msg: *mut libc::c_char) -> *mut libc::c_void;
    fn gk_errexit(signum: libc::c_int, _: *mut libc::c_char, _: ...);
    fn gk_CPUSeconds() -> libc::c_double;
    fn libmetis__imalloc(n: size_t, msg: *mut libc::c_char) -> *mut idx_t;
    fn libmetis__iset(n: size_t, val: idx_t, x: *mut idx_t) -> *mut idx_t;
    fn libmetis__icopy(n: size_t, a: *mut idx_t, b: *mut idx_t) -> *mut idx_t;
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
pub unsafe extern "C" fn libmetis__RefineKWay(
    mut ctrl: *mut ctrl_t,
    mut orggraph: *mut graph_t,
    mut graph: *mut graph_t,
) {
    let mut i: idx_t = 0;
    let mut nlevels: idx_t = 0;
    let mut contig: idx_t = (*ctrl).contig;
    let mut ptr: *mut graph_t = 0 as *mut graph_t;
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint
        != 0
    {
        (*ctrl).UncoarsenTmr -= gk_CPUSeconds();
    }
    ptr = graph;
    nlevels = 0 as libc::c_int;
    while ptr != orggraph {
        ptr = (*ptr).finer;
        nlevels += 1;
        nlevels;
    }
    libmetis__ComputeKWayPartitionParams(ctrl, graph);
    if (*ctrl).minconn != 0 {
        libmetis__EliminateSubDomainEdges(ctrl, graph);
    }
    if contig != 0
        && libmetis__FindPartitionInducedComponents(
            graph,
            (*graph).where_0,
            0 as *mut idx_t,
            0 as *mut idx_t,
        ) > (*ctrl).nparts
    {
        libmetis__EliminateComponents(ctrl, graph);
        libmetis__ComputeKWayBoundary(ctrl, graph, 2 as libc::c_int);
        libmetis__Greedy_KWayOptimize(
            ctrl,
            graph,
            5 as libc::c_int,
            0 as libc::c_int as real_t,
            2 as libc::c_int,
        );
        libmetis__ComputeKWayBoundary(ctrl, graph, 1 as libc::c_int);
        libmetis__Greedy_KWayOptimize(
            ctrl,
            graph,
            (*ctrl).niter,
            0 as libc::c_int as real_t,
            1 as libc::c_int,
        );
        (*ctrl).contig = 0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    loop {
        if (*ctrl).minconn != 0 && i == nlevels / 2 as libc::c_int {
            libmetis__EliminateSubDomainEdges(ctrl, graph);
        }
        if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint
            != 0
        {
            (*ctrl).RefTmr -= gk_CPUSeconds();
        }
        if 2 as libc::c_int * i >= nlevels
            && libmetis__IsBalanced(ctrl, graph, 0.02f64 as real_t) == 0
        {
            libmetis__ComputeKWayBoundary(ctrl, graph, 2 as libc::c_int);
            libmetis__Greedy_KWayOptimize(
                ctrl,
                graph,
                1 as libc::c_int,
                0 as libc::c_int as real_t,
                2 as libc::c_int,
            );
            libmetis__ComputeKWayBoundary(ctrl, graph, 1 as libc::c_int);
        }
        libmetis__Greedy_KWayOptimize(
            ctrl,
            graph,
            (*ctrl).niter,
            5.0f64 as real_t,
            1 as libc::c_int,
        );
        if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint
            != 0
        {
            (*ctrl).RefTmr += gk_CPUSeconds();
        }
        if contig != 0 && i == nlevels / 2 as libc::c_int {
            if libmetis__FindPartitionInducedComponents(
                graph,
                (*graph).where_0,
                0 as *mut idx_t,
                0 as *mut idx_t,
            ) > (*ctrl).nparts
            {
                libmetis__EliminateComponents(ctrl, graph);
                if libmetis__IsBalanced(ctrl, graph, 0.02f64 as real_t) == 0 {
                    (*ctrl).contig = 1 as libc::c_int;
                    libmetis__ComputeKWayBoundary(ctrl, graph, 2 as libc::c_int);
                    libmetis__Greedy_KWayOptimize(
                        ctrl,
                        graph,
                        5 as libc::c_int,
                        0 as libc::c_int as real_t,
                        2 as libc::c_int,
                    );
                    libmetis__ComputeKWayBoundary(ctrl, graph, 1 as libc::c_int);
                    libmetis__Greedy_KWayOptimize(
                        ctrl,
                        graph,
                        (*ctrl).niter,
                        0 as libc::c_int as real_t,
                        1 as libc::c_int,
                    );
                    (*ctrl).contig = 0 as libc::c_int;
                }
            }
        }
        if graph == orggraph {
            break;
        }
        graph = (*graph).finer;
        if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint
            != 0
        {
            (*ctrl).ProjectTmr -= gk_CPUSeconds();
        }
        libmetis__ProjectKWayPartition(ctrl, graph);
        if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint
            != 0
        {
            (*ctrl).ProjectTmr += gk_CPUSeconds();
        }
        i += 1;
        i;
    }
    (*ctrl).contig = contig;
    if contig != 0
        && libmetis__FindPartitionInducedComponents(
            graph,
            (*graph).where_0,
            0 as *mut idx_t,
            0 as *mut idx_t,
        ) > (*ctrl).nparts
    {
        libmetis__EliminateComponents(ctrl, graph);
    }
    if libmetis__IsBalanced(ctrl, graph, 0.0f64 as real_t) == 0 {
        libmetis__ComputeKWayBoundary(ctrl, graph, 2 as libc::c_int);
        libmetis__Greedy_KWayOptimize(
            ctrl,
            graph,
            10 as libc::c_int,
            0 as libc::c_int as real_t,
            2 as libc::c_int,
        );
        libmetis__ComputeKWayBoundary(ctrl, graph, 1 as libc::c_int);
        libmetis__Greedy_KWayOptimize(
            ctrl,
            graph,
            (*ctrl).niter,
            0 as libc::c_int as real_t,
            1 as libc::c_int,
        );
    }
    (*ctrl).contig != 0;
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint
        != 0
    {
        (*ctrl).UncoarsenTmr += gk_CPUSeconds();
    }
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__AllocateKWayPartitionMemory(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
) {
    (*graph)
        .pwgts = libmetis__imalloc(
        ((*ctrl).nparts * (*graph).ncon) as size_t,
        b"AllocateKWayPartitionMemory: pwgts\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*graph)
        .where_0 = libmetis__imalloc(
        (*graph).nvtxs as size_t,
        b"AllocateKWayPartitionMemory: where\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*graph)
        .bndptr = libmetis__imalloc(
        (*graph).nvtxs as size_t,
        b"AllocateKWayPartitionMemory: bndptr\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*graph)
        .bndind = libmetis__imalloc(
        (*graph).nvtxs as size_t,
        b"AllocateKWayPartitionMemory: bndind\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    match (*ctrl).objtype as libc::c_uint {
        0 => {
            (*graph)
                .ckrinfo = gk_malloc(
                ((*graph).nvtxs as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<ckrinfo_t>() as libc::c_ulong),
                b"AllocateKWayPartitionMemory: ckrinfo\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            ) as *mut ckrinfo_t;
        }
        1 => {
            (*graph)
                .vkrinfo = gk_malloc(
                ((*graph).nvtxs as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<vkrinfo_t>() as libc::c_ulong),
                b"AllocateKWayVolPartitionMemory: vkrinfo\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            ) as *mut vkrinfo_t;
            (*graph).ckrinfo = (*graph).vkrinfo as *mut ckrinfo_t;
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
pub unsafe extern "C" fn libmetis__ComputeKWayPartitionParams(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
) {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut k: idx_t = 0;
    let mut l: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut ncon: idx_t = 0;
    let mut nparts: idx_t = 0;
    let mut nbnd: idx_t = 0;
    let mut mincut: idx_t = 0;
    let mut me: idx_t = 0;
    let mut other: idx_t = 0;
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
    let mut vwgt: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut adjwgt: *mut idx_t = 0 as *mut idx_t;
    let mut pwgts: *mut idx_t = 0 as *mut idx_t;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    let mut bndind: *mut idx_t = 0 as *mut idx_t;
    let mut bndptr: *mut idx_t = 0 as *mut idx_t;
    nparts = (*ctrl).nparts;
    nvtxs = (*graph).nvtxs;
    ncon = (*graph).ncon;
    xadj = (*graph).xadj;
    vwgt = (*graph).vwgt;
    adjncy = (*graph).adjncy;
    adjwgt = (*graph).adjwgt;
    where_0 = (*graph).where_0;
    pwgts = libmetis__iset((nparts * ncon) as size_t, 0 as libc::c_int, (*graph).pwgts);
    bndind = (*graph).bndind;
    bndptr = libmetis__iset(nvtxs as size_t, -(1 as libc::c_int), (*graph).bndptr);
    mincut = 0 as libc::c_int;
    nbnd = mincut;
    if ncon == 1 as libc::c_int {
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
    match (*ctrl).objtype as libc::c_uint {
        0 => {
            let mut myrinfo: *mut ckrinfo_t = 0 as *mut ckrinfo_t;
            let mut mynbrs: *mut cnbr_t = 0 as *mut cnbr_t;
            memset(
                (*graph).ckrinfo as *mut libc::c_void,
                0 as libc::c_int,
                (::core::mem::size_of::<ckrinfo_t>() as libc::c_ulong)
                    .wrapping_mul(nvtxs as libc::c_ulong),
            );
            libmetis__cnbrpoolReset(ctrl);
            i = 0 as libc::c_int;
            while i < nvtxs {
                me = *where_0.offset(i as isize);
                myrinfo = ((*graph).ckrinfo).offset(i as isize);
                j = *xadj.offset(i as isize);
                while j < *xadj.offset((i + 1 as libc::c_int) as isize) {
                    if me == *where_0.offset(*adjncy.offset(j as isize) as isize) {
                        (*myrinfo).id += *adjwgt.offset(j as isize);
                    } else {
                        (*myrinfo).ed += *adjwgt.offset(j as isize);
                    }
                    j += 1;
                    j;
                }
                if (*myrinfo).ed > 0 as libc::c_int {
                    mincut += (*myrinfo).ed;
                    (*myrinfo)
                        .inbr = libmetis__cnbrpoolGetNext(
                        ctrl,
                        *xadj.offset((i + 1 as libc::c_int) as isize)
                            - *xadj.offset(i as isize) + 1 as libc::c_int,
                    );
                    mynbrs = ((*ctrl).cnbrpool).offset((*myrinfo).inbr as isize);
                    j = *xadj.offset(i as isize);
                    while j < *xadj.offset((i + 1 as libc::c_int) as isize) {
                        other = *where_0.offset(*adjncy.offset(j as isize) as isize);
                        if me != other {
                            k = 0 as libc::c_int;
                            while k < (*myrinfo).nnbrs {
                                if (*mynbrs.offset(k as isize)).pid == other {
                                    let ref mut fresh2 = (*mynbrs.offset(k as isize)).ed;
                                    *fresh2 += *adjwgt.offset(j as isize);
                                    break;
                                } else {
                                    k += 1;
                                    k;
                                }
                            }
                            if k == (*myrinfo).nnbrs {
                                (*mynbrs.offset(k as isize)).pid = other;
                                (*mynbrs.offset(k as isize))
                                    .ed = *adjwgt.offset(j as isize);
                                (*myrinfo).nnbrs += 1;
                                (*myrinfo).nnbrs;
                            }
                        }
                        j += 1;
                        j;
                    }
                    if (*myrinfo).ed - (*myrinfo).id >= 0 as libc::c_int {
                        *bndind.offset(nbnd as isize) = i;
                        let fresh3 = nbnd;
                        nbnd = nbnd + 1;
                        *bndptr.offset(i as isize) = fresh3;
                    }
                } else {
                    (*myrinfo).inbr = -(1 as libc::c_int);
                }
                i += 1;
                i;
            }
            (*graph).mincut = mincut / 2 as libc::c_int;
            (*graph).nbnd = nbnd;
        }
        1 => {
            let mut myrinfo_0: *mut vkrinfo_t = 0 as *mut vkrinfo_t;
            let mut mynbrs_0: *mut vnbr_t = 0 as *mut vnbr_t;
            memset(
                (*graph).vkrinfo as *mut libc::c_void,
                0 as libc::c_int,
                (::core::mem::size_of::<vkrinfo_t>() as libc::c_ulong)
                    .wrapping_mul(nvtxs as libc::c_ulong),
            );
            libmetis__vnbrpoolReset(ctrl);
            i = 0 as libc::c_int;
            while i < nvtxs {
                me = *where_0.offset(i as isize);
                myrinfo_0 = ((*graph).vkrinfo).offset(i as isize);
                j = *xadj.offset(i as isize);
                while j < *xadj.offset((i + 1 as libc::c_int) as isize) {
                    if me == *where_0.offset(*adjncy.offset(j as isize) as isize) {
                        (*myrinfo_0).nid += 1;
                        (*myrinfo_0).nid;
                    } else {
                        (*myrinfo_0).ned += 1;
                        (*myrinfo_0).ned;
                    }
                    j += 1;
                    j;
                }
                if (*myrinfo_0).ned > 0 as libc::c_int {
                    mincut += (*myrinfo_0).ned;
                    (*myrinfo_0)
                        .inbr = libmetis__vnbrpoolGetNext(
                        ctrl,
                        *xadj.offset((i + 1 as libc::c_int) as isize)
                            - *xadj.offset(i as isize) + 1 as libc::c_int,
                    );
                    mynbrs_0 = ((*ctrl).vnbrpool).offset((*myrinfo_0).inbr as isize);
                    j = *xadj.offset(i as isize);
                    while j < *xadj.offset((i + 1 as libc::c_int) as isize) {
                        other = *where_0.offset(*adjncy.offset(j as isize) as isize);
                        if me != other {
                            k = 0 as libc::c_int;
                            while k < (*myrinfo_0).nnbrs {
                                if (*mynbrs_0.offset(k as isize)).pid == other {
                                    let ref mut fresh4 = (*mynbrs_0.offset(k as isize)).ned;
                                    *fresh4 += 1;
                                    *fresh4;
                                    break;
                                } else {
                                    k += 1;
                                    k;
                                }
                            }
                            if k == (*myrinfo_0).nnbrs {
                                (*mynbrs_0.offset(k as isize)).gv = 0 as libc::c_int;
                                (*mynbrs_0.offset(k as isize)).pid = other;
                                (*mynbrs_0.offset(k as isize)).ned = 1 as libc::c_int;
                                (*myrinfo_0).nnbrs += 1;
                                (*myrinfo_0).nnbrs;
                            }
                        }
                        j += 1;
                        j;
                    }
                } else {
                    (*myrinfo_0).inbr = -(1 as libc::c_int);
                }
                i += 1;
                i;
            }
            (*graph).mincut = mincut / 2 as libc::c_int;
            libmetis__ComputeKWayVolGains(ctrl, graph);
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
pub unsafe extern "C" fn libmetis__ProjectKWayPartition(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
) {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut k: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut nbnd: idx_t = 0;
    let mut nparts: idx_t = 0;
    let mut me: idx_t = 0;
    let mut other: idx_t = 0;
    let mut istart: idx_t = 0;
    let mut iend: idx_t = 0;
    let mut tid: idx_t = 0;
    let mut ted: idx_t = 0;
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut adjwgt: *mut idx_t = 0 as *mut idx_t;
    let mut cmap: *mut idx_t = 0 as *mut idx_t;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    let mut bndptr: *mut idx_t = 0 as *mut idx_t;
    let mut bndind: *mut idx_t = 0 as *mut idx_t;
    let mut cwhere: *mut idx_t = 0 as *mut idx_t;
    let mut htable: *mut idx_t = 0 as *mut idx_t;
    let mut cgraph: *mut graph_t = 0 as *mut graph_t;
    libmetis__wspacepush(ctrl);
    nparts = (*ctrl).nparts;
    cgraph = (*graph).coarser;
    cwhere = (*cgraph).where_0;
    nvtxs = (*graph).nvtxs;
    cmap = (*graph).cmap;
    xadj = (*graph).xadj;
    adjncy = (*graph).adjncy;
    adjwgt = (*graph).adjwgt;
    libmetis__AllocateKWayPartitionMemory(ctrl, graph);
    where_0 = (*graph).where_0;
    bndind = (*graph).bndind;
    bndptr = libmetis__iset(nvtxs as size_t, -(1 as libc::c_int), (*graph).bndptr);
    htable = libmetis__iset(
        nparts as size_t,
        -(1 as libc::c_int),
        libmetis__iwspacemalloc(ctrl, nparts),
    );
    match (*ctrl).objtype as libc::c_uint {
        0 => {
            let mut myrinfo: *mut ckrinfo_t = 0 as *mut ckrinfo_t;
            let mut mynbrs: *mut cnbr_t = 0 as *mut cnbr_t;
            i = 0 as libc::c_int;
            while i < nvtxs {
                k = *cmap.offset(i as isize);
                *where_0.offset(i as isize) = *cwhere.offset(k as isize);
                *cmap.offset(i as isize) = (*((*cgraph).ckrinfo).offset(k as isize)).ed;
                i += 1;
                i;
            }
            memset(
                (*graph).ckrinfo as *mut libc::c_void,
                0 as libc::c_int,
                (::core::mem::size_of::<ckrinfo_t>() as libc::c_ulong)
                    .wrapping_mul(nvtxs as libc::c_ulong),
            );
            libmetis__cnbrpoolReset(ctrl);
            nbnd = 0 as libc::c_int;
            i = 0 as libc::c_int;
            while i < nvtxs {
                istart = *xadj.offset(i as isize);
                iend = *xadj.offset((i + 1 as libc::c_int) as isize);
                myrinfo = ((*graph).ckrinfo).offset(i as isize);
                if *cmap.offset(i as isize) == 0 as libc::c_int {
                    tid = 0 as libc::c_int;
                    j = istart;
                    while j < iend {
                        tid += *adjwgt.offset(j as isize);
                        j += 1;
                        j;
                    }
                    (*myrinfo).id = tid;
                    (*myrinfo).inbr = -(1 as libc::c_int);
                } else {
                    (*myrinfo)
                        .inbr = libmetis__cnbrpoolGetNext(
                        ctrl,
                        iend - istart + 1 as libc::c_int,
                    );
                    mynbrs = ((*ctrl).cnbrpool).offset((*myrinfo).inbr as isize);
                    me = *where_0.offset(i as isize);
                    tid = 0 as libc::c_int;
                    ted = 0 as libc::c_int;
                    j = istart;
                    while j < iend {
                        other = *where_0.offset(*adjncy.offset(j as isize) as isize);
                        if me == other {
                            tid += *adjwgt.offset(j as isize);
                        } else {
                            ted += *adjwgt.offset(j as isize);
                            k = *htable.offset(other as isize);
                            if k == -(1 as libc::c_int) {
                                *htable.offset(other as isize) = (*myrinfo).nnbrs;
                                (*mynbrs.offset((*myrinfo).nnbrs as isize)).pid = other;
                                let fresh5 = (*myrinfo).nnbrs;
                                (*myrinfo).nnbrs = (*myrinfo).nnbrs + 1;
                                (*mynbrs.offset(fresh5 as isize))
                                    .ed = *adjwgt.offset(j as isize);
                            } else {
                                let ref mut fresh6 = (*mynbrs.offset(k as isize)).ed;
                                *fresh6 += *adjwgt.offset(j as isize);
                            }
                        }
                        j += 1;
                        j;
                    }
                    (*myrinfo).id = tid;
                    (*myrinfo).ed = ted;
                    if ted == 0 as libc::c_int {
                        (*ctrl)
                            .nbrpoolcpos = ((*ctrl).nbrpoolcpos as libc::c_ulong)
                            .wrapping_sub(
                                (iend - istart + 1 as libc::c_int) as libc::c_ulong,
                            ) as size_t as size_t;
                        (*myrinfo).inbr = -(1 as libc::c_int);
                    } else {
                        if ted - tid >= 0 as libc::c_int {
                            *bndind.offset(nbnd as isize) = i;
                            let fresh7 = nbnd;
                            nbnd = nbnd + 1;
                            *bndptr.offset(i as isize) = fresh7;
                        }
                        j = 0 as libc::c_int;
                        while j < (*myrinfo).nnbrs {
                            *htable
                                .offset(
                                    (*mynbrs.offset(j as isize)).pid as isize,
                                ) = -(1 as libc::c_int);
                            j += 1;
                            j;
                        }
                    }
                }
                i += 1;
                i;
            }
            (*graph).nbnd = nbnd;
        }
        1 => {
            let mut myrinfo_0: *mut vkrinfo_t = 0 as *mut vkrinfo_t;
            let mut mynbrs_0: *mut vnbr_t = 0 as *mut vnbr_t;
            i = 0 as libc::c_int;
            while i < nvtxs {
                k = *cmap.offset(i as isize);
                *where_0.offset(i as isize) = *cwhere.offset(k as isize);
                *cmap.offset(i as isize) = (*((*cgraph).vkrinfo).offset(k as isize)).ned;
                i += 1;
                i;
            }
            memset(
                (*graph).vkrinfo as *mut libc::c_void,
                0 as libc::c_int,
                (::core::mem::size_of::<vkrinfo_t>() as libc::c_ulong)
                    .wrapping_mul(nvtxs as libc::c_ulong),
            );
            libmetis__vnbrpoolReset(ctrl);
            i = 0 as libc::c_int;
            while i < nvtxs {
                istart = *xadj.offset(i as isize);
                iend = *xadj.offset((i + 1 as libc::c_int) as isize);
                myrinfo_0 = ((*graph).vkrinfo).offset(i as isize);
                if *cmap.offset(i as isize) == 0 as libc::c_int {
                    (*myrinfo_0).nid = iend - istart;
                    (*myrinfo_0).inbr = -(1 as libc::c_int);
                } else {
                    (*myrinfo_0)
                        .inbr = libmetis__vnbrpoolGetNext(
                        ctrl,
                        iend - istart + 1 as libc::c_int,
                    );
                    mynbrs_0 = ((*ctrl).vnbrpool).offset((*myrinfo_0).inbr as isize);
                    me = *where_0.offset(i as isize);
                    tid = 0 as libc::c_int;
                    ted = 0 as libc::c_int;
                    j = istart;
                    while j < iend {
                        other = *where_0.offset(*adjncy.offset(j as isize) as isize);
                        if me == other {
                            tid += 1;
                            tid;
                        } else {
                            ted += 1;
                            ted;
                            k = *htable.offset(other as isize);
                            if k == -(1 as libc::c_int) {
                                *htable.offset(other as isize) = (*myrinfo_0).nnbrs;
                                (*mynbrs_0.offset((*myrinfo_0).nnbrs as isize))
                                    .gv = 0 as libc::c_int;
                                (*mynbrs_0.offset((*myrinfo_0).nnbrs as isize)).pid = other;
                                let fresh8 = (*myrinfo_0).nnbrs;
                                (*myrinfo_0).nnbrs = (*myrinfo_0).nnbrs + 1;
                                (*mynbrs_0.offset(fresh8 as isize)).ned = 1 as libc::c_int;
                            } else {
                                let ref mut fresh9 = (*mynbrs_0.offset(k as isize)).ned;
                                *fresh9 += 1;
                                *fresh9;
                            }
                        }
                        j += 1;
                        j;
                    }
                    (*myrinfo_0).nid = tid;
                    (*myrinfo_0).ned = ted;
                    if ted == 0 as libc::c_int {
                        (*ctrl)
                            .nbrpoolcpos = ((*ctrl).nbrpoolcpos as libc::c_ulong)
                            .wrapping_sub(
                                (iend - istart + 1 as libc::c_int) as libc::c_ulong,
                            ) as size_t as size_t;
                        (*myrinfo_0).inbr = -(1 as libc::c_int);
                    } else {
                        j = 0 as libc::c_int;
                        while j < (*myrinfo_0).nnbrs {
                            *htable
                                .offset(
                                    (*mynbrs_0.offset(j as isize)).pid as isize,
                                ) = -(1 as libc::c_int);
                            j += 1;
                            j;
                        }
                    }
                }
                i += 1;
                i;
            }
            libmetis__ComputeKWayVolGains(ctrl, graph);
        }
        _ => {
            gk_errexit(
                15 as libc::c_int,
                b"Unknown objtype of %d\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*ctrl).objtype as libc::c_uint,
            );
        }
    }
    (*graph).mincut = (*cgraph).mincut;
    libmetis__icopy((nparts * (*graph).ncon) as size_t, (*cgraph).pwgts, (*graph).pwgts);
    libmetis__FreeGraph(&mut (*graph).coarser);
    (*graph).coarser = 0 as *mut graph_t;
    libmetis__wspacepop(ctrl);
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__ComputeKWayBoundary(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut bndtype: idx_t,
) {
    let mut i: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut nbnd: idx_t = 0;
    let mut bndind: *mut idx_t = 0 as *mut idx_t;
    let mut bndptr: *mut idx_t = 0 as *mut idx_t;
    nvtxs = (*graph).nvtxs;
    bndind = (*graph).bndind;
    bndptr = libmetis__iset(nvtxs as size_t, -(1 as libc::c_int), (*graph).bndptr);
    nbnd = 0 as libc::c_int;
    match (*ctrl).objtype as libc::c_uint {
        0 => {
            if bndtype == 1 as libc::c_int {
                i = 0 as libc::c_int;
                while i < nvtxs {
                    if (*((*graph).ckrinfo).offset(i as isize)).ed
                        - (*((*graph).ckrinfo).offset(i as isize)).id >= 0 as libc::c_int
                    {
                        *bndind.offset(nbnd as isize) = i;
                        let fresh10 = nbnd;
                        nbnd = nbnd + 1;
                        *bndptr.offset(i as isize) = fresh10;
                    }
                    i += 1;
                    i;
                }
            } else {
                i = 0 as libc::c_int;
                while i < nvtxs {
                    if (*((*graph).ckrinfo).offset(i as isize)).ed > 0 as libc::c_int {
                        *bndind.offset(nbnd as isize) = i;
                        let fresh11 = nbnd;
                        nbnd = nbnd + 1;
                        *bndptr.offset(i as isize) = fresh11;
                    }
                    i += 1;
                    i;
                }
            }
        }
        1 => {
            if bndtype == 1 as libc::c_int {
                i = 0 as libc::c_int;
                while i < nvtxs {
                    if (*((*graph).vkrinfo).offset(i as isize)).gv >= 0 as libc::c_int {
                        *bndind.offset(nbnd as isize) = i;
                        let fresh12 = nbnd;
                        nbnd = nbnd + 1;
                        *bndptr.offset(i as isize) = fresh12;
                    }
                    i += 1;
                    i;
                }
            } else {
                i = 0 as libc::c_int;
                while i < nvtxs {
                    if (*((*graph).vkrinfo).offset(i as isize)).ned > 0 as libc::c_int {
                        *bndind.offset(nbnd as isize) = i;
                        let fresh13 = nbnd;
                        nbnd = nbnd + 1;
                        *bndptr.offset(i as isize) = fresh13;
                    }
                    i += 1;
                    i;
                }
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
    }
    (*graph).nbnd = nbnd;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__ComputeKWayVolGains(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
) {
    let mut i: idx_t = 0;
    let mut ii: idx_t = 0;
    let mut j: idx_t = 0;
    let mut k: idx_t = 0;
    let mut l: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut nparts: idx_t = 0;
    let mut me: idx_t = 0;
    let mut other: idx_t = 0;
    let mut pid: idx_t = 0;
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
    let mut vsize: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut adjwgt: *mut idx_t = 0 as *mut idx_t;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    let mut bndind: *mut idx_t = 0 as *mut idx_t;
    let mut bndptr: *mut idx_t = 0 as *mut idx_t;
    let mut ophtable: *mut idx_t = 0 as *mut idx_t;
    let mut myrinfo: *mut vkrinfo_t = 0 as *mut vkrinfo_t;
    let mut orinfo: *mut vkrinfo_t = 0 as *mut vkrinfo_t;
    let mut mynbrs: *mut vnbr_t = 0 as *mut vnbr_t;
    let mut onbrs: *mut vnbr_t = 0 as *mut vnbr_t;
    libmetis__wspacepush(ctrl);
    nparts = (*ctrl).nparts;
    nvtxs = (*graph).nvtxs;
    xadj = (*graph).xadj;
    vsize = (*graph).vsize;
    adjncy = (*graph).adjncy;
    adjwgt = (*graph).adjwgt;
    where_0 = (*graph).where_0;
    bndind = (*graph).bndind;
    bndptr = libmetis__iset(nvtxs as size_t, -(1 as libc::c_int), (*graph).bndptr);
    ophtable = libmetis__iset(
        nparts as size_t,
        -(1 as libc::c_int),
        libmetis__iwspacemalloc(ctrl, nparts),
    );
    (*graph).nbnd = 0 as libc::c_int;
    (*graph).minvol = (*graph).nbnd;
    i = 0 as libc::c_int;
    while i < nvtxs {
        myrinfo = ((*graph).vkrinfo).offset(i as isize);
        (*myrinfo).gv = -(2147483647 as libc::c_int) - 1 as libc::c_int;
        if (*myrinfo).nnbrs > 0 as libc::c_int {
            me = *where_0.offset(i as isize);
            mynbrs = ((*ctrl).vnbrpool).offset((*myrinfo).inbr as isize);
            (*graph).minvol += (*myrinfo).nnbrs * *vsize.offset(i as isize);
            j = *xadj.offset(i as isize);
            while j < *xadj.offset((i + 1 as libc::c_int) as isize) {
                ii = *adjncy.offset(j as isize);
                other = *where_0.offset(ii as isize);
                orinfo = ((*graph).vkrinfo).offset(ii as isize);
                onbrs = ((*ctrl).vnbrpool).offset((*orinfo).inbr as isize);
                k = 0 as libc::c_int;
                while k < (*orinfo).nnbrs {
                    *ophtable.offset((*onbrs.offset(k as isize)).pid as isize) = k;
                    k += 1;
                    k;
                }
                *ophtable.offset(other as isize) = 1 as libc::c_int;
                if me == other {
                    k = 0 as libc::c_int;
                    while k < (*myrinfo).nnbrs {
                        if *ophtable.offset((*mynbrs.offset(k as isize)).pid as isize)
                            == -(1 as libc::c_int)
                        {
                            let ref mut fresh14 = (*mynbrs.offset(k as isize)).gv;
                            *fresh14 -= *vsize.offset(ii as isize);
                        }
                        k += 1;
                        k;
                    }
                } else if (*onbrs.offset(*ophtable.offset(me as isize) as isize)).ned
                    == 1 as libc::c_int
                {
                    k = 0 as libc::c_int;
                    while k < (*myrinfo).nnbrs {
                        if *ophtable.offset((*mynbrs.offset(k as isize)).pid as isize)
                            != -(1 as libc::c_int)
                        {
                            let ref mut fresh15 = (*mynbrs.offset(k as isize)).gv;
                            *fresh15 += *vsize.offset(ii as isize);
                        }
                        k += 1;
                        k;
                    }
                } else {
                    k = 0 as libc::c_int;
                    while k < (*myrinfo).nnbrs {
                        if *ophtable.offset((*mynbrs.offset(k as isize)).pid as isize)
                            == -(1 as libc::c_int)
                        {
                            let ref mut fresh16 = (*mynbrs.offset(k as isize)).gv;
                            *fresh16 -= *vsize.offset(ii as isize);
                        }
                        k += 1;
                        k;
                    }
                }
                k = 0 as libc::c_int;
                while k < (*orinfo).nnbrs {
                    *ophtable
                        .offset(
                            (*onbrs.offset(k as isize)).pid as isize,
                        ) = -(1 as libc::c_int);
                    k += 1;
                    k;
                }
                *ophtable.offset(other as isize) = -(1 as libc::c_int);
                j += 1;
                j;
            }
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
        }
        if (*myrinfo).gv >= 0 as libc::c_int {
            *bndind.offset((*graph).nbnd as isize) = i;
            let fresh17 = (*graph).nbnd;
            (*graph).nbnd = (*graph).nbnd + 1;
            *bndptr.offset(i as isize) = fresh17;
        }
        i += 1;
        i;
    }
    libmetis__wspacepop(ctrl);
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__IsBalanced(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut ffactor: real_t,
) -> libc::c_int {
    return (libmetis__ComputeLoadImbalanceDiff(
        graph,
        (*ctrl).nparts,
        (*ctrl).pijbm,
        (*ctrl).ubfactors,
    ) <= ffactor) as libc::c_int;
}
