use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn gk_malloc(nbytes: size_t, msg: *mut libc::c_char) -> *mut libc::c_void;
    fn gk_realloc(
        oldptr: *mut libc::c_void,
        nbytes: size_t,
        msg: *mut libc::c_char,
    ) -> *mut libc::c_void;
    fn gk_free(ptr1: *mut *mut libc::c_void, _: ...);
    fn gk_errexit(signum: libc::c_int, _: *mut libc::c_char, _: ...);
    fn gk_mcoreCreate(coresize: size_t) -> *mut gk_mcore_t;
    fn gk_mcoreDestroy(r_mcore: *mut *mut gk_mcore_t, showstats: libc::c_int);
    fn gk_mcoreMalloc(mcore: *mut gk_mcore_t, nbytes: size_t) -> *mut libc::c_void;
    fn gk_mcorePush(mcore: *mut gk_mcore_t);
    fn gk_mcorePop(mcore: *mut gk_mcore_t);
    fn libmetis__imalloc(n: size_t, msg: *mut libc::c_char) -> *mut idx_t;
    fn libmetis__ismalloc(n: size_t, ival: idx_t, msg: *mut libc::c_char) -> *mut idx_t;
    fn libmetis__iAllocMatrix(
        ndim1: size_t,
        ndim2: size_t,
        value: idx_t,
        errmsg: *mut libc::c_char,
    ) -> *mut *mut idx_t;
    fn libmetis__iFreeMatrix(
        r_matrix: *mut *mut *mut idx_t,
        ndim1: size_t,
        ndim2: size_t,
    );
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
pub unsafe extern "C" fn libmetis__AllocateWorkSpace(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
) {
    let mut coresize: size_t = 0;
    match (*ctrl).optype as libc::c_uint {
        0 => {
            coresize = ((3 as libc::c_int * ((*graph).nvtxs + 1 as libc::c_int))
                as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<idx_t>() as libc::c_ulong)
                .wrapping_add(
                    ((5 as libc::c_int * ((*ctrl).nparts + 1 as libc::c_int)
                        * (*graph).ncon) as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<idx_t>() as libc::c_ulong),
                )
                .wrapping_add(
                    ((5 as libc::c_int * ((*ctrl).nparts + 1 as libc::c_int)
                        * (*graph).ncon) as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<real_t>() as libc::c_ulong),
                );
        }
        _ => {
            coresize = ((4 as libc::c_int * ((*graph).nvtxs + 1 as libc::c_int))
                as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<idx_t>() as libc::c_ulong)
                .wrapping_add(
                    ((5 as libc::c_int * ((*ctrl).nparts + 1 as libc::c_int)
                        * (*graph).ncon) as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<idx_t>() as libc::c_ulong),
                )
                .wrapping_add(
                    ((5 as libc::c_int * ((*ctrl).nparts + 1 as libc::c_int)
                        * (*graph).ncon) as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<real_t>() as libc::c_ulong),
                );
        }
    }
    (*ctrl).mcore = gk_mcoreCreate(coresize);
    (*ctrl).nbrpoolsize = 0 as libc::c_int as size_t;
    (*ctrl).nbrpoolcpos = 0 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__AllocateRefinementWorkSpace(
    mut ctrl: *mut ctrl_t,
    mut nbrpoolsize: idx_t,
) {
    (*ctrl).nbrpoolsize = nbrpoolsize as size_t;
    (*ctrl).nbrpoolcpos = 0 as libc::c_int as size_t;
    (*ctrl).nbrpoolreallocs = 0 as libc::c_int as size_t;
    match (*ctrl).objtype as libc::c_uint {
        0 => {
            (*ctrl)
                .cnbrpool = gk_malloc(
                ((*ctrl).nbrpoolsize)
                    .wrapping_mul(::core::mem::size_of::<cnbr_t>() as libc::c_ulong),
                b"AllocateRefinementWorkSpace: cnbrpool\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            ) as *mut cnbr_t;
        }
        1 => {
            (*ctrl)
                .vnbrpool = gk_malloc(
                ((*ctrl).nbrpoolsize)
                    .wrapping_mul(::core::mem::size_of::<vnbr_t>() as libc::c_ulong),
                b"AllocateRefinementWorkSpace: vnbrpool\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            ) as *mut vnbr_t;
        }
        _ => {
            gk_errexit(
                15 as libc::c_int,
                b"Unknown objtype of %d\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*ctrl).objtype as libc::c_uint,
            );
        }
    }
    if (*ctrl).minconn != 0 {
        (*ctrl)
            .pvec1 = libmetis__imalloc(
            ((*ctrl).nparts + 1 as libc::c_int) as size_t,
            b"AllocateRefinementWorkSpace: pvec1\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        (*ctrl)
            .pvec2 = libmetis__imalloc(
            ((*ctrl).nparts + 1 as libc::c_int) as size_t,
            b"AllocateRefinementWorkSpace: pvec2\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        (*ctrl)
            .maxnads = libmetis__ismalloc(
            (*ctrl).nparts as size_t,
            200 as libc::c_int,
            b"AllocateRefinementWorkSpace: maxnads\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        (*ctrl)
            .nads = libmetis__imalloc(
            (*ctrl).nparts as size_t,
            b"AllocateRefinementWorkSpace: nads\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        (*ctrl)
            .adids = libmetis__iAllocMatrix(
            (*ctrl).nparts as size_t,
            200 as libc::c_int as size_t,
            0 as libc::c_int,
            b"AllocateRefinementWorkSpace: adids\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        (*ctrl)
            .adwgts = libmetis__iAllocMatrix(
            (*ctrl).nparts as size_t,
            200 as libc::c_int as size_t,
            0 as libc::c_int,
            b"AllocateRefinementWorkSpace: adwgts\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__FreeWorkSpace(mut ctrl: *mut ctrl_t) {
    gk_mcoreDestroy(
        &mut (*ctrl).mcore,
        ((*ctrl).dbglvl as libc::c_uint & METIS_DBG_INFO as libc::c_int as libc::c_uint)
            as libc::c_int,
    );
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_INFO as libc::c_int as libc::c_uint
        != 0
    {
        printf(
            b" nbrpool statistics\n        nbrpoolsize: %12zu   nbrpoolcpos: %12zu\n    nbrpoolreallocs: %12zu\n\n\0"
                as *const u8 as *const libc::c_char,
            (*ctrl).nbrpoolsize,
            (*ctrl).nbrpoolcpos,
            (*ctrl).nbrpoolreallocs,
        );
    }
    gk_free(
        &mut (*ctrl).cnbrpool as *mut *mut cnbr_t as *mut *mut libc::c_void,
        &mut (*ctrl).vnbrpool as *mut *mut vnbr_t,
        0 as *mut *mut libc::c_void,
    );
    (*ctrl).nbrpoolsize = 0 as libc::c_int as size_t;
    (*ctrl).nbrpoolcpos = 0 as libc::c_int as size_t;
    if (*ctrl).minconn != 0 {
        libmetis__iFreeMatrix(
            &mut (*ctrl).adids,
            (*ctrl).nparts as size_t,
            200 as libc::c_int as size_t,
        );
        libmetis__iFreeMatrix(
            &mut (*ctrl).adwgts,
            (*ctrl).nparts as size_t,
            200 as libc::c_int as size_t,
        );
        gk_free(
            &mut (*ctrl).pvec1 as *mut *mut idx_t as *mut *mut libc::c_void,
            &mut (*ctrl).pvec2 as *mut *mut idx_t,
            &mut (*ctrl).maxnads as *mut *mut idx_t,
            &mut (*ctrl).nads as *mut *mut idx_t,
            0 as *mut *mut libc::c_void,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__wspacemalloc(
    mut ctrl: *mut ctrl_t,
    mut nbytes: size_t,
) -> *mut libc::c_void {
    return gk_mcoreMalloc((*ctrl).mcore, nbytes);
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__wspacepush(mut ctrl: *mut ctrl_t) {
    gk_mcorePush((*ctrl).mcore);
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__wspacepop(mut ctrl: *mut ctrl_t) {
    gk_mcorePop((*ctrl).mcore);
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__iwspacemalloc(
    mut ctrl: *mut ctrl_t,
    mut n: idx_t,
) -> *mut idx_t {
    return libmetis__wspacemalloc(
        ctrl,
        (n as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<idx_t>() as libc::c_ulong),
    ) as *mut idx_t;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__rwspacemalloc(
    mut ctrl: *mut ctrl_t,
    mut n: idx_t,
) -> *mut real_t {
    return libmetis__wspacemalloc(
        ctrl,
        (n as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<real_t>() as libc::c_ulong),
    ) as *mut real_t;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__ikvwspacemalloc(
    mut ctrl: *mut ctrl_t,
    mut n: idx_t,
) -> *mut ikv_t {
    return libmetis__wspacemalloc(
        ctrl,
        (n as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<ikv_t>() as libc::c_ulong),
    ) as *mut ikv_t;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__cnbrpoolReset(mut ctrl: *mut ctrl_t) {
    (*ctrl).nbrpoolcpos = 0 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__cnbrpoolGetNext(
    mut ctrl: *mut ctrl_t,
    mut nnbrs: idx_t,
) -> idx_t {
    (*ctrl)
        .nbrpoolcpos = ((*ctrl).nbrpoolcpos as libc::c_ulong)
        .wrapping_add(nnbrs as libc::c_ulong) as size_t as size_t;
    if (*ctrl).nbrpoolcpos > (*ctrl).nbrpoolsize {
        (*ctrl)
            .nbrpoolsize = ((*ctrl).nbrpoolsize as libc::c_ulong)
            .wrapping_add(
                if (10 as libc::c_int * nnbrs) as libc::c_ulong
                    >= ((*ctrl).nbrpoolsize)
                        .wrapping_div(2 as libc::c_int as libc::c_ulong)
                {
                    (10 as libc::c_int * nnbrs) as libc::c_ulong
                } else {
                    ((*ctrl).nbrpoolsize).wrapping_div(2 as libc::c_int as libc::c_ulong)
                },
            ) as size_t as size_t;
        (*ctrl)
            .cnbrpool = gk_realloc(
            (*ctrl).cnbrpool as *mut libc::c_void,
            ((*ctrl).nbrpoolsize)
                .wrapping_mul(::core::mem::size_of::<cnbr_t>() as libc::c_ulong),
            b"cnbrpoolGet: cnbrpool\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        ) as *mut cnbr_t;
        (*ctrl).nbrpoolreallocs = ((*ctrl).nbrpoolreallocs).wrapping_add(1);
        (*ctrl).nbrpoolreallocs;
    }
    return ((*ctrl).nbrpoolcpos).wrapping_sub(nnbrs as libc::c_ulong) as idx_t;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__vnbrpoolReset(mut ctrl: *mut ctrl_t) {
    (*ctrl).nbrpoolcpos = 0 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__vnbrpoolGetNext(
    mut ctrl: *mut ctrl_t,
    mut nnbrs: idx_t,
) -> idx_t {
    (*ctrl)
        .nbrpoolcpos = ((*ctrl).nbrpoolcpos as libc::c_ulong)
        .wrapping_add(nnbrs as libc::c_ulong) as size_t as size_t;
    if (*ctrl).nbrpoolcpos > (*ctrl).nbrpoolsize {
        (*ctrl)
            .nbrpoolsize = ((*ctrl).nbrpoolsize as libc::c_ulong)
            .wrapping_add(
                if (10 as libc::c_int * nnbrs) as libc::c_ulong
                    >= ((*ctrl).nbrpoolsize)
                        .wrapping_div(2 as libc::c_int as libc::c_ulong)
                {
                    (10 as libc::c_int * nnbrs) as libc::c_ulong
                } else {
                    ((*ctrl).nbrpoolsize).wrapping_div(2 as libc::c_int as libc::c_ulong)
                },
            ) as size_t as size_t;
        (*ctrl)
            .vnbrpool = gk_realloc(
            (*ctrl).vnbrpool as *mut libc::c_void,
            ((*ctrl).nbrpoolsize)
                .wrapping_mul(::core::mem::size_of::<vnbr_t>() as libc::c_ulong),
            b"vnbrpoolGet: vnbrpool\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        ) as *mut vnbr_t;
        (*ctrl).nbrpoolreallocs = ((*ctrl).nbrpoolreallocs).wrapping_add(1);
        (*ctrl).nbrpoolreallocs;
    }
    return ((*ctrl).nbrpoolcpos).wrapping_sub(nnbrs as libc::c_ulong) as idx_t;
}
