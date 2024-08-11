use ::libc;

use super::sfm::graph_t;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn libmetis__iargmax_strd(_: size_t, _: *mut idx_t, _: idx_t) -> idx_t;
    fn gk_free(ptr1: *mut *mut libc::c_void, _: ...);
    fn libmetis__iargmax(n: size_t, x: *mut idx_t) -> size_t;
    fn libmetis__iargmin(n: size_t, x: *mut idx_t) -> size_t;
    fn libmetis__isum(n: size_t, x: *mut idx_t, incx: size_t) -> idx_t;
    fn libmetis__ismalloc(n: size_t, ival: idx_t, msg: *mut libc::c_char) -> *mut idx_t;
    fn libmetis__iset(n: size_t, val: idx_t, x: *mut idx_t) -> *mut idx_t;
    fn libmetis__ComputeCut(graph: *mut graph_t, where_0: *mut idx_t) -> idx_t;
    fn libmetis__ComputeVolume(_: *mut graph_t, _: *mut idx_t) -> idx_t;
}
pub type __int32_t = libc::c_int;
pub type int32_t = __int32_t;
pub type size_t = u64;
pub type idx_t = int32_t;
pub type real_t = libc::c_float;
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

#[no_mangle]
pub unsafe extern "C" fn libmetis__ComputePartitionInfoBipartite(
    mut graph: *mut graph_t,
    mut nparts: idx_t,
    mut where_0: *mut idx_t,
) {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut k: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut ncon: idx_t = 0;
    let mut mustfree: idx_t = 0 as libc::c_int;

    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut vwgt: *mut idx_t = 0 as *mut idx_t;
    let mut vsize: *mut idx_t = 0 as *mut idx_t;
    let mut adjwgt: *mut idx_t = 0 as *mut idx_t;
    let mut kpwgts: *mut idx_t = 0 as *mut idx_t;
    let mut tmpptr: *mut idx_t = 0 as *mut idx_t;
    let mut padjncy: *mut idx_t = 0 as *mut idx_t;
    let mut padjwgt: *mut idx_t = 0 as *mut idx_t;
    let mut padjcut: *mut idx_t = 0 as *mut idx_t;
    nvtxs = (*graph).nvtxs;
    ncon = (*graph).ncon;
    let mut xadj = &mut (*graph).xadj;
    adjncy = (*graph).adjncy;
    vwgt = (*graph).vwgt;
    vsize = (*graph).vsize;
    adjwgt = (*graph).adjwgt;
    if vwgt.is_null() {
        (*graph).vwgt = libmetis__ismalloc(
            nvtxs as size_t,
            1,
            b"vwgt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        vwgt = (*graph).vwgt;
        mustfree = 1;
    }
    if adjwgt.is_null() {
        (*graph).adjwgt = libmetis__ismalloc(
            xadj[nvtxs as usize] as size_t,
            1,
            b"adjwgt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        adjwgt = (*graph).adjwgt;
        mustfree += 2 as libc::c_int;
    }
    printf(
        b"%d-way Cut: %5d, Vol: %5d, \0" as *const u8 as *const libc::c_char,
        nparts,
        libmetis__ComputeCut(graph, where_0),
        libmetis__ComputeVolume(graph, where_0),
    );
    kpwgts = libmetis__ismalloc(
        (ncon * nparts) as size_t,
        0 as libc::c_int,
        b"ComputePartitionInfo: kpwgts\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < nvtxs {
        j = 0 as libc::c_int;
        while j < ncon {
            let ref mut fresh0 = *kpwgts.offset((*where_0.offset(i as isize) * ncon + j) as isize);
            *fresh0 += *vwgt.offset((i * ncon + j) as isize);
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    if ncon == 1 {
        printf(
            b"\tBalance: %5.3f out of %5.3f\n\0" as *const u8 as *const libc::c_char,
            1.0f64
                * nparts as libc::c_double
                * *kpwgts.offset(libmetis__iargmax(nparts as size_t, kpwgts) as isize)
                    as libc::c_double
                / (1.0f64
                    * libmetis__isum(nparts as size_t, kpwgts, 1 as size_t) as libc::c_double),
            1.0f64
                * nparts as libc::c_double
                * *vwgt.offset(libmetis__iargmax(nvtxs as size_t, vwgt) as isize) as libc::c_double
                / (1.0f64
                    * libmetis__isum(nparts as size_t, kpwgts, 1 as size_t) as libc::c_double),
        );
    } else {
        printf(b"\tBalance:\0" as *const u8 as *const libc::c_char);
        j = 0 as libc::c_int;
        while j < ncon {
            printf(
                b" (%5.3f out of %5.3f)\0" as *const u8 as *const libc::c_char,
                1.0f64
                    * nparts as libc::c_double
                    * *kpwgts.offset(
                        (ncon
                            * libmetis__iargmax_strd(
                                nparts as size_t,
                                kpwgts.offset(j as isize),
                                ncon,
                            )
                            + j) as isize,
                    ) as libc::c_double
                    / (1.0f64
                        * libmetis__isum(
                            nparts as size_t,
                            kpwgts.offset(j as isize),
                            ncon as size_t,
                        ) as libc::c_double),
                1.0f64
                    * nparts as libc::c_double
                    * *vwgt.offset(
                        (ncon
                            * libmetis__iargmax_strd(
                                nvtxs as size_t,
                                vwgt.offset(j as isize),
                                ncon,
                            )
                            + j) as isize,
                    ) as libc::c_double
                    / (1.0f64
                        * libmetis__isum(
                            nparts as size_t,
                            kpwgts.offset(j as isize),
                            ncon as size_t,
                        ) as libc::c_double),
            );
            j += 1;
            j;
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
    }
    padjncy = libmetis__ismalloc(
        (nparts * nparts) as size_t,
        0 as libc::c_int,
        b"ComputePartitionInfo: padjncy\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    padjwgt = libmetis__ismalloc(
        (nparts * nparts) as size_t,
        0 as libc::c_int,
        b"ComputePartitionInfo: padjwgt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    padjcut = libmetis__ismalloc(
        (nparts * nparts) as size_t,
        0 as libc::c_int,
        b"ComputePartitionInfo: padjwgt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    libmetis__iset(nparts as size_t, 0 as libc::c_int, kpwgts);
    i = 0 as libc::c_int;
    while i < nvtxs {
        j = xadj[i as usize];
        while j < xadj[(i + 1) as usize] {
            if *where_0.offset(i as isize) != *where_0.offset(*adjncy.offset(j as isize) as isize) {
                *padjncy.offset(
                    (*where_0.offset(i as isize) * nparts
                        + *where_0.offset(*adjncy.offset(j as isize) as isize))
                        as isize,
                ) = 1;
                let ref mut fresh1 = *padjcut.offset(
                    (*where_0.offset(i as isize) * nparts
                        + *where_0.offset(*adjncy.offset(j as isize) as isize))
                        as isize,
                );
                *fresh1 += *adjwgt.offset(j as isize);
                if *kpwgts.offset(*where_0.offset(*adjncy.offset(j as isize) as isize) as isize)
                    == 0 as libc::c_int
                {
                    let ref mut fresh2 = *padjwgt.offset(
                        (*where_0.offset(i as isize) * nparts
                            + *where_0.offset(*adjncy.offset(j as isize) as isize))
                            as isize,
                    );
                    *fresh2 += *vsize.offset(i as isize);
                    *kpwgts.offset(*where_0.offset(*adjncy.offset(j as isize) as isize) as isize) =
                        1;
                }
            }
            j += 1;
            j;
        }
        j = xadj[i as usize];
        while j < xadj[(i + 1) as usize] {
            *kpwgts.offset(*where_0.offset(*adjncy.offset(j as isize) as isize) as isize) =
                0 as libc::c_int;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < nparts {
        *kpwgts.offset(i as isize) = libmetis__isum(
            nparts as size_t,
            padjncy.offset((i * nparts) as isize),
            1 as size_t,
        );
        i += 1;
        i;
    }
    printf(
        b"Min/Max/Avg/Bal # of adjacent     subdomains: %5d %5d %5d %7.3f\n\0" as *const u8
            as *const libc::c_char,
        *kpwgts.offset(libmetis__iargmin(nparts as size_t, kpwgts) as isize),
        *kpwgts.offset(libmetis__iargmax(nparts as size_t, kpwgts) as isize),
        libmetis__isum(nparts as size_t, kpwgts, 1 as size_t) / nparts,
        1.0f64
            * nparts as libc::c_double
            * *kpwgts.offset(libmetis__iargmax(nparts as size_t, kpwgts) as isize)
                as libc::c_double
            / (1.0f64 * libmetis__isum(nparts as size_t, kpwgts, 1 as size_t) as libc::c_double),
    );
    i = 0 as libc::c_int;
    while i < nparts {
        *kpwgts.offset(i as isize) = libmetis__isum(
            nparts as size_t,
            padjcut.offset((i * nparts) as isize),
            1 as size_t,
        );
        i += 1;
        i;
    }
    printf(
        b"Min/Max/Avg/Bal # of adjacent subdomain cuts: %5d %5d %5d %7.3f\n\0" as *const u8
            as *const libc::c_char,
        *kpwgts.offset(libmetis__iargmin(nparts as size_t, kpwgts) as isize),
        *kpwgts.offset(libmetis__iargmax(nparts as size_t, kpwgts) as isize),
        libmetis__isum(nparts as size_t, kpwgts, 1 as size_t) / nparts,
        1.0f64
            * nparts as libc::c_double
            * *kpwgts.offset(libmetis__iargmax(nparts as size_t, kpwgts) as isize)
                as libc::c_double
            / (1.0f64 * libmetis__isum(nparts as size_t, kpwgts, 1 as size_t) as libc::c_double),
    );
    i = 0 as libc::c_int;
    while i < nparts {
        *kpwgts.offset(i as isize) = libmetis__isum(
            nparts as size_t,
            padjwgt.offset((i * nparts) as isize),
            1 as size_t,
        );
        i += 1;
        i;
    }
    printf(
        b"Min/Max/Avg/Bal/Frac # of interface    nodes: %5d %5d %5d %7.3f %7.3f\n\0" as *const u8
            as *const libc::c_char,
        *kpwgts.offset(libmetis__iargmin(nparts as size_t, kpwgts) as isize),
        *kpwgts.offset(libmetis__iargmax(nparts as size_t, kpwgts) as isize),
        libmetis__isum(nparts as size_t, kpwgts, 1 as size_t) / nparts,
        1.0f64
            * nparts as libc::c_double
            * *kpwgts.offset(libmetis__iargmax(nparts as size_t, kpwgts) as isize)
                as libc::c_double
            / (1.0f64 * libmetis__isum(nparts as size_t, kpwgts, 1 as size_t) as libc::c_double),
        1.0f64 * libmetis__isum(nparts as size_t, kpwgts, 1 as size_t) as libc::c_double
            / (1.0f64 * nvtxs as libc::c_double),
    );
    if mustfree == 1 || mustfree == 3 as libc::c_int {
        gk_free(
            &mut vwgt as *mut *mut idx_t as *mut *mut libc::c_void,
            0 as *mut *mut libc::c_void,
        );
        (*graph).vwgt = 0 as *mut idx_t;
    }
    if mustfree == 2 as libc::c_int || mustfree == 3 as libc::c_int {
        gk_free(
            &mut adjwgt as *mut *mut idx_t as *mut *mut libc::c_void,
            0 as *mut *mut libc::c_void,
        );
        (*graph).adjwgt = 0 as *mut idx_t;
    }
    gk_free(
        &mut kpwgts as *mut *mut idx_t as *mut *mut libc::c_void,
        &mut padjncy as *mut *mut idx_t,
        &mut padjwgt as *mut *mut idx_t,
        &mut padjcut as *mut *mut idx_t,
        0 as *mut *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__ComputePartitionBalance(
    mut graph: *mut graph_t,
    mut nparts: idx_t,
    mut where_0: *mut idx_t,
    mut ubvec: *mut real_t,
) {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut ncon: idx_t = 0;
    let mut kpwgts: *mut idx_t = 0 as *mut idx_t;
    let mut vwgt: *mut idx_t = 0 as *mut idx_t;
    let mut balance: real_t = 0.;
    nvtxs = (*graph).nvtxs;
    ncon = (*graph).ncon;
    vwgt = (*graph).vwgt;
    kpwgts = libmetis__ismalloc(
        nparts as size_t,
        0 as libc::c_int,
        b"ComputePartitionInfo: kpwgts\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if vwgt.is_null() {
        i = 0 as libc::c_int;
        while i < nvtxs {
            let ref mut fresh3 = *kpwgts.offset(*where_0.offset(i as isize) as isize);
            *fresh3 += 1;
            *fresh3;
            i += 1;
            i;
        }
        *ubvec.offset(0 as libc::c_int as isize) = (1.0f64
            * nparts as libc::c_double
            * *kpwgts.offset(libmetis__iargmax(nparts as size_t, kpwgts) as isize)
                as libc::c_double
            / (1.0f64 * nvtxs as libc::c_double))
            as real_t;
    } else {
        j = 0 as libc::c_int;
        while j < ncon {
            libmetis__iset(nparts as size_t, 0 as libc::c_int, kpwgts);
            i = 0 as libc::c_int;
            while i < (*graph).nvtxs {
                let ref mut fresh4 = *kpwgts.offset(*where_0.offset(i as isize) as isize);
                *fresh4 += *vwgt.offset((i * ncon + j) as isize);
                i += 1;
                i;
            }
            *ubvec.offset(j as isize) = (1.0f64
                * nparts as libc::c_double
                * *kpwgts.offset(libmetis__iargmax(nparts as size_t, kpwgts) as isize)
                    as libc::c_double
                / (1.0f64
                    * libmetis__isum(nparts as size_t, kpwgts, 1 as size_t) as libc::c_double))
                as real_t;
            j += 1;
            j;
        }
    }
    gk_free(
        &mut kpwgts as *mut *mut idx_t as *mut *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__ComputeElementBalance(
    mut ne: idx_t,
    mut nparts: idx_t,
    mut where_0: *mut idx_t,
) -> real_t {
    let mut i: idx_t = 0;
    let mut kpwgts: *mut idx_t = 0 as *mut idx_t;
    let mut balance: real_t = 0.;
    kpwgts = libmetis__ismalloc(
        nparts as size_t,
        0 as libc::c_int,
        b"ComputeElementBalance: kpwgts\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < ne {
        let ref mut fresh5 = *kpwgts.offset(*where_0.offset(i as isize) as isize);
        *fresh5 += 1;
        *fresh5;
        i += 1;
        i;
    }
    balance = (1.0f64
        * nparts as libc::c_double
        * *kpwgts.offset(libmetis__iargmax(nparts as size_t, kpwgts) as isize) as libc::c_double
        / (1.0f64 * libmetis__isum(nparts as size_t, kpwgts, 1 as size_t) as libc::c_double))
        as real_t;
    gk_free(
        &mut kpwgts as *mut *mut idx_t as *mut *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
    return balance;
}
