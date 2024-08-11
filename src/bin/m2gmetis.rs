use std::ffi::CStr;

use ::libc;
extern "C" {
    fn exit(_: libc::c_int) -> !;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn parse_cmdline(argc: libc::c_int, argv: *mut *mut libc::c_char) -> *mut params_t;
    fn WriteGraph(graph: *mut graph_t, filename: *mut libc::c_char);
    fn ReadMesh(_: *mut params_t) -> *mut mesh_t;
    fn gk_malloc_init() -> libc::c_int;
    fn gk_malloc_cleanup(showstats: libc::c_int);
    fn gk_free(ptr1: *mut *mut libc::c_void, _: ...);
    fn gk_GetCurMemoryUsed() -> size_t;
    fn gk_GetMaxMemoryUsed() -> size_t;
    fn gk_CPUSeconds() -> libc::c_double;
    fn METIS_MeshToDual(
        ne: *mut idx_t,
        nn: *mut idx_t,
        eptr: *mut idx_t,
        eind: *mut idx_t,
        ncommon: *mut idx_t,
        numflag: *mut idx_t,
        r_xadj: *mut *mut idx_t,
        r_adjncy: *mut *mut idx_t,
    ) -> libc::c_int;
    fn METIS_MeshToNodal(
        ne: *mut idx_t,
        nn: *mut idx_t,
        eptr: *mut idx_t,
        eind: *mut idx_t,
        numflag: *mut idx_t,
        r_xadj: *mut *mut idx_t,
        r_adjncy: *mut *mut idx_t,
    ) -> libc::c_int;
    fn libmetis__CreateGraph() -> *mut graph_t;
    fn libmetis__FreeGraph(graph: *mut *mut graph_t);
    fn libmetis__FreeMesh(mesh: *mut *mut mesh_t);
}
pub type __int32_t = libc::c_int;
pub type int32_t = __int32_t;
pub type size_t = u64;
pub type idx_t = int32_t;
pub type real_t = libc::c_float;
pub type C2RustUnnamed = libc::c_int;
pub const METIS_ERROR: C2RustUnnamed = -4;
pub const METIS_ERROR_MEMORY: C2RustUnnamed = -3;
pub const METIS_ERROR_INPUT: C2RustUnnamed = -2;
pub const METIS_OK: C2RustUnnamed = 1;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const METIS_GTYPE_NODAL: C2RustUnnamed_0 = 1;
pub const METIS_GTYPE_DUAL: C2RustUnnamed_0 = 0;
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
static mut gtypenames: [&CStr; 2] = unsafe { [c"dual", c"nodal"] };
static mut iptypenames: [&CStr; 5] = unsafe { [c"grow", c"random", c"edge", c"node", c"metisrb"] };
static mut rtypenames: [&CStr; 4] = unsafe { [c"fm", c"greedy", c"2sided", c"1sided"] };
static mut ctypenames: [&CStr; 2] = unsafe { [c"rm", c"shem"] };
static mut objtypenames: [&CStr; 3] = unsafe { [c"cut", c"vol", c"node"] };
static mut ptypenames: [&CStr; 2] = unsafe { [c"rb", c"kway"] };
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut mesh: *mut mesh_t = 0 as *mut mesh_t;
    let mut graph: *mut graph_t = 0 as *mut graph_t;
    let mut params: *mut params_t = 0 as *mut params_t;
    let mut status: libc::c_int = 0 as libc::c_int;
    params = parse_cmdline(argc, argv);
    (*params).iotimer = ((*params).iotimer as libc::c_double - gk_CPUSeconds()) as real_t;
    mesh = ReadMesh(params);
    (*params).iotimer = ((*params).iotimer as libc::c_double + gk_CPUSeconds()) as real_t;
    if (*mesh).ncon > 1 as libc::c_int {
        printf(
            b"*** Meshes with more than one balancing constraint are not supported yet.\n\0"
                as *const u8 as *const libc::c_char,
        );
        exit(0 as libc::c_int);
    }
    M2GPrintInfo(params, mesh);
    graph = libmetis__CreateGraph();
    gk_malloc_init();
    (*params).parttimer = ((*params).parttimer as libc::c_double - gk_CPUSeconds()) as real_t;
    match (*params).gtype {
        0 => {
            status = METIS_MeshToDual(
                &mut (*mesh).ne,
                &mut (*mesh).nn,
                (*mesh).eptr,
                (*mesh).eind,
                &mut (*params).ncommon,
                &mut (*params).numflag,
                &mut (*graph).xadj,
                &mut (*graph).adjncy,
            );
            if status == METIS_OK as libc::c_int {
                (*graph).nvtxs = (*mesh).ne;
                (*graph).nedges = *((*graph).xadj).offset((*graph).nvtxs as isize);
                (*graph).ncon = 1 as libc::c_int;
            }
        }
        1 => {
            status = METIS_MeshToNodal(
                &mut (*mesh).ne,
                &mut (*mesh).nn,
                (*mesh).eptr,
                (*mesh).eind,
                &mut (*params).numflag,
                &mut (*graph).xadj,
                &mut (*graph).adjncy,
            );
            if status == METIS_OK as libc::c_int {
                (*graph).nvtxs = (*mesh).nn;
                (*graph).nedges = *((*graph).xadj).offset((*graph).nvtxs as isize);
                (*graph).ncon = 1 as libc::c_int;
            }
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
        (*params).iotimer = ((*params).iotimer as libc::c_double - gk_CPUSeconds()) as real_t;
        WriteGraph(graph, (*params).outfile);
        (*params).iotimer = ((*params).iotimer as libc::c_double + gk_CPUSeconds()) as real_t;
        M2GReportResults(params, mesh, graph);
    }
    libmetis__FreeGraph(&mut graph);
    libmetis__FreeMesh(&mut mesh);
    gk_free(
        &mut (*params).filename as *mut *mut libc::c_char as *mut *mut libc::c_void,
        &mut (*params).outfile as *mut *mut libc::c_char,
        &mut params as *mut *mut params_t,
        0 as *mut *mut libc::c_void,
    );
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn M2GPrintInfo(mut params: *mut params_t, mut mesh: *mut mesh_t) {
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
        b" Name: %s, #Elements: %d, #Nodes: %d\n\0" as *const u8 as *const libc::c_char,
        (*params).filename,
        (*mesh).ne,
        (*mesh).nn,
    );
    printf(
        b"Options ---------------------------------------------------------------------\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b" gtype=%s, ncommon=%d, outfile=%s\n\0" as *const u8 as *const libc::c_char,
        (gtypenames[(*params).gtype as usize]).as_ptr(),
        (*params).ncommon,
        (*params).outfile,
    );
    printf(b"\n\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn M2GReportResults(
    mut params: *mut params_t,
    mut mesh: *mut mesh_t,
    mut graph: *mut graph_t,
) {
    (*params).reporttimer = ((*params).reporttimer as libc::c_double - gk_CPUSeconds()) as real_t;
    printf(
        b" - #nvtxs: %d, #edges: %d\n\0" as *const u8 as *const libc::c_char,
        (*graph).nvtxs,
        (*graph).nedges,
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
