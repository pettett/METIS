use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
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
pub struct vnbr_t {
    pub pid: idx_t,
    pub ned: idx_t,
    pub gv: idx_t,
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
pub unsafe extern "C" fn libmetis__InitTimers(mut ctrl: *mut ctrl_t) {
    (*ctrl).TotalTmr = 0.0f64;
    (*ctrl).InitPartTmr = 0.0f64;
    (*ctrl).MatchTmr = 0.0f64;
    (*ctrl).ContractTmr = 0.0f64;
    (*ctrl).CoarsenTmr = 0.0f64;
    (*ctrl).UncoarsenTmr = 0.0f64;
    (*ctrl).RefTmr = 0.0f64;
    (*ctrl).ProjectTmr = 0.0f64;
    (*ctrl).SplitTmr = 0.0f64;
    (*ctrl).Aux1Tmr = 0.0f64;
    (*ctrl).Aux2Tmr = 0.0f64;
    (*ctrl).Aux3Tmr = 0.0f64;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__PrintTimers(mut ctrl: *mut ctrl_t) {
    printf(
        b"\nTiming Information -------------------------------------------------\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"\n Multilevel: \t\t %7.3f\0" as *const u8 as *const libc::c_char,
        (*ctrl).TotalTmr,
    );
    printf(
        b"\n     Coarsening: \t\t %7.3f\0" as *const u8 as *const libc::c_char,
        (*ctrl).CoarsenTmr,
    );
    printf(
        b"\n            Matching: \t\t\t %7.3f\0" as *const u8 as *const libc::c_char,
        (*ctrl).MatchTmr,
    );
    printf(
        b"\n            Contract: \t\t\t %7.3f\0" as *const u8 as *const libc::c_char,
        (*ctrl).ContractTmr,
    );
    printf(
        b"\n     Initial Partition: \t %7.3f\0" as *const u8 as *const libc::c_char,
        (*ctrl).InitPartTmr,
    );
    printf(
        b"\n     Uncoarsening: \t\t %7.3f\0" as *const u8 as *const libc::c_char,
        (*ctrl).UncoarsenTmr,
    );
    printf(
        b"\n          Refinement: \t\t\t %7.3f\0" as *const u8 as *const libc::c_char,
        (*ctrl).RefTmr,
    );
    printf(
        b"\n          Projection: \t\t\t %7.3f\0" as *const u8 as *const libc::c_char,
        (*ctrl).ProjectTmr,
    );
    printf(
        b"\n     Splitting: \t\t %7.3f\0" as *const u8 as *const libc::c_char,
        (*ctrl).SplitTmr,
    );
    printf(
        b"\n********************************************************************\n\0"
            as *const u8 as *const libc::c_char,
    );
}
