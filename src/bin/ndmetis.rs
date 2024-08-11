use std::ffi::CStr;

use ::libc;
use libc::{exit, printf};
use metis::{
    libmetis::{
        auxapi::METIS_SetDefaultOptions, gklib::libmetis__imalloc, graph::libmetis__FreeGraph,
        ometis::METIS_NodeND, options::*, util::METIS_OK,
    },
    programs::{
        cmdline_ndmetis::parse_cmdline,
        io::{ReadGraph, WritePermutation},
        smbfactor::ComputeFillIn,
    },
    GKlib::{
        memory::{
            gk_GetCurMemoryUsed, gk_GetMaxMemoryUsed, gk_free, gk_malloc_cleanup, gk_malloc_init,
        },
        timers::gk_CPUSeconds,
    },
};

use metis::libmetis::structure::*;

static mut gtypenames: [&CStr; 2] = unsafe { [c"dual", c"nodal"] };
static mut iptypenames: [&CStr; 5] = unsafe { [c"grow", c"random", c"edge", c"node", c"metisrb"] };
static mut rtypenames: [&CStr; 4] = unsafe { [c"fm", c"greedy", c"2sided", c"1sided"] };
static mut ctypenames: [&CStr; 2] = unsafe { [c"rm", c"shem"] };
static mut objtypenames: [&CStr; 3] = unsafe { [c"cut", c"vol", c"node"] };
static mut ptypenames: [&CStr; 2] = unsafe { [c"rb", c"kway"] };

unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut options: [idx_t; 40] = [0; 40];
    let mut graph: *mut graph_t = 0 as *mut graph_t;
    let mut perm: *mut idx_t = 0 as *mut idx_t;
    let mut iperm: *mut idx_t = 0 as *mut idx_t;
    let mut params: *mut params_t = 0 as *mut params_t;
    let mut status: libc::c_int = 0 as libc::c_int;
    params = parse_cmdline(argc, argv);
    (*params).iotimer = ((*params).iotimer as libc::c_double - gk_CPUSeconds()) as real_t;
    graph = ReadGraph(params);
    (*params).iotimer = ((*params).iotimer as libc::c_double + gk_CPUSeconds()) as real_t;
    if (*graph).ncon != 1 {
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
    (*params).parttimer = ((*params).parttimer as libc::c_double - gk_CPUSeconds()) as real_t;
    status = METIS_NodeND(
        &mut (*graph).nvtxs,
        &mut (*graph).xadj,
        (*graph).adjncy,
        (*graph).vwgt,
        options.as_mut_ptr(),
        perm,
        iperm,
    );
    (*params).parttimer = ((*params).parttimer as libc::c_double + gk_CPUSeconds()) as real_t;
    if gk_GetCurMemoryUsed() != 0 as libc::c_int as u64 {
        printf(
            b"***It seems that Metis did not free all of its memory! Report this.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    (*params).maxmemory = gk_GetMaxMemoryUsed();
    gk_malloc_cleanup(0 as libc::c_int);
    if status != METIS_OK as libc::c_int {
        printf(b"\n***Metis returned with an error.\n\0" as *const u8 as *const libc::c_char);
    } else {
        if (*params).nooutput == 0 {
            (*params).iotimer = ((*params).iotimer as libc::c_double - gk_CPUSeconds()) as real_t;
            WritePermutation((*params).filename, iperm, (*graph).nvtxs);
            (*params).iotimer = ((*params).iotimer as libc::c_double + gk_CPUSeconds()) as real_t;
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
pub unsafe extern "C" fn NDPrintInfo(mut params: *mut params_t, mut graph: *mut graph_t) {
    printf(
        b"******************************************************************************\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"%s\0" as *const u8 as *const libc::c_char,
        b"METIS 5.0 Copyright 1998-13, Regents of the University of Minnesota\n\0" as *const u8
            as *const libc::c_char,
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
        (8 as libc::c_int as u64).wrapping_mul(::core::mem::size_of::<idx_t>() as u64),
        (8 as libc::c_int as u64).wrapping_mul(::core::mem::size_of::<real_t>() as u64),
        (8 as libc::c_int as u64).wrapping_mul(::core::mem::size_of::<*mut idx_t>() as u64),
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
        (ctypenames[(*params).ctype as usize]).as_ptr(),
        (rtypenames[(*params).rtype as usize]).as_ptr(),
        (iptypenames[(*params).iptype as usize]).as_ptr(),
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
    (*params).reporttimer = ((*params).reporttimer as libc::c_double - gk_CPUSeconds()) as real_t;
    ComputeFillIn(graph, perm, iperm, &mut maxlnz, &mut opc);
    printf(
        b"  Nonzeros: %6.3le \tOperation Count: %6.3le\n\0" as *const u8 as *const libc::c_char,
        maxlnz as libc::c_double,
        opc as libc::c_double,
    );
    (*params).reporttimer = ((*params).reporttimer as libc::c_double + gk_CPUSeconds()) as real_t;
    printf(
        b"\nTiming Information ----------------------------------------------------------\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"  I/O:          \t\t %7.3f sec\n\0" as *const u8 as *const libc::c_char,
        (*params).iotimer as libc::c_double,
    );
    printf(
        b"  Ordering:     \t\t %7.3f sec   (METIS time)\n\0" as *const u8 as *const libc::c_char,
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
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as libc::c_int,
            args.as_mut_ptr() as *mut *mut libc::c_char,
        ) as i32)
    }
}
