use libc::{self, exit, fprintf, free, fscanf, printf, sprintf, sscanf, strtof, strtol};
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;

}
pub type __off_t = i64;
pub type __off64_t = i64;

use crate::libmetis::structure::*;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = libc::FILE;
pub type gk_idx_t = ssize_t;

use crate::{
    libmetis::{
        gklib::{
            libmetis__imalloc, libmetis__imax, libmetis__ismalloc, libmetis__rscale,
            libmetis__rsmalloc,
        },
        graph::libmetis__CreateGraph,
        mesh::libmetis__CreateMesh,
        structure::*,
    },
    GKlib::{
        error::errexit,
        fs::{gk_fexists, gk_getfilestats},
        io::{gk_fclose, gk_fopen, gk_getline},
        memory::gk_free,
        string::gk_strchr_replace,
    },
};

#[no_mangle]
pub unsafe extern "C" fn ReadGraph(mut params: *mut params_t) -> *mut graph_t {
    println!("Reading graph");

    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut k: idx_t = 0;
    let mut l: idx_t = 0;
    let mut fmt: idx_t = 0;
    let mut ncon: idx_t = 0;
    let mut nfields: idx_t = 0;
    let mut readew: idx_t = 0;
    let mut readvw: idx_t = 0;
    let mut readvs: idx_t = 0;
    let mut edge: idx_t = 0;
    let mut ewgt: idx_t = 0;

    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut vwgt: *mut idx_t = 0 as *mut idx_t;
    let mut adjwgt: *mut idx_t = 0 as *mut idx_t;
    let mut vsize: *mut idx_t = 0 as *mut idx_t;
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fmtstr: [libc::c_char; 256] = [0; 256];
    let mut curstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut newstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lnlen: size_t = 0 as libc::c_int as size_t;
    let mut fpin: *mut FILE = 0 as *mut FILE;
    let mut graph: *mut graph_t = 0 as *mut graph_t;
    if gk_fexists((*params).filename) == 0 {
        errexit(
            b"File %s does not exist!\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (*params).filename,
        );
    }
    graph = libmetis__CreateGraph();
    fpin = gk_fopen(
        (*params).filename,
        b"r\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"ReadGRaph: Graph\0" as *const u8 as *const libc::c_char,
    );
    loop {
        if gk_getline(&mut line, &mut lnlen, fpin) == -(1) as i64 {
            errexit(
                b"Premature end of input file: file: %s\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*params).filename,
            );
        }
        if !(*line.offset(0 as libc::c_int as isize) as libc::c_int == '%' as i32) {
            break;
        }
    }
    ncon = 0 as libc::c_int;
    fmt = ncon;
    nfields = sscanf(
        line,
        b"%d %d %d %d\0" as *const u8 as *const libc::c_char,
        &mut (*graph).nvtxs as *mut idx_t,
        &mut (*graph).nedges as *mut idx_t,
        &mut fmt as *mut idx_t,
        &mut ncon as *mut idx_t,
    );
    if nfields < 2 as libc::c_int {
        errexit(
            b"The input file does not specify the number of vertices and edges.\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
    }
    if (*graph).nvtxs <= 0 as libc::c_int || (*graph).nedges <= 0 as libc::c_int {
        errexit(
            b"The supplied nvtxs:%d and nedges:%d must be positive.\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            (*graph).nvtxs,
            (*graph).nedges,
        );
    }
    if fmt > 111 {
        errexit(
            b"Cannot read this type of file format [fmt=%d]!\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            fmt,
        );
    }
    sprintf(
        fmtstr.as_mut_ptr(),
        b"%03d\0" as *const u8 as *const libc::c_char,
        fmt % 1000 as libc::c_int,
    );
    readvs = (fmtstr[0] as libc::c_int == '1' as i32) as libc::c_int;
    readvw = (fmtstr[1] as libc::c_int == '1' as i32) as libc::c_int;
    readew = (fmtstr[2 as libc::c_int as usize] as libc::c_int == '1' as i32) as libc::c_int;
    if ncon > 0 as libc::c_int && readvw == 0 {
        errexit(
            b"------------------------------------------------------------------------------\n***  I detected an error in your input file  ***\n\nYou specified ncon=%d, but the fmt parameter does not specify vertex weights\nMake sure that the fmt parameter is set to either 10 or 11.\n------------------------------------------------------------------------------\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            ncon,
        );
    }
    (*graph).nedges *= 2 as libc::c_int;
    (*graph).ncon = if ncon == 0 as libc::c_int { 1 } else { ncon };
    ncon = (*graph).ncon;
    (*graph).xadj = vec![0; ((*graph).nvtxs + 1) as usize];

    let mut xadj = &mut (*graph).xadj;
    (*graph).adjncy = libmetis__imalloc(
        (*graph).nedges as size_t,
        b"ReadGraph: adjncy\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    adjncy = (*graph).adjncy;
    (*graph).vwgt = libmetis__ismalloc(
        (ncon * (*graph).nvtxs) as size_t,
        1,
        b"ReadGraph: vwgt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    vwgt = (*graph).vwgt;
    (*graph).adjwgt = libmetis__ismalloc(
        (*graph).nedges as size_t,
        1,
        b"ReadGraph: adjwgt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    adjwgt = (*graph).adjwgt;
    (*graph).vsize = libmetis__ismalloc(
        (*graph).nvtxs as size_t,
        1,
        b"ReadGraph: vsize\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    vsize = (*graph).vsize;
    xadj[0] = 0 as libc::c_int;
    k = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*graph).nvtxs {
        loop {
            if gk_getline(&mut line, &mut lnlen, fpin) == -(1) as i64 {
                errexit(
                    b"Premature end of input file while reading vertex %d.\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    i + 1,
                );
            }
            if !(*line.offset(0 as libc::c_int as isize) as libc::c_int == '%' as i32) {
                break;
            }
        }
        curstr = line;
        newstr = 0 as *mut libc::c_char;
        if readvs != 0 {
            *vsize.offset(i as isize) = strtol(curstr, &mut newstr, 10 as libc::c_int) as idx_t;
            if newstr == curstr {
                errexit(
                    b"The line for vertex %d does not have vsize information\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    i + 1,
                );
            }
            if *vsize.offset(i as isize) < 0 as libc::c_int {
                errexit(
                    b"The size for vertex %d must be >= 0\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    i + 1,
                );
            }
            curstr = newstr;
        }
        if readvw != 0 {
            l = 0 as libc::c_int;
            while l < ncon {
                *vwgt.offset((i * ncon + l) as isize) =
                    strtol(curstr, &mut newstr, 10 as libc::c_int) as idx_t;
                if newstr == curstr {
                    errexit(
                        b"The line for vertex %d does not have enough weights for the %d constraints.\n\0"
                            as *const u8 as *const libc::c_char as *mut libc::c_char,
                        i + 1,
                        ncon,
                    );
                }
                if *vwgt.offset((i * ncon + l) as isize) < 0 as libc::c_int {
                    errexit(
                        b"The weight vertex %d and constraint %d must be >= 0\n\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        i + 1,
                        l,
                    );
                }
                curstr = newstr;
                l += 1;
                l;
            }
        }
        loop {
            edge = strtol(curstr, &mut newstr, 10 as libc::c_int) as idx_t;
            if newstr == curstr {
                break;
            }
            curstr = newstr;
            if edge < 1 || edge > (*graph).nvtxs {
                errexit(
                    b"Edge %d for vertex %d is out of bounds\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    edge,
                    i + 1,
                );
            }
            ewgt = 1;
            if readew != 0 {
                ewgt = strtol(curstr, &mut newstr, 10 as libc::c_int) as idx_t;
                if newstr == curstr {
                    errexit(
                        b"Premature end of line for vertex %d\n\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        i + 1,
                    );
                }
                if ewgt <= 0 as libc::c_int {
                    errexit(
                        b"The weight (%d) for edge (%d, %d) must be positive.\n\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        ewgt,
                        i + 1,
                        edge,
                    );
                }
                curstr = newstr;
            }
            if k == (*graph).nedges {
                errexit(
                    b"There are more edges in the file than the %d specified.\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    (*graph).nedges / 2 as libc::c_int,
                );
            }
            *adjncy.offset(k as isize) = edge - 1;
            *adjwgt.offset(k as isize) = ewgt;
            k += 1;
            k;
        }
        xadj[(i + 1) as usize] = k;
        i += 1;
        i;
    }
    gk_fclose(fpin);
    if k != (*graph).nedges {
        printf(
            b"------------------------------------------------------------------------------\n\0"
                as *const u8 as *const libc::c_char,
        );
        printf(
            b"***  I detected an error in your input file  ***\n\n\0" as *const u8
                as *const libc::c_char,
        );
        printf(
            b"In the first line of the file, you specified that the graph contained\n%d edges. However, I only found %d edges in the file.\n\0"
                as *const u8 as *const libc::c_char,
            (*graph).nedges / 2 as libc::c_int,
            k / 2 as libc::c_int,
        );
        if 2 as libc::c_int * k == (*graph).nedges {
            printf(
                b"\n *> I detected that you specified twice the number of edges that you have in\n\0"
                    as *const u8 as *const libc::c_char,
            );
            printf(
                b"    the file. Remember that the number of edges specified in the first line\n\0"
                    as *const u8 as *const libc::c_char,
            );
            printf(
                b"    counts each edge between vertices v and u only once.\n\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        printf(
            b"Please specify the correct number of edges in the first line of the file.\n\0"
                as *const u8 as *const libc::c_char,
        );
        printf(
            b"------------------------------------------------------------------------------\n\0"
                as *const u8 as *const libc::c_char,
        );
        exit(0 as libc::c_int);
    }
    gk_free(
        &mut line as *mut *mut libc::c_char as *mut libc::c_void as *mut *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
    return graph;
}
#[no_mangle]
pub unsafe extern "C" fn ReadMesh(mut params: *mut params_t) -> *mut mesh_t {
    println!("Reading Mesh");
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut k: idx_t = 0;
    let mut l: idx_t = 0;
    let mut nfields: idx_t = 0;
    let mut ncon: idx_t = 0;
    let mut node: idx_t = 0;
    let mut eptr: *mut idx_t = 0 as *mut idx_t;
    let mut eind: *mut idx_t = 0 as *mut idx_t;
    let mut ewgt: *mut idx_t = 0 as *mut idx_t;
    let mut nlines: size_t = 0;
    let mut ntokens: size_t = 0;
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut curstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut newstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lnlen: size_t = 0 as libc::c_int as size_t;
    let mut fpin: *mut FILE = 0 as *mut FILE;
    let mut mesh: *mut mesh_t = 0 as *mut mesh_t;
    if gk_fexists((*params).filename) == 0 {
        errexit(
            b"File %s does not exist!\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (*params).filename,
        );
    }
    mesh = libmetis__CreateMesh();
    gk_getfilestats(
        (*params).filename,
        &mut nlines,
        &mut ntokens,
        0 as *mut size_t,
        0 as *mut size_t,
    );
    fpin = gk_fopen(
        (*params).filename,
        b"r\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"ReadMesh\0")).as_ptr(),
    );
    loop {
        if gk_getline(&mut line, &mut lnlen, fpin) == -(1) as i64 {
            errexit(
                b"Premature end of input file: file: %s\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*params).filename,
            );
        }
        if !(*line.offset(0 as libc::c_int as isize) as libc::c_int == '%' as i32) {
            break;
        }
    }
    (*mesh).ncon = 0 as libc::c_int;
    nfields = sscanf(
        line,
        b"%d %d\0" as *const u8 as *const libc::c_char,
        &mut (*mesh).ne as *mut idx_t,
        &mut (*mesh).ncon as *mut idx_t,
    );
    if nfields < 1 {
        errexit(
            b"The input file does not specify the number of elements.\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
    }
    if (*mesh).ne <= 0 as libc::c_int {
        errexit(
            b"The supplied number of elements:%d must be positive.\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            (*mesh).ne,
        );
    }
    if (*mesh).ne as u64 > nlines {
        errexit(
            b"The file has %zu lines which smaller than the number of elements of %d specified in the header line.\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            nlines,
            (*mesh).ne,
        );
    }
    ncon = (*mesh).ncon;
    (*mesh).eptr = libmetis__ismalloc(
        ((*mesh).ne + 1) as size_t,
        0 as libc::c_int,
        b"ReadMesh: eptr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    eptr = (*mesh).eptr;
    (*mesh).eind = libmetis__imalloc(
        ntokens,
        b"ReadMesh: eind\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    eind = (*mesh).eind;
    (*mesh).ewgt = libmetis__ismalloc(
        ((if ncon == 0 as libc::c_int { 1 } else { ncon }) * (*mesh).ne) as size_t,
        1,
        b"ReadMesh: ewgt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    ewgt = (*mesh).ewgt;
    *eptr.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    k = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*mesh).ne {
        loop {
            if gk_getline(&mut line, &mut lnlen, fpin) == -(1) as i64 {
                errexit(
                    b"Premature end of input file while reading element %d.\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    i + 1,
                );
            }
            if !(*line.offset(0 as libc::c_int as isize) as libc::c_int == '%' as i32) {
                break;
            }
        }
        curstr = line;
        newstr = 0 as *mut libc::c_char;
        l = 0 as libc::c_int;
        while l < ncon {
            *ewgt.offset((i * ncon + l) as isize) =
                strtol(curstr, &mut newstr, 10 as libc::c_int) as idx_t;
            if newstr == curstr {
                errexit(
                    b"The line for vertex %d does not have enough weights for the %d constraints.\n\0"
                        as *const u8 as *const libc::c_char as *mut libc::c_char,
                    i + 1,
                    ncon,
                );
            }
            if *ewgt.offset((i * ncon + l) as isize) < 0 as libc::c_int {
                errexit(
                    b"The weight for element %d and constraint %d must be >= 0\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    i + 1,
                    l,
                );
            }
            curstr = newstr;
            l += 1;
            l;
        }
        loop {
            node = strtol(curstr, &mut newstr, 10 as libc::c_int) as idx_t;
            if newstr == curstr {
                break;
            }
            curstr = newstr;
            if node < 1 {
                errexit(
                    b"Node %d for element %d is out of bounds\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    node,
                    i + 1,
                );
            }
            let fresh0 = k;
            k = k + 1;
            *eind.offset(fresh0 as isize) = node - 1;
        }
        *eptr.offset((i + 1) as isize) = k;
        i += 1;
        i;
    }
    gk_fclose(fpin);
    (*mesh).ncon = if ncon == 0 as libc::c_int { 1 } else { ncon };
    (*mesh).nn = libmetis__imax(*eptr.offset((*mesh).ne as isize) as size_t, eind) + 1;
    gk_free(
        &mut line as *mut *mut libc::c_char as *mut libc::c_void as *mut *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
    return mesh;
}
#[no_mangle]
pub unsafe extern "C" fn ReadTPwgts(mut params: *mut params_t, mut ncon: idx_t) {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut from: idx_t = 0;
    let mut to: idx_t = 0;
    let mut fromcnum: idx_t = 0;
    let mut tocnum: idx_t = 0;
    let mut nleft: idx_t = 0;
    let mut awgt: real_t = 0.0f64 as real_t;
    let mut twgt: real_t = 0.;
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut curstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut newstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lnlen: size_t = 0 as libc::c_int as size_t;
    let mut fpin: *mut FILE = 0 as *mut FILE;
    (*params).tpwgts = libmetis__rsmalloc(
        ((*params).nparts * ncon) as size_t,
        -1.0f64 as real_t,
        b"ReadTPwgts: tpwgts\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if ((*params).tpwgtsfile).is_null() {
        i = 0 as libc::c_int;
        while i < (*params).nparts {
            j = 0 as libc::c_int;
            while j < ncon {
                *((*params).tpwgts).offset((i * ncon + j) as isize) =
                    (1.0f64 / (*params).nparts as libc::c_double) as real_t;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
        return;
    }
    if gk_fexists((*params).tpwgtsfile) == 0 {
        errexit(
            b"Graph file %s does not exist!\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (*params).tpwgtsfile,
        );
    }
    fpin = gk_fopen(
        (*params).tpwgtsfile,
        b"r\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"ReadTPwgts: tpwgtsfile\0" as *const u8 as *const libc::c_char,
    );
    while gk_getline(&mut line, &mut lnlen, fpin) != -(1) as i64 {
        gk_strchr_replace(
            line,
            b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        curstr = line;
        newstr = 0 as *mut libc::c_char;
        from = strtol(curstr, &mut newstr, 10 as libc::c_int) as idx_t;
        if newstr == curstr {
            errexit(
                b"The 'from' component of line <%s> in the tpwgts file is incorrect.\n\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
                line,
            );
        }
        curstr = newstr;
        if *curstr.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32 {
            to = strtol(curstr.offset(1 as isize), &mut newstr, 10 as libc::c_int) as idx_t;
            if newstr == curstr {
                errexit(
                    b"The 'to' component of line <%s> in the tpwgts file is incorrect.\n\0"
                        as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    line,
                );
            }
            curstr = newstr;
        } else {
            to = from;
        }
        if *curstr.offset(0 as libc::c_int as isize) as libc::c_int == ':' as i32 {
            fromcnum = strtol(curstr.offset(1 as isize), &mut newstr, 10 as libc::c_int) as idx_t;
            if newstr == curstr {
                errexit(
                    b"The 'fromcnum' component of line <%s> in the tpwgts file is incorrect.\n\0"
                        as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    line,
                );
            }
            curstr = newstr;
            if *curstr.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32 {
                tocnum = strtol(curstr.offset(1 as isize), &mut newstr, 10 as libc::c_int) as idx_t;
                if newstr == curstr {
                    errexit(
                        b"The 'tocnum' component of line <%s> in the tpwgts file is incorrect.\n\0"
                            as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        line,
                    );
                }
                curstr = newstr;
            } else {
                tocnum = fromcnum;
            }
        } else {
            fromcnum = 0 as libc::c_int;
            tocnum = ncon - 1;
        }
        if *curstr.offset(0 as libc::c_int as isize) as libc::c_int == '=' as i32 {
            awgt = strtof(curstr.offset(1 as isize), &mut newstr);
            if newstr == curstr {
                errexit(
                    b"The 'wgt' component of line <%s> in the tpwgts file is incorrect.\n\0"
                        as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    line,
                );
            }
            curstr = newstr;
        } else {
            errexit(
                b"The 'wgt' component of line <%s> in the tpwgts file is missing.\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                line,
            );
        }
        if from < 0 as libc::c_int
            || to < 0 as libc::c_int
            || from >= (*params).nparts
            || to >= (*params).nparts
        {
            errexit(
                b"Invalid partition range for %d:%d\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                from,
                to,
            );
        }
        if fromcnum < 0 as libc::c_int
            || tocnum < 0 as libc::c_int
            || fromcnum >= ncon
            || tocnum >= ncon
        {
            errexit(
                b"Invalid constraint number range for %d:%d\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fromcnum,
                tocnum,
            );
        }
        if awgt as libc::c_double <= 0.0f64 || awgt as libc::c_double >= 1.0f64 {
            errexit(
                b"Invalid partition weight of %f\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                awgt as libc::c_double,
            );
        }
        i = from;
        while i <= to {
            j = fromcnum;
            while j <= tocnum {
                *((*params).tpwgts).offset((i * ncon + j) as isize) = awgt;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    }
    gk_fclose(fpin);
    j = 0 as libc::c_int;
    while j < ncon {
        twgt = 0.0f64 as real_t;
        nleft = (*params).nparts;
        i = 0 as libc::c_int;
        while i < (*params).nparts {
            if *((*params).tpwgts).offset((i * ncon + j) as isize)
                > 0 as libc::c_int as libc::c_float
            {
                twgt += *((*params).tpwgts).offset((i * ncon + j) as isize);
                nleft -= 1;
                nleft;
            }
            i += 1;
            i;
        }
        if nleft == 0 as libc::c_int {
            libmetis__rscale(
                (*params).nparts as size_t,
                (1.0f64 / twgt as libc::c_double) as real_t,
                ((*params).tpwgts).offset(j as isize),
                ncon as size_t,
            );
        }
        if nleft > 0 as libc::c_int {
            if twgt > 1 as libc::c_float {
                errexit(
                    b"The total specified target partition weights for constraint #%d of %f exceeds 1.0.\n\0"
                        as *const u8 as *const libc::c_char as *mut libc::c_char,
                    j,
                    twgt as libc::c_double,
                );
            }
            awgt = ((1.0f64 - twgt as libc::c_double) / nleft as libc::c_double) as real_t;
            i = 0 as libc::c_int;
            while i < (*params).nparts {
                *((*params).tpwgts).offset((i * ncon + j) as isize) = if *((*params).tpwgts)
                    .offset((i * ncon + j) as isize)
                    < 0 as libc::c_int as libc::c_float
                {
                    awgt
                } else {
                    *((*params).tpwgts).offset((i * ncon + j) as isize)
                };
                i += 1;
                i;
            }
        }
        j += 1;
        j;
    }
    free(line as *mut libc::c_void);
    line = 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn ReadPOVector(
    mut graph: *mut graph_t,
    mut filename: *mut libc::c_char,
    mut vector: *mut idx_t,
) {
    let mut i: idx_t = 0;
    let mut fpin: *mut FILE = 0 as *mut FILE;
    fpin = gk_fopen(
        filename,
        b"r\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"ReadPOVector\0")).as_ptr(),
    );
    i = 0 as libc::c_int;
    while i < (*graph).nvtxs {
        if fscanf(
            fpin,
            b"%d\n\0" as *const u8 as *const libc::c_char,
            vector.offset(i as isize),
        ) != 1
        {
            errexit(
                b"[%s] Premature end of file %s at line %d [nvtxs: %d]\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"ReadPOVector\0"))
                    .as_ptr(),
                filename,
                i,
                (*graph).nvtxs,
            );
        }
        i += 1;
        i;
    }
    gk_fclose(fpin);
}
#[no_mangle]
pub unsafe extern "C" fn WritePartition(
    mut fname: *mut libc::c_char,
    mut part: *mut idx_t,
    mut n: idx_t,
    mut nparts: idx_t,
) {
    let mut fpout: *mut FILE = 0 as *mut FILE;
    let mut i: idx_t = 0;
    let mut filename: [libc::c_char; 1280000] = [0; 1280000];
    sprintf(
        filename.as_mut_ptr(),
        b"%s.part.%d\0" as *const u8 as *const libc::c_char,
        fname,
        nparts,
    );
    fpout = gk_fopen(
        filename.as_mut_ptr(),
        b"w\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"WritePartition\0")).as_ptr(),
    );
    i = 0 as libc::c_int;
    while i < n {
        fprintf(
            fpout,
            b"%d\n\0" as *const u8 as *const libc::c_char,
            *part.offset(i as isize),
        );
        i += 1;
        i;
    }
    gk_fclose(fpout);
}
#[no_mangle]
pub unsafe extern "C" fn WriteMeshPartition(
    mut fname: *mut libc::c_char,
    mut nparts: idx_t,
    mut ne: idx_t,
    mut epart: *mut idx_t,
    mut nn: idx_t,
    mut npart: *mut idx_t,
) {
    let mut fpout: *mut FILE = 0 as *mut FILE;
    let mut i: idx_t = 0;
    let mut filename: [libc::c_char; 256] = [0; 256];
    fprintf(
        fpout,
        b"%s.epart.%d\0" as *const u8 as *const libc::c_char,
        fname,
        nparts,
    );
    fpout = gk_fopen(
        filename.as_mut_ptr(),
        b"w\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"WriteMeshPartition\0"))
            .as_ptr(),
    );
    i = 0 as libc::c_int;
    while i < ne {
        fprintf(
            fpout,
            b"%d\n\0" as *const u8 as *const libc::c_char,
            *epart.offset(i as isize),
        );
        i += 1;
        i;
    }
    gk_fclose(fpout);
    sprintf(
        filename.as_mut_ptr(),
        b"%s.npart.%d\0" as *const u8 as *const libc::c_char,
        fname,
        nparts,
    );
    fpout = gk_fopen(
        filename.as_mut_ptr(),
        b"w\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"WriteMeshPartition\0"))
            .as_ptr(),
    );
    i = 0 as libc::c_int;
    while i < nn {
        fprintf(
            fpout,
            b"%d\n\0" as *const u8 as *const libc::c_char,
            *npart.offset(i as isize),
        );
        i += 1;
        i;
    }
    gk_fclose(fpout);
}
#[no_mangle]
pub unsafe extern "C" fn WritePermutation(
    mut fname: *mut libc::c_char,
    mut iperm: *mut idx_t,
    mut n: idx_t,
) {
    let mut fpout: *mut FILE = 0 as *mut FILE;
    let mut i: idx_t = 0;
    let mut filename: [libc::c_char; 1280000] = [0; 1280000];
    sprintf(
        filename.as_mut_ptr(),
        b"%s.iperm\0" as *const u8 as *const libc::c_char,
        fname,
    );
    fpout = gk_fopen(
        filename.as_mut_ptr(),
        b"w\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"WritePermutation\0")).as_ptr(),
    );
    i = 0 as libc::c_int;
    while i < n {
        fprintf(
            fpout,
            b"%d\n\0" as *const u8 as *const libc::c_char,
            *iperm.offset(i as isize),
        );
        i += 1;
        i;
    }
    gk_fclose(fpout);
}
#[no_mangle]
pub unsafe extern "C" fn WriteGraph(mut graph: *mut graph_t, mut filename: *mut libc::c_char) {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut ncon: idx_t = 0;

    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut adjwgt: *mut idx_t = 0 as *mut idx_t;
    let mut vwgt: *mut idx_t = 0 as *mut idx_t;
    let mut vsize: *mut idx_t = 0 as *mut idx_t;
    let mut hasvwgt: libc::c_int = 0 as libc::c_int;
    let mut hasewgt: libc::c_int = 0 as libc::c_int;
    let mut hasvsize: libc::c_int = 0 as libc::c_int;
    let mut fpout: *mut FILE = 0 as *mut FILE;
    nvtxs = (*graph).nvtxs;
    ncon = (*graph).ncon;
    let mut xadj = &mut (*graph).xadj;
    adjncy = (*graph).adjncy;
    vwgt = (*graph).vwgt;
    vsize = (*graph).vsize;
    adjwgt = (*graph).adjwgt;
    if !vwgt.is_null() {
        i = 0 as libc::c_int;
        while i < nvtxs * ncon {
            if *vwgt.offset(i as isize) != 1 {
                hasvwgt = 1;
                break;
            } else {
                i += 1;
                i;
            }
        }
    }
    if !vsize.is_null() {
        i = 0 as libc::c_int;
        while i < nvtxs {
            if *vsize.offset(i as isize) != 1 {
                hasvsize = 1;
                break;
            } else {
                i += 1;
                i;
            }
        }
    }
    if !adjwgt.is_null() {
        i = 0 as libc::c_int;
        while i < xadj[nvtxs as usize] {
            if *adjwgt.offset(i as isize) != 1 {
                hasewgt = 1;
                break;
            } else {
                i += 1;
                i;
            }
        }
    }
    fpout = gk_fopen(
        filename,
        b"w\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"WriteGraph\0")).as_ptr(),
    );
    fprintf(
        fpout,
        b"%d %d\0" as *const u8 as *const libc::c_char,
        nvtxs,
        xadj[nvtxs as usize] / 2 as libc::c_int,
    );
    if hasvwgt != 0 || hasvsize != 0 || hasewgt != 0 {
        fprintf(
            fpout,
            b" %d%d%d\0" as *const u8 as *const libc::c_char,
            hasvsize,
            hasvwgt,
            hasewgt,
        );
        if hasvwgt != 0 {
            fprintf(
                fpout,
                b" %d\0" as *const u8 as *const libc::c_char,
                (*graph).ncon,
            );
        }
    }
    i = 0 as libc::c_int;
    while i < nvtxs {
        fprintf(fpout, b"\n\0" as *const u8 as *const libc::c_char);
        if hasvsize != 0 {
            fprintf(
                fpout,
                b" %d\0" as *const u8 as *const libc::c_char,
                *vsize.offset(i as isize),
            );
        }
        if hasvwgt != 0 {
            j = 0 as libc::c_int;
            while j < ncon {
                fprintf(
                    fpout,
                    b" %d\0" as *const u8 as *const libc::c_char,
                    *vwgt.offset((i * ncon + j) as isize),
                );
                j += 1;
                j;
            }
        }
        j = xadj[i as usize];
        while j < xadj[(i + 1) as usize] {
            fprintf(
                fpout,
                b" %d\0" as *const u8 as *const libc::c_char,
                *adjncy.offset(j as isize) + 1,
            );
            if hasewgt != 0 {
                fprintf(
                    fpout,
                    b" %d\0" as *const u8 as *const libc::c_char,
                    *adjwgt.offset(j as isize),
                );
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    gk_fclose(fpout);
}
