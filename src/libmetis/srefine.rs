use ::libc;
extern "C" {
    fn gk_malloc(nbytes: size_t, msg: *mut libc::c_char) -> *mut libc::c_void;
    fn gk_errexit(signum: libc::c_int, _: *mut libc::c_char, _: ...);
    fn gk_CPUSeconds() -> libc::c_double;
    fn libmetis__imalloc(n: size_t, msg: *mut libc::c_char) -> *mut idx_t;
    fn libmetis__iset(n: size_t, val: idx_t, x: *mut idx_t) -> *mut idx_t;
    fn libmetis__FreeGraph(graph: *mut *mut graph_t);
    fn libmetis__FM_2WayNodeRefine2Sided(ctrl: *mut ctrl_t, graph: *mut graph_t, niter: idx_t);
    fn libmetis__FM_2WayNodeRefine1Sided(ctrl: *mut ctrl_t, graph: *mut graph_t, niter: idx_t);
    fn libmetis__FM_2WayNodeBalance(ctrl: *mut ctrl_t, graph: *mut graph_t);
}

pub use super::structure::*;

#[no_mangle]
pub unsafe extern "C" fn libmetis__Refine2WayNode(
    mut ctrl: *mut ctrl_t,
    mut orggraph: *mut graph_t,
    mut graph: *mut graph_t,
) {
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint != 0 {
        (*ctrl).UncoarsenTmr -= gk_CPUSeconds();
    }
    if graph == orggraph {
        libmetis__Compute2WayNodePartitionParams(ctrl, graph);
    } else {
        loop {
            graph = (*graph).finer;
            if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint != 0 {
                (*ctrl).ProjectTmr -= gk_CPUSeconds();
            }
            libmetis__Project2WayNodePartition(ctrl, graph);
            if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint != 0 {
                (*ctrl).ProjectTmr += gk_CPUSeconds();
            }
            if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint != 0 {
                (*ctrl).RefTmr -= gk_CPUSeconds();
            }
            libmetis__FM_2WayNodeBalance(ctrl, graph);
            match (*ctrl).rtype as libc::c_uint {
                2 => {
                    libmetis__FM_2WayNodeRefine2Sided(ctrl, graph, (*ctrl).niter);
                }
                3 => {
                    libmetis__FM_2WayNodeRefine1Sided(ctrl, graph, (*ctrl).niter);
                }
                _ => {
                    gk_errexit(
                        15 as libc::c_int,
                        b"Unknown rtype of %d\n\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        (*ctrl).rtype as libc::c_uint,
                    );
                }
            }
            if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint != 0 {
                (*ctrl).RefTmr += gk_CPUSeconds();
            }
            if !(graph != orggraph) {
                break;
            }
        }
    }
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_TIME as libc::c_int as libc::c_uint != 0 {
        (*ctrl).UncoarsenTmr += gk_CPUSeconds();
    }
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__Allocate2WayNodePartitionMemory(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
) {
    let mut nvtxs: idx_t = 0;
    nvtxs = (*graph).nvtxs;
    (*graph).pwgts = libmetis__imalloc(
        3 as libc::c_int as size_t,
        b"Allocate2WayNodePartitionMemory: pwgts\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*graph).where_0 = libmetis__imalloc(
        nvtxs as size_t,
        b"Allocate2WayNodePartitionMemory: where\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*graph).bndptr = libmetis__imalloc(
        nvtxs as size_t,
        b"Allocate2WayNodePartitionMemory: bndptr\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*graph).bndind = libmetis__imalloc(
        nvtxs as size_t,
        b"Allocate2WayNodePartitionMemory: bndind\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*graph).nrinfo = gk_malloc(
        (nvtxs as u64).wrapping_mul(::core::mem::size_of::<nrinfo_t>() as u64),
        b"Allocate2WayNodePartitionMemory: nrinfo\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    ) as *mut nrinfo_t;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__Compute2WayNodePartitionParams(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
) {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut nbnd: idx_t = 0;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut vwgt: *mut idx_t = 0 as *mut idx_t;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    let mut pwgts: *mut idx_t = 0 as *mut idx_t;
    let mut bndind: *mut idx_t = 0 as *mut idx_t;
    let mut bndptr: *mut idx_t = 0 as *mut idx_t;
    let mut edegrees: *mut idx_t = 0 as *mut idx_t;
    let mut rinfo: *mut nrinfo_t = 0 as *mut nrinfo_t;
    let mut me: idx_t = 0;
    let mut other: idx_t = 0;
    nvtxs = (*graph).nvtxs;
    let mut xadj = &mut (*graph).xadj;
    vwgt = (*graph).vwgt;
    adjncy = (*graph).adjncy;
    where_0 = (*graph).where_0;
    rinfo = (*graph).nrinfo;
    pwgts = libmetis__iset(3 as libc::c_int as size_t, 0 as libc::c_int, (*graph).pwgts);
    bndind = (*graph).bndind;
    bndptr = libmetis__iset(nvtxs as size_t, -(1), (*graph).bndptr);
    nbnd = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < nvtxs {
        me = *where_0.offset(i as isize);
        let ref mut fresh0 = *pwgts.offset(me as isize);
        *fresh0 += *vwgt.offset(i as isize);
        if me == 2 as libc::c_int {
            *bndind.offset(nbnd as isize) = i;
            let fresh1 = nbnd;
            nbnd = nbnd + 1;
            *bndptr.offset(i as isize) = fresh1;
            edegrees = ((*rinfo.offset(i as isize)).edegrees).as_mut_ptr();
            let ref mut fresh2 = *edegrees.offset(1 as isize);
            *fresh2 = 0 as libc::c_int;
            *edegrees.offset(0 as libc::c_int as isize) = *fresh2;
            j = xadj[i as usize];
            while j < xadj[(i + 1) as usize] {
                other = *where_0.offset(*adjncy.offset(j as isize) as isize);
                if other != 2 as libc::c_int {
                    let ref mut fresh3 = *edegrees.offset(other as isize);
                    *fresh3 += *vwgt.offset(*adjncy.offset(j as isize) as isize);
                }
                j += 1;
                j;
            }
        }
        i += 1;
        i;
    }
    (*graph).mincut = *pwgts.offset(2 as libc::c_int as isize);
    (*graph).nbnd = nbnd;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__Project2WayNodePartition(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
) {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut cmap: *mut idx_t = 0 as *mut idx_t;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    let mut cwhere: *mut idx_t = 0 as *mut idx_t;
    let mut cgraph: *mut graph_t = 0 as *mut graph_t;
    cgraph = (*graph).coarser;
    cwhere = (*cgraph).where_0;
    nvtxs = (*graph).nvtxs;
    cmap = (*graph).cmap;
    libmetis__Allocate2WayNodePartitionMemory(ctrl, graph);
    where_0 = (*graph).where_0;
    i = 0 as libc::c_int;
    while i < nvtxs {
        *where_0.offset(i as isize) = *cwhere.offset(*cmap.offset(i as isize) as isize);
        i += 1;
        i;
    }
    libmetis__FreeGraph(&mut (*graph).coarser);
    (*graph).coarser = 0 as *mut graph_t;
    libmetis__Compute2WayNodePartitionParams(ctrl, graph);
}
