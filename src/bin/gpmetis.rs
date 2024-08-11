use std::ffi::CStr;

use ::libc;

use libc::{printf, strtof};
use metis::{
    libmetis::{
        auxapi::METIS_SetDefaultOptions,
        contig::libmetis__IsConnected,
        gklib::{libmetis__imalloc, libmetis__rmalloc},
        graph::libmetis__FreeGraph,
        kmetis::METIS_PartGraphKway,
        meshpart::{METIS_PTYPE_KWAY, METIS_PTYPE_RB},
        options::*,
        pmetis::METIS_PartGraphRecursive,
        structure::*,
        util::METIS_OK,
    },
    programs::{cmdline_gpmetis::parse_cmdline, io::*, stat::ComputePartitionInfo},
    GKlib::{
        error::errexit,
        memory::{
            gk_GetCurMemoryUsed, gk_GetMaxMemoryUsed, gk_free, gk_malloc_cleanup, gk_malloc_init,
        },
        timers::gk_CPUSeconds,
    },
};

static mut gtypenames: [&CStr; 2] = unsafe { [c"dual", c"nodal"] };
static mut iptypenames: [&CStr; 5] = unsafe { [c"grow", c"random", c"edge", c"node", c"metisrb"] };
static mut rtypenames: [&CStr; 4] = unsafe { [c"fm", c"greedy", c"2sided", c"1sided"] };
static mut ctypenames: [&CStr; 2] = unsafe { [c"rm", c"shem"] };
static mut objtypenames: [&CStr; 3] = unsafe { [c"cut", c"vol", c"node"] };
static mut ptypenames: [&CStr; 2] = unsafe { [c"rb", c"kway"] };

unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut i: idx_t = 0;
    let mut curptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut newptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut options: [idx_t; 40] = [0; 40];
    let mut graph: *mut graph_t = 0 as *mut graph_t;
    let mut part: *mut idx_t = 0 as *mut idx_t;
    let mut objval: idx_t = 0;
    let mut params: *mut params_t = 0 as *mut params_t;
    let mut status: libc::c_int = 0 as libc::c_int;
    params = parse_cmdline(argc, argv);
    (*params).iotimer = ((*params).iotimer as libc::c_double - gk_CPUSeconds()) as real_t;
    graph = ReadGraph(params);
    ReadTPwgts(params, (*graph).ncon);
    (*params).iotimer = ((*params).iotimer as libc::c_double + gk_CPUSeconds()) as real_t;
    if (*params).contig != 0 && libmetis__IsConnected(graph, 0 as libc::c_int) == 0 {
        printf(
            b"***The input graph is not contiguous.\n***The specified -contig option will be ignored.\n\0"
                as *const u8 as *const libc::c_char,
        );
        (*params).contig = 0 as libc::c_int;
    }
    if !((*params).ubvecstr).is_null() {
        (*params).ubvec = libmetis__rmalloc(
            (*graph).ncon as size_t,
            b"main\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        curptr = (*params).ubvecstr;
        i = 0 as libc::c_int;
        while i < (*graph).ncon {
            *((*params).ubvec).offset(i as isize) = strtof(curptr, &mut newptr);
            if curptr == newptr {
                errexit(
                    b"Error parsing entry #%d of ubvec [%s] (possibly missing).\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    i,
                    (*params).ubvecstr,
                );
            }
            curptr = newptr;
            i += 1;
            i;
        }
    }
    if (*params).iptype == -(1 as libc::c_int) {
        if (*params).ptype == METIS_PTYPE_RB as libc::c_int {
            if (*graph).ncon == 1 as libc::c_int {
                (*params).iptype = METIS_IPTYPE_GROW as libc::c_int;
            } else {
                (*params).iptype = METIS_IPTYPE_RANDOM as libc::c_int;
            }
        }
    }
    GPPrintInfo(params, graph);
    part = libmetis__imalloc(
        (*graph).nvtxs as size_t,
        b"main: part\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    METIS_SetDefaultOptions(options.as_mut_ptr());
    options[METIS_OPTION_OBJTYPE as libc::c_int as usize] = (*params).objtype;
    options[METIS_OPTION_CTYPE as libc::c_int as usize] = (*params).ctype;
    options[METIS_OPTION_IPTYPE as libc::c_int as usize] = (*params).iptype;
    options[METIS_OPTION_RTYPE as libc::c_int as usize] = (*params).rtype;
    options[METIS_OPTION_NO2HOP as libc::c_int as usize] = (*params).no2hop;
    options[METIS_OPTION_MINCONN as libc::c_int as usize] = (*params).minconn;
    options[METIS_OPTION_CONTIG as libc::c_int as usize] = (*params).contig;
    options[METIS_OPTION_SEED as libc::c_int as usize] = (*params).seed;
    options[METIS_OPTION_NITER as libc::c_int as usize] = (*params).niter;
    options[METIS_OPTION_NCUTS as libc::c_int as usize] = (*params).ncuts;
    options[METIS_OPTION_UFACTOR as libc::c_int as usize] = (*params).ufactor;
    options[METIS_OPTION_DBGLVL as libc::c_int as usize] = (*params).dbglvl;
    gk_malloc_init();
    (*params).parttimer = ((*params).parttimer as libc::c_double - gk_CPUSeconds()) as real_t;
    match (*params).ptype {
        0 => {
            status = METIS_PartGraphRecursive(
                &mut (*graph).nvtxs,
                &mut (*graph).ncon,
                (*graph).xadj,
                (*graph).adjncy,
                (*graph).vwgt,
                (*graph).vsize,
                (*graph).adjwgt,
                &mut (*params).nparts,
                (*params).tpwgts,
                (*params).ubvec,
                options.as_mut_ptr(),
                &mut objval,
                part,
            );
        }
        1 => {
            status = METIS_PartGraphKway(
                &mut (*graph).nvtxs,
                &mut (*graph).ncon,
                (*graph).xadj,
                (*graph).adjncy,
                (*graph).vwgt,
                (*graph).vsize,
                (*graph).adjwgt,
                &mut (*params).nparts,
                (*params).tpwgts,
                (*params).ubvec,
                options.as_mut_ptr(),
                &mut objval,
                part,
            );
        }
        _ => {}
    }
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
            WritePartition((*params).filename, part, (*graph).nvtxs, (*params).nparts);
            (*params).iotimer = ((*params).iotimer as libc::c_double + gk_CPUSeconds()) as real_t;
        }
        GPReportResults(params, graph, part, objval);
    }
    libmetis__FreeGraph(&mut graph);
    gk_free(
        &mut part as *mut *mut idx_t as *mut *mut libc::c_void,
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
pub unsafe extern "C" fn GPPrintInfo(mut params: *mut params_t, mut graph: *mut graph_t) {
    let mut i: idx_t = 0;
    if (*params).ufactor == -(1 as libc::c_int) {
        if (*params).ptype == METIS_PTYPE_KWAY as libc::c_int {
            (*params).ufactor = 30 as libc::c_int;
        } else if (*graph).ncon == 1 as libc::c_int {
            (*params).ufactor = 1 as libc::c_int;
        } else {
            (*params).ufactor = 10 as libc::c_int;
        }
    }
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
        b"16:17:13\0" as *const u8 as *const libc::c_char,
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
        b" Name: %s, #Vertices: %d, #Edges: %d, #Parts: %d\n\0" as *const u8 as *const libc::c_char,
        (*params).filename,
        (*graph).nvtxs,
        (*graph).nedges / 2 as libc::c_int,
        (*params).nparts,
    );
    if (*graph).ncon > 1 as libc::c_int {
        printf(
            b" Balancing constraints: %d\n\0" as *const u8 as *const libc::c_char,
            (*graph).ncon,
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
        (ptypenames[(*params).ptype as usize]).as_ptr(),
        (objtypenames[(*params).objtype as usize]).as_ptr(),
        (ctypenames[(*params).ctype as usize]).as_ptr(),
        (rtypenames[(*params).rtype as usize]).as_ptr(),
        (iptypenames[(*params).iptype as usize]).as_ptr(),
    );
    printf(
        b" dbglvl=%d, ufactor=%.3f, no2hop=%s, minconn=%s, contig=%s, nooutput=%s\n\0" as *const u8
            as *const libc::c_char,
        (*params).dbglvl,
        1.0f64 + 0.001f64 * (*params).ufactor as libc::c_double,
        if (*params).no2hop != 0 {
            b"YES\0" as *const u8 as *const libc::c_char
        } else {
            b"NO\0" as *const u8 as *const libc::c_char
        },
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
    if !((*params).ubvec).is_null() {
        printf(b" ubvec=(\0" as *const u8 as *const libc::c_char);
        i = 0 as libc::c_int;
        while i < (*graph).ncon {
            printf(
                b"%s%.2e\0" as *const u8 as *const libc::c_char,
                if i == 0 as libc::c_int {
                    b"\0" as *const u8 as *const libc::c_char
                } else {
                    b" \0" as *const u8 as *const libc::c_char
                },
                *((*params).ubvec).offset(i as isize) as libc::c_double,
            );
            i += 1;
            i;
        }
        printf(b")\n\0" as *const u8 as *const libc::c_char);
    }
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
pub unsafe extern "C" fn GPReportResults(
    mut params: *mut params_t,
    mut graph: *mut graph_t,
    mut part: *mut idx_t,
    mut objval: idx_t,
) {
    (*params).reporttimer = ((*params).reporttimer as libc::c_double - gk_CPUSeconds()) as real_t;
    ComputePartitionInfo(params, graph, part);
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
        b"  Partitioning: \t\t %7.3f sec   (METIS time)\n\0" as *const u8 as *const libc::c_char,
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
