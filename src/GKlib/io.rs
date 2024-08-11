use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __getdelim(
        __lineptr: *mut *mut libc::c_char,
        __n: *mut size_t,
        __delimiter: libc::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn fread(_: *mut libc::c_void, _: u64, _: u64, _: *mut FILE) -> u64;
    fn fwrite(_: *const libc::c_void, _: u64, _: u64, _: *mut FILE) -> u64;
    fn perror(__s: *const libc::c_char);
    fn errexit(_: *mut libc::c_char, _: ...);
    fn gk_free(ptr1: *mut *mut libc::c_void, _: ...);
    fn gk_strdup(orgstr: *mut libc::c_char) -> *mut libc::c_char;
    fn gk_strtprune(_: *mut libc::c_char, _: *mut libc::c_char) -> *mut libc::c_char;
    fn gk_malloc(nbytes: size_t, msg: *mut libc::c_char) -> *mut libc::c_void;
    fn gk_getfilestats(
        fname: *mut libc::c_char,
        r_nlines: *mut size_t,
        r_ntokens: *mut size_t,
        r_max_nlntokens: *mut size_t,
        r_nbytes: *mut size_t,
    );
    fn gk_i32malloc(n: size_t, msg: *mut libc::c_char) -> *mut int32_t;
    fn gk_i64malloc(n: size_t, msg: *mut libc::c_char) -> *mut int64_t;
    fn gk_errexit(signum: libc::c_int, _: *mut libc::c_char, _: ...);
    fn gk_getfsize(_: *mut libc::c_char) -> intmax_t;
    fn gk_fmalloc(n: size_t, msg: *mut libc::c_char) -> *mut libc::c_float;
    fn gk_dmalloc(n: size_t, msg: *mut libc::c_char) -> *mut libc::c_double;
}
pub type size_t = u64;
pub type __int32_t = libc::c_int;
pub type __int64_t = i64;
pub type __intmax_t = i64;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __ssize_t = i64;
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
pub type ssize_t = __ssize_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type intmax_t = __intmax_t;
pub type gk_idx_t = ssize_t;
#[inline]
unsafe extern "C" fn getline(
    mut __lineptr: *mut *mut libc::c_char,
    mut __n: *mut size_t,
    mut __stream: *mut FILE,
) -> __ssize_t {
    return __getdelim(__lineptr, __n, '\n' as i32, __stream);
}
#[no_mangle]
pub unsafe extern "C" fn gk_fopen(
    mut fname: *mut libc::c_char,
    mut mode: *mut libc::c_char,
    mut msg: *const libc::c_char,
) -> *mut FILE {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut errmsg: [libc::c_char; 8192] = [0; 8192];
    fp = fopen(fname, mode);
    if !fp.is_null() {
        return fp;
    }
    sprintf(
        errmsg.as_mut_ptr(),
        b"file: %s, mode: %s, [%s]\0" as *const u8 as *const libc::c_char,
        fname,
        mode,
        msg,
    );
    perror(errmsg.as_mut_ptr());
    errexit(b"Failed on gk_fopen()\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    return 0 as *mut FILE;
}
#[no_mangle]
pub unsafe extern "C" fn gk_fclose(mut fp: *mut FILE) {
    fclose(fp);
}
#[no_mangle]
pub unsafe extern "C" fn gk_getline(
    mut lineptr: *mut *mut libc::c_char,
    mut n: *mut size_t,
    mut stream: *mut FILE,
) -> gk_idx_t {
    return getline(lineptr, n, stream);
}
#[no_mangle]
pub unsafe extern "C" fn gk_readfile(
    mut fname: *mut libc::c_char,
    mut r_nlines: *mut gk_idx_t,
) -> *mut *mut libc::c_char {
    let mut lnlen: size_t = 0;
    let mut nlines: size_t = 0;
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lines: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut fpin: *mut FILE = 0 as *mut FILE;
    gk_getfilestats(
        fname,
        &mut nlines,
        0 as *mut size_t,
        0 as *mut size_t,
        0 as *mut size_t,
    );
    if nlines > 0 as libc::c_int as u64 {
        lines = gk_malloc(
            nlines.wrapping_mul(::core::mem::size_of::<*mut libc::c_char>() as u64),
            b"gk_readfile: lines\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) as *mut *mut libc::c_char;
        fpin = gk_fopen(
            fname,
            b"r\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"gk_readfile\0" as *const u8 as *const libc::c_char,
        );
        nlines = 0 as libc::c_int as size_t;
        while gk_getline(&mut line, &mut lnlen, fpin) != -(1 as libc::c_int) as i64 {
            gk_strtprune(
                line,
                b"\n\r\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            let fresh0 = nlines;
            nlines = nlines.wrapping_add(1);
            let ref mut fresh1 = *lines.offset(fresh0 as isize);
            *fresh1 = gk_strdup(line);
        }
        gk_fclose(fpin);
    }
    gk_free(
        &mut line as *mut *mut libc::c_char as *mut *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
    if !r_nlines.is_null() {
        *r_nlines = nlines as gk_idx_t;
    }
    return lines;
}
#[no_mangle]
pub unsafe extern "C" fn gk_i32readfile(
    mut fname: *mut libc::c_char,
    mut r_nlines: *mut gk_idx_t,
) -> *mut int32_t {
    let mut lnlen: size_t = 0;
    let mut nlines: size_t = 0;
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut array: *mut int32_t = 0 as *mut int32_t;
    let mut fpin: *mut FILE = 0 as *mut FILE;
    gk_getfilestats(
        fname,
        &mut nlines,
        0 as *mut size_t,
        0 as *mut size_t,
        0 as *mut size_t,
    );
    if nlines > 0 as libc::c_int as u64 {
        array = gk_i32malloc(
            nlines,
            b"gk_i32readfile: array\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        fpin = gk_fopen(
            fname,
            b"r\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"gk_readfile\0" as *const u8 as *const libc::c_char,
        );
        nlines = 0 as libc::c_int as size_t;
        while gk_getline(&mut line, &mut lnlen, fpin) != -(1 as libc::c_int) as i64 {
            let fresh2 = nlines;
            nlines = nlines.wrapping_add(1);
            sscanf(
                line,
                b"%d\0" as *const u8 as *const libc::c_char,
                &mut *array.offset(fresh2 as isize) as *mut int32_t,
            );
        }
        gk_fclose(fpin);
    }
    gk_free(
        &mut line as *mut *mut libc::c_char as *mut *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
    if !r_nlines.is_null() {
        *r_nlines = nlines as gk_idx_t;
    }
    return array;
}
#[no_mangle]
pub unsafe extern "C" fn gk_i64readfile(
    mut fname: *mut libc::c_char,
    mut r_nlines: *mut gk_idx_t,
) -> *mut int64_t {
    let mut lnlen: size_t = 0;
    let mut nlines: size_t = 0;
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut array: *mut int64_t = 0 as *mut int64_t;
    let mut fpin: *mut FILE = 0 as *mut FILE;
    gk_getfilestats(
        fname,
        &mut nlines,
        0 as *mut size_t,
        0 as *mut size_t,
        0 as *mut size_t,
    );
    if nlines > 0 as libc::c_int as u64 {
        array = gk_i64malloc(
            nlines,
            b"gk_i64readfile: array\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        fpin = gk_fopen(
            fname,
            b"r\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"gk_readfile\0" as *const u8 as *const libc::c_char,
        );
        nlines = 0 as libc::c_int as size_t;
        while gk_getline(&mut line, &mut lnlen, fpin) != -(1 as libc::c_int) as i64 {
            let fresh3 = nlines;
            nlines = nlines.wrapping_add(1);
            sscanf(
                line,
                b"%ld\0" as *const u8 as *const libc::c_char,
                &mut *array.offset(fresh3 as isize) as *mut int64_t,
            );
        }
        gk_fclose(fpin);
    }
    gk_free(
        &mut line as *mut *mut libc::c_char as *mut *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
    if !r_nlines.is_null() {
        *r_nlines = nlines as gk_idx_t;
    }
    return array;
}
#[no_mangle]
pub unsafe extern "C" fn gk_i32readfilebin(
    mut fname: *mut libc::c_char,
    mut r_nelmnts: *mut ssize_t,
) -> *mut int32_t {
    let mut fsize: ssize_t = 0;
    let mut nelmnts: ssize_t = 0;
    let mut array: *mut int32_t = 0 as *mut int32_t;
    let mut fpin: *mut FILE = 0 as *mut FILE;
    *r_nelmnts = -(1 as libc::c_int) as ssize_t;
    fsize = gk_getfsize(fname);
    if (fsize as u64).wrapping_rem(::core::mem::size_of::<int32_t>() as u64)
        != 0 as libc::c_int as u64
    {
        gk_errexit(
            15 as libc::c_int,
            b"The size of the file is not in multiples of sizeof(int32_t).\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
        return 0 as *mut int32_t;
    }
    nelmnts = (fsize as u64).wrapping_div(::core::mem::size_of::<int32_t>() as u64) as ssize_t;
    array = gk_i32malloc(
        nelmnts as size_t,
        b"gk_i32readfilebin: array\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    fpin = gk_fopen(
        fname,
        b"rb\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"gk_i32readfilebin\0" as *const u8 as *const libc::c_char,
    );
    if fread(
        array as *mut libc::c_void,
        ::core::mem::size_of::<int32_t>() as u64,
        nelmnts as u64,
        fpin,
    ) != nelmnts as u64
    {
        gk_errexit(
            15 as libc::c_int,
            b"Failed to read the number of words requested. %zd\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            nelmnts,
        );
        gk_free(
            &mut array as *mut *mut int32_t as *mut *mut libc::c_void,
            0 as *mut *mut libc::c_void,
        );
        return 0 as *mut int32_t;
    }
    gk_fclose(fpin);
    *r_nelmnts = nelmnts;
    return array;
}
#[no_mangle]
pub unsafe extern "C" fn gk_i64readfilebin(
    mut fname: *mut libc::c_char,
    mut r_nelmnts: *mut ssize_t,
) -> *mut int64_t {
    let mut fsize: ssize_t = 0;
    let mut nelmnts: ssize_t = 0;
    let mut array: *mut int64_t = 0 as *mut int64_t;
    let mut fpin: *mut FILE = 0 as *mut FILE;
    *r_nelmnts = -(1 as libc::c_int) as ssize_t;
    fsize = gk_getfsize(fname);
    if (fsize as u64).wrapping_rem(::core::mem::size_of::<int64_t>() as u64)
        != 0 as libc::c_int as u64
    {
        gk_errexit(
            15 as libc::c_int,
            b"The size of the file is not in multiples of sizeof(int64_t).\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
        return 0 as *mut int64_t;
    }
    nelmnts = (fsize as u64).wrapping_div(::core::mem::size_of::<int64_t>() as u64) as ssize_t;
    array = gk_i64malloc(
        nelmnts as size_t,
        b"gk_i64readfilebin: array\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    fpin = gk_fopen(
        fname,
        b"rb\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"gk_i64readfilebin\0" as *const u8 as *const libc::c_char,
    );
    if fread(
        array as *mut libc::c_void,
        ::core::mem::size_of::<int64_t>() as u64,
        nelmnts as u64,
        fpin,
    ) != nelmnts as u64
    {
        gk_errexit(
            15 as libc::c_int,
            b"Failed to read the number of words requested. %zd\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            nelmnts,
        );
        gk_free(
            &mut array as *mut *mut int64_t as *mut *mut libc::c_void,
            0 as *mut *mut libc::c_void,
        );
        return 0 as *mut int64_t;
    }
    gk_fclose(fpin);
    *r_nelmnts = nelmnts;
    return array;
}
#[no_mangle]
pub unsafe extern "C" fn gk_freadfilebin(
    mut fname: *mut libc::c_char,
    mut r_nelmnts: *mut ssize_t,
) -> *mut libc::c_float {
    let mut fsize: ssize_t = 0;
    let mut nelmnts: ssize_t = 0;
    let mut array: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut fpin: *mut FILE = 0 as *mut FILE;
    *r_nelmnts = -(1 as libc::c_int) as ssize_t;
    fsize = gk_getfsize(fname);
    if (fsize as u64).wrapping_rem(::core::mem::size_of::<libc::c_float>() as u64)
        != 0 as libc::c_int as u64
    {
        gk_errexit(
            15 as libc::c_int,
            b"The size of the file is not in multiples of sizeof(float).\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
        return 0 as *mut libc::c_float;
    }
    nelmnts =
        (fsize as u64).wrapping_div(::core::mem::size_of::<libc::c_float>() as u64) as ssize_t;
    array = gk_fmalloc(
        nelmnts as size_t,
        b"gk_freadfilebin: array\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    fpin = gk_fopen(
        fname,
        b"rb\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"gk_freadfilebin\0" as *const u8 as *const libc::c_char,
    );
    if fread(
        array as *mut libc::c_void,
        ::core::mem::size_of::<libc::c_float>() as u64,
        nelmnts as u64,
        fpin,
    ) != nelmnts as u64
    {
        gk_errexit(
            15 as libc::c_int,
            b"Failed to read the number of words requested. %zd\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            nelmnts,
        );
        gk_free(
            &mut array as *mut *mut libc::c_float as *mut *mut libc::c_void,
            0 as *mut *mut libc::c_void,
        );
        return 0 as *mut libc::c_float;
    }
    gk_fclose(fpin);
    *r_nelmnts = nelmnts;
    return array;
}
#[no_mangle]
pub unsafe extern "C" fn gk_fwritefilebin(
    mut fname: *mut libc::c_char,
    mut n: size_t,
    mut a: *mut libc::c_float,
) -> size_t {
    let mut fsize: size_t = 0;
    let mut fp: *mut FILE = 0 as *mut FILE;
    fp = gk_fopen(
        fname,
        b"wb\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"gk_fwritefilebin\0" as *const u8 as *const libc::c_char,
    );
    fsize = fwrite(
        a as *const libc::c_void,
        ::core::mem::size_of::<libc::c_float>() as u64,
        n,
        fp,
    );
    gk_fclose(fp);
    return fsize;
}
#[no_mangle]
pub unsafe extern "C" fn gk_dreadfilebin(
    mut fname: *mut libc::c_char,
    mut r_nelmnts: *mut ssize_t,
) -> *mut libc::c_double {
    let mut fsize: ssize_t = 0;
    let mut nelmnts: ssize_t = 0;
    let mut array: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut fpin: *mut FILE = 0 as *mut FILE;
    *r_nelmnts = -(1 as libc::c_int) as ssize_t;
    fsize = gk_getfsize(fname);
    if (fsize as u64).wrapping_rem(::core::mem::size_of::<libc::c_double>() as u64)
        != 0 as libc::c_int as u64
    {
        gk_errexit(
            15 as libc::c_int,
            b"The size of the file is not in multiples of sizeof(double).\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
        return 0 as *mut libc::c_double;
    }
    nelmnts =
        (fsize as u64).wrapping_div(::core::mem::size_of::<libc::c_double>() as u64) as ssize_t;
    array = gk_dmalloc(
        nelmnts as size_t,
        b"gk_dreadfilebin: array\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    fpin = gk_fopen(
        fname,
        b"rb\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"gk_dreadfilebin\0" as *const u8 as *const libc::c_char,
    );
    if fread(
        array as *mut libc::c_void,
        ::core::mem::size_of::<libc::c_double>() as u64,
        nelmnts as u64,
        fpin,
    ) != nelmnts as u64
    {
        gk_errexit(
            15 as libc::c_int,
            b"Failed to read the number of words requested. %zd\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            nelmnts,
        );
        gk_free(
            &mut array as *mut *mut libc::c_double as *mut *mut libc::c_void,
            0 as *mut *mut libc::c_void,
        );
        return 0 as *mut libc::c_double;
    }
    gk_fclose(fpin);
    *r_nelmnts = nelmnts;
    return array;
}
