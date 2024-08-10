use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn libmetis__iargmax_strd(_: size_t, _: *mut idx_t, _: idx_t) -> idx_t;
    fn libmetis__ComputeVolume(_: *mut graph_t, _: *mut idx_t) -> idx_t;
    fn libmetis__ComputeCut(graph: *mut graph_t, where_0: *mut idx_t) -> idx_t;
    fn libmetis__IsConnected(graph: *mut graph_t, report: idx_t) -> idx_t;
    fn libmetis__FindPartitionInducedComponents(
        graph: *mut graph_t,
        where_0: *mut idx_t,
        cptr: *mut idx_t,
        cind: *mut idx_t,
    ) -> idx_t;
    fn gk_free(ptr1: *mut *mut libc::c_void, _: ...);
    fn libmetis__imax(n: size_t, x: *mut idx_t) -> idx_t;
    fn libmetis__iargmax(n: size_t, x: *mut idx_t) -> size_t;
    fn libmetis__isum(n: size_t, x: *mut idx_t, incx: size_t) -> idx_t;
    fn libmetis__imalloc(n: size_t, msg: *mut libc::c_char) -> *mut idx_t;
    fn libmetis__ismalloc(n: size_t, ival: idx_t, msg: *mut libc::c_char) -> *mut idx_t;
    fn libmetis__iset(n: size_t, val: idx_t, x: *mut idx_t) -> *mut idx_t;
    fn libmetis__iarray2csr(
        n: idx_t,
        range: idx_t,
        array: *mut idx_t,
        ptr: *mut idx_t,
        ind: *mut idx_t,
    );
}
pub type __int32_t = libc::c_int;
pub type int32_t = __int32_t;
pub type size_t = libc::c_ulong;
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
pub struct params_t {
    pub ptype: idx_t,
    pub objtype: idx_t,
    pub ctype: idx_t,
    pub iptype: idx_t,
    pub rtype: idx_t,
    pub no2hop: idx_t,
    pub minconn: idx_t,
    pub contig: idx_t,
    pub nooutput: idx_t,
    pub balance: idx_t,
    pub ncuts: idx_t,
    pub niter: idx_t,
    pub gtype: idx_t,
    pub ncommon: idx_t,
    pub seed: idx_t,
    pub dbglvl: idx_t,
    pub nparts: idx_t,
    pub nseps: idx_t,
    pub ufactor: idx_t,
    pub pfactor: idx_t,
    pub compress: idx_t,
    pub ccorder: idx_t,
    pub filename: *mut libc::c_char,
    pub outfile: *mut libc::c_char,
    pub xyzfile: *mut libc::c_char,
    pub tpwgtsfile: *mut libc::c_char,
    pub ubvecstr: *mut libc::c_char,
    pub wgtflag: idx_t,
    pub numflag: idx_t,
    pub tpwgts: *mut real_t,
    pub ubvec: *mut real_t,
    pub iotimer: real_t,
    pub parttimer: real_t,
    pub reporttimer: real_t,
    pub maxmemory: size_t,
}
#[no_mangle]
pub unsafe extern "C" fn ComputePartitionInfo(
    mut params: *mut params_t,
    mut graph: *mut graph_t,
    mut where_0: *mut idx_t,
) {
    let mut i: idx_t = 0;
    let mut ii: idx_t = 0;
    let mut j: idx_t = 0;
    let mut k: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut ncon: idx_t = 0;
    let mut nparts: idx_t = 0;
    let mut tvwgt: idx_t = 0;
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut vwgt: *mut idx_t = 0 as *mut idx_t;
    let mut adjwgt: *mut idx_t = 0 as *mut idx_t;
    let mut kpwgts: *mut idx_t = 0 as *mut idx_t;
    let mut tpwgts: *mut real_t = 0 as *mut real_t;
    let mut unbalance: real_t = 0.;
    let mut pid: idx_t = 0;
    let mut ndom: idx_t = 0;
    let mut maxndom: idx_t = 0;
    let mut minndom: idx_t = 0;
    let mut tndom: idx_t = 0;
    let mut pptr: *mut idx_t = 0 as *mut idx_t;
    let mut pind: *mut idx_t = 0 as *mut idx_t;
    let mut pdom: *mut idx_t = 0 as *mut idx_t;
    let mut ncmps: idx_t = 0;
    let mut nover: idx_t = 0;
    let mut cptr: *mut idx_t = 0 as *mut idx_t;
    let mut cind: *mut idx_t = 0 as *mut idx_t;
    let mut cpwgts: *mut idx_t = 0 as *mut idx_t;
    nvtxs = (*graph).nvtxs;
    ncon = (*graph).ncon;
    xadj = (*graph).xadj;
    adjncy = (*graph).adjncy;
    vwgt = (*graph).vwgt;
    adjwgt = (*graph).adjwgt;
    nparts = (*params).nparts;
    tpwgts = (*params).tpwgts;
    printf(
        b" - Edgecut: %d, communication volume: %d.\n\n\0" as *const u8 as *const libc::c_char,
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
    printf(b" - Balance:\n\0" as *const u8 as *const libc::c_char);
    j = 0 as libc::c_int;
    while j < ncon {
        tvwgt = libmetis__isum(nparts as size_t, kpwgts.offset(j as isize), ncon as size_t);
        k = 0 as libc::c_int;
        unbalance = (1.0f64 * *kpwgts.offset((k * ncon + j) as isize) as libc::c_double
            / (*tpwgts.offset((k * ncon + j) as isize) * tvwgt as libc::c_float) as libc::c_double)
            as real_t;
        i = 1 as libc::c_int;
        while i < nparts {
            if (unbalance as libc::c_double)
                < 1.0f64 * *kpwgts.offset((i * ncon + j) as isize) as libc::c_double
                    / (*tpwgts.offset((i * ncon + j) as isize) * tvwgt as libc::c_float)
                        as libc::c_double
            {
                unbalance = (1.0f64 * *kpwgts.offset((i * ncon + j) as isize) as libc::c_double
                    / (*tpwgts.offset((i * ncon + j) as isize) * tvwgt as libc::c_float)
                        as libc::c_double) as real_t;
                k = i;
            }
            i += 1;
            i;
        }
        printf(
            b"     constraint #%d:  %5.3f out of %5.3f\n\0" as *const u8 as *const libc::c_char,
            j,
            unbalance as libc::c_double,
            1.0f64
                * nparts as libc::c_double
                * *vwgt.offset(
                    (ncon * libmetis__iargmax_strd(nvtxs as size_t, vwgt.offset(j as isize), ncon)
                        + j) as isize,
                ) as libc::c_double
                / (1.0f64
                    * libmetis__isum(nparts as size_t, kpwgts.offset(j as isize), ncon as size_t)
                        as libc::c_double),
        );
        j += 1;
        j;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    if ncon == 1 as libc::c_int {
        tvwgt = libmetis__isum(nparts as size_t, kpwgts, 1 as libc::c_int as size_t);
        k = 0 as libc::c_int;
        unbalance = *kpwgts.offset(k as isize) as libc::c_float
            / (*tpwgts.offset(k as isize) * tvwgt as libc::c_float);
        i = 1 as libc::c_int;
        while i < nparts {
            if unbalance
                < *kpwgts.offset(i as isize) as libc::c_float
                    / (*tpwgts.offset(i as isize) * tvwgt as libc::c_float)
            {
                unbalance = *kpwgts.offset(i as isize) as libc::c_float
                    / (*tpwgts.offset(i as isize) * tvwgt as libc::c_float);
                k = i;
            }
            i += 1;
            i;
        }
        printf(
            b" - Most overweight partition:\n     pid: %d, actual: %d, desired: %d, ratio: %.2f.\n\n\0"
                as *const u8 as *const libc::c_char,
            k,
            *kpwgts.offset(k as isize),
            (tvwgt as libc::c_float * *tpwgts.offset(k as isize)) as idx_t,
            unbalance as libc::c_double,
        );
    }
    gk_free(
        &mut kpwgts as *mut *mut idx_t as *mut *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
    pptr = libmetis__imalloc(
        (nparts + 1 as libc::c_int) as size_t,
        b"ComputePartitionInfo: pptr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    pind = libmetis__imalloc(
        nvtxs as size_t,
        b"ComputePartitionInfo: pind\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    pdom = libmetis__imalloc(
        nparts as size_t,
        b"ComputePartitionInfo: pdom\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    libmetis__iarray2csr(nvtxs, nparts, where_0, pptr, pind);
    maxndom = nparts + 1 as libc::c_int;
    minndom = 0 as libc::c_int;
    tndom = 0 as libc::c_int;
    pid = 0 as libc::c_int;
    while pid < nparts {
        libmetis__iset(nparts as size_t, 0 as libc::c_int, pdom);
        ii = *pptr.offset(pid as isize);
        while ii < *pptr.offset((pid + 1 as libc::c_int) as isize) {
            i = *pind.offset(ii as isize);
            j = *xadj.offset(i as isize);
            while j < *xadj.offset((i + 1 as libc::c_int) as isize) {
                let ref mut fresh1 =
                    *pdom.offset(*where_0.offset(*adjncy.offset(j as isize) as isize) as isize);
                *fresh1 += *adjwgt.offset(j as isize);
                j += 1;
                j;
            }
            ii += 1;
            ii;
        }
        *pdom.offset(pid as isize) = 0 as libc::c_int;
        ndom = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < nparts {
            ndom += if *pdom.offset(i as isize) > 0 as libc::c_int {
                1 as libc::c_int
            } else {
                0 as libc::c_int
            };
            i += 1;
            i;
        }
        tndom += ndom;
        if pid == 0 as libc::c_int || maxndom < ndom {
            maxndom = ndom;
        }
        if pid == 0 as libc::c_int || minndom > ndom {
            minndom = ndom;
        }
        pid += 1;
        pid;
    }
    printf(
        b" - Subdomain connectivity: max: %d, min: %d, avg: %.2f\n\n\0" as *const u8
            as *const libc::c_char,
        maxndom,
        minndom,
        1.0f64 * tndom as libc::c_double / nparts as libc::c_double,
    );
    gk_free(
        &mut pptr as *mut *mut idx_t as *mut *mut libc::c_void,
        &mut pind as *mut *mut idx_t,
        &mut pdom as *mut *mut idx_t,
        0 as *mut *mut libc::c_void,
    );
    cptr = libmetis__imalloc(
        (nvtxs + 1 as libc::c_int) as size_t,
        b"ComputePartitionInfo: cptr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    cind = libmetis__imalloc(
        nvtxs as size_t,
        b"ComputePartitionInfo: cind\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    cpwgts = libmetis__ismalloc(
        nparts as size_t,
        0 as libc::c_int,
        b"ComputePartitionInfo: cpwgts\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    ncmps = libmetis__FindPartitionInducedComponents(graph, where_0, cptr, cind);
    if ncmps == nparts {
        printf(b" - Each partition is contiguous.\n\0" as *const u8 as *const libc::c_char);
    } else if libmetis__IsConnected(graph, 0 as libc::c_int) != 0 {
        nover = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < ncmps {
            let ref mut fresh2 = *cpwgts.offset(
                *where_0.offset(*cind.offset(*cptr.offset(i as isize) as isize) as isize) as isize,
            );
            *fresh2 += 1;
            *fresh2;
            if *cpwgts.offset(
                *where_0.offset(*cind.offset(*cptr.offset(i as isize) as isize) as isize) as isize,
            ) == 2 as libc::c_int
            {
                nover += 1;
                nover;
            }
            i += 1;
            i;
        }
        printf(
            b" - There are %d non-contiguous partitions.\n   Total components after removing the cut edges: %d,\n   max components: %d for pid: %d.\n\0"
                as *const u8 as *const libc::c_char,
            nover,
            ncmps,
            libmetis__imax(nparts as size_t, cpwgts),
            libmetis__iargmax(nparts as size_t, cpwgts) as idx_t,
        );
    } else {
        printf(
            b" - The original graph had %d connected components and the resulting\n   partitioning after removing the cut edges has %d components.\0"
                as *const u8 as *const libc::c_char,
            libmetis__FindPartitionInducedComponents(
                graph,
                0 as *mut idx_t,
                0 as *mut idx_t,
                0 as *mut idx_t,
            ),
            ncmps,
        );
    }
    gk_free(
        &mut cptr as *mut *mut idx_t as *mut *mut libc::c_void,
        &mut cind as *mut *mut idx_t,
        &mut cpwgts as *mut *mut idx_t,
        0 as *mut *mut libc::c_void,
    );
}
