use ::libc;
use libc::{memset, printf};

use crate::GKlib::error::gk_errexit;
use crate::GKlib::memory::{gk_free, gk_malloc};

use super::{
    auxapi::*, contig::*, fortran::*, gklib::*, graph::*, kwayrefine::*, structure::*, util::*,
    wspace::*,
};

pub type C2RustUnnamed = libc::c_uint;
pub const METIS_OPTION_UBVEC: C2RustUnnamed = 24;
pub const METIS_OPTION_GTYPE: C2RustUnnamed = 23;
pub const METIS_OPTION_BALANCE: C2RustUnnamed = 22;
pub const METIS_OPTION_NOOUTPUT: C2RustUnnamed = 21;
pub const METIS_OPTION_NCOMMON: C2RustUnnamed = 20;
pub const METIS_OPTION_TPWGTS: C2RustUnnamed = 19;
pub const METIS_OPTION_HELP: C2RustUnnamed = 18;
pub const METIS_OPTION_NUMBERING: C2RustUnnamed = 17;
pub const METIS_OPTION_UFACTOR: C2RustUnnamed = 16;
pub const METIS_OPTION_NSEPS: C2RustUnnamed = 15;
pub const METIS_OPTION_PFACTOR: C2RustUnnamed = 14;
pub const METIS_OPTION_CCORDER: C2RustUnnamed = 13;
pub const METIS_OPTION_COMPRESS: C2RustUnnamed = 12;
pub const METIS_OPTION_CONTIG: C2RustUnnamed = 11;
pub const METIS_OPTION_MINCONN: C2RustUnnamed = 10;
pub const METIS_OPTION_NO2HOP: C2RustUnnamed = 9;
pub const METIS_OPTION_SEED: C2RustUnnamed = 8;
pub const METIS_OPTION_NCUTS: C2RustUnnamed = 7;
pub const METIS_OPTION_NITER: C2RustUnnamed = 6;
pub const METIS_OPTION_DBGLVL: C2RustUnnamed = 5;
pub const METIS_OPTION_RTYPE: C2RustUnnamed = 4;
pub const METIS_OPTION_IPTYPE: C2RustUnnamed = 3;
pub const METIS_OPTION_CTYPE: C2RustUnnamed = 2;
pub const METIS_OPTION_OBJTYPE: C2RustUnnamed = 1;
pub const METIS_OPTION_PTYPE: C2RustUnnamed = 0;

