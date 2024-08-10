use ::libc;
extern "C" {
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn exit(_: libc::c_int) -> !;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
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
    fn gk_GetStringID(
        strmap: *mut gk_StringMap_t,
        key: *mut libc::c_char,
    ) -> libc::c_int;
}
pub type __int32_t = libc::c_int;
pub type int32_t = __int32_t;
pub type size_t = libc::c_ulong;
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
pub const METIS_CTYPE_SHEM: C2RustUnnamed_0 = 1;
pub const METIS_CTYPE_RM: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const METIS_IPTYPE_METISRB: C2RustUnnamed_1 = 4;
pub const METIS_IPTYPE_NODE: C2RustUnnamed_1 = 3;
pub const METIS_IPTYPE_EDGE: C2RustUnnamed_1 = 2;
pub const METIS_IPTYPE_RANDOM: C2RustUnnamed_1 = 1;
pub const METIS_IPTYPE_GROW: C2RustUnnamed_1 = 0;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const METIS_RTYPE_SEP1SIDED: C2RustUnnamed_2 = 3;
pub const METIS_RTYPE_SEP2SIDED: C2RustUnnamed_2 = 2;
pub const METIS_RTYPE_GREEDY: C2RustUnnamed_2 = 1;
pub const METIS_RTYPE_FM: C2RustUnnamed_2 = 0;
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
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
static mut long_options: [gk_option; 15] = [
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
            name: b"rtype\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: METIS_OPTION_RTYPE as libc::c_int,
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
            name: b"pfactor\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: METIS_OPTION_PFACTOR as libc::c_int,
        };
        init
    },
    {
        let mut init = gk_option {
            name: b"nocompress\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: METIS_OPTION_COMPRESS as libc::c_int,
        };
        init
    },
    {
        let mut init = gk_option {
            name: b"ccorder\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: METIS_OPTION_CCORDER as libc::c_int,
        };
        init
    },
    {
        let mut init = gk_option {
            name: b"no2hop\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: METIS_OPTION_NO2HOP as libc::c_int,
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
            name: b"niter\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: METIS_OPTION_NITER as libc::c_int,
        };
        init
    },
    {
        let mut init = gk_option {
            name: b"nseps\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: METIS_OPTION_NSEPS as libc::c_int,
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
            name: b"edge\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            id: METIS_IPTYPE_EDGE as libc::c_int,
        };
        init
    },
    {
        let mut init = gk_StringMap_t {
            name: b"node\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            id: METIS_IPTYPE_NODE as libc::c_int,
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
static mut rtype_options: [gk_StringMap_t; 3] = [
    {
        let mut init = gk_StringMap_t {
            name: b"2sided\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            id: METIS_RTYPE_SEP2SIDED as libc::c_int,
        };
        init
    },
    {
        let mut init = gk_StringMap_t {
            name: b"1sided\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            id: METIS_RTYPE_SEP1SIDED as libc::c_int,
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
static mut helpstr: [[libc::c_char; 100]; 77] = unsafe {
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
            b"Usage: ndmetis [options] <filename>\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
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
            b"    filename    Stores the graph to be partitioned.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
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
            b"     Specifies the scheme to be used to compute the initial bisection\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
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
            b"        edge     - Separator from an edge cut\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"        node     - Separator from a greedy node-based strategy [default]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
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
            b"  -rtype=string\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     Specifies the scheme to be used for refinement.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
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
            b"        1sided   - 1-sided node-based refinement [default]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"        2sided   - 2-sided node-based refinement\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
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
            b"     Specifies the maximum allowed load imbalance between the left and\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     right partitions during each bisection. The load imbalanced is\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     measured as the ratio of the 2*max(left,right)/(left+right), where\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     left and right are the sizes of the respective partitions. \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
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
            b"     Default is 200, indicating a load imbalance of 1.20.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
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
            b"  -pfactor=int\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     Specifies the minimum degree of the vertices that will be ordered \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     last. If the specified value is x>0, then any vertices with a degree\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     greater than 0.1*x*(average degree) are removed from the graph, an\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     ordering of the rest of the vertices is computed, and an overall \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     ordering is computed by ordering the removed vertices at the end \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     of the overall ordering.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     Default value is 0, indicating that no vertices are removed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
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
            b"  -no2hop\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     Specifies that the coarsening will not perform any 2-hop matchings\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     when the standard matching fails to sufficiently contract the graph.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
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
            b"  -nocompress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     Specifies that the graph should not be compressed by combining\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     together vertices that have identical adjacency lists.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
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
            b"  -ccorder\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     Specifies if the connected components of the graph should first be \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     identified and ordered separately.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
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
            b"     Specifies the maximum number of iterations for the refinement \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     algorithms at each stage of the uncoarsening process. Default is 10.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
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
            b"  -nseps=int\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     Specifies the number of different separators that it will compute at\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     each level of the nested dissection. The final separator that is used\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     is the smallest one. Default is 1.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
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
            b"     Specifies that no ordering file should be generated.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
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
            b"   Usage: ndmetis [options] <filename>\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"          use 'ndmetis -help' for a summary of the options.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
    ]
};
#[no_mangle]
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
        ::core::mem::size_of::<params_t>() as libc::c_ulong,
        b"parse_cmdline\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut params_t;
    memset(
        params as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<params_t>() as libc::c_ulong,
    );
    (*params).ctype = METIS_CTYPE_SHEM as libc::c_int;
    (*params).iptype = METIS_IPTYPE_NODE as libc::c_int;
    (*params).rtype = METIS_RTYPE_SEP1SIDED as libc::c_int;
    (*params).ufactor = 200 as libc::c_int;
    (*params).pfactor = 0 as libc::c_int;
    (*params).compress = 1 as libc::c_int;
    (*params).ccorder = 0 as libc::c_int;
    (*params).no2hop = 0 as libc::c_int;
    (*params).nooutput = 0 as libc::c_int;
    (*params).wgtflag = 1 as libc::c_int;
    (*params).nseps = 1 as libc::c_int;
    (*params).niter = 10 as libc::c_int;
    (*params).seed = -(1 as libc::c_int);
    (*params).dbglvl = 0 as libc::c_int;
    (*params).filename = 0 as *mut libc::c_char;
    (*params).nparts = 1 as libc::c_int;
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
            2 => {
                if !gk_optarg.is_null() {
                    (*params)
                        .ctype = gk_GetStringID(ctype_options.as_mut_ptr(), gk_optarg);
                    if (*params).ctype == -(1 as libc::c_int) {
                        errexit(
                            b"Invalid option -%s=%s\n\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                            long_options[option_index as usize].name,
                            gk_optarg,
                        );
                    }
                }
            }
            3 => {
                if !gk_optarg.is_null() {
                    (*params)
                        .iptype = gk_GetStringID(iptype_options.as_mut_ptr(), gk_optarg);
                    if (*params).iptype == -(1 as libc::c_int) {
                        errexit(
                            b"Invalid option -%s=%s\n\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                            long_options[option_index as usize].name,
                            gk_optarg,
                        );
                    }
                }
            }
            4 => {
                if !gk_optarg.is_null() {
                    (*params)
                        .rtype = gk_GetStringID(rtype_options.as_mut_ptr(), gk_optarg);
                    if (*params).rtype == -(1 as libc::c_int) {
                        errexit(
                            b"Invalid option -%s=%s\n\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                            long_options[option_index as usize].name,
                            gk_optarg,
                        );
                    }
                }
            }
            16 => {
                if !gk_optarg.is_null() {
                    (*params).ufactor = atoi(gk_optarg);
                }
            }
            14 => {
                if !gk_optarg.is_null() {
                    (*params).pfactor = atoi(gk_optarg);
                }
            }
            12 => {
                (*params).compress = 0 as libc::c_int;
            }
            13 => {
                (*params).ccorder = 1 as libc::c_int;
            }
            9 => {
                (*params).no2hop = 1 as libc::c_int;
            }
            21 => {
                (*params).nooutput = 1 as libc::c_int;
            }
            15 => {
                if !gk_optarg.is_null() {
                    (*params).nseps = atoi(gk_optarg);
                }
            }
            6 => {
                if !gk_optarg.is_null() {
                    (*params).niter = atoi(gk_optarg);
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
                while strlen((helpstr[i as usize]).as_mut_ptr())
                    > 0 as libc::c_int as libc::c_ulong
                {
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
    if argc - gk_optind != 1 as libc::c_int {
        printf(b"Missing parameters.\0" as *const u8 as *const libc::c_char);
        i = 0 as libc::c_int;
        while strlen((shorthelpstr[i as usize]).as_mut_ptr())
            > 0 as libc::c_int as libc::c_ulong
        {
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
    return params;
}
