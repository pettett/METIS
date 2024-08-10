use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn libmetis__ikvwspacemalloc(_: *mut ctrl_t, _: idx_t) -> *mut ikv_t;
    fn libmetis__cnbrpoolGetNext(ctrl: *mut ctrl_t, nnbrs: idx_t) -> idx_t;
    fn libmetis__vnbrpoolGetNext(ctrl: *mut ctrl_t, nnbrs: idx_t) -> idx_t;
    fn libmetis__wspacepush(ctrl: *mut ctrl_t);
    fn libmetis__iwspacemalloc(_: *mut ctrl_t, _: idx_t) -> *mut idx_t;
    fn libmetis__wspacepop(ctrl: *mut ctrl_t);
    fn libmetis__ivecaxpylez(
        n: idx_t,
        a: idx_t,
        x: *mut idx_t,
        y: *mut idx_t,
        z: *mut idx_t,
    ) -> libc::c_int;
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
    fn libmetis__ikvsortd(n: size_t, base: *mut ikv_t);
    fn libmetis__ikvsorti(n: size_t, base: *mut ikv_t);
    fn libmetis__iarray2csr(
        n: idx_t,
        range: idx_t,
        array: *mut idx_t,
        ptr: *mut idx_t,
        ind: *mut idx_t,
    );
    fn libmetis__ipqGetTop(queue: *mut ipq_t) -> idx_t;
    fn libmetis__ipqInsert(queue: *mut ipq_t, node: idx_t, key: idx_t) -> libc::c_int;
    fn libmetis__ipqFree(queue: *mut ipq_t);
    fn libmetis__ipqReset(queue: *mut ipq_t);
    fn libmetis__ipqInit(queue: *mut ipq_t, maxnodes: size_t);
    fn libmetis__iset(n: size_t, val: idx_t, x: *mut idx_t) -> *mut idx_t;
    fn libmetis__ismalloc(n: size_t, ival: idx_t, msg: *mut libc::c_char) -> *mut idx_t;
    fn libmetis__irealloc(
        ptr: *mut idx_t,
        n: size_t,
        msg: *mut libc::c_char,
    ) -> *mut idx_t;
    fn libmetis__iaxpy(
        n: size_t,
        alpha: idx_t,
        x: *mut idx_t,
        incx: size_t,
        y: *mut idx_t,
        incy: size_t,
    ) -> *mut idx_t;
    fn libmetis__isum(n: size_t, x: *mut idx_t, incx: size_t) -> idx_t;
    fn libmetis__iargmax(n: size_t, x: *mut idx_t) -> size_t;
    fn gk_free(ptr1: *mut *mut libc::c_void, _: ...);
    fn gk_errexit(signum: libc::c_int, _: *mut libc::c_char, _: ...);
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
pub unsafe extern "C" fn libmetis__ComputeSubDomainGraph(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
) {
    let mut i: idx_t = 0;
    let mut ii: idx_t = 0;
    let mut j: idx_t = 0;
    let mut pid: idx_t = 0;
    let mut other: idx_t = 0;
    let mut nparts: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut nnbrs: idx_t = 0;
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut adjwgt: *mut idx_t = 0 as *mut idx_t;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    let mut pptr: *mut idx_t = 0 as *mut idx_t;
    let mut pind: *mut idx_t = 0 as *mut idx_t;
    let mut nads: idx_t = 0 as libc::c_int;
    let mut vadids: *mut idx_t = 0 as *mut idx_t;
    let mut vadwgts: *mut idx_t = 0 as *mut idx_t;
    libmetis__wspacepush(ctrl);
    nvtxs = (*graph).nvtxs;
    xadj = (*graph).xadj;
    adjncy = (*graph).adjncy;
    adjwgt = (*graph).adjwgt;
    where_0 = (*graph).where_0;
    nparts = (*ctrl).nparts;
    vadids = (*ctrl).pvec1;
    vadwgts = libmetis__iset(nparts as size_t, 0 as libc::c_int, (*ctrl).pvec2);
    pptr = libmetis__iwspacemalloc(ctrl, nparts + 1 as libc::c_int);
    pind = libmetis__iwspacemalloc(ctrl, nvtxs);
    libmetis__iarray2csr(nvtxs, nparts, where_0, pptr, pind);
    pid = 0 as libc::c_int;
    while pid < nparts {
        match (*ctrl).objtype as libc::c_uint {
            0 => {
                let mut rinfo: *mut ckrinfo_t = 0 as *mut ckrinfo_t;
                let mut nbrs: *mut cnbr_t = 0 as *mut cnbr_t;
                rinfo = (*graph).ckrinfo;
                nads = 0 as libc::c_int;
                ii = *pptr.offset(pid as isize);
                while ii < *pptr.offset((pid + 1 as libc::c_int) as isize) {
                    i = *pind.offset(ii as isize);
                    if (*rinfo.offset(i as isize)).ed > 0 as libc::c_int {
                        nnbrs = (*rinfo.offset(i as isize)).nnbrs;
                        nbrs = ((*ctrl).cnbrpool)
                            .offset((*rinfo.offset(i as isize)).inbr as isize);
                        j = 0 as libc::c_int;
                        while j < nnbrs {
                            other = (*nbrs.offset(j as isize)).pid;
                            if *vadwgts.offset(other as isize) == 0 as libc::c_int {
                                let fresh0 = nads;
                                nads = nads + 1;
                                *vadids.offset(fresh0 as isize) = other;
                            }
                            let ref mut fresh1 = *vadwgts.offset(other as isize);
                            *fresh1 += (*nbrs.offset(j as isize)).ed;
                            j += 1;
                            j;
                        }
                    }
                    ii += 1;
                    ii;
                }
            }
            1 => {
                let mut rinfo_0: *mut vkrinfo_t = 0 as *mut vkrinfo_t;
                let mut nbrs_0: *mut vnbr_t = 0 as *mut vnbr_t;
                rinfo_0 = (*graph).vkrinfo;
                nads = 0 as libc::c_int;
                ii = *pptr.offset(pid as isize);
                while ii < *pptr.offset((pid + 1 as libc::c_int) as isize) {
                    i = *pind.offset(ii as isize);
                    if (*rinfo_0.offset(i as isize)).ned > 0 as libc::c_int {
                        nnbrs = (*rinfo_0.offset(i as isize)).nnbrs;
                        nbrs_0 = ((*ctrl).vnbrpool)
                            .offset((*rinfo_0.offset(i as isize)).inbr as isize);
                        j = 0 as libc::c_int;
                        while j < nnbrs {
                            other = (*nbrs_0.offset(j as isize)).pid;
                            if *vadwgts.offset(other as isize) == 0 as libc::c_int {
                                let fresh2 = nads;
                                nads = nads + 1;
                                *vadids.offset(fresh2 as isize) = other;
                            }
                            let ref mut fresh3 = *vadwgts.offset(other as isize);
                            *fresh3 += (*nbrs_0.offset(j as isize)).ned;
                            j += 1;
                            j;
                        }
                    }
                    ii += 1;
                    ii;
                }
            }
            _ => {
                gk_errexit(
                    15 as libc::c_int,
                    b"Unknown objtype: %d\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    (*ctrl).objtype as libc::c_uint,
                );
            }
        }
        if *((*ctrl).maxnads).offset(pid as isize) < nads {
            *((*ctrl).maxnads).offset(pid as isize) = 2 as libc::c_int * nads;
            let ref mut fresh4 = *((*ctrl).adids).offset(pid as isize);
            *fresh4 = libmetis__irealloc(
                *((*ctrl).adids).offset(pid as isize),
                *((*ctrl).maxnads).offset(pid as isize) as size_t,
                b"ComputeSubDomainGraph: adids[pid]\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
            let ref mut fresh5 = *((*ctrl).adwgts).offset(pid as isize);
            *fresh5 = libmetis__irealloc(
                *((*ctrl).adwgts).offset(pid as isize),
                *((*ctrl).maxnads).offset(pid as isize) as size_t,
                b"ComputeSubDomainGraph: adids[pid]\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
        *((*ctrl).nads).offset(pid as isize) = nads;
        j = 0 as libc::c_int;
        while j < nads {
            *(*((*ctrl).adids).offset(pid as isize))
                .offset(j as isize) = *vadids.offset(j as isize);
            *(*((*ctrl).adwgts).offset(pid as isize))
                .offset(
                    j as isize,
                ) = *vadwgts.offset(*vadids.offset(j as isize) as isize);
            *vadwgts.offset(*vadids.offset(j as isize) as isize) = 0 as libc::c_int;
            j += 1;
            j;
        }
        pid += 1;
        pid;
    }
    libmetis__wspacepop(ctrl);
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__UpdateEdgeSubDomainGraph(
    mut ctrl: *mut ctrl_t,
    mut u: idx_t,
    mut v: idx_t,
    mut ewgt: idx_t,
    mut r_maxndoms: *mut idx_t,
) {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut nads: idx_t = 0;
    if ewgt == 0 as libc::c_int {
        return;
    }
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        nads = *((*ctrl).nads).offset(u as isize);
        j = 0 as libc::c_int;
        while j < nads {
            if *(*((*ctrl).adids).offset(u as isize)).offset(j as isize) == v {
                let ref mut fresh6 = *(*((*ctrl).adwgts).offset(u as isize))
                    .offset(j as isize);
                *fresh6 += ewgt;
                break;
            } else {
                j += 1;
                j;
            }
        }
        if j == nads {
            if *((*ctrl).maxnads).offset(u as isize) == nads {
                *((*ctrl).maxnads)
                    .offset(u as isize) = 2 as libc::c_int * (nads + 1 as libc::c_int);
                let ref mut fresh7 = *((*ctrl).adids).offset(u as isize);
                *fresh7 = libmetis__irealloc(
                    *((*ctrl).adids).offset(u as isize),
                    *((*ctrl).maxnads).offset(u as isize) as size_t,
                    b"IncreaseEdgeSubDomainGraph: adids[pid]\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
                let ref mut fresh8 = *((*ctrl).adwgts).offset(u as isize);
                *fresh8 = libmetis__irealloc(
                    *((*ctrl).adwgts).offset(u as isize),
                    *((*ctrl).maxnads).offset(u as isize) as size_t,
                    b"IncreaseEdgeSubDomainGraph: adids[pid]\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
            }
            *(*((*ctrl).adids).offset(u as isize)).offset(nads as isize) = v;
            *(*((*ctrl).adwgts).offset(u as isize)).offset(nads as isize) = ewgt;
            nads += 1;
            nads;
            if !r_maxndoms.is_null() && nads > *r_maxndoms {
                printf(
                    b"You just increased the maxndoms: %d %d\n\0" as *const u8
                        as *const libc::c_char,
                    nads,
                    *r_maxndoms,
                );
                *r_maxndoms = nads;
            }
        } else if *(*((*ctrl).adwgts).offset(u as isize)).offset(j as isize)
            == 0 as libc::c_int
        {
            *(*((*ctrl).adids).offset(u as isize))
                .offset(
                    j as isize,
                ) = *(*((*ctrl).adids).offset(u as isize))
                .offset((nads - 1 as libc::c_int) as isize);
            *(*((*ctrl).adwgts).offset(u as isize))
                .offset(
                    j as isize,
                ) = *(*((*ctrl).adwgts).offset(u as isize))
                .offset((nads - 1 as libc::c_int) as isize);
            nads -= 1;
            nads;
            if !r_maxndoms.is_null() && nads + 1 as libc::c_int == *r_maxndoms {
                *r_maxndoms = *((*ctrl).nads)
                    .offset(
                        libmetis__iargmax((*ctrl).nparts as size_t, (*ctrl).nads)
                            as isize,
                    );
            }
        }
        *((*ctrl).nads).offset(u as isize) = nads;
        j = u;
        u = v;
        v = j;
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__EliminateSubDomainEdges(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
) {
    let mut i: idx_t = 0;
    let mut ii: idx_t = 0;
    let mut j: idx_t = 0;
    let mut k: idx_t = 0;
    let mut ncon: idx_t = 0;
    let mut nparts: idx_t = 0;
    let mut scheme: idx_t = 0;
    let mut pid_from: idx_t = 0;
    let mut pid_to: idx_t = 0;
    let mut me: idx_t = 0;
    let mut other: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut total: idx_t = 0;
    let mut max: idx_t = 0;
    let mut avg: idx_t = 0;
    let mut totalout: idx_t = 0;
    let mut nind: idx_t = 0 as libc::c_int;
    let mut ncand: idx_t = 0 as libc::c_int;
    let mut ncand2: idx_t = 0;
    let mut target: idx_t = 0;
    let mut target2: idx_t = 0;
    let mut nadd: idx_t = 0;
    let mut bestnadd: idx_t = 0 as libc::c_int;
    let mut min: idx_t = 0;
    let mut move_0: idx_t = 0;
    let mut cpwgt: *mut idx_t = 0 as *mut idx_t;
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut vwgt: *mut idx_t = 0 as *mut idx_t;
    let mut adjwgt: *mut idx_t = 0 as *mut idx_t;
    let mut pwgts: *mut idx_t = 0 as *mut idx_t;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    let mut maxpwgt: *mut idx_t = 0 as *mut idx_t;
    let mut mypmat: *mut idx_t = 0 as *mut idx_t;
    let mut otherpmat: *mut idx_t = 0 as *mut idx_t;
    let mut kpmat: *mut idx_t = 0 as *mut idx_t;
    let mut ind: *mut idx_t = 0 as *mut idx_t;
    let mut nads: *mut idx_t = 0 as *mut idx_t;
    let mut adids: *mut *mut idx_t = 0 as *mut *mut idx_t;
    let mut adwgts: *mut *mut idx_t = 0 as *mut *mut idx_t;
    let mut cand: *mut ikv_t = 0 as *mut ikv_t;
    let mut cand2: *mut ikv_t = 0 as *mut ikv_t;
    let mut queue: ipq_t = ipq_t {
        nnodes: 0,
        maxnodes: 0,
        heap: 0 as *mut ikv_t,
        locator: 0 as *mut gk_idx_t,
    };
    let mut tpwgts: *mut real_t = 0 as *mut real_t;
    let mut badfactor: real_t = 1.4f64 as real_t;
    let mut pptr: *mut idx_t = 0 as *mut idx_t;
    let mut pind: *mut idx_t = 0 as *mut idx_t;
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
    cpwgt = libmetis__iwspacemalloc(ctrl, ncon);
    maxpwgt = libmetis__iwspacemalloc(ctrl, nparts * ncon);
    ind = libmetis__iwspacemalloc(ctrl, nvtxs);
    otherpmat = libmetis__iset(
        nparts as size_t,
        0 as libc::c_int,
        libmetis__iwspacemalloc(ctrl, nparts),
    );
    cand = libmetis__ikvwspacemalloc(ctrl, nparts);
    cand2 = libmetis__ikvwspacemalloc(ctrl, nparts);
    pptr = libmetis__iwspacemalloc(ctrl, nparts + 1 as libc::c_int);
    pind = libmetis__iwspacemalloc(ctrl, nvtxs);
    libmetis__iarray2csr(nvtxs, nparts, where_0, pptr, pind);
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
    libmetis__ComputeSubDomainGraph(ctrl, graph);
    nads = (*ctrl).nads;
    adids = (*ctrl).adids;
    adwgts = (*ctrl).adwgts;
    mypmat = libmetis__iset(nparts as size_t, 0 as libc::c_int, (*ctrl).pvec1);
    kpmat = libmetis__iset(nparts as size_t, 0 as libc::c_int, (*ctrl).pvec2);
    i = 0 as libc::c_int;
    while i < nparts {
        j = 0 as libc::c_int;
        while j < ncon {
            *maxpwgt
                .offset(
                    (i * ncon + j) as isize,
                ) = ((if ncon == 1 as libc::c_int { 1.25f64 } else { 1.025f64 })
                * *tpwgts.offset(i as isize) as libc::c_double
                * *((*graph).tvwgt).offset(j as isize) as libc::c_double
                * *((*ctrl).ubfactors).offset(j as isize) as libc::c_double) as idx_t;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    libmetis__ipqInit(&mut queue, nparts as size_t);
    loop {
        total = libmetis__isum(nparts as size_t, nads, 1 as libc::c_int as size_t);
        avg = total / nparts;
        max = *nads.offset(libmetis__iargmax(nparts as size_t, nads) as isize);
        if (*ctrl).dbglvl as libc::c_uint
            & METIS_DBG_CONNINFO as libc::c_int as libc::c_uint != 0
        {
            printf(
                b"Adjacent Subdomain Stats: Total: %3d, Max: %3d[%zu], Avg: %3d\n\0"
                    as *const u8 as *const libc::c_char,
                total,
                max,
                libmetis__iargmax(nparts as size_t, nads),
                avg,
            );
        }
        if (max as libc::c_float) < badfactor * avg as libc::c_float {
            break;
        }
        libmetis__ipqReset(&mut queue);
        i = 0 as libc::c_int;
        while i < nparts {
            if *nads.offset(i as isize) >= avg + (max - avg) / 2 as libc::c_int {
                libmetis__ipqInsert(&mut queue, i, *nads.offset(i as isize));
            }
            i += 1;
            i;
        }
        move_0 = 0 as libc::c_int;
        loop {
            me = libmetis__ipqGetTop(&mut queue);
            if !(me != -(1 as libc::c_int)) {
                break;
            }
            totalout = libmetis__isum(
                *nads.offset(me as isize) as size_t,
                *adwgts.offset(me as isize),
                1 as libc::c_int as size_t,
            );
            ncand2 = 0 as libc::c_int;
            i = 0 as libc::c_int;
            while i < *nads.offset(me as isize) {
                *mypmat
                    .offset(
                        *(*adids.offset(me as isize)).offset(i as isize) as isize,
                    ) = *(*adwgts.offset(me as isize)).offset(i as isize);
                if 2 as libc::c_int * *nads.offset(me as isize)
                    * *(*adwgts.offset(me as isize)).offset(i as isize) < totalout
                {
                    (*cand2.offset(ncand2 as isize))
                        .val = *(*adids.offset(me as isize)).offset(i as isize);
                    let fresh9 = ncand2;
                    ncand2 = ncand2 + 1;
                    (*cand2.offset(fresh9 as isize))
                        .key = *(*adwgts.offset(me as isize)).offset(i as isize);
                }
                i += 1;
                i;
            }
            if (*ctrl).dbglvl as libc::c_uint
                & METIS_DBG_CONNINFO as libc::c_int as libc::c_uint != 0
            {
                printf(
                    b"Me: %d, Degree: %4d, TotalOut: %d,\n\0" as *const u8
                        as *const libc::c_char,
                    me,
                    *nads.offset(me as isize),
                    totalout,
                );
            }
            libmetis__ikvsorti(ncand2 as size_t, cand2);
            target2 = -(1 as libc::c_int);
            target = target2;
            scheme = 0 as libc::c_int;
            while scheme < 2 as libc::c_int {
                min = 0 as libc::c_int;
                while min < ncand2 {
                    other = (*cand2.offset(min as isize)).val;
                    if scheme == 0 as libc::c_int {
                        pid_from = other;
                        pid_to = me;
                    } else {
                        pid_from = me;
                        pid_to = other;
                    }
                    nind = 0 as libc::c_int;
                    ii = *pptr.offset(pid_from as isize);
                    while ii < *pptr.offset((pid_from + 1 as libc::c_int) as isize) {
                        i = *pind.offset(ii as isize);
                        j = *xadj.offset(i as isize);
                        while j < *xadj.offset((i + 1 as libc::c_int) as isize) {
                            if *where_0.offset(*adjncy.offset(j as isize) as isize)
                                == pid_to
                            {
                                let fresh10 = nind;
                                nind = nind + 1;
                                *ind.offset(fresh10 as isize) = i;
                                break;
                            } else {
                                j += 1;
                                j;
                            }
                        }
                        ii += 1;
                        ii;
                    }
                    libmetis__iset(ncon as size_t, 0 as libc::c_int, cpwgt);
                    ncand = 0 as libc::c_int;
                    ii = 0 as libc::c_int;
                    while ii < nind {
                        i = *ind.offset(ii as isize);
                        libmetis__iaxpy(
                            ncon as size_t,
                            1 as libc::c_int,
                            vwgt.offset((i * ncon) as isize),
                            1 as libc::c_int as size_t,
                            cpwgt,
                            1 as libc::c_int as size_t,
                        );
                        j = *xadj.offset(i as isize);
                        while j < *xadj.offset((i + 1 as libc::c_int) as isize) {
                            k = *where_0.offset(*adjncy.offset(j as isize) as isize);
                            if !(k == pid_from) {
                                if *otherpmat.offset(k as isize) == 0 as libc::c_int {
                                    let fresh11 = ncand;
                                    ncand = ncand + 1;
                                    (*cand.offset(fresh11 as isize)).val = k;
                                }
                                let ref mut fresh12 = *otherpmat.offset(k as isize);
                                *fresh12
                                    += if !adjwgt.is_null() {
                                        *adjwgt.offset(j as isize)
                                    } else {
                                        1 as libc::c_int
                                    };
                            }
                            j += 1;
                            j;
                        }
                        ii += 1;
                        ii;
                    }
                    i = 0 as libc::c_int;
                    while i < ncand {
                        (*cand.offset(i as isize))
                            .key = *otherpmat
                            .offset((*cand.offset(i as isize)).val as isize);
                        i += 1;
                        i;
                    }
                    libmetis__ikvsortd(ncand as size_t, cand);
                    if (*ctrl).dbglvl as libc::c_uint
                        & METIS_DBG_CONNINFO as libc::c_int as libc::c_uint != 0
                    {
                        printf(
                            b"\tMinOut: %4d, to: %3d, TtlWgt: %5d[#:%d]\n\0" as *const u8
                                as *const libc::c_char,
                            *mypmat.offset(other as isize),
                            other,
                            libmetis__isum(
                                ncon as size_t,
                                cpwgt,
                                1 as libc::c_int as size_t,
                            ),
                            nind,
                        );
                    }
                    let mut current_block_132: u64;
                    i = 0 as libc::c_int;
                    while i < ncand {
                        k = (*cand.offset(i as isize)).val;
                        if *mypmat.offset(k as isize) > 0 as libc::c_int {
                            if libmetis__ivecaxpylez(
                                ncon,
                                1 as libc::c_int,
                                cpwgt,
                                pwgts.offset((k * ncon) as isize),
                                maxpwgt.offset((k * ncon) as isize),
                            ) == 0
                            {
                                current_block_132 = 9505035279996566320;
                            } else {
                                j = 0 as libc::c_int;
                                while j < *nads.offset(k as isize) {
                                    *kpmat
                                        .offset(
                                            *(*adids.offset(k as isize)).offset(j as isize) as isize,
                                        ) = *(*adwgts.offset(k as isize)).offset(j as isize);
                                    j += 1;
                                    j;
                                }
                                j = 0 as libc::c_int;
                                while j < nparts {
                                    if *otherpmat.offset(j as isize) > 0 as libc::c_int
                                        && *kpmat.offset(j as isize) == 0 as libc::c_int
                                        && *nads.offset(j as isize) + 1 as libc::c_int
                                            >= *nads.offset(me as isize)
                                    {
                                        break;
                                    }
                                    j += 1;
                                    j;
                                }
                                if j == nparts {
                                    nadd = 0 as libc::c_int;
                                    j = 0 as libc::c_int;
                                    while j < nparts {
                                        if *otherpmat.offset(j as isize) > 0 as libc::c_int
                                            && *kpmat.offset(j as isize) == 0 as libc::c_int
                                        {
                                            nadd += 1;
                                            nadd;
                                        }
                                        j += 1;
                                        j;
                                    }
                                    if (*ctrl).dbglvl as libc::c_uint
                                        & METIS_DBG_CONNINFO as libc::c_int as libc::c_uint != 0
                                    {
                                        printf(
                                            b"\t\tto=%d, nadd=%d, %d\n\0" as *const u8
                                                as *const libc::c_char,
                                            k,
                                            nadd,
                                            *nads.offset(k as isize),
                                        );
                                    }
                                    if *nads.offset(k as isize) + nadd
                                        < *nads.offset(me as isize)
                                    {
                                        if target2 == -(1 as libc::c_int)
                                            || *nads.offset(target2 as isize) + bestnadd
                                                > *nads.offset(k as isize) + nadd
                                            || *nads.offset(target2 as isize) + bestnadd
                                                == *nads.offset(k as isize) + nadd && bestnadd > nadd
                                        {
                                            target2 = k;
                                            bestnadd = nadd;
                                        }
                                    }
                                    if nadd == 0 as libc::c_int {
                                        target = k;
                                    }
                                }
                                j = 0 as libc::c_int;
                                while j < *nads.offset(k as isize) {
                                    *kpmat
                                        .offset(
                                            *(*adids.offset(k as isize)).offset(j as isize) as isize,
                                        ) = 0 as libc::c_int;
                                    j += 1;
                                    j;
                                }
                                current_block_132 = 13253659531982233645;
                            }
                        } else {
                            current_block_132 = 13253659531982233645;
                        }
                        match current_block_132 {
                            13253659531982233645 => {
                                if target != -(1 as libc::c_int) {
                                    break;
                                }
                            }
                            _ => {}
                        }
                        i += 1;
                        i;
                    }
                    i = 0 as libc::c_int;
                    while i < ncand {
                        *otherpmat
                            .offset(
                                (*cand.offset(i as isize)).val as isize,
                            ) = 0 as libc::c_int;
                        i += 1;
                        i;
                    }
                    if target == -(1 as libc::c_int) && target2 != -(1 as libc::c_int) {
                        target = target2;
                    }
                    if target != -(1 as libc::c_int) {
                        if (*ctrl).dbglvl as libc::c_uint
                            & METIS_DBG_CONNINFO as libc::c_int as libc::c_uint != 0
                        {
                            printf(
                                b"\t\tScheme: %d. Moving to %d\n\0" as *const u8
                                    as *const libc::c_char,
                                scheme,
                                target,
                            );
                        }
                        move_0 = 1 as libc::c_int;
                        break;
                    } else {
                        min += 1;
                        min;
                    }
                }
                if target != -(1 as libc::c_int) {
                    break;
                }
                scheme += 1;
                scheme;
            }
            i = 0 as libc::c_int;
            while i < *nads.offset(me as isize) {
                *mypmat
                    .offset(
                        *(*adids.offset(me as isize)).offset(i as isize) as isize,
                    ) = 0 as libc::c_int;
                i += 1;
                i;
            }
            if target != -(1 as libc::c_int) {
                match (*ctrl).objtype as libc::c_uint {
                    0 => {
                        libmetis__MoveGroupMinConnForCut(ctrl, graph, target, nind, ind);
                    }
                    1 => {
                        libmetis__MoveGroupMinConnForVol(
                            ctrl,
                            graph,
                            target,
                            nind,
                            ind,
                            vmarker,
                            pmarker,
                            modind,
                        );
                    }
                    _ => {
                        gk_errexit(
                            15 as libc::c_int,
                            b"Unknown objtype of %d\n\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                            (*ctrl).objtype as libc::c_uint,
                        );
                    }
                }
                libmetis__iarray2csr(nvtxs, nparts, where_0, pptr, pind);
            }
        }
        if move_0 == 0 as libc::c_int {
            break;
        }
    }
    libmetis__ipqFree(&mut queue);
    libmetis__wspacepop(ctrl);
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__MoveGroupMinConnForCut(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut to: idx_t,
    mut nind: idx_t,
    mut ind: *mut idx_t,
) {
    let mut i: idx_t = 0;
    let mut ii: idx_t = 0;
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
    loop {
        nind -= 1;
        if !(nind >= 0 as libc::c_int) {
            break;
        }
        i = *ind.offset(nind as isize);
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
        (*graph).mincut -= (*mynbrs.offset(k as isize)).ed - (*myrinfo).id;
        libmetis__UpdateEdgeSubDomainGraph(
            ctrl,
            from,
            to,
            (*myrinfo).id - (*mynbrs.offset(k as isize)).ed,
            0 as *mut idx_t,
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
                let fresh13 = nbnd;
                nbnd = nbnd + 1;
                *bndptr.offset(i as isize) = fresh13;
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
                let fresh14 = nbnd;
                nbnd = nbnd + 1;
                *bndptr.offset(i as isize) = fresh14;
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
                        let fresh15 = nbnd;
                        nbnd = nbnd + 1;
                        *bndptr.offset(ii as isize) = fresh15;
                    }
                } else if (*myrinfo).ed > 0 as libc::c_int
                    && *bndptr.offset(ii as isize) == -(1 as libc::c_int)
                {
                    *bndind.offset(nbnd as isize) = ii;
                    let fresh16 = nbnd;
                    nbnd = nbnd + 1;
                    *bndptr.offset(ii as isize) = fresh16;
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
                            let ref mut fresh17 = (*mynbrs_0.offset(k_0 as isize)).ed;
                            *fresh17 -= *adjwgt.offset(j as isize);
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
                        let ref mut fresh18 = (*mynbrs_0.offset(k_0 as isize)).ed;
                        *fresh18 += *adjwgt.offset(j as isize);
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
            if me != from && me != to {
                libmetis__UpdateEdgeSubDomainGraph(
                    ctrl,
                    from,
                    me,
                    -*adjwgt.offset(j as isize),
                    0 as *mut idx_t,
                );
                libmetis__UpdateEdgeSubDomainGraph(
                    ctrl,
                    to,
                    me,
                    *adjwgt.offset(j as isize),
                    0 as *mut idx_t,
                );
            }
            j += 1;
            j;
        }
    }
    (*graph).nbnd = nbnd;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__MoveGroupMinConnForVol(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut to: idx_t,
    mut nind: idx_t,
    mut ind: *mut idx_t,
    mut vmarker: *mut idx_t,
    mut pmarker: *mut idx_t,
    mut modind: *mut idx_t,
) {
    let mut i: idx_t = 0;
    let mut ii: idx_t = 0;
    let mut j: idx_t = 0;
    let mut jj: idx_t = 0;
    let mut k: idx_t = 0;
    let mut l: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut from: idx_t = 0;
    let mut me: idx_t = 0;
    let mut other: idx_t = 0;
    let mut xgain: idx_t = 0;
    let mut ewgt: idx_t = 0;
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
    loop {
        nind -= 1;
        if !(nind >= 0 as libc::c_int) {
            break;
        }
        i = *ind.offset(nind as isize);
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
            ewgt = (*myrinfo).nid;
        } else {
            (*graph).minvol -= xgain + (*mynbrs.offset(k as isize)).gv;
            (*graph).mincut -= (*mynbrs.offset(k as isize)).ned - (*myrinfo).nid;
            ewgt = (*myrinfo).nid - (*mynbrs.offset(k as isize)).ned;
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
        libmetis__UpdateEdgeSubDomainGraph(ctrl, from, to, ewgt, 0 as *mut idx_t);
        j = *xadj.offset(i as isize);
        while j < *xadj.offset((i + 1 as libc::c_int) as isize) {
            me = *where_0.offset(*adjncy.offset(j as isize) as isize);
            if me != from && me != to {
                libmetis__UpdateEdgeSubDomainGraph(
                    ctrl,
                    from,
                    me,
                    -(1 as libc::c_int),
                    0 as *mut idx_t,
                );
                libmetis__UpdateEdgeSubDomainGraph(
                    ctrl,
                    to,
                    me,
                    1 as libc::c_int,
                    0 as *mut idx_t,
                );
            }
            j += 1;
            j;
        }
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
    };
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__PrintSubDomainGraph(
    mut graph: *mut graph_t,
    mut nparts: idx_t,
    mut where_0: *mut idx_t,
) {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut k: idx_t = 0;
    let mut me: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut total: idx_t = 0;
    let mut max: idx_t = 0;
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut adjwgt: *mut idx_t = 0 as *mut idx_t;
    let mut pmat: *mut idx_t = 0 as *mut idx_t;
    nvtxs = (*graph).nvtxs;
    xadj = (*graph).xadj;
    adjncy = (*graph).adjncy;
    adjwgt = (*graph).adjwgt;
    pmat = libmetis__ismalloc(
        (nparts * nparts) as size_t,
        0 as libc::c_int,
        b"ComputeSubDomainGraph: pmat\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < nvtxs {
        me = *where_0.offset(i as isize);
        j = *xadj.offset(i as isize);
        while j < *xadj.offset((i + 1 as libc::c_int) as isize) {
            k = *adjncy.offset(j as isize);
            if *where_0.offset(k as isize) != me {
                let ref mut fresh19 = *pmat
                    .offset((me * nparts + *where_0.offset(k as isize)) as isize);
                *fresh19 += *adjwgt.offset(j as isize);
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    max = 0 as libc::c_int;
    total = max;
    i = 0 as libc::c_int;
    while i < nparts {
        k = 0 as libc::c_int;
        j = 0 as libc::c_int;
        while j < nparts {
            if *pmat.offset((i * nparts + j) as isize) > 0 as libc::c_int {
                k += 1;
                k;
            }
            j += 1;
            j;
        }
        total += k;
        if k > max {
            max = k;
        }
        i += 1;
        i;
    }
    printf(
        b"Total adjacent subdomains: %d, Max: %d\n\0" as *const u8
            as *const libc::c_char,
        total,
        max,
    );
    gk_free(
        &mut pmat as *mut *mut idx_t as *mut *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
}
