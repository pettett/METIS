use ::libc;

use super::{
    auxapi::*, coarsen::libmetis__CoarsenGraph, contig::*, fortran::*, gklib::*, graph::*,
    kwayrefine::*, options::*, structure::*, timing::*, util::*, wspace::*,
};

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
use super::structure::*;
#[no_mangle]
pub unsafe extern "C" fn libmetis__BucketSortKeysInc(
    mut ctrl: *mut ctrl_t,
    mut n: idx_t,
    mut max: idx_t,
    mut keys: *mut idx_t,
    mut tperm: *mut idx_t,
    mut perm: *mut idx_t,
) {
    let mut i: idx_t = 0;
    let mut ii: idx_t = 0;
    let mut counts: *mut idx_t = 0 as *mut idx_t;
    libmetis__wspacepush(ctrl);
    counts = libmetis__iset(
        (max + 2 as libc::c_int) as size_t,
        0 as libc::c_int,
        libmetis__iwspacemalloc(ctrl, max + 2 as libc::c_int),
    );
    i = 0 as libc::c_int;
    while i < n {
        let ref mut fresh0 = *counts.offset(*keys.offset(i as isize) as isize);
        *fresh0 += 1;
        *fresh0;
        i += 1;
        i;
    }
    i = 1 as libc::c_int;
    while i < max + 1 as libc::c_int {
        let ref mut fresh1 = *counts.offset(i as isize);
        *fresh1 += *counts.offset((i - 1 as libc::c_int) as isize);
        i += 1;
        i;
    }
    i = max + 1 as libc::c_int;
    while i > 0 as libc::c_int {
        *counts.offset(i as isize) = *counts.offset((i - 1 as libc::c_int) as isize);
        i -= 1;
        i;
    }
    *counts.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    ii = 0 as libc::c_int;
    while ii < n {
        i = *tperm.offset(ii as isize);
        let ref mut fresh2 = *counts.offset(*keys.offset(i as isize) as isize);
        let fresh3 = *fresh2;
        *fresh2 = *fresh2 + 1;
        *perm.offset(fresh3 as isize) = i;
        ii += 1;
        ii;
    }
    libmetis__wspacepop(ctrl);
}
