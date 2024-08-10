use ::libc;
extern "C" {
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn gk_malloc(nbytes: size_t, msg: *mut libc::c_char) -> *mut libc::c_void;
    fn gk_free(ptr1: *mut *mut libc::c_void, _: ...);
    fn libmetis__isum(n: size_t, x: *mut idx_t, incx: size_t) -> idx_t;
    fn libmetis__imalloc(n: size_t, msg: *mut libc::c_char) -> *mut idx_t;
    fn libmetis__ismalloc(n: size_t, ival: idx_t, msg: *mut libc::c_char) -> *mut idx_t;
    fn libmetis__rmalloc(n: size_t, msg: *mut libc::c_char) -> *mut real_t;
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
pub unsafe extern "C" fn libmetis__SetupGraph(
    mut ctrl: *mut ctrl_t,
    mut nvtxs: idx_t,
    mut ncon: idx_t,
    mut xadj: *mut idx_t,
    mut adjncy: *mut idx_t,
    mut vwgt: *mut idx_t,
    mut vsize: *mut idx_t,
    mut adjwgt: *mut idx_t,
) -> *mut graph_t {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut k: idx_t = 0;
    let mut sum: idx_t = 0;
    let mut nvwgt: *mut real_t = 0 as *mut real_t;
    let mut graph: *mut graph_t = 0 as *mut graph_t;
    graph = libmetis__CreateGraph();
    (*graph).nvtxs = nvtxs;
    (*graph).nedges = *xadj.offset(nvtxs as isize);
    (*graph).ncon = ncon;
    (*graph).xadj = xadj;
    (*graph).free_xadj = 0 as libc::c_int;
    (*graph).adjncy = adjncy;
    (*graph).free_adjncy = 0 as libc::c_int;
    if !vwgt.is_null() {
        (*graph).vwgt = vwgt;
        (*graph).free_vwgt = 0 as libc::c_int;
    } else {
        (*graph)
            .vwgt = libmetis__ismalloc(
            (ncon * nvtxs) as size_t,
            1 as libc::c_int,
            b"SetupGraph: vwgt\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        vwgt = (*graph).vwgt;
    }
    (*graph)
        .tvwgt = libmetis__imalloc(
        ncon as size_t,
        b"SetupGraph: tvwgts\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (*graph)
        .invtvwgt = libmetis__rmalloc(
        ncon as size_t,
        b"SetupGraph: invtvwgts\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < ncon {
        *((*graph).tvwgt)
            .offset(
                i as isize,
            ) = libmetis__isum(nvtxs as size_t, vwgt.offset(i as isize), ncon as size_t);
        *((*graph).invtvwgt)
            .offset(
                i as isize,
            ) = (1.0f64
            / (if *((*graph).tvwgt).offset(i as isize) > 0 as libc::c_int {
                *((*graph).tvwgt).offset(i as isize)
            } else {
                1 as libc::c_int
            }) as libc::c_double) as real_t;
        i += 1;
        i;
    }
    if (*ctrl).objtype as libc::c_uint
        == METIS_OBJTYPE_VOL as libc::c_int as libc::c_uint
    {
        if !vsize.is_null() {
            (*graph).vsize = vsize;
            (*graph).free_vsize = 0 as libc::c_int;
        } else {
            (*graph)
                .vsize = libmetis__ismalloc(
                nvtxs as size_t,
                1 as libc::c_int,
                b"SetupGraph: vsize\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            vsize = (*graph).vsize;
        }
        (*graph)
            .adjwgt = libmetis__imalloc(
            (*graph).nedges as size_t,
            b"SetupGraph: adjwgt\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        adjwgt = (*graph).adjwgt;
        i = 0 as libc::c_int;
        while i < nvtxs {
            j = *xadj.offset(i as isize);
            while j < *xadj.offset((i + 1 as libc::c_int) as isize) {
                *adjwgt
                    .offset(
                        j as isize,
                    ) = 1 as libc::c_int + *vsize.offset(i as isize)
                    + *vsize.offset(*adjncy.offset(j as isize) as isize);
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    } else if !adjwgt.is_null() {
        (*graph).adjwgt = adjwgt;
        (*graph).free_adjwgt = 0 as libc::c_int;
    } else {
        (*graph)
            .adjwgt = libmetis__ismalloc(
            (*graph).nedges as size_t,
            1 as libc::c_int,
            b"SetupGraph: adjwgt\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        adjwgt = (*graph).adjwgt;
    }
    libmetis__SetupGraph_tvwgt(graph);
    if (*ctrl).optype as libc::c_uint == METIS_OP_PMETIS as libc::c_int as libc::c_uint
        || (*ctrl).optype as libc::c_uint
            == METIS_OP_OMETIS as libc::c_int as libc::c_uint
    {
        libmetis__SetupGraph_label(graph);
    }
    return graph;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__SetupGraph_tvwgt(mut graph: *mut graph_t) {
    let mut i: idx_t = 0;
    if ((*graph).tvwgt).is_null() {
        (*graph)
            .tvwgt = libmetis__imalloc(
            (*graph).ncon as size_t,
            b"SetupGraph_tvwgt: tvwgt\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if ((*graph).invtvwgt).is_null() {
        (*graph)
            .invtvwgt = libmetis__rmalloc(
            (*graph).ncon as size_t,
            b"SetupGraph_tvwgt: invtvwgt\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    i = 0 as libc::c_int;
    while i < (*graph).ncon {
        *((*graph).tvwgt)
            .offset(
                i as isize,
            ) = libmetis__isum(
            (*graph).nvtxs as size_t,
            ((*graph).vwgt).offset(i as isize),
            (*graph).ncon as size_t,
        );
        *((*graph).invtvwgt)
            .offset(
                i as isize,
            ) = (1.0f64
            / (if *((*graph).tvwgt).offset(i as isize) > 0 as libc::c_int {
                *((*graph).tvwgt).offset(i as isize)
            } else {
                1 as libc::c_int
            }) as libc::c_double) as real_t;
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__SetupGraph_label(mut graph: *mut graph_t) {
    let mut i: idx_t = 0;
    if ((*graph).label).is_null() {
        (*graph)
            .label = libmetis__imalloc(
            (*graph).nvtxs as size_t,
            b"SetupGraph_label: label\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    i = 0 as libc::c_int;
    while i < (*graph).nvtxs {
        *((*graph).label).offset(i as isize) = i;
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__SetupSplitGraph(
    mut graph: *mut graph_t,
    mut snvtxs: idx_t,
    mut snedges: idx_t,
) -> *mut graph_t {
    let mut sgraph: *mut graph_t = 0 as *mut graph_t;
    sgraph = libmetis__CreateGraph();
    (*sgraph).nvtxs = snvtxs;
    (*sgraph).nedges = snedges;
    (*sgraph).ncon = (*graph).ncon;
    (*sgraph)
        .xadj = libmetis__imalloc(
        (snvtxs + 1 as libc::c_int) as size_t,
        b"SetupSplitGraph: xadj\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*sgraph)
        .vwgt = libmetis__imalloc(
        ((*sgraph).ncon * snvtxs) as size_t,
        b"SetupSplitGraph: vwgt\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*sgraph)
        .adjncy = libmetis__imalloc(
        snedges as size_t,
        b"SetupSplitGraph: adjncy\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*sgraph)
        .adjwgt = libmetis__imalloc(
        snedges as size_t,
        b"SetupSplitGraph: adjwgt\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*sgraph)
        .label = libmetis__imalloc(
        snvtxs as size_t,
        b"SetupSplitGraph: label\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*sgraph)
        .tvwgt = libmetis__imalloc(
        (*sgraph).ncon as size_t,
        b"SetupSplitGraph: tvwgt\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*sgraph)
        .invtvwgt = libmetis__rmalloc(
        (*sgraph).ncon as size_t,
        b"SetupSplitGraph: invtvwgt\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    if !((*graph).vsize).is_null() {
        (*sgraph)
            .vsize = libmetis__imalloc(
            snvtxs as size_t,
            b"SetupSplitGraph: vsize\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    return sgraph;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__CreateGraph() -> *mut graph_t {
    let mut graph: *mut graph_t = 0 as *mut graph_t;
    graph = gk_malloc(
        ::core::mem::size_of::<graph_t>() as libc::c_ulong,
        b"CreateGraph: graph\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut graph_t;
    libmetis__InitGraph(graph);
    return graph;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__InitGraph(mut graph: *mut graph_t) {
    memset(
        graph as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<graph_t>() as libc::c_ulong,
    );
    (*graph).nvtxs = -(1 as libc::c_int);
    (*graph).nedges = -(1 as libc::c_int);
    (*graph).ncon = -(1 as libc::c_int);
    (*graph).mincut = -(1 as libc::c_int);
    (*graph).minvol = -(1 as libc::c_int);
    (*graph).nbnd = -(1 as libc::c_int);
    (*graph).xadj = 0 as *mut idx_t;
    (*graph).vwgt = 0 as *mut idx_t;
    (*graph).vsize = 0 as *mut idx_t;
    (*graph).adjncy = 0 as *mut idx_t;
    (*graph).adjwgt = 0 as *mut idx_t;
    (*graph).label = 0 as *mut idx_t;
    (*graph).cmap = 0 as *mut idx_t;
    (*graph).tvwgt = 0 as *mut idx_t;
    (*graph).invtvwgt = 0 as *mut real_t;
    (*graph).free_xadj = 1 as libc::c_int;
    (*graph).free_vwgt = 1 as libc::c_int;
    (*graph).free_vsize = 1 as libc::c_int;
    (*graph).free_adjncy = 1 as libc::c_int;
    (*graph).free_adjwgt = 1 as libc::c_int;
    (*graph).where_0 = 0 as *mut idx_t;
    (*graph).pwgts = 0 as *mut idx_t;
    (*graph).id = 0 as *mut idx_t;
    (*graph).ed = 0 as *mut idx_t;
    (*graph).bndptr = 0 as *mut idx_t;
    (*graph).bndind = 0 as *mut idx_t;
    (*graph).nrinfo = 0 as *mut nrinfo_t;
    (*graph).ckrinfo = 0 as *mut ckrinfo_t;
    (*graph).vkrinfo = 0 as *mut vkrinfo_t;
    (*graph).coarser = 0 as *mut graph_t;
    (*graph).finer = 0 as *mut graph_t;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__FreeRData(mut graph: *mut graph_t) {
    if (*graph).ckrinfo as *mut libc::c_void == (*graph).vkrinfo as *mut libc::c_void {
        (*graph).ckrinfo = 0 as *mut ckrinfo_t;
    }
    gk_free(
        &mut (*graph).where_0 as *mut *mut idx_t as *mut *mut libc::c_void,
        &mut (*graph).pwgts as *mut *mut idx_t,
        &mut (*graph).id as *mut *mut idx_t,
        &mut (*graph).ed as *mut *mut idx_t,
        &mut (*graph).bndptr as *mut *mut idx_t,
        &mut (*graph).bndind as *mut *mut idx_t,
        &mut (*graph).nrinfo as *mut *mut nrinfo_t,
        &mut (*graph).ckrinfo as *mut *mut ckrinfo_t,
        &mut (*graph).vkrinfo as *mut *mut vkrinfo_t,
        0 as *mut *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__FreeGraph(mut r_graph: *mut *mut graph_t) {
    let mut graph: *mut graph_t = 0 as *mut graph_t;
    graph = *r_graph;
    if (*graph).free_xadj != 0 {
        gk_free(
            &mut (*graph).xadj as *mut *mut idx_t as *mut *mut libc::c_void,
            0 as *mut *mut libc::c_void,
        );
    }
    if (*graph).free_vwgt != 0 {
        gk_free(
            &mut (*graph).vwgt as *mut *mut idx_t as *mut *mut libc::c_void,
            0 as *mut *mut libc::c_void,
        );
    }
    if (*graph).free_vsize != 0 {
        gk_free(
            &mut (*graph).vsize as *mut *mut idx_t as *mut *mut libc::c_void,
            0 as *mut *mut libc::c_void,
        );
    }
    if (*graph).free_adjncy != 0 {
        gk_free(
            &mut (*graph).adjncy as *mut *mut idx_t as *mut *mut libc::c_void,
            0 as *mut *mut libc::c_void,
        );
    }
    if (*graph).free_adjwgt != 0 {
        gk_free(
            &mut (*graph).adjwgt as *mut *mut idx_t as *mut *mut libc::c_void,
            0 as *mut *mut libc::c_void,
        );
    }
    libmetis__FreeRData(graph);
    gk_free(
        &mut (*graph).tvwgt as *mut *mut idx_t as *mut *mut libc::c_void,
        &mut (*graph).invtvwgt as *mut *mut real_t,
        &mut (*graph).label as *mut *mut idx_t,
        &mut (*graph).cmap as *mut *mut idx_t,
        &mut graph as *mut *mut graph_t,
        0 as *mut *mut libc::c_void,
    );
    *r_graph = 0 as *mut graph_t;
}
