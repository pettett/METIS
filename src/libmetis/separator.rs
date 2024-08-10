use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn libmetis__icopy(n: size_t, a: *mut idx_t, b: *mut idx_t) -> *mut idx_t;
    fn libmetis__FreeRData(graph: *mut graph_t);
    fn libmetis__MinCover(
        _: *mut idx_t,
        _: *mut idx_t,
        _: idx_t,
        _: idx_t,
        _: *mut idx_t,
        _: *mut idx_t,
    );
    fn libmetis__Compute2WayNodePartitionParams(ctrl: *mut ctrl_t, graph: *mut graph_t);
    fn libmetis__Allocate2WayNodePartitionMemory(ctrl: *mut ctrl_t, graph: *mut graph_t);
    fn libmetis__FM_2WayNodeRefine2Sided(
        ctrl: *mut ctrl_t,
        graph: *mut graph_t,
        niter: idx_t,
    );
    fn libmetis__FM_2WayNodeRefine1Sided(
        ctrl: *mut ctrl_t,
        graph: *mut graph_t,
        niter: idx_t,
    );
    fn libmetis__wspacepush(ctrl: *mut ctrl_t);
    fn libmetis__wspacepop(ctrl: *mut ctrl_t);
    fn libmetis__iwspacemalloc(_: *mut ctrl_t, _: idx_t) -> *mut idx_t;
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
                bnedges[k as usize]
                    += *xadj.offset((j + 1 as libc::c_int) as isize)
                        - *xadj.offset(j as isize);
            }
            i += 1;
            i;
        }
        bnvtxs[2 as libc::c_int
            as usize] = bnvtxs[0 as libc::c_int as usize]
            + bnvtxs[1 as libc::c_int as usize];
        bnvtxs[1 as libc::c_int as usize] = bnvtxs[0 as libc::c_int as usize];
        bnvtxs[0 as libc::c_int as usize] = 0 as libc::c_int;
        bxadj = libmetis__iwspacemalloc(
            ctrl,
            bnvtxs[2 as libc::c_int as usize] + 1 as libc::c_int,
        );
        badjncy = libmetis__iwspacemalloc(
            ctrl,
            bnedges[0 as libc::c_int as usize] + bnedges[1 as libc::c_int as usize]
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
                    && *xadj.offset(i as isize)
                        < *xadj.offset((i + 1 as libc::c_int) as isize)
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
        if (*ctrl).dbglvl as libc::c_uint
            & METIS_DBG_SEPINFO as libc::c_int as libc::c_uint != 0
        {
            printf(
                b"Nvtxs: %6d, [%5d %5d], Cut: %6d, SS: [%6d %6d], Cover: %6d\n\0"
                    as *const u8 as *const libc::c_char,
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
    } else if (*ctrl).dbglvl as libc::c_uint
        & METIS_DBG_SEPINFO as libc::c_int as libc::c_uint != 0
    {
        printf(
            b"Nvtxs: %6d, [%5d %5d], Cut: %6d, SS: [%6d %6d], Cover: %6d\n\0"
                as *const u8 as *const libc::c_char,
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
