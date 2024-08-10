use ::libc;
extern "C" {
    fn exit(_: libc::c_int) -> !;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn gk_malloc_init() -> libc::c_int;
    fn gk_malloc_cleanup(showstats: libc::c_int);
    fn gk_free(ptr1: *mut *mut libc::c_void, _: ...);
    fn gk_GetCurMemoryUsed() -> size_t;
    fn gk_GetMaxMemoryUsed() -> size_t;
    fn gk_CPUSeconds() -> libc::c_double;
    fn METIS_PartMeshNodal(
        ne: *mut idx_t,
        nn: *mut idx_t,
        eptr: *mut idx_t,
        eind: *mut idx_t,
        vwgt: *mut idx_t,
        vsize: *mut idx_t,
        nparts: *mut idx_t,
        tpwgts: *mut real_t,
        options: *mut idx_t,
        objval: *mut idx_t,
        epart: *mut idx_t,
        npart: *mut idx_t,
    ) -> libc::c_int;
    fn METIS_PartMeshDual(
        ne: *mut idx_t,
        nn: *mut idx_t,
        eptr: *mut idx_t,
        eind: *mut idx_t,
        vwgt: *mut idx_t,
        vsize: *mut idx_t,
        ncommon: *mut idx_t,
        nparts: *mut idx_t,
        tpwgts: *mut real_t,
        options: *mut idx_t,
        objval: *mut idx_t,
        epart: *mut idx_t,
        npart: *mut idx_t,
    ) -> libc::c_int;
    fn METIS_SetDefaultOptions(options: *mut idx_t) -> libc::c_int;
    fn parse_cmdline(argc: libc::c_int, argv: *mut *mut libc::c_char) -> *mut params_t;
    fn WriteMeshPartition(
        _: *mut libc::c_char,
        _: idx_t,
        _: idx_t,
        _: *mut idx_t,
        _: idx_t,
        _: *mut idx_t,
    );
    fn ReadTPwgts(params: *mut params_t, ncon: idx_t);
    fn ReadMesh(_: *mut params_t) -> *mut mesh_t;
    fn libmetis__FreeMesh(mesh: *mut *mut mesh_t);
    fn libmetis__imalloc(n: size_t, msg: *mut libc::c_char) -> *mut idx_t;
}
pub type __int32_t = libc::c_int;
pub type int32_t = __int32_t;
pub type size_t = libc::c_ulong;
pub type idx_t = int32_t;
pub type real_t = libc::c_float;
pub type C2RustUnnamed = libc::c_int;
pub const METIS_ERROR: C2RustUnnamed = -4;
pub const METIS_ERROR_MEMORY: C2RustUnnamed = -3;
pub const METIS_ERROR_INPUT: C2RustUnnamed = -2;
pub const METIS_OK: C2RustUnnamed = 1;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const METIS_OPTION_UBVEC: C2RustUnnamed_0 = 24;
pub const METIS_OPTION_GTYPE: C2RustUnnamed_0 = 23;
pub const METIS_OPTION_BALANCE: C2RustUnnamed_0 = 22;
pub const METIS_OPTION_NOOUTPUT: C2RustUnnamed_0 = 21;
pub const METIS_OPTION_NCOMMON: C2RustUnnamed_0 = 20;
pub const METIS_OPTION_TPWGTS: C2RustUnnamed_0 = 19;
pub const METIS_OPTION_HELP: C2RustUnnamed_0 = 18;
pub const METIS_OPTION_NUMBERING: C2RustUnnamed_0 = 17;
pub const METIS_OPTION_UFACTOR: C2RustUnnamed_0 = 16;
pub const METIS_OPTION_NSEPS: C2RustUnnamed_0 = 15;
pub const METIS_OPTION_PFACTOR: C2RustUnnamed_0 = 14;
pub const METIS_OPTION_CCORDER: C2RustUnnamed_0 = 13;
pub const METIS_OPTION_COMPRESS: C2RustUnnamed_0 = 12;
pub const METIS_OPTION_CONTIG: C2RustUnnamed_0 = 11;
pub const METIS_OPTION_MINCONN: C2RustUnnamed_0 = 10;
pub const METIS_OPTION_NO2HOP: C2RustUnnamed_0 = 9;
pub const METIS_OPTION_SEED: C2RustUnnamed_0 = 8;
pub const METIS_OPTION_NCUTS: C2RustUnnamed_0 = 7;
pub const METIS_OPTION_NITER: C2RustUnnamed_0 = 6;
pub const METIS_OPTION_DBGLVL: C2RustUnnamed_0 = 5;
pub const METIS_OPTION_RTYPE: C2RustUnnamed_0 = 4;
pub const METIS_OPTION_IPTYPE: C2RustUnnamed_0 = 3;
pub const METIS_OPTION_CTYPE: C2RustUnnamed_0 = 2;
pub const METIS_OPTION_OBJTYPE: C2RustUnnamed_0 = 1;
pub const METIS_OPTION_PTYPE: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const METIS_PTYPE_KWAY: C2RustUnnamed_1 = 1;
pub const METIS_PTYPE_RB: C2RustUnnamed_1 = 0;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const METIS_GTYPE_NODAL: C2RustUnnamed_2 = 1;
pub const METIS_GTYPE_DUAL: C2RustUnnamed_2 = 0;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const METIS_OBJTYPE_NODE: C2RustUnnamed_3 = 2;
pub const METIS_OBJTYPE_VOL: C2RustUnnamed_3 = 1;
pub const METIS_OBJTYPE_CUT: C2RustUnnamed_3 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mesh_t {
    pub ne: idx_t,
    pub nn: idx_t,
    pub ncon: idx_t,
    pub eptr: *mut idx_t,
    pub eind: *mut idx_t,
    pub ewgt: *mut idx_t,
}
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
static mut gtypenames: [[libc::c_char; 15]; 2] = unsafe {
    [
        *::core::mem::transmute::<
            &[u8; 15],
            &mut [libc::c_char; 15],
        >(b"dual\0\0\0\0\0\0\0\0\0\0\0"),
        *::core::mem::transmute::<
            &[u8; 15],
            &mut [libc::c_char; 15],
        >(b"nodal\0\0\0\0\0\0\0\0\0\0"),
    ]
};
static mut iptypenames: [[libc::c_char; 15]; 5] = unsafe {
    [
        *::core::mem::transmute::<
            &[u8; 15],
            &mut [libc::c_char; 15],
        >(b"grow\0\0\0\0\0\0\0\0\0\0\0"),
        *::core::mem::transmute::<
            &[u8; 15],
            &mut [libc::c_char; 15],
        >(b"random\0\0\0\0\0\0\0\0\0"),
        *::core::mem::transmute::<
            &[u8; 15],
            &mut [libc::c_char; 15],
        >(b"edge\0\0\0\0\0\0\0\0\0\0\0"),
        *::core::mem::transmute::<
            &[u8; 15],
            &mut [libc::c_char; 15],
        >(b"node\0\0\0\0\0\0\0\0\0\0\0"),
        *::core::mem::transmute::<
            &[u8; 15],
            &mut [libc::c_char; 15],
        >(b"metisrb\0\0\0\0\0\0\0\0"),
    ]
};
static mut rtypenames: [[libc::c_char; 15]; 4] = unsafe {
    [
        *::core::mem::transmute::<
            &[u8; 15],
            &mut [libc::c_char; 15],
        >(b"fm\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::core::mem::transmute::<
            &[u8; 15],
            &mut [libc::c_char; 15],
        >(b"greedy\0\0\0\0\0\0\0\0\0"),
        *::core::mem::transmute::<
            &[u8; 15],
            &mut [libc::c_char; 15],
        >(b"2sided\0\0\0\0\0\0\0\0\0"),
        *::core::mem::transmute::<
            &[u8; 15],
            &mut [libc::c_char; 15],
        >(b"1sided\0\0\0\0\0\0\0\0\0"),
    ]
};
static mut ctypenames: [[libc::c_char; 15]; 2] = unsafe {
    [
        *::core::mem::transmute::<
            &[u8; 15],
            &mut [libc::c_char; 15],
        >(b"rm\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::core::mem::transmute::<
            &[u8; 15],
            &mut [libc::c_char; 15],
        >(b"shem\0\0\0\0\0\0\0\0\0\0\0"),
    ]
};
static mut objtypenames: [[libc::c_char; 15]; 3] = unsafe {
    [
        *::core::mem::transmute::<
            &[u8; 15],
            &mut [libc::c_char; 15],
        >(b"cut\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::core::mem::transmute::<
            &[u8; 15],
            &mut [libc::c_char; 15],
        >(b"vol\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::core::mem::transmute::<
            &[u8; 15],
            &mut [libc::c_char; 15],
        >(b"node\0\0\0\0\0\0\0\0\0\0\0"),
    ]
};
static mut ptypenames: [[libc::c_char; 15]; 2] = unsafe {
    [
        *::core::mem::transmute::<
            &[u8; 15],
            &mut [libc::c_char; 15],
        >(b"rb\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::core::mem::transmute::<
            &[u8; 15],
            &mut [libc::c_char; 15],
        >(b"kway\0\0\0\0\0\0\0\0\0\0\0"),
    ]
};
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut options: [idx_t; 40] = [0; 40];
    let mut mesh: *mut mesh_t = 0 as *mut mesh_t;
    let mut epart: *mut idx_t = 0 as *mut idx_t;
    let mut npart: *mut idx_t = 0 as *mut idx_t;
    let mut objval: idx_t = 0;
    let mut params: *mut params_t = 0 as *mut params_t;
    let mut status: libc::c_int = 0 as libc::c_int;
    params = parse_cmdline(argc, argv);
    (*params)
        .iotimer = ((*params).iotimer as libc::c_double - gk_CPUSeconds()) as real_t;
    mesh = ReadMesh(params);
    if (*mesh).ncon > 1 as libc::c_int {
        printf(
            b"*** Meshes with more than one balancing constraint are not supported yet.\n\0"
                as *const u8 as *const libc::c_char,
        );
        exit(0 as libc::c_int);
    }
    ReadTPwgts(params, (*mesh).ncon);
    (*params)
        .iotimer = ((*params).iotimer as libc::c_double + gk_CPUSeconds()) as real_t;
    MPPrintInfo(params, mesh);
    epart = libmetis__imalloc(
        (*mesh).ne as size_t,
        b"main: epart\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    npart = libmetis__imalloc(
        (*mesh).nn as size_t,
        b"main: npart\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    METIS_SetDefaultOptions(options.as_mut_ptr());
    options[METIS_OPTION_PTYPE as libc::c_int as usize] = (*params).ptype;
    options[METIS_OPTION_OBJTYPE as libc::c_int as usize] = (*params).objtype;
    options[METIS_OPTION_CTYPE as libc::c_int as usize] = (*params).ctype;
    options[METIS_OPTION_IPTYPE as libc::c_int as usize] = (*params).iptype;
    options[METIS_OPTION_RTYPE as libc::c_int as usize] = (*params).rtype;
    options[METIS_OPTION_DBGLVL as libc::c_int as usize] = (*params).dbglvl;
    options[METIS_OPTION_UFACTOR as libc::c_int as usize] = (*params).ufactor;
    options[METIS_OPTION_MINCONN as libc::c_int as usize] = (*params).minconn;
    options[METIS_OPTION_CONTIG as libc::c_int as usize] = (*params).contig;
    options[METIS_OPTION_SEED as libc::c_int as usize] = (*params).seed;
    options[METIS_OPTION_NITER as libc::c_int as usize] = (*params).niter;
    options[METIS_OPTION_NCUTS as libc::c_int as usize] = (*params).ncuts;
    gk_malloc_init();
    (*params)
        .parttimer = ((*params).parttimer as libc::c_double - gk_CPUSeconds()) as real_t;
    match (*params).gtype {
        0 => {
            status = METIS_PartMeshDual(
                &mut (*mesh).ne,
                &mut (*mesh).nn,
                (*mesh).eptr,
                (*mesh).eind,
                (*mesh).ewgt,
                0 as *mut idx_t,
                &mut (*params).ncommon,
                &mut (*params).nparts,
                (*params).tpwgts,
                options.as_mut_ptr(),
                &mut objval,
                epart,
                npart,
            );
        }
        1 => {
            status = METIS_PartMeshNodal(
                &mut (*mesh).ne,
                &mut (*mesh).nn,
                (*mesh).eptr,
                (*mesh).eind,
                0 as *mut idx_t,
                0 as *mut idx_t,
                &mut (*params).nparts,
                (*params).tpwgts,
                options.as_mut_ptr(),
                &mut objval,
                epart,
                npart,
            );
        }
        _ => {}
    }
    (*params)
        .parttimer = ((*params).parttimer as libc::c_double + gk_CPUSeconds()) as real_t;
    if gk_GetCurMemoryUsed() != 0 as libc::c_int as libc::c_ulong {
        printf(
            b"***It seems that Metis did not free all of its memory! Report this.\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    (*params).maxmemory = gk_GetMaxMemoryUsed();
    gk_malloc_cleanup(0 as libc::c_int);
    if status != METIS_OK as libc::c_int {
        printf(
            b"\n***Metis returned with an error.\n\0" as *const u8 as *const libc::c_char,
        );
    } else {
        if (*params).nooutput == 0 {
            (*params)
                .iotimer = ((*params).iotimer as libc::c_double - gk_CPUSeconds())
                as real_t;
            WriteMeshPartition(
                (*params).filename,
                (*params).nparts,
                (*mesh).ne,
                epart,
                (*mesh).nn,
                npart,
            );
            (*params)
                .iotimer = ((*params).iotimer as libc::c_double + gk_CPUSeconds())
                as real_t;
        }
        MPReportResults(params, mesh, epart, npart, objval);
    }
    libmetis__FreeMesh(&mut mesh);
    gk_free(
        &mut epart as *mut *mut idx_t as *mut *mut libc::c_void,
        &mut npart as *mut *mut idx_t,
        0 as *mut *mut libc::c_void,
    );
    gk_free(
        &mut (*params).filename as *mut *mut libc::c_char as *mut *mut libc::c_void,
        &mut (*params).tpwgtsfile as *mut *mut libc::c_char,
        &mut (*params).tpwgts as *mut *mut real_t,
        &mut (*params).ubvecstr as *mut *mut libc::c_char,
        &mut (*params).ubvec as *mut *mut real_t,
        &mut params as *mut *mut params_t,
        0 as *mut *mut libc::c_void,
    );
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn MPPrintInfo(mut params: *mut params_t, mut mesh: *mut mesh_t) {
    if (*params).ufactor == -(1 as libc::c_int) {
        if (*params).ptype == METIS_PTYPE_KWAY as libc::c_int {
            (*params).ufactor = 30 as libc::c_int;
        } else {
            (*params).ufactor = 1 as libc::c_int;
        }
    }
    printf(
        b"******************************************************************************\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"%s\0" as *const u8 as *const libc::c_char,
        b"METIS 5.0 Copyright 1998-13, Regents of the University of Minnesota\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b" (HEAD: %s, Built on: %s, %s)\n\0" as *const u8 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
        b"Aug 10 2024\0" as *const u8 as *const libc::c_char,
        b"16:17:17\0" as *const u8 as *const libc::c_char,
    );
    printf(
        b" size of idx_t: %zubits, real_t: %zubits, idx_t *: %zubits\n\0" as *const u8
            as *const libc::c_char,
        (8 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<idx_t>() as libc::c_ulong),
        (8 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<real_t>() as libc::c_ulong),
        (8 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut idx_t>() as libc::c_ulong),
    );
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"Mesh Information ------------------------------------------------------------\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b" Name: %s, #Elements: %d, #Nodes: %d, #Parts: %d\n\0" as *const u8
            as *const libc::c_char,
        (*params).filename,
        (*mesh).ne,
        (*mesh).nn,
        (*params).nparts,
    );
    if (*mesh).ncon > 1 as libc::c_int {
        printf(
            b"  Balancing Constraints: %d\n\0" as *const u8 as *const libc::c_char,
            (*mesh).ncon,
        );
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"Options ---------------------------------------------------------------------\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b" ptype=%s, objtype=%s, ctype=%s, rtype=%s, iptype=%s\n\0" as *const u8
            as *const libc::c_char,
        (ptypenames[(*params).ptype as usize]).as_mut_ptr(),
        (objtypenames[(*params).objtype as usize]).as_mut_ptr(),
        (ctypenames[(*params).ctype as usize]).as_mut_ptr(),
        (rtypenames[(*params).rtype as usize]).as_mut_ptr(),
        (iptypenames[(*params).iptype as usize]).as_mut_ptr(),
    );
    printf(
        b" dbglvl=%d, ufactor=%.3f, minconn=%s, contig=%s, nooutput=%s\n\0" as *const u8
            as *const libc::c_char,
        (*params).dbglvl,
        1.0f64 + 0.001f64 * (*params).ufactor as libc::c_double,
        if (*params).minconn != 0 {
            b"YES\0" as *const u8 as *const libc::c_char
        } else {
            b"NO\0" as *const u8 as *const libc::c_char
        },
        if (*params).contig != 0 {
            b"YES\0" as *const u8 as *const libc::c_char
        } else {
            b"NO\0" as *const u8 as *const libc::c_char
        },
        if (*params).nooutput != 0 {
            b"YES\0" as *const u8 as *const libc::c_char
        } else {
            b"NO\0" as *const u8 as *const libc::c_char
        },
    );
    printf(
        b" seed=%d, niter=%d, ncuts=%d\n\0" as *const u8 as *const libc::c_char,
        (*params).seed,
        (*params).niter,
        (*params).ncuts,
    );
    printf(
        b" gtype=%s, ncommon=%d, niter=%d, ncuts=%d\n\0" as *const u8
            as *const libc::c_char,
        (gtypenames[(*params).gtype as usize]).as_mut_ptr(),
        (*params).ncommon,
        (*params).niter,
        (*params).ncuts,
    );
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    match (*params).ptype {
        0 => {
            printf(
                b"Recursive Partitioning ------------------------------------------------------\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        1 => {
            printf(
                b"Direct k-way Partitioning ---------------------------------------------------\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn MPReportResults(
    mut params: *mut params_t,
    mut mesh: *mut mesh_t,
    mut epart: *mut idx_t,
    mut npart: *mut idx_t,
    mut objval: idx_t,
) {
    (*params)
        .reporttimer = ((*params).reporttimer as libc::c_double - gk_CPUSeconds())
        as real_t;
    printf(
        b" - %s: %d.\n\n\0" as *const u8 as *const libc::c_char,
        if (*params).objtype == METIS_OBJTYPE_CUT as libc::c_int {
            b"Edgecut\0" as *const u8 as *const libc::c_char
        } else {
            b"Volume\0" as *const u8 as *const libc::c_char
        },
        objval,
    );
    (*params)
        .reporttimer = ((*params).reporttimer as libc::c_double + gk_CPUSeconds())
        as real_t;
    printf(
        b"\nTiming Information ----------------------------------------------------------\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"  I/O:          \t\t %7.3f sec\n\0" as *const u8 as *const libc::c_char,
        (*params).iotimer as libc::c_double,
    );
    printf(
        b"  Partitioning: \t\t %7.3f sec   (METIS time)\n\0" as *const u8
            as *const libc::c_char,
        (*params).parttimer as libc::c_double,
    );
    printf(
        b"  Reporting:    \t\t %7.3f sec\n\0" as *const u8 as *const libc::c_char,
        (*params).reporttimer as libc::c_double,
    );
    printf(
        b"\nMemory Information ----------------------------------------------------------\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"  Max memory used:\t\t %7.3f MB\n\0" as *const u8 as *const libc::c_char,
        ((*params).maxmemory as libc::c_double / (1024.0f64 * 1024.0f64)) as real_t
            as libc::c_double,
    );
    printf(
        b"******************************************************************************\n\0"
            as *const u8 as *const libc::c_char,
    );
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
