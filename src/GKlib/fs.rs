use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn system(__command: *const libc::c_char) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: u64,
        _: u64,
        _: *mut FILE,
    ) -> u64;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn gk_fopen(
        _: *mut libc::c_char,
        _: *mut libc::c_char,
        _: *const libc::c_char,
    ) -> *mut FILE;
    fn gk_fclose(_: *mut FILE);
    fn gk_strdup(orgstr: *mut libc::c_char) -> *mut libc::c_char;
}
pub type __intmax_t = i64;
pub type __dev_t = u64;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = u64;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __time_t = i64;
pub type __blksize_t = i64;
pub type __blkcnt_t = i64;
pub type __syscall_slong_t = i64;
pub type __syscall_ulong_t = u64;
pub type intmax_t = __intmax_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atime: __time_t,
    pub st_atimensec: __syscall_ulong_t,
    pub st_mtime: __time_t,
    pub st_mtimensec: __syscall_ulong_t,
    pub st_ctime: __time_t,
    pub st_ctimensec: __syscall_ulong_t,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
#[no_mangle]
pub unsafe extern "C" fn gk_fexists(mut fname: *mut libc::c_char) -> libc::c_int {
    let mut status: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atime: 0,
        st_atimensec: 0,
        st_mtime: 0,
        st_mtimensec: 0,
        st_ctime: 0,
        st_ctimensec: 0,
        __glibc_reserved: [0; 3],
    };
    if stat(fname, &mut status) == -(1 as libc::c_int) {
        return 0 as libc::c_int;
    }
    return (status.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o100000 as libc::c_int as libc::c_uint) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gk_dexists(mut dirname: *mut libc::c_char) -> libc::c_int {
    let mut status: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atime: 0,
        st_atimensec: 0,
        st_mtime: 0,
        st_mtimensec: 0,
        st_ctime: 0,
        st_ctimensec: 0,
        __glibc_reserved: [0; 3],
    };
    if stat(dirname, &mut status) == -(1 as libc::c_int) {
        return 0 as libc::c_int;
    }
    return (status.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o40000 as libc::c_int as libc::c_uint) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gk_getfsize(mut filename: *mut libc::c_char) -> intmax_t {
    let mut status: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atime: 0,
        st_atimensec: 0,
        st_mtime: 0,
        st_mtimensec: 0,
        st_ctime: 0,
        st_ctimensec: 0,
        __glibc_reserved: [0; 3],
    };
    if stat(filename, &mut status) == -(1 as libc::c_int) {
        return -(1 as libc::c_int) as intmax_t;
    }
    return status.st_size;
}
#[no_mangle]
pub unsafe extern "C" fn gk_getfilestats(
    mut fname: *mut libc::c_char,
    mut r_nlines: *mut size_t,
    mut r_ntokens: *mut size_t,
    mut r_max_nlntokens: *mut size_t,
    mut r_nbytes: *mut size_t,
) {
    let mut nlines: size_t = 0 as libc::c_int as size_t;
    let mut ntokens: size_t = 0 as libc::c_int as size_t;
    let mut max_nlntokens: size_t = 0 as libc::c_int as size_t;
    let mut nbytes: size_t = 0 as libc::c_int as size_t;
    let mut oldntokens: size_t = 0 as libc::c_int as size_t;
    let mut nread: size_t = 0;
    let mut intoken: libc::c_int = 0 as libc::c_int;
    let mut buffer: [libc::c_char; 2049] = [0; 2049];
    let mut cptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fpin: *mut FILE = 0 as *mut FILE;
    fpin = gk_fopen(
        fname,
        b"r\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"gk_GetFileStats\0" as *const u8 as *const libc::c_char,
    );
    while feof(fpin) == 0 {
        nread = fread(
            buffer.as_mut_ptr() as *mut libc::c_void,
            ::core::mem::size_of::<libc::c_char>() as u64,
            2048 as libc::c_int as u64,
            fpin,
        );
        nbytes = (nbytes as u64).wrapping_add(nread) as size_t as size_t;
        buffer[nread as usize] = '\0' as i32 as libc::c_char;
        cptr = buffer.as_mut_ptr();
        while *cptr as libc::c_int != '\0' as i32 {
            if *cptr as libc::c_int == '\n' as i32 {
                nlines = nlines.wrapping_add(1);
                nlines;
                ntokens = (ntokens as u64)
                    .wrapping_add(intoken as u64) as size_t as size_t;
                intoken = 0 as libc::c_int;
                if max_nlntokens < ntokens.wrapping_sub(oldntokens) {
                    max_nlntokens = ntokens.wrapping_sub(oldntokens);
                }
                oldntokens = ntokens;
            } else if *cptr as libc::c_int == ' ' as i32
                || *cptr as libc::c_int == '\t' as i32
            {
                ntokens = (ntokens as u64)
                    .wrapping_add(intoken as u64) as size_t as size_t;
                intoken = 0 as libc::c_int;
            } else {
                intoken = 1 as libc::c_int;
            }
            cptr = cptr.offset(1);
            cptr;
        }
    }
    ntokens = (ntokens as u64).wrapping_add(intoken as u64) as size_t
        as size_t;
    if max_nlntokens < ntokens.wrapping_sub(oldntokens) {
        max_nlntokens = ntokens.wrapping_sub(oldntokens);
    }
    gk_fclose(fpin);
    if !r_nlines.is_null() {
        *r_nlines = nlines;
    }
    if !r_ntokens.is_null() {
        *r_ntokens = ntokens;
    }
    if !r_max_nlntokens.is_null() {
        *r_max_nlntokens = max_nlntokens;
    }
    if !r_nbytes.is_null() {
        *r_nbytes = nbytes;
    }
}
#[no_mangle]
pub unsafe extern "C" fn gk_getbasename(
    mut path: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut startptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut endptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut basename: *mut libc::c_char = 0 as *mut libc::c_char;
    startptr = strrchr(path, '/' as i32);
    if startptr.is_null() {
        startptr = path;
    } else {
        startptr = startptr.offset(1 as libc::c_int as isize);
    }
    basename = gk_strdup(startptr);
    endptr = strrchr(basename, '.' as i32);
    if !endptr.is_null() {
        *endptr = '\0' as i32 as libc::c_char;
    }
    return basename;
}
#[no_mangle]
pub unsafe extern "C" fn gk_getextname(
    mut path: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut startptr: *mut libc::c_char = 0 as *mut libc::c_char;
    startptr = strrchr(path, '.' as i32);
    if startptr.is_null() {
        return gk_strdup(path)
    } else {
        return gk_strdup(startptr.offset(1 as libc::c_int as isize))
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_getfilename(
    mut path: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut startptr: *mut libc::c_char = 0 as *mut libc::c_char;
    startptr = strrchr(path, '/' as i32);
    if startptr.is_null() {
        return gk_strdup(path)
    } else {
        return gk_strdup(startptr.offset(1 as libc::c_int as isize))
    };
}
#[no_mangle]
pub unsafe extern "C" fn getpathname(mut path: *mut libc::c_char) -> *mut libc::c_char {
    let mut endptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    endptr = strrchr(path, '/' as i32);
    if endptr.is_null() {
        return gk_strdup(b".\0" as *const u8 as *const libc::c_char as *mut libc::c_char)
    } else {
        tmp = gk_strdup(path);
        *strrchr(tmp, '/' as i32) = '\0' as i32 as libc::c_char;
        return tmp;
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_mkpath(mut pathname: *mut libc::c_char) -> libc::c_int {
    let mut tmp: [libc::c_char; 2048] = [0; 2048];
    sprintf(
        tmp.as_mut_ptr(),
        b"mkdir -p %s\0" as *const u8 as *const libc::c_char,
        pathname,
    );
    return system(tmp.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn gk_rmpath(mut pathname: *mut libc::c_char) -> libc::c_int {
    let mut tmp: [libc::c_char; 2048] = [0; 2048];
    sprintf(
        tmp.as_mut_ptr(),
        b"rm -r %s\0" as *const u8 as *const libc::c_char,
        pathname,
    );
    return system(tmp.as_mut_ptr());
}
