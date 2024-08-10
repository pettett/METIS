use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn libmetis__Setup2WayBalMultipliers(
        ctrl: *mut ctrl_t,
        graph: *mut graph_t,
        tpwgts: *mut real_t,
    );
    fn libmetis__ConstructSeparator(ctrl: *mut ctrl_t, graph: *mut graph_t);
    fn libmetis__Compute2WayNodePartitionParams(ctrl: *mut ctrl_t, graph: *mut graph_t);
    fn gk_malloc(nbytes: size_t, msg: *mut libc::c_char) -> *mut libc::c_void;
    fn gk_errexit(signum: libc::c_int, _: *mut libc::c_char, _: ...);
    fn gk_CPUSeconds() -> libc::c_double;
    fn libmetis__iargmax(n: size_t, x: *mut idx_t) -> size_t;
    fn libmetis__imalloc(n: size_t, msg: *mut libc::c_char) -> *mut idx_t;
    fn libmetis__iset(n: size_t, val: idx_t, x: *mut idx_t) -> *mut idx_t;
    fn libmetis__icopy(n: size_t, a: *mut idx_t, b: *mut idx_t) -> *mut idx_t;
    fn libmetis__irandInRange(max: idx_t) -> idx_t;
    fn libmetis__irandArrayPermute(
        n: idx_t,
        p: *mut idx_t,
        nshuffles: idx_t,
        flag: libc::c_int,
    );
    fn libmetis__Balance2Way(
        ctrl: *mut ctrl_t,
        graph: *mut graph_t,
        ntpwgts: *mut real_t,
    );
    fn libmetis__General2WayBalance(
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
    fn libmetis__FM_2WayNodeRefine2Sided(
        ctrl: *mut ctrl_t,
        graph: *mut graph_t,
        niter: idx_t,
    );
    fn libmetis__wspacepop(ctrl: *mut ctrl_t);
    fn libmetis__Compute2WayPartitionParams(ctrl: *mut ctrl_t, graph: *mut graph_t);
    fn libmetis__iwspacemalloc(_: *mut ctrl_t, _: idx_t) -> *mut idx_t;
    fn libmetis__Allocate2WayPartitionMemory(ctrl: *mut ctrl_t, graph: *mut graph_t);
    fn libmetis__wspacepush(ctrl: *mut ctrl_t);
    fn libmetis__FM_2WayNodeRefine1Sided(
        ctrl: *mut ctrl_t,
        graph: *mut graph_t,
        niter: idx_t,
    );
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
pub unsafe extern "C" fn libmetis__Init2WayPartition(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut ntpwgts: *mut real_t,
    mut niparts: idx_t,
) {
    let mut dbglvl: mdbglvl_et = 0 as mdbglvl_et;
    dbglvl = (*ctrl).dbglvl;
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_REFINE as libc::c_int as libc::c_uint
        != 0
    {
        (*ctrl)
            .dbglvl = ::core::mem::transmute::<
            libc::c_uint,
            mdbglvl_et,
        >(
            ((*ctrl).dbglvl as libc::c_uint)
                .wrapping_sub(METIS_DBG_REFINE as libc::c_int as libc::c_uint),
        ) as mdbglvl_et;
    }
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_MOVEINFO as libc::c_int as libc::c_uint
        != 0
    {
        (*ctrl)
            .dbglvl = ::core::mem::transmute::<
            libc::c_uint,
            mdbglvl_et,
        >(
            ((*ctrl).dbglvl as libc::c_uint)
                .wrapping_sub(METIS_DBG_MOVEINFO as libc::c_int as libc::c_uint),
        ) as mdbglvl_et;
    }
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint
        != 0
    {
        (*ctrl).InitPartTmr -= gk_CPUSeconds();
    }
    match (*ctrl).iptype as libc::c_uint {
        1 => {
            if (*graph).ncon == 1 as libc::c_int {
                libmetis__RandomBisection(ctrl, graph, ntpwgts, niparts);
            } else {
                libmetis__McRandomBisection(ctrl, graph, ntpwgts, niparts);
            }
        }
        0 => {
            if (*graph).nedges == 0 as libc::c_int {
                if (*graph).ncon == 1 as libc::c_int {
                    libmetis__RandomBisection(ctrl, graph, ntpwgts, niparts);
                } else {
                    libmetis__McRandomBisection(ctrl, graph, ntpwgts, niparts);
                }
            } else if (*graph).ncon == 1 as libc::c_int {
                libmetis__GrowBisection(ctrl, graph, ntpwgts, niparts);
            } else {
                libmetis__McGrowBisection(ctrl, graph, ntpwgts, niparts);
            }
        }
        _ => {
            gk_errexit(
                15 as libc::c_int,
                b"Unknown initial partition type: %d\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                (*ctrl).iptype as libc::c_uint,
            );
        }
    }
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_IPART as libc::c_int as libc::c_uint
        != 0
    {
        printf(
            b"Initial Cut: %d\n\0" as *const u8 as *const libc::c_char,
            (*graph).mincut,
        );
    }
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint
        != 0
    {
        (*ctrl).InitPartTmr += gk_CPUSeconds();
    }
    (*ctrl).dbglvl = dbglvl;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__InitSeparator(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut niparts: idx_t,
) {
    let mut ntpwgts: [real_t; 2] = [0.5f64 as real_t, 0.5f64 as real_t];
    let mut dbglvl: mdbglvl_et = 0 as mdbglvl_et;
    dbglvl = (*ctrl).dbglvl;
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_REFINE as libc::c_int as libc::c_uint
        != 0
    {
        (*ctrl)
            .dbglvl = ::core::mem::transmute::<
            libc::c_uint,
            mdbglvl_et,
        >(
            ((*ctrl).dbglvl as libc::c_uint)
                .wrapping_sub(METIS_DBG_REFINE as libc::c_int as libc::c_uint),
        ) as mdbglvl_et;
    }
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_MOVEINFO as libc::c_int as libc::c_uint
        != 0
    {
        (*ctrl)
            .dbglvl = ::core::mem::transmute::<
            libc::c_uint,
            mdbglvl_et,
        >(
            ((*ctrl).dbglvl as libc::c_uint)
                .wrapping_sub(METIS_DBG_MOVEINFO as libc::c_int as libc::c_uint),
        ) as mdbglvl_et;
    }
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint
        != 0
    {
        (*ctrl).InitPartTmr -= gk_CPUSeconds();
    }
    libmetis__Setup2WayBalMultipliers(ctrl, graph, ntpwgts.as_mut_ptr());
    match (*ctrl).iptype as libc::c_uint {
        2 => {
            if (*graph).nedges == 0 as libc::c_int {
                libmetis__RandomBisection(ctrl, graph, ntpwgts.as_mut_ptr(), niparts);
            } else {
                libmetis__GrowBisection(ctrl, graph, ntpwgts.as_mut_ptr(), niparts);
            }
            libmetis__Compute2WayPartitionParams(ctrl, graph);
            libmetis__ConstructSeparator(ctrl, graph);
        }
        3 => {
            libmetis__GrowBisectionNode(ctrl, graph, ntpwgts.as_mut_ptr(), niparts);
        }
        _ => {
            gk_errexit(
                15 as libc::c_int,
                b"Unkown iptype of %d\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*ctrl).iptype as libc::c_uint,
            );
        }
    }
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_IPART as libc::c_int as libc::c_uint
        != 0
    {
        printf(
            b"Initial Sep: %d\n\0" as *const u8 as *const libc::c_char,
            (*graph).mincut,
        );
    }
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint
        != 0
    {
        (*ctrl).InitPartTmr += gk_CPUSeconds();
    }
    (*ctrl).dbglvl = dbglvl;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__RandomBisection(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut ntpwgts: *mut real_t,
    mut niparts: idx_t,
) {
    let mut i: idx_t = 0;
    let mut ii: idx_t = 0;
    let mut j: idx_t = 0;
    let mut k: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut pwgts: [idx_t; 2] = [0; 2];
    let mut zeromaxpwgt: idx_t = 0;
    let mut from: idx_t = 0;
    let mut me: idx_t = 0;
    let mut bestcut: idx_t = 0 as libc::c_int;
    let mut icut: idx_t = 0;
    let mut mincut: idx_t = 0;
    let mut inbfs: idx_t = 0;
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
    let mut vwgt: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut adjwgt: *mut idx_t = 0 as *mut idx_t;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    let mut perm: *mut idx_t = 0 as *mut idx_t;
    let mut bestwhere: *mut idx_t = 0 as *mut idx_t;
    libmetis__wspacepush(ctrl);
    nvtxs = (*graph).nvtxs;
    xadj = (*graph).xadj;
    vwgt = (*graph).vwgt;
    adjncy = (*graph).adjncy;
    adjwgt = (*graph).adjwgt;
    libmetis__Allocate2WayPartitionMemory(ctrl, graph);
    where_0 = (*graph).where_0;
    bestwhere = libmetis__iwspacemalloc(ctrl, nvtxs);
    perm = libmetis__iwspacemalloc(ctrl, nvtxs);
    zeromaxpwgt = (*((*ctrl).ubfactors).offset(0 as libc::c_int as isize)
        * *((*graph).tvwgt).offset(0 as libc::c_int as isize) as libc::c_float
        * *ntpwgts.offset(0 as libc::c_int as isize)) as idx_t;
    inbfs = 0 as libc::c_int;
    while inbfs < niparts {
        libmetis__iset(nvtxs as size_t, 1 as libc::c_int, where_0);
        if inbfs > 0 as libc::c_int {
            libmetis__irandArrayPermute(
                nvtxs,
                perm,
                nvtxs / 2 as libc::c_int,
                1 as libc::c_int,
            );
            pwgts[1 as libc::c_int
                as usize] = *((*graph).tvwgt).offset(0 as libc::c_int as isize);
            pwgts[0 as libc::c_int as usize] = 0 as libc::c_int;
            ii = 0 as libc::c_int;
            while ii < nvtxs {
                i = *perm.offset(ii as isize);
                if pwgts[0 as libc::c_int as usize] + *vwgt.offset(i as isize)
                    < zeromaxpwgt
                {
                    *where_0.offset(i as isize) = 0 as libc::c_int;
                    pwgts[0 as libc::c_int as usize] += *vwgt.offset(i as isize);
                    pwgts[1 as libc::c_int as usize] -= *vwgt.offset(i as isize);
                    if pwgts[0 as libc::c_int as usize] > zeromaxpwgt {
                        break;
                    }
                }
                ii += 1;
                ii;
            }
        }
        libmetis__Compute2WayPartitionParams(ctrl, graph);
        libmetis__Balance2Way(ctrl, graph, ntpwgts);
        libmetis__FM_2WayRefine(ctrl, graph, ntpwgts, 4 as libc::c_int);
        if inbfs == 0 as libc::c_int || bestcut > (*graph).mincut {
            bestcut = (*graph).mincut;
            libmetis__icopy(nvtxs as size_t, where_0, bestwhere);
            if bestcut == 0 as libc::c_int {
                break;
            }
        }
        inbfs += 1;
        inbfs;
    }
    (*graph).mincut = bestcut;
    libmetis__icopy(nvtxs as size_t, bestwhere, where_0);
    libmetis__wspacepop(ctrl);
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__GrowBisection(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut ntpwgts: *mut real_t,
    mut niparts: idx_t,
) {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut k: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut drain: idx_t = 0;
    let mut nleft: idx_t = 0;
    let mut first: idx_t = 0;
    let mut last: idx_t = 0;
    let mut pwgts: [idx_t; 2] = [0; 2];
    let mut oneminpwgt: idx_t = 0;
    let mut onemaxpwgt: idx_t = 0;
    let mut from: idx_t = 0;
    let mut me: idx_t = 0;
    let mut bestcut: idx_t = 0 as libc::c_int;
    let mut icut: idx_t = 0;
    let mut mincut: idx_t = 0;
    let mut inbfs: idx_t = 0;
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
    let mut vwgt: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut adjwgt: *mut idx_t = 0 as *mut idx_t;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    let mut queue: *mut idx_t = 0 as *mut idx_t;
    let mut touched: *mut idx_t = 0 as *mut idx_t;
    let mut gain: *mut idx_t = 0 as *mut idx_t;
    let mut bestwhere: *mut idx_t = 0 as *mut idx_t;
    libmetis__wspacepush(ctrl);
    nvtxs = (*graph).nvtxs;
    xadj = (*graph).xadj;
    vwgt = (*graph).vwgt;
    adjncy = (*graph).adjncy;
    adjwgt = (*graph).adjwgt;
    libmetis__Allocate2WayPartitionMemory(ctrl, graph);
    where_0 = (*graph).where_0;
    bestwhere = libmetis__iwspacemalloc(ctrl, nvtxs);
    queue = libmetis__iwspacemalloc(ctrl, nvtxs);
    touched = libmetis__iwspacemalloc(ctrl, nvtxs);
    onemaxpwgt = (*((*ctrl).ubfactors).offset(0 as libc::c_int as isize)
        * *((*graph).tvwgt).offset(0 as libc::c_int as isize) as libc::c_float
        * *ntpwgts.offset(1 as libc::c_int as isize)) as idx_t;
    oneminpwgt = (1.0f64
        / *((*ctrl).ubfactors).offset(0 as libc::c_int as isize) as libc::c_double
        * *((*graph).tvwgt).offset(0 as libc::c_int as isize) as libc::c_double
        * *ntpwgts.offset(1 as libc::c_int as isize) as libc::c_double) as idx_t;
    inbfs = 0 as libc::c_int;
    while inbfs < niparts {
        libmetis__iset(nvtxs as size_t, 1 as libc::c_int, where_0);
        libmetis__iset(nvtxs as size_t, 0 as libc::c_int, touched);
        pwgts[1 as libc::c_int
            as usize] = *((*graph).tvwgt).offset(0 as libc::c_int as isize);
        pwgts[0 as libc::c_int as usize] = 0 as libc::c_int;
        *queue.offset(0 as libc::c_int as isize) = libmetis__irandInRange(nvtxs);
        *touched
            .offset(
                *queue.offset(0 as libc::c_int as isize) as isize,
            ) = 1 as libc::c_int;
        first = 0 as libc::c_int;
        last = 1 as libc::c_int;
        nleft = nvtxs - 1 as libc::c_int;
        drain = 0 as libc::c_int;
        loop {
            if first == last {
                if nleft == 0 as libc::c_int || drain != 0 {
                    break;
                }
                k = libmetis__irandInRange(nleft);
                i = 0 as libc::c_int;
                while i < nvtxs {
                    if *touched.offset(i as isize) == 0 as libc::c_int {
                        if k == 0 as libc::c_int {
                            break;
                        }
                        k -= 1;
                        k;
                    }
                    i += 1;
                    i;
                }
                *queue.offset(0 as libc::c_int as isize) = i;
                *touched.offset(i as isize) = 1 as libc::c_int;
                first = 0 as libc::c_int;
                last = 1 as libc::c_int;
                nleft -= 1;
                nleft;
            }
            let fresh0 = first;
            first = first + 1;
            i = *queue.offset(fresh0 as isize);
            if pwgts[0 as libc::c_int as usize] > 0 as libc::c_int
                && pwgts[1 as libc::c_int as usize] - *vwgt.offset(i as isize)
                    < oneminpwgt
            {
                drain = 1 as libc::c_int;
            } else {
                *where_0.offset(i as isize) = 0 as libc::c_int;
                pwgts[0 as libc::c_int as usize] += *vwgt.offset(i as isize);
                pwgts[1 as libc::c_int as usize] -= *vwgt.offset(i as isize);
                if pwgts[1 as libc::c_int as usize] <= onemaxpwgt {
                    break;
                }
                drain = 0 as libc::c_int;
                j = *xadj.offset(i as isize);
                while j < *xadj.offset((i + 1 as libc::c_int) as isize) {
                    k = *adjncy.offset(j as isize);
                    if *touched.offset(k as isize) == 0 as libc::c_int {
                        let fresh1 = last;
                        last = last + 1;
                        *queue.offset(fresh1 as isize) = k;
                        *touched.offset(k as isize) = 1 as libc::c_int;
                        nleft -= 1;
                        nleft;
                    }
                    j += 1;
                    j;
                }
            }
        }
        if pwgts[1 as libc::c_int as usize] == 0 as libc::c_int {
            *where_0.offset(libmetis__irandInRange(nvtxs) as isize) = 1 as libc::c_int;
        }
        if pwgts[0 as libc::c_int as usize] == 0 as libc::c_int {
            *where_0.offset(libmetis__irandInRange(nvtxs) as isize) = 0 as libc::c_int;
        }
        libmetis__Compute2WayPartitionParams(ctrl, graph);
        libmetis__Balance2Way(ctrl, graph, ntpwgts);
        libmetis__FM_2WayRefine(ctrl, graph, ntpwgts, (*ctrl).niter);
        if inbfs == 0 as libc::c_int || bestcut > (*graph).mincut {
            bestcut = (*graph).mincut;
            libmetis__icopy(nvtxs as size_t, where_0, bestwhere);
            if bestcut == 0 as libc::c_int {
                break;
            }
        }
        inbfs += 1;
        inbfs;
    }
    (*graph).mincut = bestcut;
    libmetis__icopy(nvtxs as size_t, bestwhere, where_0);
    libmetis__wspacepop(ctrl);
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__McRandomBisection(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut ntpwgts: *mut real_t,
    mut niparts: idx_t,
) {
    let mut i: idx_t = 0;
    let mut ii: idx_t = 0;
    let mut j: idx_t = 0;
    let mut k: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut ncon: idx_t = 0;
    let mut from: idx_t = 0;
    let mut bestcut: idx_t = 0 as libc::c_int;
    let mut mincut: idx_t = 0;
    let mut inbfs: idx_t = 0;
    let mut qnum: idx_t = 0;
    let mut bestwhere: *mut idx_t = 0 as *mut idx_t;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    let mut perm: *mut idx_t = 0 as *mut idx_t;
    let mut counts: *mut idx_t = 0 as *mut idx_t;
    let mut vwgt: *mut idx_t = 0 as *mut idx_t;
    libmetis__wspacepush(ctrl);
    nvtxs = (*graph).nvtxs;
    ncon = (*graph).ncon;
    vwgt = (*graph).vwgt;
    libmetis__Allocate2WayPartitionMemory(ctrl, graph);
    where_0 = (*graph).where_0;
    bestwhere = libmetis__iwspacemalloc(ctrl, nvtxs);
    perm = libmetis__iwspacemalloc(ctrl, nvtxs);
    counts = libmetis__iwspacemalloc(ctrl, ncon);
    inbfs = 0 as libc::c_int;
    while inbfs < 2 as libc::c_int * niparts {
        libmetis__irandArrayPermute(
            nvtxs,
            perm,
            nvtxs / 2 as libc::c_int,
            1 as libc::c_int,
        );
        libmetis__iset(ncon as size_t, 0 as libc::c_int, counts);
        ii = 0 as libc::c_int;
        while ii < nvtxs {
            i = *perm.offset(ii as isize);
            qnum = libmetis__iargmax(ncon as size_t, vwgt.offset((i * ncon) as isize))
                as idx_t;
            let ref mut fresh2 = *counts.offset(qnum as isize);
            let fresh3 = *fresh2;
            *fresh2 = *fresh2 + 1;
            *where_0.offset(i as isize) = fresh3 % 2 as libc::c_int;
            ii += 1;
            ii;
        }
        libmetis__Compute2WayPartitionParams(ctrl, graph);
        libmetis__FM_2WayRefine(ctrl, graph, ntpwgts, (*ctrl).niter);
        libmetis__Balance2Way(ctrl, graph, ntpwgts);
        libmetis__FM_2WayRefine(ctrl, graph, ntpwgts, (*ctrl).niter);
        libmetis__Balance2Way(ctrl, graph, ntpwgts);
        libmetis__FM_2WayRefine(ctrl, graph, ntpwgts, (*ctrl).niter);
        if inbfs == 0 as libc::c_int || bestcut >= (*graph).mincut {
            bestcut = (*graph).mincut;
            libmetis__icopy(nvtxs as size_t, where_0, bestwhere);
            if bestcut == 0 as libc::c_int {
                break;
            }
        }
        inbfs += 1;
        inbfs;
    }
    (*graph).mincut = bestcut;
    libmetis__icopy(nvtxs as size_t, bestwhere, where_0);
    libmetis__wspacepop(ctrl);
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__McGrowBisection(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut ntpwgts: *mut real_t,
    mut niparts: idx_t,
) {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut k: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut ncon: idx_t = 0;
    let mut from: idx_t = 0;
    let mut bestcut: idx_t = 0 as libc::c_int;
    let mut mincut: idx_t = 0;
    let mut inbfs: idx_t = 0;
    let mut bestwhere: *mut idx_t = 0 as *mut idx_t;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    libmetis__wspacepush(ctrl);
    nvtxs = (*graph).nvtxs;
    libmetis__Allocate2WayPartitionMemory(ctrl, graph);
    where_0 = (*graph).where_0;
    bestwhere = libmetis__iwspacemalloc(ctrl, nvtxs);
    inbfs = 0 as libc::c_int;
    while inbfs < 2 as libc::c_int * niparts {
        libmetis__iset(nvtxs as size_t, 1 as libc::c_int, where_0);
        *where_0.offset(libmetis__irandInRange(nvtxs) as isize) = 0 as libc::c_int;
        libmetis__Compute2WayPartitionParams(ctrl, graph);
        libmetis__Balance2Way(ctrl, graph, ntpwgts);
        libmetis__FM_2WayRefine(ctrl, graph, ntpwgts, (*ctrl).niter);
        libmetis__Balance2Way(ctrl, graph, ntpwgts);
        libmetis__FM_2WayRefine(ctrl, graph, ntpwgts, (*ctrl).niter);
        if inbfs == 0 as libc::c_int || bestcut >= (*graph).mincut {
            bestcut = (*graph).mincut;
            libmetis__icopy(nvtxs as size_t, where_0, bestwhere);
            if bestcut == 0 as libc::c_int {
                break;
            }
        }
        inbfs += 1;
        inbfs;
    }
    (*graph).mincut = bestcut;
    libmetis__icopy(nvtxs as size_t, bestwhere, where_0);
    libmetis__wspacepop(ctrl);
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__GrowBisectionNode(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut ntpwgts: *mut real_t,
    mut niparts: idx_t,
) {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut k: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut drain: idx_t = 0;
    let mut nleft: idx_t = 0;
    let mut first: idx_t = 0;
    let mut last: idx_t = 0;
    let mut pwgts: [idx_t; 2] = [0; 2];
    let mut oneminpwgt: idx_t = 0;
    let mut onemaxpwgt: idx_t = 0;
    let mut from: idx_t = 0;
    let mut me: idx_t = 0;
    let mut bestcut: idx_t = 0 as libc::c_int;
    let mut icut: idx_t = 0;
    let mut mincut: idx_t = 0;
    let mut inbfs: idx_t = 0;
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
    let mut vwgt: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut adjwgt: *mut idx_t = 0 as *mut idx_t;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    let mut bndind: *mut idx_t = 0 as *mut idx_t;
    let mut queue: *mut idx_t = 0 as *mut idx_t;
    let mut touched: *mut idx_t = 0 as *mut idx_t;
    let mut gain: *mut idx_t = 0 as *mut idx_t;
    let mut bestwhere: *mut idx_t = 0 as *mut idx_t;
    libmetis__wspacepush(ctrl);
    nvtxs = (*graph).nvtxs;
    xadj = (*graph).xadj;
    vwgt = (*graph).vwgt;
    adjncy = (*graph).adjncy;
    adjwgt = (*graph).adjwgt;
    bestwhere = libmetis__iwspacemalloc(ctrl, nvtxs);
    queue = libmetis__iwspacemalloc(ctrl, nvtxs);
    touched = libmetis__iwspacemalloc(ctrl, nvtxs);
    onemaxpwgt = ((*((*ctrl).ubfactors).offset(0 as libc::c_int as isize)
        * *((*graph).tvwgt).offset(0 as libc::c_int as isize) as libc::c_float)
        as libc::c_double * 0.5f64) as idx_t;
    oneminpwgt = (1.0f64
        / *((*ctrl).ubfactors).offset(0 as libc::c_int as isize) as libc::c_double
        * *((*graph).tvwgt).offset(0 as libc::c_int as isize) as libc::c_double * 0.5f64)
        as idx_t;
    (*graph)
        .pwgts = libmetis__imalloc(
        3 as libc::c_int as size_t,
        b"GrowBisectionNode: pwgts\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*graph)
        .where_0 = libmetis__imalloc(
        nvtxs as size_t,
        b"GrowBisectionNode: where\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*graph)
        .bndptr = libmetis__imalloc(
        nvtxs as size_t,
        b"GrowBisectionNode: bndptr\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*graph)
        .bndind = libmetis__imalloc(
        nvtxs as size_t,
        b"GrowBisectionNode: bndind\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*graph)
        .id = libmetis__imalloc(
        nvtxs as size_t,
        b"GrowBisectionNode: id\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*graph)
        .ed = libmetis__imalloc(
        nvtxs as size_t,
        b"GrowBisectionNode: ed\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*graph)
        .nrinfo = gk_malloc(
        (nvtxs as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<nrinfo_t>() as libc::c_ulong),
        b"GrowBisectionNode: nrinfo\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    ) as *mut nrinfo_t;
    where_0 = (*graph).where_0;
    bndind = (*graph).bndind;
    inbfs = 0 as libc::c_int;
    while inbfs < niparts {
        libmetis__iset(nvtxs as size_t, 1 as libc::c_int, where_0);
        libmetis__iset(nvtxs as size_t, 0 as libc::c_int, touched);
        pwgts[1 as libc::c_int
            as usize] = *((*graph).tvwgt).offset(0 as libc::c_int as isize);
        pwgts[0 as libc::c_int as usize] = 0 as libc::c_int;
        *queue.offset(0 as libc::c_int as isize) = libmetis__irandInRange(nvtxs);
        *touched
            .offset(
                *queue.offset(0 as libc::c_int as isize) as isize,
            ) = 1 as libc::c_int;
        first = 0 as libc::c_int;
        last = 1 as libc::c_int;
        nleft = nvtxs - 1 as libc::c_int;
        drain = 0 as libc::c_int;
        loop {
            if first == last {
                if nleft == 0 as libc::c_int || drain != 0 {
                    break;
                }
                k = libmetis__irandInRange(nleft);
                i = 0 as libc::c_int;
                while i < nvtxs {
                    if *touched.offset(i as isize) == 0 as libc::c_int {
                        if k == 0 as libc::c_int {
                            break;
                        }
                        k -= 1;
                        k;
                    }
                    i += 1;
                    i;
                }
                *queue.offset(0 as libc::c_int as isize) = i;
                *touched.offset(i as isize) = 1 as libc::c_int;
                first = 0 as libc::c_int;
                last = 1 as libc::c_int;
                nleft -= 1;
                nleft;
            }
            let fresh4 = first;
            first = first + 1;
            i = *queue.offset(fresh4 as isize);
            if pwgts[1 as libc::c_int as usize] - *vwgt.offset(i as isize) < oneminpwgt {
                drain = 1 as libc::c_int;
            } else {
                *where_0.offset(i as isize) = 0 as libc::c_int;
                pwgts[0 as libc::c_int as usize] += *vwgt.offset(i as isize);
                pwgts[1 as libc::c_int as usize] -= *vwgt.offset(i as isize);
                if pwgts[1 as libc::c_int as usize] <= onemaxpwgt {
                    break;
                }
                drain = 0 as libc::c_int;
                j = *xadj.offset(i as isize);
                while j < *xadj.offset((i + 1 as libc::c_int) as isize) {
                    k = *adjncy.offset(j as isize);
                    if *touched.offset(k as isize) == 0 as libc::c_int {
                        let fresh5 = last;
                        last = last + 1;
                        *queue.offset(fresh5 as isize) = k;
                        *touched.offset(k as isize) = 1 as libc::c_int;
                        nleft -= 1;
                        nleft;
                    }
                    j += 1;
                    j;
                }
            }
        }
        libmetis__Compute2WayPartitionParams(ctrl, graph);
        libmetis__Balance2Way(ctrl, graph, ntpwgts);
        libmetis__FM_2WayRefine(ctrl, graph, ntpwgts, 4 as libc::c_int);
        i = 0 as libc::c_int;
        while i < (*graph).nbnd {
            j = *bndind.offset(i as isize);
            if *xadj.offset((j + 1 as libc::c_int) as isize) - *xadj.offset(j as isize)
                > 0 as libc::c_int
            {
                *where_0.offset(j as isize) = 2 as libc::c_int;
            }
            i += 1;
            i;
        }
        libmetis__Compute2WayNodePartitionParams(ctrl, graph);
        libmetis__FM_2WayNodeRefine2Sided(ctrl, graph, 1 as libc::c_int);
        libmetis__FM_2WayNodeRefine1Sided(ctrl, graph, 4 as libc::c_int);
        if inbfs == 0 as libc::c_int || bestcut > (*graph).mincut {
            bestcut = (*graph).mincut;
            libmetis__icopy(nvtxs as size_t, where_0, bestwhere);
        }
        inbfs += 1;
        inbfs;
    }
    (*graph).mincut = bestcut;
    libmetis__icopy(nvtxs as size_t, bestwhere, where_0);
    libmetis__wspacepop(ctrl);
}
#[no_mangle]
pub unsafe extern "C" fn GrowBisectionNode2(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut ntpwgts: *mut real_t,
    mut niparts: idx_t,
) {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut k: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut bestcut: idx_t = 0 as libc::c_int;
    let mut mincut: idx_t = 0;
    let mut inbfs: idx_t = 0;
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    let mut bndind: *mut idx_t = 0 as *mut idx_t;
    let mut bestwhere: *mut idx_t = 0 as *mut idx_t;
    libmetis__wspacepush(ctrl);
    nvtxs = (*graph).nvtxs;
    xadj = (*graph).xadj;
    (*graph)
        .pwgts = libmetis__imalloc(
        3 as libc::c_int as size_t,
        b"GrowBisectionNode: pwgts\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*graph)
        .where_0 = libmetis__imalloc(
        nvtxs as size_t,
        b"GrowBisectionNode: where\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*graph)
        .bndptr = libmetis__imalloc(
        nvtxs as size_t,
        b"GrowBisectionNode: bndptr\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*graph)
        .bndind = libmetis__imalloc(
        nvtxs as size_t,
        b"GrowBisectionNode: bndind\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*graph)
        .id = libmetis__imalloc(
        nvtxs as size_t,
        b"GrowBisectionNode: id\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*graph)
        .ed = libmetis__imalloc(
        nvtxs as size_t,
        b"GrowBisectionNode: ed\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*graph)
        .nrinfo = gk_malloc(
        (nvtxs as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<nrinfo_t>() as libc::c_ulong),
        b"GrowBisectionNode: nrinfo\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    ) as *mut nrinfo_t;
    bestwhere = libmetis__iwspacemalloc(ctrl, nvtxs);
    where_0 = (*graph).where_0;
    bndind = (*graph).bndind;
    inbfs = 0 as libc::c_int;
    while inbfs < niparts {
        libmetis__iset(nvtxs as size_t, 1 as libc::c_int, where_0);
        if inbfs > 0 as libc::c_int {
            *where_0.offset(libmetis__irandInRange(nvtxs) as isize) = 0 as libc::c_int;
        }
        libmetis__Compute2WayPartitionParams(ctrl, graph);
        libmetis__General2WayBalance(ctrl, graph, ntpwgts);
        libmetis__FM_2WayRefine(ctrl, graph, ntpwgts, (*ctrl).niter);
        i = 0 as libc::c_int;
        while i < (*graph).nbnd {
            j = *bndind.offset(i as isize);
            if *xadj.offset((j + 1 as libc::c_int) as isize) - *xadj.offset(j as isize)
                > 0 as libc::c_int
            {
                *where_0.offset(j as isize) = 2 as libc::c_int;
            }
            i += 1;
            i;
        }
        libmetis__Compute2WayNodePartitionParams(ctrl, graph);
        libmetis__FM_2WayNodeRefine2Sided(ctrl, graph, 4 as libc::c_int);
        if inbfs == 0 as libc::c_int || bestcut > (*graph).mincut {
            bestcut = (*graph).mincut;
            libmetis__icopy(nvtxs as size_t, where_0, bestwhere);
        }
        inbfs += 1;
        inbfs;
    }
    (*graph).mincut = bestcut;
    libmetis__icopy(nvtxs as size_t, bestwhere, where_0);
    libmetis__wspacepop(ctrl);
}
