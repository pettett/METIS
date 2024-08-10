use ::libc;
extern "C" {
    fn gk_malloc(nbytes: size_t, msg: *mut libc::c_char) -> *mut libc::c_void;
    fn gk_errexit(signum: libc::c_int, _: *mut libc::c_char, _: ...);
    fn gk_CPUSeconds() -> libc::c_double;
    fn libmetis__imalloc(n: size_t, msg: *mut libc::c_char) -> *mut idx_t;
    fn libmetis__iset(n: size_t, val: idx_t, x: *mut idx_t) -> *mut idx_t;
    fn libmetis__FreeGraph(graph: *mut *mut graph_t);
    fn libmetis__FM_2WayNodeRefine2Sided(
        ctrl: *mut ctrl_t,
        graph: *mut graph_t,
        niter: idx_t,
    );
    fn libmetis__FM_2WayNodeRefine1Sided(
        ctrl: *mut ctrl_t,
        graph: *mut graph_t,
        niter: idx_t,
    );
    fn libmetis__FM_2WayNodeBalance(ctrl: *mut ctrl_t, graph: *mut graph_t);
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
pub unsafe extern "C" fn libmetis__Refine2WayNode(
    mut ctrl: *mut ctrl_t,
    mut orggraph: *mut graph_t,
    mut graph: *mut graph_t,
) {
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint
        != 0
    {
        (*ctrl).UncoarsenTmr -= gk_CPUSeconds();
    }
    if graph == orggraph {
        libmetis__Compute2WayNodePartitionParams(ctrl, graph);
    } else {
        loop {
            graph = (*graph).finer;
            if (*ctrl).dbglvl as libc::c_uint
                & METIS_DBG_TIME as libc::c_int as libc::c_uint != 0
            {
                (*ctrl).ProjectTmr -= gk_CPUSeconds();
            }
            libmetis__Project2WayNodePartition(ctrl, graph);
            if (*ctrl).dbglvl as libc::c_uint
                & METIS_DBG_TIME as libc::c_int as libc::c_uint != 0
            {
                (*ctrl).ProjectTmr += gk_CPUSeconds();
            }
            if (*ctrl).dbglvl as libc::c_uint
                & METIS_DBG_TIME as libc::c_int as libc::c_uint != 0
            {
                (*ctrl).RefTmr -= gk_CPUSeconds();
            }
            libmetis__FM_2WayNodeBalance(ctrl, graph);
            match (*ctrl).rtype as libc::c_uint {
                2 => {
                    libmetis__FM_2WayNodeRefine2Sided(ctrl, graph, (*ctrl).niter);
                }
                3 => {
                    libmetis__FM_2WayNodeRefine1Sided(ctrl, graph, (*ctrl).niter);
                }
                _ => {
                    gk_errexit(
                        15 as libc::c_int,
                        b"Unknown rtype of %d\n\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        (*ctrl).rtype as libc::c_uint,
                    );
                }
            }
            if (*ctrl).dbglvl as libc::c_uint
                & METIS_DBG_TIME as libc::c_int as libc::c_uint != 0
            {
                (*ctrl).RefTmr += gk_CPUSeconds();
            }
            if !(graph != orggraph) {
                break;
            }
        }
    }
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint
        != 0
    {
        (*ctrl).UncoarsenTmr += gk_CPUSeconds();
    }
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__Allocate2WayNodePartitionMemory(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
) {
    let mut nvtxs: idx_t = 0;
    nvtxs = (*graph).nvtxs;
    (*graph)
        .pwgts = libmetis__imalloc(
        3 as libc::c_int as size_t,
        b"Allocate2WayNodePartitionMemory: pwgts\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*graph)
        .where_0 = libmetis__imalloc(
        nvtxs as size_t,
        b"Allocate2WayNodePartitionMemory: where\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*graph)
        .bndptr = libmetis__imalloc(
        nvtxs as size_t,
        b"Allocate2WayNodePartitionMemory: bndptr\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*graph)
        .bndind = libmetis__imalloc(
        nvtxs as size_t,
        b"Allocate2WayNodePartitionMemory: bndind\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*graph)
        .nrinfo = gk_malloc(
        (nvtxs as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<nrinfo_t>() as libc::c_ulong),
        b"Allocate2WayNodePartitionMemory: nrinfo\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    ) as *mut nrinfo_t;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__Compute2WayNodePartitionParams(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
) {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut nbnd: idx_t = 0;
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut vwgt: *mut idx_t = 0 as *mut idx_t;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    let mut pwgts: *mut idx_t = 0 as *mut idx_t;
    let mut bndind: *mut idx_t = 0 as *mut idx_t;
    let mut bndptr: *mut idx_t = 0 as *mut idx_t;
    let mut edegrees: *mut idx_t = 0 as *mut idx_t;
    let mut rinfo: *mut nrinfo_t = 0 as *mut nrinfo_t;
    let mut me: idx_t = 0;
    let mut other: idx_t = 0;
    nvtxs = (*graph).nvtxs;
    xadj = (*graph).xadj;
    vwgt = (*graph).vwgt;
    adjncy = (*graph).adjncy;
    where_0 = (*graph).where_0;
    rinfo = (*graph).nrinfo;
    pwgts = libmetis__iset(3 as libc::c_int as size_t, 0 as libc::c_int, (*graph).pwgts);
    bndind = (*graph).bndind;
    bndptr = libmetis__iset(nvtxs as size_t, -(1 as libc::c_int), (*graph).bndptr);
    nbnd = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < nvtxs {
        me = *where_0.offset(i as isize);
        let ref mut fresh0 = *pwgts.offset(me as isize);
        *fresh0 += *vwgt.offset(i as isize);
        if me == 2 as libc::c_int {
            *bndind.offset(nbnd as isize) = i;
            let fresh1 = nbnd;
            nbnd = nbnd + 1;
            *bndptr.offset(i as isize) = fresh1;
            edegrees = ((*rinfo.offset(i as isize)).edegrees).as_mut_ptr();
            let ref mut fresh2 = *edegrees.offset(1 as libc::c_int as isize);
            *fresh2 = 0 as libc::c_int;
            *edegrees.offset(0 as libc::c_int as isize) = *fresh2;
            j = *xadj.offset(i as isize);
            while j < *xadj.offset((i + 1 as libc::c_int) as isize) {
                other = *where_0.offset(*adjncy.offset(j as isize) as isize);
                if other != 2 as libc::c_int {
                    let ref mut fresh3 = *edegrees.offset(other as isize);
                    *fresh3 += *vwgt.offset(*adjncy.offset(j as isize) as isize);
                }
                j += 1;
                j;
            }
        }
        i += 1;
        i;
    }
    (*graph).mincut = *pwgts.offset(2 as libc::c_int as isize);
    (*graph).nbnd = nbnd;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__Project2WayNodePartition(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
) {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut cmap: *mut idx_t = 0 as *mut idx_t;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    let mut cwhere: *mut idx_t = 0 as *mut idx_t;
    let mut cgraph: *mut graph_t = 0 as *mut graph_t;
    cgraph = (*graph).coarser;
    cwhere = (*cgraph).where_0;
    nvtxs = (*graph).nvtxs;
    cmap = (*graph).cmap;
    libmetis__Allocate2WayNodePartitionMemory(ctrl, graph);
    where_0 = (*graph).where_0;
    i = 0 as libc::c_int;
    while i < nvtxs {
        *where_0.offset(i as isize) = *cwhere.offset(*cmap.offset(i as isize) as isize);
        i += 1;
        i;
    }
    libmetis__FreeGraph(&mut (*graph).coarser);
    (*graph).coarser = 0 as *mut graph_t;
    libmetis__Compute2WayNodePartitionParams(ctrl, graph);
}
