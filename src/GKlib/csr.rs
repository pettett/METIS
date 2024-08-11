use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn log(_: libc::c_double) -> libc::c_double;
    fn powf(_: libc::c_float, _: libc::c_float) -> libc::c_float;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    static mut stdout: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: u64,
        _: u64,
        _: *mut FILE,
    ) -> u64;
    fn fwrite(
        _: *const libc::c_void,
        _: u64,
        _: u64,
        _: *mut FILE,
    ) -> u64;
    fn strtof(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_float;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> i64;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gk_imax(n: size_t, x: *mut libc::c_int) -> libc::c_int;
    fn gk_fsum(n: size_t, x: *mut libc::c_float, incx: size_t) -> libc::c_float;
    fn gk_fdot(
        n: size_t,
        x: *mut libc::c_float,
        incx: size_t,
        y: *mut libc::c_float,
        incy: size_t,
    ) -> libc::c_float;
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
    fn gk_getfilestats(
        fname: *mut libc::c_char,
        r_nlines: *mut size_t,
        r_ntokens: *mut size_t,
        r_max_nlntokens: *mut size_t,
        r_nbytes: *mut size_t,
    );
    fn gk_imalloc(n: size_t, msg: *mut libc::c_char) -> *mut libc::c_int;
    fn gk_ismalloc(
        n: size_t,
        ival: libc::c_int,
        msg: *mut libc::c_char,
    ) -> *mut libc::c_int;
    fn gk_icopy(n: size_t, a: *mut libc::c_int, b: *mut libc::c_int) -> *mut libc::c_int;
    fn gk_zmalloc(n: size_t, msg: *mut libc::c_char) -> *mut ssize_t;
    fn gk_zsmalloc(n: size_t, ival: ssize_t, msg: *mut libc::c_char) -> *mut ssize_t;
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
    fn gk_ikvmalloc(n: size_t, msg: *mut libc::c_char) -> *mut gk_ikv_t;
    fn gk_fkvmalloc(n: size_t, msg: *mut libc::c_char) -> *mut gk_fkv_t;
    fn gk_fkvcopy(n: size_t, a: *mut gk_fkv_t, b: *mut gk_fkv_t) -> *mut gk_fkv_t;
    fn gk_malloc(nbytes: size_t, msg: *mut libc::c_char) -> *mut libc::c_void;
    fn gk_free(ptr1: *mut *mut libc::c_void, _: ...);
    fn errexit(_: *mut libc::c_char, _: ...);
    fn gk_errexit(signum: libc::c_int, _: *mut libc::c_char, _: ...);
    fn gk_ikvsorti(_: size_t, _: *mut gk_ikv_t);
    fn gk_ikvsortd(_: size_t, _: *mut gk_ikv_t);
    fn gk_fkvsortd(_: size_t, _: *mut gk_fkv_t);
    fn gk_dfkvkselect(_: size_t, _: libc::c_int, _: *mut gk_fkv_t) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: u64,
    ) -> *mut libc::c_void;
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
pub struct gk_ikv_t {
    pub key: libc::c_int,
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
pub struct gk_csr_t {
    pub nrows: int32_t,
    pub ncols: int32_t,
    pub rowptr: *mut ssize_t,
    pub colptr: *mut ssize_t,
    pub rowind: *mut int32_t,
    pub colind: *mut int32_t,
    pub rowids: *mut int32_t,
    pub colids: *mut int32_t,
    pub rowval: *mut libc::c_float,
    pub colval: *mut libc::c_float,
    pub rnorms: *mut libc::c_float,
    pub cnorms: *mut libc::c_float,
    pub rsums: *mut libc::c_float,
    pub csums: *mut libc::c_float,
    pub rsizes: *mut libc::c_float,
    pub csizes: *mut libc::c_float,
    pub rvols: *mut libc::c_float,
    pub cvols: *mut libc::c_float,
    pub rwgts: *mut libc::c_float,
    pub cwgts: *mut libc::c_float,
}
#[no_mangle]
pub unsafe extern "C" fn gk_csr_Create() -> *mut gk_csr_t {
    let mut mat: *mut gk_csr_t = 0 as *mut gk_csr_t;
    mat = gk_malloc(
        ::core::mem::size_of::<gk_csr_t>() as u64,
        b"gk_csr_Create: mat\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut gk_csr_t;
    gk_csr_Init(mat);
    return mat;
}
#[no_mangle]
pub unsafe extern "C" fn gk_csr_Init(mut mat: *mut gk_csr_t) {
    memset(
        mat as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<gk_csr_t>() as u64,
    );
    (*mat).ncols = -(1);
    (*mat).nrows = (*mat).ncols;
}
#[no_mangle]
pub unsafe extern "C" fn gk_csr_Free(mut mat: *mut *mut gk_csr_t) {
    if (*mat).is_null() {
        return;
    }
    gk_csr_FreeContents(*mat);
    gk_free(mat as *mut *mut libc::c_void, 0 as *mut *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn gk_csr_FreeContents(mut mat: *mut gk_csr_t) {
    gk_free(
        &mut (*mat).rowptr as *mut *mut ssize_t as *mut libc::c_void
            as *mut *mut libc::c_void,
        &mut (*mat).rowind as *mut *mut int32_t,
        &mut (*mat).rowval as *mut *mut libc::c_float,
        &mut (*mat).rowids as *mut *mut int32_t,
        &mut (*mat).colptr as *mut *mut ssize_t,
        &mut (*mat).colind as *mut *mut int32_t,
        &mut (*mat).colval as *mut *mut libc::c_float,
        &mut (*mat).colids as *mut *mut int32_t,
        &mut (*mat).rnorms as *mut *mut libc::c_float,
        &mut (*mat).cnorms as *mut *mut libc::c_float,
        &mut (*mat).rsums as *mut *mut libc::c_float,
        &mut (*mat).csums as *mut *mut libc::c_float,
        &mut (*mat).rsizes as *mut *mut libc::c_float,
        &mut (*mat).csizes as *mut *mut libc::c_float,
        &mut (*mat).rvols as *mut *mut libc::c_float,
        &mut (*mat).cvols as *mut *mut libc::c_float,
        &mut (*mat).rwgts as *mut *mut libc::c_float,
        &mut (*mat).cwgts as *mut *mut libc::c_float,
        0 as *mut *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gk_csr_Dup(mut mat: *mut gk_csr_t) -> *mut gk_csr_t {
    let mut nmat: *mut gk_csr_t = 0 as *mut gk_csr_t;
    nmat = gk_csr_Create();
    (*nmat).nrows = (*mat).nrows;
    (*nmat).ncols = (*mat).ncols;
    if !((*mat).rowptr).is_null() {
        (*nmat)
            .rowptr = gk_zcopy(
            ((*mat).nrows + 1) as size_t,
            (*mat).rowptr,
            gk_zmalloc(
                ((*mat).nrows + 1) as size_t,
                b"gk_csr_Dup: rowptr\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ),
        );
    }
    if !((*mat).rowids).is_null() {
        (*nmat)
            .rowids = gk_icopy(
            (*mat).nrows as size_t,
            (*mat).rowids,
            gk_imalloc(
                (*mat).nrows as size_t,
                b"gk_csr_Dup: rowids\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ),
        );
    }
    if !((*mat).rnorms).is_null() {
        (*nmat)
            .rnorms = gk_fcopy(
            (*mat).nrows as size_t,
            (*mat).rnorms,
            gk_fmalloc(
                (*mat).nrows as size_t,
                b"gk_csr_Dup: rnorms\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ),
        );
    }
    if !((*mat).rowind).is_null() {
        (*nmat)
            .rowind = gk_icopy(
            *((*mat).rowptr).offset((*mat).nrows as isize) as size_t,
            (*mat).rowind,
            gk_imalloc(
                *((*mat).rowptr).offset((*mat).nrows as isize) as size_t,
                b"gk_csr_Dup: rowind\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ),
        );
    }
    if !((*mat).rowval).is_null() {
        (*nmat)
            .rowval = gk_fcopy(
            *((*mat).rowptr).offset((*mat).nrows as isize) as size_t,
            (*mat).rowval,
            gk_fmalloc(
                *((*mat).rowptr).offset((*mat).nrows as isize) as size_t,
                b"gk_csr_Dup: rowval\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ),
        );
    }
    if !((*mat).colptr).is_null() {
        (*nmat)
            .colptr = gk_zcopy(
            ((*mat).ncols + 1) as size_t,
            (*mat).colptr,
            gk_zmalloc(
                ((*mat).ncols + 1) as size_t,
                b"gk_csr_Dup: colptr\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ),
        );
    }
    if !((*mat).colids).is_null() {
        (*nmat)
            .colids = gk_icopy(
            (*mat).ncols as size_t,
            (*mat).colids,
            gk_imalloc(
                (*mat).ncols as size_t,
                b"gk_csr_Dup: colids\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ),
        );
    }
    if !((*mat).cnorms).is_null() {
        (*nmat)
            .cnorms = gk_fcopy(
            (*mat).ncols as size_t,
            (*mat).cnorms,
            gk_fmalloc(
                (*mat).ncols as size_t,
                b"gk_csr_Dup: cnorms\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ),
        );
    }
    if !((*mat).colind).is_null() {
        (*nmat)
            .colind = gk_icopy(
            *((*mat).colptr).offset((*mat).ncols as isize) as size_t,
            (*mat).colind,
            gk_imalloc(
                *((*mat).colptr).offset((*mat).ncols as isize) as size_t,
                b"gk_csr_Dup: colind\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ),
        );
    }
    if !((*mat).colval).is_null() {
        (*nmat)
            .colval = gk_fcopy(
            *((*mat).colptr).offset((*mat).ncols as isize) as size_t,
            (*mat).colval,
            gk_fmalloc(
                *((*mat).colptr).offset((*mat).ncols as isize) as size_t,
                b"gk_csr_Dup: colval\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ),
        );
    }
    return nmat;
}
#[no_mangle]
pub unsafe extern "C" fn gk_csr_ExtractSubmatrix(
    mut mat: *mut gk_csr_t,
    mut rstart: libc::c_int,
    mut nrows: libc::c_int,
) -> *mut gk_csr_t {
    let mut i: ssize_t = 0;
    let mut nmat: *mut gk_csr_t = 0 as *mut gk_csr_t;
    if rstart + nrows > (*mat).nrows {
        return 0 as *mut gk_csr_t;
    }
    nmat = gk_csr_Create();
    (*nmat).nrows = nrows;
    (*nmat).ncols = (*mat).ncols;
    if !((*mat).rowptr).is_null() {
        (*nmat)
            .rowptr = gk_zcopy(
            (nrows + 1) as size_t,
            ((*mat).rowptr).offset(rstart as isize),
            gk_zmalloc(
                (nrows + 1) as size_t,
                b"gk_csr_ExtractSubmatrix: rowptr\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ),
        );
    }
    i = nrows as ssize_t;
    while i >= 0 as libc::c_int as i64 {
        let ref mut fresh0 = *((*nmat).rowptr).offset(i as isize);
        *fresh0 -= *((*nmat).rowptr).offset(0 as libc::c_int as isize);
        i -= 1;
        i;
    }
    if !((*mat).rowids).is_null() {
        (*nmat)
            .rowids = gk_icopy(
            nrows as size_t,
            ((*mat).rowids).offset(rstart as isize),
            gk_imalloc(
                nrows as size_t,
                b"gk_csr_ExtractSubmatrix: rowids\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ),
        );
    }
    if !((*mat).rnorms).is_null() {
        (*nmat)
            .rnorms = gk_fcopy(
            nrows as size_t,
            ((*mat).rnorms).offset(rstart as isize),
            gk_fmalloc(
                nrows as size_t,
                b"gk_csr_ExtractSubmatrix: rnorms\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ),
        );
    }
    if !((*mat).rsums).is_null() {
        (*nmat)
            .rsums = gk_fcopy(
            nrows as size_t,
            ((*mat).rsums).offset(rstart as isize),
            gk_fmalloc(
                nrows as size_t,
                b"gk_csr_ExtractSubmatrix: rsums\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ),
        );
    }
    if !((*mat).rowind).is_null() {
        (*nmat)
            .rowind = gk_icopy(
            (*((*mat).rowptr).offset((rstart + nrows) as isize)
                - *((*mat).rowptr).offset(rstart as isize)) as size_t,
            ((*mat).rowind).offset(*((*mat).rowptr).offset(rstart as isize) as isize),
            gk_imalloc(
                (*((*mat).rowptr).offset((rstart + nrows) as isize)
                    - *((*mat).rowptr).offset(rstart as isize)) as size_t,
                b"gk_csr_ExtractSubmatrix: rowind\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ),
        );
    }
    if !((*mat).rowval).is_null() {
        (*nmat)
            .rowval = gk_fcopy(
            (*((*mat).rowptr).offset((rstart + nrows) as isize)
                - *((*mat).rowptr).offset(rstart as isize)) as size_t,
            ((*mat).rowval).offset(*((*mat).rowptr).offset(rstart as isize) as isize),
            gk_fmalloc(
                (*((*mat).rowptr).offset((rstart + nrows) as isize)
                    - *((*mat).rowptr).offset(rstart as isize)) as size_t,
                b"gk_csr_ExtractSubmatrix: rowval\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ),
        );
    }
    return nmat;
}
#[no_mangle]
pub unsafe extern "C" fn gk_csr_ExtractRows(
    mut mat: *mut gk_csr_t,
    mut nrows: libc::c_int,
    mut rind: *mut libc::c_int,
) -> *mut gk_csr_t {
    let mut i: ssize_t = 0;
    let mut ii: ssize_t = 0;
    let mut j: ssize_t = 0;
    let mut nnz: ssize_t = 0;
    let mut nmat: *mut gk_csr_t = 0 as *mut gk_csr_t;
    nmat = gk_csr_Create();
    (*nmat).nrows = nrows;
    (*nmat).ncols = (*mat).ncols;
    nnz = 0 as libc::c_int as ssize_t;
    i = 0 as libc::c_int as ssize_t;
    while i < nrows as i64 {
        nnz
            += *((*mat).rowptr)
                .offset((*rind.offset(i as isize) + 1) as isize)
                - *((*mat).rowptr).offset(*rind.offset(i as isize) as isize);
        i += 1;
        i;
    }
    (*nmat)
        .rowptr = gk_zmalloc(
        ((*nmat).nrows + 1) as size_t,
        b"gk_csr_ExtractPartition: rowptr\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*nmat)
        .rowind = gk_imalloc(
        nnz as size_t,
        b"gk_csr_ExtractPartition: rowind\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*nmat)
        .rowval = gk_fmalloc(
        nnz as size_t,
        b"gk_csr_ExtractPartition: rowval\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    *((*nmat).rowptr).offset(0 as libc::c_int as isize) = 0 as libc::c_int as ssize_t;
    nnz = 0 as libc::c_int as ssize_t;
    j = 0 as libc::c_int as ssize_t;
    ii = 0 as libc::c_int as ssize_t;
    while ii < nrows as i64 {
        i = *rind.offset(ii as isize) as ssize_t;
        gk_icopy(
            (*((*mat).rowptr).offset((i + 1 as i64) as isize)
                - *((*mat).rowptr).offset(i as isize)) as size_t,
            ((*mat).rowind).offset(*((*mat).rowptr).offset(i as isize) as isize),
            ((*nmat).rowind).offset(nnz as isize),
        );
        gk_fcopy(
            (*((*mat).rowptr).offset((i + 1 as i64) as isize)
                - *((*mat).rowptr).offset(i as isize)) as size_t,
            ((*mat).rowval).offset(*((*mat).rowptr).offset(i as isize) as isize),
            ((*nmat).rowval).offset(nnz as isize),
        );
        nnz
            += *((*mat).rowptr).offset((i + 1 as i64) as isize)
                - *((*mat).rowptr).offset(i as isize);
        j += 1;
        *((*nmat).rowptr).offset(j as isize) = nnz;
        ii += 1;
        ii;
    }
    return nmat;
}
#[no_mangle]
pub unsafe extern "C" fn gk_csr_ExtractPartition(
    mut mat: *mut gk_csr_t,
    mut part: *mut libc::c_int,
    mut pid: libc::c_int,
) -> *mut gk_csr_t {
    let mut i: ssize_t = 0;
    let mut j: ssize_t = 0;
    let mut nnz: ssize_t = 0;
    let mut nmat: *mut gk_csr_t = 0 as *mut gk_csr_t;
    nmat = gk_csr_Create();
    (*nmat).nrows = 0 as libc::c_int;
    (*nmat).ncols = (*mat).ncols;
    nnz = 0 as libc::c_int as ssize_t;
    i = 0 as libc::c_int as ssize_t;
    while i < (*mat).nrows as i64 {
        if *part.offset(i as isize) == pid {
            (*nmat).nrows += 1;
            (*nmat).nrows;
            nnz
                += *((*mat).rowptr)
                    .offset((i + 1 as i64) as isize)
                    - *((*mat).rowptr).offset(i as isize);
        }
        i += 1;
        i;
    }
    (*nmat)
        .rowptr = gk_zmalloc(
        ((*nmat).nrows + 1) as size_t,
        b"gk_csr_ExtractPartition: rowptr\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*nmat)
        .rowind = gk_imalloc(
        nnz as size_t,
        b"gk_csr_ExtractPartition: rowind\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*nmat)
        .rowval = gk_fmalloc(
        nnz as size_t,
        b"gk_csr_ExtractPartition: rowval\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    *((*nmat).rowptr).offset(0 as libc::c_int as isize) = 0 as libc::c_int as ssize_t;
    nnz = 0 as libc::c_int as ssize_t;
    j = 0 as libc::c_int as ssize_t;
    i = 0 as libc::c_int as ssize_t;
    while i < (*mat).nrows as i64 {
        if *part.offset(i as isize) == pid {
            gk_icopy(
                (*((*mat).rowptr).offset((i + 1 as i64) as isize)
                    - *((*mat).rowptr).offset(i as isize)) as size_t,
                ((*mat).rowind).offset(*((*mat).rowptr).offset(i as isize) as isize),
                ((*nmat).rowind).offset(nnz as isize),
            );
            gk_fcopy(
                (*((*mat).rowptr).offset((i + 1 as i64) as isize)
                    - *((*mat).rowptr).offset(i as isize)) as size_t,
                ((*mat).rowval).offset(*((*mat).rowptr).offset(i as isize) as isize),
                ((*nmat).rowval).offset(nnz as isize),
            );
            nnz
                += *((*mat).rowptr)
                    .offset((i + 1 as i64) as isize)
                    - *((*mat).rowptr).offset(i as isize);
            j += 1;
            *((*nmat).rowptr).offset(j as isize) = nnz;
        }
        i += 1;
        i;
    }
    return nmat;
}
#[no_mangle]
pub unsafe extern "C" fn gk_csr_Split(
    mut mat: *mut gk_csr_t,
    mut color: *mut libc::c_int,
) -> *mut *mut gk_csr_t {
    let mut i: ssize_t = 0;
    let mut j: ssize_t = 0;
    let mut nrows: libc::c_int = 0;
    let mut ncolors: libc::c_int = 0;
    let mut rowptr: *mut ssize_t = 0 as *mut ssize_t;
    let mut rowind: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut rowval: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut smats: *mut *mut gk_csr_t = 0 as *mut *mut gk_csr_t;
    nrows = (*mat).nrows;
    rowptr = (*mat).rowptr;
    rowind = (*mat).rowind;
    rowval = (*mat).rowval;
    ncolors = gk_imax(*rowptr.offset(nrows as isize) as size_t, color)
        + 1;
    smats = gk_malloc(
        (::core::mem::size_of::<*mut gk_csr_t>() as u64)
            .wrapping_mul(ncolors as u64),
        b"gk_csr_Split: smats\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut *mut gk_csr_t;
    i = 0 as libc::c_int as ssize_t;
    while i < ncolors as i64 {
        let ref mut fresh1 = *smats.offset(i as isize);
        *fresh1 = gk_csr_Create();
        (**smats.offset(i as isize)).nrows = (*mat).nrows;
        (**smats.offset(i as isize)).ncols = (*mat).ncols;
        let ref mut fresh2 = (**smats.offset(i as isize)).rowptr;
        *fresh2 = gk_zsmalloc(
            (nrows + 1) as size_t,
            0 as libc::c_int as ssize_t,
            b"gk_csr_Split: smats[i]->rowptr\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        i += 1;
        i;
    }
    i = 0 as libc::c_int as ssize_t;
    while i < nrows as i64 {
        j = *rowptr.offset(i as isize);
        while j < *rowptr.offset((i + 1 as i64) as isize) {
            let ref mut fresh3 = *((**smats.offset(*color.offset(j as isize) as isize))
                .rowptr)
                .offset(i as isize);
            *fresh3 += 1;
            *fresh3;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    i = 0 as libc::c_int as ssize_t;
    while i < ncolors as i64 {
        j = 1 as ssize_t;
        while j < nrows as i64 {
            let ref mut fresh4 = *((**smats.offset(i as isize)).rowptr)
                .offset(j as isize);
            *fresh4
                += *((**smats.offset(i as isize)).rowptr)
                    .offset((j - 1 as i64) as isize);
            j += 1;
            j;
        }
        j = nrows as ssize_t;
        while j > 0 as libc::c_int as i64 {
            *((**smats.offset(i as isize)).rowptr)
                .offset(
                    j as isize,
                ) = *((**smats.offset(i as isize)).rowptr)
                .offset((j - 1 as i64) as isize);
            j -= 1;
            j;
        }
        *((**smats.offset(i as isize)).rowptr)
            .offset(0 as libc::c_int as isize) = 0 as libc::c_int as ssize_t;
        i += 1;
        i;
    }
    i = 0 as libc::c_int as ssize_t;
    while i < ncolors as i64 {
        let ref mut fresh5 = (**smats.offset(i as isize)).rowind;
        *fresh5 = gk_imalloc(
            *((**smats.offset(i as isize)).rowptr).offset(nrows as isize) as size_t,
            b"gk_csr_Split: smats[i]->rowind\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        let ref mut fresh6 = (**smats.offset(i as isize)).rowval;
        *fresh6 = gk_fmalloc(
            *((**smats.offset(i as isize)).rowptr).offset(nrows as isize) as size_t,
            b"gk_csr_Split: smats[i]->rowval\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        i += 1;
        i;
    }
    i = 0 as libc::c_int as ssize_t;
    while i < nrows as i64 {
        j = *rowptr.offset(i as isize);
        while j < *rowptr.offset((i + 1 as i64) as isize) {
            *((**smats.offset(*color.offset(j as isize) as isize)).rowind)
                .offset(
                    *((**smats.offset(*color.offset(j as isize) as isize)).rowptr)
                        .offset(i as isize) as isize,
                ) = *rowind.offset(j as isize);
            *((**smats.offset(*color.offset(j as isize) as isize)).rowval)
                .offset(
                    *((**smats.offset(*color.offset(j as isize) as isize)).rowptr)
                        .offset(i as isize) as isize,
                ) = *rowval.offset(j as isize);
            let ref mut fresh7 = *((**smats.offset(*color.offset(j as isize) as isize))
                .rowptr)
                .offset(i as isize);
            *fresh7 += 1;
            *fresh7;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    i = 0 as libc::c_int as ssize_t;
    while i < ncolors as i64 {
        j = nrows as ssize_t;
        while j > 0 as libc::c_int as i64 {
            *((**smats.offset(i as isize)).rowptr)
                .offset(
                    j as isize,
                ) = *((**smats.offset(i as isize)).rowptr)
                .offset((j - 1 as i64) as isize);
            j -= 1;
            j;
        }
        *((**smats.offset(i as isize)).rowptr)
            .offset(0 as libc::c_int as isize) = 0 as libc::c_int as ssize_t;
        i += 1;
        i;
    }
    return smats;
}
#[no_mangle]
pub unsafe extern "C" fn gk_csr_Read(
    mut filename: *mut libc::c_char,
    mut format: libc::c_int,
    mut readvals: libc::c_int,
    mut numbering: libc::c_int,
) -> *mut gk_csr_t {
    let mut i: ssize_t = 0;
    let mut k: ssize_t = 0;
    let mut l: ssize_t = 0;
    let mut nfields: size_t = 0;
    let mut nrows: size_t = 0;
    let mut ncols: size_t = 0;
    let mut nnz: size_t = 0;
    let mut fmt: size_t = 0;
    let mut ncon: size_t = 0;
    let mut lnlen: size_t = 0;
    let mut rowptr: *mut ssize_t = 0 as *mut ssize_t;
    let mut rowind: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ival: libc::c_int = 0;
    let mut rowval: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut fval: libc::c_float = 0.;
    let mut readsizes: libc::c_int = 0;
    let mut readwgts: libc::c_int = 0;
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut head: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tail: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fmtstr: [libc::c_char; 256] = [0; 256];
    let mut fpin: *mut FILE = 0 as *mut FILE;
    let mut mat: *mut gk_csr_t = 0 as *mut gk_csr_t;
    if gk_fexists(filename) == 0 {
        gk_errexit(
            15 as libc::c_int,
            b"File %s does not exist!\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            filename,
        );
    }
    if format == 4 as libc::c_int {
        mat = gk_csr_Create();
        fpin = gk_fopen(
            filename,
            b"rb\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"gk_csr_Read: fpin\0" as *const u8 as *const libc::c_char,
        );
        if fread(
            &mut (*mat).nrows as *mut int32_t as *mut libc::c_void,
            ::core::mem::size_of::<int32_t>() as u64,
            1 as u64,
            fpin,
        ) != 1 as u64
        {
            gk_errexit(
                15 as libc::c_int,
                b"Failed to read the nrows from file %s!\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                filename,
            );
        }
        if fread(
            &mut (*mat).ncols as *mut int32_t as *mut libc::c_void,
            ::core::mem::size_of::<int32_t>() as u64,
            1 as u64,
            fpin,
        ) != 1 as u64
        {
            gk_errexit(
                15 as libc::c_int,
                b"Failed to read the ncols from file %s!\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                filename,
            );
        }
        (*mat)
            .rowptr = gk_zmalloc(
            ((*mat).nrows + 1) as size_t,
            b"gk_csr_Read: rowptr\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        if fread(
            (*mat).rowptr as *mut libc::c_void,
            ::core::mem::size_of::<ssize_t>() as u64,
            ((*mat).nrows + 1) as u64,
            fpin,
        ) != ((*mat).nrows + 1) as u64
        {
            gk_errexit(
                15 as libc::c_int,
                b"Failed to read the rowptr from file %s!\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                filename,
            );
        }
        (*mat)
            .rowind = gk_imalloc(
            *((*mat).rowptr).offset((*mat).nrows as isize) as size_t,
            b"gk_csr_Read: rowind\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        if fread(
            (*mat).rowind as *mut libc::c_void,
            ::core::mem::size_of::<int32_t>() as u64,
            *((*mat).rowptr).offset((*mat).nrows as isize) as u64,
            fpin,
        ) != *((*mat).rowptr).offset((*mat).nrows as isize) as u64
        {
            gk_errexit(
                15 as libc::c_int,
                b"Failed to read the rowind from file %s!\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                filename,
            );
        }
        if readvals == 1 {
            (*mat)
                .rowval = gk_fmalloc(
                *((*mat).rowptr).offset((*mat).nrows as isize) as size_t,
                b"gk_csr_Read: rowval\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            if fread(
                (*mat).rowval as *mut libc::c_void,
                ::core::mem::size_of::<libc::c_float>() as u64,
                *((*mat).rowptr).offset((*mat).nrows as isize) as u64,
                fpin,
            ) != *((*mat).rowptr).offset((*mat).nrows as isize) as u64
            {
                gk_errexit(
                    15 as libc::c_int,
                    b"Failed to read the rowval from file %s!\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    filename,
                );
            }
        }
        gk_fclose(fpin);
        return mat;
    }
    if format == 5 as libc::c_int {
        mat = gk_csr_Create();
        fpin = gk_fopen(
            filename,
            b"rb\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"gk_csr_Read: fpin\0" as *const u8 as *const libc::c_char,
        );
        if fread(
            &mut (*mat).nrows as *mut int32_t as *mut libc::c_void,
            ::core::mem::size_of::<int32_t>() as u64,
            1 as u64,
            fpin,
        ) != 1 as u64
        {
            gk_errexit(
                15 as libc::c_int,
                b"Failed to read the nrows from file %s!\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                filename,
            );
        }
        if fread(
            &mut (*mat).ncols as *mut int32_t as *mut libc::c_void,
            ::core::mem::size_of::<int32_t>() as u64,
            1 as u64,
            fpin,
        ) != 1 as u64
        {
            gk_errexit(
                15 as libc::c_int,
                b"Failed to read the ncols from file %s!\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                filename,
            );
        }
        (*mat)
            .colptr = gk_zmalloc(
            ((*mat).ncols + 1) as size_t,
            b"gk_csr_Read: colptr\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        if fread(
            (*mat).colptr as *mut libc::c_void,
            ::core::mem::size_of::<ssize_t>() as u64,
            ((*mat).ncols + 1) as u64,
            fpin,
        ) != ((*mat).ncols + 1) as u64
        {
            gk_errexit(
                15 as libc::c_int,
                b"Failed to read the colptr from file %s!\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                filename,
            );
        }
        (*mat)
            .colind = gk_imalloc(
            *((*mat).colptr).offset((*mat).ncols as isize) as size_t,
            b"gk_csr_Read: colind\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        if fread(
            (*mat).colind as *mut libc::c_void,
            ::core::mem::size_of::<int32_t>() as u64,
            *((*mat).colptr).offset((*mat).ncols as isize) as u64,
            fpin,
        ) != *((*mat).colptr).offset((*mat).ncols as isize) as u64
        {
            gk_errexit(
                15 as libc::c_int,
                b"Failed to read the colind from file %s!\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                filename,
            );
        }
        if readvals != 0 {
            (*mat)
                .colval = gk_fmalloc(
                *((*mat).colptr).offset((*mat).ncols as isize) as size_t,
                b"gk_csr_Read: colval\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            if fread(
                (*mat).colval as *mut libc::c_void,
                ::core::mem::size_of::<libc::c_float>() as u64,
                *((*mat).colptr).offset((*mat).ncols as isize) as u64,
                fpin,
            ) != *((*mat).colptr).offset((*mat).ncols as isize) as u64
            {
                gk_errexit(
                    15 as libc::c_int,
                    b"Failed to read the colval from file %s!\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    filename,
                );
            }
        }
        gk_fclose(fpin);
        return mat;
    }
    if format == 1 {
        fpin = gk_fopen(
            filename,
            b"r\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"gk_csr_Read: fpin\0" as *const u8 as *const libc::c_char,
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
        if sscanf(
            line,
            b"%zu %zu %zu\0" as *const u8 as *const libc::c_char,
            &mut nrows as *mut size_t,
            &mut ncols as *mut size_t,
            &mut nnz as *mut size_t,
        ) != 3 as libc::c_int
        {
            gk_errexit(
                15 as libc::c_int,
                b"Header line must contain 3 integers.\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
        readsizes = 0 as libc::c_int;
        readwgts = 0 as libc::c_int;
        readvals = 1;
        numbering = 1;
    } else if format == 3 as libc::c_int {
        fpin = gk_fopen(
            filename,
            b"r\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"gk_csr_Read: fpin\0" as *const u8 as *const libc::c_char,
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
            &mut nrows as *mut size_t,
            &mut nnz as *mut size_t,
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
        ncols = nrows;
        nnz = (nnz as u64).wrapping_mul(2 as libc::c_int as u64)
            as size_t as size_t;
        if fmt > 111 as u64 {
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
        readsizes = (fmtstr[0] as libc::c_int == '1' as i32)
            as libc::c_int;
        readwgts = (fmtstr[1] as libc::c_int == '1' as i32)
            as libc::c_int;
        readvals = (fmtstr[2 as libc::c_int as usize] as libc::c_int == '1' as i32)
            as libc::c_int;
        numbering = 1;
        ncon = if ncon == 0 as libc::c_int as u64 {
            1 as u64
        } else {
            ncon
        };
    } else {
        readsizes = 0 as libc::c_int;
        readwgts = 0 as libc::c_int;
        gk_getfilestats(
            filename,
            &mut nrows,
            &mut nnz,
            0 as *mut size_t,
            0 as *mut size_t,
        );
        if readvals == 1
            && nnz.wrapping_rem(2 as libc::c_int as u64)
                == 1 as u64
        {
            gk_errexit(
                15 as libc::c_int,
                b"Error: The number of numbers (%zd %d) in the input file is not even.\n\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
                nnz,
                readvals,
            );
        }
        if readvals == 1 {
            nnz = nnz.wrapping_div(2 as libc::c_int as u64);
        }
        fpin = gk_fopen(
            filename,
            b"r\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"gk_csr_Read: fpin\0" as *const u8 as *const libc::c_char,
        );
    }
    mat = gk_csr_Create();
    (*mat).nrows = nrows as int32_t;
    (*mat)
        .rowptr = gk_zmalloc(
        nrows.wrapping_add(1 as u64),
        b"gk_csr_Read: rowptr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    rowptr = (*mat).rowptr;
    (*mat)
        .rowind = gk_imalloc(
        nnz,
        b"gk_csr_Read: rowind\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    rowind = (*mat).rowind;
    if readvals != 2 as libc::c_int {
        (*mat)
            .rowval = gk_fsmalloc(
            nnz,
            1.0f64 as libc::c_float,
            b"gk_csr_Read: rowval\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        rowval = (*mat).rowval;
    }
    if readsizes != 0 {
        (*mat)
            .rsizes = gk_fsmalloc(
            nrows,
            0.0f64 as libc::c_float,
            b"gk_csr_Read: rsizes\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if readwgts != 0 {
        (*mat)
            .rwgts = gk_fsmalloc(
            nrows.wrapping_mul(ncon),
            0.0f64 as libc::c_float,
            b"gk_csr_Read: rwgts\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    numbering = if numbering != 0 { -(1) } else { 0 as libc::c_int };
    ncols = 0 as libc::c_int as size_t;
    *rowptr.offset(0 as libc::c_int as isize) = 0 as libc::c_int as ssize_t;
    k = 0 as libc::c_int as ssize_t;
    i = 0 as libc::c_int as ssize_t;
    while (i as u64) < nrows {
        loop {
            if gk_getline(&mut line, &mut lnlen, fpin)
                == -(1) as i64
            {
                gk_errexit(
                    15 as libc::c_int,
                    b"Premature end of input file: file while reading row %d\n\0"
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
            *((*mat).rsizes).offset(i as isize) = strtof(head, &mut tail);
            if tail == head {
                gk_errexit(
                    15 as libc::c_int,
                    b"The line for vertex %zd does not have size information\n\0"
                        as *const u8 as *const libc::c_char as *mut libc::c_char,
                    i + 1 as i64,
                );
            }
            if *((*mat).rsizes).offset(i as isize) < 0 as libc::c_int as libc::c_float {
                errexit(
                    b"The size for vertex %zd must be >= 0\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    i + 1 as i64,
                );
            }
            head = tail;
        }
        if readwgts != 0 {
            l = 0 as libc::c_int as ssize_t;
            while (l as u64) < ncon {
                *((*mat).rwgts)
                    .offset(
                        (i as u64)
                            .wrapping_mul(ncon)
                            .wrapping_add(l as u64) as isize,
                    ) = strtof(head, &mut tail);
                if tail == head {
                    errexit(
                        b"The line for vertex %zd does not have enough weights for the %d constraints.\n\0"
                            as *const u8 as *const libc::c_char as *mut libc::c_char,
                        i + 1 as i64,
                        ncon,
                    );
                }
                if *((*mat).rwgts)
                    .offset(
                        (i as u64)
                            .wrapping_mul(ncon)
                            .wrapping_add(l as u64) as isize,
                    ) < 0 as libc::c_int as libc::c_float
                {
                    errexit(
                        b"The weight vertex %zd and constraint %zd must be >= 0\n\0"
                            as *const u8 as *const libc::c_char as *mut libc::c_char,
                        i + 1 as i64,
                        l,
                    );
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
            let ref mut fresh8 = *rowind.offset(k as isize);
            *fresh8 = ival + numbering;
            if *fresh8 < 0 as libc::c_int {
                gk_errexit(
                    15 as libc::c_int,
                    b"Error: Invalid column number %d at row %zd.\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    ival,
                    i,
                );
            }
            ncols = if *rowind.offset(k as isize) as u64 >= ncols {
                *rowind.offset(k as isize) as u64
            } else {
                ncols
            };
            if readvals == 1 {
                fval = strtof(head, &mut tail);
                if tail == head {
                    gk_errexit(
                        15 as libc::c_int,
                        b"Value could not be found for column! Row:%zd, NNZ:%zd\n\0"
                            as *const u8 as *const libc::c_char as *mut libc::c_char,
                        i,
                        k,
                    );
                }
                head = tail;
                *rowval.offset(k as isize) = fval;
            }
            k += 1;
            k;
        }
        *rowptr.offset((i + 1 as i64) as isize) = k;
        i += 1;
        i;
    }
    if format == 3 as libc::c_int {
        (*mat).ncols = (*mat).nrows;
    } else {
        (*mat).ncols = ncols.wrapping_add(1 as u64) as int32_t;
    }
    if k as u64 != nnz {
        gk_errexit(
            15 as libc::c_int,
            b"gk_csr_Read: Something wrong with the number of nonzeros in the input file. NNZ=%zd, ActualNNZ=%zd.\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            nnz,
            k,
        );
    }
    gk_fclose(fpin);
    gk_free(
        &mut line as *mut *mut libc::c_char as *mut *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
    return mat;
}
#[no_mangle]
pub unsafe extern "C" fn gk_csr_Write(
    mut mat: *mut gk_csr_t,
    mut filename: *mut libc::c_char,
    mut format: libc::c_int,
    mut writevals: libc::c_int,
    mut numbering: libc::c_int,
) {
    let mut i: ssize_t = 0;
    let mut j: ssize_t = 0;
    let mut fpout: *mut FILE = 0 as *mut FILE;
    if format == 4 as libc::c_int {
        if filename.is_null() {
            gk_errexit(
                15 as libc::c_int,
                b"The filename parameter cannot be NULL.\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
        fpout = gk_fopen(
            filename,
            b"wb\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"gk_csr_Write: fpout\0" as *const u8 as *const libc::c_char,
        );
        fwrite(
            &mut (*mat).nrows as *mut int32_t as *const libc::c_void,
            ::core::mem::size_of::<int32_t>() as u64,
            1 as u64,
            fpout,
        );
        fwrite(
            &mut (*mat).ncols as *mut int32_t as *const libc::c_void,
            ::core::mem::size_of::<int32_t>() as u64,
            1 as u64,
            fpout,
        );
        fwrite(
            (*mat).rowptr as *const libc::c_void,
            ::core::mem::size_of::<ssize_t>() as u64,
            ((*mat).nrows + 1) as u64,
            fpout,
        );
        fwrite(
            (*mat).rowind as *const libc::c_void,
            ::core::mem::size_of::<int32_t>() as u64,
            *((*mat).rowptr).offset((*mat).nrows as isize) as u64,
            fpout,
        );
        if writevals != 0 {
            fwrite(
                (*mat).rowval as *const libc::c_void,
                ::core::mem::size_of::<libc::c_float>() as u64,
                *((*mat).rowptr).offset((*mat).nrows as isize) as u64,
                fpout,
            );
        }
        gk_fclose(fpout);
        return;
    }
    if format == 5 as libc::c_int {
        if filename.is_null() {
            gk_errexit(
                15 as libc::c_int,
                b"The filename parameter cannot be NULL.\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
        fpout = gk_fopen(
            filename,
            b"wb\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"gk_csr_Write: fpout\0" as *const u8 as *const libc::c_char,
        );
        fwrite(
            &mut (*mat).nrows as *mut int32_t as *const libc::c_void,
            ::core::mem::size_of::<int32_t>() as u64,
            1 as u64,
            fpout,
        );
        fwrite(
            &mut (*mat).ncols as *mut int32_t as *const libc::c_void,
            ::core::mem::size_of::<int32_t>() as u64,
            1 as u64,
            fpout,
        );
        fwrite(
            (*mat).colptr as *const libc::c_void,
            ::core::mem::size_of::<ssize_t>() as u64,
            ((*mat).ncols + 1) as u64,
            fpout,
        );
        fwrite(
            (*mat).colind as *const libc::c_void,
            ::core::mem::size_of::<int32_t>() as u64,
            *((*mat).colptr).offset((*mat).ncols as isize) as u64,
            fpout,
        );
        if writevals != 0 {
            fwrite(
                (*mat).colval as *const libc::c_void,
                ::core::mem::size_of::<libc::c_float>() as u64,
                *((*mat).colptr).offset((*mat).ncols as isize) as u64,
                fpout,
            );
        }
        gk_fclose(fpout);
        return;
    }
    if !filename.is_null() {
        fpout = gk_fopen(
            filename,
            b"w\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"gk_csr_Write: fpout\0" as *const u8 as *const libc::c_char,
        );
    } else {
        fpout = stdout;
    }
    if format == 1 {
        fprintf(
            fpout,
            b"%d %d %zd\n\0" as *const u8 as *const libc::c_char,
            (*mat).nrows,
            (*mat).ncols,
            *((*mat).rowptr).offset((*mat).nrows as isize),
        );
        writevals = 1;
        numbering = 1;
    }
    i = 0 as libc::c_int as ssize_t;
    while i < (*mat).nrows as i64 {
        j = *((*mat).rowptr).offset(i as isize);
        while j
            < *((*mat).rowptr).offset((i + 1 as i64) as isize)
        {
            fprintf(
                fpout,
                b" %d\0" as *const u8 as *const libc::c_char,
                *((*mat).rowind).offset(j as isize)
                    + (if numbering != 0 { 1 } else { 0 as libc::c_int }),
            );
            if writevals != 0 {
                fprintf(
                    fpout,
                    b" %f\0" as *const u8 as *const libc::c_char,
                    *((*mat).rowval).offset(j as isize) as libc::c_double,
                );
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
pub unsafe extern "C" fn gk_csr_Prune(
    mut mat: *mut gk_csr_t,
    mut what: libc::c_int,
    mut minf: libc::c_int,
    mut maxf: libc::c_int,
) -> *mut gk_csr_t {
    let mut i: ssize_t = 0;
    let mut j: ssize_t = 0;
    let mut nnz: ssize_t = 0;
    let mut nrows: libc::c_int = 0;
    let mut ncols: libc::c_int = 0;
    let mut rowptr: *mut ssize_t = 0 as *mut ssize_t;
    let mut nrowptr: *mut ssize_t = 0 as *mut ssize_t;
    let mut rowind: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nrowind: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut collen: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut rowval: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut nrowval: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut nmat: *mut gk_csr_t = 0 as *mut gk_csr_t;
    nmat = gk_csr_Create();
    (*nmat).nrows = (*mat).nrows;
    nrows = (*nmat).nrows;
    (*nmat).ncols = (*mat).ncols;
    ncols = (*nmat).ncols;
    rowptr = (*mat).rowptr;
    rowind = (*mat).rowind;
    rowval = (*mat).rowval;
    (*nmat)
        .rowptr = gk_zmalloc(
        (nrows + 1) as size_t,
        b"gk_csr_Prune: nrowptr\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    nrowptr = (*nmat).rowptr;
    (*nmat)
        .rowind = gk_imalloc(
        *rowptr.offset(nrows as isize) as size_t,
        b"gk_csr_Prune: nrowind\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    nrowind = (*nmat).rowind;
    (*nmat)
        .rowval = gk_fmalloc(
        *rowptr.offset(nrows as isize) as size_t,
        b"gk_csr_Prune: nrowval\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    nrowval = (*nmat).rowval;
    match what {
        2 => {
            collen = gk_ismalloc(
                ncols as size_t,
                0 as libc::c_int,
                b"gk_csr_Prune: collen\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            i = 0 as libc::c_int as ssize_t;
            while i < nrows as i64 {
                j = *rowptr.offset(i as isize);
                while j < *rowptr.offset((i + 1 as i64) as isize)
                {
                    let ref mut fresh9 = *collen
                        .offset(*rowind.offset(j as isize) as isize);
                    *fresh9 += 1;
                    *fresh9;
                    j += 1;
                    j;
                }
                i += 1;
                i;
            }
            i = 0 as libc::c_int as ssize_t;
            while i < ncols as i64 {
                *collen
                    .offset(
                        i as isize,
                    ) = if *collen.offset(i as isize) >= minf
                    && *collen.offset(i as isize) <= maxf
                {
                    1
                } else {
                    0 as libc::c_int
                };
                i += 1;
                i;
            }
            *nrowptr.offset(0 as libc::c_int as isize) = 0 as libc::c_int as ssize_t;
            nnz = 0 as libc::c_int as ssize_t;
            i = 0 as libc::c_int as ssize_t;
            while i < nrows as i64 {
                j = *rowptr.offset(i as isize);
                while j < *rowptr.offset((i + 1 as i64) as isize)
                {
                    if *collen.offset(*rowind.offset(j as isize) as isize) != 0 {
                        *nrowind.offset(nnz as isize) = *rowind.offset(j as isize);
                        *nrowval.offset(nnz as isize) = *rowval.offset(j as isize);
                        nnz += 1;
                        nnz;
                    }
                    j += 1;
                    j;
                }
                *nrowptr.offset((i + 1 as i64) as isize) = nnz;
                i += 1;
                i;
            }
            gk_free(
                &mut collen as *mut *mut libc::c_int as *mut *mut libc::c_void,
                0 as *mut *mut libc::c_void,
            );
        }
        1 => {
            *nrowptr.offset(0 as libc::c_int as isize) = 0 as libc::c_int as ssize_t;
            nnz = 0 as libc::c_int as ssize_t;
            i = 0 as libc::c_int as ssize_t;
            while i < nrows as i64 {
                if *rowptr.offset((i + 1 as i64) as isize)
                    - *rowptr.offset(i as isize) >= minf as i64
                    && *rowptr.offset((i + 1 as i64) as isize)
                        - *rowptr.offset(i as isize) <= maxf as i64
                {
                    j = *rowptr.offset(i as isize);
                    while j
                        < *rowptr.offset((i + 1 as i64) as isize)
                    {
                        *nrowind.offset(nnz as isize) = *rowind.offset(j as isize);
                        *nrowval.offset(nnz as isize) = *rowval.offset(j as isize);
                        j += 1;
                        j;
                        nnz += 1;
                        nnz;
                    }
                }
                *nrowptr.offset((i + 1 as i64) as isize) = nnz;
                i += 1;
                i;
            }
        }
        _ => {
            gk_csr_Free(&mut nmat);
            gk_errexit(
                15 as libc::c_int,
                b"Unknown prunning type of %d\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                what,
            );
            return 0 as *mut gk_csr_t;
        }
    }
    return nmat;
}
#[no_mangle]
pub unsafe extern "C" fn gk_csr_LowFilter(
    mut mat: *mut gk_csr_t,
    mut what: libc::c_int,
    mut norm: libc::c_int,
    mut fraction: libc::c_float,
) -> *mut gk_csr_t {
    let mut i: ssize_t = 0;
    let mut j: ssize_t = 0;
    let mut nnz: ssize_t = 0;
    let mut nrows: libc::c_int = 0;
    let mut ncols: libc::c_int = 0;
    let mut ncand: libc::c_int = 0;
    let mut maxlen: libc::c_int = 0 as libc::c_int;
    let mut rowptr: *mut ssize_t = 0 as *mut ssize_t;
    let mut colptr: *mut ssize_t = 0 as *mut ssize_t;
    let mut nrowptr: *mut ssize_t = 0 as *mut ssize_t;
    let mut rowind: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut colind: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nrowind: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut rowval: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut colval: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut nrowval: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut rsum: libc::c_float = 0.;
    let mut tsum: libc::c_float = 0.;
    let mut nmat: *mut gk_csr_t = 0 as *mut gk_csr_t;
    let mut cand: *mut gk_fkv_t = 0 as *mut gk_fkv_t;
    nmat = gk_csr_Create();
    (*nmat).nrows = (*mat).nrows;
    nrows = (*nmat).nrows;
    (*nmat).ncols = (*mat).ncols;
    ncols = (*nmat).ncols;
    rowptr = (*mat).rowptr;
    rowind = (*mat).rowind;
    rowval = (*mat).rowval;
    colptr = (*mat).colptr;
    colind = (*mat).colind;
    colval = (*mat).colval;
    (*nmat)
        .rowptr = gk_zmalloc(
        (nrows + 1) as size_t,
        b"gk_csr_LowFilter: nrowptr\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    nrowptr = (*nmat).rowptr;
    (*nmat)
        .rowind = gk_imalloc(
        *rowptr.offset(nrows as isize) as size_t,
        b"gk_csr_LowFilter: nrowind\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    nrowind = (*nmat).rowind;
    (*nmat)
        .rowval = gk_fmalloc(
        *rowptr.offset(nrows as isize) as size_t,
        b"gk_csr_LowFilter: nrowval\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    nrowval = (*nmat).rowval;
    match what {
        2 => {
            if ((*mat).colptr).is_null() {
                gk_errexit(
                    15 as libc::c_int,
                    b"Cannot filter columns when column-based structure has not been created.\n\0"
                        as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            }
            gk_zcopy((nrows + 1) as size_t, rowptr, nrowptr);
            i = 0 as libc::c_int as ssize_t;
            while i < ncols as i64 {
                maxlen = (if maxlen as i64
                    >= *colptr.offset((i + 1 as i64) as isize)
                        - *colptr.offset(i as isize)
                {
                    maxlen as i64
                } else {
                    *colptr.offset((i + 1 as i64) as isize)
                        - *colptr.offset(i as isize)
                }) as libc::c_int;
                i += 1;
                i;
            }
            cand = gk_fkvmalloc(
                maxlen as size_t,
                b"gk_csr_LowFilter: cand\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            i = 0 as libc::c_int as ssize_t;
            while i < ncols as i64 {
                tsum = 0.0f64 as libc::c_float;
                ncand = 0 as libc::c_int;
                j = *colptr.offset(i as isize);
                while j < *colptr.offset((i + 1 as i64) as isize)
                {
                    (*cand.offset(ncand as isize))
                        .val = *colind.offset(j as isize) as ssize_t;
                    (*cand.offset(ncand as isize)).key = *colval.offset(j as isize);
                    tsum
                        += if norm == 1 {
                            *colval.offset(j as isize)
                        } else {
                            *colval.offset(j as isize) * *colval.offset(j as isize)
                        };
                    j += 1;
                    j;
                    ncand += 1;
                    ncand;
                }
                gk_fkvsortd(ncand as size_t, cand);
                rsum = 0.0f64 as libc::c_float;
                j = 0 as libc::c_int as ssize_t;
                while j < ncand as i64 && rsum <= fraction * tsum {
                    rsum
                        += if norm == 1 {
                            (*cand.offset(j as isize)).key
                        } else {
                            (*cand.offset(j as isize)).key
                                * (*cand.offset(j as isize)).key
                        };
                    *nrowind
                        .offset(
                            *nrowptr.offset((*cand.offset(j as isize)).val as isize)
                                as isize,
                        ) = i as libc::c_int;
                    *nrowval
                        .offset(
                            *nrowptr.offset((*cand.offset(j as isize)).val as isize)
                                as isize,
                        ) = (*cand.offset(j as isize)).key;
                    let ref mut fresh10 = *nrowptr
                        .offset((*cand.offset(j as isize)).val as isize);
                    *fresh10 += 1;
                    *fresh10;
                    j += 1;
                    j;
                }
                i += 1;
                i;
            }
            gk_free(
                &mut cand as *mut *mut gk_fkv_t as *mut *mut libc::c_void,
                0 as *mut *mut libc::c_void,
            );
            nnz = 0 as libc::c_int as ssize_t;
            i = 0 as libc::c_int as ssize_t;
            while i < nrows as i64 {
                j = *rowptr.offset(i as isize);
                while j < *nrowptr.offset(i as isize) {
                    *nrowind.offset(nnz as isize) = *nrowind.offset(j as isize);
                    *nrowval.offset(nnz as isize) = *nrowval.offset(j as isize);
                    j += 1;
                    j;
                    nnz += 1;
                    nnz;
                }
                *nrowptr.offset(i as isize) = nnz;
                i += 1;
                i;
            }
            i = nrows as ssize_t;
            while i > 0 as libc::c_int as i64 {
                *nrowptr
                    .offset(
                        i as isize,
                    ) = *nrowptr.offset((i - 1 as i64) as isize);
                i -= 1;
                i;
            }
            *nrowptr.offset(0 as libc::c_int as isize) = 0 as libc::c_int as ssize_t;
        }
        1 => {
            if ((*mat).rowptr).is_null() {
                gk_errexit(
                    15 as libc::c_int,
                    b"Cannot filter rows when row-based structure has not been created.\n\0"
                        as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            }
            i = 0 as libc::c_int as ssize_t;
            while i < nrows as i64 {
                maxlen = (if maxlen as i64
                    >= *rowptr.offset((i + 1 as i64) as isize)
                        - *rowptr.offset(i as isize)
                {
                    maxlen as i64
                } else {
                    *rowptr.offset((i + 1 as i64) as isize)
                        - *rowptr.offset(i as isize)
                }) as libc::c_int;
                i += 1;
                i;
            }
            cand = gk_fkvmalloc(
                maxlen as size_t,
                b"gk_csr_LowFilter: cand\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            i = 0 as libc::c_int as ssize_t;
            while i < nrows as i64 {
                tsum = 0.0f64 as libc::c_float;
                ncand = 0 as libc::c_int;
                j = *rowptr.offset(i as isize);
                while j < *rowptr.offset((i + 1 as i64) as isize)
                {
                    (*cand.offset(ncand as isize))
                        .val = *rowind.offset(j as isize) as ssize_t;
                    (*cand.offset(ncand as isize)).key = *rowval.offset(j as isize);
                    tsum
                        += if norm == 1 {
                            *rowval.offset(j as isize)
                        } else {
                            *rowval.offset(j as isize) * *rowval.offset(j as isize)
                        };
                    j += 1;
                    j;
                    ncand += 1;
                    ncand;
                }
                gk_fkvsortd(ncand as size_t, cand);
                rsum = 0.0f64 as libc::c_float;
                j = 0 as libc::c_int as ssize_t;
                while j < ncand as i64 && rsum <= fraction * tsum {
                    rsum
                        += if norm == 1 {
                            (*cand.offset(j as isize)).key
                        } else {
                            (*cand.offset(j as isize)).key
                                * (*cand.offset(j as isize)).key
                        };
                    *nrowind
                        .offset(
                            (*rowptr.offset(i as isize) + j) as isize,
                        ) = (*cand.offset(j as isize)).val as libc::c_int;
                    *nrowval
                        .offset(
                            (*rowptr.offset(i as isize) + j) as isize,
                        ) = (*cand.offset(j as isize)).key;
                    j += 1;
                    j;
                }
                *nrowptr
                    .offset(
                        (i + 1 as i64) as isize,
                    ) = *rowptr.offset(i as isize) + j;
                i += 1;
                i;
            }
            gk_free(
                &mut cand as *mut *mut gk_fkv_t as *mut *mut libc::c_void,
                0 as *mut *mut libc::c_void,
            );
            nnz = 0 as libc::c_int as ssize_t;
            *nrowptr.offset(0 as libc::c_int as isize) = nnz;
            i = 0 as libc::c_int as ssize_t;
            while i < nrows as i64 {
                j = *rowptr.offset(i as isize);
                while j
                    < *nrowptr.offset((i + 1 as i64) as isize)
                {
                    *nrowind.offset(nnz as isize) = *nrowind.offset(j as isize);
                    *nrowval.offset(nnz as isize) = *nrowval.offset(j as isize);
                    j += 1;
                    j;
                    nnz += 1;
                    nnz;
                }
                *nrowptr.offset((i + 1 as i64) as isize) = nnz;
                i += 1;
                i;
            }
        }
        _ => {
            gk_csr_Free(&mut nmat);
            gk_errexit(
                15 as libc::c_int,
                b"Unknown prunning type of %d\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                what,
            );
            return 0 as *mut gk_csr_t;
        }
    }
    return nmat;
}
#[no_mangle]
pub unsafe extern "C" fn gk_csr_TopKPlusFilter(
    mut mat: *mut gk_csr_t,
    mut what: libc::c_int,
    mut topk: libc::c_int,
    mut keepval: libc::c_float,
) -> *mut gk_csr_t {
    let mut i: ssize_t = 0;
    let mut j: ssize_t = 0;
    let mut k: ssize_t = 0;
    let mut nnz: ssize_t = 0;
    let mut nrows: libc::c_int = 0;
    let mut ncols: libc::c_int = 0;
    let mut ncand: libc::c_int = 0;
    let mut rowptr: *mut ssize_t = 0 as *mut ssize_t;
    let mut colptr: *mut ssize_t = 0 as *mut ssize_t;
    let mut nrowptr: *mut ssize_t = 0 as *mut ssize_t;
    let mut rowind: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut colind: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nrowind: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut rowval: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut colval: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut nrowval: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut nmat: *mut gk_csr_t = 0 as *mut gk_csr_t;
    let mut cand: *mut gk_fkv_t = 0 as *mut gk_fkv_t;
    nmat = gk_csr_Create();
    (*nmat).nrows = (*mat).nrows;
    nrows = (*nmat).nrows;
    (*nmat).ncols = (*mat).ncols;
    ncols = (*nmat).ncols;
    rowptr = (*mat).rowptr;
    rowind = (*mat).rowind;
    rowval = (*mat).rowval;
    colptr = (*mat).colptr;
    colind = (*mat).colind;
    colval = (*mat).colval;
    (*nmat)
        .rowptr = gk_zmalloc(
        (nrows + 1) as size_t,
        b"gk_csr_LowFilter: nrowptr\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    nrowptr = (*nmat).rowptr;
    (*nmat)
        .rowind = gk_imalloc(
        *rowptr.offset(nrows as isize) as size_t,
        b"gk_csr_LowFilter: nrowind\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    nrowind = (*nmat).rowind;
    (*nmat)
        .rowval = gk_fmalloc(
        *rowptr.offset(nrows as isize) as size_t,
        b"gk_csr_LowFilter: nrowval\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    nrowval = (*nmat).rowval;
    match what {
        2 => {
            if ((*mat).colptr).is_null() {
                gk_errexit(
                    15 as libc::c_int,
                    b"Cannot filter columns when column-based structure has not been created.\n\0"
                        as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            }
            cand = gk_fkvmalloc(
                nrows as size_t,
                b"gk_csr_LowFilter: cand\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            gk_zcopy((nrows + 1) as size_t, rowptr, nrowptr);
            i = 0 as libc::c_int as ssize_t;
            while i < ncols as i64 {
                ncand = 0 as libc::c_int;
                j = *colptr.offset(i as isize);
                while j < *colptr.offset((i + 1 as i64) as isize)
                {
                    (*cand.offset(ncand as isize))
                        .val = *colind.offset(j as isize) as ssize_t;
                    (*cand.offset(ncand as isize)).key = *colval.offset(j as isize);
                    j += 1;
                    j;
                    ncand += 1;
                    ncand;
                }
                gk_fkvsortd(ncand as size_t, cand);
                k = (if topk >= ncand { ncand } else { topk }) as ssize_t;
                j = 0 as libc::c_int as ssize_t;
                while j < k {
                    *nrowind
                        .offset(
                            *nrowptr.offset((*cand.offset(j as isize)).val as isize)
                                as isize,
                        ) = i as libc::c_int;
                    *nrowval
                        .offset(
                            *nrowptr.offset((*cand.offset(j as isize)).val as isize)
                                as isize,
                        ) = (*cand.offset(j as isize)).key;
                    let ref mut fresh11 = *nrowptr
                        .offset((*cand.offset(j as isize)).val as isize);
                    *fresh11 += 1;
                    *fresh11;
                    j += 1;
                    j;
                }
                while j < ncand as i64 {
                    if (*cand.offset(j as isize)).key < keepval {
                        break;
                    }
                    *nrowind
                        .offset(
                            *nrowptr.offset((*cand.offset(j as isize)).val as isize)
                                as isize,
                        ) = i as libc::c_int;
                    *nrowval
                        .offset(
                            *nrowptr.offset((*cand.offset(j as isize)).val as isize)
                                as isize,
                        ) = (*cand.offset(j as isize)).key;
                    let ref mut fresh12 = *nrowptr
                        .offset((*cand.offset(j as isize)).val as isize);
                    *fresh12 += 1;
                    *fresh12;
                    j += 1;
                    j;
                }
                i += 1;
                i;
            }
            nnz = 0 as libc::c_int as ssize_t;
            i = 0 as libc::c_int as ssize_t;
            while i < nrows as i64 {
                j = *rowptr.offset(i as isize);
                while j < *nrowptr.offset(i as isize) {
                    *nrowind.offset(nnz as isize) = *nrowind.offset(j as isize);
                    *nrowval.offset(nnz as isize) = *nrowval.offset(j as isize);
                    j += 1;
                    j;
                    nnz += 1;
                    nnz;
                }
                *nrowptr.offset(i as isize) = nnz;
                i += 1;
                i;
            }
            i = nrows as ssize_t;
            while i > 0 as libc::c_int as i64 {
                *nrowptr
                    .offset(
                        i as isize,
                    ) = *nrowptr.offset((i - 1 as i64) as isize);
                i -= 1;
                i;
            }
            *nrowptr.offset(0 as libc::c_int as isize) = 0 as libc::c_int as ssize_t;
            gk_free(
                &mut cand as *mut *mut gk_fkv_t as *mut *mut libc::c_void,
                0 as *mut *mut libc::c_void,
            );
        }
        1 => {
            if ((*mat).rowptr).is_null() {
                gk_errexit(
                    15 as libc::c_int,
                    b"Cannot filter rows when row-based structure has not been created.\n\0"
                        as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            }
            cand = gk_fkvmalloc(
                ncols as size_t,
                b"gk_csr_LowFilter: cand\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            *nrowptr.offset(0 as libc::c_int as isize) = 0 as libc::c_int as ssize_t;
            nnz = 0 as libc::c_int as ssize_t;
            i = 0 as libc::c_int as ssize_t;
            while i < nrows as i64 {
                ncand = 0 as libc::c_int;
                j = *rowptr.offset(i as isize);
                while j < *rowptr.offset((i + 1 as i64) as isize)
                {
                    (*cand.offset(ncand as isize))
                        .val = *rowind.offset(j as isize) as ssize_t;
                    (*cand.offset(ncand as isize)).key = *rowval.offset(j as isize);
                    j += 1;
                    j;
                    ncand += 1;
                    ncand;
                }
                gk_fkvsortd(ncand as size_t, cand);
                k = (if topk >= ncand { ncand } else { topk }) as ssize_t;
                j = 0 as libc::c_int as ssize_t;
                while j < k {
                    *nrowind
                        .offset(
                            nnz as isize,
                        ) = (*cand.offset(j as isize)).val as libc::c_int;
                    *nrowval.offset(nnz as isize) = (*cand.offset(j as isize)).key;
                    j += 1;
                    j;
                    nnz += 1;
                    nnz;
                }
                while j < ncand as i64 {
                    if (*cand.offset(j as isize)).key < keepval {
                        break;
                    }
                    *nrowind
                        .offset(
                            nnz as isize,
                        ) = (*cand.offset(j as isize)).val as libc::c_int;
                    *nrowval.offset(nnz as isize) = (*cand.offset(j as isize)).key;
                    j += 1;
                    j;
                    nnz += 1;
                    nnz;
                }
                *nrowptr.offset((i + 1 as i64) as isize) = nnz;
                i += 1;
                i;
            }
            gk_free(
                &mut cand as *mut *mut gk_fkv_t as *mut *mut libc::c_void,
                0 as *mut *mut libc::c_void,
            );
        }
        _ => {
            gk_csr_Free(&mut nmat);
            gk_errexit(
                15 as libc::c_int,
                b"Unknown prunning type of %d\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                what,
            );
            return 0 as *mut gk_csr_t;
        }
    }
    return nmat;
}
#[no_mangle]
pub unsafe extern "C" fn gk_csr_ZScoreFilter(
    mut mat: *mut gk_csr_t,
    mut what: libc::c_int,
    mut zscore: libc::c_float,
) -> *mut gk_csr_t {
    let mut i: ssize_t = 0;
    let mut j: ssize_t = 0;
    let mut nnz: ssize_t = 0;
    let mut nrows: libc::c_int = 0;
    let mut rowptr: *mut ssize_t = 0 as *mut ssize_t;
    let mut nrowptr: *mut ssize_t = 0 as *mut ssize_t;
    let mut rowind: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nrowind: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut rowval: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut nrowval: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut avgwgt: libc::c_float = 0.;
    let mut nmat: *mut gk_csr_t = 0 as *mut gk_csr_t;
    nmat = gk_csr_Create();
    (*nmat).nrows = (*mat).nrows;
    (*nmat).ncols = (*mat).ncols;
    nrows = (*mat).nrows;
    rowptr = (*mat).rowptr;
    rowind = (*mat).rowind;
    rowval = (*mat).rowval;
    (*nmat)
        .rowptr = gk_zmalloc(
        (nrows + 1) as size_t,
        b"gk_csr_ZScoreFilter: nrowptr\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    nrowptr = (*nmat).rowptr;
    (*nmat)
        .rowind = gk_imalloc(
        *rowptr.offset(nrows as isize) as size_t,
        b"gk_csr_ZScoreFilter: nrowind\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    nrowind = (*nmat).rowind;
    (*nmat)
        .rowval = gk_fmalloc(
        *rowptr.offset(nrows as isize) as size_t,
        b"gk_csr_ZScoreFilter: nrowval\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    nrowval = (*nmat).rowval;
    match what {
        2 => {
            gk_errexit(
                15 as libc::c_int,
                b"This has not been implemented yet.\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
        1 => {
            if ((*mat).rowptr).is_null() {
                gk_errexit(
                    15 as libc::c_int,
                    b"Cannot filter rows when row-based structure has not been created.\n\0"
                        as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            }
            *nrowptr.offset(0 as libc::c_int as isize) = 0 as libc::c_int as ssize_t;
            nnz = 0 as libc::c_int as ssize_t;
            i = 0 as libc::c_int as ssize_t;
            while i < nrows as i64 {
                avgwgt = zscore
                    / (*rowptr.offset((i + 1 as i64) as isize)
                        - *rowptr.offset(i as isize)) as libc::c_float;
                j = *rowptr.offset(i as isize);
                while j < *rowptr.offset((i + 1 as i64) as isize)
                {
                    if *rowval.offset(j as isize) > avgwgt {
                        *nrowind.offset(nnz as isize) = *rowind.offset(j as isize);
                        *nrowval.offset(nnz as isize) = *rowval.offset(j as isize);
                        nnz += 1;
                        nnz;
                    }
                    j += 1;
                    j;
                }
                *nrowptr.offset((i + 1 as i64) as isize) = nnz;
                i += 1;
                i;
            }
        }
        _ => {
            gk_csr_Free(&mut nmat);
            gk_errexit(
                15 as libc::c_int,
                b"Unknown prunning type of %d\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                what,
            );
            return 0 as *mut gk_csr_t;
        }
    }
    return nmat;
}
#[no_mangle]
pub unsafe extern "C" fn gk_csr_CompactColumns(mut mat: *mut gk_csr_t) {
    let mut i: ssize_t = 0;
    let mut nrows: libc::c_int = 0;
    let mut ncols: libc::c_int = 0;
    let mut nncols: libc::c_int = 0;
    let mut rowptr: *mut ssize_t = 0 as *mut ssize_t;
    let mut rowind: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut colmap: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut clens: *mut gk_ikv_t = 0 as *mut gk_ikv_t;
    nrows = (*mat).nrows;
    ncols = (*mat).ncols;
    rowptr = (*mat).rowptr;
    rowind = (*mat).rowind;
    colmap = gk_imalloc(
        ncols as size_t,
        b"gk_csr_CompactColumns: colmap\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    clens = gk_ikvmalloc(
        ncols as size_t,
        b"gk_csr_CompactColumns: clens\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    i = 0 as libc::c_int as ssize_t;
    while i < ncols as i64 {
        (*clens.offset(i as isize)).key = 0 as libc::c_int;
        (*clens.offset(i as isize)).val = i;
        i += 1;
        i;
    }
    i = 0 as libc::c_int as ssize_t;
    while i < *rowptr.offset(nrows as isize) {
        let ref mut fresh13 = (*clens.offset(*rowind.offset(i as isize) as isize)).key;
        *fresh13 += 1;
        *fresh13;
        i += 1;
        i;
    }
    gk_ikvsortd(ncols as size_t, clens);
    nncols = 0 as libc::c_int;
    i = 0 as libc::c_int as ssize_t;
    while i < ncols as i64 {
        if !((*clens.offset(i as isize)).key > 0 as libc::c_int) {
            break;
        }
        let fresh14 = nncols;
        nncols = nncols + 1;
        *colmap.offset((*clens.offset(i as isize)).val as isize) = fresh14;
        i += 1;
        i;
    }
    i = 0 as libc::c_int as ssize_t;
    while i < *rowptr.offset(nrows as isize) {
        *rowind.offset(i as isize) = *colmap.offset(*rowind.offset(i as isize) as isize);
        i += 1;
        i;
    }
    (*mat).ncols = nncols;
    gk_free(
        &mut colmap as *mut *mut libc::c_int as *mut *mut libc::c_void,
        &mut clens as *mut *mut gk_ikv_t,
        0 as *mut *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gk_csr_SortIndices(
    mut mat: *mut gk_csr_t,
    mut what: libc::c_int,
) {
    let mut n: libc::c_int = 0;
    let mut nn: libc::c_int = 0 as libc::c_int;
    let mut ptr: *mut ssize_t = 0 as *mut ssize_t;
    let mut ind: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut val: *mut libc::c_float = 0 as *mut libc::c_float;
    match what {
        1 => {
            if ((*mat).rowptr).is_null() {
                gk_errexit(
                    15 as libc::c_int,
                    b"Row-based view of the matrix does not exists.\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
            }
            n = (*mat).nrows;
            ptr = (*mat).rowptr;
            ind = (*mat).rowind;
            val = (*mat).rowval;
        }
        2 => {
            if ((*mat).colptr).is_null() {
                gk_errexit(
                    15 as libc::c_int,
                    b"Column-based view of the matrix does not exists.\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
            }
            n = (*mat).ncols;
            ptr = (*mat).colptr;
            ind = (*mat).colind;
            val = (*mat).colval;
        }
        _ => {
            gk_errexit(
                15 as libc::c_int,
                b"Invalid index type of %d.\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                what,
            );
            return;
        }
    }
    let mut i: ssize_t = 0;
    let mut j: ssize_t = 0;
    let mut k: ssize_t = 0;
    let mut cand: *mut gk_ikv_t = 0 as *mut gk_ikv_t;
    let mut tval: *mut libc::c_float = 0 as *mut libc::c_float;
    i = 0 as libc::c_int as ssize_t;
    while i < n as i64 {
        nn = (if nn as i64
            >= *ptr.offset((i + 1 as i64) as isize)
                - *ptr.offset(i as isize)
        {
            nn as i64
        } else {
            *ptr.offset((i + 1 as i64) as isize)
                - *ptr.offset(i as isize)
        }) as libc::c_int;
        i += 1;
        i;
    }
    cand = gk_ikvmalloc(
        nn as size_t,
        b"gk_csr_SortIndices: cand\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    tval = gk_fmalloc(
        nn as size_t,
        b"gk_csr_SortIndices: tval\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    i = 0 as libc::c_int as ssize_t;
    while i < n as i64 {
        k = 0 as libc::c_int as ssize_t;
        j = *ptr.offset(i as isize);
        while j < *ptr.offset((i + 1 as i64) as isize) {
            if j > *ptr.offset(i as isize)
                && *ind.offset(j as isize)
                    < *ind.offset((j - 1 as i64) as isize)
            {
                k = 1 as ssize_t;
            }
            (*cand.offset((j - *ptr.offset(i as isize)) as isize))
                .val = j - *ptr.offset(i as isize);
            (*cand.offset((j - *ptr.offset(i as isize)) as isize))
                .key = *ind.offset(j as isize);
            *tval
                .offset(
                    (j - *ptr.offset(i as isize)) as isize,
                ) = *val.offset(j as isize);
            j += 1;
            j;
        }
        if k != 0 {
            gk_ikvsorti(
                (*ptr.offset((i + 1 as i64) as isize)
                    - *ptr.offset(i as isize)) as size_t,
                cand,
            );
            j = *ptr.offset(i as isize);
            while j < *ptr.offset((i + 1 as i64) as isize) {
                *ind
                    .offset(
                        j as isize,
                    ) = (*cand.offset((j - *ptr.offset(i as isize)) as isize)).key;
                *val
                    .offset(
                        j as isize,
                    ) = *tval
                    .offset(
                        (*cand.offset((j - *ptr.offset(i as isize)) as isize)).val
                            as isize,
                    );
                j += 1;
                j;
            }
        }
        i += 1;
        i;
    }
    gk_free(
        &mut cand as *mut *mut gk_ikv_t as *mut *mut libc::c_void,
        &mut tval as *mut *mut libc::c_float,
        0 as *mut *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gk_csr_CreateIndex(
    mut mat: *mut gk_csr_t,
    mut what: libc::c_int,
) {
    let mut i: ssize_t = 0;
    let mut j: ssize_t = 0;
    let mut k: ssize_t = 0;
    let mut nf: ssize_t = 0;
    let mut nr: ssize_t = 0;
    let mut fptr: *mut ssize_t = 0 as *mut ssize_t;
    let mut rptr: *mut ssize_t = 0 as *mut ssize_t;
    let mut find: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut rind: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut fval: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut rval: *mut libc::c_float = 0 as *mut libc::c_float;
    match what {
        2 => {
            nf = (*mat).nrows as ssize_t;
            fptr = (*mat).rowptr;
            find = (*mat).rowind;
            fval = (*mat).rowval;
            if !((*mat).colptr).is_null() {
                gk_free(
                    &mut (*mat).colptr as *mut *mut ssize_t as *mut *mut libc::c_void,
                    0 as *mut *mut libc::c_void,
                );
            }
            if !((*mat).colind).is_null() {
                gk_free(
                    &mut (*mat).colind as *mut *mut int32_t as *mut *mut libc::c_void,
                    0 as *mut *mut libc::c_void,
                );
            }
            if !((*mat).colval).is_null() {
                gk_free(
                    &mut (*mat).colval as *mut *mut libc::c_float
                        as *mut *mut libc::c_void,
                    0 as *mut *mut libc::c_void,
                );
            }
            nr = (*mat).ncols as ssize_t;
            (*mat)
                .colptr = gk_zsmalloc(
                (nr + 1 as i64) as size_t,
                0 as libc::c_int as ssize_t,
                b"gk_csr_CreateIndex: rptr\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            rptr = (*mat).colptr;
            (*mat)
                .colind = gk_imalloc(
                *fptr.offset(nf as isize) as size_t,
                b"gk_csr_CreateIndex: rind\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            rind = (*mat).colind;
            (*mat)
                .colval = if !fval.is_null() {
                gk_fmalloc(
                    *fptr.offset(nf as isize) as size_t,
                    b"gk_csr_CreateIndex: rval\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                )
            } else {
                0 as *mut libc::c_float
            };
            rval = (*mat).colval;
        }
        1 => {
            nf = (*mat).ncols as ssize_t;
            fptr = (*mat).colptr;
            find = (*mat).colind;
            fval = (*mat).colval;
            if !((*mat).rowptr).is_null() {
                gk_free(
                    &mut (*mat).rowptr as *mut *mut ssize_t as *mut *mut libc::c_void,
                    0 as *mut *mut libc::c_void,
                );
            }
            if !((*mat).rowind).is_null() {
                gk_free(
                    &mut (*mat).rowind as *mut *mut int32_t as *mut *mut libc::c_void,
                    0 as *mut *mut libc::c_void,
                );
            }
            if !((*mat).rowval).is_null() {
                gk_free(
                    &mut (*mat).rowval as *mut *mut libc::c_float
                        as *mut *mut libc::c_void,
                    0 as *mut *mut libc::c_void,
                );
            }
            nr = (*mat).nrows as ssize_t;
            (*mat)
                .rowptr = gk_zsmalloc(
                (nr + 1 as i64) as size_t,
                0 as libc::c_int as ssize_t,
                b"gk_csr_CreateIndex: rptr\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            rptr = (*mat).rowptr;
            (*mat)
                .rowind = gk_imalloc(
                *fptr.offset(nf as isize) as size_t,
                b"gk_csr_CreateIndex: rind\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            rind = (*mat).rowind;
            (*mat)
                .rowval = if !fval.is_null() {
                gk_fmalloc(
                    *fptr.offset(nf as isize) as size_t,
                    b"gk_csr_CreateIndex: rval\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                )
            } else {
                0 as *mut libc::c_float
            };
            rval = (*mat).rowval;
        }
        _ => {
            gk_errexit(
                15 as libc::c_int,
                b"Invalid index type of %d.\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                what,
            );
            return;
        }
    }
    i = 0 as libc::c_int as ssize_t;
    while i < nf {
        j = *fptr.offset(i as isize);
        while j < *fptr.offset((i + 1 as i64) as isize) {
            let ref mut fresh15 = *rptr.offset(*find.offset(j as isize) as isize);
            *fresh15 += 1;
            *fresh15;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    i = 1 as ssize_t;
    while i < nr {
        let ref mut fresh16 = *rptr.offset(i as isize);
        *fresh16 += *rptr.offset((i - 1 as i64) as isize);
        i += 1;
        i;
    }
    i = nr;
    while i > 0 as libc::c_int as i64 {
        *rptr
            .offset(
                i as isize,
            ) = *rptr.offset((i - 1 as i64) as isize);
        i -= 1;
        i;
    }
    *rptr.offset(0 as libc::c_int as isize) = 0 as libc::c_int as ssize_t;
    if *rptr.offset(nr as isize) > 6 as libc::c_int as i64 * nr {
        i = 0 as libc::c_int as ssize_t;
        while i < nf {
            j = *fptr.offset(i as isize);
            while j < *fptr.offset((i + 1 as i64) as isize) {
                let ref mut fresh17 = *rptr.offset(*find.offset(j as isize) as isize);
                let fresh18 = *fresh17;
                *fresh17 = *fresh17 + 1;
                *rind.offset(fresh18 as isize) = i as libc::c_int;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
        i = nr;
        while i > 0 as libc::c_int as i64 {
            *rptr
                .offset(
                    i as isize,
                ) = *rptr.offset((i - 1 as i64) as isize);
            i -= 1;
            i;
        }
        *rptr.offset(0 as libc::c_int as isize) = 0 as libc::c_int as ssize_t;
        if !fval.is_null() {
            i = 0 as libc::c_int as ssize_t;
            while i < nf {
                j = *fptr.offset(i as isize);
                while j < *fptr.offset((i + 1 as i64) as isize) {
                    let ref mut fresh19 = *rptr
                        .offset(*find.offset(j as isize) as isize);
                    let fresh20 = *fresh19;
                    *fresh19 = *fresh19 + 1;
                    *rval.offset(fresh20 as isize) = *fval.offset(j as isize);
                    j += 1;
                    j;
                }
                i += 1;
                i;
            }
            i = nr;
            while i > 0 as libc::c_int as i64 {
                *rptr
                    .offset(
                        i as isize,
                    ) = *rptr.offset((i - 1 as i64) as isize);
                i -= 1;
                i;
            }
            *rptr.offset(0 as libc::c_int as isize) = 0 as libc::c_int as ssize_t;
        }
    } else {
        if !fval.is_null() {
            i = 0 as libc::c_int as ssize_t;
            while i < nf {
                j = *fptr.offset(i as isize);
                while j < *fptr.offset((i + 1 as i64) as isize) {
                    k = *find.offset(j as isize) as ssize_t;
                    *rind.offset(*rptr.offset(k as isize) as isize) = i as libc::c_int;
                    let ref mut fresh21 = *rptr.offset(k as isize);
                    let fresh22 = *fresh21;
                    *fresh21 = *fresh21 + 1;
                    *rval.offset(fresh22 as isize) = *fval.offset(j as isize);
                    j += 1;
                    j;
                }
                i += 1;
                i;
            }
        } else {
            i = 0 as libc::c_int as ssize_t;
            while i < nf {
                j = *fptr.offset(i as isize);
                while j < *fptr.offset((i + 1 as i64) as isize) {
                    let ref mut fresh23 = *rptr
                        .offset(*find.offset(j as isize) as isize);
                    let fresh24 = *fresh23;
                    *fresh23 = *fresh23 + 1;
                    *rind.offset(fresh24 as isize) = i as libc::c_int;
                    j += 1;
                    j;
                }
                i += 1;
                i;
            }
        }
        i = nr;
        while i > 0 as libc::c_int as i64 {
            *rptr
                .offset(
                    i as isize,
                ) = *rptr.offset((i - 1 as i64) as isize);
            i -= 1;
            i;
        }
        *rptr.offset(0 as libc::c_int as isize) = 0 as libc::c_int as ssize_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_csr_Normalize(
    mut mat: *mut gk_csr_t,
    mut what: libc::c_int,
    mut norm: libc::c_int,
) {
    let mut i: ssize_t = 0;
    let mut j: ssize_t = 0;
    let mut n: libc::c_int = 0;
    let mut ptr: *mut ssize_t = 0 as *mut ssize_t;
    let mut val: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut sum: libc::c_float = 0.;
    if what & 1 != 0 && !((*mat).rowval).is_null() {
        n = (*mat).nrows;
        ptr = (*mat).rowptr;
        val = (*mat).rowval;
        i = 0 as libc::c_int as ssize_t;
        while i < n as i64 {
            sum = 0.0f64 as libc::c_float;
            j = *ptr.offset(i as isize);
            while j < *ptr.offset((i + 1 as i64) as isize) {
                if norm == 2 as libc::c_int {
                    sum += *val.offset(j as isize) * *val.offset(j as isize);
                } else if norm == 1 {
                    sum += *val.offset(j as isize);
                }
                j += 1;
                j;
            }
            if sum > 0 as libc::c_int as libc::c_float {
                if norm == 2 as libc::c_int {
                    sum = (1.0f64 / sqrt(sum as libc::c_double)) as libc::c_float;
                } else if norm == 1 {
                    sum = (1.0f64 / sum as libc::c_double) as libc::c_float;
                }
                j = *ptr.offset(i as isize);
                while j < *ptr.offset((i + 1 as i64) as isize) {
                    *val.offset(j as isize) *= sum;
                    j += 1;
                    j;
                }
            }
            i += 1;
            i;
        }
    }
    if what & 2 as libc::c_int != 0 && !((*mat).colval).is_null() {
        n = (*mat).ncols;
        ptr = (*mat).colptr;
        val = (*mat).colval;
        i = 0 as libc::c_int as ssize_t;
        while i < n as i64 {
            sum = 0.0f64 as libc::c_float;
            j = *ptr.offset(i as isize);
            while j < *ptr.offset((i + 1 as i64) as isize) {
                if norm == 2 as libc::c_int {
                    sum += *val.offset(j as isize) * *val.offset(j as isize);
                } else if norm == 1 {
                    sum += *val.offset(j as isize);
                }
                j += 1;
                j;
            }
            if sum > 0 as libc::c_int as libc::c_float {
                if norm == 2 as libc::c_int {
                    sum = (1.0f64 / sqrt(sum as libc::c_double)) as libc::c_float;
                } else if norm == 1 {
                    sum = (1.0f64 / sum as libc::c_double) as libc::c_float;
                }
                j = *ptr.offset(i as isize);
                while j < *ptr.offset((i + 1 as i64) as isize) {
                    *val.offset(j as isize) *= sum;
                    j += 1;
                    j;
                }
            }
            i += 1;
            i;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn gk_csr_Scale(mut mat: *mut gk_csr_t, mut type_0: libc::c_int) {
    let mut i: ssize_t = 0;
    let mut j: ssize_t = 0;
    let mut nrows: libc::c_int = 0;
    let mut ncols: libc::c_int = 0;
    let mut nnzcols: libc::c_int = 0;
    let mut bgfreq: libc::c_int = 0;
    let mut rowptr: *mut ssize_t = 0 as *mut ssize_t;
    let mut rowind: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut collen: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut rowval: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut cscale: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut maxtf: libc::c_float = 0.;
    nrows = (*mat).nrows;
    rowptr = (*mat).rowptr;
    rowind = (*mat).rowind;
    rowval = (*mat).rowval;
    match type_0 {
        1 => {
            i = 0 as libc::c_int as ssize_t;
            while i < nrows as i64 {
                maxtf = fabs(
                    *rowval.offset(*rowptr.offset(i as isize) as isize) as libc::c_double,
                ) as libc::c_float;
                j = *rowptr.offset(i as isize);
                while j < *rowptr.offset((i + 1 as i64) as isize)
                {
                    maxtf = (if (maxtf as libc::c_double)
                        < fabs(*rowval.offset(j as isize) as libc::c_double)
                    {
                        fabs(*rowval.offset(j as isize) as libc::c_double)
                    } else {
                        maxtf as libc::c_double
                    }) as libc::c_float;
                    j += 1;
                    j;
                }
                j = *rowptr.offset(i as isize);
                while j < *rowptr.offset((i + 1 as i64) as isize)
                {
                    *rowval
                        .offset(
                            j as isize,
                        ) = (0.5f64
                        + 0.5f64 * *rowval.offset(j as isize) as libc::c_double
                            / maxtf as libc::c_double) as libc::c_float;
                    j += 1;
                    j;
                }
                i += 1;
                i;
            }
        }
        10 => {
            i = 0 as libc::c_int as ssize_t;
            while i < nrows as i64 {
                maxtf = fabs(
                    *rowval.offset(*rowptr.offset(i as isize) as isize) as libc::c_double,
                ) as libc::c_float;
                j = *rowptr.offset(i as isize);
                while j < *rowptr.offset((i + 1 as i64) as isize)
                {
                    maxtf = (if (maxtf as libc::c_double)
                        < fabs(*rowval.offset(j as isize) as libc::c_double)
                    {
                        fabs(*rowval.offset(j as isize) as libc::c_double)
                    } else {
                        maxtf as libc::c_double
                    }) as libc::c_float;
                    j += 1;
                    j;
                }
                j = *rowptr.offset(i as isize);
                while j < *rowptr.offset((i + 1 as i64) as isize)
                {
                    *rowval
                        .offset(
                            j as isize,
                        ) = (0.1f64
                        + 0.9f64 * *rowval.offset(j as isize) as libc::c_double
                            / maxtf as libc::c_double) as libc::c_float;
                    j += 1;
                    j;
                }
                i += 1;
                i;
            }
        }
        2 => {
            i = 0 as libc::c_int as ssize_t;
            while i < nrows as i64 {
                j = *rowptr.offset(i as isize);
                while j < *rowptr.offset((i + 1 as i64) as isize)
                {
                    if *rowval.offset(j as isize) as libc::c_double != 0.0f64 {
                        *rowval
                            .offset(
                                j as isize,
                            ) = (0.1f64
                            + (if *rowval.offset(j as isize)
                                >= 0 as libc::c_int as libc::c_float
                            {
                                sqrt(fabs(*rowval.offset(j as isize) as libc::c_double))
                            } else {
                                -sqrt(fabs(*rowval.offset(j as isize) as libc::c_double))
                            })) as libc::c_float;
                    }
                    j += 1;
                    j;
                }
                i += 1;
                i;
            }
        }
        3 => {
            i = 0 as libc::c_int as ssize_t;
            while i < nrows as i64 {
                j = *rowptr.offset(i as isize);
                while j < *rowptr.offset((i + 1 as i64) as isize)
                {
                    if *rowval.offset(j as isize) as libc::c_double != 0.0f64 {
                        *rowval
                            .offset(
                                j as isize,
                            ) = (0.1f64
                            + (if *rowval.offset(j as isize)
                                >= 0 as libc::c_int as libc::c_float
                            {
                                sqrt(
                                    sqrt(fabs(*rowval.offset(j as isize) as libc::c_double)),
                                )
                            } else {
                                -sqrt(
                                    sqrt(fabs(*rowval.offset(j as isize) as libc::c_double)),
                                )
                            })) as libc::c_float;
                    }
                    j += 1;
                    j;
                }
                i += 1;
                i;
            }
        }
        4 => {
            i = 0 as libc::c_int as ssize_t;
            while i < nrows as i64 {
                j = *rowptr.offset(i as isize);
                while j < *rowptr.offset((i + 1 as i64) as isize)
                {
                    if *rowval.offset(j as isize) as libc::c_double != 0.0f64 {
                        *rowval
                            .offset(
                                j as isize,
                            ) = (0.1f64
                            + (if *rowval.offset(j as isize)
                                >= 0 as libc::c_int as libc::c_float
                            {
                                powf(
                                    fabs(*rowval.offset(j as isize) as libc::c_double)
                                        as libc::c_float,
                                    0.65f64 as libc::c_float,
                                )
                            } else {
                                -powf(
                                    fabs(*rowval.offset(j as isize) as libc::c_double)
                                        as libc::c_float,
                                    0.65f64 as libc::c_float,
                                )
                            }) as libc::c_double) as libc::c_float;
                    }
                    j += 1;
                    j;
                }
                i += 1;
                i;
            }
        }
        5 => {
            i = 0 as libc::c_int as ssize_t;
            while i < nrows as i64 {
                j = *rowptr.offset(i as isize);
                while j < *rowptr.offset((i + 1 as i64) as isize)
                {
                    if *rowval.offset(j as isize) as libc::c_double != 0.0f64 {
                        *rowval
                            .offset(
                                j as isize,
                            ) = (0.1f64
                            + (if *rowval.offset(j as isize)
                                >= 0 as libc::c_int as libc::c_float
                            {
                                powf(
                                    fabs(*rowval.offset(j as isize) as libc::c_double)
                                        as libc::c_float,
                                    0.75f64 as libc::c_float,
                                )
                            } else {
                                -powf(
                                    fabs(*rowval.offset(j as isize) as libc::c_double)
                                        as libc::c_float,
                                    0.75f64 as libc::c_float,
                                )
                            }) as libc::c_double) as libc::c_float;
                    }
                    j += 1;
                    j;
                }
                i += 1;
                i;
            }
        }
        6 => {
            i = 0 as libc::c_int as ssize_t;
            while i < nrows as i64 {
                j = *rowptr.offset(i as isize);
                while j < *rowptr.offset((i + 1 as i64) as isize)
                {
                    if *rowval.offset(j as isize) as libc::c_double != 0.0f64 {
                        *rowval
                            .offset(
                                j as isize,
                            ) = (0.1f64
                            + (if *rowval.offset(j as isize)
                                >= 0 as libc::c_int as libc::c_float
                            {
                                powf(
                                    fabs(*rowval.offset(j as isize) as libc::c_double)
                                        as libc::c_float,
                                    0.85f64 as libc::c_float,
                                )
                            } else {
                                -powf(
                                    fabs(*rowval.offset(j as isize) as libc::c_double)
                                        as libc::c_float,
                                    0.85f64 as libc::c_float,
                                )
                            }) as libc::c_double) as libc::c_float;
                    }
                    j += 1;
                    j;
                }
                i += 1;
                i;
            }
        }
        7 => {
            let mut logscale: libc::c_double = 1.0f64 / log(2.0f64);
            i = 0 as libc::c_int as ssize_t;
            while i < *rowptr.offset(nrows as isize) {
                if *rowval.offset(i as isize) as libc::c_double != 0.0f64 {
                    *rowval
                        .offset(
                            i as isize,
                        ) = (1 as libc::c_double
                        + (if *rowval.offset(i as isize) as libc::c_double > 0.0f64 {
                            log(*rowval.offset(i as isize) as libc::c_double)
                        } else {
                            -log(-*rowval.offset(i as isize) as libc::c_double)
                        }) * logscale) as libc::c_float;
                }
                i += 1;
                i;
            }
        }
        8 => {
            ncols = (*mat).ncols;
            cscale = gk_fmalloc(
                ncols as size_t,
                b"gk_csr_Scale: cscale\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            collen = gk_ismalloc(
                ncols as size_t,
                0 as libc::c_int,
                b"gk_csr_Scale: collen\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            i = 0 as libc::c_int as ssize_t;
            while i < nrows as i64 {
                j = *rowptr.offset(i as isize);
                while j < *rowptr.offset((i + 1 as i64) as isize)
                {
                    let ref mut fresh25 = *collen
                        .offset(*rowind.offset(j as isize) as isize);
                    *fresh25 += 1;
                    *fresh25;
                    j += 1;
                    j;
                }
                i += 1;
                i;
            }
            i = 0 as libc::c_int as ssize_t;
            while i < ncols as i64 {
                *cscale
                    .offset(
                        i as isize,
                    ) = (if *collen.offset(i as isize) > 0 as libc::c_int {
                    log(
                        1.0f64 * nrows as libc::c_double
                            / *collen.offset(i as isize) as libc::c_double,
                    )
                } else {
                    0.0f64
                }) as libc::c_float;
                i += 1;
                i;
            }
            i = 0 as libc::c_int as ssize_t;
            while i < nrows as i64 {
                j = *rowptr.offset(i as isize);
                while j < *rowptr.offset((i + 1 as i64) as isize)
                {
                    *rowval.offset(j as isize)
                        *= *cscale.offset(*rowind.offset(j as isize) as isize);
                    j += 1;
                    j;
                }
                i += 1;
                i;
            }
            gk_free(
                &mut cscale as *mut *mut libc::c_float as *mut *mut libc::c_void,
                &mut collen as *mut *mut libc::c_int,
                0 as *mut *mut libc::c_void,
            );
        }
        9 => {
            ncols = (*mat).ncols;
            cscale = gk_fmalloc(
                ncols as size_t,
                b"gk_csr_Scale: cscale\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            collen = gk_ismalloc(
                ncols as size_t,
                0 as libc::c_int,
                b"gk_csr_Scale: collen\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            i = 0 as libc::c_int as ssize_t;
            while i < nrows as i64 {
                j = *rowptr.offset(i as isize);
                while j < *rowptr.offset((i + 1 as i64) as isize)
                {
                    let ref mut fresh26 = *collen
                        .offset(*rowind.offset(j as isize) as isize);
                    *fresh26 += 1;
                    *fresh26;
                    j += 1;
                    j;
                }
                i += 1;
                i;
            }
            nnzcols = 0 as libc::c_int;
            i = 0 as libc::c_int as ssize_t;
            while i < ncols as i64 {
                nnzcols
                    += if *collen.offset(i as isize) > 0 as libc::c_int {
                        1
                    } else {
                        0 as libc::c_int
                    };
                i += 1;
                i;
            }
            bgfreq = (if 10 as libc::c_int as i64
                >= (0.5f64 * *rowptr.offset(nrows as isize) as libc::c_double
                    / nnzcols as libc::c_double) as ssize_t
            {
                10 as libc::c_int as i64
            } else {
                (0.5f64 * *rowptr.offset(nrows as isize) as libc::c_double
                    / nnzcols as libc::c_double) as ssize_t
            }) as libc::c_int;
            printf(
                b"nnz: %zd, nnzcols: %d, bgfreq: %d\n\0" as *const u8
                    as *const libc::c_char,
                *rowptr.offset(nrows as isize),
                nnzcols,
                bgfreq,
            );
            i = 0 as libc::c_int as ssize_t;
            while i < ncols as i64 {
                *cscale
                    .offset(
                        i as isize,
                    ) = (if *collen.offset(i as isize) > 0 as libc::c_int {
                    log(
                        1.0f64 * (nrows + 2 as libc::c_int * bgfreq) as libc::c_double
                            / (bgfreq + *collen.offset(i as isize)) as libc::c_double,
                    )
                } else {
                    0.0f64
                }) as libc::c_float;
                i += 1;
                i;
            }
            i = 0 as libc::c_int as ssize_t;
            while i < nrows as i64 {
                j = *rowptr.offset(i as isize);
                while j < *rowptr.offset((i + 1 as i64) as isize)
                {
                    *rowval.offset(j as isize)
                        *= *cscale.offset(*rowind.offset(j as isize) as isize);
                    j += 1;
                    j;
                }
                i += 1;
                i;
            }
            gk_free(
                &mut cscale as *mut *mut libc::c_float as *mut *mut libc::c_void,
                &mut collen as *mut *mut libc::c_int,
                0 as *mut *mut libc::c_void,
            );
        }
        _ => {
            gk_errexit(
                15 as libc::c_int,
                b"Unknown scaling type of %d\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                type_0,
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_csr_ComputeSums(
    mut mat: *mut gk_csr_t,
    mut what: libc::c_int,
) {
    let mut i: ssize_t = 0;
    let mut n: libc::c_int = 0;
    let mut ptr: *mut ssize_t = 0 as *mut ssize_t;
    let mut val: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut sums: *mut libc::c_float = 0 as *mut libc::c_float;
    match what {
        1 => {
            n = (*mat).nrows;
            ptr = (*mat).rowptr;
            val = (*mat).rowval;
            if !((*mat).rsums).is_null() {
                gk_free(
                    &mut (*mat).rsums as *mut *mut libc::c_float
                        as *mut *mut libc::c_void,
                    0 as *mut *mut libc::c_void,
                );
            }
            (*mat)
                .rsums = gk_fsmalloc(
                n as size_t,
                0 as libc::c_int as libc::c_float,
                b"gk_csr_ComputeSums: sums\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            sums = (*mat).rsums;
        }
        2 => {
            n = (*mat).ncols;
            ptr = (*mat).colptr;
            val = (*mat).colval;
            if !((*mat).csums).is_null() {
                gk_free(
                    &mut (*mat).csums as *mut *mut libc::c_float
                        as *mut *mut libc::c_void,
                    0 as *mut *mut libc::c_void,
                );
            }
            (*mat)
                .csums = gk_fsmalloc(
                n as size_t,
                0 as libc::c_int as libc::c_float,
                b"gk_csr_ComputeSums: sums\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            sums = (*mat).csums;
        }
        _ => {
            gk_errexit(
                15 as libc::c_int,
                b"Invalid sum type of %d.\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                what,
            );
            return;
        }
    }
    i = 0 as libc::c_int as ssize_t;
    while i < n as i64 {
        *sums
            .offset(
                i as isize,
            ) = gk_fsum(
            (*ptr.offset((i + 1 as i64) as isize)
                - *ptr.offset(i as isize)) as size_t,
            val.offset(*ptr.offset(i as isize) as isize),
            1 as size_t,
        );
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn gk_csr_ComputeSquaredNorms(
    mut mat: *mut gk_csr_t,
    mut what: libc::c_int,
) {
    let mut i: ssize_t = 0;
    let mut n: libc::c_int = 0;
    let mut ptr: *mut ssize_t = 0 as *mut ssize_t;
    let mut val: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut norms: *mut libc::c_float = 0 as *mut libc::c_float;
    match what {
        1 => {
            n = (*mat).nrows;
            ptr = (*mat).rowptr;
            val = (*mat).rowval;
            if !((*mat).rnorms).is_null() {
                gk_free(
                    &mut (*mat).rnorms as *mut *mut libc::c_float
                        as *mut *mut libc::c_void,
                    0 as *mut *mut libc::c_void,
                );
            }
            (*mat)
                .rnorms = gk_fsmalloc(
                n as size_t,
                0 as libc::c_int as libc::c_float,
                b"gk_csr_ComputeSums: norms\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            norms = (*mat).rnorms;
        }
        2 => {
            n = (*mat).ncols;
            ptr = (*mat).colptr;
            val = (*mat).colval;
            if !((*mat).cnorms).is_null() {
                gk_free(
                    &mut (*mat).cnorms as *mut *mut libc::c_float
                        as *mut *mut libc::c_void,
                    0 as *mut *mut libc::c_void,
                );
            }
            (*mat)
                .cnorms = gk_fsmalloc(
                n as size_t,
                0 as libc::c_int as libc::c_float,
                b"gk_csr_ComputeSums: norms\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            norms = (*mat).cnorms;
        }
        _ => {
            gk_errexit(
                15 as libc::c_int,
                b"Invalid norm type of %d.\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                what,
            );
            return;
        }
    }
    i = 0 as libc::c_int as ssize_t;
    while i < n as i64 {
        *norms
            .offset(
                i as isize,
            ) = gk_fdot(
            (*ptr.offset((i + 1 as i64) as isize)
                - *ptr.offset(i as isize)) as size_t,
            val.offset(*ptr.offset(i as isize) as isize),
            1 as size_t,
            val.offset(*ptr.offset(i as isize) as isize),
            1 as size_t,
        );
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn gk_csr_ComputeSimilarity(
    mut mat: *mut gk_csr_t,
    mut i1: libc::c_int,
    mut i2: libc::c_int,
    mut what: libc::c_int,
    mut simtype: libc::c_int,
) -> libc::c_float {
    let mut nind1: libc::c_int = 0;
    let mut nind2: libc::c_int = 0;
    let mut ind1: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ind2: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut val1: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut val2: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut stat1: libc::c_float = 0.;
    let mut stat2: libc::c_float = 0.;
    let mut sim: libc::c_float = 0.;
    match what {
        1 => {
            if ((*mat).rowptr).is_null() {
                gk_errexit(
                    15 as libc::c_int,
                    b"Row-based view of the matrix does not exists.\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
            }
            nind1 = (*((*mat).rowptr).offset((i1 + 1) as isize)
                - *((*mat).rowptr).offset(i1 as isize)) as libc::c_int;
            nind2 = (*((*mat).rowptr).offset((i2 + 1) as isize)
                - *((*mat).rowptr).offset(i2 as isize)) as libc::c_int;
            ind1 = ((*mat).rowind).offset(*((*mat).rowptr).offset(i1 as isize) as isize);
            ind2 = ((*mat).rowind).offset(*((*mat).rowptr).offset(i2 as isize) as isize);
            val1 = ((*mat).rowval).offset(*((*mat).rowptr).offset(i1 as isize) as isize);
            val2 = ((*mat).rowval).offset(*((*mat).rowptr).offset(i2 as isize) as isize);
        }
        2 => {
            if ((*mat).colptr).is_null() {
                gk_errexit(
                    15 as libc::c_int,
                    b"Column-based view of the matrix does not exists.\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
            }
            nind1 = (*((*mat).colptr).offset((i1 + 1) as isize)
                - *((*mat).colptr).offset(i1 as isize)) as libc::c_int;
            nind2 = (*((*mat).colptr).offset((i2 + 1) as isize)
                - *((*mat).colptr).offset(i2 as isize)) as libc::c_int;
            ind1 = ((*mat).colind).offset(*((*mat).colptr).offset(i1 as isize) as isize);
            ind2 = ((*mat).colind).offset(*((*mat).colptr).offset(i2 as isize) as isize);
            val1 = ((*mat).colval).offset(*((*mat).colptr).offset(i1 as isize) as isize);
            val2 = ((*mat).colval).offset(*((*mat).colptr).offset(i2 as isize) as isize);
        }
        _ => {
            gk_errexit(
                15 as libc::c_int,
                b"Invalid index type of %d.\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                what,
            );
            return 0.0f64 as libc::c_float;
        }
    }
    match simtype {
        1 | 2 => {
            stat2 = 0.0f64 as libc::c_float;
            stat1 = stat2;
            sim = stat1;
            i2 = 0 as libc::c_int;
            i1 = i2;
            while i1 < nind1 && i2 < nind2 {
                if i1 == nind1 {
                    stat2 += *val2.offset(i2 as isize) * *val2.offset(i2 as isize);
                    i2 += 1;
                    i2;
                } else if i2 == nind2 {
                    stat1 += *val1.offset(i1 as isize) * *val1.offset(i1 as isize);
                    i1 += 1;
                    i1;
                } else if *ind1.offset(i1 as isize) < *ind2.offset(i2 as isize) {
                    stat1 += *val1.offset(i1 as isize) * *val1.offset(i1 as isize);
                    i1 += 1;
                    i1;
                } else if *ind1.offset(i1 as isize) > *ind2.offset(i2 as isize) {
                    stat2 += *val2.offset(i2 as isize) * *val2.offset(i2 as isize);
                    i2 += 1;
                    i2;
                } else {
                    sim += *val1.offset(i1 as isize) * *val2.offset(i2 as isize);
                    stat1 += *val1.offset(i1 as isize) * *val1.offset(i1 as isize);
                    stat2 += *val2.offset(i2 as isize) * *val2.offset(i2 as isize);
                    i1 += 1;
                    i1;
                    i2 += 1;
                    i2;
                }
            }
            if simtype == 1 {
                sim = (if (stat1 * stat2) as libc::c_double > 0.0f64 {
                    sim as libc::c_double / sqrt((stat1 * stat2) as libc::c_double)
                } else {
                    0.0f64
                }) as libc::c_float;
            } else {
                sim = (if (stat1 + stat2 - sim) as libc::c_double > 0.0f64 {
                    (sim / (stat1 + stat2 - sim)) as libc::c_double
                } else {
                    0.0f64
                }) as libc::c_float;
            }
        }
        3 => {
            stat2 = 0.0f64 as libc::c_float;
            stat1 = stat2;
            sim = stat1;
            i2 = 0 as libc::c_int;
            i1 = i2;
            while i1 < nind1 && i2 < nind2 {
                if i1 == nind1 {
                    stat2 += *val2.offset(i2 as isize);
                    i2 += 1;
                    i2;
                } else if i2 == nind2 {
                    stat1 += *val1.offset(i1 as isize);
                    i1 += 1;
                    i1;
                } else if *ind1.offset(i1 as isize) < *ind2.offset(i2 as isize) {
                    stat1 += *val1.offset(i1 as isize);
                    i1 += 1;
                    i1;
                } else if *ind1.offset(i1 as isize) > *ind2.offset(i2 as isize) {
                    stat2 += *val2.offset(i2 as isize);
                    i2 += 1;
                    i2;
                } else {
                    sim
                        += if *val1.offset(i1 as isize) >= *val2.offset(i2 as isize) {
                            *val2.offset(i2 as isize)
                        } else {
                            *val1.offset(i1 as isize)
                        };
                    stat1 += *val1.offset(i1 as isize);
                    stat2 += *val2.offset(i2 as isize);
                    i1 += 1;
                    i1;
                    i2 += 1;
                    i2;
                }
            }
            sim = (if (stat1 + stat2 - sim) as libc::c_double > 0.0f64 {
                (sim / (stat1 + stat2 - sim)) as libc::c_double
            } else {
                0.0f64
            }) as libc::c_float;
        }
        4 => {
            stat2 = 0.0f64 as libc::c_float;
            stat1 = stat2;
            sim = stat1;
            i2 = 0 as libc::c_int;
            i1 = i2;
            while i1 < nind1 && i2 < nind2 {
                if i1 == nind1 {
                    stat2 += *val2.offset(i2 as isize);
                    i2 += 1;
                    i2;
                } else if i2 == nind2 {
                    stat1 += *val1.offset(i1 as isize);
                    i1 += 1;
                    i1;
                } else if *ind1.offset(i1 as isize) < *ind2.offset(i2 as isize) {
                    stat1 += *val1.offset(i1 as isize);
                    i1 += 1;
                    i1;
                } else if *ind1.offset(i1 as isize) > *ind2.offset(i2 as isize) {
                    stat2 += *val2.offset(i2 as isize);
                    i2 += 1;
                    i2;
                } else {
                    sim
                        += if *val1.offset(i1 as isize) >= *val2.offset(i2 as isize) {
                            *val2.offset(i2 as isize)
                        } else {
                            *val1.offset(i1 as isize)
                        };
                    stat1 += *val1.offset(i1 as isize);
                    stat2 += *val2.offset(i2 as isize);
                    i1 += 1;
                    i1;
                    i2 += 1;
                    i2;
                }
            }
            sim = (if stat1 as libc::c_double > 0.0f64 {
                (sim / stat1) as libc::c_double
            } else {
                0.0f64
            }) as libc::c_float;
        }
        _ => {
            gk_errexit(
                15 as libc::c_int,
                b"Unknown similarity measure %d\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                simtype,
            );
            return -(1) as libc::c_float;
        }
    }
    return sim;
}
#[no_mangle]
pub unsafe extern "C" fn gk_csr_GetSimilarRows(
    mut mat: *mut gk_csr_t,
    mut nqterms: libc::c_int,
    mut qind: *mut libc::c_int,
    mut qval: *mut libc::c_float,
    mut simtype: libc::c_int,
    mut nsim: libc::c_int,
    mut minsim: libc::c_float,
    mut hits: *mut gk_fkv_t,
    mut i_marker: *mut libc::c_int,
    mut i_cand: *mut gk_fkv_t,
) -> libc::c_int {
    let mut i: ssize_t = 0;
    let mut ii: ssize_t = 0;
    let mut j: ssize_t = 0;
    let mut k: ssize_t = 0;
    let mut nrows: libc::c_int = 0;
    let mut ncols: libc::c_int = 0;
    let mut ncand: libc::c_int = 0;
    let mut colptr: *mut ssize_t = 0 as *mut ssize_t;
    let mut colind: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut marker: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut colval: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut rnorms: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut mynorm: libc::c_float = 0.;
    let mut rsums: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut mysum: libc::c_float = 0.;
    let mut cand: *mut gk_fkv_t = 0 as *mut gk_fkv_t;
    if nqterms == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    nrows = (*mat).nrows;
    ncols = (*mat).ncols;
    colptr = (*mat).colptr;
    colind = (*mat).colind;
    colval = (*mat).colval;
    marker = if !i_marker.is_null() {
        i_marker
    } else {
        gk_ismalloc(
            nrows as size_t,
            -(1),
            b"gk_csr_SimilarRows: marker\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        )
    };
    cand = if !i_cand.is_null() {
        i_cand
    } else {
        gk_fkvmalloc(
            nrows as size_t,
            b"gk_csr_SimilarRows: cand\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        )
    };
    match simtype {
        1 => {
            ncand = 0 as libc::c_int;
            ii = 0 as libc::c_int as ssize_t;
            while ii < nqterms as i64 {
                i = *qind.offset(ii as isize) as ssize_t;
                if i < ncols as i64 {
                    j = *colptr.offset(i as isize);
                    while j
                        < *colptr.offset((i + 1 as i64) as isize)
                    {
                        k = *colind.offset(j as isize) as ssize_t;
                        if *marker.offset(k as isize) == -(1) {
                            (*cand.offset(ncand as isize)).val = k;
                            (*cand.offset(ncand as isize))
                                .key = 0 as libc::c_int as libc::c_float;
                            let fresh27 = ncand;
                            ncand = ncand + 1;
                            *marker.offset(k as isize) = fresh27;
                        }
                        (*cand.offset(*marker.offset(k as isize) as isize)).key
                            += *colval.offset(j as isize) * *qval.offset(ii as isize);
                        j += 1;
                        j;
                    }
                }
                ii += 1;
                ii;
            }
        }
        2 => {
            ncand = 0 as libc::c_int;
            ii = 0 as libc::c_int as ssize_t;
            while ii < nqterms as i64 {
                i = *qind.offset(ii as isize) as ssize_t;
                if i < ncols as i64 {
                    j = *colptr.offset(i as isize);
                    while j
                        < *colptr.offset((i + 1 as i64) as isize)
                    {
                        k = *colind.offset(j as isize) as ssize_t;
                        if *marker.offset(k as isize) == -(1) {
                            (*cand.offset(ncand as isize)).val = k;
                            (*cand.offset(ncand as isize))
                                .key = 0 as libc::c_int as libc::c_float;
                            let fresh28 = ncand;
                            ncand = ncand + 1;
                            *marker.offset(k as isize) = fresh28;
                        }
                        (*cand.offset(*marker.offset(k as isize) as isize)).key
                            += *colval.offset(j as isize) * *qval.offset(ii as isize);
                        j += 1;
                        j;
                    }
                }
                ii += 1;
                ii;
            }
            rnorms = (*mat).rnorms;
            mynorm = gk_fdot(
                nqterms as size_t,
                qval,
                1 as size_t,
                qval,
                1 as size_t,
            );
            i = 0 as libc::c_int as ssize_t;
            while i < ncand as i64 {
                (*cand.offset(i as isize))
                    .key = (*cand.offset(i as isize)).key
                    / (*rnorms.offset((*cand.offset(i as isize)).val as isize) + mynorm
                        - (*cand.offset(i as isize)).key);
                i += 1;
                i;
            }
        }
        3 => {
            ncand = 0 as libc::c_int;
            ii = 0 as libc::c_int as ssize_t;
            while ii < nqterms as i64 {
                i = *qind.offset(ii as isize) as ssize_t;
                if i < ncols as i64 {
                    j = *colptr.offset(i as isize);
                    while j
                        < *colptr.offset((i + 1 as i64) as isize)
                    {
                        k = *colind.offset(j as isize) as ssize_t;
                        if *marker.offset(k as isize) == -(1) {
                            (*cand.offset(ncand as isize)).val = k;
                            (*cand.offset(ncand as isize))
                                .key = 0 as libc::c_int as libc::c_float;
                            let fresh29 = ncand;
                            ncand = ncand + 1;
                            *marker.offset(k as isize) = fresh29;
                        }
                        (*cand.offset(*marker.offset(k as isize) as isize)).key
                            += if *colval.offset(j as isize) >= *qval.offset(ii as isize)
                            {
                                *qval.offset(ii as isize)
                            } else {
                                *colval.offset(j as isize)
                            };
                        j += 1;
                        j;
                    }
                }
                ii += 1;
                ii;
            }
            rsums = (*mat).rsums;
            mysum = gk_fsum(nqterms as size_t, qval, 1 as size_t);
            i = 0 as libc::c_int as ssize_t;
            while i < ncand as i64 {
                (*cand.offset(i as isize))
                    .key = (*cand.offset(i as isize)).key
                    / (*rsums.offset((*cand.offset(i as isize)).val as isize) + mysum
                        - (*cand.offset(i as isize)).key);
                i += 1;
                i;
            }
        }
        4 => {
            ncand = 0 as libc::c_int;
            ii = 0 as libc::c_int as ssize_t;
            while ii < nqterms as i64 {
                i = *qind.offset(ii as isize) as ssize_t;
                if i < ncols as i64 {
                    j = *colptr.offset(i as isize);
                    while j
                        < *colptr.offset((i + 1 as i64) as isize)
                    {
                        k = *colind.offset(j as isize) as ssize_t;
                        if *marker.offset(k as isize) == -(1) {
                            (*cand.offset(ncand as isize)).val = k;
                            (*cand.offset(ncand as isize))
                                .key = 0 as libc::c_int as libc::c_float;
                            let fresh30 = ncand;
                            ncand = ncand + 1;
                            *marker.offset(k as isize) = fresh30;
                        }
                        (*cand.offset(*marker.offset(k as isize) as isize)).key
                            += if *colval.offset(j as isize) >= *qval.offset(ii as isize)
                            {
                                *qval.offset(ii as isize)
                            } else {
                                *colval.offset(j as isize)
                            };
                        j += 1;
                        j;
                    }
                }
                ii += 1;
                ii;
            }
            mysum = gk_fsum(nqterms as size_t, qval, 1 as size_t);
            i = 0 as libc::c_int as ssize_t;
            while i < ncand as i64 {
                (*cand.offset(i as isize)).key = (*cand.offset(i as isize)).key / mysum;
                i += 1;
                i;
            }
        }
        _ => {
            gk_errexit(
                15 as libc::c_int,
                b"Unknown similarity measure %d\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                simtype,
            );
            return -(1);
        }
    }
    j = 0 as libc::c_int as ssize_t;
    i = 0 as libc::c_int as ssize_t;
    while i < ncand as i64 {
        *marker.offset((*cand.offset(i as isize)).val as isize) = -(1);
        if (*cand.offset(i as isize)).key >= minsim {
            let fresh31 = j;
            j = j + 1;
            *cand.offset(fresh31 as isize) = *cand.offset(i as isize);
        }
        i += 1;
        i;
    }
    ncand = j as libc::c_int;
    if nsim == -(1) || nsim >= ncand {
        nsim = ncand;
    } else {
        nsim = if nsim >= ncand { ncand } else { nsim };
        gk_dfkvkselect(ncand as size_t, nsim, cand);
        gk_fkvsortd(nsim as size_t, cand);
    }
    gk_fkvcopy(nsim as size_t, cand, hits);
    if i_marker.is_null() {
        gk_free(
            &mut marker as *mut *mut libc::c_int as *mut *mut libc::c_void,
            0 as *mut *mut libc::c_void,
        );
    }
    if i_cand.is_null() {
        gk_free(
            &mut cand as *mut *mut gk_fkv_t as *mut *mut libc::c_void,
            0 as *mut *mut libc::c_void,
        );
    }
    return nsim;
}
