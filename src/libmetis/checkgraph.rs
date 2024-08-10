use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn libmetis__CreateGraph() -> *mut graph_t;
    fn libmetis__uvwsorti(n: size_t, base: *mut uvw_t);
    fn gk_malloc(nbytes: size_t, msg: *mut libc::c_char) -> *mut libc::c_void;
    fn gk_free(ptr1: *mut *mut libc::c_void, _: ...);
    fn libmetis__imalloc(n: size_t, msg: *mut libc::c_char) -> *mut idx_t;
    fn libmetis__ismalloc(n: size_t, ival: idx_t, msg: *mut libc::c_char) -> *mut idx_t;
    fn libmetis__icopy(n: size_t, a: *mut idx_t, b: *mut idx_t) -> *mut idx_t;
}
pub type __int32_t = libc::c_int;
pub type int32_t = __int32_t;
pub type size_t = libc::c_ulong;
pub type idx_t = int32_t;
pub type real_t = libc::c_float;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uvw_t {
    pub u: idx_t,
    pub v: idx_t,
    pub w: idx_t,
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
#[no_mangle]
pub unsafe extern "C" fn libmetis__CheckGraph(
    mut graph: *mut graph_t,
    mut numflag: libc::c_int,
    mut verbose: libc::c_int,
) -> libc::c_int {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut k: idx_t = 0;
    let mut l: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut err: idx_t = 0 as libc::c_int;
    let mut minedge: idx_t = 0;
    let mut maxedge: idx_t = 0;
    let mut minewgt: idx_t = 0;
    let mut maxewgt: idx_t = 0;
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut adjwgt: *mut idx_t = 0 as *mut idx_t;
    let mut htable: *mut idx_t = 0 as *mut idx_t;
    numflag = if numflag == 0 as libc::c_int {
        0 as libc::c_int
    } else {
        1 as libc::c_int
    };
    nvtxs = (*graph).nvtxs;
    xadj = (*graph).xadj;
    adjncy = (*graph).adjncy;
    adjwgt = (*graph).adjwgt;
    htable = libmetis__ismalloc(
        nvtxs as size_t,
        0 as libc::c_int,
        b"htable\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    maxedge = *adjncy.offset(0 as libc::c_int as isize);
    minedge = maxedge;
    maxewgt = *adjwgt.offset(0 as libc::c_int as isize);
    minewgt = maxewgt;
    i = 0 as libc::c_int;
    while i < nvtxs {
        j = *xadj.offset(i as isize);
        while j < *xadj.offset((i + 1 as libc::c_int) as isize) {
            k = *adjncy.offset(j as isize);
            minedge = if k < minedge { k } else { minedge };
            maxedge = if k > maxedge { k } else { maxedge };
            minewgt = if *adjwgt.offset(j as isize) < minewgt {
                *adjwgt.offset(j as isize)
            } else {
                minewgt
            };
            maxewgt = if *adjwgt.offset(j as isize) > maxewgt {
                *adjwgt.offset(j as isize)
            } else {
                maxewgt
            };
            if i == k {
                if verbose != 0 {
                    printf(
                        b"Vertex %d contains a self-loop (i.e., diagonal entry in the matrix)!\n\0"
                            as *const u8 as *const libc::c_char,
                        i + numflag,
                    );
                }
                err += 1;
                err;
            } else {
                l = *xadj.offset(k as isize);
                while l < *xadj.offset((k + 1 as libc::c_int) as isize) {
                    if *adjncy.offset(l as isize) == i {
                        if *adjwgt.offset(l as isize) != *adjwgt.offset(j as isize) {
                            if verbose != 0 {
                                printf(
                                    b"Edges (u:%d v:%d wgt:%d) and (v:%d u:%d wgt:%d) do not have the same weight!\n\0"
                                        as *const u8 as *const libc::c_char,
                                    i + numflag,
                                    k + numflag,
                                    *adjwgt.offset(j as isize),
                                    k + numflag,
                                    i + numflag,
                                    *adjwgt.offset(l as isize),
                                );
                            }
                            err += 1;
                            err;
                        }
                        break;
                    } else {
                        l += 1;
                        l;
                    }
                }
                if l == *xadj.offset((k + 1 as libc::c_int) as isize) {
                    if verbose != 0 {
                        printf(
                            b"Missing edge: (%d %d)!\n\0" as *const u8
                                as *const libc::c_char,
                            k + numflag,
                            i + numflag,
                        );
                    }
                    err += 1;
                    err;
                }
            }
            if *htable.offset(k as isize) == 0 as libc::c_int {
                let ref mut fresh0 = *htable.offset(k as isize);
                *fresh0 += 1;
                *fresh0;
            } else {
                if verbose != 0 {
                    let ref mut fresh1 = *htable.offset(k as isize);
                    let fresh2 = *fresh1;
                    *fresh1 = *fresh1 + 1;
                    printf(
                        b"Edge %d from vertex %d is repeated %d times\n\0" as *const u8
                            as *const libc::c_char,
                        k + numflag,
                        i + numflag,
                        fresh2,
                    );
                }
                err += 1;
                err;
            }
            j += 1;
            j;
        }
        j = *xadj.offset(i as isize);
        while j < *xadj.offset((i + 1 as libc::c_int) as isize) {
            *htable.offset(*adjncy.offset(j as isize) as isize) = 0 as libc::c_int;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    if err > 0 as libc::c_int && verbose != 0 {
        printf(
            b"A total of %d errors exist in the input file. Correct them, and run again!\n\0"
                as *const u8 as *const libc::c_char,
            err,
        );
    }
    gk_free(
        &mut htable as *mut *mut idx_t as *mut *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
    return if err == 0 as libc::c_int { 1 as libc::c_int } else { 0 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__CheckInputGraphWeights(
    mut nvtxs: idx_t,
    mut ncon: idx_t,
    mut xadj: *mut idx_t,
    mut adjncy: *mut idx_t,
    mut vwgt: *mut idx_t,
    mut vsize: *mut idx_t,
    mut adjwgt: *mut idx_t,
) -> libc::c_int {
    let mut i: idx_t = 0;
    if ncon <= 0 as libc::c_int {
        printf(
            b"Input Error: ncon must be >= 1.\n\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if !vwgt.is_null() {
        i = ncon * nvtxs;
        while i >= 0 as libc::c_int {
            if *vwgt.offset(i as isize) < 0 as libc::c_int {
                printf(
                    b"Input Error: negative vertex weight(s).\n\0" as *const u8
                        as *const libc::c_char,
                );
                return 0 as libc::c_int;
            }
            i -= 1;
            i;
        }
    }
    if !vsize.is_null() {
        i = nvtxs;
        while i >= 0 as libc::c_int {
            if *vsize.offset(i as isize) < 0 as libc::c_int {
                printf(
                    b"Input Error: negative vertex sizes(s).\n\0" as *const u8
                        as *const libc::c_char,
                );
                return 0 as libc::c_int;
            }
            i -= 1;
            i;
        }
    }
    if !adjwgt.is_null() {
        i = *xadj.offset(nvtxs as isize) - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            if *adjwgt.offset(i as isize) < 0 as libc::c_int {
                printf(
                    b"Input Error: non-positive edge weight(s).\n\0" as *const u8
                        as *const libc::c_char,
                );
                return 0 as libc::c_int;
            }
            i -= 1;
            i;
        }
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__FixGraph(mut graph: *mut graph_t) -> *mut graph_t {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut k: idx_t = 0;
    let mut l: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut nedges: idx_t = 0;
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut adjwgt: *mut idx_t = 0 as *mut idx_t;
    let mut nxadj: *mut idx_t = 0 as *mut idx_t;
    let mut nadjncy: *mut idx_t = 0 as *mut idx_t;
    let mut nadjwgt: *mut idx_t = 0 as *mut idx_t;
    let mut ngraph: *mut graph_t = 0 as *mut graph_t;
    let mut edges: *mut uvw_t = 0 as *mut uvw_t;
    nvtxs = (*graph).nvtxs;
    xadj = (*graph).xadj;
    adjncy = (*graph).adjncy;
    adjwgt = (*graph).adjwgt;
    ngraph = libmetis__CreateGraph();
    (*ngraph).nvtxs = nvtxs;
    (*ngraph).ncon = (*graph).ncon;
    (*ngraph)
        .vwgt = libmetis__icopy(
        (nvtxs * (*graph).ncon) as size_t,
        (*graph).vwgt,
        libmetis__imalloc(
            (nvtxs * (*graph).ncon) as size_t,
            b"FixGraph: vwgt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
    );
    (*ngraph)
        .vsize = libmetis__ismalloc(
        nvtxs as size_t,
        1 as libc::c_int,
        b"FixGraph: vsize\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !((*graph).vsize).is_null() {
        libmetis__icopy(nvtxs as size_t, (*graph).vsize, (*ngraph).vsize);
    }
    edges = gk_malloc(
        (::core::mem::size_of::<uvw_t>() as libc::c_ulong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(*xadj.offset(nvtxs as isize) as libc::c_ulong),
        b"FixGraph: edges\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut uvw_t;
    nedges = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < nvtxs {
        j = *xadj.offset(i as isize);
        while j < *xadj.offset((i + 1 as libc::c_int) as isize) {
            if i < *adjncy.offset(j as isize) {
                (*edges.offset(nedges as isize)).u = i;
                (*edges.offset(nedges as isize)).v = *adjncy.offset(j as isize);
                (*edges.offset(nedges as isize)).w = *adjwgt.offset(j as isize);
                nedges += 1;
                nedges;
            } else if i > *adjncy.offset(j as isize) {
                (*edges.offset(nedges as isize)).u = *adjncy.offset(j as isize);
                (*edges.offset(nedges as isize)).v = i;
                (*edges.offset(nedges as isize)).w = *adjwgt.offset(j as isize);
                nedges += 1;
                nedges;
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    libmetis__uvwsorti(nedges as size_t, edges);
    k = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i < nedges {
        if (*edges.offset(k as isize)).v != (*edges.offset(i as isize)).v
            || (*edges.offset(k as isize)).u != (*edges.offset(i as isize)).u
        {
            k += 1;
            *edges.offset(k as isize) = *edges.offset(i as isize);
        }
        i += 1;
        i;
    }
    nedges = k + 1 as libc::c_int;
    (*ngraph)
        .xadj = libmetis__ismalloc(
        (nvtxs + 1 as libc::c_int) as size_t,
        0 as libc::c_int,
        b"FixGraph: nxadj\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    nxadj = (*ngraph).xadj;
    (*ngraph)
        .adjncy = libmetis__imalloc(
        (2 as libc::c_int * nedges) as size_t,
        b"FixGraph: nadjncy\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    nadjncy = (*ngraph).adjncy;
    (*ngraph)
        .adjwgt = libmetis__imalloc(
        (2 as libc::c_int * nedges) as size_t,
        b"FixGraph: nadjwgt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    nadjwgt = (*ngraph).adjwgt;
    k = 0 as libc::c_int;
    while k < nedges {
        let ref mut fresh3 = *nxadj.offset((*edges.offset(k as isize)).u as isize);
        *fresh3 += 1;
        *fresh3;
        let ref mut fresh4 = *nxadj.offset((*edges.offset(k as isize)).v as isize);
        *fresh4 += 1;
        *fresh4;
        k += 1;
        k;
    }
    i = 1 as libc::c_int;
    while i < nvtxs {
        let ref mut fresh5 = *nxadj.offset(i as isize);
        *fresh5 += *nxadj.offset((i - 1 as libc::c_int) as isize);
        i += 1;
        i;
    }
    i = nvtxs;
    while i > 0 as libc::c_int {
        *nxadj.offset(i as isize) = *nxadj.offset((i - 1 as libc::c_int) as isize);
        i -= 1;
        i;
    }
    *nxadj.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    k = 0 as libc::c_int;
    while k < nedges {
        *nadjncy
            .offset(
                *nxadj.offset((*edges.offset(k as isize)).u as isize) as isize,
            ) = (*edges.offset(k as isize)).v;
        *nadjncy
            .offset(
                *nxadj.offset((*edges.offset(k as isize)).v as isize) as isize,
            ) = (*edges.offset(k as isize)).u;
        *nadjwgt
            .offset(
                *nxadj.offset((*edges.offset(k as isize)).u as isize) as isize,
            ) = (*edges.offset(k as isize)).w;
        *nadjwgt
            .offset(
                *nxadj.offset((*edges.offset(k as isize)).v as isize) as isize,
            ) = (*edges.offset(k as isize)).w;
        let ref mut fresh6 = *nxadj.offset((*edges.offset(k as isize)).u as isize);
        *fresh6 += 1;
        *fresh6;
        let ref mut fresh7 = *nxadj.offset((*edges.offset(k as isize)).v as isize);
        *fresh7 += 1;
        *fresh7;
        k += 1;
        k;
    }
    i = nvtxs;
    while i > 0 as libc::c_int {
        *nxadj.offset(i as isize) = *nxadj.offset((i - 1 as libc::c_int) as isize);
        i -= 1;
        i;
    }
    *nxadj.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    gk_free(
        &mut edges as *mut *mut uvw_t as *mut *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
    return ngraph;
}
