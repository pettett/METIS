use ::libc;
extern "C" {
    fn exit(_: libc::c_int) -> !;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn parse_cmdline(argc: libc::c_int, argv: *mut *mut libc::c_char) -> *mut params_t;
    fn ComputeFillIn(
        graph: *mut graph_t,
        perm: *mut idx_t,
        iperm: *mut idx_t,
        r_maxlnz: *mut size_t,
        r_opc: *mut size_t,
    );
    fn WritePermutation(_: *mut libc::c_char, _: *mut idx_t, _: idx_t);
    fn ReadGraph(_: *mut params_t) -> *mut graph_t;
    fn gk_malloc_init() -> libc::c_int;
    fn gk_malloc_cleanup(showstats: libc::c_int);
    fn gk_free(ptr1: *mut *mut libc::c_void, _: ...);
    fn gk_GetCurMemoryUsed() -> size_t;
    fn gk_GetMaxMemoryUsed() -> size_t;
    fn gk_CPUSeconds() -> libc::c_double;
    fn METIS_NodeND(
        nvtxs: *mut idx_t,
        xadj: *mut idx_t,
        adjncy: *mut idx_t,
        vwgt: *mut idx_t,
        options: *mut idx_t,
        perm: *mut idx_t,
        iperm: *mut idx_t,
    ) -> libc::c_int;
    fn METIS_SetDefaultOptions(options: *mut idx_t) -> libc::c_int;
    fn libmetis__imalloc(n: size_t, msg: *mut libc::c_char) -> *mut idx_t;
    fn libmetis__FreeGraph(graph: *mut *mut graph_t);
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
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut options: [idx_t; 40] = [0; 40];
    let mut graph: *mut graph_t = 0 as *mut graph_t;
    let mut perm: *mut idx_t = 0 as *mut idx_t;
    let mut iperm: *mut idx_t = 0 as *mut idx_t;
    let mut params: *mut params_t = 0 as *mut params_t;
    let mut status: libc::c_int = 0 as libc::c_int;
    params = parse_cmdline(argc, argv);
    (*params)
        .iotimer = ((*params).iotimer as libc::c_double - gk_CPUSeconds()) as real_t;
    graph = ReadGraph(params);
    (*params)
        .iotimer = ((*params).iotimer as libc::c_double + gk_CPUSeconds()) as real_t;
    if (*graph).ncon != 1 as libc::c_int {
        printf(
            b"***The input graph contains %d constraints..\n***Ordering requires a graph with one constraint.\n\0"
                as *const u8 as *const libc::c_char,
            (*graph).ncon,
        );
        exit(0 as libc::c_int);
    }
    NDPrintInfo(params, graph);
    perm = libmetis__imalloc(
        (*graph).nvtxs as size_t,
        b"main: perm\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    iperm = libmetis__imalloc(
        (*graph).nvtxs as size_t,
        b"main: iperm\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    METIS_SetDefaultOptions(options.as_mut_ptr());
    options[METIS_OPTION_CTYPE as libc::c_int as usize] = (*params).ctype;
    options[METIS_OPTION_IPTYPE as libc::c_int as usize] = (*params).iptype;
    options[METIS_OPTION_RTYPE as libc::c_int as usize] = (*params).rtype;
    options[METIS_OPTION_DBGLVL as libc::c_int as usize] = (*params).dbglvl;
    options[METIS_OPTION_UFACTOR as libc::c_int as usize] = (*params).ufactor;
    options[METIS_OPTION_NO2HOP as libc::c_int as usize] = (*params).no2hop;
    options[METIS_OPTION_COMPRESS as libc::c_int as usize] = (*params).compress;
    options[METIS_OPTION_CCORDER as libc::c_int as usize] = (*params).ccorder;
    options[METIS_OPTION_SEED as libc::c_int as usize] = (*params).seed;
    options[METIS_OPTION_NITER as libc::c_int as usize] = (*params).niter;
    options[METIS_OPTION_NSEPS as libc::c_int as usize] = (*params).nseps;
    options[METIS_OPTION_PFACTOR as libc::c_int as usize] = (*params).pfactor;
    gk_malloc_init();
    (*params)
        .parttimer = ((*params).parttimer as libc::c_double - gk_CPUSeconds()) as real_t;
    status = METIS_NodeND(
        &mut (*graph).nvtxs,
        (*graph).xadj,
        (*graph).adjncy,
        (*graph).vwgt,
        options.as_mut_ptr(),
        perm,
        iperm,
    );
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
            WritePermutation((*params).filename, iperm, (*graph).nvtxs);
            (*params)
                .iotimer = ((*params).iotimer as libc::c_double + gk_CPUSeconds())
                as real_t;
        }
        NDReportResults(params, graph, perm, iperm);
    }
    libmetis__FreeGraph(&mut graph);
    gk_free(
        &mut perm as *mut *mut idx_t as *mut *mut libc::c_void,
        &mut iperm as *mut *mut idx_t,
        0 as *mut *mut libc::c_void,
    );
    gk_free(
        &mut (*params).filename as *mut *mut libc::c_char as *mut *mut libc::c_void,
        &mut (*params).tpwgtsfile as *mut *mut libc::c_char,
        &mut (*params).tpwgts as *mut *mut real_t,
        &mut (*params).ubvec as *mut *mut real_t,
        &mut params as *mut *mut params_t,
        0 as *mut *mut libc::c_void,
    );
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn NDPrintInfo(
    mut params: *mut params_t,
    mut graph: *mut graph_t,
) {
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
        b"16:17:16\0" as *const u8 as *const libc::c_char,
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
        b"Graph Information -----------------------------------------------------------\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b" Name: %s, #Vertices: %d, #Edges: %d\n\0" as *const u8 as *const libc::c_char,
        (*params).filename,
        (*graph).nvtxs,
        (*graph).nedges / 2 as libc::c_int,
    );
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"Options ---------------------------------------------------------------------\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b" ctype=%s, rtype=%s, iptype=%s, seed=%d, dbglvl=%d\n\0" as *const u8
            as *const libc::c_char,
        (ctypenames[(*params).ctype as usize]).as_mut_ptr(),
        (rtypenames[(*params).rtype as usize]).as_mut_ptr(),
        (iptypenames[(*params).iptype as usize]).as_mut_ptr(),
        (*params).seed,
        (*params).dbglvl,
    );
    printf(
        b" ufactor=%.3f, pfactor=%.2f, no2hop=%s, ccorder=%s, compress=%s, , nooutput=%s\n\0"
            as *const u8 as *const libc::c_char,
        1.0f64 + 0.001f64 * (*params).ufactor as libc::c_double,
        0.1f64 * (*params).pfactor as libc::c_double,
        if (*params).no2hop != 0 {
            b"YES\0" as *const u8 as *const libc::c_char
        } else {
            b"NO\0" as *const u8 as *const libc::c_char
        },
        if (*params).ccorder != 0 {
            b"YES\0" as *const u8 as *const libc::c_char
        } else {
            b"NO\0" as *const u8 as *const libc::c_char
        },
        if (*params).compress != 0 {
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
        b" niter=%d, nseps=%d\n\0" as *const u8 as *const libc::c_char,
        (*params).niter,
        (*params).nseps,
    );
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"Node-based Nested Dissection ------------------------------------------------\n\0"
            as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn NDReportResults(
    mut params: *mut params_t,
    mut graph: *mut graph_t,
    mut perm: *mut idx_t,
    mut iperm: *mut idx_t,
) {
    let mut maxlnz: size_t = 0;
    let mut opc: size_t = 0;
    (*params)
        .reporttimer = ((*params).reporttimer as libc::c_double - gk_CPUSeconds())
        as real_t;
    ComputeFillIn(graph, perm, iperm, &mut maxlnz, &mut opc);
    printf(
        b"  Nonzeros: %6.3le \tOperation Count: %6.3le\n\0" as *const u8
            as *const libc::c_char,
        maxlnz as libc::c_double,
        opc as libc::c_double,
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
        b"  Ordering:     \t\t %7.3f sec   (METIS time)\n\0" as *const u8
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
