use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn gk_free(ptr1: *mut *mut libc::c_void, _: ...);
    fn libmetis__iargmax(n: size_t, x: *mut idx_t) -> size_t;
    fn libmetis__ismalloc(n: size_t, ival: idx_t, msg: *mut libc::c_char) -> *mut idx_t;
    fn libmetis__wspacepop(ctrl: *mut ctrl_t);
    fn libmetis__wspacemalloc(ctrl: *mut ctrl_t, nbytes: size_t) -> *mut libc::c_void;
    fn libmetis__wspacepush(ctrl: *mut ctrl_t);
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
pub unsafe extern "C" fn libmetis__ComputeCut(
    mut graph: *mut graph_t,
    mut where_0: *mut idx_t,
) -> idx_t {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut cut: idx_t = 0;
    if ((*graph).adjwgt).is_null() {
        cut = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < (*graph).nvtxs {
            j = *((*graph).xadj).offset(i as isize);
            while j < *((*graph).xadj).offset((i + 1 as libc::c_int) as isize) {
                if *where_0.offset(i as isize)
                    != *where_0.offset(*((*graph).adjncy).offset(j as isize) as isize)
                {
                    cut += 1;
                    cut;
                }
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    } else {
        cut = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < (*graph).nvtxs {
            j = *((*graph).xadj).offset(i as isize);
            while j < *((*graph).xadj).offset((i + 1 as libc::c_int) as isize) {
                if *where_0.offset(i as isize)
                    != *where_0.offset(*((*graph).adjncy).offset(j as isize) as isize)
                {
                    cut += *((*graph).adjwgt).offset(j as isize);
                }
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    }
    return cut / 2 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__ComputeVolume(
    mut graph: *mut graph_t,
    mut where_0: *mut idx_t,
) -> idx_t {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut k: idx_t = 0;
    let mut me: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut nparts: idx_t = 0;
    let mut totalv: idx_t = 0;
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut vsize: *mut idx_t = 0 as *mut idx_t;
    let mut marker: *mut idx_t = 0 as *mut idx_t;
    nvtxs = (*graph).nvtxs;
    xadj = (*graph).xadj;
    adjncy = (*graph).adjncy;
    vsize = (*graph).vsize;
    nparts = *where_0.offset(libmetis__iargmax(nvtxs as size_t, where_0) as isize)
        + 1 as libc::c_int;
    marker = libmetis__ismalloc(
        nparts as size_t,
        -(1 as libc::c_int),
        b"ComputeVolume: marker\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    totalv = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < nvtxs {
        *marker.offset(*where_0.offset(i as isize) as isize) = i;
        j = *xadj.offset(i as isize);
        while j < *xadj.offset((i + 1 as libc::c_int) as isize) {
            k = *where_0.offset(*adjncy.offset(j as isize) as isize);
            if *marker.offset(k as isize) != i {
                *marker.offset(k as isize) = i;
                totalv
                    += if !vsize.is_null() {
                        *vsize.offset(i as isize)
                    } else {
                        1 as libc::c_int
                    };
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    gk_free(
        &mut marker as *mut *mut idx_t as *mut *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
    return totalv;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__ComputeMaxCut(
    mut graph: *mut graph_t,
    mut nparts: idx_t,
    mut where_0: *mut idx_t,
) -> idx_t {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut maxcut: idx_t = 0;
    let mut cuts: *mut idx_t = 0 as *mut idx_t;
    cuts = libmetis__ismalloc(
        nparts as size_t,
        0 as libc::c_int,
        b"ComputeMaxCut: cuts\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if ((*graph).adjwgt).is_null() {
        i = 0 as libc::c_int;
        while i < (*graph).nvtxs {
            j = *((*graph).xadj).offset(i as isize);
            while j < *((*graph).xadj).offset((i + 1 as libc::c_int) as isize) {
                if *where_0.offset(i as isize)
                    != *where_0.offset(*((*graph).adjncy).offset(j as isize) as isize)
                {
                    let ref mut fresh0 = *cuts
                        .offset(*where_0.offset(i as isize) as isize);
                    *fresh0 += 1;
                    *fresh0;
                }
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    } else {
        i = 0 as libc::c_int;
        while i < (*graph).nvtxs {
            j = *((*graph).xadj).offset(i as isize);
            while j < *((*graph).xadj).offset((i + 1 as libc::c_int) as isize) {
                if *where_0.offset(i as isize)
                    != *where_0.offset(*((*graph).adjncy).offset(j as isize) as isize)
                {
                    let ref mut fresh1 = *cuts
                        .offset(*where_0.offset(i as isize) as isize);
                    *fresh1 += *((*graph).adjwgt).offset(j as isize);
                }
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    }
    maxcut = *cuts.offset(libmetis__iargmax(nparts as size_t, cuts) as isize);
    printf(
        b"%zu => %d\n\0" as *const u8 as *const libc::c_char,
        libmetis__iargmax(nparts as size_t, cuts),
        maxcut,
    );
    gk_free(
        &mut cuts as *mut *mut idx_t as *mut *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
    return maxcut;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__CheckBnd(mut graph: *mut graph_t) -> idx_t {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut nbnd: idx_t = 0;
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    let mut bndptr: *mut idx_t = 0 as *mut idx_t;
    let mut bndind: *mut idx_t = 0 as *mut idx_t;
    nvtxs = (*graph).nvtxs;
    xadj = (*graph).xadj;
    adjncy = (*graph).adjncy;
    where_0 = (*graph).where_0;
    bndptr = (*graph).bndptr;
    bndind = (*graph).bndind;
    nbnd = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < nvtxs {
        if *xadj.offset((i + 1 as libc::c_int) as isize) - *xadj.offset(i as isize)
            == 0 as libc::c_int
        {
            nbnd += 1;
            nbnd;
        }
        j = *xadj.offset(i as isize);
        while j < *xadj.offset((i + 1 as libc::c_int) as isize) {
            if *where_0.offset(i as isize)
                != *where_0.offset(*adjncy.offset(j as isize) as isize)
            {
                nbnd += 1;
                nbnd;
                break;
            } else {
                j += 1;
                j;
            }
        }
        i += 1;
        i;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__CheckBnd2(mut graph: *mut graph_t) -> idx_t {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut nbnd: idx_t = 0;
    let mut id: idx_t = 0;
    let mut ed: idx_t = 0;
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    let mut bndptr: *mut idx_t = 0 as *mut idx_t;
    let mut bndind: *mut idx_t = 0 as *mut idx_t;
    nvtxs = (*graph).nvtxs;
    xadj = (*graph).xadj;
    adjncy = (*graph).adjncy;
    where_0 = (*graph).where_0;
    bndptr = (*graph).bndptr;
    bndind = (*graph).bndind;
    nbnd = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < nvtxs {
        ed = 0 as libc::c_int;
        id = ed;
        j = *xadj.offset(i as isize);
        while j < *xadj.offset((i + 1 as libc::c_int) as isize) {
            if *where_0.offset(i as isize)
                != *where_0.offset(*adjncy.offset(j as isize) as isize)
            {
                ed += *((*graph).adjwgt).offset(j as isize);
            } else {
                id += *((*graph).adjwgt).offset(j as isize);
            }
            j += 1;
            j;
        }
        if ed - id >= 0 as libc::c_int
            && *xadj.offset(i as isize) < *xadj.offset((i + 1 as libc::c_int) as isize)
        {
            nbnd += 1;
            nbnd;
        }
        i += 1;
        i;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__CheckNodeBnd(
    mut graph: *mut graph_t,
    mut onbnd: idx_t,
) -> idx_t {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut nbnd: idx_t = 0;
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    let mut bndptr: *mut idx_t = 0 as *mut idx_t;
    let mut bndind: *mut idx_t = 0 as *mut idx_t;
    nvtxs = (*graph).nvtxs;
    xadj = (*graph).xadj;
    adjncy = (*graph).adjncy;
    where_0 = (*graph).where_0;
    bndptr = (*graph).bndptr;
    bndind = (*graph).bndind;
    nbnd = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < nvtxs {
        if *where_0.offset(i as isize) == 2 as libc::c_int {
            nbnd += 1;
            nbnd;
        }
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < nvtxs {
        *where_0.offset(i as isize) != 2 as libc::c_int;
        i += 1;
        i;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__CheckRInfo(
    mut ctrl: *mut ctrl_t,
    mut rinfo: *mut ckrinfo_t,
) -> idx_t {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut nbrs: *mut cnbr_t = 0 as *mut cnbr_t;
    nbrs = ((*ctrl).cnbrpool).offset((*rinfo).inbr as isize);
    i = 0 as libc::c_int;
    while i < (*rinfo).nnbrs {
        j = i + 1 as libc::c_int;
        while j < (*rinfo).nnbrs {
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__CheckNodePartitionParams(
    mut graph: *mut graph_t,
) -> idx_t {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut k: idx_t = 0;
    let mut l: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut me: idx_t = 0;
    let mut other: idx_t = 0;
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut adjwgt: *mut idx_t = 0 as *mut idx_t;
    let mut vwgt: *mut idx_t = 0 as *mut idx_t;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    let mut edegrees: [idx_t; 2] = [0; 2];
    let mut pwgts: [idx_t; 3] = [0; 3];
    nvtxs = (*graph).nvtxs;
    xadj = (*graph).xadj;
    vwgt = (*graph).vwgt;
    adjncy = (*graph).adjncy;
    adjwgt = (*graph).adjwgt;
    where_0 = (*graph).where_0;
    pwgts[2 as libc::c_int as usize] = 0 as libc::c_int;
    pwgts[1 as libc::c_int as usize] = pwgts[2 as libc::c_int as usize];
    pwgts[0 as libc::c_int as usize] = pwgts[1 as libc::c_int as usize];
    i = 0 as libc::c_int;
    while i < nvtxs {
        me = *where_0.offset(i as isize);
        pwgts[me as usize] += *vwgt.offset(i as isize);
        if me == 2 as libc::c_int {
            edegrees[1 as libc::c_int as usize] = 0 as libc::c_int;
            edegrees[0 as libc::c_int as usize] = edegrees[1 as libc::c_int as usize];
            j = *xadj.offset(i as isize);
            while j < *xadj.offset((i + 1 as libc::c_int) as isize) {
                other = *where_0.offset(*adjncy.offset(j as isize) as isize);
                if other != 2 as libc::c_int {
                    edegrees[other as usize]
                        += *vwgt.offset(*adjncy.offset(j as isize) as isize);
                }
                j += 1;
                j;
            }
            if edegrees[0 as libc::c_int as usize]
                != (*((*graph).nrinfo).offset(i as isize))
                    .edegrees[0 as libc::c_int as usize]
                || edegrees[1 as libc::c_int as usize]
                    != (*((*graph).nrinfo).offset(i as isize))
                        .edegrees[1 as libc::c_int as usize]
            {
                printf(
                    b"Something wrong with edegrees: %d %d %d %d %d\n\0" as *const u8
                        as *const libc::c_char,
                    i,
                    edegrees[0 as libc::c_int as usize],
                    edegrees[1 as libc::c_int as usize],
                    (*((*graph).nrinfo).offset(i as isize))
                        .edegrees[0 as libc::c_int as usize],
                    (*((*graph).nrinfo).offset(i as isize))
                        .edegrees[1 as libc::c_int as usize],
                );
                return 0 as libc::c_int;
            }
        }
        i += 1;
        i;
    }
    if pwgts[0 as libc::c_int as usize]
        != *((*graph).pwgts).offset(0 as libc::c_int as isize)
        || pwgts[1 as libc::c_int as usize]
            != *((*graph).pwgts).offset(1 as libc::c_int as isize)
        || pwgts[2 as libc::c_int as usize]
            != *((*graph).pwgts).offset(2 as libc::c_int as isize)
    {
        printf(
            b"Something wrong with part-weights: %d %d %d %d %d %d\n\0" as *const u8
                as *const libc::c_char,
            pwgts[0 as libc::c_int as usize],
            pwgts[1 as libc::c_int as usize],
            pwgts[2 as libc::c_int as usize],
            *((*graph).pwgts).offset(0 as libc::c_int as isize),
            *((*graph).pwgts).offset(1 as libc::c_int as isize),
            *((*graph).pwgts).offset(2 as libc::c_int as isize),
        );
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__IsSeparable(mut graph: *mut graph_t) -> idx_t {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut other: idx_t = 0;
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    nvtxs = (*graph).nvtxs;
    xadj = (*graph).xadj;
    adjncy = (*graph).adjncy;
    where_0 = (*graph).where_0;
    i = 0 as libc::c_int;
    while i < nvtxs {
        if !(*where_0.offset(i as isize) == 2 as libc::c_int) {
            other = (*where_0.offset(i as isize) + 1 as libc::c_int) % 2 as libc::c_int;
            j = *xadj.offset(i as isize);
            while j < *xadj.offset((i + 1 as libc::c_int) as isize) {
                j += 1;
                j;
            }
        }
        i += 1;
        i;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__CheckKWayVolPartitionParams(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
) {
    let mut i: idx_t = 0;
    let mut ii: idx_t = 0;
    let mut j: idx_t = 0;
    let mut k: idx_t = 0;
    let mut kk: idx_t = 0;
    let mut l: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut nbnd: idx_t = 0;
    let mut mincut: idx_t = 0;
    let mut minvol: idx_t = 0;
    let mut me: idx_t = 0;
    let mut other: idx_t = 0;
    let mut pid: idx_t = 0;
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
    let mut vsize: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut pwgts: *mut idx_t = 0 as *mut idx_t;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    let mut bndind: *mut idx_t = 0 as *mut idx_t;
    let mut bndptr: *mut idx_t = 0 as *mut idx_t;
    let mut rinfo: *mut vkrinfo_t = 0 as *mut vkrinfo_t;
    let mut myrinfo: *mut vkrinfo_t = 0 as *mut vkrinfo_t;
    let mut orinfo: *mut vkrinfo_t = 0 as *mut vkrinfo_t;
    let mut tmprinfo: vkrinfo_t = vkrinfo_t {
        nid: 0,
        ned: 0,
        gv: 0,
        nnbrs: 0,
        inbr: 0,
    };
    let mut mynbrs: *mut vnbr_t = 0 as *mut vnbr_t;
    let mut onbrs: *mut vnbr_t = 0 as *mut vnbr_t;
    let mut tmpnbrs: *mut vnbr_t = 0 as *mut vnbr_t;
    libmetis__wspacepush(ctrl);
    nvtxs = (*graph).nvtxs;
    xadj = (*graph).xadj;
    vsize = (*graph).vsize;
    adjncy = (*graph).adjncy;
    where_0 = (*graph).where_0;
    rinfo = (*graph).vkrinfo;
    tmpnbrs = libmetis__wspacemalloc(
        ctrl,
        ((*ctrl).nparts as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<vnbr_t>() as libc::c_ulong),
    ) as *mut vnbr_t;
    i = 0 as libc::c_int;
    while i < nvtxs {
        me = *where_0.offset(i as isize);
        myrinfo = rinfo.offset(i as isize);
        mynbrs = ((*ctrl).vnbrpool).offset((*myrinfo).inbr as isize);
        k = 0 as libc::c_int;
        while k < (*myrinfo).nnbrs {
            *tmpnbrs.offset(k as isize) = *mynbrs.offset(k as isize);
            k += 1;
            k;
        }
        tmprinfo.nnbrs = (*myrinfo).nnbrs;
        tmprinfo.nid = (*myrinfo).nid;
        tmprinfo.ned = (*myrinfo).ned;
        myrinfo = &mut tmprinfo;
        mynbrs = tmpnbrs;
        k = 0 as libc::c_int;
        while k < (*myrinfo).nnbrs {
            (*mynbrs.offset(k as isize)).gv = 0 as libc::c_int;
            k += 1;
            k;
        }
        j = *xadj.offset(i as isize);
        while j < *xadj.offset((i + 1 as libc::c_int) as isize) {
            ii = *adjncy.offset(j as isize);
            other = *where_0.offset(ii as isize);
            orinfo = rinfo.offset(ii as isize);
            onbrs = ((*ctrl).vnbrpool).offset((*orinfo).inbr as isize);
            if me == other {
                k = 0 as libc::c_int;
                while k < (*myrinfo).nnbrs {
                    pid = (*mynbrs.offset(k as isize)).pid;
                    kk = 0 as libc::c_int;
                    while kk < (*orinfo).nnbrs {
                        if (*onbrs.offset(kk as isize)).pid == pid {
                            break;
                        }
                        kk += 1;
                        kk;
                    }
                    if kk == (*orinfo).nnbrs {
                        let ref mut fresh2 = (*mynbrs.offset(k as isize)).gv;
                        *fresh2 -= *vsize.offset(ii as isize);
                    }
                    k += 1;
                    k;
                }
            } else {
                k = 0 as libc::c_int;
                while k < (*orinfo).nnbrs {
                    if (*onbrs.offset(k as isize)).pid == me {
                        break;
                    }
                    k += 1;
                    k;
                }
                if (*onbrs.offset(k as isize)).ned == 1 as libc::c_int {
                    k = 0 as libc::c_int;
                    while k < (*myrinfo).nnbrs {
                        if (*mynbrs.offset(k as isize)).pid == other {
                            let ref mut fresh3 = (*mynbrs.offset(k as isize)).gv;
                            *fresh3 += *vsize.offset(ii as isize);
                            break;
                        } else {
                            k += 1;
                            k;
                        }
                    }
                    k = 0 as libc::c_int;
                    while k < (*myrinfo).nnbrs {
                        pid = (*mynbrs.offset(k as isize)).pid;
                        if !(pid == other) {
                            kk = 0 as libc::c_int;
                            while kk < (*orinfo).nnbrs {
                                if (*onbrs.offset(kk as isize)).pid == pid {
                                    let ref mut fresh4 = (*mynbrs.offset(k as isize)).gv;
                                    *fresh4 += *vsize.offset(ii as isize);
                                    break;
                                } else {
                                    kk += 1;
                                    kk;
                                }
                            }
                        }
                        k += 1;
                        k;
                    }
                } else {
                    k = 0 as libc::c_int;
                    while k < (*myrinfo).nnbrs {
                        pid = (*mynbrs.offset(k as isize)).pid;
                        if !(pid == other) {
                            kk = 0 as libc::c_int;
                            while kk < (*orinfo).nnbrs {
                                if (*onbrs.offset(kk as isize)).pid == pid {
                                    break;
                                }
                                kk += 1;
                                kk;
                            }
                            if kk == (*orinfo).nnbrs {
                                let ref mut fresh5 = (*mynbrs.offset(k as isize)).gv;
                                *fresh5 -= *vsize.offset(ii as isize);
                            }
                        }
                        k += 1;
                        k;
                    }
                }
            }
            j += 1;
            j;
        }
        myrinfo = rinfo.offset(i as isize);
        mynbrs = ((*ctrl).vnbrpool).offset((*myrinfo).inbr as isize);
        k = 0 as libc::c_int;
        while k < (*myrinfo).nnbrs {
            pid = (*mynbrs.offset(k as isize)).pid;
            kk = 0 as libc::c_int;
            while kk < tmprinfo.nnbrs {
                if (*tmpnbrs.offset(kk as isize)).pid == pid {
                    if (*tmpnbrs.offset(kk as isize)).gv
                        != (*mynbrs.offset(k as isize)).gv
                    {
                        printf(
                            b"[%8d %8d %8d %+8d %+8d]\n\0" as *const u8
                                as *const libc::c_char,
                            i,
                            *where_0.offset(i as isize),
                            pid,
                            (*mynbrs.offset(k as isize)).gv,
                            (*tmpnbrs.offset(kk as isize)).gv,
                        );
                    }
                    break;
                } else {
                    kk += 1;
                    kk;
                }
            }
            k += 1;
            k;
        }
        i += 1;
        i;
    }
    libmetis__wspacepop(ctrl);
}
