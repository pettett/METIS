use ::libc;
extern "C" {
    fn abs(_: libc::c_int) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn libmetis__ComputeLoadImbalanceDiff(
        graph: *mut graph_t,
        nparts: idx_t,
        pijbm: *mut real_t,
        ubvec: *mut real_t,
    ) -> real_t;
    fn libmetis__wspacepush(ctrl: *mut ctrl_t);
    fn libmetis__rwspacemalloc(_: *mut ctrl_t, _: idx_t) -> *mut real_t;
    fn libmetis__iwspacemalloc(_: *mut ctrl_t, _: idx_t) -> *mut idx_t;
    fn libmetis__wspacemalloc(ctrl: *mut ctrl_t, nbytes: size_t) -> *mut libc::c_void;
    fn libmetis__iargmax_nrm(n: size_t, x: *mut idx_t, y: *mut real_t) -> idx_t;
    fn libmetis__iargmax2_nrm(n: size_t, x: *mut idx_t, y: *mut real_t) -> idx_t;
    fn libmetis__SelectQueue(
        graph: *mut graph_t,
        pijbm: *mut real_t,
        ubfactors: *mut real_t,
        queues: *mut *mut rpq_t,
        from: *mut idx_t,
        cnum: *mut idx_t,
    );
    fn libmetis__ComputeLoadImbalanceDiffVec(
        graph: *mut graph_t,
        nparts: idx_t,
        pijbm: *mut real_t,
        ubfactors: *mut real_t,
        diffvec: *mut real_t,
    ) -> real_t;
    fn libmetis__BetterBalance2Way(
        n: idx_t,
        x: *mut real_t,
        y: *mut real_t,
    ) -> libc::c_int;
    fn libmetis__ComputeLoadImbalance(
        graph: *mut graph_t,
        nparts: idx_t,
        pijbm: *mut real_t,
    ) -> real_t;
    fn libmetis__wspacepop(ctrl: *mut ctrl_t);
    fn libmetis__irandArrayPermute(
        n: idx_t,
        p: *mut idx_t,
        nshuffles: idx_t,
        flag: libc::c_int,
    );
    fn libmetis__rpqGetTop(queue: *mut rpq_t) -> idx_t;
    fn libmetis__rpqUpdate(queue: *mut rpq_t, node: idx_t, newkey: real_t);
    fn libmetis__rpqDelete(queue: *mut rpq_t, node: idx_t) -> libc::c_int;
    fn libmetis__rpqInsert(queue: *mut rpq_t, node: idx_t, key: real_t) -> libc::c_int;
    fn libmetis__rpqDestroy(queue: *mut rpq_t);
    fn libmetis__rpqCreate(maxnodes: size_t) -> *mut rpq_t;
    fn libmetis__rcopy(n: size_t, a: *mut real_t, b: *mut real_t) -> *mut real_t;
    fn libmetis__iset(n: size_t, val: idx_t, x: *mut idx_t) -> *mut idx_t;
    fn libmetis__iaxpy(
        n: size_t,
        alpha: idx_t,
        x: *mut idx_t,
        incx: size_t,
        y: *mut idx_t,
        incy: size_t,
    ) -> *mut idx_t;
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
pub struct rkv_t {
    pub key: real_t,
    pub val: idx_t,
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
pub unsafe extern "C" fn libmetis__Balance2Way(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut ntpwgts: *mut real_t,
) {
    if libmetis__ComputeLoadImbalanceDiff(
        graph,
        2 as libc::c_int,
        (*ctrl).pijbm,
        (*ctrl).ubfactors,
    ) <= 0 as libc::c_int as libc::c_float
    {
        return;
    }
    if (*graph).ncon == 1 as libc::c_int {
        if abs(
            (*ntpwgts.offset(0 as libc::c_int as isize)
                * *((*graph).tvwgt).offset(0 as libc::c_int as isize) as libc::c_float
                - *((*graph).pwgts).offset(0 as libc::c_int as isize) as libc::c_float)
                as libc::c_int,
        )
            < 3 as libc::c_int * *((*graph).tvwgt).offset(0 as libc::c_int as isize)
                / (*graph).nvtxs
        {
            return;
        }
        if (*graph).nbnd > 0 as libc::c_int {
            libmetis__Bnd2WayBalance(ctrl, graph, ntpwgts);
        } else {
            libmetis__General2WayBalance(ctrl, graph, ntpwgts);
        }
    } else {
        libmetis__McGeneral2WayBalance(ctrl, graph, ntpwgts);
    };
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__Bnd2WayBalance(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut ntpwgts: *mut real_t,
) {
    let mut i: idx_t = 0;
    let mut ii: idx_t = 0;
    let mut j: idx_t = 0;
    let mut k: idx_t = 0;
    let mut kwgt: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut nbnd: idx_t = 0;
    let mut nswaps: idx_t = 0;
    let mut from: idx_t = 0;
    let mut to: idx_t = 0;
    let mut pass: idx_t = 0;
    let mut me: idx_t = 0;
    let mut tmp: idx_t = 0;
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
    let mut vwgt: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut adjwgt: *mut idx_t = 0 as *mut idx_t;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    let mut id: *mut idx_t = 0 as *mut idx_t;
    let mut ed: *mut idx_t = 0 as *mut idx_t;
    let mut bndptr: *mut idx_t = 0 as *mut idx_t;
    let mut bndind: *mut idx_t = 0 as *mut idx_t;
    let mut pwgts: *mut idx_t = 0 as *mut idx_t;
    let mut moved: *mut idx_t = 0 as *mut idx_t;
    let mut perm: *mut idx_t = 0 as *mut idx_t;
    let mut queue: *mut rpq_t = 0 as *mut rpq_t;
    let mut higain: idx_t = 0;
    let mut mincut: idx_t = 0;
    let mut mindiff: idx_t = 0;
    let mut tpwgts: [idx_t; 2] = [0; 2];
    libmetis__wspacepush(ctrl);
    nvtxs = (*graph).nvtxs;
    xadj = (*graph).xadj;
    vwgt = (*graph).vwgt;
    adjncy = (*graph).adjncy;
    adjwgt = (*graph).adjwgt;
    where_0 = (*graph).where_0;
    id = (*graph).id;
    ed = (*graph).ed;
    pwgts = (*graph).pwgts;
    bndptr = (*graph).bndptr;
    bndind = (*graph).bndind;
    moved = libmetis__iwspacemalloc(ctrl, nvtxs);
    perm = libmetis__iwspacemalloc(ctrl, nvtxs);
    tpwgts[0 as libc::c_int
        as usize] = (*((*graph).tvwgt).offset(0 as libc::c_int as isize) as libc::c_float
        * *ntpwgts.offset(0 as libc::c_int as isize)) as idx_t;
    tpwgts[1 as libc::c_int
        as usize] = *((*graph).tvwgt).offset(0 as libc::c_int as isize)
        - tpwgts[0 as libc::c_int as usize];
    mindiff = abs(
        tpwgts[0 as libc::c_int as usize] - *pwgts.offset(0 as libc::c_int as isize),
    );
    from = if *pwgts.offset(0 as libc::c_int as isize)
        < tpwgts[0 as libc::c_int as usize]
    {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    to = (from + 1 as libc::c_int) % 2 as libc::c_int;
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_REFINE as libc::c_int as libc::c_uint
        != 0
    {
        printf(
            b"Partitions: [%6d %6d] T[%6d %6d], Nv-Nb[%6d %6d]. ICut: %6d [B]\n\0"
                as *const u8 as *const libc::c_char,
            *pwgts.offset(0 as libc::c_int as isize),
            *pwgts.offset(1 as libc::c_int as isize),
            tpwgts[0 as libc::c_int as usize],
            tpwgts[1 as libc::c_int as usize],
            (*graph).nvtxs,
            (*graph).nbnd,
            (*graph).mincut,
        );
    }
    queue = libmetis__rpqCreate(nvtxs as size_t);
    libmetis__iset(nvtxs as size_t, -(1 as libc::c_int), moved);
    nbnd = (*graph).nbnd;
    libmetis__irandArrayPermute(nbnd, perm, nbnd / 5 as libc::c_int, 1 as libc::c_int);
    ii = 0 as libc::c_int;
    while ii < nbnd {
        i = *perm.offset(ii as isize);
        if *where_0.offset(*bndind.offset(i as isize) as isize) == from
            && *vwgt.offset(*bndind.offset(i as isize) as isize) <= mindiff
        {
            libmetis__rpqInsert(
                queue,
                *bndind.offset(i as isize),
                (*ed.offset(*bndind.offset(i as isize) as isize)
                    - *id.offset(*bndind.offset(i as isize) as isize)) as real_t,
            );
        }
        ii += 1;
        ii;
    }
    mincut = (*graph).mincut;
    nswaps = 0 as libc::c_int;
    while nswaps < nvtxs {
        higain = libmetis__rpqGetTop(queue);
        if higain == -(1 as libc::c_int) {
            break;
        }
        if *pwgts.offset(to as isize) + *vwgt.offset(higain as isize)
            > tpwgts[to as usize]
        {
            break;
        }
        mincut -= *ed.offset(higain as isize) - *id.offset(higain as isize);
        let ref mut fresh0 = *pwgts.offset(to as isize);
        *fresh0 += *vwgt.offset(higain as isize);
        let ref mut fresh1 = *pwgts.offset(from as isize);
        *fresh1 -= *vwgt.offset(higain as isize);
        *where_0.offset(higain as isize) = to;
        *moved.offset(higain as isize) = nswaps;
        if (*ctrl).dbglvl as libc::c_uint
            & METIS_DBG_MOVEINFO as libc::c_int as libc::c_uint != 0
        {
            printf(
                b"Moved %6d from %d. [%3d %3d] %5d [%4d %4d]\n\0" as *const u8
                    as *const libc::c_char,
                higain,
                from,
                *ed.offset(higain as isize) - *id.offset(higain as isize),
                *vwgt.offset(higain as isize),
                mincut,
                *pwgts.offset(0 as libc::c_int as isize),
                *pwgts.offset(1 as libc::c_int as isize),
            );
        }
        tmp = *id.offset(higain as isize);
        *id.offset(higain as isize) = *ed.offset(higain as isize);
        *ed.offset(higain as isize) = tmp;
        if *ed.offset(higain as isize) == 0 as libc::c_int
            && *xadj.offset(higain as isize)
                < *xadj.offset((higain + 1 as libc::c_int) as isize)
        {
            nbnd -= 1;
            *bndind
                .offset(
                    *bndptr.offset(higain as isize) as isize,
                ) = *bndind.offset(nbnd as isize);
            *bndptr
                .offset(
                    *bndind.offset(nbnd as isize) as isize,
                ) = *bndptr.offset(higain as isize);
            *bndptr.offset(higain as isize) = -(1 as libc::c_int);
        }
        j = *xadj.offset(higain as isize);
        while j < *xadj.offset((higain + 1 as libc::c_int) as isize) {
            k = *adjncy.offset(j as isize);
            kwgt = if to == *where_0.offset(k as isize) {
                *adjwgt.offset(j as isize)
            } else {
                -*adjwgt.offset(j as isize)
            };
            let ref mut fresh2 = *id.offset(k as isize);
            *fresh2 += kwgt;
            let ref mut fresh3 = *ed.offset(k as isize);
            *fresh3 -= kwgt;
            if *bndptr.offset(k as isize) != -(1 as libc::c_int) {
                if *ed.offset(k as isize) == 0 as libc::c_int {
                    nbnd -= 1;
                    *bndind
                        .offset(
                            *bndptr.offset(k as isize) as isize,
                        ) = *bndind.offset(nbnd as isize);
                    *bndptr
                        .offset(
                            *bndind.offset(nbnd as isize) as isize,
                        ) = *bndptr.offset(k as isize);
                    *bndptr.offset(k as isize) = -(1 as libc::c_int);
                    if *moved.offset(k as isize) == -(1 as libc::c_int)
                        && *where_0.offset(k as isize) == from
                        && *vwgt.offset(k as isize) <= mindiff
                    {
                        libmetis__rpqDelete(queue, k);
                    }
                } else if *moved.offset(k as isize) == -(1 as libc::c_int)
                    && *where_0.offset(k as isize) == from
                    && *vwgt.offset(k as isize) <= mindiff
                {
                    libmetis__rpqUpdate(
                        queue,
                        k,
                        (*ed.offset(k as isize) - *id.offset(k as isize)) as real_t,
                    );
                }
            } else if *ed.offset(k as isize) > 0 as libc::c_int {
                *bndind.offset(nbnd as isize) = k;
                let fresh4 = nbnd;
                nbnd = nbnd + 1;
                *bndptr.offset(k as isize) = fresh4;
                if *moved.offset(k as isize) == -(1 as libc::c_int)
                    && *where_0.offset(k as isize) == from
                    && *vwgt.offset(k as isize) <= mindiff
                {
                    libmetis__rpqInsert(
                        queue,
                        k,
                        (*ed.offset(k as isize) - *id.offset(k as isize)) as real_t,
                    );
                }
            }
            j += 1;
            j;
        }
        nswaps += 1;
        nswaps;
    }
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_REFINE as libc::c_int as libc::c_uint
        != 0
    {
        printf(
            b"\tMinimum cut: %6d, PWGTS: [%6d %6d], NBND: %6d\n\0" as *const u8
                as *const libc::c_char,
            mincut,
            *pwgts.offset(0 as libc::c_int as isize),
            *pwgts.offset(1 as libc::c_int as isize),
            nbnd,
        );
    }
    (*graph).mincut = mincut;
    (*graph).nbnd = nbnd;
    libmetis__rpqDestroy(queue);
    libmetis__wspacepop(ctrl);
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__General2WayBalance(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut ntpwgts: *mut real_t,
) {
    let mut i: idx_t = 0;
    let mut ii: idx_t = 0;
    let mut j: idx_t = 0;
    let mut k: idx_t = 0;
    let mut kwgt: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut nbnd: idx_t = 0;
    let mut nswaps: idx_t = 0;
    let mut from: idx_t = 0;
    let mut to: idx_t = 0;
    let mut pass: idx_t = 0;
    let mut me: idx_t = 0;
    let mut tmp: idx_t = 0;
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
    let mut vwgt: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut adjwgt: *mut idx_t = 0 as *mut idx_t;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    let mut id: *mut idx_t = 0 as *mut idx_t;
    let mut ed: *mut idx_t = 0 as *mut idx_t;
    let mut bndptr: *mut idx_t = 0 as *mut idx_t;
    let mut bndind: *mut idx_t = 0 as *mut idx_t;
    let mut pwgts: *mut idx_t = 0 as *mut idx_t;
    let mut moved: *mut idx_t = 0 as *mut idx_t;
    let mut perm: *mut idx_t = 0 as *mut idx_t;
    let mut queue: *mut rpq_t = 0 as *mut rpq_t;
    let mut higain: idx_t = 0;
    let mut mincut: idx_t = 0;
    let mut mindiff: idx_t = 0;
    let mut tpwgts: [idx_t; 2] = [0; 2];
    libmetis__wspacepush(ctrl);
    nvtxs = (*graph).nvtxs;
    xadj = (*graph).xadj;
    vwgt = (*graph).vwgt;
    adjncy = (*graph).adjncy;
    adjwgt = (*graph).adjwgt;
    where_0 = (*graph).where_0;
    id = (*graph).id;
    ed = (*graph).ed;
    pwgts = (*graph).pwgts;
    bndptr = (*graph).bndptr;
    bndind = (*graph).bndind;
    moved = libmetis__iwspacemalloc(ctrl, nvtxs);
    perm = libmetis__iwspacemalloc(ctrl, nvtxs);
    tpwgts[0 as libc::c_int
        as usize] = (*((*graph).tvwgt).offset(0 as libc::c_int as isize) as libc::c_float
        * *ntpwgts.offset(0 as libc::c_int as isize)) as idx_t;
    tpwgts[1 as libc::c_int
        as usize] = *((*graph).tvwgt).offset(0 as libc::c_int as isize)
        - tpwgts[0 as libc::c_int as usize];
    mindiff = abs(
        tpwgts[0 as libc::c_int as usize] - *pwgts.offset(0 as libc::c_int as isize),
    );
    from = if *pwgts.offset(0 as libc::c_int as isize)
        < tpwgts[0 as libc::c_int as usize]
    {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    to = (from + 1 as libc::c_int) % 2 as libc::c_int;
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_REFINE as libc::c_int as libc::c_uint
        != 0
    {
        printf(
            b"Partitions: [%6d %6d] T[%6d %6d], Nv-Nb[%6d %6d]. ICut: %6d [B]\n\0"
                as *const u8 as *const libc::c_char,
            *pwgts.offset(0 as libc::c_int as isize),
            *pwgts.offset(1 as libc::c_int as isize),
            tpwgts[0 as libc::c_int as usize],
            tpwgts[1 as libc::c_int as usize],
            (*graph).nvtxs,
            (*graph).nbnd,
            (*graph).mincut,
        );
    }
    queue = libmetis__rpqCreate(nvtxs as size_t);
    libmetis__iset(nvtxs as size_t, -(1 as libc::c_int), moved);
    libmetis__irandArrayPermute(nvtxs, perm, nvtxs / 5 as libc::c_int, 1 as libc::c_int);
    ii = 0 as libc::c_int;
    while ii < nvtxs {
        i = *perm.offset(ii as isize);
        if *where_0.offset(i as isize) == from && *vwgt.offset(i as isize) <= mindiff {
            libmetis__rpqInsert(
                queue,
                i,
                (*ed.offset(i as isize) - *id.offset(i as isize)) as real_t,
            );
        }
        ii += 1;
        ii;
    }
    mincut = (*graph).mincut;
    nbnd = (*graph).nbnd;
    nswaps = 0 as libc::c_int;
    while nswaps < nvtxs {
        higain = libmetis__rpqGetTop(queue);
        if higain == -(1 as libc::c_int) {
            break;
        }
        if *pwgts.offset(to as isize) + *vwgt.offset(higain as isize)
            > tpwgts[to as usize]
        {
            break;
        }
        mincut -= *ed.offset(higain as isize) - *id.offset(higain as isize);
        let ref mut fresh5 = *pwgts.offset(to as isize);
        *fresh5 += *vwgt.offset(higain as isize);
        let ref mut fresh6 = *pwgts.offset(from as isize);
        *fresh6 -= *vwgt.offset(higain as isize);
        *where_0.offset(higain as isize) = to;
        *moved.offset(higain as isize) = nswaps;
        if (*ctrl).dbglvl as libc::c_uint
            & METIS_DBG_MOVEINFO as libc::c_int as libc::c_uint != 0
        {
            printf(
                b"Moved %6d from %d. [%3d %3d] %5d [%4d %4d]\n\0" as *const u8
                    as *const libc::c_char,
                higain,
                from,
                *ed.offset(higain as isize) - *id.offset(higain as isize),
                *vwgt.offset(higain as isize),
                mincut,
                *pwgts.offset(0 as libc::c_int as isize),
                *pwgts.offset(1 as libc::c_int as isize),
            );
        }
        tmp = *id.offset(higain as isize);
        *id.offset(higain as isize) = *ed.offset(higain as isize);
        *ed.offset(higain as isize) = tmp;
        if *ed.offset(higain as isize) == 0 as libc::c_int
            && *bndptr.offset(higain as isize) != -(1 as libc::c_int)
            && *xadj.offset(higain as isize)
                < *xadj.offset((higain + 1 as libc::c_int) as isize)
        {
            nbnd -= 1;
            *bndind
                .offset(
                    *bndptr.offset(higain as isize) as isize,
                ) = *bndind.offset(nbnd as isize);
            *bndptr
                .offset(
                    *bndind.offset(nbnd as isize) as isize,
                ) = *bndptr.offset(higain as isize);
            *bndptr.offset(higain as isize) = -(1 as libc::c_int);
        }
        if *ed.offset(higain as isize) > 0 as libc::c_int
            && *bndptr.offset(higain as isize) == -(1 as libc::c_int)
        {
            *bndind.offset(nbnd as isize) = higain;
            let fresh7 = nbnd;
            nbnd = nbnd + 1;
            *bndptr.offset(higain as isize) = fresh7;
        }
        j = *xadj.offset(higain as isize);
        while j < *xadj.offset((higain + 1 as libc::c_int) as isize) {
            k = *adjncy.offset(j as isize);
            kwgt = if to == *where_0.offset(k as isize) {
                *adjwgt.offset(j as isize)
            } else {
                -*adjwgt.offset(j as isize)
            };
            let ref mut fresh8 = *id.offset(k as isize);
            *fresh8 += kwgt;
            let ref mut fresh9 = *ed.offset(k as isize);
            *fresh9 -= kwgt;
            if *moved.offset(k as isize) == -(1 as libc::c_int)
                && *where_0.offset(k as isize) == from
                && *vwgt.offset(k as isize) <= mindiff
            {
                libmetis__rpqUpdate(
                    queue,
                    k,
                    (*ed.offset(k as isize) - *id.offset(k as isize)) as real_t,
                );
            }
            if *ed.offset(k as isize) == 0 as libc::c_int
                && *bndptr.offset(k as isize) != -(1 as libc::c_int)
            {
                nbnd -= 1;
                *bndind
                    .offset(
                        *bndptr.offset(k as isize) as isize,
                    ) = *bndind.offset(nbnd as isize);
                *bndptr
                    .offset(
                        *bndind.offset(nbnd as isize) as isize,
                    ) = *bndptr.offset(k as isize);
                *bndptr.offset(k as isize) = -(1 as libc::c_int);
            } else if *ed.offset(k as isize) > 0 as libc::c_int
                && *bndptr.offset(k as isize) == -(1 as libc::c_int)
            {
                *bndind.offset(nbnd as isize) = k;
                let fresh10 = nbnd;
                nbnd = nbnd + 1;
                *bndptr.offset(k as isize) = fresh10;
            }
            j += 1;
            j;
        }
        nswaps += 1;
        nswaps;
    }
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_REFINE as libc::c_int as libc::c_uint
        != 0
    {
        printf(
            b"\tMinimum cut: %6d, PWGTS: [%6d %6d], NBND: %6d\n\0" as *const u8
                as *const libc::c_char,
            mincut,
            *pwgts.offset(0 as libc::c_int as isize),
            *pwgts.offset(1 as libc::c_int as isize),
            nbnd,
        );
    }
    (*graph).mincut = mincut;
    (*graph).nbnd = nbnd;
    libmetis__rpqDestroy(queue);
    libmetis__wspacepop(ctrl);
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__McGeneral2WayBalance(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut ntpwgts: *mut real_t,
) {
    let mut i: idx_t = 0;
    let mut ii: idx_t = 0;
    let mut j: idx_t = 0;
    let mut k: idx_t = 0;
    let mut l: idx_t = 0;
    let mut kwgt: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut ncon: idx_t = 0;
    let mut nbnd: idx_t = 0;
    let mut nswaps: idx_t = 0;
    let mut from: idx_t = 0;
    let mut to: idx_t = 0;
    let mut pass: idx_t = 0;
    let mut me: idx_t = 0;
    let mut limit: idx_t = 0;
    let mut tmp: idx_t = 0;
    let mut cnum: idx_t = 0;
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut vwgt: *mut idx_t = 0 as *mut idx_t;
    let mut adjwgt: *mut idx_t = 0 as *mut idx_t;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    let mut pwgts: *mut idx_t = 0 as *mut idx_t;
    let mut id: *mut idx_t = 0 as *mut idx_t;
    let mut ed: *mut idx_t = 0 as *mut idx_t;
    let mut bndptr: *mut idx_t = 0 as *mut idx_t;
    let mut bndind: *mut idx_t = 0 as *mut idx_t;
    let mut moved: *mut idx_t = 0 as *mut idx_t;
    let mut swaps: *mut idx_t = 0 as *mut idx_t;
    let mut perm: *mut idx_t = 0 as *mut idx_t;
    let mut qnum: *mut idx_t = 0 as *mut idx_t;
    let mut qsizes: *mut idx_t = 0 as *mut idx_t;
    let mut higain: idx_t = 0;
    let mut mincut: idx_t = 0;
    let mut newcut: idx_t = 0;
    let mut mincutorder: idx_t = 0;
    let mut invtvwgt: *mut real_t = 0 as *mut real_t;
    let mut minbalv: *mut real_t = 0 as *mut real_t;
    let mut newbalv: *mut real_t = 0 as *mut real_t;
    let mut minbal: real_t = 0.;
    let mut newbal: real_t = 0.;
    let mut queues: *mut *mut rpq_t = 0 as *mut *mut rpq_t;
    libmetis__wspacepush(ctrl);
    nvtxs = (*graph).nvtxs;
    ncon = (*graph).ncon;
    xadj = (*graph).xadj;
    vwgt = (*graph).vwgt;
    adjncy = (*graph).adjncy;
    adjwgt = (*graph).adjwgt;
    invtvwgt = (*graph).invtvwgt;
    where_0 = (*graph).where_0;
    id = (*graph).id;
    ed = (*graph).ed;
    pwgts = (*graph).pwgts;
    bndptr = (*graph).bndptr;
    bndind = (*graph).bndind;
    moved = libmetis__iwspacemalloc(ctrl, nvtxs);
    swaps = libmetis__iwspacemalloc(ctrl, nvtxs);
    perm = libmetis__iwspacemalloc(ctrl, nvtxs);
    qnum = libmetis__iwspacemalloc(ctrl, nvtxs);
    newbalv = libmetis__rwspacemalloc(ctrl, ncon);
    minbalv = libmetis__rwspacemalloc(ctrl, ncon);
    qsizes = libmetis__iwspacemalloc(ctrl, 2 as libc::c_int * ncon);
    limit = (if (if 0.01f64 * nvtxs as libc::c_double
        >= 15 as libc::c_int as libc::c_double
    {
        0.01f64 * nvtxs as libc::c_double
    } else {
        15 as libc::c_int as libc::c_double
    }) >= 100 as libc::c_int as libc::c_double
    {
        100 as libc::c_int as libc::c_double
    } else if 0.01f64 * nvtxs as libc::c_double >= 15 as libc::c_int as libc::c_double {
        0.01f64 * nvtxs as libc::c_double
    } else {
        15 as libc::c_int as libc::c_double
    }) as idx_t;
    queues = libmetis__wspacemalloc(
        ctrl,
        ((2 as libc::c_int * ncon) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut rpq_t>() as libc::c_ulong),
    ) as *mut *mut rpq_t;
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int * ncon {
        let ref mut fresh11 = *queues.offset(i as isize);
        *fresh11 = libmetis__rpqCreate(nvtxs as size_t);
        *qsizes.offset(i as isize) = 0 as libc::c_int;
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < nvtxs {
        *qnum
            .offset(
                i as isize,
            ) = libmetis__iargmax_nrm(
            ncon as size_t,
            vwgt.offset((i * ncon) as isize),
            invtvwgt,
        );
        let ref mut fresh12 = *qsizes
            .offset(
                (2 as libc::c_int * *qnum.offset(i as isize)
                    + *where_0.offset(i as isize)) as isize,
            );
        *fresh12 += 1;
        *fresh12;
        i += 1;
        i;
    }
    from = 0 as libc::c_int;
    while from < 2 as libc::c_int {
        j = 0 as libc::c_int;
        while j < ncon {
            if *qsizes.offset((2 as libc::c_int * j + from) as isize) == 0 as libc::c_int
            {
                i = 0 as libc::c_int;
                while i < nvtxs {
                    if !(*where_0.offset(i as isize) != from) {
                        k = libmetis__iargmax2_nrm(
                            ncon as size_t,
                            vwgt.offset((i * ncon) as isize),
                            invtvwgt,
                        );
                        if k == j
                            && *qsizes
                                .offset(
                                    (2 as libc::c_int * *qnum.offset(i as isize) + from)
                                        as isize,
                                ) > *qsizes.offset((2 as libc::c_int * j + from) as isize)
                            && ((*vwgt
                                .offset((i * ncon + *qnum.offset(i as isize)) as isize)
                                as libc::c_float
                                * *invtvwgt.offset(*qnum.offset(i as isize) as isize))
                                as libc::c_double)
                                < 1.3f64
                                    * *vwgt.offset((i * ncon + j) as isize) as libc::c_double
                                    * *invtvwgt.offset(j as isize) as libc::c_double
                        {
                            let ref mut fresh13 = *qsizes
                                .offset(
                                    (2 as libc::c_int * *qnum.offset(i as isize) + from)
                                        as isize,
                                );
                            *fresh13 -= 1;
                            *fresh13;
                            let ref mut fresh14 = *qsizes
                                .offset((2 as libc::c_int * j + from) as isize);
                            *fresh14 += 1;
                            *fresh14;
                            *qnum.offset(i as isize) = j;
                        }
                    }
                    i += 1;
                    i;
                }
            }
            j += 1;
            j;
        }
        from += 1;
        from;
    }
    minbal = libmetis__ComputeLoadImbalanceDiffVec(
        graph,
        2 as libc::c_int,
        (*ctrl).pijbm,
        (*ctrl).ubfactors,
        minbalv,
    );
    mincut = (*graph).mincut;
    newcut = mincut;
    mincutorder = -(1 as libc::c_int);
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_REFINE as libc::c_int as libc::c_uint
        != 0
    {
        printf(b"Parts: [\0" as *const u8 as *const libc::c_char);
        l = 0 as libc::c_int;
        while l < ncon {
            printf(
                b"(%6d %6d %.3f %.3f) \0" as *const u8 as *const libc::c_char,
                *pwgts.offset(l as isize),
                *pwgts.offset((ncon + l) as isize),
                *ntpwgts.offset(l as isize) as libc::c_double,
                *ntpwgts.offset((ncon + l) as isize) as libc::c_double,
            );
            l += 1;
            l;
        }
        printf(
            b"] Nv-Nb[%5d, %5d]. ICut: %6d, LB: %+.3f [B]\n\0" as *const u8
                as *const libc::c_char,
            (*graph).nvtxs,
            (*graph).nbnd,
            (*graph).mincut,
            minbal as libc::c_double,
        );
    }
    libmetis__iset(nvtxs as size_t, -(1 as libc::c_int), moved);
    nbnd = (*graph).nbnd;
    libmetis__irandArrayPermute(
        nvtxs,
        perm,
        nvtxs / 10 as libc::c_int,
        1 as libc::c_int,
    );
    ii = 0 as libc::c_int;
    while ii < nvtxs {
        i = *perm.offset(ii as isize);
        libmetis__rpqInsert(
            *queues
                .offset(
                    (2 as libc::c_int * *qnum.offset(i as isize)
                        + *where_0.offset(i as isize)) as isize,
                ),
            i,
            (*ed.offset(i as isize) - *id.offset(i as isize)) as real_t,
        );
        ii += 1;
        ii;
    }
    nswaps = 0 as libc::c_int;
    while nswaps < nvtxs {
        if minbal as libc::c_double <= 0.0f64 {
            break;
        }
        libmetis__SelectQueue(
            graph,
            (*ctrl).pijbm,
            (*ctrl).ubfactors,
            queues,
            &mut from,
            &mut cnum,
        );
        to = (from + 1 as libc::c_int) % 2 as libc::c_int;
        if from == -(1 as libc::c_int)
            || {
                higain = libmetis__rpqGetTop(
                    *queues.offset((2 as libc::c_int * cnum + from) as isize),
                );
                higain == -(1 as libc::c_int)
            }
        {
            break;
        }
        newcut -= *ed.offset(higain as isize) - *id.offset(higain as isize);
        libmetis__iaxpy(
            ncon as size_t,
            1 as libc::c_int,
            vwgt.offset((higain * ncon) as isize),
            1 as libc::c_int as size_t,
            pwgts.offset((to * ncon) as isize),
            1 as libc::c_int as size_t,
        );
        libmetis__iaxpy(
            ncon as size_t,
            -(1 as libc::c_int),
            vwgt.offset((higain * ncon) as isize),
            1 as libc::c_int as size_t,
            pwgts.offset((from * ncon) as isize),
            1 as libc::c_int as size_t,
        );
        newbal = libmetis__ComputeLoadImbalanceDiffVec(
            graph,
            2 as libc::c_int,
            (*ctrl).pijbm,
            (*ctrl).ubfactors,
            newbalv,
        );
        if newbal < minbal
            || newbal == minbal
                && (newcut < mincut
                    || newcut == mincut
                        && libmetis__BetterBalance2Way(ncon, minbalv, newbalv) != 0)
        {
            mincut = newcut;
            minbal = newbal;
            mincutorder = nswaps;
            libmetis__rcopy(ncon as size_t, newbalv, minbalv);
        } else if nswaps - mincutorder > limit {
            newcut += *ed.offset(higain as isize) - *id.offset(higain as isize);
            libmetis__iaxpy(
                ncon as size_t,
                1 as libc::c_int,
                vwgt.offset((higain * ncon) as isize),
                1 as libc::c_int as size_t,
                pwgts.offset((from * ncon) as isize),
                1 as libc::c_int as size_t,
            );
            libmetis__iaxpy(
                ncon as size_t,
                -(1 as libc::c_int),
                vwgt.offset((higain * ncon) as isize),
                1 as libc::c_int as size_t,
                pwgts.offset((to * ncon) as isize),
                1 as libc::c_int as size_t,
            );
            break;
        }
        *where_0.offset(higain as isize) = to;
        *moved.offset(higain as isize) = nswaps;
        *swaps.offset(nswaps as isize) = higain;
        if (*ctrl).dbglvl as libc::c_uint
            & METIS_DBG_MOVEINFO as libc::c_int as libc::c_uint != 0
        {
            printf(
                b"Moved %6d from %d(%d). Gain: %5d, Cut: %5d, NPwgts: \0" as *const u8
                    as *const libc::c_char,
                higain,
                from,
                cnum,
                *ed.offset(higain as isize) - *id.offset(higain as isize),
                newcut,
            );
            l = 0 as libc::c_int;
            while l < ncon {
                printf(
                    b"(%6d, %6d) \0" as *const u8 as *const libc::c_char,
                    *pwgts.offset(l as isize),
                    *pwgts.offset((ncon + l) as isize),
                );
                l += 1;
                l;
            }
            printf(
                b", %+.3f LB: %+.3f\n\0" as *const u8 as *const libc::c_char,
                minbal as libc::c_double,
                newbal as libc::c_double,
            );
        }
        tmp = *id.offset(higain as isize);
        *id.offset(higain as isize) = *ed.offset(higain as isize);
        *ed.offset(higain as isize) = tmp;
        if *ed.offset(higain as isize) == 0 as libc::c_int
            && *bndptr.offset(higain as isize) != -(1 as libc::c_int)
            && *xadj.offset(higain as isize)
                < *xadj.offset((higain + 1 as libc::c_int) as isize)
        {
            nbnd -= 1;
            *bndind
                .offset(
                    *bndptr.offset(higain as isize) as isize,
                ) = *bndind.offset(nbnd as isize);
            *bndptr
                .offset(
                    *bndind.offset(nbnd as isize) as isize,
                ) = *bndptr.offset(higain as isize);
            *bndptr.offset(higain as isize) = -(1 as libc::c_int);
        }
        if *ed.offset(higain as isize) > 0 as libc::c_int
            && *bndptr.offset(higain as isize) == -(1 as libc::c_int)
        {
            *bndind.offset(nbnd as isize) = higain;
            let fresh15 = nbnd;
            nbnd = nbnd + 1;
            *bndptr.offset(higain as isize) = fresh15;
        }
        j = *xadj.offset(higain as isize);
        while j < *xadj.offset((higain + 1 as libc::c_int) as isize) {
            k = *adjncy.offset(j as isize);
            kwgt = if to == *where_0.offset(k as isize) {
                *adjwgt.offset(j as isize)
            } else {
                -*adjwgt.offset(j as isize)
            };
            let ref mut fresh16 = *id.offset(k as isize);
            *fresh16 += kwgt;
            let ref mut fresh17 = *ed.offset(k as isize);
            *fresh17 -= kwgt;
            if *moved.offset(k as isize) == -(1 as libc::c_int) {
                libmetis__rpqUpdate(
                    *queues
                        .offset(
                            (2 as libc::c_int * *qnum.offset(k as isize)
                                + *where_0.offset(k as isize)) as isize,
                        ),
                    k,
                    (*ed.offset(k as isize) - *id.offset(k as isize)) as real_t,
                );
            }
            if *ed.offset(k as isize) == 0 as libc::c_int
                && *bndptr.offset(k as isize) != -(1 as libc::c_int)
            {
                nbnd -= 1;
                *bndind
                    .offset(
                        *bndptr.offset(k as isize) as isize,
                    ) = *bndind.offset(nbnd as isize);
                *bndptr
                    .offset(
                        *bndind.offset(nbnd as isize) as isize,
                    ) = *bndptr.offset(k as isize);
                *bndptr.offset(k as isize) = -(1 as libc::c_int);
            } else if *ed.offset(k as isize) > 0 as libc::c_int
                && *bndptr.offset(k as isize) == -(1 as libc::c_int)
            {
                *bndind.offset(nbnd as isize) = k;
                let fresh18 = nbnd;
                nbnd = nbnd + 1;
                *bndptr.offset(k as isize) = fresh18;
            }
            j += 1;
            j;
        }
        nswaps += 1;
        nswaps;
    }
    nswaps -= 1;
    nswaps;
    while nswaps > mincutorder {
        higain = *swaps.offset(nswaps as isize);
        let ref mut fresh19 = *where_0.offset(higain as isize);
        *fresh19 = (*where_0.offset(higain as isize) + 1 as libc::c_int)
            % 2 as libc::c_int;
        to = *fresh19;
        tmp = *id.offset(higain as isize);
        *id.offset(higain as isize) = *ed.offset(higain as isize);
        *ed.offset(higain as isize) = tmp;
        if *ed.offset(higain as isize) == 0 as libc::c_int
            && *bndptr.offset(higain as isize) != -(1 as libc::c_int)
            && *xadj.offset(higain as isize)
                < *xadj.offset((higain + 1 as libc::c_int) as isize)
        {
            nbnd -= 1;
            *bndind
                .offset(
                    *bndptr.offset(higain as isize) as isize,
                ) = *bndind.offset(nbnd as isize);
            *bndptr
                .offset(
                    *bndind.offset(nbnd as isize) as isize,
                ) = *bndptr.offset(higain as isize);
            *bndptr.offset(higain as isize) = -(1 as libc::c_int);
        } else if *ed.offset(higain as isize) > 0 as libc::c_int
            && *bndptr.offset(higain as isize) == -(1 as libc::c_int)
        {
            *bndind.offset(nbnd as isize) = higain;
            let fresh20 = nbnd;
            nbnd = nbnd + 1;
            *bndptr.offset(higain as isize) = fresh20;
        }
        libmetis__iaxpy(
            ncon as size_t,
            1 as libc::c_int,
            vwgt.offset((higain * ncon) as isize),
            1 as libc::c_int as size_t,
            pwgts.offset((to * ncon) as isize),
            1 as libc::c_int as size_t,
        );
        libmetis__iaxpy(
            ncon as size_t,
            -(1 as libc::c_int),
            vwgt.offset((higain * ncon) as isize),
            1 as libc::c_int as size_t,
            pwgts.offset(((to + 1 as libc::c_int) % 2 as libc::c_int * ncon) as isize),
            1 as libc::c_int as size_t,
        );
        j = *xadj.offset(higain as isize);
        while j < *xadj.offset((higain + 1 as libc::c_int) as isize) {
            k = *adjncy.offset(j as isize);
            kwgt = if to == *where_0.offset(k as isize) {
                *adjwgt.offset(j as isize)
            } else {
                -*adjwgt.offset(j as isize)
            };
            let ref mut fresh21 = *id.offset(k as isize);
            *fresh21 += kwgt;
            let ref mut fresh22 = *ed.offset(k as isize);
            *fresh22 -= kwgt;
            if *bndptr.offset(k as isize) != -(1 as libc::c_int)
                && *ed.offset(k as isize) == 0 as libc::c_int
            {
                nbnd -= 1;
                *bndind
                    .offset(
                        *bndptr.offset(k as isize) as isize,
                    ) = *bndind.offset(nbnd as isize);
                *bndptr
                    .offset(
                        *bndind.offset(nbnd as isize) as isize,
                    ) = *bndptr.offset(k as isize);
                *bndptr.offset(k as isize) = -(1 as libc::c_int);
            }
            if *bndptr.offset(k as isize) == -(1 as libc::c_int)
                && *ed.offset(k as isize) > 0 as libc::c_int
            {
                *bndind.offset(nbnd as isize) = k;
                let fresh23 = nbnd;
                nbnd = nbnd + 1;
                *bndptr.offset(k as isize) = fresh23;
            }
            j += 1;
            j;
        }
        nswaps -= 1;
        nswaps;
    }
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_REFINE as libc::c_int as libc::c_uint
        != 0
    {
        printf(
            b"\tMincut: %6d at %5d, NBND: %6d, NPwgts: [\0" as *const u8
                as *const libc::c_char,
            mincut,
            mincutorder,
            nbnd,
        );
        l = 0 as libc::c_int;
        while l < ncon {
            printf(
                b"(%6d, %6d) \0" as *const u8 as *const libc::c_char,
                *pwgts.offset(l as isize),
                *pwgts.offset((ncon + l) as isize),
            );
            l += 1;
            l;
        }
        printf(
            b"], LB: %.3f\n\0" as *const u8 as *const libc::c_char,
            libmetis__ComputeLoadImbalance(graph, 2 as libc::c_int, (*ctrl).pijbm)
                as libc::c_double,
        );
    }
    (*graph).mincut = mincut;
    (*graph).nbnd = nbnd;
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int * ncon {
        libmetis__rpqDestroy(*queues.offset(i as isize));
        i += 1;
        i;
    }
    libmetis__wspacepop(ctrl);
}
