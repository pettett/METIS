use ::libc;
extern "C" {
    fn gk_CPUSeconds() -> libc::c_double;
    fn libmetis__imalloc(n: size_t, msg: *mut libc::c_char) -> *mut idx_t;
    fn libmetis__iset(n: size_t, val: idx_t, x: *mut idx_t) -> *mut idx_t;
    fn libmetis__icopy(n: size_t, a: *mut idx_t, b: *mut idx_t) -> *mut idx_t;
    fn libmetis__Balance2Way(
        ctrl: *mut ctrl_t,
        graph: *mut graph_t,
        ntpwgts: *mut real_t,
    );
    fn libmetis__FM_2WayRefine(
        ctrl: *mut ctrl_t,
        graph: *mut graph_t,
        ntpwgts: *mut real_t,
        niter: idx_t,
    );
    fn libmetis__FreeGraph(graph: *mut *mut graph_t);
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
pub unsafe extern "C" fn libmetis__Refine2Way(
    mut ctrl: *mut ctrl_t,
    mut orggraph: *mut graph_t,
    mut graph: *mut graph_t,
    mut tpwgts: *mut real_t,
) {
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint
        != 0
    {
        (*ctrl).UncoarsenTmr -= gk_CPUSeconds();
    }
    libmetis__Compute2WayPartitionParams(ctrl, graph);
    loop {
        if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint
            != 0
        {
            (*ctrl).RefTmr -= gk_CPUSeconds();
        }
        libmetis__Balance2Way(ctrl, graph, tpwgts);
        libmetis__FM_2WayRefine(ctrl, graph, tpwgts, (*ctrl).niter);
        if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint
            != 0
        {
            (*ctrl).RefTmr += gk_CPUSeconds();
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
        libmetis__Project2WayPartition(ctrl, graph);
        if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint
            != 0
        {
            (*ctrl).ProjectTmr += gk_CPUSeconds();
        }
    }
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint
        != 0
    {
        (*ctrl).UncoarsenTmr += gk_CPUSeconds();
    }
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__Allocate2WayPartitionMemory(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
) {
    let mut nvtxs: idx_t = 0;
    let mut ncon: idx_t = 0;
    nvtxs = (*graph).nvtxs;
    ncon = (*graph).ncon;
    (*graph)
        .pwgts = libmetis__imalloc(
        (2 as libc::c_int * ncon) as size_t,
        b"Allocate2WayPartitionMemory: pwgts\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*graph)
        .where_0 = libmetis__imalloc(
        nvtxs as size_t,
        b"Allocate2WayPartitionMemory: where\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*graph)
        .bndptr = libmetis__imalloc(
        nvtxs as size_t,
        b"Allocate2WayPartitionMemory: bndptr\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*graph)
        .bndind = libmetis__imalloc(
        nvtxs as size_t,
        b"Allocate2WayPartitionMemory: bndind\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*graph)
        .id = libmetis__imalloc(
        nvtxs as size_t,
        b"Allocate2WayPartitionMemory: id\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*graph)
        .ed = libmetis__imalloc(
        nvtxs as size_t,
        b"Allocate2WayPartitionMemory: ed\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__Compute2WayPartitionParams(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
) {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut ncon: idx_t = 0;
    let mut nbnd: idx_t = 0;
    let mut mincut: idx_t = 0;
    let mut istart: idx_t = 0;
    let mut iend: idx_t = 0;
    let mut tid: idx_t = 0;
    let mut ted: idx_t = 0;
    let mut me: idx_t = 0;
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
    let mut vwgt: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut adjwgt: *mut idx_t = 0 as *mut idx_t;
    let mut pwgts: *mut idx_t = 0 as *mut idx_t;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    let mut bndptr: *mut idx_t = 0 as *mut idx_t;
    let mut bndind: *mut idx_t = 0 as *mut idx_t;
    let mut id: *mut idx_t = 0 as *mut idx_t;
    let mut ed: *mut idx_t = 0 as *mut idx_t;
    nvtxs = (*graph).nvtxs;
    ncon = (*graph).ncon;
    xadj = (*graph).xadj;
    vwgt = (*graph).vwgt;
    adjncy = (*graph).adjncy;
    adjwgt = (*graph).adjwgt;
    where_0 = (*graph).where_0;
    id = (*graph).id;
    ed = (*graph).ed;
    pwgts = libmetis__iset(
        (2 as libc::c_int * ncon) as size_t,
        0 as libc::c_int,
        (*graph).pwgts,
    );
    bndptr = libmetis__iset(nvtxs as size_t, -(1 as libc::c_int), (*graph).bndptr);
    bndind = (*graph).bndind;
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
    nbnd = 0 as libc::c_int;
    mincut = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < nvtxs {
        istart = *xadj.offset(i as isize);
        iend = *xadj.offset((i + 1 as libc::c_int) as isize);
        me = *where_0.offset(i as isize);
        ted = 0 as libc::c_int;
        tid = ted;
        j = istart;
        while j < iend {
            if me == *where_0.offset(*adjncy.offset(j as isize) as isize) {
                tid += *adjwgt.offset(j as isize);
            } else {
                ted += *adjwgt.offset(j as isize);
            }
            j += 1;
            j;
        }
        *id.offset(i as isize) = tid;
        *ed.offset(i as isize) = ted;
        if ted > 0 as libc::c_int || istart == iend {
            *bndind.offset(nbnd as isize) = i;
            let fresh2 = nbnd;
            nbnd = nbnd + 1;
            *bndptr.offset(i as isize) = fresh2;
            mincut += ted;
        }
        i += 1;
        i;
    }
    (*graph).mincut = mincut / 2 as libc::c_int;
    (*graph).nbnd = nbnd;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__Project2WayPartition(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
) {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut istart: idx_t = 0;
    let mut iend: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut nbnd: idx_t = 0;
    let mut me: idx_t = 0;
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
    let mut cbndptr: *mut idx_t = 0 as *mut idx_t;
    let mut id: *mut idx_t = 0 as *mut idx_t;
    let mut ed: *mut idx_t = 0 as *mut idx_t;
    let mut cgraph: *mut graph_t = 0 as *mut graph_t;
    libmetis__Allocate2WayPartitionMemory(ctrl, graph);
    cgraph = (*graph).coarser;
    cwhere = (*cgraph).where_0;
    cbndptr = (*cgraph).bndptr;
    nvtxs = (*graph).nvtxs;
    cmap = (*graph).cmap;
    xadj = (*graph).xadj;
    adjncy = (*graph).adjncy;
    adjwgt = (*graph).adjwgt;
    where_0 = (*graph).where_0;
    id = (*graph).id;
    ed = (*graph).ed;
    bndptr = libmetis__iset(nvtxs as size_t, -(1 as libc::c_int), (*graph).bndptr);
    bndind = (*graph).bndind;
    i = 0 as libc::c_int;
    while i < nvtxs {
        j = *cmap.offset(i as isize);
        *where_0.offset(i as isize) = *cwhere.offset(j as isize);
        *cmap.offset(i as isize) = *cbndptr.offset(j as isize);
        i += 1;
        i;
    }
    nbnd = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < nvtxs {
        istart = *xadj.offset(i as isize);
        iend = *xadj.offset((i + 1 as libc::c_int) as isize);
        ted = 0 as libc::c_int;
        tid = ted;
        if *cmap.offset(i as isize) == -(1 as libc::c_int) {
            j = istart;
            while j < iend {
                tid += *adjwgt.offset(j as isize);
                j += 1;
                j;
            }
        } else {
            me = *where_0.offset(i as isize);
            j = istart;
            while j < iend {
                if me == *where_0.offset(*adjncy.offset(j as isize) as isize) {
                    tid += *adjwgt.offset(j as isize);
                } else {
                    ted += *adjwgt.offset(j as isize);
                }
                j += 1;
                j;
            }
        }
        *id.offset(i as isize) = tid;
        *ed.offset(i as isize) = ted;
        if ted > 0 as libc::c_int || istart == iend {
            *bndind.offset(nbnd as isize) = i;
            let fresh3 = nbnd;
            nbnd = nbnd + 1;
            *bndptr.offset(i as isize) = fresh3;
        }
        i += 1;
        i;
    }
    (*graph).mincut = (*cgraph).mincut;
    (*graph).nbnd = nbnd;
    libmetis__icopy(
        (2 as libc::c_int * (*graph).ncon) as size_t,
        (*cgraph).pwgts,
        (*graph).pwgts,
    );
    libmetis__FreeGraph(&mut (*graph).coarser);
    (*graph).coarser = 0 as *mut graph_t;
}
