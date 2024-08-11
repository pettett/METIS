use ::libc;
use libc::printf;

use super::{
    gklib::libmetis__icopy,
    graph::libmetis__FreeRData,
    mincover::libmetis__MinCover,
    sfm::{libmetis__FM_2WayNodeRefine1Sided, libmetis__FM_2WayNodeRefine2Sided},
    srefine::{
        libmetis__Allocate2WayNodePartitionMemory, libmetis__Compute2WayNodePartitionParams,
    },
    wspace::{libmetis__iwspacemalloc, libmetis__wspacepop, libmetis__wspacepush},
};

pub use super::structure::*;

#[no_mangle]
pub unsafe extern "C" fn libmetis__ConstructSeparator(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
) {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut k: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut nbnd: idx_t = 0;
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    let mut bndind: *mut idx_t = 0 as *mut idx_t;
    libmetis__wspacepush(ctrl);
    nvtxs = (*graph).nvtxs;
    xadj = (*graph).xadj;
    nbnd = (*graph).nbnd;
    bndind = (*graph).bndind;
    where_0 = libmetis__icopy(
        nvtxs as size_t,
        (*graph).where_0,
        libmetis__iwspacemalloc(ctrl, nvtxs),
    );
    i = 0 as libc::c_int;
    while i < nbnd {
        j = *bndind.offset(i as isize);
        if *xadj.offset((j + 1 as libc::c_int) as isize) - *xadj.offset(j as isize)
            > 0 as libc::c_int
        {
            *where_0.offset(j as isize) = 2 as libc::c_int;
        }
        i += 1;
        i;
    }
    libmetis__FreeRData(graph);
    libmetis__Allocate2WayNodePartitionMemory(ctrl, graph);
    libmetis__icopy(nvtxs as size_t, where_0, (*graph).where_0);
    libmetis__wspacepop(ctrl);
    libmetis__Compute2WayNodePartitionParams(ctrl, graph);
    libmetis__FM_2WayNodeRefine2Sided(ctrl, graph, 1 as libc::c_int);
    libmetis__FM_2WayNodeRefine1Sided(ctrl, graph, 4 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__ConstructMinCoverSeparator(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
) {
    let mut i: idx_t = 0;
    let mut ii: idx_t = 0;
    let mut j: idx_t = 0;
    let mut jj: idx_t = 0;
    let mut k: idx_t = 0;
    let mut l: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut nbnd: idx_t = 0;
    let mut bnvtxs: [idx_t; 3] = [0; 3];
    let mut bnedges: [idx_t; 2] = [0; 2];
    let mut csize: idx_t = 0;
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut bxadj: *mut idx_t = 0 as *mut idx_t;
    let mut badjncy: *mut idx_t = 0 as *mut idx_t;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    let mut bndind: *mut idx_t = 0 as *mut idx_t;
    let mut bndptr: *mut idx_t = 0 as *mut idx_t;
    let mut vmap: *mut idx_t = 0 as *mut idx_t;
    let mut ivmap: *mut idx_t = 0 as *mut idx_t;
    let mut cover: *mut idx_t = 0 as *mut idx_t;
    libmetis__wspacepush(ctrl);
    nvtxs = (*graph).nvtxs;
    xadj = (*graph).xadj;
    adjncy = (*graph).adjncy;
    nbnd = (*graph).nbnd;
    bndind = (*graph).bndind;
    bndptr = (*graph).bndptr;
    where_0 = (*graph).where_0;
    vmap = libmetis__iwspacemalloc(ctrl, nvtxs);
    ivmap = libmetis__iwspacemalloc(ctrl, nbnd);
    cover = libmetis__iwspacemalloc(ctrl, nbnd);
    if nbnd > 0 as libc::c_int {
        bnedges[1 as libc::c_int as usize] = 0 as libc::c_int;
        bnedges[0 as libc::c_int as usize] = bnedges[1 as libc::c_int as usize];
        bnvtxs[1 as libc::c_int as usize] = bnedges[0 as libc::c_int as usize];
        bnvtxs[0 as libc::c_int as usize] = bnvtxs[1 as libc::c_int as usize];
        i = 0 as libc::c_int;
        while i < nbnd {
            j = *bndind.offset(i as isize);
            k = *where_0.offset(j as isize);
            if *xadj.offset((j + 1 as libc::c_int) as isize) - *xadj.offset(j as isize)
                > 0 as libc::c_int
            {
                bnvtxs[k as usize] += 1;
                bnvtxs[k as usize];
                bnedges[k as usize] +=
                    *xadj.offset((j + 1 as libc::c_int) as isize) - *xadj.offset(j as isize);
            }
            i += 1;
            i;
        }
        bnvtxs[2 as libc::c_int as usize] =
            bnvtxs[0 as libc::c_int as usize] + bnvtxs[1 as libc::c_int as usize];
        bnvtxs[1 as libc::c_int as usize] = bnvtxs[0 as libc::c_int as usize];
        bnvtxs[0 as libc::c_int as usize] = 0 as libc::c_int;
        bxadj = libmetis__iwspacemalloc(ctrl, bnvtxs[2 as libc::c_int as usize] + 1 as libc::c_int);
        badjncy = libmetis__iwspacemalloc(
            ctrl,
            bnedges[0 as libc::c_int as usize]
                + bnedges[1 as libc::c_int as usize]
                + 1 as libc::c_int,
        );
        i = 0 as libc::c_int;
        while i < nbnd {
            j = *bndind.offset(i as isize);
            k = *where_0.offset(j as isize);
            if *xadj.offset((j + 1 as libc::c_int) as isize) - *xadj.offset(j as isize)
                > 0 as libc::c_int
            {
                *vmap.offset(j as isize) = bnvtxs[k as usize];
                let fresh0 = bnvtxs[k as usize];
                bnvtxs[k as usize] = bnvtxs[k as usize] + 1;
                *ivmap.offset(fresh0 as isize) = j;
            }
            i += 1;
            i;
        }
        bnvtxs[1 as libc::c_int as usize] = bnvtxs[0 as libc::c_int as usize];
        bnvtxs[0 as libc::c_int as usize] = 0 as libc::c_int;
        l = 0 as libc::c_int;
        *bxadj.offset(0 as libc::c_int as isize) = l;
        k = 0 as libc::c_int;
        while k < 2 as libc::c_int {
            ii = 0 as libc::c_int;
            while ii < nbnd {
                i = *bndind.offset(ii as isize);
                if *where_0.offset(i as isize) == k
                    && *xadj.offset(i as isize) < *xadj.offset((i + 1 as libc::c_int) as isize)
                {
                    j = *xadj.offset(i as isize);
                    while j < *xadj.offset((i + 1 as libc::c_int) as isize) {
                        jj = *adjncy.offset(j as isize);
                        if *where_0.offset(jj as isize) != k {
                            let fresh1 = l;
                            l = l + 1;
                            *badjncy.offset(fresh1 as isize) = *vmap.offset(jj as isize);
                        }
                        j += 1;
                        j;
                    }
                    bnvtxs[k as usize] += 1;
                    *bxadj.offset(bnvtxs[k as usize] as isize) = l;
                }
                ii += 1;
                ii;
            }
            k += 1;
            k;
        }
        libmetis__MinCover(
            bxadj,
            badjncy,
            bnvtxs[0 as libc::c_int as usize],
            bnvtxs[1 as libc::c_int as usize],
            cover,
            &mut csize,
        );
        if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_SEPINFO as libc::c_int as libc::c_uint != 0 {
            printf(
                b"Nvtxs: %6d, [%5d %5d], Cut: %6d, SS: [%6d %6d], Cover: %6d\n\0" as *const u8
                    as *const libc::c_char,
                nvtxs,
                *((*graph).pwgts).offset(0 as libc::c_int as isize),
                *((*graph).pwgts).offset(1 as libc::c_int as isize),
                (*graph).mincut,
                bnvtxs[0 as libc::c_int as usize],
                bnvtxs[1 as libc::c_int as usize] - bnvtxs[0 as libc::c_int as usize],
                csize,
            );
        }
        i = 0 as libc::c_int;
        while i < csize {
            j = *ivmap.offset(*cover.offset(i as isize) as isize);
            *where_0.offset(j as isize) = 2 as libc::c_int;
            i += 1;
            i;
        }
    } else if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_SEPINFO as libc::c_int as libc::c_uint != 0
    {
        printf(
            b"Nvtxs: %6d, [%5d %5d], Cut: %6d, SS: [%6d %6d], Cover: %6d\n\0" as *const u8
                as *const libc::c_char,
            nvtxs,
            *((*graph).pwgts).offset(0 as libc::c_int as isize),
            *((*graph).pwgts).offset(1 as libc::c_int as isize),
            (*graph).mincut,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
        );
    }
    libmetis__icopy(nvtxs as size_t, (*graph).where_0, vmap);
    libmetis__FreeRData(graph);
    libmetis__Allocate2WayNodePartitionMemory(ctrl, graph);
    libmetis__icopy(nvtxs as size_t, vmap, (*graph).where_0);
    libmetis__wspacepop(ctrl);
    libmetis__Compute2WayNodePartitionParams(ctrl, graph);
    libmetis__FM_2WayNodeRefine1Sided(ctrl, graph, (*ctrl).niter);
}
