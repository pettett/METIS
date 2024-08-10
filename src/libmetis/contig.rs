use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn libmetis__wspacemalloc(ctrl: *mut ctrl_t, nbytes: size_t) -> *mut libc::c_void;
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
    fn libmetis__cnbrpoolGetNext(ctrl: *mut ctrl_t, nnbrs: idx_t) -> idx_t;
    fn libmetis__vnbrpoolGetNext(ctrl: *mut ctrl_t, nnbrs: idx_t) -> idx_t;
    fn libmetis__KWayVolUpdate(
        ctrl: *mut ctrl_t,
        graph: *mut graph_t,
        v: idx_t,
        from: idx_t,
        to: idx_t,
        queue: *mut ipq_t,
        vstatus: *mut idx_t,
        r_nupd: *mut idx_t,
        updptr: *mut idx_t,
        updind: *mut idx_t,
        bndtype: idx_t,
        vmarker: *mut idx_t,
        pmarker: *mut idx_t,
        modind: *mut idx_t,
    );
    fn libmetis__wspacepush(ctrl: *mut ctrl_t);
    fn libmetis__iwspacemalloc(_: *mut ctrl_t, _: idx_t) -> *mut idx_t;
    fn libmetis__wspacepop(ctrl: *mut ctrl_t);
    fn gk_free(ptr1: *mut *mut libc::c_void, _: ...);
    fn gk_errexit(signum: libc::c_int, _: *mut libc::c_char, _: ...);
    fn libmetis__iincset(n: size_t, baseval: idx_t, x: *mut idx_t) -> *mut idx_t;
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
    fn libmetis__ismalloc(n: size_t, ival: idx_t, msg: *mut libc::c_char) -> *mut idx_t;
    fn libmetis__iset(n: size_t, val: idx_t, x: *mut idx_t) -> *mut idx_t;
    fn libmetis__icopy(n: size_t, a: *mut idx_t, b: *mut idx_t) -> *mut idx_t;
    fn libmetis__rkvsortd(n: size_t, base: *mut rkv_t);
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
pub unsafe extern "C" fn libmetis__FindPartitionInducedComponents(
    mut graph: *mut graph_t,
    mut where_0: *mut idx_t,
    mut cptr: *mut idx_t,
    mut cind: *mut idx_t,
) -> idx_t {
    let mut i: idx_t = 0;
    let mut ii: idx_t = 0;
    let mut j: idx_t = 0;
    let mut jj: idx_t = 0;
    let mut k: idx_t = 0;
    let mut me: idx_t = 0 as libc::c_int;
    let mut nvtxs: idx_t = 0;
    let mut first: idx_t = 0;
    let mut last: idx_t = 0;
    let mut nleft: idx_t = 0;
    let mut ncmps: idx_t = 0;
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut touched: *mut idx_t = 0 as *mut idx_t;
    let mut perm: *mut idx_t = 0 as *mut idx_t;
    let mut todo: *mut idx_t = 0 as *mut idx_t;
    let mut mustfree_ccsr: idx_t = 0 as libc::c_int;
    let mut mustfree_where: idx_t = 0 as libc::c_int;
    nvtxs = (*graph).nvtxs;
    xadj = (*graph).xadj;
    adjncy = (*graph).adjncy;
    if cptr.is_null() {
        cptr = libmetis__imalloc(
            (nvtxs + 1 as libc::c_int) as size_t,
            b"FindPartitionInducedComponents: cptr\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        cind = libmetis__imalloc(
            nvtxs as size_t,
            b"FindPartitionInducedComponents: cind\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        mustfree_ccsr = 1 as libc::c_int;
    }
    if where_0.is_null() {
        where_0 = libmetis__ismalloc(
            nvtxs as size_t,
            0 as libc::c_int,
            b"FindPartitionInducedComponents: where\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
        mustfree_where = 1 as libc::c_int;
    }
    perm = libmetis__iincset(
        nvtxs as size_t,
        0 as libc::c_int,
        libmetis__imalloc(
            nvtxs as size_t,
            b"FindPartitionInducedComponents: perm\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        ),
    );
    todo = libmetis__iincset(
        nvtxs as size_t,
        0 as libc::c_int,
        libmetis__imalloc(
            nvtxs as size_t,
            b"FindPartitionInducedComponents: todo\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        ),
    );
    touched = libmetis__ismalloc(
        nvtxs as size_t,
        0 as libc::c_int,
        b"FindPartitionInducedComponents: touched\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    ncmps = -(1 as libc::c_int);
    last = 0 as libc::c_int;
    first = last;
    nleft = nvtxs;
    while nleft > 0 as libc::c_int {
        if first == last {
            ncmps += 1;
            *cptr.offset(ncmps as isize) = first;
            i = *todo.offset(0 as libc::c_int as isize);
            let fresh0 = last;
            last = last + 1;
            *cind.offset(fresh0 as isize) = i;
            *touched.offset(i as isize) = 1 as libc::c_int;
            me = *where_0.offset(i as isize);
        }
        let fresh1 = first;
        first = first + 1;
        i = *cind.offset(fresh1 as isize);
        k = *perm.offset(i as isize);
        nleft -= 1;
        let ref mut fresh2 = *todo.offset(k as isize);
        *fresh2 = *todo.offset(nleft as isize);
        j = *fresh2;
        *perm.offset(j as isize) = k;
        j = *xadj.offset(i as isize);
        while j < *xadj.offset((i + 1 as libc::c_int) as isize) {
            k = *adjncy.offset(j as isize);
            if *where_0.offset(k as isize) == me && *touched.offset(k as isize) == 0 {
                let fresh3 = last;
                last = last + 1;
                *cind.offset(fresh3 as isize) = k;
                *touched.offset(k as isize) = 1 as libc::c_int;
            }
            j += 1;
            j;
        }
    }
    ncmps += 1;
    *cptr.offset(ncmps as isize) = first;
    if mustfree_ccsr != 0 {
        gk_free(
            &mut cptr as *mut *mut idx_t as *mut *mut libc::c_void,
            &mut cind as *mut *mut idx_t,
            0 as *mut *mut libc::c_void,
        );
    }
    if mustfree_where != 0 {
        gk_free(
            &mut where_0 as *mut *mut idx_t as *mut *mut libc::c_void,
            0 as *mut *mut libc::c_void,
        );
    }
    gk_free(
        &mut perm as *mut *mut idx_t as *mut *mut libc::c_void,
        &mut todo as *mut *mut idx_t,
        &mut touched as *mut *mut idx_t,
        0 as *mut *mut libc::c_void,
    );
    return ncmps;
}
#[no_mangle]
pub unsafe extern "C" fn ComputeBFSOrdering(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut bfsperm: *mut idx_t,
) {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut k: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut first: idx_t = 0;
    let mut last: idx_t = 0;
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut perm: *mut idx_t = 0 as *mut idx_t;
    libmetis__wspacepush(ctrl);
    nvtxs = (*graph).nvtxs;
    xadj = (*graph).xadj;
    adjncy = (*graph).adjncy;
    perm = libmetis__iincset(
        nvtxs as size_t,
        0 as libc::c_int,
        libmetis__iwspacemalloc(ctrl, nvtxs),
    );
    libmetis__iincset(nvtxs as size_t, 0 as libc::c_int, bfsperm);
    last = 0 as libc::c_int;
    first = last;
    while first < nvtxs {
        if first == last {
            k = *bfsperm.offset(last as isize);
            *perm.offset(k as isize) = -(1 as libc::c_int);
            last += 1;
            last;
        }
        let fresh4 = first;
        first = first + 1;
        i = *bfsperm.offset(fresh4 as isize);
        j = *xadj.offset(i as isize);
        while j < *xadj.offset((i + 1 as libc::c_int) as isize) {
            k = *adjncy.offset(j as isize);
            if *perm.offset(k as isize) != -(1 as libc::c_int) {
                *bfsperm
                    .offset(
                        *perm.offset(k as isize) as isize,
                    ) = *bfsperm.offset(last as isize);
                *perm
                    .offset(
                        *bfsperm.offset(last as isize) as isize,
                    ) = *perm.offset(k as isize);
                let fresh5 = last;
                last = last + 1;
                *bfsperm.offset(fresh5 as isize) = k;
                *perm.offset(k as isize) = -(1 as libc::c_int);
            }
            j += 1;
            j;
        }
    }
    libmetis__wspacepop(ctrl);
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__IsConnected(
    mut graph: *mut graph_t,
    mut report: idx_t,
) -> idx_t {
    let mut ncmps: idx_t = 0;
    ncmps = libmetis__FindPartitionInducedComponents(
        graph,
        0 as *mut idx_t,
        0 as *mut idx_t,
        0 as *mut idx_t,
    );
    if ncmps != 1 as libc::c_int && report != 0 {
        printf(
            b"The graph is not connected. It has %d connected components.\n\0"
                as *const u8 as *const libc::c_char,
            ncmps,
        );
    }
    return (ncmps == 1 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__IsConnectedSubdomain(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut pid: idx_t,
    mut report: idx_t,
) -> idx_t {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut k: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut first: idx_t = 0;
    let mut last: idx_t = 0;
    let mut nleft: idx_t = 0;
    let mut ncmps: idx_t = 0;
    let mut wgt: idx_t = 0;
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    let mut touched: *mut idx_t = 0 as *mut idx_t;
    let mut queue: *mut idx_t = 0 as *mut idx_t;
    let mut cptr: *mut idx_t = 0 as *mut idx_t;
    nvtxs = (*graph).nvtxs;
    xadj = (*graph).xadj;
    adjncy = (*graph).adjncy;
    where_0 = (*graph).where_0;
    touched = libmetis__ismalloc(
        nvtxs as size_t,
        0 as libc::c_int,
        b"IsConnected: touched\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    queue = libmetis__imalloc(
        nvtxs as size_t,
        b"IsConnected: queue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    cptr = libmetis__imalloc(
        (nvtxs + 1 as libc::c_int) as size_t,
        b"IsConnected: cptr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    nleft = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < nvtxs {
        if *where_0.offset(i as isize) == pid {
            nleft += 1;
            nleft;
        }
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < nvtxs {
        if *where_0.offset(i as isize) == pid {
            break;
        }
        i += 1;
        i;
    }
    *touched.offset(i as isize) = 1 as libc::c_int;
    *queue.offset(0 as libc::c_int as isize) = i;
    first = 0 as libc::c_int;
    last = 1 as libc::c_int;
    *cptr.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    ncmps = 0 as libc::c_int;
    while first != nleft {
        if first == last {
            ncmps += 1;
            *cptr.offset(ncmps as isize) = first;
            i = 0 as libc::c_int;
            while i < nvtxs {
                if *where_0.offset(i as isize) == pid && *touched.offset(i as isize) == 0
                {
                    break;
                }
                i += 1;
                i;
            }
            let fresh6 = last;
            last = last + 1;
            *queue.offset(fresh6 as isize) = i;
            *touched.offset(i as isize) = 1 as libc::c_int;
        }
        let fresh7 = first;
        first = first + 1;
        i = *queue.offset(fresh7 as isize);
        j = *xadj.offset(i as isize);
        while j < *xadj.offset((i + 1 as libc::c_int) as isize) {
            k = *adjncy.offset(j as isize);
            if *where_0.offset(k as isize) == pid && *touched.offset(k as isize) == 0 {
                let fresh8 = last;
                last = last + 1;
                *queue.offset(fresh8 as isize) = k;
                *touched.offset(k as isize) = 1 as libc::c_int;
            }
            j += 1;
            j;
        }
    }
    ncmps += 1;
    *cptr.offset(ncmps as isize) = first;
    if ncmps > 1 as libc::c_int && report != 0 {
        printf(
            b"The graph has %d connected components in partition %d:\t\0" as *const u8
                as *const libc::c_char,
            ncmps,
            pid,
        );
        i = 0 as libc::c_int;
        while i < ncmps {
            wgt = 0 as libc::c_int;
            j = *cptr.offset(i as isize);
            while j < *cptr.offset((i + 1 as libc::c_int) as isize) {
                wgt += *((*graph).vwgt).offset(*queue.offset(j as isize) as isize);
                j += 1;
                j;
            }
            printf(
                b"[%5d %5d] \0" as *const u8 as *const libc::c_char,
                *cptr.offset((i + 1 as libc::c_int) as isize) - *cptr.offset(i as isize),
                wgt,
            );
            i += 1;
            i;
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
    }
    gk_free(
        &mut touched as *mut *mut idx_t as *mut *mut libc::c_void,
        &mut queue as *mut *mut idx_t,
        &mut cptr as *mut *mut idx_t,
        0 as *mut *mut libc::c_void,
    );
    return if ncmps == 1 as libc::c_int { 1 as libc::c_int } else { 0 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__FindSepInducedComponents(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut cptr: *mut idx_t,
    mut cind: *mut idx_t,
) -> idx_t {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut k: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut first: idx_t = 0;
    let mut last: idx_t = 0;
    let mut nleft: idx_t = 0;
    let mut ncmps: idx_t = 0;
    let mut wgt: idx_t = 0;
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    let mut touched: *mut idx_t = 0 as *mut idx_t;
    let mut queue: *mut idx_t = 0 as *mut idx_t;
    nvtxs = (*graph).nvtxs;
    xadj = (*graph).xadj;
    adjncy = (*graph).adjncy;
    where_0 = (*graph).where_0;
    touched = libmetis__ismalloc(
        nvtxs as size_t,
        0 as libc::c_int,
        b"IsConnected: queue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < (*graph).nbnd {
        *touched
            .offset(*((*graph).bndind).offset(i as isize) as isize) = 1 as libc::c_int;
        i += 1;
        i;
    }
    queue = cind;
    nleft = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < nvtxs {
        if *where_0.offset(i as isize) != 2 as libc::c_int {
            nleft += 1;
            nleft;
        }
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < nvtxs {
        if *where_0.offset(i as isize) != 2 as libc::c_int {
            break;
        }
        i += 1;
        i;
    }
    *touched.offset(i as isize) = 1 as libc::c_int;
    *queue.offset(0 as libc::c_int as isize) = i;
    first = 0 as libc::c_int;
    last = 1 as libc::c_int;
    *cptr.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    ncmps = 0 as libc::c_int;
    while first != nleft {
        if first == last {
            ncmps += 1;
            *cptr.offset(ncmps as isize) = first;
            i = 0 as libc::c_int;
            while i < nvtxs {
                if *touched.offset(i as isize) == 0 {
                    break;
                }
                i += 1;
                i;
            }
            let fresh9 = last;
            last = last + 1;
            *queue.offset(fresh9 as isize) = i;
            *touched.offset(i as isize) = 1 as libc::c_int;
        }
        let fresh10 = first;
        first = first + 1;
        i = *queue.offset(fresh10 as isize);
        j = *xadj.offset(i as isize);
        while j < *xadj.offset((i + 1 as libc::c_int) as isize) {
            k = *adjncy.offset(j as isize);
            if *touched.offset(k as isize) == 0 {
                let fresh11 = last;
                last = last + 1;
                *queue.offset(fresh11 as isize) = k;
                *touched.offset(k as isize) = 1 as libc::c_int;
            }
            j += 1;
            j;
        }
    }
    ncmps += 1;
    *cptr.offset(ncmps as isize) = first;
    gk_free(
        &mut touched as *mut *mut idx_t as *mut *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
    return ncmps;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__EliminateComponents(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
) {
    let mut i: idx_t = 0;
    let mut ii: idx_t = 0;
    let mut j: idx_t = 0;
    let mut jj: idx_t = 0;
    let mut k: idx_t = 0;
    let mut me: idx_t = 0;
    let mut nparts: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut ncon: idx_t = 0;
    let mut ncmps: idx_t = 0;
    let mut other: idx_t = 0;
    let mut ncand: idx_t = 0;
    let mut target: idx_t = 0;
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut vwgt: *mut idx_t = 0 as *mut idx_t;
    let mut adjwgt: *mut idx_t = 0 as *mut idx_t;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    let mut pwgts: *mut idx_t = 0 as *mut idx_t;
    let mut cptr: *mut idx_t = 0 as *mut idx_t;
    let mut cind: *mut idx_t = 0 as *mut idx_t;
    let mut cpvec: *mut idx_t = 0 as *mut idx_t;
    let mut pcptr: *mut idx_t = 0 as *mut idx_t;
    let mut pcind: *mut idx_t = 0 as *mut idx_t;
    let mut cwhere: *mut idx_t = 0 as *mut idx_t;
    let mut cid: idx_t = 0;
    let mut bestcid: idx_t = 0;
    let mut cwgt: *mut idx_t = 0 as *mut idx_t;
    let mut bestcwgt: *mut idx_t = 0 as *mut idx_t;
    let mut ntodo: idx_t = 0;
    let mut oldntodo: idx_t = 0;
    let mut todo: *mut idx_t = 0 as *mut idx_t;
    let mut cand: *mut rkv_t = 0 as *mut rkv_t;
    let mut tpwgts: *mut real_t = 0 as *mut real_t;
    let mut vmarker: *mut idx_t = 0 as *mut idx_t;
    let mut pmarker: *mut idx_t = 0 as *mut idx_t;
    let mut modind: *mut idx_t = 0 as *mut idx_t;
    libmetis__wspacepush(ctrl);
    nvtxs = (*graph).nvtxs;
    ncon = (*graph).ncon;
    xadj = (*graph).xadj;
    adjncy = (*graph).adjncy;
    vwgt = (*graph).vwgt;
    adjwgt = if (*ctrl).objtype as libc::c_uint
        == METIS_OBJTYPE_VOL as libc::c_int as libc::c_uint
    {
        0 as *mut idx_t
    } else {
        (*graph).adjwgt
    };
    where_0 = (*graph).where_0;
    pwgts = (*graph).pwgts;
    nparts = (*ctrl).nparts;
    tpwgts = (*ctrl).tpwgts;
    cptr = libmetis__iwspacemalloc(ctrl, nvtxs + 1 as libc::c_int);
    cind = libmetis__iwspacemalloc(ctrl, nvtxs);
    ncmps = libmetis__FindPartitionInducedComponents(graph, where_0, cptr, cind);
    if (*ctrl).dbglvl as libc::c_uint
        & METIS_DBG_CONTIGINFO as libc::c_int as libc::c_uint != 0
    {
        printf(
            b"I found %d components, for this %d-way partition\n\0" as *const u8
                as *const libc::c_char,
            ncmps,
            nparts,
        );
    }
    if ncmps > nparts {
        cwgt = libmetis__iwspacemalloc(ctrl, ncon);
        bestcwgt = libmetis__iwspacemalloc(ctrl, ncon);
        cpvec = libmetis__iwspacemalloc(ctrl, nparts);
        pcptr = libmetis__iset(
            (nparts + 1 as libc::c_int) as size_t,
            0 as libc::c_int,
            libmetis__iwspacemalloc(ctrl, nparts + 1 as libc::c_int),
        );
        pcind = libmetis__iwspacemalloc(ctrl, ncmps);
        cwhere = libmetis__iset(
            nvtxs as size_t,
            -(1 as libc::c_int),
            libmetis__iwspacemalloc(ctrl, nvtxs),
        );
        todo = libmetis__iwspacemalloc(ctrl, ncmps);
        cand = libmetis__wspacemalloc(
            ctrl,
            (nparts as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<rkv_t>() as libc::c_ulong),
        ) as *mut rkv_t;
        if (*ctrl).objtype as libc::c_uint
            == METIS_OBJTYPE_VOL as libc::c_int as libc::c_uint
        {
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
        }
        i = 0 as libc::c_int;
        while i < ncmps {
            let ref mut fresh12 = *pcptr
                .offset(
                    *where_0
                        .offset(*cind.offset(*cptr.offset(i as isize) as isize) as isize)
                        as isize,
                );
            *fresh12 += 1;
            *fresh12;
            i += 1;
            i;
        }
        i = 1 as libc::c_int;
        while i < nparts {
            let ref mut fresh13 = *pcptr.offset(i as isize);
            *fresh13 += *pcptr.offset((i - 1 as libc::c_int) as isize);
            i += 1;
            i;
        }
        i = nparts;
        while i > 0 as libc::c_int {
            *pcptr.offset(i as isize) = *pcptr.offset((i - 1 as libc::c_int) as isize);
            i -= 1;
            i;
        }
        *pcptr.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < ncmps {
            let ref mut fresh14 = *pcptr
                .offset(
                    *where_0
                        .offset(*cind.offset(*cptr.offset(i as isize) as isize) as isize)
                        as isize,
                );
            let fresh15 = *fresh14;
            *fresh14 = *fresh14 + 1;
            *pcind.offset(fresh15 as isize) = i;
            i += 1;
            i;
        }
        i = nparts;
        while i > 0 as libc::c_int {
            *pcptr.offset(i as isize) = *pcptr.offset((i - 1 as libc::c_int) as isize);
            i -= 1;
            i;
        }
        *pcptr.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
        ntodo = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < nparts {
            if *pcptr.offset((i + 1 as libc::c_int) as isize) - *pcptr.offset(i as isize)
                == 1 as libc::c_int
            {
                bestcid = *pcind.offset(*pcptr.offset(i as isize) as isize);
            } else {
                bestcid = -(1 as libc::c_int);
                j = *pcptr.offset(i as isize);
                while j < *pcptr.offset((i + 1 as libc::c_int) as isize) {
                    cid = *pcind.offset(j as isize);
                    libmetis__iset(ncon as size_t, 0 as libc::c_int, cwgt);
                    ii = *cptr.offset(cid as isize);
                    while ii < *cptr.offset((cid + 1 as libc::c_int) as isize) {
                        libmetis__iaxpy(
                            ncon as size_t,
                            1 as libc::c_int,
                            vwgt.offset((*cind.offset(ii as isize) * ncon) as isize),
                            1 as libc::c_int as size_t,
                            cwgt,
                            1 as libc::c_int as size_t,
                        );
                        ii += 1;
                        ii;
                    }
                    if bestcid == -(1 as libc::c_int)
                        || libmetis__isum(
                            ncon as size_t,
                            bestcwgt,
                            1 as libc::c_int as size_t,
                        )
                            < libmetis__isum(
                                ncon as size_t,
                                cwgt,
                                1 as libc::c_int as size_t,
                            )
                    {
                        bestcid = cid;
                        libmetis__icopy(ncon as size_t, cwgt, bestcwgt);
                    }
                    j += 1;
                    j;
                }
                j = *pcptr.offset(i as isize);
                while j < *pcptr.offset((i + 1 as libc::c_int) as isize) {
                    if *pcind.offset(j as isize) != bestcid {
                        let fresh16 = ntodo;
                        ntodo = ntodo + 1;
                        *todo.offset(fresh16 as isize) = *pcind.offset(j as isize);
                    }
                    j += 1;
                    j;
                }
            }
            j = *cptr.offset(bestcid as isize);
            while j < *cptr.offset((bestcid + 1 as libc::c_int) as isize) {
                *cwhere.offset(*cind.offset(j as isize) as isize) = i;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
        while ntodo > 0 as libc::c_int {
            oldntodo = ntodo;
            i = 0 as libc::c_int;
            while i < ntodo {
                cid = *todo.offset(i as isize);
                me = *where_0
                    .offset(*cind.offset(*cptr.offset(cid as isize) as isize) as isize);
                libmetis__iset(ncon as size_t, 0 as libc::c_int, cwgt);
                j = *cptr.offset(cid as isize);
                while j < *cptr.offset((cid + 1 as libc::c_int) as isize) {
                    libmetis__iaxpy(
                        ncon as size_t,
                        1 as libc::c_int,
                        vwgt.offset((*cind.offset(j as isize) * ncon) as isize),
                        1 as libc::c_int as size_t,
                        cwgt,
                        1 as libc::c_int as size_t,
                    );
                    j += 1;
                    j;
                }
                if (*ctrl).dbglvl as libc::c_uint
                    & METIS_DBG_CONTIGINFO as libc::c_int as libc::c_uint != 0
                {
                    printf(
                        b"Trying to move %d [%d] from %d\n\0" as *const u8
                            as *const libc::c_char,
                        cid,
                        libmetis__isum(ncon as size_t, cwgt, 1 as libc::c_int as size_t),
                        me,
                    );
                }
                libmetis__iset(nparts as size_t, 0 as libc::c_int, cpvec);
                j = *cptr.offset(cid as isize);
                while j < *cptr.offset((cid + 1 as libc::c_int) as isize) {
                    ii = *cind.offset(j as isize);
                    jj = *xadj.offset(ii as isize);
                    while jj < *xadj.offset((ii + 1 as libc::c_int) as isize) {
                        if *cwhere.offset(*adjncy.offset(jj as isize) as isize)
                            != -(1 as libc::c_int)
                        {
                            let ref mut fresh17 = *cpvec
                                .offset(
                                    *cwhere.offset(*adjncy.offset(jj as isize) as isize)
                                        as isize,
                                );
                            *fresh17
                                += if !adjwgt.is_null() {
                                    *adjwgt.offset(jj as isize)
                                } else {
                                    1 as libc::c_int
                                };
                        }
                        jj += 1;
                        jj;
                    }
                    j += 1;
                    j;
                }
                ncand = 0 as libc::c_int;
                j = 0 as libc::c_int;
                while j < nparts {
                    if *cpvec.offset(j as isize) > 0 as libc::c_int {
                        (*cand.offset(ncand as isize))
                            .key = *cpvec.offset(j as isize) as real_t;
                        let fresh18 = ncand;
                        ncand = ncand + 1;
                        (*cand.offset(fresh18 as isize)).val = j;
                    }
                    j += 1;
                    j;
                }
                if !(ncand == 0 as libc::c_int) {
                    libmetis__rkvsortd(ncand as size_t, cand);
                    if ncon == 1 as libc::c_int {
                        j = 1 as libc::c_int;
                        while j < ncand {
                            if ((*cand.offset(j as isize)).key as libc::c_double)
                                < 0.5f64
                                    * (*cand.offset(0 as libc::c_int as isize)).key
                                        as libc::c_double
                            {
                                break;
                            }
                            j += 1;
                            j;
                        }
                        ncand = j;
                    }
                    target = (*cand.offset(0 as libc::c_int as isize)).val;
                    j = 1 as libc::c_int;
                    while j < ncand {
                        if libmetis__BetterBalanceKWay(
                            ncon,
                            cwgt,
                            (*ctrl).ubfactors,
                            1 as libc::c_int,
                            pwgts.offset((target * ncon) as isize),
                            ((*ctrl).pijbm).offset((target * ncon) as isize),
                            1 as libc::c_int,
                            pwgts
                                .offset(((*cand.offset(j as isize)).val * ncon) as isize),
                            ((*ctrl).pijbm)
                                .offset(((*cand.offset(j as isize)).val * ncon) as isize),
                        ) != 0
                        {
                            target = (*cand.offset(j as isize)).val;
                        }
                        j += 1;
                        j;
                    }
                    if (*ctrl).dbglvl as libc::c_uint
                        & METIS_DBG_CONTIGINFO as libc::c_int as libc::c_uint != 0
                    {
                        printf(
                            b"\tMoving it to %d [%d] [%d]\n\0" as *const u8
                                as *const libc::c_char,
                            target,
                            *cpvec.offset(target as isize),
                            ncand,
                        );
                    }
                    if target != me {
                        match (*ctrl).objtype as libc::c_uint {
                            0 => {
                                libmetis__MoveGroupContigForCut(
                                    ctrl,
                                    graph,
                                    target,
                                    cid,
                                    cptr,
                                    cind,
                                );
                            }
                            1 => {
                                libmetis__MoveGroupContigForVol(
                                    ctrl,
                                    graph,
                                    target,
                                    cid,
                                    cptr,
                                    cind,
                                    vmarker,
                                    pmarker,
                                    modind,
                                );
                            }
                            _ => {
                                gk_errexit(
                                    15 as libc::c_int,
                                    b"Unknown objtype %d\n\0" as *const u8
                                        as *const libc::c_char as *mut libc::c_char,
                                    (*ctrl).objtype as libc::c_uint,
                                );
                            }
                        }
                    }
                    j = *cptr.offset(cid as isize);
                    while j < *cptr.offset((cid + 1 as libc::c_int) as isize) {
                        *cwhere.offset(*cind.offset(j as isize) as isize) = target;
                        j += 1;
                        j;
                    }
                    ntodo -= 1;
                    *todo.offset(i as isize) = *todo.offset(ntodo as isize);
                }
                i += 1;
                i;
            }
            if !(oldntodo == ntodo) {
                continue;
            }
            if (*ctrl).dbglvl as libc::c_uint
                & METIS_DBG_CONTIGINFO as libc::c_int as libc::c_uint != 0
            {
                printf(
                    b"Stopped at ntodo: %d\n\0" as *const u8 as *const libc::c_char,
                    ntodo,
                );
            }
            break;
        }
        i = 0 as libc::c_int;
        while i < nvtxs {
            i += 1;
            i;
        }
    }
    libmetis__wspacepop(ctrl);
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__MoveGroupContigForCut(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut to: idx_t,
    mut gid: idx_t,
    mut ptr: *mut idx_t,
    mut ind: *mut idx_t,
) {
    let mut i: idx_t = 0;
    let mut ii: idx_t = 0;
    let mut iii: idx_t = 0;
    let mut j: idx_t = 0;
    let mut jj: idx_t = 0;
    let mut k: idx_t = 0;
    let mut l: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut nbnd: idx_t = 0;
    let mut from: idx_t = 0;
    let mut me: idx_t = 0;
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut adjwgt: *mut idx_t = 0 as *mut idx_t;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    let mut bndptr: *mut idx_t = 0 as *mut idx_t;
    let mut bndind: *mut idx_t = 0 as *mut idx_t;
    let mut myrinfo: *mut ckrinfo_t = 0 as *mut ckrinfo_t;
    let mut mynbrs: *mut cnbr_t = 0 as *mut cnbr_t;
    nvtxs = (*graph).nvtxs;
    xadj = (*graph).xadj;
    adjncy = (*graph).adjncy;
    adjwgt = (*graph).adjwgt;
    where_0 = (*graph).where_0;
    bndptr = (*graph).bndptr;
    bndind = (*graph).bndind;
    nbnd = (*graph).nbnd;
    iii = *ptr.offset(gid as isize);
    while iii < *ptr.offset((gid + 1 as libc::c_int) as isize) {
        i = *ind.offset(iii as isize);
        from = *where_0.offset(i as isize);
        myrinfo = ((*graph).ckrinfo).offset(i as isize);
        if (*myrinfo).inbr == -(1 as libc::c_int) {
            (*myrinfo)
                .inbr = libmetis__cnbrpoolGetNext(
                ctrl,
                *xadj.offset((i + 1 as libc::c_int) as isize) - *xadj.offset(i as isize)
                    + 1 as libc::c_int,
            );
            (*myrinfo).nnbrs = 0 as libc::c_int;
        }
        mynbrs = ((*ctrl).cnbrpool).offset((*myrinfo).inbr as isize);
        k = 0 as libc::c_int;
        while k < (*myrinfo).nnbrs {
            if (*mynbrs.offset(k as isize)).pid == to {
                break;
            }
            k += 1;
            k;
        }
        if k == (*myrinfo).nnbrs {
            (*mynbrs.offset(k as isize)).pid = to;
            (*mynbrs.offset(k as isize)).ed = 0 as libc::c_int;
            (*myrinfo).nnbrs += 1;
            (*myrinfo).nnbrs;
        }
        (*graph).mincut -= (*mynbrs.offset(k as isize)).ed - (*myrinfo).id;
        libmetis__iaxpy(
            (*graph).ncon as size_t,
            1 as libc::c_int,
            ((*graph).vwgt).offset((i * (*graph).ncon) as isize),
            1 as libc::c_int as size_t,
            ((*graph).pwgts).offset((to * (*graph).ncon) as isize),
            1 as libc::c_int as size_t,
        );
        libmetis__iaxpy(
            (*graph).ncon as size_t,
            -(1 as libc::c_int),
            ((*graph).vwgt).offset((i * (*graph).ncon) as isize),
            1 as libc::c_int as size_t,
            ((*graph).pwgts).offset((from * (*graph).ncon) as isize),
            1 as libc::c_int as size_t,
        );
        *where_0.offset(i as isize) = to;
        (*myrinfo).ed += (*myrinfo).id - (*mynbrs.offset(k as isize)).ed;
        j = (*myrinfo).id;
        (*myrinfo).id = (*mynbrs.offset(k as isize)).ed;
        (*mynbrs.offset(k as isize)).ed = j;
        if (*mynbrs.offset(k as isize)).ed == 0 as libc::c_int {
            (*myrinfo).nnbrs -= 1;
            *mynbrs.offset(k as isize) = *mynbrs.offset((*myrinfo).nnbrs as isize);
        } else {
            (*mynbrs.offset(k as isize)).pid = from;
        }
        if 1 as libc::c_int == 1 as libc::c_int {
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
                let fresh19 = nbnd;
                nbnd = nbnd + 1;
                *bndptr.offset(i as isize) = fresh19;
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
                let fresh20 = nbnd;
                nbnd = nbnd + 1;
                *bndptr.offset(i as isize) = fresh20;
            }
        }
        j = *xadj.offset(i as isize);
        while j < *xadj.offset((i + 1 as libc::c_int) as isize) {
            ii = *adjncy.offset(j as isize);
            me = *where_0.offset(ii as isize);
            myrinfo = ((*graph).ckrinfo).offset(ii as isize);
            let mut k_0: idx_t = 0;
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
            mynbrs_0 = ((*ctrl).cnbrpool).offset((*myrinfo).inbr as isize);
            if me == from {
                (*myrinfo).ed += *adjwgt.offset(j as isize);
                (*myrinfo).id -= *adjwgt.offset(j as isize);
                if 1 as libc::c_int == 1 as libc::c_int {
                    if (*myrinfo).ed - (*myrinfo).id >= 0 as libc::c_int
                        && *bndptr.offset(ii as isize) == -(1 as libc::c_int)
                    {
                        *bndind.offset(nbnd as isize) = ii;
                        let fresh21 = nbnd;
                        nbnd = nbnd + 1;
                        *bndptr.offset(ii as isize) = fresh21;
                    }
                } else if (*myrinfo).ed > 0 as libc::c_int
                    && *bndptr.offset(ii as isize) == -(1 as libc::c_int)
                {
                    *bndind.offset(nbnd as isize) = ii;
                    let fresh22 = nbnd;
                    nbnd = nbnd + 1;
                    *bndptr.offset(ii as isize) = fresh22;
                }
            } else if me == to {
                (*myrinfo).id += *adjwgt.offset(j as isize);
                (*myrinfo).ed -= *adjwgt.offset(j as isize);
                if 1 as libc::c_int == 1 as libc::c_int {
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
                k_0 = 0 as libc::c_int;
                while k_0 < (*myrinfo).nnbrs {
                    if (*mynbrs_0.offset(k_0 as isize)).pid == from {
                        if (*mynbrs_0.offset(k_0 as isize)).ed
                            == *adjwgt.offset(j as isize)
                        {
                            (*myrinfo).nnbrs -= 1;
                            *mynbrs_0
                                .offset(
                                    k_0 as isize,
                                ) = *mynbrs_0.offset((*myrinfo).nnbrs as isize);
                        } else {
                            let ref mut fresh23 = (*mynbrs_0.offset(k_0 as isize)).ed;
                            *fresh23 -= *adjwgt.offset(j as isize);
                        }
                        break;
                    } else {
                        k_0 += 1;
                        k_0;
                    }
                }
            }
            if me != to {
                k_0 = 0 as libc::c_int;
                while k_0 < (*myrinfo).nnbrs {
                    if (*mynbrs_0.offset(k_0 as isize)).pid == to {
                        let ref mut fresh24 = (*mynbrs_0.offset(k_0 as isize)).ed;
                        *fresh24 += *adjwgt.offset(j as isize);
                        break;
                    } else {
                        k_0 += 1;
                        k_0;
                    }
                }
                if k_0 == (*myrinfo).nnbrs {
                    (*mynbrs_0.offset(k_0 as isize)).pid = to;
                    (*mynbrs_0.offset(k_0 as isize)).ed = *adjwgt.offset(j as isize);
                    (*myrinfo).nnbrs += 1;
                    (*myrinfo).nnbrs;
                }
            }
            j += 1;
            j;
        }
        iii += 1;
        iii;
    }
    (*graph).nbnd = nbnd;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__MoveGroupContigForVol(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut to: idx_t,
    mut gid: idx_t,
    mut ptr: *mut idx_t,
    mut ind: *mut idx_t,
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
    let mut l: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut from: idx_t = 0;
    let mut me: idx_t = 0;
    let mut other: idx_t = 0;
    let mut xgain: idx_t = 0;
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
    let mut vsize: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    let mut myrinfo: *mut vkrinfo_t = 0 as *mut vkrinfo_t;
    let mut orinfo: *mut vkrinfo_t = 0 as *mut vkrinfo_t;
    let mut mynbrs: *mut vnbr_t = 0 as *mut vnbr_t;
    let mut onbrs: *mut vnbr_t = 0 as *mut vnbr_t;
    nvtxs = (*graph).nvtxs;
    xadj = (*graph).xadj;
    vsize = (*graph).vsize;
    adjncy = (*graph).adjncy;
    where_0 = (*graph).where_0;
    iii = *ptr.offset(gid as isize);
    while iii < *ptr.offset((gid + 1 as libc::c_int) as isize) {
        i = *ind.offset(iii as isize);
        from = *where_0.offset(i as isize);
        myrinfo = ((*graph).vkrinfo).offset(i as isize);
        if (*myrinfo).inbr == -(1 as libc::c_int) {
            (*myrinfo)
                .inbr = libmetis__vnbrpoolGetNext(
                ctrl,
                *xadj.offset((i + 1 as libc::c_int) as isize) - *xadj.offset(i as isize)
                    + 1 as libc::c_int,
            );
            (*myrinfo).nnbrs = 0 as libc::c_int;
        }
        mynbrs = ((*ctrl).vnbrpool).offset((*myrinfo).inbr as isize);
        xgain = if (*myrinfo).nid == 0 as libc::c_int
            && (*myrinfo).ned > 0 as libc::c_int
        {
            *vsize.offset(i as isize)
        } else {
            0 as libc::c_int
        };
        k = 0 as libc::c_int;
        while k < (*myrinfo).nnbrs {
            if (*mynbrs.offset(k as isize)).pid == to {
                break;
            }
            k += 1;
            k;
        }
        if k == (*myrinfo).nnbrs {
            if (*myrinfo).nid > 0 as libc::c_int {
                xgain -= *vsize.offset(i as isize);
            }
            j = *xadj.offset(i as isize);
            while j < *xadj.offset((i + 1 as libc::c_int) as isize) {
                ii = *adjncy.offset(j as isize);
                other = *where_0.offset(ii as isize);
                orinfo = ((*graph).vkrinfo).offset(ii as isize);
                onbrs = ((*ctrl).vnbrpool).offset((*orinfo).inbr as isize);
                if from == other {
                    l = 0 as libc::c_int;
                    while l < (*orinfo).nnbrs {
                        if (*onbrs.offset(l as isize)).pid == to {
                            break;
                        }
                        l += 1;
                        l;
                    }
                    if l == (*orinfo).nnbrs {
                        xgain -= *vsize.offset(ii as isize);
                    }
                } else {
                    l = 0 as libc::c_int;
                    while l < (*orinfo).nnbrs {
                        if (*onbrs.offset(l as isize)).pid == to {
                            break;
                        }
                        l += 1;
                        l;
                    }
                    if l == (*orinfo).nnbrs {
                        xgain -= *vsize.offset(ii as isize);
                    }
                    l = 0 as libc::c_int;
                    while l < (*orinfo).nnbrs {
                        if (*onbrs.offset(l as isize)).pid == from
                            && (*onbrs.offset(l as isize)).ned == 1 as libc::c_int
                        {
                            xgain += *vsize.offset(ii as isize);
                            break;
                        } else {
                            l += 1;
                            l;
                        }
                    }
                }
                j += 1;
                j;
            }
            (*graph).minvol -= xgain;
            (*graph).mincut -= -(*myrinfo).nid;
        } else {
            (*graph).minvol -= xgain + (*mynbrs.offset(k as isize)).gv;
            (*graph).mincut -= (*mynbrs.offset(k as isize)).ned - (*myrinfo).nid;
        }
        *where_0.offset(i as isize) = to;
        libmetis__iaxpy(
            (*graph).ncon as size_t,
            1 as libc::c_int,
            ((*graph).vwgt).offset((i * (*graph).ncon) as isize),
            1 as libc::c_int as size_t,
            ((*graph).pwgts).offset((to * (*graph).ncon) as isize),
            1 as libc::c_int as size_t,
        );
        libmetis__iaxpy(
            (*graph).ncon as size_t,
            -(1 as libc::c_int),
            ((*graph).vwgt).offset((i * (*graph).ncon) as isize),
            1 as libc::c_int as size_t,
            ((*graph).pwgts).offset((from * (*graph).ncon) as isize),
            1 as libc::c_int as size_t,
        );
        libmetis__KWayVolUpdate(
            ctrl,
            graph,
            i,
            from,
            to,
            0 as *mut ipq_t,
            0 as *mut idx_t,
            0 as *mut idx_t,
            0 as *mut idx_t,
            0 as *mut idx_t,
            1 as libc::c_int,
            vmarker,
            pmarker,
            modind,
        );
        iii += 1;
        iii;
    }
}
