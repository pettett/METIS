use std::env;
use std::ffi::CStr;

use ::libc;
use libc::{exit, printf};

use metis::libmetis::mesh::libmetis__FreeMesh;
use metis::libmetis::meshpart::{METIS_PartMeshDual, METIS_PartMeshNodal, METIS_PTYPE_KWAY};
use metis::programs::cmdline_mpmetis::parse_cmdline;

use metis::libmetis::{
    auxapi::*, coarsen::libmetis__CoarsenGraph, contig::*, fortran::*, gklib::*, graph::*,
    kwayrefine::*, options::*, structure::*, timing::*, util::*, wspace::*,
};
use metis::programs::io::{ReadMesh, ReadTPwgts, WriteMeshPartition};
use metis::GKlib::memory::{
    gk_GetCurMemoryUsed, gk_GetMaxMemoryUsed, gk_free, gk_malloc_cleanup, gk_malloc_init,
};
use metis::GKlib::timers::gk_CPUSeconds;

static mut gtypenames: [&CStr; 2] = unsafe { [c"dual", c"nodal"] };
static mut iptypenames: [&CStr; 5] = unsafe { [c"grow", c"random", c"edge", c"node", c"metisrb"] };
static mut rtypenames: [&CStr; 4] = unsafe { [c"fm", c"greedy", c"2sided", c"1sided"] };
static mut ctypenames: [&CStr; 2] = unsafe { [c"rm", c"shem"] };
static mut objtypenames: [&CStr; 3] = unsafe { [c"cut", c"vol", c"node"] };
static mut ptypenames: [&CStr; 2] = unsafe { [c"rb", c"kway"] };

unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut options: [idx_t; 40] = [0; 40];
    let mut mesh: *mut mesh_t = 0 as *mut mesh_t;
    let mut epart: *mut idx_t = 0 as *mut idx_t;
    let mut npart: *mut idx_t = 0 as *mut idx_t;
    let mut objval: idx_t = 0;
    let mut params: *mut params_t = 0 as *mut params_t;
    let mut status: libc::c_int = 0 as libc::c_int;
    params = parse_cmdline(argc, argv);
    (*params).iotimer = ((*params).iotimer as libc::c_double - gk_CPUSeconds()) as real_t;
    mesh = ReadMesh(params);
    if (*mesh).ncon > 1 as libc::c_int {
        printf(
            b"*** Meshes with more than one balancing constraint are not supported yet.\n\0"
                as *const u8 as *const libc::c_char,
        );
        exit(0 as libc::c_int);
    }
    ReadTPwgts(params, (*mesh).ncon);
    (*params).iotimer = ((*params).iotimer as libc::c_double + gk_CPUSeconds()) as real_t;
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
    (*params).parttimer = ((*params).parttimer as libc::c_double - gk_CPUSeconds()) as real_t;
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
            WriteMeshPartition(
                (*params).filename,
                (*params).nparts,
                (*mesh).ne,
                epart,
                (*mesh).nn,
                npart,
            );
            (*params).iotimer = ((*params).iotimer as libc::c_double + gk_CPUSeconds()) as real_t;
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
        b"METIS 5.0 Copyright 1998-13, Regents of the University of Minnesota\n\0" as *const u8
            as *const libc::c_char,
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
        (8 as libc::c_int as u64).wrapping_mul(::core::mem::size_of::<idx_t>() as u64),
        (8 as libc::c_int as u64).wrapping_mul(::core::mem::size_of::<real_t>() as u64),
        (8 as libc::c_int as u64).wrapping_mul(::core::mem::size_of::<*mut idx_t>() as u64),
    );
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"Mesh Information ------------------------------------------------------------\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b" Name: %s, #Elements: %d, #Nodes: %d, #Parts: %d\n\0" as *const u8 as *const libc::c_char,
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
        (ptypenames[(*params).ptype as usize]).as_ptr(),
        (objtypenames[(*params).objtype as usize]).as_ptr(),
        (ctypenames[(*params).ctype as usize]).as_ptr(),
        (rtypenames[(*params).rtype as usize]).as_ptr(),
        (iptypenames[(*params).iptype as usize]).as_ptr(),
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
        b" gtype=%s, ncommon=%d, niter=%d, ncuts=%d\n\0" as *const u8 as *const libc::c_char,
        (gtypenames[(*params).gtype as usize]).as_ptr(),
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
    (*params).reporttimer = ((*params).reporttimer as libc::c_double - gk_CPUSeconds()) as real_t;
    printf(
        b" - %s: %d.\n\n\0" as *const u8 as *const libc::c_char,
        if (*params).objtype == METIS_OBJTYPE_CUT as libc::c_int {
            b"Edgecut\0" as *const u8 as *const libc::c_char
        } else {
            b"Volume\0" as *const u8 as *const libc::c_char
        },
        objval,
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