use super::structure::*;
#[no_mangle]
pub unsafe extern "C" fn libmetis__SetupCtrl(
    mut optype: moptype_et,
    mut options: *mut idx_t,
    mut ncon: idx_t,
    mut nparts: idx_t,
    mut tpwgts: *mut real_t,
    mut ubvec: *mut real_t,
) -> *mut ctrl_t {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut ctrl: *mut ctrl_t = 0 as *mut ctrl_t;
    ctrl = gk_malloc(
        ::core::mem::size_of::<ctrl_t>() as u64,
        b"SetupCtrl: ctrl\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut ctrl_t;
    memset(
        ctrl as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<ctrl_t>() as usize,
    );
    match optype as libc::c_uint {
        0 => {
            (*ctrl).objtype = (if options.is_null()
                || *options.offset(METIS_OPTION_OBJTYPE as libc::c_int as isize)
                    == -(1 as libc::c_int)
            {
                METIS_OBJTYPE_CUT as libc::c_int
            } else {
                *options.offset(METIS_OPTION_OBJTYPE as libc::c_int as isize)
            }) as mobjtype_et;
            (*ctrl).rtype = METIS_RTYPE_FM;
            (*ctrl).ncuts = if options.is_null()
                || *options.offset(METIS_OPTION_NCUTS as libc::c_int as isize)
                    == -(1 as libc::c_int)
            {
                1 as libc::c_int
            } else {
                *options.offset(METIS_OPTION_NCUTS as libc::c_int as isize)
            };
            (*ctrl).niter = if options.is_null()
                || *options.offset(METIS_OPTION_NITER as libc::c_int as isize)
                    == -(1 as libc::c_int)
            {
                10 as libc::c_int
            } else {
                *options.offset(METIS_OPTION_NITER as libc::c_int as isize)
            };
            if ncon == 1 as libc::c_int {
                (*ctrl).iptype = (if options.is_null()
                    || *options.offset(METIS_OPTION_IPTYPE as libc::c_int as isize)
                        == -(1 as libc::c_int)
                {
                    METIS_IPTYPE_GROW as libc::c_int
                } else {
                    *options.offset(METIS_OPTION_IPTYPE as libc::c_int as isize)
                }) as miptype_et;
                (*ctrl).ufactor = if options.is_null()
                    || *options.offset(METIS_OPTION_UFACTOR as libc::c_int as isize)
                        == -(1 as libc::c_int)
                {
                    1 as libc::c_int
                } else {
                    *options.offset(METIS_OPTION_UFACTOR as libc::c_int as isize)
                };
                (*ctrl).CoarsenTo = 20 as libc::c_int;
            } else {
                (*ctrl).iptype = (if options.is_null()
                    || *options.offset(METIS_OPTION_IPTYPE as libc::c_int as isize)
                        == -(1 as libc::c_int)
                {
                    METIS_IPTYPE_RANDOM as libc::c_int
                } else {
                    *options.offset(METIS_OPTION_IPTYPE as libc::c_int as isize)
                }) as miptype_et;
                (*ctrl).ufactor = if options.is_null()
                    || *options.offset(METIS_OPTION_UFACTOR as libc::c_int as isize)
                        == -(1 as libc::c_int)
                {
                    10 as libc::c_int
                } else {
                    *options.offset(METIS_OPTION_UFACTOR as libc::c_int as isize)
                };
                (*ctrl).CoarsenTo = 100 as libc::c_int;
            }
        }
        1 => {
            (*ctrl).objtype = (if options.is_null()
                || *options.offset(METIS_OPTION_OBJTYPE as libc::c_int as isize)
                    == -(1 as libc::c_int)
            {
                METIS_OBJTYPE_CUT as libc::c_int
            } else {
                *options.offset(METIS_OPTION_OBJTYPE as libc::c_int as isize)
            }) as mobjtype_et;
            (*ctrl).iptype = METIS_IPTYPE_METISRB;
            (*ctrl).rtype = METIS_RTYPE_GREEDY;
            (*ctrl).ncuts = if options.is_null()
                || *options.offset(METIS_OPTION_NCUTS as libc::c_int as isize)
                    == -(1 as libc::c_int)
            {
                1 as libc::c_int
            } else {
                *options.offset(METIS_OPTION_NCUTS as libc::c_int as isize)
            };
            (*ctrl).niter = if options.is_null()
                || *options.offset(METIS_OPTION_NITER as libc::c_int as isize)
                    == -(1 as libc::c_int)
            {
                10 as libc::c_int
            } else {
                *options.offset(METIS_OPTION_NITER as libc::c_int as isize)
            };
            (*ctrl).ufactor = if options.is_null()
                || *options.offset(METIS_OPTION_UFACTOR as libc::c_int as isize)
                    == -(1 as libc::c_int)
            {
                30 as libc::c_int
            } else {
                *options.offset(METIS_OPTION_UFACTOR as libc::c_int as isize)
            };
            (*ctrl).minconn = if options.is_null()
                || *options.offset(METIS_OPTION_MINCONN as libc::c_int as isize)
                    == -(1 as libc::c_int)
            {
                0 as libc::c_int
            } else {
                *options.offset(METIS_OPTION_MINCONN as libc::c_int as isize)
            };
            (*ctrl).contig = if options.is_null()
                || *options.offset(METIS_OPTION_CONTIG as libc::c_int as isize)
                    == -(1 as libc::c_int)
            {
                0 as libc::c_int
            } else {
                *options.offset(METIS_OPTION_CONTIG as libc::c_int as isize)
            };
        }
        2 => {
            (*ctrl).objtype = (if options.is_null()
                || *options.offset(METIS_OPTION_OBJTYPE as libc::c_int as isize)
                    == -(1 as libc::c_int)
            {
                METIS_OBJTYPE_NODE as libc::c_int
            } else {
                *options.offset(METIS_OPTION_OBJTYPE as libc::c_int as isize)
            }) as mobjtype_et;
            (*ctrl).rtype = (if options.is_null()
                || *options.offset(METIS_OPTION_RTYPE as libc::c_int as isize)
                    == -(1 as libc::c_int)
            {
                METIS_RTYPE_SEP1SIDED as libc::c_int
            } else {
                *options.offset(METIS_OPTION_RTYPE as libc::c_int as isize)
            }) as mrtype_et;
            (*ctrl).iptype = (if options.is_null()
                || *options.offset(METIS_OPTION_IPTYPE as libc::c_int as isize)
                    == -(1 as libc::c_int)
            {
                METIS_IPTYPE_EDGE as libc::c_int
            } else {
                *options.offset(METIS_OPTION_IPTYPE as libc::c_int as isize)
            }) as miptype_et;
            (*ctrl).nseps = if options.is_null()
                || *options.offset(METIS_OPTION_NSEPS as libc::c_int as isize)
                    == -(1 as libc::c_int)
            {
                1 as libc::c_int
            } else {
                *options.offset(METIS_OPTION_NSEPS as libc::c_int as isize)
            };
            (*ctrl).niter = if options.is_null()
                || *options.offset(METIS_OPTION_NITER as libc::c_int as isize)
                    == -(1 as libc::c_int)
            {
                10 as libc::c_int
            } else {
                *options.offset(METIS_OPTION_NITER as libc::c_int as isize)
            };
            (*ctrl).ufactor = if options.is_null()
                || *options.offset(METIS_OPTION_UFACTOR as libc::c_int as isize)
                    == -(1 as libc::c_int)
            {
                200 as libc::c_int
            } else {
                *options.offset(METIS_OPTION_UFACTOR as libc::c_int as isize)
            };
            (*ctrl).compress = if options.is_null()
                || *options.offset(METIS_OPTION_COMPRESS as libc::c_int as isize)
                    == -(1 as libc::c_int)
            {
                1 as libc::c_int
            } else {
                *options.offset(METIS_OPTION_COMPRESS as libc::c_int as isize)
            };
            (*ctrl).ccorder = if options.is_null()
                || *options.offset(METIS_OPTION_CCORDER as libc::c_int as isize)
                    == -(1 as libc::c_int)
            {
                0 as libc::c_int
            } else {
                *options.offset(METIS_OPTION_CCORDER as libc::c_int as isize)
            };
            (*ctrl).pfactor = (0.1f64
                * (if options.is_null()
                    || *options.offset(METIS_OPTION_PFACTOR as libc::c_int as isize)
                        == -(1 as libc::c_int)
                {
                    0 as libc::c_int
                } else {
                    *options.offset(METIS_OPTION_PFACTOR as libc::c_int as isize)
                }) as libc::c_double) as real_t;
            (*ctrl).CoarsenTo = 100 as libc::c_int;
        }
        _ => {
            gk_errexit(
                15 as libc::c_int,
                b"Unknown optype of %d\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                optype as libc::c_uint,
            );
        }
    }
    (*ctrl).ctype = (if options.is_null()
        || *options.offset(METIS_OPTION_CTYPE as libc::c_int as isize) == -(1 as libc::c_int)
    {
        METIS_CTYPE_SHEM as libc::c_int
    } else {
        *options.offset(METIS_OPTION_CTYPE as libc::c_int as isize)
    }) as mctype_et;
    (*ctrl).no2hop = if options.is_null()
        || *options.offset(METIS_OPTION_NO2HOP as libc::c_int as isize) == -(1 as libc::c_int)
    {
        0 as libc::c_int
    } else {
        *options.offset(METIS_OPTION_NO2HOP as libc::c_int as isize)
    };
    (*ctrl).seed = if options.is_null()
        || *options.offset(METIS_OPTION_SEED as libc::c_int as isize) == -(1 as libc::c_int)
    {
        -(1 as libc::c_int)
    } else {
        *options.offset(METIS_OPTION_SEED as libc::c_int as isize)
    };
    (*ctrl).dbglvl = (if options.is_null()
        || *options.offset(METIS_OPTION_DBGLVL as libc::c_int as isize) == -(1 as libc::c_int)
    {
        0 as libc::c_int
    } else {
        *options.offset(METIS_OPTION_DBGLVL as libc::c_int as isize)
    }) as mdbglvl_et;
    (*ctrl).numflag = if options.is_null()
        || *options.offset(METIS_OPTION_NUMBERING as libc::c_int as isize) == -(1 as libc::c_int)
    {
        0 as libc::c_int
    } else {
        *options.offset(METIS_OPTION_NUMBERING as libc::c_int as isize)
    };
    (*ctrl).optype = optype;
    (*ctrl).ncon = ncon;
    (*ctrl).nparts = nparts;
    (*ctrl).maxvwgt = libmetis__ismalloc(
        ncon as size_t,
        0 as libc::c_int,
        b"SetupCtrl: maxvwgt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if (*ctrl).optype as libc::c_uint != METIS_OP_OMETIS as libc::c_int as libc::c_uint {
        (*ctrl).tpwgts = libmetis__rmalloc(
            (nparts * ncon) as size_t,
            b"SetupCtrl: ctrl->tpwgts\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if !tpwgts.is_null() {
            libmetis__rcopy((nparts * ncon) as size_t, tpwgts, (*ctrl).tpwgts);
        } else {
            i = 0 as libc::c_int;
            while i < nparts {
                j = 0 as libc::c_int;
                while j < ncon {
                    *((*ctrl).tpwgts).offset((i * ncon + j) as isize) =
                        (1.0f64 / nparts as libc::c_double) as real_t;
                    j += 1;
                    j;
                }
                i += 1;
                i;
            }
        }
    } else {
        (*ctrl).tpwgts = libmetis__rsmalloc(
            2 as libc::c_int as size_t,
            0.5f64 as real_t,
            b"SetupCtrl: ctrl->tpwgts\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    (*ctrl).ubfactors = libmetis__rsmalloc(
        (*ctrl).ncon as size_t,
        (1.0f64 + 0.001f64 * (*ctrl).ufactor as libc::c_double) as real_t,
        b"SetupCtrl: ubfactors\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !ubvec.is_null() {
        libmetis__rcopy((*ctrl).ncon as size_t, ubvec, (*ctrl).ubfactors);
    }
    i = 0 as libc::c_int;
    while i < (*ctrl).ncon {
        let ref mut fresh0 = *((*ctrl).ubfactors).offset(i as isize);
        *fresh0 = (*fresh0 as libc::c_double + 0.0000499f64) as real_t;
        i += 1;
        i;
    }
    (*ctrl).pijbm = libmetis__rmalloc(
        (nparts * ncon) as size_t,
        b"SetupCtrl: ctrl->pijbm\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    libmetis__InitRandom((*ctrl).seed);
    if (*ctrl).dbglvl as libc::c_uint & METIS_DBG_INFO as libc::c_int as libc::c_uint != 0 {
        libmetis__PrintCtrl(ctrl);
    }
    if libmetis__CheckParams(ctrl) == 0 {
        libmetis__FreeCtrl(&mut ctrl);
        return 0 as *mut ctrl_t;
    } else {
        return ctrl;
    };
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__SetupKWayBalMultipliers(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
) {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    i = 0 as libc::c_int;
    while i < (*ctrl).nparts {
        j = 0 as libc::c_int;
        while j < (*graph).ncon {
            *((*ctrl).pijbm).offset((i * (*graph).ncon + j) as isize) = *((*graph).invtvwgt)
                .offset(j as isize)
                / *((*ctrl).tpwgts).offset((i * (*graph).ncon + j) as isize);
            j += 1;
            j;
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__Setup2WayBalMultipliers(
    mut ctrl: *mut ctrl_t,
    mut graph: *mut graph_t,
    mut tpwgts: *mut real_t,
) {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        j = 0 as libc::c_int;
        while j < (*graph).ncon {
            *((*ctrl).pijbm).offset((i * (*graph).ncon + j) as isize) = *((*graph).invtvwgt)
                .offset(j as isize)
                / *tpwgts.offset((i * (*graph).ncon + j) as isize);
            j += 1;
            j;
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__PrintCtrl(mut ctrl: *mut ctrl_t) {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut modnum: idx_t = 0;
    printf(b" Runtime parameters:\n\0" as *const u8 as *const libc::c_char);
    printf(b"   Objective type: \0" as *const u8 as *const libc::c_char);
    match (*ctrl).objtype as libc::c_uint {
        0 => {
            printf(b"METIS_OBJTYPE_CUT\n\0" as *const u8 as *const libc::c_char);
        }
        1 => {
            printf(b"METIS_OBJTYPE_VOL\n\0" as *const u8 as *const libc::c_char);
        }
        2 => {
            printf(b"METIS_OBJTYPE_NODE\n\0" as *const u8 as *const libc::c_char);
        }
        _ => {
            printf(b"Unknown!\n\0" as *const u8 as *const libc::c_char);
        }
    }
    printf(b"   Coarsening type: \0" as *const u8 as *const libc::c_char);
    match (*ctrl).ctype as libc::c_uint {
        0 => {
            printf(b"METIS_CTYPE_RM\n\0" as *const u8 as *const libc::c_char);
        }
        1 => {
            printf(b"METIS_CTYPE_SHEM\n\0" as *const u8 as *const libc::c_char);
        }
        _ => {
            printf(b"Unknown!\n\0" as *const u8 as *const libc::c_char);
        }
    }
    printf(b"   Initial partitioning type: \0" as *const u8 as *const libc::c_char);
    match (*ctrl).iptype as libc::c_uint {
        0 => {
            printf(b"METIS_IPTYPE_GROW\n\0" as *const u8 as *const libc::c_char);
        }
        1 => {
            printf(b"METIS_IPTYPE_RANDOM\n\0" as *const u8 as *const libc::c_char);
        }
        2 => {
            printf(b"METIS_IPTYPE_EDGE\n\0" as *const u8 as *const libc::c_char);
        }
        3 => {
            printf(b"METIS_IPTYPE_NODE\n\0" as *const u8 as *const libc::c_char);
        }
        4 => {
            printf(b"METIS_IPTYPE_METISRB\n\0" as *const u8 as *const libc::c_char);
        }
        _ => {
            printf(b"Unknown!\n\0" as *const u8 as *const libc::c_char);
        }
    }
    printf(b"   Refinement type: \0" as *const u8 as *const libc::c_char);
    match (*ctrl).rtype as libc::c_uint {
        0 => {
            printf(b"METIS_RTYPE_FM\n\0" as *const u8 as *const libc::c_char);
        }
        1 => {
            printf(b"METIS_RTYPE_GREEDY\n\0" as *const u8 as *const libc::c_char);
        }
        2 => {
            printf(b"METIS_RTYPE_SEP2SIDED\n\0" as *const u8 as *const libc::c_char);
        }
        3 => {
            printf(b"METIS_RTYPE_SEP1SIDED\n\0" as *const u8 as *const libc::c_char);
        }
        _ => {
            printf(b"Unknown!\n\0" as *const u8 as *const libc::c_char);
        }
    }
    printf(
        b"   Perform a 2-hop matching: %s\n\0" as *const u8 as *const libc::c_char,
        if (*ctrl).no2hop != 0 {
            b"Yes\0" as *const u8 as *const libc::c_char
        } else {
            b"No\0" as *const u8 as *const libc::c_char
        },
    );
    printf(
        b"   Number of balancing constraints: %d\n\0" as *const u8 as *const libc::c_char,
        (*ctrl).ncon,
    );
    printf(
        b"   Number of refinement iterations: %d\n\0" as *const u8 as *const libc::c_char,
        (*ctrl).niter,
    );
    printf(
        b"   Random number seed: %d\n\0" as *const u8 as *const libc::c_char,
        (*ctrl).seed,
    );
    if (*ctrl).optype as libc::c_uint == METIS_OP_OMETIS as libc::c_int as libc::c_uint {
        printf(
            b"   Number of separators: %d\n\0" as *const u8 as *const libc::c_char,
            (*ctrl).nseps,
        );
        printf(
            b"   Compress graph prior to ordering: %s\n\0" as *const u8 as *const libc::c_char,
            if (*ctrl).compress != 0 {
                b"Yes\0" as *const u8 as *const libc::c_char
            } else {
                b"No\0" as *const u8 as *const libc::c_char
            },
        );
        printf(
            b"   Detect & order connected components separately: %s\n\0" as *const u8
                as *const libc::c_char,
            if (*ctrl).ccorder != 0 {
                b"Yes\0" as *const u8 as *const libc::c_char
            } else {
                b"No\0" as *const u8 as *const libc::c_char
            },
        );
        printf(
            b"   Prunning factor for high degree vertices: %f\n\0" as *const u8
                as *const libc::c_char,
            (*ctrl).pfactor as libc::c_double,
        );
    } else {
        printf(
            b"   Number of partitions: %d\n\0" as *const u8 as *const libc::c_char,
            (*ctrl).nparts,
        );
        printf(
            b"   Number of cuts: %d\n\0" as *const u8 as *const libc::c_char,
            (*ctrl).ncuts,
        );
        printf(
            b"   User-supplied ufactor: %d\n\0" as *const u8 as *const libc::c_char,
            (*ctrl).ufactor,
        );
        if (*ctrl).optype as libc::c_uint == METIS_OP_KMETIS as libc::c_int as libc::c_uint {
            printf(
                b"   Minimize connectivity: %s\n\0" as *const u8 as *const libc::c_char,
                if (*ctrl).minconn != 0 {
                    b"Yes\0" as *const u8 as *const libc::c_char
                } else {
                    b"No\0" as *const u8 as *const libc::c_char
                },
            );
            printf(
                b"   Create contigous partitions: %s\n\0" as *const u8 as *const libc::c_char,
                if (*ctrl).contig != 0 {
                    b"Yes\0" as *const u8 as *const libc::c_char
                } else {
                    b"No\0" as *const u8 as *const libc::c_char
                },
            );
        }
        modnum = if (*ctrl).ncon == 1 as libc::c_int {
            5 as libc::c_int
        } else if (*ctrl).ncon == 2 as libc::c_int {
            3 as libc::c_int
        } else if (*ctrl).ncon == 3 as libc::c_int {
            2 as libc::c_int
        } else {
            1 as libc::c_int
        };
        printf(b"   Target partition weights: \0" as *const u8 as *const libc::c_char);
        i = 0 as libc::c_int;
        while i < (*ctrl).nparts {
            if i % modnum == 0 as libc::c_int {
                printf(b"\n     \0" as *const u8 as *const libc::c_char);
            }
            printf(b"%4d=[\0" as *const u8 as *const libc::c_char, i);
            j = 0 as libc::c_int;
            while j < (*ctrl).ncon {
                printf(
                    b"%s%.2e\0" as *const u8 as *const libc::c_char,
                    if j == 0 as libc::c_int {
                        b"\0" as *const u8 as *const libc::c_char
                    } else {
                        b" \0" as *const u8 as *const libc::c_char
                    },
                    *((*ctrl).tpwgts).offset((i * (*ctrl).ncon + j) as isize) as libc::c_double,
                );
                j += 1;
                j;
            }
            printf(b"]\0" as *const u8 as *const libc::c_char);
            i += 1;
            i;
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
    }
    printf(b"   Allowed maximum load imbalance: \0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < (*ctrl).ncon {
        printf(
            b"%.3f \0" as *const u8 as *const libc::c_char,
            *((*ctrl).ubfactors).offset(i as isize) as libc::c_double,
        );
        i += 1;
        i;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"\n\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__CheckParams(mut ctrl: *mut ctrl_t) -> libc::c_int {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut sum: real_t = 0.;
    let mut dbglvl: mdbglvl_et = METIS_DBG_INFO;
    match (*ctrl).optype as libc::c_uint {
        0 => {
            if (*ctrl).objtype as libc::c_uint != METIS_OBJTYPE_CUT as libc::c_int as libc::c_uint {
                if dbglvl as libc::c_uint & METIS_DBG_INFO as libc::c_int as libc::c_uint != 0 {
                    printf(
                        b"Input Error: Incorrect objective type.\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                return 0 as libc::c_int;
            }
            if (*ctrl).ctype as libc::c_uint != METIS_CTYPE_RM as libc::c_int as libc::c_uint
                && (*ctrl).ctype as libc::c_uint != METIS_CTYPE_SHEM as libc::c_int as libc::c_uint
            {
                if dbglvl as libc::c_uint & METIS_DBG_INFO as libc::c_int as libc::c_uint != 0 {
                    printf(
                        b"Input Error: Incorrect coarsening scheme.\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                return 0 as libc::c_int;
            }
            if (*ctrl).iptype as libc::c_uint != METIS_IPTYPE_GROW as libc::c_int as libc::c_uint
                && (*ctrl).iptype as libc::c_uint
                    != METIS_IPTYPE_RANDOM as libc::c_int as libc::c_uint
            {
                if dbglvl as libc::c_uint & METIS_DBG_INFO as libc::c_int as libc::c_uint != 0 {
                    printf(
                        b"Input Error: Incorrect initial partitioning scheme.\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                return 0 as libc::c_int;
            }
            if (*ctrl).rtype as libc::c_uint != METIS_RTYPE_FM as libc::c_int as libc::c_uint {
                if dbglvl as libc::c_uint & METIS_DBG_INFO as libc::c_int as libc::c_uint != 0 {
                    printf(
                        b"Input Error: Incorrect refinement scheme.\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                return 0 as libc::c_int;
            }
            if (*ctrl).ncuts <= 0 as libc::c_int {
                if dbglvl as libc::c_uint & METIS_DBG_INFO as libc::c_int as libc::c_uint != 0 {
                    printf(
                        b"Input Error: Incorrect ncuts.\n\0" as *const u8 as *const libc::c_char,
                    );
                }
                return 0 as libc::c_int;
            }
            if (*ctrl).niter <= 0 as libc::c_int {
                if dbglvl as libc::c_uint & METIS_DBG_INFO as libc::c_int as libc::c_uint != 0 {
                    printf(
                        b"Input Error: Incorrect niter.\n\0" as *const u8 as *const libc::c_char,
                    );
                }
                return 0 as libc::c_int;
            }
            if (*ctrl).ufactor <= 0 as libc::c_int {
                if dbglvl as libc::c_uint & METIS_DBG_INFO as libc::c_int as libc::c_uint != 0 {
                    printf(
                        b"Input Error: Incorrect ufactor.\n\0" as *const u8 as *const libc::c_char,
                    );
                }
                return 0 as libc::c_int;
            }
            if (*ctrl).numflag != 0 as libc::c_int && (*ctrl).numflag != 1 as libc::c_int {
                if dbglvl as libc::c_uint & METIS_DBG_INFO as libc::c_int as libc::c_uint != 0 {
                    printf(
                        b"Input Error: Incorrect numflag.\n\0" as *const u8 as *const libc::c_char,
                    );
                }
                return 0 as libc::c_int;
            }
            if (*ctrl).nparts <= 0 as libc::c_int {
                if dbglvl as libc::c_uint & METIS_DBG_INFO as libc::c_int as libc::c_uint != 0 {
                    printf(
                        b"Input Error: Incorrect nparts.\n\0" as *const u8 as *const libc::c_char,
                    );
                }
                return 0 as libc::c_int;
            }
            if (*ctrl).ncon <= 0 as libc::c_int {
                if dbglvl as libc::c_uint & METIS_DBG_INFO as libc::c_int as libc::c_uint != 0 {
                    printf(b"Input Error: Incorrect ncon.\n\0" as *const u8 as *const libc::c_char);
                }
                return 0 as libc::c_int;
            }
            i = 0 as libc::c_int;
            while i < (*ctrl).ncon {
                sum = libmetis__rsum(
                    (*ctrl).nparts as size_t,
                    ((*ctrl).tpwgts).offset(i as isize),
                    (*ctrl).ncon as size_t,
                );
                if (sum as libc::c_double) < 0.99f64 || sum as libc::c_double > 1.01f64 {
                    if dbglvl as libc::c_uint & METIS_DBG_INFO as libc::c_int as libc::c_uint != 0 {
                        printf(
                            b"Input Error: Incorrect sum of %f for tpwgts for constraint %d.\n\0"
                                as *const u8 as *const libc::c_char,
                            sum as libc::c_double,
                            i,
                        );
                    }
                    return 0 as libc::c_int;
                }
                i += 1;
                i;
            }
            i = 0 as libc::c_int;
            while i < (*ctrl).ncon {
                j = 0 as libc::c_int;
                while j < (*ctrl).nparts {
                    if *((*ctrl).tpwgts).offset((j * (*ctrl).ncon + i) as isize) as libc::c_double
                        <= 0.0f64
                    {
                        if dbglvl as libc::c_uint & METIS_DBG_INFO as libc::c_int as libc::c_uint
                            != 0
                        {
                            printf(
                                b"Input Error: Incorrect tpwgts for partition %d and constraint %d.\n\0"
                                    as *const u8 as *const libc::c_char,
                                j,
                                i,
                            );
                        }
                        return 0 as libc::c_int;
                    }
                    j += 1;
                    j;
                }
                i += 1;
                i;
            }
            i = 0 as libc::c_int;
            while i < (*ctrl).ncon {
                if *((*ctrl).ubfactors).offset(i as isize) as libc::c_double <= 1.0f64 {
                    if dbglvl as libc::c_uint & METIS_DBG_INFO as libc::c_int as libc::c_uint != 0 {
                        printf(
                            b"Input Error: Incorrect ubfactor for constraint %d.\n\0" as *const u8
                                as *const libc::c_char,
                            i,
                        );
                    }
                    return 0 as libc::c_int;
                }
                i += 1;
                i;
            }
        }
        1 => {
            if (*ctrl).objtype as libc::c_uint != METIS_OBJTYPE_CUT as libc::c_int as libc::c_uint
                && (*ctrl).objtype as libc::c_uint
                    != METIS_OBJTYPE_VOL as libc::c_int as libc::c_uint
            {
                if dbglvl as libc::c_uint & METIS_DBG_INFO as libc::c_int as libc::c_uint != 0 {
                    printf(
                        b"Input Error: Incorrect objective type.\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                return 0 as libc::c_int;
            }
            if (*ctrl).ctype as libc::c_uint != METIS_CTYPE_RM as libc::c_int as libc::c_uint
                && (*ctrl).ctype as libc::c_uint != METIS_CTYPE_SHEM as libc::c_int as libc::c_uint
            {
                if dbglvl as libc::c_uint & METIS_DBG_INFO as libc::c_int as libc::c_uint != 0 {
                    printf(
                        b"Input Error: Incorrect coarsening scheme.\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                return 0 as libc::c_int;
            }
            if (*ctrl).iptype as libc::c_uint != METIS_IPTYPE_METISRB as libc::c_int as libc::c_uint
            {
                if dbglvl as libc::c_uint & METIS_DBG_INFO as libc::c_int as libc::c_uint != 0 {
                    printf(
                        b"Input Error: Incorrect initial partitioning scheme.\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                return 0 as libc::c_int;
            }
            if (*ctrl).rtype as libc::c_uint != METIS_RTYPE_GREEDY as libc::c_int as libc::c_uint {
                if dbglvl as libc::c_uint & METIS_DBG_INFO as libc::c_int as libc::c_uint != 0 {
                    printf(
                        b"Input Error: Incorrect refinement scheme.\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                return 0 as libc::c_int;
            }
            if (*ctrl).ncuts <= 0 as libc::c_int {
                if dbglvl as libc::c_uint & METIS_DBG_INFO as libc::c_int as libc::c_uint != 0 {
                    printf(
                        b"Input Error: Incorrect ncuts.\n\0" as *const u8 as *const libc::c_char,
                    );
                }
                return 0 as libc::c_int;
            }
            if (*ctrl).niter <= 0 as libc::c_int {
                if dbglvl as libc::c_uint & METIS_DBG_INFO as libc::c_int as libc::c_uint != 0 {
                    printf(
                        b"Input Error: Incorrect niter.\n\0" as *const u8 as *const libc::c_char,
                    );
                }
                return 0 as libc::c_int;
            }
            if (*ctrl).ufactor <= 0 as libc::c_int {
                if dbglvl as libc::c_uint & METIS_DBG_INFO as libc::c_int as libc::c_uint != 0 {
                    printf(
                        b"Input Error: Incorrect ufactor.\n\0" as *const u8 as *const libc::c_char,
                    );
                }
                return 0 as libc::c_int;
            }
            if (*ctrl).numflag != 0 as libc::c_int && (*ctrl).numflag != 1 as libc::c_int {
                if dbglvl as libc::c_uint & METIS_DBG_INFO as libc::c_int as libc::c_uint != 0 {
                    printf(
                        b"Input Error: Incorrect numflag.\n\0" as *const u8 as *const libc::c_char,
                    );
                }
                return 0 as libc::c_int;
            }
            if (*ctrl).nparts <= 0 as libc::c_int {
                if dbglvl as libc::c_uint & METIS_DBG_INFO as libc::c_int as libc::c_uint != 0 {
                    printf(
                        b"Input Error: Incorrect nparts.\n\0" as *const u8 as *const libc::c_char,
                    );
                }
                return 0 as libc::c_int;
            }
            if (*ctrl).ncon <= 0 as libc::c_int {
                if dbglvl as libc::c_uint & METIS_DBG_INFO as libc::c_int as libc::c_uint != 0 {
                    printf(b"Input Error: Incorrect ncon.\n\0" as *const u8 as *const libc::c_char);
                }
                return 0 as libc::c_int;
            }
            if (*ctrl).contig != 0 as libc::c_int && (*ctrl).contig != 1 as libc::c_int {
                if dbglvl as libc::c_uint & METIS_DBG_INFO as libc::c_int as libc::c_uint != 0 {
                    printf(
                        b"Input Error: Incorrect contig.\n\0" as *const u8 as *const libc::c_char,
                    );
                }
                return 0 as libc::c_int;
            }
            if (*ctrl).minconn != 0 as libc::c_int && (*ctrl).minconn != 1 as libc::c_int {
                if dbglvl as libc::c_uint & METIS_DBG_INFO as libc::c_int as libc::c_uint != 0 {
                    printf(
                        b"Input Error: Incorrect minconn.\n\0" as *const u8 as *const libc::c_char,
                    );
                }
                return 0 as libc::c_int;
            }
            i = 0 as libc::c_int;
            while i < (*ctrl).ncon {
                sum = libmetis__rsum(
                    (*ctrl).nparts as size_t,
                    ((*ctrl).tpwgts).offset(i as isize),
                    (*ctrl).ncon as size_t,
                );
                if (sum as libc::c_double) < 0.99f64 || sum as libc::c_double > 1.01f64 {
                    if dbglvl as libc::c_uint & METIS_DBG_INFO as libc::c_int as libc::c_uint != 0 {
                        printf(
                            b"Input Error: Incorrect sum of %f for tpwgts for constraint %d.\n\0"
                                as *const u8 as *const libc::c_char,
                            sum as libc::c_double,
                            i,
                        );
                    }
                    return 0 as libc::c_int;
                }
                i += 1;
                i;
            }
            i = 0 as libc::c_int;
            while i < (*ctrl).ncon {
                j = 0 as libc::c_int;
                while j < (*ctrl).nparts {
                    if *((*ctrl).tpwgts).offset((j * (*ctrl).ncon + i) as isize) as libc::c_double
                        <= 0.0f64
                    {
                        if dbglvl as libc::c_uint & METIS_DBG_INFO as libc::c_int as libc::c_uint
                            != 0
                        {
                            printf(
                                b"Input Error: Incorrect tpwgts for partition %d and constraint %d.\n\0"
                                    as *const u8 as *const libc::c_char,
                                j,
                                i,
                            );
                        }
                        return 0 as libc::c_int;
                    }
                    j += 1;
                    j;
                }
                i += 1;
                i;
            }
            i = 0 as libc::c_int;
            while i < (*ctrl).ncon {
                if *((*ctrl).ubfactors).offset(i as isize) as libc::c_double <= 1.0f64 {
                    if dbglvl as libc::c_uint & METIS_DBG_INFO as libc::c_int as libc::c_uint != 0 {
                        printf(
                            b"Input Error: Incorrect ubfactor for constraint %d.\n\0" as *const u8
                                as *const libc::c_char,
                            i,
                        );
                    }
                    return 0 as libc::c_int;
                }
                i += 1;
                i;
            }
        }
        2 => {
            if (*ctrl).objtype as libc::c_uint != METIS_OBJTYPE_NODE as libc::c_int as libc::c_uint
            {
                if dbglvl as libc::c_uint & METIS_DBG_INFO as libc::c_int as libc::c_uint != 0 {
                    printf(
                        b"Input Error: Incorrect objective type.\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                return 0 as libc::c_int;
            }
            if (*ctrl).ctype as libc::c_uint != METIS_CTYPE_RM as libc::c_int as libc::c_uint
                && (*ctrl).ctype as libc::c_uint != METIS_CTYPE_SHEM as libc::c_int as libc::c_uint
            {
                if dbglvl as libc::c_uint & METIS_DBG_INFO as libc::c_int as libc::c_uint != 0 {
                    printf(
                        b"Input Error: Incorrect coarsening scheme.\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                return 0 as libc::c_int;
            }
            if (*ctrl).iptype as libc::c_uint != METIS_IPTYPE_EDGE as libc::c_int as libc::c_uint
                && (*ctrl).iptype as libc::c_uint
                    != METIS_IPTYPE_NODE as libc::c_int as libc::c_uint
            {
                if dbglvl as libc::c_uint & METIS_DBG_INFO as libc::c_int as libc::c_uint != 0 {
                    printf(
                        b"Input Error: Incorrect initial partitioning scheme.\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                return 0 as libc::c_int;
            }
            if (*ctrl).rtype as libc::c_uint != METIS_RTYPE_SEP1SIDED as libc::c_int as libc::c_uint
                && (*ctrl).rtype as libc::c_uint
                    != METIS_RTYPE_SEP2SIDED as libc::c_int as libc::c_uint
            {
                if dbglvl as libc::c_uint & METIS_DBG_INFO as libc::c_int as libc::c_uint != 0 {
                    printf(
                        b"Input Error: Incorrect refinement scheme.\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                return 0 as libc::c_int;
            }
            if (*ctrl).nseps <= 0 as libc::c_int {
                if dbglvl as libc::c_uint & METIS_DBG_INFO as libc::c_int as libc::c_uint != 0 {
                    printf(
                        b"Input Error: Incorrect nseps.\n\0" as *const u8 as *const libc::c_char,
                    );
                }
                return 0 as libc::c_int;
            }
            if (*ctrl).niter <= 0 as libc::c_int {
                if dbglvl as libc::c_uint & METIS_DBG_INFO as libc::c_int as libc::c_uint != 0 {
                    printf(
                        b"Input Error: Incorrect niter.\n\0" as *const u8 as *const libc::c_char,
                    );
                }
                return 0 as libc::c_int;
            }
            if (*ctrl).ufactor <= 0 as libc::c_int {
                if dbglvl as libc::c_uint & METIS_DBG_INFO as libc::c_int as libc::c_uint != 0 {
                    printf(
                        b"Input Error: Incorrect ufactor.\n\0" as *const u8 as *const libc::c_char,
                    );
                }
                return 0 as libc::c_int;
            }
            if (*ctrl).numflag != 0 as libc::c_int && (*ctrl).numflag != 1 as libc::c_int {
                if dbglvl as libc::c_uint & METIS_DBG_INFO as libc::c_int as libc::c_uint != 0 {
                    printf(
                        b"Input Error: Incorrect numflag.\n\0" as *const u8 as *const libc::c_char,
                    );
                }
                return 0 as libc::c_int;
            }
            if (*ctrl).nparts != 3 as libc::c_int {
                if dbglvl as libc::c_uint & METIS_DBG_INFO as libc::c_int as libc::c_uint != 0 {
                    printf(
                        b"Input Error: Incorrect nparts.\n\0" as *const u8 as *const libc::c_char,
                    );
                }
                return 0 as libc::c_int;
            }
            if (*ctrl).ncon != 1 as libc::c_int {
                if dbglvl as libc::c_uint & METIS_DBG_INFO as libc::c_int as libc::c_uint != 0 {
                    printf(b"Input Error: Incorrect ncon.\n\0" as *const u8 as *const libc::c_char);
                }
                return 0 as libc::c_int;
            }
            if (*ctrl).compress != 0 as libc::c_int && (*ctrl).compress != 1 as libc::c_int {
                if dbglvl as libc::c_uint & METIS_DBG_INFO as libc::c_int as libc::c_uint != 0 {
                    printf(
                        b"Input Error: Incorrect compress.\n\0" as *const u8 as *const libc::c_char,
                    );
                }
                return 0 as libc::c_int;
            }
            if (*ctrl).ccorder != 0 as libc::c_int && (*ctrl).ccorder != 1 as libc::c_int {
                if dbglvl as libc::c_uint & METIS_DBG_INFO as libc::c_int as libc::c_uint != 0 {
                    printf(
                        b"Input Error: Incorrect ccorder.\n\0" as *const u8 as *const libc::c_char,
                    );
                }
                return 0 as libc::c_int;
            }
            if ((*ctrl).pfactor as libc::c_double) < 0.0f64 {
                if dbglvl as libc::c_uint & METIS_DBG_INFO as libc::c_int as libc::c_uint != 0 {
                    printf(
                        b"Input Error: Incorrect pfactor.\n\0" as *const u8 as *const libc::c_char,
                    );
                }
                return 0 as libc::c_int;
            }
            i = 0 as libc::c_int;
            while i < (*ctrl).ncon {
                if *((*ctrl).ubfactors).offset(i as isize) as libc::c_double <= 1.0f64 {
                    if dbglvl as libc::c_uint & METIS_DBG_INFO as libc::c_int as libc::c_uint != 0 {
                        printf(
                            b"Input Error: Incorrect ubfactor for constraint %d.\n\0" as *const u8
                                as *const libc::c_char,
                            i,
                        );
                    }
                    return 0 as libc::c_int;
                }
                i += 1;
                i;
            }
        }
        _ => {
            if dbglvl as libc::c_uint & METIS_DBG_INFO as libc::c_int as libc::c_uint != 0 {
                printf(b"Input Error: Incorrect optype\n\0" as *const u8 as *const libc::c_char);
            }
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__FreeCtrl(mut r_ctrl: *mut *mut ctrl_t) {
    let mut ctrl: *mut ctrl_t = *r_ctrl;
    libmetis__FreeWorkSpace(ctrl);
    gk_free(
        &mut (*ctrl).tpwgts as *mut *mut real_t as *mut *mut libc::c_void,
        &mut (*ctrl).pijbm as *mut *mut real_t,
        &mut (*ctrl).ubfactors as *mut *mut real_t,
        &mut (*ctrl).maxvwgt as *mut *mut idx_t,
        &mut ctrl as *mut *mut ctrl_t,
        0 as *mut *mut libc::c_void,
    );
    *r_ctrl = 0 as *mut ctrl_t;
}
