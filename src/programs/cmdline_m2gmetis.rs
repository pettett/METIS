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
    fn gk_GetStringID(strmap: *mut gk_StringMap_t, key: *mut libc::c_char) -> libc::c_int;
    fn gk_strdup(orgstr: *mut libc::c_char) -> *mut libc::c_char;
}
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
pub const METIS_GTYPE_NODAL: C2RustUnnamed_0 = 1;
pub const METIS_GTYPE_DUAL: C2RustUnnamed_0 = 0;
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
static mut long_options: [gk_option; 5] = [
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
            name: b"ncommon\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: METIS_OPTION_NCOMMON as libc::c_int,
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
static mut helpstr: [[libc::c_char; 100]; 24] = unsafe {
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
            b"Usage: m2gmetis [options] <meshfile> <graphfile>\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
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
            b"    meshfile    Stores the input mesh.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"    graphfile   The filename of the output graph.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
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
            b"     Specifies the graph that will be generated.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
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
            b"        dual     - Generate dual graph of the mesh [default]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"        nodal    - Generate the nodal graph of the mesh\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
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
            b"  -ncommon=int [applies when gtype=dual]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
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
            b"  -dbglvl=int      \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"     Selects the dbglvl.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
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
            b"   Usage: m2gmetis [options] <meshfile> <graphfile>\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *::core::mem::transmute::<
            &[u8; 100],
            &mut [libc::c_char; 100],
        >(
            b"          use 'm2gmetis -help' for a summary of the options.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
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
    (*params).ncommon = 1 as libc::c_int;
    (*params).dbglvl = 0 as libc::c_int;
    (*params).filename = 0 as *mut libc::c_char;
    (*params).outfile = 0 as *mut libc::c_char;
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
            20 => {
                if !gk_optarg.is_null() {
                    (*params).ncommon = atoi(gk_optarg);
                }
                if (*params).ncommon < 1 as libc::c_int {
                    errexit(
                        b"The -ncommon option should specify a number >= 1.\n\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
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
    (*params).outfile = gk_strdup(*argv.offset(fresh1 as isize));
    return params;
}
