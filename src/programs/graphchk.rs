use ::libc;
extern "C" {
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: u64) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn WriteGraph(graph: *mut graph_t, filename: *mut libc::c_char);
    fn ReadGraph(_: *mut params_t) -> *mut graph_t;
    fn libmetis__FreeGraph(graph: *mut *mut graph_t);
    fn libmetis__FixGraph(graph: *mut graph_t) -> *mut graph_t;
    fn libmetis__CheckGraph(
        graph: *mut graph_t,
        numflag: libc::c_int,
        verbose: libc::c_int,
    ) -> libc::c_int;
    fn gk_strdup(orgstr: *mut libc::c_char) -> *mut libc::c_char;
    fn gk_free(ptr1: *mut *mut libc::c_void, _: ...);
}
pub type __int32_t = libc::c_int;
pub type int32_t = __int32_t;
pub type size_t = u64;
pub type idx_t = int32_t;
pub type real_t = libc::c_float;
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
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut graph: *mut graph_t = 0 as *mut graph_t;
    let mut fgraph: *mut graph_t = 0 as *mut graph_t;
    let mut filename: [libc::c_char; 256] = [0; 256];
    let mut wgtflag: idx_t = 0;
    let mut params: params_t = params_t {
        ptype: 0,
        objtype: 0,
        ctype: 0,
        iptype: 0,
        rtype: 0,
        no2hop: 0,
        minconn: 0,
        contig: 0,
        nooutput: 0,
        balance: 0,
        ncuts: 0,
        niter: 0,
        gtype: 0,
        ncommon: 0,
        seed: 0,
        dbglvl: 0,
        nparts: 0,
        nseps: 0,
        ufactor: 0,
        pfactor: 0,
        compress: 0,
        ccorder: 0,
        filename: 0 as *mut libc::c_char,
        outfile: 0 as *mut libc::c_char,
        xyzfile: 0 as *mut libc::c_char,
        tpwgtsfile: 0 as *mut libc::c_char,
        ubvecstr: 0 as *mut libc::c_char,
        wgtflag: 0,
        numflag: 0,
        tpwgts: 0 as *mut real_t,
        ubvec: 0 as *mut real_t,
        iotimer: 0.,
        parttimer: 0.,
        reporttimer: 0.,
        maxmemory: 0,
    };
    if argc != 2 as libc::c_int && argc != 3 as libc::c_int {
        printf(
            b"Usage: %s <GraphFile> [FixedGraphFile (for storing the fixed graph)]\n\0" as *const u8
                as *const libc::c_char,
            *argv.offset(0 as libc::c_int as isize),
        );
        exit(0 as libc::c_int);
    }
    memset(
        &mut params as *mut params_t as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<params_t>() as u64,
    );
    params.filename = gk_strdup(*argv.offset(1 as libc::c_int as isize));
    graph = ReadGraph(&mut params);
    if (*graph).nvtxs == 0 as libc::c_int {
        printf(b"Empty graph!\n\0" as *const u8 as *const libc::c_char);
        exit(0 as libc::c_int);
    }
    printf(
        b"**********************************************************************\n\0" as *const u8
            as *const libc::c_char,
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
        b"16:17:18\0" as *const u8 as *const libc::c_char,
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
        b"Graph Information ---------------------------------------------------\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"  Name: %s, #Vertices: %d, #Edges: %d\n\n\0" as *const u8 as *const libc::c_char,
        params.filename,
        (*graph).nvtxs,
        (*graph).nedges / 2 as libc::c_int,
    );
    printf(
        b"Checking Graph... ---------------------------------------------------\n\0" as *const u8
            as *const libc::c_char,
    );
    if libmetis__CheckGraph(graph, 1 as libc::c_int, 1 as libc::c_int) != 0 {
        printf(b"   The format of the graph is correct!\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"   The format of the graph is incorrect!\n\0" as *const u8 as *const libc::c_char);
        if argc == 3 as libc::c_int {
            fgraph = libmetis__FixGraph(graph);
            WriteGraph(fgraph, *argv.offset(2 as libc::c_int as isize));
            libmetis__FreeGraph(&mut fgraph);
            printf(
                b"   A corrected version was stored at %s\n\0" as *const u8 as *const libc::c_char,
                *argv.offset(2 as libc::c_int as isize),
            );
        }
    }
    printf(
        b"\n**********************************************************************\n\0" as *const u8
            as *const libc::c_char,
    );
    libmetis__FreeGraph(&mut graph);
    gk_free(
        &mut params.filename as *mut *mut libc::c_char as *mut *mut libc::c_void,
        &mut params.tpwgtsfile as *mut *mut libc::c_char,
        &mut params.tpwgts as *mut *mut real_t,
        0 as *mut *mut libc::c_void,
    );
    return 0;
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
