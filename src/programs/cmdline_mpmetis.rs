use ::libc;
extern "C" {
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: u64) -> *mut libc::c_void;
    fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char, _: libc::c_int) -> i64;
    fn exit(_: libc::c_int) -> !;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> u64;
    static mut gk_optarg: *mut libc::c_char;
    static mut gk_optind: libc::c_int;
    fn gk_getopt_long_only(
        _: libc::c_int,
        _: *mut *mut libc::c_char,
        _: *mut libc::c_char,
        _: *mut gk_option,
        _: *mut libc::c_int,
    ) -> libc::c_int;
    fn gk_malloc(nbytes: size_t, msg: *mut libc::c_char) -> *mut libc::c_void;
    fn errexit(_: *mut libc::c_char, _: ...);
    fn gk_strdup(orgstr: *mut libc::c_char) -> *mut libc::c_char;
    fn gk_GetStringID(strmap: *mut gk_StringMap_t, key: *mut libc::c_char) -> libc::c_int;
}

use crate::libmetis::structure::*;

pub type __int32_t = libc::c_int;
pub type int32_t = __int32_t;
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gk_StringMap_t {
    pub name: *mut libc::c_char,
    pub id: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gk_option {
    pub name: *mut libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub type idx_t = int32_t;
pub type real_t = libc::c_float;
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const METIS_PTYPE_KWAY: C2RustUnnamed_0 = 1;
pub const METIS_PTYPE_RB: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const METIS_GTYPE_NODAL: C2RustUnnamed_1 = 1;
pub const METIS_GTYPE_DUAL: C2RustUnnamed_1 = 0;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const METIS_CTYPE_SHEM: C2RustUnnamed_2 = 1;
pub const METIS_CTYPE_RM: C2RustUnnamed_2 = 0;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const METIS_IPTYPE_METISRB: C2RustUnnamed_3 = 4;
pub const METIS_IPTYPE_NODE: C2RustUnnamed_3 = 3;
pub const METIS_IPTYPE_EDGE: C2RustUnnamed_3 = 2;
pub const METIS_IPTYPE_RANDOM: C2RustUnnamed_3 = 1;
pub const METIS_IPTYPE_GROW: C2RustUnnamed_3 = 0;
pub type C2RustUnnamed_4 = libc::c_uint;
pub const METIS_RTYPE_SEP1SIDED: C2RustUnnamed_4 = 3;
pub const METIS_RTYPE_SEP2SIDED: C2RustUnnamed_4 = 2;
pub const METIS_RTYPE_GREEDY: C2RustUnnamed_4 = 1;
pub const METIS_RTYPE_FM: C2RustUnnamed_4 = 0;
pub type C2RustUnnamed_5 = libc::c_uint;
pub const METIS_OBJTYPE_NODE: C2RustUnnamed_5 = 2;
pub const METIS_OBJTYPE_VOL: C2RustUnnamed_5 = 1;
pub const METIS_OBJTYPE_CUT: C2RustUnnamed_5 = 0;

#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
static mut long_options: [gk_option; 17] = [
    {
        let mut init = gk_option {
            name: b"gtype\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: METIS_OPTION_GTYPE as libc::c_int,
        };
        init
    },
    {
        let mut init = gk_option {
            name: b"ptype\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: METIS_OPTION_PTYPE as libc::c_int,
        };
        init
    },
    {
        let mut init = gk_option {
            name: b"objtype\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: METIS_OPTION_OBJTYPE as libc::c_int,
        };
        init
    },
    {
        let mut init = gk_option {
            name: b"ctype\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: METIS_OPTION_CTYPE as libc::c_int,
        };
        init
    },
    {
        let mut init = gk_option {
            name: b"iptype\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: METIS_OPTION_IPTYPE as libc::c_int,
        };
        init
    },
    {
        let mut init = gk_option {
            name: b"minconn\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: METIS_OPTION_MINCONN as libc::c_int,
        };
        init
    },
    {
        let mut init = gk_option {
            name: b"contig\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: METIS_OPTION_CONTIG as libc::c_int,
        };
        init
    },
    {
        let mut init = gk_option {
            name: b"nooutput\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: METIS_OPTION_NOOUTPUT as libc::c_int,
        };
        init
    },
    {
        let mut init = gk_option {
            name: b"ufactor\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: METIS_OPTION_UFACTOR as libc::c_int,
        };
        init
    },
    {
        let mut init = gk_option {
            name: b"niter\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: METIS_OPTION_NITER as libc::c_int,
        };
        init
    },
    {
        let mut init = gk_option {
            name: b"ncuts\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: METIS_OPTION_NCUTS as libc::c_int,
        };
        init
    },
    {
        let mut init = gk_option {
            name: b"ncommon\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: METIS_OPTION_NCOMMON as libc::c_int,
        };
        init
    },
    {
        let mut init = gk_option {
            name: b"tpwgts\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: METIS_OPTION_TPWGTS as libc::c_int,
        };
        init
    },
    {
        let mut init = gk_option {
            name: b"seed\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: METIS_OPTION_SEED as libc::c_int,
        };
        init
    },
    {
        let mut init = gk_option {
            name: b"dbglvl\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: METIS_OPTION_DBGLVL as libc::c_int,
        };
        init
    },
    {
        let mut init = gk_option {
            name: b"help\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: METIS_OPTION_HELP as libc::c_int,
        };
        init
    },
    {
        let mut init = gk_option {
            name: 0 as *const libc::c_char as *mut libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
];
static mut gtype_options: [gk_StringMap_t; 3] = [
    {
        let mut init = gk_StringMap_t {
            name: b"dual\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            id: METIS_GTYPE_DUAL as libc::c_int,
        };
        init
    },
    {
        let mut init = gk_StringMap_t {
            name: b"nodal\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            id: METIS_GTYPE_NODAL as libc::c_int,
        };
        init
    },
    {
        let mut init = gk_StringMap_t {
            name: 0 as *const libc::c_char as *mut libc::c_char,
            id: 0 as libc::c_int,
        };
        init
    },
];
static mut ptype_options: [gk_StringMap_t; 3] = [
    {
        let mut init = gk_StringMap_t {
            name: b"rb\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            id: METIS_PTYPE_RB as libc::c_int,
        };
        init
    },
    {
        let mut init = gk_StringMap_t {
            name: b"kway\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            id: METIS_PTYPE_KWAY as libc::c_int,
        };
        init
    },
    {
        let mut init = gk_StringMap_t {
            name: 0 as *const libc::c_char as *mut libc::c_char,
            id: 0 as libc::c_int,
        };
        init
    },
];
static mut objtype_options: [gk_StringMap_t; 3] = [
    {
        let mut init = gk_StringMap_t {
            name: b"cut\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            id: METIS_OBJTYPE_CUT as libc::c_int,
        };
        init
    },
    {
        let mut init = gk_StringMap_t {
            name: b"vol\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            id: METIS_OBJTYPE_VOL as libc::c_int,
        };
        init
    },
    {
        let mut init = gk_StringMap_t {
            name: 0 as *const libc::c_char as *mut libc::c_char,
            id: 0 as libc::c_int,
        };
        init
    },
];
static mut ctype_options: [gk_StringMap_t; 3] = [
    {
        let mut init = gk_StringMap_t {
            name: b"rm\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            id: METIS_CTYPE_RM as libc::c_int,
        };
        init
    },
    {
        let mut init = gk_StringMap_t {
            name: b"shem\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            id: METIS_CTYPE_SHEM as libc::c_int,
        };
        init
    },
    {
        let mut init = gk_StringMap_t {
            name: 0 as *const libc::c_char as *mut libc::c_char,
            id: 0 as libc::c_int,
        };
        init
    },
];
static mut iptype_options: [gk_StringMap_t; 3] = [
    {
        let mut init = gk_StringMap_t {
            name: b"grow\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            id: METIS_IPTYPE_GROW as libc::c_int,
        };
        init
    },
    {
        let mut init = gk_StringMap_t {
            name: b"random\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            id: METIS_IPTYPE_RANDOM as libc::c_int,
        };
        init
    },
    {
        let mut init = gk_StringMap_t {
            name: 0 as *const libc::c_char as *mut libc::c_char,
            id: 0 as libc::c_int,
        };
        init
    },
];
static mut helpstr: [[libc::c_char; 100]; 94] = unsafe {
    [
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b" \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"Usage: mpmetis [options] meshfile nparts\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b" \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b" Required parameters\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"    meshfile    Stores the mesh to be partitioned.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"    nparts      The number of partitions to split the mesh.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b" \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b" Optional parameters\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"  -gtype=string\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     Specifies the graph to be used for computing the partitioning\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     The possible values are:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"        dual     - Partition the dual graph of the mesh [default]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"        nodal    - Partition the nodal graph of the mesh\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b" \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"  -ptype=string\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     Specifies the scheme to be used for computing the k-way partitioning.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     The possible values are:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"        rb       - Recursive bisectioning\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"        kway     - Direct k-way partitioning [default]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b" \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"  -ctype=string\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     Specifies the scheme to be used to match the vertices of the graph\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     during the coarsening.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     The possible values are:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"        rm       - Random matching\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"        shem     - Sorted heavy-edge matching [default]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b" \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"  -iptype=string [applies only when -ptype=rb]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     Specifies the scheme to be used to compute the initial partitioning\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     of the graph.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     The possible values are:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"        grow     - Grow a bisection using a greedy strategy [default]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"        random   - Compute a bisection at random\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b" \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"  -objtype=string [applies only when -ptype=kway]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     Specifies the objective that the partitioning routines will optimize.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     The possible values are:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"        cut      - Minimize the edgecut [default]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"        vol      - Minimize the total communication volume\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b" \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"  -contig [applies only when -ptype=kway]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     Specifies that the partitioning routines should try to produce\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     partitions that are contiguous. Note that if the input graph is not\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     connected this option is ignored.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b" \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"  -minconn [applies only when -ptype=kway]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     Specifies that the partitioning routines should try to minimize the\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     maximum degree of the subdomain graph, i.e., the graph in which each\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     partition is a node, and edges connect subdomains with a shared\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     interface.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b" \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"  -tpwgts=filename\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     Specifies the name of the file that stores the target weights for\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     each partition. By default, all partitions are assumed to be of \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     the same size.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b" \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"  -ufactor=int\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     Specifies the maximum allowed load imbalance among the partitions.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     A value of x indicates that the allowed load imbalance is 1+x/1000.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     For ptype=rb, the load imbalance is measured as the ratio of the \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     2*max(left,right)/(left+right), where left and right are the sizes\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     of the respective partitions at each bisection. \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     For ptype=kway, the load imbalance is measured as the ratio of \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     max_i(pwgts[i])/avgpwgt, where pwgts[i] is the weight of the ith\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     partition and avgpwgt is the sum of the total vertex weights divided\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     by the number of partitions requested.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     For ptype=rb, the default value is 1 (i.e., load imbalance of 1.001).\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     For ptype=kway, the default value is 30 (i.e., load imbalance of 1.03).\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b" \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"  -ncommon=int\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     Specifies the common number of nodes that two elements must have\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     in order to put an edge between them in the dual graph. Default is 1.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b" \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"  -niter=int\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     Specifies the number of iterations for the refinement algorithms\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     at each stage of the uncoarsening process. Default is 10.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b" \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"  -ncuts=int\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     Specifies the number of different partitionings that it will compute.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     The final partitioning is the one that achieves the best edgecut or\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     communication volume. Default is 1.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b" \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"  -nooutput\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     Specifies that no partitioning file should be generated.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b" \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"  -seed=int\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     Selects the seed of the random number generator.  \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b" \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"  -dbglvl=int      \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     Selects the dbglvl.  \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b" \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"  -help\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     Prints this message.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
    ]
};
static mut shorthelpstr: [[libc::c_char; 100]; 4] = unsafe {
    [
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b" \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"   Usage: mpmetis [options] <filename> <nparts>\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"          use 'mpmetis -help' for a summary of the options.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
    ]
};
pub unsafe extern "C" fn parse_cmdline(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> *mut params_t {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut option_index: libc::c_int = 0;
    let mut params: *mut params_t = 0 as *mut params_t;
    params = gk_malloc(
        ::core::mem::size_of::<params_t>() as u64,
        b"parse_cmdline\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut params_t;
    memset(
        params as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<params_t>() as u64,
    );
    (*params).gtype = METIS_GTYPE_DUAL as libc::c_int;
    (*params).ptype = METIS_PTYPE_KWAY as libc::c_int;
    (*params).objtype = METIS_OBJTYPE_CUT as libc::c_int;
    (*params).ctype = METIS_CTYPE_SHEM as libc::c_int;
    (*params).iptype = METIS_IPTYPE_GROW as libc::c_int;
    (*params).rtype = -(1 as libc::c_int);
    (*params).minconn = 0 as libc::c_int;
    (*params).contig = 0 as libc::c_int;
    (*params).nooutput = 0 as libc::c_int;
    (*params).wgtflag = 3 as libc::c_int;
    (*params).ncuts = 1 as libc::c_int;
    (*params).niter = 10 as libc::c_int;
    (*params).ncommon = 1 as libc::c_int;
    (*params).dbglvl = 0 as libc::c_int;
    (*params).balance = 0 as libc::c_int;
    (*params).seed = -(1 as libc::c_int);
    (*params).dbglvl = 0 as libc::c_int;
    (*params).tpwgtsfile = 0 as *mut libc::c_char;
    (*params).filename = 0 as *mut libc::c_char;
    (*params).nparts = 1 as libc::c_int;
    (*params).ufactor = -(1 as libc::c_int);
    (*params).iotimer = 0.0f64 as real_t;
    (*params).parttimer = 0.0f64 as real_t;
    (*params).reporttimer = 0.0f64 as real_t;
    loop {
        c = gk_getopt_long_only(
            argc,
            argv,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            long_options.as_mut_ptr(),
            &mut option_index,
        );
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            23 => {
                if !gk_optarg.is_null() {
                    (*params).gtype = gk_GetStringID(gtype_options.as_mut_ptr(), gk_optarg);
                    if (*params).gtype == -(1 as libc::c_int) {
                        errexit(
                            b"Invalid option -%s=%s\n\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            long_options[option_index as usize].name,
                            gk_optarg,
                        );
                    }
                }
            }
            0 => {
                if !gk_optarg.is_null() {
                    (*params).ptype = gk_GetStringID(ptype_options.as_mut_ptr(), gk_optarg);
                    if (*params).ptype == -(1 as libc::c_int) {
                        errexit(
                            b"Invalid option -%s=%s\n\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            long_options[option_index as usize].name,
                            gk_optarg,
                        );
                    }
                }
            }
            1 => {
                if !gk_optarg.is_null() {
                    (*params).objtype = gk_GetStringID(objtype_options.as_mut_ptr(), gk_optarg);
                    if (*params).objtype == -(1 as libc::c_int) {
                        errexit(
                            b"Invalid option -%s=%s\n\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            long_options[option_index as usize].name,
                            gk_optarg,
                        );
                    }
                }
            }
            2 => {
                if !gk_optarg.is_null() {
                    (*params).ctype = gk_GetStringID(ctype_options.as_mut_ptr(), gk_optarg);
                    if (*params).ctype == -(1 as libc::c_int) {
                        errexit(
                            b"Invalid option -%s=%s\n\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            long_options[option_index as usize].name,
                            gk_optarg,
                        );
                    }
                }
            }
            3 => {
                if !gk_optarg.is_null() {
                    (*params).iptype = gk_GetStringID(iptype_options.as_mut_ptr(), gk_optarg);
                    if (*params).iptype == -(1 as libc::c_int) {
                        errexit(
                            b"Invalid option -%s=%s\n\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            long_options[option_index as usize].name,
                            gk_optarg,
                        );
                    }
                }
            }
            11 => {
                (*params).contig = 1 as libc::c_int;
            }
            10 => {
                (*params).minconn = 1 as libc::c_int;
            }
            21 => {
                (*params).nooutput = 1 as libc::c_int;
            }
            22 => {
                (*params).balance = 1 as libc::c_int;
            }
            19 => {
                if !gk_optarg.is_null() {
                    (*params).tpwgtsfile = gk_strdup(gk_optarg);
                }
            }
            7 => {
                if !gk_optarg.is_null() {
                    (*params).ncuts = atoi(gk_optarg);
                }
            }
            6 => {
                if !gk_optarg.is_null() {
                    (*params).niter = atoi(gk_optarg);
                }
            }
            20 => {
                if !gk_optarg.is_null() {
                    (*params).ncommon = atoi(gk_optarg);
                }
            }
            16 => {
                if !gk_optarg.is_null() {
                    (*params).ufactor = atoi(gk_optarg);
                }
            }
            8 => {
                if !gk_optarg.is_null() {
                    (*params).seed = atoi(gk_optarg);
                }
            }
            5 => {
                if !gk_optarg.is_null() {
                    (*params).dbglvl = atoi(gk_optarg);
                }
            }
            18 => {
                i = 0 as libc::c_int;
                while strlen((helpstr[i as usize]).as_mut_ptr()) > 0 as libc::c_int as u64 {
                    printf(
                        b"%s\n\0" as *const u8 as *const libc::c_char,
                        (helpstr[i as usize]).as_mut_ptr(),
                    );
                    i += 1;
                    i;
                }
                exit(0 as libc::c_int);
            }
            63 | _ => {
                errexit(
                    b"Illegal command-line option(s)\nUse %s -help for a summary of the options.\n\0"
                        as *const u8 as *const libc::c_char as *mut libc::c_char,
                    *argv.offset(0 as libc::c_int as isize),
                );
            }
        }
    }
    if argc - gk_optind != 2 as libc::c_int {
        printf(b"Missing parameters.\0" as *const u8 as *const libc::c_char);
        i = 0 as libc::c_int;
        while strlen((shorthelpstr[i as usize]).as_mut_ptr()) > 0 as libc::c_int as u64 {
            printf(
                b"%s\n\0" as *const u8 as *const libc::c_char,
                (shorthelpstr[i as usize]).as_mut_ptr(),
            );
            i += 1;
            i;
        }
        exit(0 as libc::c_int);
    }
    let fresh0 = gk_optind;
    gk_optind = gk_optind + 1;
    (*params).filename = gk_strdup(*argv.offset(fresh0 as isize));
    let fresh1 = gk_optind;
    gk_optind = gk_optind + 1;
    (*params).nparts = atoi(*argv.offset(fresh1 as isize));
    if (*params).nparts < 2 as libc::c_int {
        errexit(
            b"The number of partitions should be greater than 1!\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
    }
    if (*params).ptype == METIS_PTYPE_RB as libc::c_int {
        (*params).rtype = METIS_RTYPE_FM as libc::c_int;
    }
    if (*params).ptype == METIS_PTYPE_KWAY as libc::c_int {
        (*params).iptype = METIS_IPTYPE_METISRB as libc::c_int;
        (*params).rtype = METIS_RTYPE_GREEDY as libc::c_int;
    }
    if (*params).ptype == METIS_PTYPE_RB as libc::c_int {
        if (*params).contig != 0 {
            errexit(
                b"The -contig option cannot be specified with rb partitioning.\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
        if (*params).minconn != 0 {
            errexit(
                b"The -minconn option cannot be specified with rb partitioning.\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
        if (*params).objtype == METIS_OBJTYPE_VOL as libc::c_int {
            errexit(
                b"The -objtype=vol option cannot be specified with rb partitioning.\n\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
    }
    return params;
}
