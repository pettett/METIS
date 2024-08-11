use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> i64;
    fn strtof(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_float;
    static mut stdout: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: u64,
    ) -> *mut libc::c_void;
    fn gk_i32incset(n: size_t, baseval: int32_t, x: *mut int32_t) -> *mut int32_t;
    fn gk_fopen(
        _: *mut libc::c_char,
        _: *mut libc::c_char,
        _: *const libc::c_char,
    ) -> *mut FILE;
    fn gk_fclose(_: *mut FILE);
    fn gk_getline(
        lineptr: *mut *mut libc::c_char,
        n: *mut size_t,
        stream: *mut FILE,
    ) -> gk_idx_t;
    fn gk_fexists(_: *mut libc::c_char) -> libc::c_int;
    fn gk_i32malloc(n: size_t, msg: *mut libc::c_char) -> *mut int32_t;
    fn gk_i32smalloc(n: size_t, ival: int32_t, msg: *mut libc::c_char) -> *mut int32_t;
    fn gk_i32copy(n: size_t, a: *mut int32_t, b: *mut int32_t) -> *mut int32_t;
    fn gk_zmalloc(n: size_t, msg: *mut libc::c_char) -> *mut ssize_t;
    fn gk_zcopy(n: size_t, a: *mut ssize_t, b: *mut ssize_t) -> *mut ssize_t;
    fn gk_fmalloc(n: size_t, msg: *mut libc::c_char) -> *mut libc::c_float;
    fn gk_fsmalloc(
        n: size_t,
        ival: libc::c_float,
        msg: *mut libc::c_char,
    ) -> *mut libc::c_float;
    fn gk_fcopy(
        n: size_t,
        a: *mut libc::c_float,
        b: *mut libc::c_float,
    ) -> *mut libc::c_float;
    fn gk_malloc(nbytes: size_t, msg: *mut libc::c_char) -> *mut libc::c_void;
    fn gk_free(ptr1: *mut *mut libc::c_void, _: ...);
    fn gk_errexit(signum: libc::c_int, _: *mut libc::c_char, _: ...);
    fn gk_i32pqCreate(maxnodes: size_t) -> *mut gk_i32pq_t;
    fn gk_i32pqDestroy(queue: *mut gk_i32pq_t);
    fn gk_i32pqInsert(
        queue: *mut gk_i32pq_t,
        node: gk_idx_t,
        key: int32_t,
    ) -> libc::c_int;
    fn gk_i32pqUpdate(queue: *mut gk_i32pq_t, node: gk_idx_t, newkey: int32_t);
    fn gk_i32pqGetTop(queue: *mut gk_i32pq_t) -> gk_idx_t;
    fn gk_fpqCreate(maxnodes: size_t) -> *mut gk_fpq_t;
    fn gk_fpqDestroy(queue: *mut gk_fpq_t);
    fn gk_fpqInsert(
        queue: *mut gk_fpq_t,
        node: gk_idx_t,
        key: libc::c_float,
    ) -> libc::c_int;
    fn gk_fpqUpdate(queue: *mut gk_fpq_t, node: gk_idx_t, newkey: libc::c_float);
    fn gk_fpqGetTop(queue: *mut gk_fpq_t) -> gk_idx_t;
}
pub type __int32_t = libc::c_int;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __ssize_t = i64;
pub type int32_t = __int32_t;
pub type ssize_t = __ssize_t;
pub type size_t = u64;
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
pub type FILE = _IO_FILE;
pub type gk_idx_t = ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gk_i32kv_t {
    pub key: int32_t,
    pub val: ssize_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gk_fkv_t {
    pub key: libc::c_float,
    pub val: ssize_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gk_i32pq_t {
    pub nnodes: gk_idx_t,
    pub maxnodes: gk_idx_t,
    pub heap: *mut gk_i32kv_t,
    pub locator: *mut gk_idx_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gk_fpq_t {
    pub nnodes: gk_idx_t,
    pub maxnodes: gk_idx_t,
    pub heap: *mut gk_fkv_t,
    pub locator: *mut gk_idx_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gk_graph_t {
    pub nvtxs: int32_t,
    pub xadj: *mut ssize_t,
    pub adjncy: *mut int32_t,
    pub iadjwgt: *mut int32_t,
    pub fadjwgt: *mut libc::c_float,
    pub ivwgts: *mut int32_t,
    pub fvwgts: *mut libc::c_float,
    pub ivsizes: *mut int32_t,
    pub fvsizes: *mut libc::c_float,
    pub vlabels: *mut int32_t,
}
#[no_mangle]
pub unsafe extern "C" fn gk_graph_Create() -> *mut gk_graph_t {
    let mut graph: *mut gk_graph_t = 0 as *mut gk_graph_t;
    graph = gk_malloc(
        ::core::mem::size_of::<gk_graph_t>() as u64,
        b"gk_graph_Create: graph\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    ) as *mut gk_graph_t;
    gk_graph_Init(graph);
    return graph;
}
#[no_mangle]
pub unsafe extern "C" fn gk_graph_Init(mut graph: *mut gk_graph_t) {
    memset(
        graph as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<gk_graph_t>() as u64,
    );
    (*graph).nvtxs = -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn gk_graph_Free(mut graph: *mut *mut gk_graph_t) {
    if (*graph).is_null() {
        return;
    }
    gk_graph_FreeContents(*graph);
    gk_free(graph as *mut *mut libc::c_void, 0 as *mut *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn gk_graph_FreeContents(mut graph: *mut gk_graph_t) {
    gk_free(
        &mut (*graph).xadj as *mut *mut ssize_t as *mut libc::c_void
            as *mut *mut libc::c_void,
        &mut (*graph).adjncy as *mut *mut int32_t,
        &mut (*graph).iadjwgt as *mut *mut int32_t,
        &mut (*graph).fadjwgt as *mut *mut libc::c_float,
        &mut (*graph).ivwgts as *mut *mut int32_t,
        &mut (*graph).fvwgts as *mut *mut libc::c_float,
        &mut (*graph).ivsizes as *mut *mut int32_t,
        &mut (*graph).fvsizes as *mut *mut libc::c_float,
        &mut (*graph).vlabels as *mut *mut int32_t,
        0 as *mut *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gk_graph_Read(
    mut filename: *mut libc::c_char,
    mut format: libc::c_int,
    mut isfewgts: libc::c_int,
    mut isfvwgts: libc::c_int,
    mut isfvsizes: libc::c_int,
) -> *mut gk_graph_t {
    let mut i: ssize_t = 0;
    let mut k: ssize_t = 0;
    let mut l: ssize_t = 0;
    let mut nfields: size_t = 0;
    let mut nvtxs: size_t = 0;
    let mut nedges: size_t = 0;
    let mut fmt: size_t = 0;
    let mut ncon: size_t = 0;
    let mut lnlen: size_t = 0;
    let mut ival: int32_t = 0;
    let mut fval: libc::c_float = 0.;
    let mut readsizes: libc::c_int = 0 as libc::c_int;
    let mut readwgts: libc::c_int = 0 as libc::c_int;
    let mut readvals: libc::c_int = 0 as libc::c_int;
    let mut numbering: libc::c_int = 0 as libc::c_int;
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut head: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tail: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fmtstr: [libc::c_char; 256] = [0; 256];
    let mut fpin: *mut FILE = 0 as *mut FILE;
    let mut graph: *mut gk_graph_t = 0 as *mut gk_graph_t;
    if gk_fexists(filename) == 0 {
        gk_errexit(
            15 as libc::c_int,
            b"File %s does not exist!\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            filename,
        );
    }
    if format == 1 as libc::c_int {
        fpin = gk_fopen(
            filename,
            b"r\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"gk_graph_Read: fpin\0" as *const u8 as *const libc::c_char,
        );
        loop {
            if gk_getline(&mut line, &mut lnlen, fpin)
                <= 0 as libc::c_int as i64
            {
                gk_errexit(
                    15 as libc::c_int,
                    b"Premature end of input file: file:%s\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    filename,
                );
            }
            if !(*line.offset(0 as libc::c_int as isize) as libc::c_int == '%' as i32) {
                break;
            }
        }
        ncon = 0 as libc::c_int as size_t;
        fmt = ncon;
        nfields = sscanf(
            line,
            b"%zu %zu %zu %zu\0" as *const u8 as *const libc::c_char,
            &mut nvtxs as *mut size_t,
            &mut nedges as *mut size_t,
            &mut fmt as *mut size_t,
            &mut ncon as *mut size_t,
        ) as size_t;
        if nfields < 2 as libc::c_int as u64 {
            gk_errexit(
                15 as libc::c_int,
                b"Header line must contain at least 2 integers (#vtxs and #edges).\n\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
        nedges = (nedges as u64)
            .wrapping_mul(2 as libc::c_int as u64) as size_t as size_t;
        if fmt > 111 as libc::c_int as u64 {
            gk_errexit(
                15 as libc::c_int,
                b"Cannot read this type of file format [fmt=%zu]!\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                fmt,
            );
        }
        sprintf(
            fmtstr.as_mut_ptr(),
            b"%03zu\0" as *const u8 as *const libc::c_char,
            fmt.wrapping_rem(1000 as libc::c_int as u64),
        );
        readsizes = (fmtstr[0 as libc::c_int as usize] as libc::c_int == '1' as i32)
            as libc::c_int;
        readwgts = (fmtstr[1 as libc::c_int as usize] as libc::c_int == '1' as i32)
            as libc::c_int;
        readvals = (fmtstr[2 as libc::c_int as usize] as libc::c_int == '1' as i32)
            as libc::c_int;
        numbering = 1 as libc::c_int;
        ncon = if ncon == 0 as libc::c_int as u64 {
            1 as libc::c_int as u64
        } else {
            ncon
        };
    } else {
        gk_errexit(
            15 as libc::c_int,
            b"Unrecognized format: %d\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            format,
        );
    }
    graph = gk_graph_Create();
    (*graph).nvtxs = nvtxs as int32_t;
    (*graph)
        .xadj = gk_zmalloc(
        nvtxs.wrapping_add(1 as libc::c_int as u64),
        b"gk_graph_Read: xadj\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (*graph)
        .adjncy = gk_i32malloc(
        nedges,
        b"gk_graph_Read: adjncy\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    if readvals != 0 {
        if isfewgts != 0 {
            (*graph)
                .fadjwgt = gk_fmalloc(
                nedges,
                b"gk_graph_Read: fadjwgt\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        } else {
            (*graph)
                .iadjwgt = gk_i32malloc(
                nedges,
                b"gk_graph_Read: iadjwgt\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
    }
    if readsizes != 0 {
        if isfvsizes != 0 {
            (*graph)
                .fvsizes = gk_fmalloc(
                nvtxs,
                b"gk_graph_Read: fvsizes\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        } else {
            (*graph)
                .ivsizes = gk_i32malloc(
                nvtxs,
                b"gk_graph_Read: ivsizes\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
    }
    if readwgts != 0 {
        if isfvwgts != 0 {
            (*graph)
                .fvwgts = gk_fmalloc(
                nvtxs.wrapping_mul(ncon),
                b"gk_graph_Read: fvwgts\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        } else {
            (*graph)
                .ivwgts = gk_i32malloc(
                nvtxs.wrapping_mul(ncon),
                b"gk_graph_Read: ivwgts\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
    }
    numbering = if numbering != 0 { -(1 as libc::c_int) } else { 0 as libc::c_int };
    *((*graph).xadj).offset(0 as libc::c_int as isize) = 0 as libc::c_int as ssize_t;
    k = 0 as libc::c_int as ssize_t;
    i = 0 as libc::c_int as ssize_t;
    while (i as u64) < nvtxs {
        loop {
            if gk_getline(&mut line, &mut lnlen, fpin)
                == -(1 as libc::c_int) as i64
            {
                gk_errexit(
                    15 as libc::c_int,
                    b"Pregraphure end of input file: file while reading row %d\n\0"
                        as *const u8 as *const libc::c_char as *mut libc::c_char,
                    i,
                );
            }
            if !(*line.offset(0 as libc::c_int as isize) as libc::c_int == '%' as i32) {
                break;
            }
        }
        head = line;
        tail = 0 as *mut libc::c_char;
        if readsizes != 0 {
            if isfvsizes != 0 {
                *((*graph).fvsizes).offset(i as isize) = strtof(head, &mut tail);
                if tail == head {
                    gk_errexit(
                        15 as libc::c_int,
                        b"The line for vertex %zd does not have size information\n\0"
                            as *const u8 as *const libc::c_char as *mut libc::c_char,
                        i + 1 as libc::c_int as i64,
                    );
                }
                if *((*graph).fvsizes).offset(i as isize)
                    < 0 as libc::c_int as libc::c_float
                {
                    gk_errexit(
                        15 as libc::c_int,
                        b"The size for vertex %zd must be >= 0\n\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        i + 1 as libc::c_int as i64,
                    );
                }
            } else {
                *((*graph).ivsizes)
                    .offset(
                        i as isize,
                    ) = strtol(head, &mut tail, 0 as libc::c_int) as int32_t;
                if tail == head {
                    gk_errexit(
                        15 as libc::c_int,
                        b"The line for vertex %zd does not have size information\n\0"
                            as *const u8 as *const libc::c_char as *mut libc::c_char,
                        i + 1 as libc::c_int as i64,
                    );
                }
                if *((*graph).ivsizes).offset(i as isize) < 0 as libc::c_int {
                    gk_errexit(
                        15 as libc::c_int,
                        b"The size for vertex %zd must be >= 0\n\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        i + 1 as libc::c_int as i64,
                    );
                }
            }
            head = tail;
        }
        if readwgts != 0 {
            l = 0 as libc::c_int as ssize_t;
            while (l as u64) < ncon {
                if isfvwgts != 0 {
                    *((*graph).fvwgts)
                        .offset(
                            (i as u64)
                                .wrapping_mul(ncon)
                                .wrapping_add(l as u64) as isize,
                        ) = strtof(head, &mut tail);
                    if tail == head {
                        gk_errexit(
                            15 as libc::c_int,
                            b"The line for vertex %zd does not have enough weights for the %d constraints.\n\0"
                                as *const u8 as *const libc::c_char as *mut libc::c_char,
                            i + 1 as libc::c_int as i64,
                            ncon,
                        );
                    }
                    if *((*graph).fvwgts)
                        .offset(
                            (i as u64)
                                .wrapping_mul(ncon)
                                .wrapping_add(l as u64) as isize,
                        ) < 0 as libc::c_int as libc::c_float
                    {
                        gk_errexit(
                            15 as libc::c_int,
                            b"The weight vertex %zd and constraint %zd must be >= 0\n\0"
                                as *const u8 as *const libc::c_char as *mut libc::c_char,
                            i + 1 as libc::c_int as i64,
                            l,
                        );
                    }
                } else {
                    *((*graph).ivwgts)
                        .offset(
                            (i as u64)
                                .wrapping_mul(ncon)
                                .wrapping_add(l as u64) as isize,
                        ) = strtol(head, &mut tail, 0 as libc::c_int) as int32_t;
                    if tail == head {
                        gk_errexit(
                            15 as libc::c_int,
                            b"The line for vertex %zd does not have enough weights for the %d constraints.\n\0"
                                as *const u8 as *const libc::c_char as *mut libc::c_char,
                            i + 1 as libc::c_int as i64,
                            ncon,
                        );
                    }
                    if *((*graph).ivwgts)
                        .offset(
                            (i as u64)
                                .wrapping_mul(ncon)
                                .wrapping_add(l as u64) as isize,
                        ) < 0 as libc::c_int
                    {
                        gk_errexit(
                            15 as libc::c_int,
                            b"The weight vertex %zd and constraint %zd must be >= 0\n\0"
                                as *const u8 as *const libc::c_char as *mut libc::c_char,
                            i + 1 as libc::c_int as i64,
                            l,
                        );
                    }
                }
                head = tail;
                l += 1;
                l;
            }
        }
        loop {
            ival = strtol(head, &mut tail, 0 as libc::c_int) as libc::c_int;
            if tail == head {
                break;
            }
            head = tail;
            let ref mut fresh0 = *((*graph).adjncy).offset(k as isize);
            *fresh0 = ival + numbering;
            if *fresh0 < 0 as libc::c_int {
                gk_errexit(
                    15 as libc::c_int,
                    b"Error: Invalid column number %d at row %zd.\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    ival,
                    i,
                );
            }
            if readvals != 0 {
                if isfewgts != 0 {
                    fval = strtof(head, &mut tail);
                    if tail == head {
                        gk_errexit(
                            15 as libc::c_int,
                            b"Value could not be found for edge! Vertex:%zd, NNZ:%zd\n\0"
                                as *const u8 as *const libc::c_char as *mut libc::c_char,
                            i,
                            k,
                        );
                    }
                    *((*graph).fadjwgt).offset(k as isize) = fval;
                } else {
                    ival = strtol(head, &mut tail, 0 as libc::c_int) as int32_t;
                    if tail == head {
                        gk_errexit(
                            15 as libc::c_int,
                            b"Value could not be found for edge! Vertex:%zd, NNZ:%zd\n\0"
                                as *const u8 as *const libc::c_char as *mut libc::c_char,
                            i,
                            k,
                        );
                    }
                    *((*graph).iadjwgt).offset(k as isize) = ival;
                }
                head = tail;
            }
            k += 1;
            k;
        }
        *((*graph).xadj).offset((i + 1 as libc::c_int as i64) as isize) = k;
        i += 1;
        i;
    }
    if k as u64 != nedges {
        gk_errexit(
            15 as libc::c_int,
            b"gk_graph_Read: Something wrong with the number of edges in the input file. nedges=%zd, Actualnedges=%zd.\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            nedges,
            k,
        );
    }
    gk_fclose(fpin);
    gk_free(
        &mut line as *mut *mut libc::c_char as *mut *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
    return graph;
}
#[no_mangle]
pub unsafe extern "C" fn gk_graph_Write(
    mut graph: *mut gk_graph_t,
    mut filename: *mut libc::c_char,
    mut format: libc::c_int,
) {
    let mut i: ssize_t = 0;
    let mut j: ssize_t = 0;
    let mut hasvwgts: libc::c_int = 0;
    let mut hasvsizes: libc::c_int = 0;
    let mut hasewgts: libc::c_int = 0;
    let mut fpout: *mut FILE = 0 as *mut FILE;
    if format != 1 as libc::c_int {
        gk_errexit(
            15 as libc::c_int,
            b"Unknown file format. %d\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            format,
        );
    }
    if !filename.is_null() {
        fpout = gk_fopen(
            filename,
            b"w\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"gk_graph_Write: fpout\0" as *const u8 as *const libc::c_char,
        );
    } else {
        fpout = stdout;
    }
    hasewgts = (!((*graph).iadjwgt).is_null() || !((*graph).fadjwgt).is_null())
        as libc::c_int;
    hasvwgts = (!((*graph).ivwgts).is_null() || !((*graph).fvwgts).is_null())
        as libc::c_int;
    hasvsizes = (!((*graph).ivsizes).is_null() || !((*graph).fvsizes).is_null())
        as libc::c_int;
    fprintf(
        fpout,
        b"%d %zd\0" as *const u8 as *const libc::c_char,
        (*graph).nvtxs,
        *((*graph).xadj).offset((*graph).nvtxs as isize)
            / 2 as libc::c_int as i64,
    );
    if hasvwgts != 0 || hasvsizes != 0 || hasewgts != 0 {
        fprintf(
            fpout,
            b" %d%d%d\0" as *const u8 as *const libc::c_char,
            hasvsizes,
            hasvwgts,
            hasewgts,
        );
    }
    fprintf(fpout, b"\n\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int as ssize_t;
    while i < (*graph).nvtxs as i64 {
        if hasvsizes != 0 {
            if !((*graph).ivsizes).is_null() {
                fprintf(
                    fpout,
                    b" %d\0" as *const u8 as *const libc::c_char,
                    *((*graph).ivsizes).offset(i as isize),
                );
            } else {
                fprintf(
                    fpout,
                    b" %f\0" as *const u8 as *const libc::c_char,
                    *((*graph).fvsizes).offset(i as isize) as libc::c_double,
                );
            }
        }
        if hasvwgts != 0 {
            if !((*graph).ivwgts).is_null() {
                fprintf(
                    fpout,
                    b" %d\0" as *const u8 as *const libc::c_char,
                    *((*graph).ivwgts).offset(i as isize),
                );
            } else {
                fprintf(
                    fpout,
                    b" %f\0" as *const u8 as *const libc::c_char,
                    *((*graph).fvwgts).offset(i as isize) as libc::c_double,
                );
            }
        }
        j = *((*graph).xadj).offset(i as isize);
        while j
            < *((*graph).xadj).offset((i + 1 as libc::c_int as i64) as isize)
        {
            fprintf(
                fpout,
                b" %d\0" as *const u8 as *const libc::c_char,
                *((*graph).adjncy).offset(j as isize) + 1 as libc::c_int,
            );
            if hasewgts != 0 {
                if !((*graph).iadjwgt).is_null() {
                    fprintf(
                        fpout,
                        b" %d\0" as *const u8 as *const libc::c_char,
                        *((*graph).iadjwgt).offset(j as isize),
                    );
                } else {
                    fprintf(
                        fpout,
                        b" %f\0" as *const u8 as *const libc::c_char,
                        *((*graph).fadjwgt).offset(j as isize) as libc::c_double,
                    );
                }
            }
            j += 1;
            j;
        }
        fprintf(fpout, b"\n\0" as *const u8 as *const libc::c_char);
        i += 1;
        i;
    }
    if !filename.is_null() {
        gk_fclose(fpout);
    }
}
#[no_mangle]
pub unsafe extern "C" fn gk_graph_Dup(mut graph: *mut gk_graph_t) -> *mut gk_graph_t {
    let mut ngraph: *mut gk_graph_t = 0 as *mut gk_graph_t;
    ngraph = gk_graph_Create();
    (*ngraph).nvtxs = (*graph).nvtxs;
    if !((*graph).xadj).is_null() {
        (*ngraph)
            .xadj = gk_zcopy(
            ((*graph).nvtxs + 1 as libc::c_int) as size_t,
            (*graph).xadj,
            gk_zmalloc(
                ((*graph).nvtxs + 1 as libc::c_int) as size_t,
                b"gk_graph_Dup: xadj\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ),
        );
    }
    if !((*graph).ivwgts).is_null() {
        (*ngraph)
            .ivwgts = gk_i32copy(
            (*graph).nvtxs as size_t,
            (*graph).ivwgts,
            gk_i32malloc(
                (*graph).nvtxs as size_t,
                b"gk_graph_Dup: ivwgts\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ),
        );
    }
    if !((*graph).ivsizes).is_null() {
        (*ngraph)
            .ivsizes = gk_i32copy(
            (*graph).nvtxs as size_t,
            (*graph).ivsizes,
            gk_i32malloc(
                (*graph).nvtxs as size_t,
                b"gk_graph_Dup: ivsizes\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ),
        );
    }
    if !((*graph).vlabels).is_null() {
        (*ngraph)
            .vlabels = gk_i32copy(
            (*graph).nvtxs as size_t,
            (*graph).vlabels,
            gk_i32malloc(
                (*graph).nvtxs as size_t,
                b"gk_graph_Dup: ivlabels\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ),
        );
    }
    if !((*graph).fvwgts).is_null() {
        (*ngraph)
            .fvwgts = gk_fcopy(
            (*graph).nvtxs as size_t,
            (*graph).fvwgts,
            gk_fmalloc(
                (*graph).nvtxs as size_t,
                b"gk_graph_Dup: fvwgts\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ),
        );
    }
    if !((*graph).fvsizes).is_null() {
        (*ngraph)
            .fvsizes = gk_fcopy(
            (*graph).nvtxs as size_t,
            (*graph).fvsizes,
            gk_fmalloc(
                (*graph).nvtxs as size_t,
                b"gk_graph_Dup: fvsizes\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ),
        );
    }
    if !((*graph).adjncy).is_null() {
        (*ngraph)
            .adjncy = gk_i32copy(
            *((*graph).xadj).offset((*graph).nvtxs as isize) as size_t,
            (*graph).adjncy,
            gk_i32malloc(
                *((*graph).xadj).offset((*graph).nvtxs as isize) as size_t,
                b"gk_graph_Dup: adjncy\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ),
        );
    }
    if !((*graph).iadjwgt).is_null() {
        (*ngraph)
            .iadjwgt = gk_i32copy(
            *((*graph).xadj).offset((*graph).nvtxs as isize) as size_t,
            (*graph).iadjwgt,
            gk_i32malloc(
                *((*graph).xadj).offset((*graph).nvtxs as isize) as size_t,
                b"gk_graph_Dup: iadjwgt\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ),
        );
    }
    if !((*graph).fadjwgt).is_null() {
        (*ngraph)
            .fadjwgt = gk_fcopy(
            *((*graph).xadj).offset((*graph).nvtxs as isize) as size_t,
            (*graph).fadjwgt,
            gk_fmalloc(
                *((*graph).xadj).offset((*graph).nvtxs as isize) as size_t,
                b"gk_graph_Dup: fadjwgt\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ),
        );
    }
    return ngraph;
}
#[no_mangle]
pub unsafe extern "C" fn gk_graph_ExtractSubgraph(
    mut graph: *mut gk_graph_t,
    mut vstart: libc::c_int,
    mut nvtxs: libc::c_int,
) -> *mut gk_graph_t {
    let mut i: ssize_t = 0;
    let mut ngraph: *mut gk_graph_t = 0 as *mut gk_graph_t;
    if vstart + nvtxs > (*graph).nvtxs {
        return 0 as *mut gk_graph_t;
    }
    ngraph = gk_graph_Create();
    (*ngraph).nvtxs = nvtxs;
    if !((*graph).xadj).is_null() {
        (*ngraph)
            .xadj = gk_zcopy(
            (nvtxs + 1 as libc::c_int) as size_t,
            ((*graph).xadj).offset(vstart as isize),
            gk_zmalloc(
                (nvtxs + 1 as libc::c_int) as size_t,
                b"gk_graph_ExtractSubgraph: xadj\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ),
        );
    }
    i = nvtxs as ssize_t;
    while i >= 0 as libc::c_int as i64 {
        let ref mut fresh1 = *((*ngraph).xadj).offset(i as isize);
        *fresh1 -= *((*ngraph).xadj).offset(0 as libc::c_int as isize);
        i -= 1;
        i;
    }
    if !((*graph).ivwgts).is_null() {
        (*ngraph)
            .ivwgts = gk_i32copy(
            nvtxs as size_t,
            ((*graph).ivwgts).offset(vstart as isize),
            gk_i32malloc(
                nvtxs as size_t,
                b"gk_graph_ExtractSubgraph: ivwgts\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ),
        );
    }
    if !((*graph).ivsizes).is_null() {
        (*ngraph)
            .ivsizes = gk_i32copy(
            nvtxs as size_t,
            ((*graph).ivsizes).offset(vstart as isize),
            gk_i32malloc(
                nvtxs as size_t,
                b"gk_graph_ExtractSubgraph: ivsizes\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            ),
        );
    }
    if !((*graph).vlabels).is_null() {
        (*ngraph)
            .vlabels = gk_i32copy(
            nvtxs as size_t,
            ((*graph).vlabels).offset(vstart as isize),
            gk_i32malloc(
                nvtxs as size_t,
                b"gk_graph_ExtractSubgraph: vlabels\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            ),
        );
    }
    if !((*graph).fvwgts).is_null() {
        (*ngraph)
            .fvwgts = gk_fcopy(
            nvtxs as size_t,
            ((*graph).fvwgts).offset(vstart as isize),
            gk_fmalloc(
                nvtxs as size_t,
                b"gk_graph_ExtractSubgraph: fvwgts\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ),
        );
    }
    if !((*graph).fvsizes).is_null() {
        (*ngraph)
            .fvsizes = gk_fcopy(
            nvtxs as size_t,
            ((*graph).fvsizes).offset(vstart as isize),
            gk_fmalloc(
                nvtxs as size_t,
                b"gk_graph_ExtractSubgraph: fvsizes\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            ),
        );
    }
    if !((*graph).adjncy).is_null() {
        (*ngraph)
            .adjncy = gk_i32copy(
            (*((*graph).xadj).offset((vstart + nvtxs) as isize)
                - *((*graph).xadj).offset(vstart as isize)) as size_t,
            ((*graph).adjncy).offset(*((*graph).xadj).offset(vstart as isize) as isize),
            gk_i32malloc(
                (*((*graph).xadj).offset((vstart + nvtxs) as isize)
                    - *((*graph).xadj).offset(vstart as isize)) as size_t,
                b"gk_graph_ExtractSubgraph: adjncy\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ),
        );
    }
    if !((*graph).iadjwgt).is_null() {
        (*ngraph)
            .iadjwgt = gk_i32copy(
            (*((*graph).xadj).offset((vstart + nvtxs) as isize)
                - *((*graph).xadj).offset(vstart as isize)) as size_t,
            ((*graph).iadjwgt).offset(*((*graph).xadj).offset(vstart as isize) as isize),
            gk_i32malloc(
                (*((*graph).xadj).offset((vstart + nvtxs) as isize)
                    - *((*graph).xadj).offset(vstart as isize)) as size_t,
                b"gk_graph_ExtractSubgraph: iadjwgt\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            ),
        );
    }
    if !((*graph).fadjwgt).is_null() {
        (*ngraph)
            .fadjwgt = gk_fcopy(
            (*((*graph).xadj).offset((vstart + nvtxs) as isize)
                - *((*graph).xadj).offset(vstart as isize)) as size_t,
            ((*graph).fadjwgt).offset(*((*graph).xadj).offset(vstart as isize) as isize),
            gk_fmalloc(
                (*((*graph).xadj).offset((vstart + nvtxs) as isize)
                    - *((*graph).xadj).offset(vstart as isize)) as size_t,
                b"gk_graph_ExtractSubgraph: fadjwgt\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            ),
        );
    }
    return ngraph;
}
#[no_mangle]
pub unsafe extern "C" fn gk_graph_Reorder(
    mut graph: *mut gk_graph_t,
    mut perm: *mut int32_t,
    mut iperm: *mut int32_t,
) -> *mut gk_graph_t {
    let mut j: ssize_t = 0;
    let mut jj: ssize_t = 0;
    let mut xadj: *mut ssize_t = 0 as *mut ssize_t;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut u: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut nvtxs: libc::c_int = 0;
    let mut freeperm: libc::c_int = 0 as libc::c_int;
    let mut freeiperm: libc::c_int = 0 as libc::c_int;
    let mut adjncy: *mut int32_t = 0 as *mut int32_t;
    let mut ngraph: *mut gk_graph_t = 0 as *mut gk_graph_t;
    if perm.is_null() && iperm.is_null() {
        return 0 as *mut gk_graph_t;
    }
    ngraph = gk_graph_Create();
    nvtxs = (*graph).nvtxs;
    (*ngraph).nvtxs = nvtxs;
    xadj = (*graph).xadj;
    adjncy = (*graph).adjncy;
    if !((*graph).xadj).is_null() {
        (*ngraph)
            .xadj = gk_zmalloc(
            (nvtxs + 1 as libc::c_int) as size_t,
            b"gk_graph_Reorder: xadj\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if !((*graph).ivwgts).is_null() {
        (*ngraph)
            .ivwgts = gk_i32malloc(
            nvtxs as size_t,
            b"gk_graph_Reorder: ivwgts\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if !((*graph).ivsizes).is_null() {
        (*ngraph)
            .ivsizes = gk_i32malloc(
            nvtxs as size_t,
            b"gk_graph_Reorder: ivsizes\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if !((*graph).vlabels).is_null() {
        (*ngraph)
            .vlabels = gk_i32malloc(
            nvtxs as size_t,
            b"gk_graph_Reorder: ivlabels\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if !((*graph).fvwgts).is_null() {
        (*ngraph)
            .fvwgts = gk_fmalloc(
            nvtxs as size_t,
            b"gk_graph_Reorder: fvwgts\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if !((*graph).fvsizes).is_null() {
        (*ngraph)
            .fvsizes = gk_fmalloc(
            nvtxs as size_t,
            b"gk_graph_Reorder: fvsizes\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if !((*graph).adjncy).is_null() {
        (*ngraph)
            .adjncy = gk_i32malloc(
            *((*graph).xadj).offset(nvtxs as isize) as size_t,
            b"gk_graph_Reorder: adjncy\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if !((*graph).iadjwgt).is_null() {
        (*ngraph)
            .iadjwgt = gk_i32malloc(
            *((*graph).xadj).offset(nvtxs as isize) as size_t,
            b"gk_graph_Reorder: iadjwgt\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if !((*graph).fadjwgt).is_null() {
        (*ngraph)
            .fadjwgt = gk_fmalloc(
            *((*graph).xadj).offset(nvtxs as isize) as size_t,
            b"gk_graph_Reorder: fadjwgt\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if perm.is_null() {
        freeperm = 1 as libc::c_int;
        perm = gk_i32malloc(
            nvtxs as size_t,
            b"gk_graph_Reorder: perm\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        i = 0 as libc::c_int;
        while i < nvtxs {
            *perm.offset(*iperm.offset(i as isize) as isize) = i;
            i += 1;
            i;
        }
    }
    if iperm.is_null() {
        freeiperm = 1 as libc::c_int;
        iperm = gk_i32malloc(
            nvtxs as size_t,
            b"gk_graph_Reorder: iperm\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        i = 0 as libc::c_int;
        while i < nvtxs {
            *iperm.offset(*perm.offset(i as isize) as isize) = i;
            i += 1;
            i;
        }
    }
    jj = 0 as libc::c_int as ssize_t;
    *((*ngraph).xadj).offset(0 as libc::c_int as isize) = jj;
    v = 0 as libc::c_int;
    while v < nvtxs {
        u = *iperm.offset(v as isize);
        j = *xadj.offset(u as isize);
        while j < *xadj.offset((u + 1 as libc::c_int) as isize) {
            *((*ngraph).adjncy)
                .offset(jj as isize) = *perm.offset(*adjncy.offset(j as isize) as isize);
            if !((*graph).iadjwgt).is_null() {
                *((*ngraph).iadjwgt)
                    .offset(jj as isize) = *((*graph).iadjwgt).offset(j as isize);
            }
            if !((*graph).fadjwgt).is_null() {
                *((*ngraph).fadjwgt)
                    .offset(jj as isize) = *((*graph).fadjwgt).offset(j as isize);
            }
            j += 1;
            j;
            jj += 1;
            jj;
        }
        if !((*graph).ivwgts).is_null() {
            *((*ngraph).ivwgts)
                .offset(v as isize) = *((*graph).ivwgts).offset(u as isize);
        }
        if !((*graph).fvwgts).is_null() {
            *((*ngraph).fvwgts)
                .offset(v as isize) = *((*graph).fvwgts).offset(u as isize);
        }
        if !((*graph).ivsizes).is_null() {
            *((*ngraph).ivsizes)
                .offset(v as isize) = *((*graph).ivsizes).offset(u as isize);
        }
        if !((*graph).fvsizes).is_null() {
            *((*ngraph).fvsizes)
                .offset(v as isize) = *((*graph).fvsizes).offset(u as isize);
        }
        if !((*graph).vlabels).is_null() {
            *((*ngraph).vlabels)
                .offset(v as isize) = *((*graph).vlabels).offset(u as isize);
        }
        *((*ngraph).xadj).offset((v + 1 as libc::c_int) as isize) = jj;
        v += 1;
        v;
    }
    if freeperm != 0 {
        gk_free(
            &mut perm as *mut *mut int32_t as *mut *mut libc::c_void,
            0 as *mut *mut libc::c_void,
        );
    }
    if freeiperm != 0 {
        gk_free(
            &mut iperm as *mut *mut int32_t as *mut *mut libc::c_void,
            0 as *mut *mut libc::c_void,
        );
    }
    return ngraph;
}
#[no_mangle]
pub unsafe extern "C" fn gk_graph_FindComponents(
    mut graph: *mut gk_graph_t,
    mut cptr: *mut int32_t,
    mut cind: *mut int32_t,
) -> libc::c_int {
    let mut i: ssize_t = 0;
    let mut ii: ssize_t = 0;
    let mut j: ssize_t = 0;
    let mut jj: ssize_t = 0;
    let mut k: ssize_t = 0;
    let mut nvtxs: ssize_t = 0;
    let mut first: ssize_t = 0;
    let mut last: ssize_t = 0;
    let mut ntodo: ssize_t = 0;
    let mut ncmps: ssize_t = 0;
    let mut xadj: *mut ssize_t = 0 as *mut ssize_t;
    let mut adjncy: *mut int32_t = 0 as *mut int32_t;
    let mut pos: *mut int32_t = 0 as *mut int32_t;
    let mut todo: *mut int32_t = 0 as *mut int32_t;
    let mut mustfree_ccsr: int32_t = 0 as libc::c_int;
    let mut mustfree_where: int32_t = 0 as libc::c_int;
    nvtxs = (*graph).nvtxs as ssize_t;
    xadj = (*graph).xadj;
    adjncy = (*graph).adjncy;
    if cptr.is_null() {
        cptr = gk_i32malloc(
            (nvtxs + 1 as libc::c_int as i64) as size_t,
            b"gk_graph_FindComponents: cptr\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        cind = gk_i32malloc(
            nvtxs as size_t,
            b"gk_graph_FindComponents: cind\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        mustfree_ccsr = 1 as libc::c_int;
    }
    todo = gk_i32incset(
        nvtxs as size_t,
        0 as libc::c_int,
        gk_i32malloc(
            nvtxs as size_t,
            b"gk_graph_FindComponents: todo\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        ),
    );
    pos = gk_i32incset(
        nvtxs as size_t,
        0 as libc::c_int,
        gk_i32malloc(
            nvtxs as size_t,
            b"gk_graph_FindComponents: pos\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        ),
    );
    ncmps = -(1 as libc::c_int) as ssize_t;
    ntodo = nvtxs;
    last = 0 as libc::c_int as ssize_t;
    first = last;
    while ntodo > 0 as libc::c_int as i64 {
        if first == last {
            ncmps += 1;
            *cptr.offset(ncmps as isize) = first as int32_t;
            i = *todo.offset(0 as libc::c_int as isize) as ssize_t;
            let fresh2 = last;
            last = last + 1;
            *cind.offset(fresh2 as isize) = i as int32_t;
            *pos.offset(i as isize) = -(1 as libc::c_int);
        }
        let fresh3 = first;
        first = first + 1;
        i = *cind.offset(fresh3 as isize) as ssize_t;
        k = *pos.offset(i as isize) as ssize_t;
        ntodo -= 1;
        let ref mut fresh4 = *todo.offset(k as isize);
        *fresh4 = *todo.offset(ntodo as isize);
        j = *fresh4 as ssize_t;
        *pos.offset(j as isize) = k as int32_t;
        j = *xadj.offset(i as isize);
        while j < *xadj.offset((i + 1 as libc::c_int as i64) as isize) {
            k = *adjncy.offset(j as isize) as ssize_t;
            if *pos.offset(k as isize) != -(1 as libc::c_int) {
                let fresh5 = last;
                last = last + 1;
                *cind.offset(fresh5 as isize) = k as int32_t;
                *pos.offset(k as isize) = -(1 as libc::c_int);
            }
            j += 1;
            j;
        }
    }
    ncmps += 1;
    *cptr.offset(ncmps as isize) = first as int32_t;
    if mustfree_ccsr != 0 {
        gk_free(
            &mut cptr as *mut *mut int32_t as *mut *mut libc::c_void,
            &mut cind as *mut *mut int32_t,
            0 as *mut *mut libc::c_void,
        );
    }
    gk_free(
        &mut pos as *mut *mut int32_t as *mut *mut libc::c_void,
        &mut todo as *mut *mut int32_t,
        0 as *mut *mut libc::c_void,
    );
    return ncmps as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gk_graph_ComputeBFSOrdering(
    mut graph: *mut gk_graph_t,
    mut v: libc::c_int,
    mut r_perm: *mut *mut int32_t,
    mut r_iperm: *mut *mut int32_t,
) {
    let mut j: ssize_t = 0;
    let mut xadj: *mut ssize_t = 0 as *mut ssize_t;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut nvtxs: libc::c_int = 0;
    let mut first: libc::c_int = 0;
    let mut last: libc::c_int = 0;
    let mut adjncy: *mut int32_t = 0 as *mut int32_t;
    let mut cot: *mut int32_t = 0 as *mut int32_t;
    let mut pos: *mut int32_t = 0 as *mut int32_t;
    if (*graph).nvtxs <= 0 as libc::c_int {
        return;
    }
    nvtxs = (*graph).nvtxs;
    xadj = (*graph).xadj;
    adjncy = (*graph).adjncy;
    pos = gk_i32incset(
        nvtxs as size_t,
        0 as libc::c_int,
        gk_i32malloc(
            nvtxs as size_t,
            b"gk_graph_ComputeBFSOrdering: pos\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        ),
    );
    cot = gk_i32incset(
        nvtxs as size_t,
        0 as libc::c_int,
        gk_i32malloc(
            nvtxs as size_t,
            b"gk_graph_ComputeBFSOrdering: cot\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        ),
    );
    let ref mut fresh6 = *cot.offset(0 as libc::c_int as isize);
    *fresh6 = v;
    *pos.offset(0 as libc::c_int as isize) = *fresh6;
    let ref mut fresh7 = *cot.offset(v as isize);
    *fresh7 = 0 as libc::c_int;
    *pos.offset(v as isize) = *fresh7;
    last = 0 as libc::c_int;
    first = last;
    while first < nvtxs {
        if first == last {
            k = *cot.offset(last as isize);
            *pos.offset(k as isize) = -(1 as libc::c_int);
            last += 1;
            last;
        }
        let fresh8 = first;
        first = first + 1;
        i = *cot.offset(fresh8 as isize);
        j = *xadj.offset(i as isize);
        while j < *xadj.offset((i + 1 as libc::c_int) as isize) {
            k = *adjncy.offset(j as isize);
            if *pos.offset(k as isize) != -(1 as libc::c_int) {
                *cot
                    .offset(
                        *pos.offset(k as isize) as isize,
                    ) = *cot.offset(last as isize);
                *pos
                    .offset(
                        *cot.offset(last as isize) as isize,
                    ) = *pos.offset(k as isize);
                let fresh9 = last;
                last = last + 1;
                *cot.offset(fresh9 as isize) = k;
                *pos.offset(k as isize) = -(1 as libc::c_int);
            }
            j += 1;
            j;
        }
    }
    if !r_perm.is_null() {
        i = 0 as libc::c_int;
        while i < nvtxs {
            *pos.offset(*cot.offset(i as isize) as isize) = i;
            i += 1;
            i;
        }
        *r_perm = pos;
        pos = 0 as *mut int32_t;
    }
    if !r_iperm.is_null() {
        *r_iperm = cot;
        cot = 0 as *mut int32_t;
    }
    gk_free(
        &mut pos as *mut *mut int32_t as *mut *mut libc::c_void,
        &mut cot as *mut *mut int32_t,
        0 as *mut *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gk_graph_ComputeBestFOrdering0(
    mut graph: *mut gk_graph_t,
    mut v: libc::c_int,
    mut type_0: libc::c_int,
    mut r_perm: *mut *mut int32_t,
    mut r_iperm: *mut *mut int32_t,
) {
    let mut j: ssize_t = 0;
    let mut jj: ssize_t = 0;
    let mut xadj: *mut ssize_t = 0 as *mut ssize_t;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut u: libc::c_int = 0;
    let mut nvtxs: libc::c_int = 0;
    let mut adjncy: *mut int32_t = 0 as *mut int32_t;
    let mut perm: *mut int32_t = 0 as *mut int32_t;
    let mut degrees: *mut int32_t = 0 as *mut int32_t;
    let mut minIDs: *mut int32_t = 0 as *mut int32_t;
    let mut open: *mut int32_t = 0 as *mut int32_t;
    let mut queue: *mut gk_i32pq_t = 0 as *mut gk_i32pq_t;
    if (*graph).nvtxs <= 0 as libc::c_int {
        return;
    }
    nvtxs = (*graph).nvtxs;
    xadj = (*graph).xadj;
    adjncy = (*graph).adjncy;
    degrees = gk_i32smalloc(
        nvtxs as size_t,
        0 as libc::c_int,
        b"gk_graph_ComputeBestFOrdering: degrees\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    minIDs = gk_i32smalloc(
        nvtxs as size_t,
        nvtxs + 1 as libc::c_int,
        b"gk_graph_ComputeBestFOrdering: minIDs\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    open = gk_i32malloc(
        nvtxs as size_t,
        b"gk_graph_ComputeBestFOrdering: open\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    perm = gk_i32smalloc(
        nvtxs as size_t,
        -(1 as libc::c_int),
        b"gk_graph_ComputeBestFOrdering: perm\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    queue = gk_i32pqCreate(nvtxs as size_t);
    i = 0 as libc::c_int;
    while i < nvtxs {
        gk_i32pqInsert(queue, i as gk_idx_t, 0 as libc::c_int);
        i += 1;
        i;
    }
    gk_i32pqUpdate(queue, v as gk_idx_t, 1 as libc::c_int);
    *open.offset(0 as libc::c_int as isize) = v;
    i = 0 as libc::c_int;
    while i < nvtxs {
        v = gk_i32pqGetTop(queue) as libc::c_int;
        if v == -(1 as libc::c_int) {
            gk_errexit(
                15 as libc::c_int,
                b"The priority queue got empty ahead of time [i=%d].\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                i,
            );
        }
        if *perm.offset(v as isize) != -(1 as libc::c_int) {
            gk_errexit(
                15 as libc::c_int,
                b"The perm[%d] has already been set.\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                v,
            );
        }
        *perm.offset(v as isize) = i;
        j = *xadj.offset(v as isize);
        while j < *xadj.offset((v + 1 as libc::c_int) as isize) {
            u = *adjncy.offset(j as isize);
            if *perm.offset(u as isize) == -(1 as libc::c_int) {
                let ref mut fresh10 = *degrees.offset(u as isize);
                *fresh10 += 1;
                *fresh10;
                *minIDs
                    .offset(
                        u as isize,
                    ) = if i < *minIDs.offset(u as isize) {
                    i
                } else {
                    *minIDs.offset(u as isize)
                };
                match type_0 {
                    1 => {
                        gk_i32pqUpdate(queue, u as gk_idx_t, 1 as libc::c_int);
                    }
                    2 => {
                        gk_i32pqUpdate(
                            queue,
                            u as gk_idx_t,
                            *degrees.offset(u as isize),
                        );
                    }
                    3 => {
                        k = 0 as libc::c_int;
                        jj = *xadj.offset(u as isize);
                        while jj < *xadj.offset((u + 1 as libc::c_int) as isize) {
                            if *perm.offset(*adjncy.offset(jj as isize) as isize)
                                != -(1 as libc::c_int)
                            {
                                k += *perm.offset(*adjncy.offset(jj as isize) as isize);
                            }
                            jj += 1;
                            jj;
                        }
                        gk_i32pqUpdate(queue, u as gk_idx_t, k);
                    }
                    4 => {
                        k = 0 as libc::c_int;
                        jj = *xadj.offset(u as isize);
                        while jj < *xadj.offset((u + 1 as libc::c_int) as isize) {
                            if *perm.offset(*adjncy.offset(jj as isize) as isize)
                                != -(1 as libc::c_int)
                            {
                                k += i - *perm.offset(*adjncy.offset(jj as isize) as isize);
                            }
                            jj += 1;
                            jj;
                        }
                        gk_i32pqUpdate(queue, u as gk_idx_t, k);
                    }
                    _ => {}
                }
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    if !r_perm.is_null() {
        *r_perm = perm;
        perm = 0 as *mut int32_t;
    }
    if !r_iperm.is_null() {
        i = 0 as libc::c_int;
        while i < nvtxs {
            *degrees.offset(*perm.offset(i as isize) as isize) = i;
            i += 1;
            i;
        }
        *r_iperm = degrees;
        degrees = 0 as *mut int32_t;
    }
    gk_i32pqDestroy(queue);
    gk_free(
        &mut perm as *mut *mut int32_t as *mut *mut libc::c_void,
        &mut degrees as *mut *mut int32_t,
        &mut minIDs as *mut *mut int32_t,
        &mut open as *mut *mut int32_t,
        0 as *mut *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gk_graph_ComputeBestFOrdering(
    mut graph: *mut gk_graph_t,
    mut v: libc::c_int,
    mut type_0: libc::c_int,
    mut r_perm: *mut *mut int32_t,
    mut r_iperm: *mut *mut int32_t,
) {
    let mut j: ssize_t = 0;
    let mut jj: ssize_t = 0;
    let mut xadj: *mut ssize_t = 0 as *mut ssize_t;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut u: libc::c_int = 0;
    let mut nvtxs: libc::c_int = 0;
    let mut nopen: libc::c_int = 0;
    let mut ntodo: libc::c_int = 0;
    let mut adjncy: *mut int32_t = 0 as *mut int32_t;
    let mut perm: *mut int32_t = 0 as *mut int32_t;
    let mut degrees: *mut int32_t = 0 as *mut int32_t;
    let mut wdegrees: *mut int32_t = 0 as *mut int32_t;
    let mut sod: *mut int32_t = 0 as *mut int32_t;
    let mut level: *mut int32_t = 0 as *mut int32_t;
    let mut ot: *mut int32_t = 0 as *mut int32_t;
    let mut pos: *mut int32_t = 0 as *mut int32_t;
    let mut queue: *mut gk_i32pq_t = 0 as *mut gk_i32pq_t;
    if (*graph).nvtxs <= 0 as libc::c_int {
        return;
    }
    nvtxs = (*graph).nvtxs;
    xadj = (*graph).xadj;
    adjncy = (*graph).adjncy;
    degrees = gk_i32smalloc(
        nvtxs as size_t,
        0 as libc::c_int,
        b"gk_graph_ComputeBestFOrdering: degrees\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    wdegrees = gk_i32smalloc(
        nvtxs as size_t,
        0 as libc::c_int,
        b"gk_graph_ComputeBestFOrdering: wdegrees\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    sod = gk_i32smalloc(
        nvtxs as size_t,
        0 as libc::c_int,
        b"gk_graph_ComputeBestFOrdering: sod\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    level = gk_i32smalloc(
        nvtxs as size_t,
        0 as libc::c_int,
        b"gk_graph_ComputeBestFOrdering: level\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    ot = gk_i32incset(
        nvtxs as size_t,
        0 as libc::c_int,
        gk_i32malloc(
            nvtxs as size_t,
            b"gk_graph_FindComponents: ot\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        ),
    );
    pos = gk_i32incset(
        nvtxs as size_t,
        0 as libc::c_int,
        gk_i32malloc(
            nvtxs as size_t,
            b"gk_graph_FindComponents: pos\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        ),
    );
    perm = gk_i32smalloc(
        nvtxs as size_t,
        -(1 as libc::c_int),
        b"gk_graph_ComputeBestFOrdering: perm\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    queue = gk_i32pqCreate(nvtxs as size_t);
    gk_i32pqInsert(queue, v as gk_idx_t, 1 as libc::c_int);
    let ref mut fresh11 = *ot.offset(0 as libc::c_int as isize);
    *fresh11 = v;
    *pos.offset(0 as libc::c_int as isize) = *fresh11;
    let ref mut fresh12 = *ot.offset(v as isize);
    *fresh12 = 0 as libc::c_int;
    *pos.offset(v as isize) = *fresh12;
    nopen = 1 as libc::c_int;
    ntodo = nvtxs;
    i = 0 as libc::c_int;
    while i < nvtxs {
        if nopen == 0 as libc::c_int {
            gk_i32pqInsert(
                queue,
                *ot.offset(0 as libc::c_int as isize) as gk_idx_t,
                1 as libc::c_int,
            );
            nopen += 1;
            nopen;
        }
        v = gk_i32pqGetTop(queue) as libc::c_int;
        if v == -(1 as libc::c_int) {
            gk_errexit(
                15 as libc::c_int,
                b"The priority queue got empty ahead of time [i=%d].\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                i,
            );
        }
        if *perm.offset(v as isize) != -(1 as libc::c_int) {
            gk_errexit(
                15 as libc::c_int,
                b"The perm[%d] has already been set.\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                v,
            );
        }
        *perm.offset(v as isize) = i;
        if *ot.offset(*pos.offset(v as isize) as isize) != v {
            gk_errexit(
                15 as libc::c_int,
                b"Something went wrong [ot[pos[%d]]!=%d.\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                v,
                v,
            );
        }
        if *pos.offset(v as isize) >= nopen {
            gk_errexit(
                15 as libc::c_int,
                b"The position of v is not in open list. pos[%d]=%d is >=%d.\n\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
                v,
                *pos.offset(v as isize),
                nopen,
            );
        }
        *ot
            .offset(
                *pos.offset(v as isize) as isize,
            ) = *ot.offset((nopen - 1 as libc::c_int) as isize);
        *pos
            .offset(
                *ot.offset((nopen - 1 as libc::c_int) as isize) as isize,
            ) = *pos.offset(v as isize);
        if ntodo > nopen {
            *ot
                .offset(
                    (nopen - 1 as libc::c_int) as isize,
                ) = *ot.offset((ntodo - 1 as libc::c_int) as isize);
            *pos
                .offset(
                    *ot.offset((ntodo - 1 as libc::c_int) as isize) as isize,
                ) = nopen - 1 as libc::c_int;
        }
        nopen -= 1;
        nopen;
        ntodo -= 1;
        ntodo;
        j = *xadj.offset(v as isize);
        while j < *xadj.offset((v + 1 as libc::c_int) as isize) {
            u = *adjncy.offset(j as isize);
            if *perm.offset(u as isize) == -(1 as libc::c_int) {
                if *degrees.offset(u as isize) == 0 as libc::c_int {
                    *ot
                        .offset(
                            *pos.offset(u as isize) as isize,
                        ) = *ot.offset(nopen as isize);
                    *pos
                        .offset(
                            *ot.offset(nopen as isize) as isize,
                        ) = *pos.offset(u as isize);
                    *ot.offset(nopen as isize) = u;
                    *pos.offset(u as isize) = nopen;
                    nopen += 1;
                    nopen;
                    *level
                        .offset(
                            u as isize,
                        ) = *level.offset(v as isize) + 1 as libc::c_int;
                    gk_i32pqInsert(queue, u as gk_idx_t, 0 as libc::c_int);
                }
                let ref mut fresh13 = *degrees.offset(u as isize);
                *fresh13 += 1;
                *fresh13;
                match type_0 {
                    1 => {
                        gk_i32pqUpdate(
                            queue,
                            u as gk_idx_t,
                            1000 as libc::c_int * (i + 1 as libc::c_int)
                                + *degrees.offset(u as isize),
                        );
                    }
                    2 => {
                        gk_i32pqUpdate(
                            queue,
                            u as gk_idx_t,
                            *degrees.offset(u as isize),
                        );
                    }
                    3 => {
                        let ref mut fresh14 = *wdegrees.offset(u as isize);
                        *fresh14 += i;
                        gk_i32pqUpdate(
                            queue,
                            u as gk_idx_t,
                            *wdegrees.offset(u as isize),
                        );
                    }
                    5 => {
                        gk_i32pqUpdate(
                            queue,
                            u as gk_idx_t,
                            -(1000 as libc::c_int * *level.offset(u as isize)
                                - *degrees.offset(u as isize)),
                        );
                    }
                    6 => {
                        gk_i32pqUpdate(
                            queue,
                            u as gk_idx_t,
                            (i + 1 as libc::c_int) * *degrees.offset(u as isize),
                        );
                    }
                    4 | _ => {}
                }
            }
            j += 1;
            j;
        }
        if type_0 == 4 as libc::c_int {
            j = 0 as libc::c_int as ssize_t;
            while j < nopen as i64 {
                u = *ot.offset(j as isize);
                if *perm.offset(u as isize) != -(1 as libc::c_int) {
                    gk_errexit(
                        15 as libc::c_int,
                        b"For i=%d, the open list contains a closed vertex: ot[%zd]=%d, perm[%d]=%d.\n\0"
                            as *const u8 as *const libc::c_char as *mut libc::c_char,
                        i,
                        j,
                        u,
                        u,
                        *perm.offset(u as isize),
                    );
                }
                let ref mut fresh15 = *sod.offset(u as isize);
                *fresh15 += *degrees.offset(u as isize);
                if i < 1000 as libc::c_int || i % 25 as libc::c_int == 0 as libc::c_int {
                    gk_i32pqUpdate(queue, u as gk_idx_t, *sod.offset(u as isize));
                }
                j += 1;
                j;
            }
        }
        i += 1;
        i;
    }
    if !r_perm.is_null() {
        *r_perm = perm;
        perm = 0 as *mut int32_t;
    }
    if !r_iperm.is_null() {
        i = 0 as libc::c_int;
        while i < nvtxs {
            *degrees.offset(*perm.offset(i as isize) as isize) = i;
            i += 1;
            i;
        }
        *r_iperm = degrees;
        degrees = 0 as *mut int32_t;
    }
    gk_i32pqDestroy(queue);
    gk_free(
        &mut perm as *mut *mut int32_t as *mut *mut libc::c_void,
        &mut degrees as *mut *mut int32_t,
        &mut wdegrees as *mut *mut int32_t,
        &mut sod as *mut *mut int32_t,
        &mut ot as *mut *mut int32_t,
        &mut pos as *mut *mut int32_t,
        &mut level as *mut *mut int32_t,
        0 as *mut *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gk_graph_SingleSourceShortestPaths(
    mut graph: *mut gk_graph_t,
    mut v: libc::c_int,
    mut r_sps: *mut *mut libc::c_void,
) {
    let mut xadj: *mut ssize_t = 0 as *mut ssize_t;
    let mut i: libc::c_int = 0;
    let mut u: libc::c_int = 0;
    let mut nvtxs: libc::c_int = 0;
    let mut adjncy: *mut int32_t = 0 as *mut int32_t;
    let mut inqueue: *mut int32_t = 0 as *mut int32_t;
    if (*graph).nvtxs <= 0 as libc::c_int {
        return;
    }
    nvtxs = (*graph).nvtxs;
    xadj = (*graph).xadj;
    adjncy = (*graph).adjncy;
    inqueue = gk_i32smalloc(
        nvtxs as size_t,
        0 as libc::c_int,
        b"gk_graph_SingleSourceShortestPaths: inqueue\0" as *const u8
            as *const libc::c_char as *mut libc::c_char,
    );
    if !((*graph).iadjwgt).is_null() {
        let mut queue: *mut gk_i32pq_t = 0 as *mut gk_i32pq_t;
        let mut adjwgt: *mut int32_t = 0 as *mut int32_t;
        let mut sps: *mut int32_t = 0 as *mut int32_t;
        adjwgt = (*graph).iadjwgt;
        queue = gk_i32pqCreate(nvtxs as size_t);
        gk_i32pqInsert(queue, v as gk_idx_t, 0 as libc::c_int);
        *inqueue.offset(v as isize) = 1 as libc::c_int;
        sps = gk_i32smalloc(
            nvtxs as size_t,
            -(1 as libc::c_int),
            b"gk_graph_SingleSourceShortestPaths: sps\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
        *sps.offset(v as isize) = 0 as libc::c_int;
        loop {
            v = gk_i32pqGetTop(queue) as libc::c_int;
            if !(v != -(1 as libc::c_int)) {
                break;
            }
            *inqueue.offset(v as isize) = 2 as libc::c_int;
            i = *xadj.offset(v as isize) as libc::c_int;
            while (i as i64) < *xadj.offset((v + 1 as libc::c_int) as isize) {
                u = *adjncy.offset(i as isize);
                if !(*inqueue.offset(u as isize) == 2 as libc::c_int) {
                    if *sps.offset(u as isize) < 0 as libc::c_int
                        || *sps.offset(v as isize) + *adjwgt.offset(i as isize)
                            < *sps.offset(u as isize)
                    {
                        *sps
                            .offset(
                                u as isize,
                            ) = *sps.offset(v as isize) + *adjwgt.offset(i as isize);
                        if *inqueue.offset(u as isize) != 0 {
                            gk_i32pqUpdate(
                                queue,
                                u as gk_idx_t,
                                -*sps.offset(u as isize),
                            );
                        } else {
                            gk_i32pqInsert(
                                queue,
                                u as gk_idx_t,
                                -*sps.offset(u as isize),
                            );
                            *inqueue.offset(u as isize) = 1 as libc::c_int;
                        }
                    }
                }
                i += 1;
                i;
            }
        }
        *r_sps = sps as *mut libc::c_void;
        gk_i32pqDestroy(queue);
    } else {
        let mut queue_0: *mut gk_fpq_t = 0 as *mut gk_fpq_t;
        let mut adjwgt_0: *mut libc::c_float = 0 as *mut libc::c_float;
        let mut sps_0: *mut libc::c_float = 0 as *mut libc::c_float;
        adjwgt_0 = (*graph).fadjwgt;
        queue_0 = gk_fpqCreate(nvtxs as size_t);
        gk_fpqInsert(queue_0, v as gk_idx_t, 0 as libc::c_int as libc::c_float);
        *inqueue.offset(v as isize) = 1 as libc::c_int;
        sps_0 = gk_fsmalloc(
            nvtxs as size_t,
            -(1 as libc::c_int) as libc::c_float,
            b"gk_graph_SingleSourceShortestPaths: sps\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
        *sps_0.offset(v as isize) = 0 as libc::c_int as libc::c_float;
        loop {
            v = gk_fpqGetTop(queue_0) as libc::c_int;
            if !(v != -(1 as libc::c_int)) {
                break;
            }
            *inqueue.offset(v as isize) = 2 as libc::c_int;
            i = *xadj.offset(v as isize) as libc::c_int;
            while (i as i64) < *xadj.offset((v + 1 as libc::c_int) as isize) {
                u = *adjncy.offset(i as isize);
                if !(*inqueue.offset(u as isize) == 2 as libc::c_int) {
                    if *sps_0.offset(u as isize) < 0 as libc::c_int as libc::c_float
                        || *sps_0.offset(v as isize) + *adjwgt_0.offset(i as isize)
                            < *sps_0.offset(u as isize)
                    {
                        *sps_0
                            .offset(
                                u as isize,
                            ) = *sps_0.offset(v as isize) + *adjwgt_0.offset(i as isize);
                        if *inqueue.offset(u as isize) != 0 {
                            gk_fpqUpdate(
                                queue_0,
                                u as gk_idx_t,
                                -*sps_0.offset(u as isize),
                            );
                        } else {
                            gk_fpqInsert(
                                queue_0,
                                u as gk_idx_t,
                                -*sps_0.offset(u as isize),
                            );
                            *inqueue.offset(u as isize) = 1 as libc::c_int;
                        }
                    }
                }
                i += 1;
                i;
            }
        }
        *r_sps = sps_0 as *mut libc::c_void;
        gk_fpqDestroy(queue_0);
    }
    gk_free(
        &mut inqueue as *mut *mut int32_t as *mut *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
}
