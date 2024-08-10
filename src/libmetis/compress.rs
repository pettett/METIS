use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn gk_free(ptr1: *mut *mut libc::c_void, _: ...);
    fn libmetis__imalloc(n: size_t, msg: *mut libc::c_char) -> *mut idx_t;
    fn libmetis__ismalloc(n: size_t, ival: idx_t, msg: *mut libc::c_char) -> *mut idx_t;
    fn libmetis__iset(n: size_t, val: idx_t, x: *mut idx_t) -> *mut idx_t;
    fn libmetis__ikvmalloc(n: size_t, msg: *mut libc::c_char) -> *mut ikv_t;
    fn libmetis__ikvsorti(n: size_t, base: *mut ikv_t);
    fn libmetis__SetupGraph_label(graph: *mut graph_t);
    fn libmetis__SetupGraph_tvwgt(graph: *mut graph_t);
    fn libmetis__CreateGraph() -> *mut graph_t;
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
pub struct ikv_t {
    pub key: idx_t,
    pub val: idx_t,
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
pub unsafe extern "C" fn libmetis__CompressGraph(
    mut ctrl: *mut ctrl_t,
    mut nvtxs: idx_t,
    mut xadj: *mut idx_t,
    mut adjncy: *mut idx_t,
    mut vwgt: *mut idx_t,
    mut cptr: *mut idx_t,
    mut cind: *mut idx_t,
) -> *mut graph_t {
    let mut i: idx_t = 0;
    let mut ii: idx_t = 0;
    let mut iii: idx_t = 0;
    let mut j: idx_t = 0;
    let mut jj: idx_t = 0;
    let mut k: idx_t = 0;
    let mut l: idx_t = 0;
    let mut cnvtxs: idx_t = 0;
    let mut cnedges: idx_t = 0;
    let mut cxadj: *mut idx_t = 0 as *mut idx_t;
    let mut cadjncy: *mut idx_t = 0 as *mut idx_t;
    let mut cvwgt: *mut idx_t = 0 as *mut idx_t;
    let mut mark: *mut idx_t = 0 as *mut idx_t;
    let mut map: *mut idx_t = 0 as *mut idx_t;
    let mut keys: *mut ikv_t = 0 as *mut ikv_t;
    let mut graph: *mut graph_t = 0 as *mut graph_t;
    mark = libmetis__ismalloc(
        nvtxs as size_t,
        -(1 as libc::c_int),
        b"CompressGraph: mark\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    map = libmetis__ismalloc(
        nvtxs as size_t,
        -(1 as libc::c_int),
        b"CompressGraph: map\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    keys = libmetis__ikvmalloc(
        nvtxs as size_t,
        b"CompressGraph: keys\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < nvtxs {
        k = 0 as libc::c_int;
        j = *xadj.offset(i as isize);
        while j < *xadj.offset((i + 1 as libc::c_int) as isize) {
            k += *adjncy.offset(j as isize);
            j += 1;
            j;
        }
        (*keys.offset(i as isize)).key = k + i;
        (*keys.offset(i as isize)).val = i;
        i += 1;
        i;
    }
    libmetis__ikvsorti(nvtxs as size_t, keys);
    let ref mut fresh0 = *cptr.offset(0 as libc::c_int as isize);
    *fresh0 = 0 as libc::c_int;
    l = *fresh0;
    i = 0 as libc::c_int;
    cnvtxs = i;
    while i < nvtxs {
        ii = (*keys.offset(i as isize)).val;
        if *map.offset(ii as isize) == -(1 as libc::c_int) {
            *mark.offset(ii as isize) = i;
            j = *xadj.offset(ii as isize);
            while j < *xadj.offset((ii + 1 as libc::c_int) as isize) {
                *mark.offset(*adjncy.offset(j as isize) as isize) = i;
                j += 1;
                j;
            }
            *map.offset(ii as isize) = cnvtxs;
            let fresh1 = l;
            l = l + 1;
            *cind.offset(fresh1 as isize) = ii;
            j = i + 1 as libc::c_int;
            while j < nvtxs {
                iii = (*keys.offset(j as isize)).val;
                if (*keys.offset(i as isize)).key != (*keys.offset(j as isize)).key
                    || *xadj.offset((ii + 1 as libc::c_int) as isize)
                        - *xadj.offset(ii as isize)
                        != *xadj.offset((iii + 1 as libc::c_int) as isize)
                            - *xadj.offset(iii as isize)
                {
                    break;
                }
                if *map.offset(iii as isize) == -(1 as libc::c_int) {
                    jj = *xadj.offset(iii as isize);
                    while jj < *xadj.offset((iii + 1 as libc::c_int) as isize) {
                        if *mark.offset(*adjncy.offset(jj as isize) as isize) != i {
                            break;
                        }
                        jj += 1;
                        jj;
                    }
                    if jj == *xadj.offset((iii + 1 as libc::c_int) as isize) {
                        *map.offset(iii as isize) = cnvtxs;
                        let fresh2 = l;
                        l = l + 1;
                        *cind.offset(fresh2 as isize) = iii;
                    }
                }
                j += 1;
                j;
            }
            cnvtxs += 1;
            *cptr.offset(cnvtxs as isize) = l;
        }
        i += 1;
        i;
    }
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_INFO as libc::c_int as libc::c_uint
        != 0
    {
        printf(
            b"  Compression: reduction in # of vertices: %d.\n\0" as *const u8
                as *const libc::c_char,
            nvtxs - cnvtxs,
        );
    }
    if (cnvtxs as libc::c_double) < 0.85f64 * nvtxs as libc::c_double {
        graph = libmetis__CreateGraph();
        cnedges = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < cnvtxs {
            ii = *cind.offset(*cptr.offset(i as isize) as isize);
            cnedges
                += *xadj.offset((ii + 1 as libc::c_int) as isize)
                    - *xadj.offset(ii as isize);
            i += 1;
            i;
        }
        (*graph)
            .xadj = libmetis__imalloc(
            (cnvtxs + 1 as libc::c_int) as size_t,
            b"CompressGraph: xadj\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        cxadj = (*graph).xadj;
        (*graph)
            .vwgt = libmetis__ismalloc(
            cnvtxs as size_t,
            0 as libc::c_int,
            b"CompressGraph: vwgt\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        cvwgt = (*graph).vwgt;
        (*graph)
            .adjncy = libmetis__imalloc(
            cnedges as size_t,
            b"CompressGraph: adjncy\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        cadjncy = (*graph).adjncy;
        (*graph)
            .adjwgt = libmetis__ismalloc(
            cnedges as size_t,
            1 as libc::c_int,
            b"CompressGraph: adjwgt\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        libmetis__iset(nvtxs as size_t, -(1 as libc::c_int), mark);
        let ref mut fresh3 = *cxadj.offset(0 as libc::c_int as isize);
        *fresh3 = 0 as libc::c_int;
        l = *fresh3;
        i = 0 as libc::c_int;
        while i < cnvtxs {
            *mark.offset(i as isize) = i;
            j = *cptr.offset(i as isize);
            while j < *cptr.offset((i + 1 as libc::c_int) as isize) {
                ii = *cind.offset(j as isize);
                let ref mut fresh4 = *cvwgt.offset(i as isize);
                *fresh4
                    += if vwgt.is_null() {
                        1 as libc::c_int
                    } else {
                        *vwgt.offset(ii as isize)
                    };
                jj = *xadj.offset(ii as isize);
                while jj < *xadj.offset((ii + 1 as libc::c_int) as isize) {
                    k = *map.offset(*adjncy.offset(jj as isize) as isize);
                    if *mark.offset(k as isize) != i {
                        *mark.offset(k as isize) = i;
                        let fresh5 = l;
                        l = l + 1;
                        *cadjncy.offset(fresh5 as isize) = k;
                    }
                    jj += 1;
                    jj;
                }
                j += 1;
                j;
            }
            *cxadj.offset((i + 1 as libc::c_int) as isize) = l;
            i += 1;
            i;
        }
        (*graph).nvtxs = cnvtxs;
        (*graph).nedges = l;
        (*graph).ncon = 1 as libc::c_int;
        libmetis__SetupGraph_tvwgt(graph);
        libmetis__SetupGraph_label(graph);
    }
    gk_free(
        &mut keys as *mut *mut ikv_t as *mut *mut libc::c_void,
        &mut map as *mut *mut idx_t,
        &mut mark as *mut *mut idx_t,
        0 as *mut *mut libc::c_void,
    );
    return graph;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__PruneGraph(
    mut ctrl: *mut ctrl_t,
    mut nvtxs: idx_t,
    mut xadj: *mut idx_t,
    mut adjncy: *mut idx_t,
    mut vwgt: *mut idx_t,
    mut iperm: *mut idx_t,
    mut factor: real_t,
) -> *mut graph_t {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut k: idx_t = 0;
    let mut l: idx_t = 0;
    let mut nlarge: idx_t = 0;
    let mut pnvtxs: idx_t = 0;
    let mut pnedges: idx_t = 0;
    let mut pxadj: *mut idx_t = 0 as *mut idx_t;
    let mut padjncy: *mut idx_t = 0 as *mut idx_t;
    let mut padjwgt: *mut idx_t = 0 as *mut idx_t;
    let mut pvwgt: *mut idx_t = 0 as *mut idx_t;
    let mut perm: *mut idx_t = 0 as *mut idx_t;
    let mut graph: *mut graph_t = 0 as *mut graph_t;
    perm = libmetis__imalloc(
        nvtxs as size_t,
        b"PruneGraph: perm\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    factor = factor * *xadj.offset(nvtxs as isize) as libc::c_float
        / nvtxs as libc::c_float;
    nlarge = 0 as libc::c_int;
    pnedges = nlarge;
    pnvtxs = pnedges;
    i = 0 as libc::c_int;
    while i < nvtxs {
        if ((*xadj.offset((i + 1 as libc::c_int) as isize) - *xadj.offset(i as isize))
            as libc::c_float) < factor
        {
            *perm.offset(i as isize) = pnvtxs;
            let fresh6 = pnvtxs;
            pnvtxs = pnvtxs + 1;
            *iperm.offset(fresh6 as isize) = i;
            pnedges
                += *xadj.offset((i + 1 as libc::c_int) as isize)
                    - *xadj.offset(i as isize);
        } else {
            nlarge += 1;
            *perm.offset(i as isize) = nvtxs - nlarge;
            *iperm.offset((nvtxs - nlarge) as isize) = i;
        }
        i += 1;
        i;
    }
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_INFO as libc::c_int as libc::c_uint
        != 0
    {
        printf(
            b"  Pruned %d of %d vertices.\n\0" as *const u8 as *const libc::c_char,
            nlarge,
            nvtxs,
        );
    }
    if nlarge > 0 as libc::c_int && nlarge < nvtxs {
        graph = libmetis__CreateGraph();
        (*graph)
            .xadj = libmetis__imalloc(
            (pnvtxs + 1 as libc::c_int) as size_t,
            b"PruneGraph: xadj\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        pxadj = (*graph).xadj;
        (*graph)
            .vwgt = libmetis__imalloc(
            pnvtxs as size_t,
            b"PruneGraph: vwgt\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        pvwgt = (*graph).vwgt;
        (*graph)
            .adjncy = libmetis__imalloc(
            pnedges as size_t,
            b"PruneGraph: adjncy\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        padjncy = (*graph).adjncy;
        (*graph)
            .adjwgt = libmetis__ismalloc(
            pnedges as size_t,
            1 as libc::c_int,
            b"PruneGraph: adjwgt\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        l = 0 as libc::c_int;
        pnedges = l;
        *pxadj.offset(0 as libc::c_int as isize) = pnedges;
        i = 0 as libc::c_int;
        while i < nvtxs {
            if ((*xadj.offset((i + 1 as libc::c_int) as isize)
                - *xadj.offset(i as isize)) as libc::c_float) < factor
            {
                *pvwgt
                    .offset(
                        l as isize,
                    ) = if vwgt.is_null() {
                    1 as libc::c_int
                } else {
                    *vwgt.offset(i as isize)
                };
                j = *xadj.offset(i as isize);
                while j < *xadj.offset((i + 1 as libc::c_int) as isize) {
                    k = *perm.offset(*adjncy.offset(j as isize) as isize);
                    if k < pnvtxs {
                        let fresh7 = pnedges;
                        pnedges = pnedges + 1;
                        *padjncy.offset(fresh7 as isize) = k;
                    }
                    j += 1;
                    j;
                }
                l += 1;
                *pxadj.offset(l as isize) = pnedges;
            }
            i += 1;
            i;
        }
        (*graph).nvtxs = pnvtxs;
        (*graph).nedges = pnedges;
        (*graph).ncon = 1 as libc::c_int;
        libmetis__SetupGraph_tvwgt(graph);
        libmetis__SetupGraph_label(graph);
    } else if nlarge > 0 as libc::c_int && nlarge == nvtxs {
        if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_INFO as libc::c_int as libc::c_uint
            != 0
        {
            printf(
                b"  Pruning is ignored as it removes all vertices.\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        nlarge = 0 as libc::c_int;
    }
    gk_free(
        &mut perm as *mut *mut idx_t as *mut *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
    return graph;
}
