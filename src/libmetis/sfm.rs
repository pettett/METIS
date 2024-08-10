use ::libc;
extern "C" {
    fn libmetis__wspacepush(ctrl: *mut ctrl_t);
    fn libmetis__iwspacemalloc(_: *mut ctrl_t, _: idx_t) -> *mut idx_t;
    fn abs(_: libc::c_int) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn libmetis__wspacepop(ctrl: *mut ctrl_t);
    fn gk_CPUSeconds() -> libc::c_double;
    fn libmetis__iset(n: size_t, val: idx_t, x: *mut idx_t) -> *mut idx_t;
    fn libmetis__rpqCreate(maxnodes: size_t) -> *mut rpq_t;
    fn libmetis__rpqReset(queue: *mut rpq_t);
    fn libmetis__rpqDestroy(queue: *mut rpq_t);
    fn libmetis__rpqInsert(queue: *mut rpq_t, node: idx_t, key: real_t) -> libc::c_int;
    fn libmetis__rpqDelete(queue: *mut rpq_t, node: idx_t) -> libc::c_int;
    fn libmetis__rpqUpdate(queue: *mut rpq_t, node: idx_t, newkey: real_t);
    fn libmetis__rpqGetTop(queue: *mut rpq_t) -> idx_t;
    fn libmetis__rpqSeeTopVal(queue: *mut rpq_t) -> idx_t;
    fn libmetis__irandArrayPermute(
        n: idx_t,
        p: *mut idx_t,
        nshuffles: idx_t,
        flag: libc::c_int,
    );
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
pub unsafe extern "C" fn libmetis__FM_2WayNodeRefine2Sided(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut niter: idx_t,
) {
    let mut i: idx_t = 0;
    let mut ii: idx_t = 0;
    let mut j: idx_t = 0;
    let mut k: idx_t = 0;
    let mut jj: idx_t = 0;
    let mut kk: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut nbnd: idx_t = 0;
    let mut nswaps: idx_t = 0;
    let mut nmind: idx_t = 0;
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
    let mut vwgt: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    let mut pwgts: *mut idx_t = 0 as *mut idx_t;
    let mut edegrees: *mut idx_t = 0 as *mut idx_t;
    let mut bndind: *mut idx_t = 0 as *mut idx_t;
    let mut bndptr: *mut idx_t = 0 as *mut idx_t;
    let mut mptr: *mut idx_t = 0 as *mut idx_t;
    let mut mind: *mut idx_t = 0 as *mut idx_t;
    let mut moved: *mut idx_t = 0 as *mut idx_t;
    let mut swaps: *mut idx_t = 0 as *mut idx_t;
    let mut queues: [*mut rpq_t; 2] = [0 as *mut rpq_t; 2];
    let mut rinfo: *mut nrinfo_t = 0 as *mut nrinfo_t;
    let mut higain: idx_t = 0;
    let mut oldgain: idx_t = 0;
    let mut mincut: idx_t = 0;
    let mut initcut: idx_t = 0;
    let mut mincutorder: idx_t = 0;
    let mut pass: idx_t = 0;
    let mut to: idx_t = 0;
    let mut other: idx_t = 0;
    let mut limit: idx_t = 0;
    let mut badmaxpwgt: idx_t = 0;
    let mut mindiff: idx_t = 0;
    let mut newdiff: idx_t = 0;
    let mut u: [idx_t; 2] = [0; 2];
    let mut g: [idx_t; 2] = [0; 2];
    let mut mult: real_t = 0.;
    libmetis__wspacepush(ctrl);
    nvtxs = (*graph).nvtxs;
    xadj = (*graph).xadj;
    adjncy = (*graph).adjncy;
    vwgt = (*graph).vwgt;
    bndind = (*graph).bndind;
    bndptr = (*graph).bndptr;
    where_0 = (*graph).where_0;
    pwgts = (*graph).pwgts;
    rinfo = (*graph).nrinfo;
    queues[0 as libc::c_int as usize] = libmetis__rpqCreate(nvtxs as size_t);
    queues[1 as libc::c_int as usize] = libmetis__rpqCreate(nvtxs as size_t);
    moved = libmetis__iwspacemalloc(ctrl, nvtxs);
    swaps = libmetis__iwspacemalloc(ctrl, nvtxs);
    mptr = libmetis__iwspacemalloc(ctrl, nvtxs + 1 as libc::c_int);
    mind = libmetis__iwspacemalloc(ctrl, 2 as libc::c_int * nvtxs);
    mult = (0.5f64
        * *((*ctrl).ubfactors).offset(0 as libc::c_int as isize) as libc::c_double)
        as real_t;
    badmaxpwgt = (mult
        * (*pwgts.offset(0 as libc::c_int as isize)
            + *pwgts.offset(1 as libc::c_int as isize)
            + *pwgts.offset(2 as libc::c_int as isize)) as libc::c_float) as idx_t;
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_REFINE as libc::c_int as libc::c_uint
        != 0
    {
        printf(
            b"Partitions-N2: [%6d %6d] Nv-Nb[%6d %6d]. ISep: %6d\n\0" as *const u8
                as *const libc::c_char,
            *pwgts.offset(0 as libc::c_int as isize),
            *pwgts.offset(1 as libc::c_int as isize),
            (*graph).nvtxs,
            (*graph).nbnd,
            (*graph).mincut,
        );
    }
    pass = 0 as libc::c_int;
    while pass < niter {
        libmetis__iset(nvtxs as size_t, -(1 as libc::c_int), moved);
        libmetis__rpqReset(queues[0 as libc::c_int as usize]);
        libmetis__rpqReset(queues[1 as libc::c_int as usize]);
        mincutorder = -(1 as libc::c_int);
        mincut = (*graph).mincut;
        initcut = mincut;
        nbnd = (*graph).nbnd;
        libmetis__irandArrayPermute(nbnd, swaps, nbnd, 1 as libc::c_int);
        ii = 0 as libc::c_int;
        while ii < nbnd {
            i = *bndind.offset(*swaps.offset(ii as isize) as isize);
            libmetis__rpqInsert(
                queues[0 as libc::c_int as usize],
                i,
                (*vwgt.offset(i as isize)
                    - (*rinfo.offset(i as isize)).edegrees[1 as libc::c_int as usize])
                    as real_t,
            );
            libmetis__rpqInsert(
                queues[1 as libc::c_int as usize],
                i,
                (*vwgt.offset(i as isize)
                    - (*rinfo.offset(i as isize)).edegrees[0 as libc::c_int as usize])
                    as real_t,
            );
            ii += 1;
            ii;
        }
        limit = if (*ctrl).compress != 0 {
            if 5 as libc::c_int * nbnd >= 400 as libc::c_int {
                400 as libc::c_int
            } else {
                5 as libc::c_int * nbnd
            }
        } else if 2 as libc::c_int * nbnd >= 300 as libc::c_int {
            300 as libc::c_int
        } else {
            2 as libc::c_int * nbnd
        };
        nmind = 0 as libc::c_int;
        *mptr.offset(0 as libc::c_int as isize) = nmind;
        mindiff = abs(
            *pwgts.offset(0 as libc::c_int as isize)
                - *pwgts.offset(1 as libc::c_int as isize),
        );
        to = if *pwgts.offset(0 as libc::c_int as isize)
            < *pwgts.offset(1 as libc::c_int as isize)
        {
            0 as libc::c_int
        } else {
            1 as libc::c_int
        };
        nswaps = 0 as libc::c_int;
        while nswaps < nvtxs {
            u[0 as libc::c_int
                as usize] = libmetis__rpqSeeTopVal(queues[0 as libc::c_int as usize]);
            u[1 as libc::c_int
                as usize] = libmetis__rpqSeeTopVal(queues[1 as libc::c_int as usize]);
            if u[0 as libc::c_int as usize] != -(1 as libc::c_int)
                && u[1 as libc::c_int as usize] != -(1 as libc::c_int)
            {
                g[0 as libc::c_int
                    as usize] = *vwgt.offset(u[0 as libc::c_int as usize] as isize)
                    - (*rinfo.offset(u[0 as libc::c_int as usize] as isize))
                        .edegrees[1 as libc::c_int as usize];
                g[1 as libc::c_int
                    as usize] = *vwgt.offset(u[1 as libc::c_int as usize] as isize)
                    - (*rinfo.offset(u[1 as libc::c_int as usize] as isize))
                        .edegrees[0 as libc::c_int as usize];
                to = if g[0 as libc::c_int as usize] > g[1 as libc::c_int as usize] {
                    0 as libc::c_int
                } else if g[0 as libc::c_int as usize] < g[1 as libc::c_int as usize] {
                    1 as libc::c_int
                } else {
                    pass % 2 as libc::c_int
                };
                if *pwgts.offset(to as isize) + *vwgt.offset(u[to as usize] as isize)
                    > badmaxpwgt
                {
                    to = (to + 1 as libc::c_int) % 2 as libc::c_int;
                }
            } else {
                if u[0 as libc::c_int as usize] == -(1 as libc::c_int)
                    && u[1 as libc::c_int as usize] == -(1 as libc::c_int)
                {
                    break;
                }
                if u[0 as libc::c_int as usize] != -(1 as libc::c_int)
                    && *pwgts.offset(0 as libc::c_int as isize)
                        + *vwgt.offset(u[0 as libc::c_int as usize] as isize)
                        <= badmaxpwgt
                {
                    to = 0 as libc::c_int;
                } else {
                    if !(u[1 as libc::c_int as usize] != -(1 as libc::c_int)
                        && *pwgts.offset(1 as libc::c_int as isize)
                            + *vwgt.offset(u[1 as libc::c_int as usize] as isize)
                            <= badmaxpwgt)
                    {
                        break;
                    }
                    to = 1 as libc::c_int;
                }
            }
            other = (to + 1 as libc::c_int) % 2 as libc::c_int;
            higain = libmetis__rpqGetTop(queues[to as usize]);
            if *moved.offset(higain as isize) == -(1 as libc::c_int) {
                libmetis__rpqDelete(queues[other as usize], higain);
            }
            if nmind + *xadj.offset((higain + 1 as libc::c_int) as isize)
                - *xadj.offset(higain as isize)
                >= 2 as libc::c_int * nvtxs - 1 as libc::c_int
            {
                break;
            }
            let ref mut fresh0 = *pwgts.offset(2 as libc::c_int as isize);
            *fresh0
                -= *vwgt.offset(higain as isize)
                    - (*rinfo.offset(higain as isize)).edegrees[other as usize];
            newdiff = abs(
                *pwgts.offset(to as isize) + *vwgt.offset(higain as isize)
                    - (*pwgts.offset(other as isize)
                        - (*rinfo.offset(higain as isize)).edegrees[other as usize]),
            );
            if *pwgts.offset(2 as libc::c_int as isize) < mincut
                || *pwgts.offset(2 as libc::c_int as isize) == mincut
                    && newdiff < mindiff
            {
                mincut = *pwgts.offset(2 as libc::c_int as isize);
                mincutorder = nswaps;
                mindiff = newdiff;
            } else if nswaps - mincutorder > 2 as libc::c_int * limit
                || nswaps - mincutorder > limit
                    && *pwgts.offset(2 as libc::c_int as isize) as libc::c_double
                        > 1.10f64 * mincut as libc::c_double
            {
                let ref mut fresh1 = *pwgts.offset(2 as libc::c_int as isize);
                *fresh1
                    += *vwgt.offset(higain as isize)
                        - (*rinfo.offset(higain as isize)).edegrees[other as usize];
                break;
            }
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
            let ref mut fresh2 = *pwgts.offset(to as isize);
            *fresh2 += *vwgt.offset(higain as isize);
            *where_0.offset(higain as isize) = to;
            *moved.offset(higain as isize) = nswaps;
            *swaps.offset(nswaps as isize) = higain;
            j = *xadj.offset(higain as isize);
            while j < *xadj.offset((higain + 1 as libc::c_int) as isize) {
                k = *adjncy.offset(j as isize);
                if *where_0.offset(k as isize) == 2 as libc::c_int {
                    oldgain = *vwgt.offset(k as isize)
                        - (*rinfo.offset(k as isize)).edegrees[to as usize];
                    let ref mut fresh3 = (*rinfo.offset(k as isize))
                        .edegrees[to as usize];
                    *fresh3 += *vwgt.offset(higain as isize);
                    if *moved.offset(k as isize) == -(1 as libc::c_int)
                        || *moved.offset(k as isize) == -(2 as libc::c_int + other)
                    {
                        libmetis__rpqUpdate(
                            queues[other as usize],
                            k,
                            (oldgain - *vwgt.offset(higain as isize)) as real_t,
                        );
                    }
                } else if *where_0.offset(k as isize) == other {
                    *bndind.offset(nbnd as isize) = k;
                    let fresh4 = nbnd;
                    nbnd = nbnd + 1;
                    *bndptr.offset(k as isize) = fresh4;
                    let fresh5 = nmind;
                    nmind = nmind + 1;
                    *mind.offset(fresh5 as isize) = k;
                    *where_0.offset(k as isize) = 2 as libc::c_int;
                    let ref mut fresh6 = *pwgts.offset(other as isize);
                    *fresh6 -= *vwgt.offset(k as isize);
                    edegrees = ((*rinfo.offset(k as isize)).edegrees).as_mut_ptr();
                    let ref mut fresh7 = *edegrees.offset(1 as libc::c_int as isize);
                    *fresh7 = 0 as libc::c_int;
                    *edegrees.offset(0 as libc::c_int as isize) = *fresh7;
                    jj = *xadj.offset(k as isize);
                    while jj < *xadj.offset((k + 1 as libc::c_int) as isize) {
                        kk = *adjncy.offset(jj as isize);
                        if *where_0.offset(kk as isize) != 2 as libc::c_int {
                            let ref mut fresh8 = *edegrees
                                .offset(*where_0.offset(kk as isize) as isize);
                            *fresh8 += *vwgt.offset(kk as isize);
                        } else {
                            oldgain = *vwgt.offset(kk as isize)
                                - (*rinfo.offset(kk as isize)).edegrees[other as usize];
                            let ref mut fresh9 = (*rinfo.offset(kk as isize))
                                .edegrees[other as usize];
                            *fresh9 -= *vwgt.offset(k as isize);
                            if *moved.offset(kk as isize) == -(1 as libc::c_int)
                                || *moved.offset(kk as isize) == -(2 as libc::c_int + to)
                            {
                                libmetis__rpqUpdate(
                                    queues[to as usize],
                                    kk,
                                    (oldgain + *vwgt.offset(k as isize)) as real_t,
                                );
                            }
                        }
                        jj += 1;
                        jj;
                    }
                    if *moved.offset(k as isize) == -(1 as libc::c_int) {
                        libmetis__rpqInsert(
                            queues[to as usize],
                            k,
                            (*vwgt.offset(k as isize) - *edegrees.offset(other as isize))
                                as real_t,
                        );
                        *moved.offset(k as isize) = -(2 as libc::c_int + to);
                    }
                }
                j += 1;
                j;
            }
            *mptr.offset((nswaps + 1 as libc::c_int) as isize) = nmind;
            if (*ctrl).dbglvl as libc::c_uint
                & METIS_DBG_MOVEINFO as libc::c_int as libc::c_uint != 0
            {
                printf(
                    b"Moved %6d to %3d, Gain: %5d [%5d] [%4d %4d] \t[%5d %5d %5d]\n\0"
                        as *const u8 as *const libc::c_char,
                    higain,
                    to,
                    g[to as usize],
                    g[other as usize],
                    *vwgt.offset(u[to as usize] as isize),
                    *vwgt.offset(u[other as usize] as isize),
                    *pwgts.offset(0 as libc::c_int as isize),
                    *pwgts.offset(1 as libc::c_int as isize),
                    *pwgts.offset(2 as libc::c_int as isize),
                );
            }
            nswaps += 1;
            nswaps;
        }
        nswaps -= 1;
        nswaps;
        while nswaps > mincutorder {
            higain = *swaps.offset(nswaps as isize);
            to = *where_0.offset(higain as isize);
            other = (to + 1 as libc::c_int) % 2 as libc::c_int;
            let ref mut fresh10 = *pwgts.offset(2 as libc::c_int as isize);
            *fresh10 += *vwgt.offset(higain as isize);
            let ref mut fresh11 = *pwgts.offset(to as isize);
            *fresh11 -= *vwgt.offset(higain as isize);
            *where_0.offset(higain as isize) = 2 as libc::c_int;
            *bndind.offset(nbnd as isize) = higain;
            let fresh12 = nbnd;
            nbnd = nbnd + 1;
            *bndptr.offset(higain as isize) = fresh12;
            edegrees = ((*rinfo.offset(higain as isize)).edegrees).as_mut_ptr();
            let ref mut fresh13 = *edegrees.offset(1 as libc::c_int as isize);
            *fresh13 = 0 as libc::c_int;
            *edegrees.offset(0 as libc::c_int as isize) = *fresh13;
            j = *xadj.offset(higain as isize);
            while j < *xadj.offset((higain + 1 as libc::c_int) as isize) {
                k = *adjncy.offset(j as isize);
                if *where_0.offset(k as isize) == 2 as libc::c_int {
                    let ref mut fresh14 = (*rinfo.offset(k as isize))
                        .edegrees[to as usize];
                    *fresh14 -= *vwgt.offset(higain as isize);
                } else {
                    let ref mut fresh15 = *edegrees
                        .offset(*where_0.offset(k as isize) as isize);
                    *fresh15 += *vwgt.offset(k as isize);
                }
                j += 1;
                j;
            }
            j = *mptr.offset(nswaps as isize);
            while j < *mptr.offset((nswaps + 1 as libc::c_int) as isize) {
                k = *mind.offset(j as isize);
                *where_0.offset(k as isize) = other;
                let ref mut fresh16 = *pwgts.offset(other as isize);
                *fresh16 += *vwgt.offset(k as isize);
                let ref mut fresh17 = *pwgts.offset(2 as libc::c_int as isize);
                *fresh17 -= *vwgt.offset(k as isize);
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
                jj = *xadj.offset(k as isize);
                while jj < *xadj.offset((k + 1 as libc::c_int) as isize) {
                    kk = *adjncy.offset(jj as isize);
                    if *where_0.offset(kk as isize) == 2 as libc::c_int {
                        let ref mut fresh18 = (*rinfo.offset(kk as isize))
                            .edegrees[other as usize];
                        *fresh18 += *vwgt.offset(k as isize);
                    }
                    jj += 1;
                    jj;
                }
                j += 1;
                j;
            }
            nswaps -= 1;
            nswaps;
        }
        if (*ctrl).dbglvl as libc::c_uint
            & METIS_DBG_REFINE as libc::c_int as libc::c_uint != 0
        {
            printf(
                b"\tMinimum sep: %6d at %5d, PWGTS: [%6d %6d], NBND: %6d\n\0"
                    as *const u8 as *const libc::c_char,
                mincut,
                mincutorder,
                *pwgts.offset(0 as libc::c_int as isize),
                *pwgts.offset(1 as libc::c_int as isize),
                nbnd,
            );
        }
        (*graph).mincut = mincut;
        (*graph).nbnd = nbnd;
        if mincutorder == -(1 as libc::c_int) || mincut >= initcut {
            break;
        }
        pass += 1;
        pass;
    }
    libmetis__rpqDestroy(queues[0 as libc::c_int as usize]);
    libmetis__rpqDestroy(queues[1 as libc::c_int as usize]);
    libmetis__wspacepop(ctrl);
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__FM_2WayNodeRefine1Sided(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut niter: idx_t,
) {
    let mut i: idx_t = 0;
    let mut ii: idx_t = 0;
    let mut j: idx_t = 0;
    let mut k: idx_t = 0;
    let mut jj: idx_t = 0;
    let mut kk: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut nbnd: idx_t = 0;
    let mut nswaps: idx_t = 0;
    let mut nmind: idx_t = 0;
    let mut iend: idx_t = 0;
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
    let mut vwgt: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    let mut pwgts: *mut idx_t = 0 as *mut idx_t;
    let mut edegrees: *mut idx_t = 0 as *mut idx_t;
    let mut bndind: *mut idx_t = 0 as *mut idx_t;
    let mut bndptr: *mut idx_t = 0 as *mut idx_t;
    let mut mptr: *mut idx_t = 0 as *mut idx_t;
    let mut mind: *mut idx_t = 0 as *mut idx_t;
    let mut swaps: *mut idx_t = 0 as *mut idx_t;
    let mut queue: *mut rpq_t = 0 as *mut rpq_t;
    let mut rinfo: *mut nrinfo_t = 0 as *mut nrinfo_t;
    let mut higain: idx_t = 0;
    let mut mincut: idx_t = 0;
    let mut initcut: idx_t = 0;
    let mut mincutorder: idx_t = 0;
    let mut pass: idx_t = 0;
    let mut to: idx_t = 0;
    let mut other: idx_t = 0;
    let mut limit: idx_t = 0;
    let mut badmaxpwgt: idx_t = 0;
    let mut mindiff: idx_t = 0;
    let mut newdiff: idx_t = 0;
    let mut mult: real_t = 0.;
    libmetis__wspacepush(ctrl);
    nvtxs = (*graph).nvtxs;
    xadj = (*graph).xadj;
    adjncy = (*graph).adjncy;
    vwgt = (*graph).vwgt;
    bndind = (*graph).bndind;
    bndptr = (*graph).bndptr;
    where_0 = (*graph).where_0;
    pwgts = (*graph).pwgts;
    rinfo = (*graph).nrinfo;
    queue = libmetis__rpqCreate(nvtxs as size_t);
    swaps = libmetis__iwspacemalloc(ctrl, nvtxs);
    mptr = libmetis__iwspacemalloc(ctrl, nvtxs + 1 as libc::c_int);
    mind = libmetis__iwspacemalloc(ctrl, 2 as libc::c_int * nvtxs);
    mult = (0.5f64
        * *((*ctrl).ubfactors).offset(0 as libc::c_int as isize) as libc::c_double)
        as real_t;
    badmaxpwgt = (mult
        * (*pwgts.offset(0 as libc::c_int as isize)
            + *pwgts.offset(1 as libc::c_int as isize)
            + *pwgts.offset(2 as libc::c_int as isize)) as libc::c_float) as idx_t;
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_REFINE as libc::c_int as libc::c_uint
        != 0
    {
        printf(
            b"Partitions-N1: [%6d %6d] Nv-Nb[%6d %6d]. ISep: %6d\n\0" as *const u8
                as *const libc::c_char,
            *pwgts.offset(0 as libc::c_int as isize),
            *pwgts.offset(1 as libc::c_int as isize),
            (*graph).nvtxs,
            (*graph).nbnd,
            (*graph).mincut,
        );
    }
    to = if *pwgts.offset(0 as libc::c_int as isize)
        < *pwgts.offset(1 as libc::c_int as isize)
    {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    pass = 0 as libc::c_int;
    while pass < 2 as libc::c_int * niter {
        other = to;
        to = (to + 1 as libc::c_int) % 2 as libc::c_int;
        libmetis__rpqReset(queue);
        mincutorder = -(1 as libc::c_int);
        mincut = (*graph).mincut;
        initcut = mincut;
        nbnd = (*graph).nbnd;
        libmetis__irandArrayPermute(nbnd, swaps, nbnd, 1 as libc::c_int);
        ii = 0 as libc::c_int;
        while ii < nbnd {
            i = *bndind.offset(*swaps.offset(ii as isize) as isize);
            libmetis__rpqInsert(
                queue,
                i,
                (*vwgt.offset(i as isize)
                    - (*rinfo.offset(i as isize)).edegrees[other as usize]) as real_t,
            );
            ii += 1;
            ii;
        }
        limit = if (*ctrl).compress != 0 {
            if 5 as libc::c_int * nbnd >= 500 as libc::c_int {
                500 as libc::c_int
            } else {
                5 as libc::c_int * nbnd
            }
        } else if 3 as libc::c_int * nbnd >= 300 as libc::c_int {
            300 as libc::c_int
        } else {
            3 as libc::c_int * nbnd
        };
        if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint
            != 0
        {
            (*ctrl).Aux3Tmr -= gk_CPUSeconds();
        }
        nmind = 0 as libc::c_int;
        *mptr.offset(0 as libc::c_int as isize) = nmind;
        mindiff = abs(
            *pwgts.offset(0 as libc::c_int as isize)
                - *pwgts.offset(1 as libc::c_int as isize),
        );
        nswaps = 0 as libc::c_int;
        while nswaps < nvtxs {
            higain = libmetis__rpqGetTop(queue);
            if higain == -(1 as libc::c_int) {
                break;
            }
            if nmind + *xadj.offset((higain + 1 as libc::c_int) as isize)
                - *xadj.offset(higain as isize)
                >= 2 as libc::c_int * nvtxs - 1 as libc::c_int
            {
                break;
            }
            if *pwgts.offset(to as isize) + *vwgt.offset(higain as isize) > badmaxpwgt {
                break;
            }
            let ref mut fresh19 = *pwgts.offset(2 as libc::c_int as isize);
            *fresh19
                -= *vwgt.offset(higain as isize)
                    - (*rinfo.offset(higain as isize)).edegrees[other as usize];
            newdiff = abs(
                *pwgts.offset(to as isize) + *vwgt.offset(higain as isize)
                    - (*pwgts.offset(other as isize)
                        - (*rinfo.offset(higain as isize)).edegrees[other as usize]),
            );
            if *pwgts.offset(2 as libc::c_int as isize) < mincut
                || *pwgts.offset(2 as libc::c_int as isize) == mincut
                    && newdiff < mindiff
            {
                mincut = *pwgts.offset(2 as libc::c_int as isize);
                mincutorder = nswaps;
                mindiff = newdiff;
            } else if nswaps - mincutorder > 3 as libc::c_int * limit
                || nswaps - mincutorder > limit
                    && *pwgts.offset(2 as libc::c_int as isize) as libc::c_double
                        > 1.10f64 * mincut as libc::c_double
            {
                let ref mut fresh20 = *pwgts.offset(2 as libc::c_int as isize);
                *fresh20
                    += *vwgt.offset(higain as isize)
                        - (*rinfo.offset(higain as isize)).edegrees[other as usize];
                break;
            }
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
            let ref mut fresh21 = *pwgts.offset(to as isize);
            *fresh21 += *vwgt.offset(higain as isize);
            *where_0.offset(higain as isize) = to;
            *swaps.offset(nswaps as isize) = higain;
            if (*ctrl).dbglvl as libc::c_uint
                & METIS_DBG_TIME as libc::c_int as libc::c_uint != 0
            {
                (*ctrl).Aux1Tmr -= gk_CPUSeconds();
            }
            j = *xadj.offset(higain as isize);
            while j < *xadj.offset((higain + 1 as libc::c_int) as isize) {
                k = *adjncy.offset(j as isize);
                if *where_0.offset(k as isize) == 2 as libc::c_int {
                    let ref mut fresh22 = (*rinfo.offset(k as isize))
                        .edegrees[to as usize];
                    *fresh22 += *vwgt.offset(higain as isize);
                } else if *where_0.offset(k as isize) == other {
                    *bndind.offset(nbnd as isize) = k;
                    let fresh23 = nbnd;
                    nbnd = nbnd + 1;
                    *bndptr.offset(k as isize) = fresh23;
                    let fresh24 = nmind;
                    nmind = nmind + 1;
                    *mind.offset(fresh24 as isize) = k;
                    *where_0.offset(k as isize) = 2 as libc::c_int;
                    let ref mut fresh25 = *pwgts.offset(other as isize);
                    *fresh25 -= *vwgt.offset(k as isize);
                    edegrees = ((*rinfo.offset(k as isize)).edegrees).as_mut_ptr();
                    let ref mut fresh26 = *edegrees.offset(1 as libc::c_int as isize);
                    *fresh26 = 0 as libc::c_int;
                    *edegrees.offset(0 as libc::c_int as isize) = *fresh26;
                    jj = *xadj.offset(k as isize);
                    iend = *xadj.offset((k + 1 as libc::c_int) as isize);
                    while jj < iend {
                        kk = *adjncy.offset(jj as isize);
                        if *where_0.offset(kk as isize) != 2 as libc::c_int {
                            let ref mut fresh27 = *edegrees
                                .offset(*where_0.offset(kk as isize) as isize);
                            *fresh27 += *vwgt.offset(kk as isize);
                        } else {
                            let ref mut fresh28 = (*rinfo.offset(kk as isize))
                                .edegrees[other as usize];
                            *fresh28 -= *vwgt.offset(k as isize);
                            libmetis__rpqUpdate(
                                queue,
                                kk,
                                (*vwgt.offset(kk as isize)
                                    - (*rinfo.offset(kk as isize)).edegrees[other as usize])
                                    as real_t,
                            );
                        }
                        jj += 1;
                        jj;
                    }
                    libmetis__rpqInsert(
                        queue,
                        k,
                        (*vwgt.offset(k as isize) - *edegrees.offset(other as isize))
                            as real_t,
                    );
                }
                j += 1;
                j;
            }
            *mptr.offset((nswaps + 1 as libc::c_int) as isize) = nmind;
            if (*ctrl).dbglvl as libc::c_uint
                & METIS_DBG_TIME as libc::c_int as libc::c_uint != 0
            {
                (*ctrl).Aux1Tmr += gk_CPUSeconds();
            }
            if (*ctrl).dbglvl as libc::c_uint
                & METIS_DBG_MOVEINFO as libc::c_int as libc::c_uint != 0
            {
                printf(
                    b"Moved %6d to %3d, Gain: %5d [%5d] \t[%5d %5d %5d] [%3d %2d]\n\0"
                        as *const u8 as *const libc::c_char,
                    higain,
                    to,
                    *vwgt.offset(higain as isize)
                        - (*rinfo.offset(higain as isize)).edegrees[other as usize],
                    *vwgt.offset(higain as isize),
                    *pwgts.offset(0 as libc::c_int as isize),
                    *pwgts.offset(1 as libc::c_int as isize),
                    *pwgts.offset(2 as libc::c_int as isize),
                    nswaps,
                    limit,
                );
            }
            nswaps += 1;
            nswaps;
        }
        if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint
            != 0
        {
            (*ctrl).Aux3Tmr += gk_CPUSeconds();
        }
        if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint
            != 0
        {
            (*ctrl).Aux2Tmr -= gk_CPUSeconds();
        }
        nswaps -= 1;
        nswaps;
        while nswaps > mincutorder {
            higain = *swaps.offset(nswaps as isize);
            let ref mut fresh29 = *pwgts.offset(2 as libc::c_int as isize);
            *fresh29 += *vwgt.offset(higain as isize);
            let ref mut fresh30 = *pwgts.offset(to as isize);
            *fresh30 -= *vwgt.offset(higain as isize);
            *where_0.offset(higain as isize) = 2 as libc::c_int;
            *bndind.offset(nbnd as isize) = higain;
            let fresh31 = nbnd;
            nbnd = nbnd + 1;
            *bndptr.offset(higain as isize) = fresh31;
            edegrees = ((*rinfo.offset(higain as isize)).edegrees).as_mut_ptr();
            let ref mut fresh32 = *edegrees.offset(1 as libc::c_int as isize);
            *fresh32 = 0 as libc::c_int;
            *edegrees.offset(0 as libc::c_int as isize) = *fresh32;
            j = *xadj.offset(higain as isize);
            while j < *xadj.offset((higain + 1 as libc::c_int) as isize) {
                k = *adjncy.offset(j as isize);
                if *where_0.offset(k as isize) == 2 as libc::c_int {
                    let ref mut fresh33 = (*rinfo.offset(k as isize))
                        .edegrees[to as usize];
                    *fresh33 -= *vwgt.offset(higain as isize);
                } else {
                    let ref mut fresh34 = *edegrees
                        .offset(*where_0.offset(k as isize) as isize);
                    *fresh34 += *vwgt.offset(k as isize);
                }
                j += 1;
                j;
            }
            j = *mptr.offset(nswaps as isize);
            while j < *mptr.offset((nswaps + 1 as libc::c_int) as isize) {
                k = *mind.offset(j as isize);
                *where_0.offset(k as isize) = other;
                let ref mut fresh35 = *pwgts.offset(other as isize);
                *fresh35 += *vwgt.offset(k as isize);
                let ref mut fresh36 = *pwgts.offset(2 as libc::c_int as isize);
                *fresh36 -= *vwgt.offset(k as isize);
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
                jj = *xadj.offset(k as isize);
                iend = *xadj.offset((k + 1 as libc::c_int) as isize);
                while jj < iend {
                    kk = *adjncy.offset(jj as isize);
                    if *where_0.offset(kk as isize) == 2 as libc::c_int {
                        let ref mut fresh37 = (*rinfo.offset(kk as isize))
                            .edegrees[other as usize];
                        *fresh37 += *vwgt.offset(k as isize);
                    }
                    jj += 1;
                    jj;
                }
                j += 1;
                j;
            }
            nswaps -= 1;
            nswaps;
        }
        if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint
            != 0
        {
            (*ctrl).Aux2Tmr += gk_CPUSeconds();
        }
        if (*ctrl).dbglvl as libc::c_uint
            & METIS_DBG_REFINE as libc::c_int as libc::c_uint != 0
        {
            printf(
                b"\tMinimum sep: %6d at %5d, PWGTS: [%6d %6d], NBND: %6d\n\0"
                    as *const u8 as *const libc::c_char,
                mincut,
                mincutorder,
                *pwgts.offset(0 as libc::c_int as isize),
                *pwgts.offset(1 as libc::c_int as isize),
                nbnd,
            );
        }
        (*graph).mincut = mincut;
        (*graph).nbnd = nbnd;
        if pass % 2 as libc::c_int == 1 as libc::c_int
            && (mincutorder == -(1 as libc::c_int) || mincut >= initcut)
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
pub unsafe extern "C" fn libmetis__FM_2WayNodeBalance(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
) {
    let mut i: idx_t = 0;
    let mut ii: idx_t = 0;
    let mut j: idx_t = 0;
    let mut k: idx_t = 0;
    let mut jj: idx_t = 0;
    let mut kk: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut nbnd: idx_t = 0;
    let mut nswaps: idx_t = 0;
    let mut gain: idx_t = 0;
    let mut badmaxpwgt: idx_t = 0;
    let mut higain: idx_t = 0;
    let mut oldgain: idx_t = 0;
    let mut pass: idx_t = 0;
    let mut to: idx_t = 0;
    let mut other: idx_t = 0;
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
    let mut vwgt: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    let mut pwgts: *mut idx_t = 0 as *mut idx_t;
    let mut edegrees: *mut idx_t = 0 as *mut idx_t;
    let mut bndind: *mut idx_t = 0 as *mut idx_t;
    let mut bndptr: *mut idx_t = 0 as *mut idx_t;
    let mut perm: *mut idx_t = 0 as *mut idx_t;
    let mut moved: *mut idx_t = 0 as *mut idx_t;
    let mut queue: *mut rpq_t = 0 as *mut rpq_t;
    let mut rinfo: *mut nrinfo_t = 0 as *mut nrinfo_t;
    let mut mult: real_t = 0.;
    nvtxs = (*graph).nvtxs;
    xadj = (*graph).xadj;
    adjncy = (*graph).adjncy;
    vwgt = (*graph).vwgt;
    bndind = (*graph).bndind;
    bndptr = (*graph).bndptr;
    where_0 = (*graph).where_0;
    pwgts = (*graph).pwgts;
    rinfo = (*graph).nrinfo;
    mult = (0.5f64
        * *((*ctrl).ubfactors).offset(0 as libc::c_int as isize) as libc::c_double)
        as real_t;
    badmaxpwgt = (mult
        * (*pwgts.offset(0 as libc::c_int as isize)
            + *pwgts.offset(1 as libc::c_int as isize)) as libc::c_float) as idx_t;
    if (if *pwgts.offset(0 as libc::c_int as isize)
        >= *pwgts.offset(1 as libc::c_int as isize)
    {
        *pwgts.offset(0 as libc::c_int as isize)
    } else {
        *pwgts.offset(1 as libc::c_int as isize)
    }) < badmaxpwgt
    {
        return;
    }
    if abs(
        *pwgts.offset(0 as libc::c_int as isize)
            - *pwgts.offset(1 as libc::c_int as isize),
    ) < 3 as libc::c_int * *((*graph).tvwgt).offset(0 as libc::c_int as isize) / nvtxs
    {
        return;
    }
    libmetis__wspacepush(ctrl);
    to = if *pwgts.offset(0 as libc::c_int as isize)
        < *pwgts.offset(1 as libc::c_int as isize)
    {
        0 as libc::c_int
    } else {
        1 as libc::c_int
    };
    other = (to + 1 as libc::c_int) % 2 as libc::c_int;
    queue = libmetis__rpqCreate(nvtxs as size_t);
    perm = libmetis__iwspacemalloc(ctrl, nvtxs);
    moved = libmetis__iset(
        nvtxs as size_t,
        -(1 as libc::c_int),
        libmetis__iwspacemalloc(ctrl, nvtxs),
    );
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_REFINE as libc::c_int as libc::c_uint
        != 0
    {
        printf(
            b"Partitions: [%6d %6d] Nv-Nb[%6d %6d]. ISep: %6d [B]\n\0" as *const u8
                as *const libc::c_char,
            *pwgts.offset(0 as libc::c_int as isize),
            *pwgts.offset(1 as libc::c_int as isize),
            (*graph).nvtxs,
            (*graph).nbnd,
            (*graph).mincut,
        );
    }
    nbnd = (*graph).nbnd;
    libmetis__irandArrayPermute(nbnd, perm, nbnd, 1 as libc::c_int);
    ii = 0 as libc::c_int;
    while ii < nbnd {
        i = *bndind.offset(*perm.offset(ii as isize) as isize);
        libmetis__rpqInsert(
            queue,
            i,
            (*vwgt.offset(i as isize)
                - (*rinfo.offset(i as isize)).edegrees[other as usize]) as real_t,
        );
        ii += 1;
        ii;
    }
    nswaps = 0 as libc::c_int;
    while nswaps < nvtxs {
        higain = libmetis__rpqGetTop(queue);
        if higain == -(1 as libc::c_int) {
            break;
        }
        *moved.offset(higain as isize) = 1 as libc::c_int;
        gain = *vwgt.offset(higain as isize)
            - (*rinfo.offset(higain as isize)).edegrees[other as usize];
        badmaxpwgt = (mult
            * (*pwgts.offset(0 as libc::c_int as isize)
                + *pwgts.offset(1 as libc::c_int as isize)) as libc::c_float) as idx_t;
        if *pwgts.offset(to as isize) > *pwgts.offset(other as isize) {
            break;
        }
        if gain < 0 as libc::c_int && *pwgts.offset(other as isize) < badmaxpwgt {
            break;
        }
        if !(*pwgts.offset(to as isize) + *vwgt.offset(higain as isize) > badmaxpwgt) {
            let ref mut fresh38 = *pwgts.offset(2 as libc::c_int as isize);
            *fresh38 -= gain;
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
            let ref mut fresh39 = *pwgts.offset(to as isize);
            *fresh39 += *vwgt.offset(higain as isize);
            *where_0.offset(higain as isize) = to;
            if (*ctrl).dbglvl as libc::c_uint
                & METIS_DBG_MOVEINFO as libc::c_int as libc::c_uint != 0
            {
                printf(
                    b"Moved %6d to %3d, Gain: %3d, \t[%5d %5d %5d]\n\0" as *const u8
                        as *const libc::c_char,
                    higain,
                    to,
                    *vwgt.offset(higain as isize)
                        - (*rinfo.offset(higain as isize)).edegrees[other as usize],
                    *pwgts.offset(0 as libc::c_int as isize),
                    *pwgts.offset(1 as libc::c_int as isize),
                    *pwgts.offset(2 as libc::c_int as isize),
                );
            }
            j = *xadj.offset(higain as isize);
            while j < *xadj.offset((higain + 1 as libc::c_int) as isize) {
                k = *adjncy.offset(j as isize);
                if *where_0.offset(k as isize) == 2 as libc::c_int {
                    let ref mut fresh40 = (*rinfo.offset(k as isize))
                        .edegrees[to as usize];
                    *fresh40 += *vwgt.offset(higain as isize);
                } else if *where_0.offset(k as isize) == other {
                    *bndind.offset(nbnd as isize) = k;
                    let fresh41 = nbnd;
                    nbnd = nbnd + 1;
                    *bndptr.offset(k as isize) = fresh41;
                    *where_0.offset(k as isize) = 2 as libc::c_int;
                    let ref mut fresh42 = *pwgts.offset(other as isize);
                    *fresh42 -= *vwgt.offset(k as isize);
                    edegrees = ((*rinfo.offset(k as isize)).edegrees).as_mut_ptr();
                    let ref mut fresh43 = *edegrees.offset(1 as libc::c_int as isize);
                    *fresh43 = 0 as libc::c_int;
                    *edegrees.offset(0 as libc::c_int as isize) = *fresh43;
                    jj = *xadj.offset(k as isize);
                    while jj < *xadj.offset((k + 1 as libc::c_int) as isize) {
                        kk = *adjncy.offset(jj as isize);
                        if *where_0.offset(kk as isize) != 2 as libc::c_int {
                            let ref mut fresh44 = *edegrees
                                .offset(*where_0.offset(kk as isize) as isize);
                            *fresh44 += *vwgt.offset(kk as isize);
                        } else {
                            oldgain = *vwgt.offset(kk as isize)
                                - (*rinfo.offset(kk as isize)).edegrees[other as usize];
                            let ref mut fresh45 = (*rinfo.offset(kk as isize))
                                .edegrees[other as usize];
                            *fresh45 -= *vwgt.offset(k as isize);
                            if *moved.offset(kk as isize) == -(1 as libc::c_int) {
                                libmetis__rpqUpdate(
                                    queue,
                                    kk,
                                    (oldgain + *vwgt.offset(k as isize)) as real_t,
                                );
                            }
                        }
                        jj += 1;
                        jj;
                    }
                    libmetis__rpqInsert(
                        queue,
                        k,
                        (*vwgt.offset(k as isize) - *edegrees.offset(other as isize))
                            as real_t,
                    );
                }
                j += 1;
                j;
            }
        }
        nswaps += 1;
        nswaps;
    }
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_REFINE as libc::c_int as libc::c_uint
        != 0
    {
        printf(
            b"\tBalanced sep: %6d at %4d, PWGTS: [%6d %6d], NBND: %6d\n\0" as *const u8
                as *const libc::c_char,
            *pwgts.offset(2 as libc::c_int as isize),
            nswaps,
            *pwgts.offset(0 as libc::c_int as isize),
            *pwgts.offset(1 as libc::c_int as isize),
            nbnd,
        );
    }
    (*graph).mincut = *pwgts.offset(2 as libc::c_int as isize);
    (*graph).nbnd = nbnd;
    libmetis__rpqDestroy(queue);
    libmetis__wspacepop(ctrl);
}
