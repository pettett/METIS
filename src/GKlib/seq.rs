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
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> u64;
    fn gk_fopen(
        _: *mut libc::c_char,
        _: *mut libc::c_char,
        _: *const libc::c_char,
    ) -> *mut FILE;
    fn gk_fclose(_: *mut FILE);
    fn gk_getfilestats(
        fname: *mut libc::c_char,
        r_nlines: *mut size_t,
        r_ntokens: *mut size_t,
        r_max_nlntokens: *mut size_t,
        r_nbytes: *mut size_t,
    );
    fn gk_getbasename(path: *mut libc::c_char) -> *mut libc::c_char;
    fn gk_cmalloc(n: size_t, msg: *mut libc::c_char) -> *mut libc::c_char;
    fn gk_cset(n: size_t, val: libc::c_char, x: *mut libc::c_char) -> *mut libc::c_char;
    fn gk_imalloc(n: size_t, msg: *mut libc::c_char) -> *mut libc::c_int;
    fn gk_iset(n: size_t, val: libc::c_int, x: *mut libc::c_int) -> *mut libc::c_int;
    fn gk_iAllocMatrix(
        ndim1: size_t,
        ndim2: size_t,
        value: libc::c_int,
        errmsg: *mut libc::c_char,
    ) -> *mut *mut libc::c_int;
    fn gk_iFreeMatrix(
        r_matrix: *mut *mut *mut libc::c_int,
        ndim1: size_t,
        ndim2: size_t,
    );
    fn gk_malloc(nbytes: size_t, msg: *mut libc::c_char) -> *mut libc::c_void;
    fn gk_free(ptr1: *mut *mut libc::c_void, _: ...);
    fn gk_strtoupper(_: *mut libc::c_char) -> *mut libc::c_char;
    fn errexit(_: *mut libc::c_char, _: ...);
    fn gk_freetokenslist(tokens: *mut gk_Tokens_t);
    fn gk_strtokenize(
        line: *mut libc::c_char,
        delim: *mut libc::c_char,
        tokens: *mut gk_Tokens_t,
    );
}
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __ssize_t = i64;
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
pub struct gk_Tokens_t {
    pub ntoks: libc::c_int,
    pub strbuf: *mut libc::c_char,
    pub list: *mut *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gk_i2cc2i_t {
    pub n: libc::c_int,
    pub i2c: *mut libc::c_char,
    pub c2i: *mut libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gk_seq_t {
    pub len: libc::c_int,
    pub sequence: *mut libc::c_int,
    pub pssm: *mut *mut libc::c_int,
    pub psfm: *mut *mut libc::c_int,
    pub name: *mut libc::c_char,
    pub nsymbols: libc::c_int,
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gk_seq_init(mut seq: *mut gk_seq_t) {
    (*seq).len = 0 as libc::c_int;
    (*seq).sequence = 0 as *mut libc::c_int;
    (*seq).pssm = 0 as *mut *mut libc::c_int;
    (*seq).psfm = 0 as *mut *mut libc::c_int;
    (*seq).name = 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn gk_i2cc2i_create_common(
    mut alphabet: *mut libc::c_char,
) -> *mut gk_i2cc2i_t {
    let mut nsymbols: libc::c_int = 0;
    let mut i: gk_idx_t = 0;
    let mut t: *mut gk_i2cc2i_t = 0 as *mut gk_i2cc2i_t;
    nsymbols = strlen(alphabet) as libc::c_int;
    t = gk_malloc(
        ::core::mem::size_of::<gk_i2cc2i_t>() as u64,
        b"gk_i2c_create_common\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    ) as *mut gk_i2cc2i_t;
    (*t).n = nsymbols;
    (*t)
        .i2c = gk_cmalloc(
        256 as libc::c_int as size_t,
        b"gk_i2c_create_common\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*t)
        .c2i = gk_imalloc(
        256 as libc::c_int as size_t,
        b"gk_i2c_create_common\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    gk_cset(256 as libc::c_int as size_t, -(1 as libc::c_int) as libc::c_char, (*t).i2c);
    gk_iset(256 as libc::c_int as size_t, -(1 as libc::c_int), (*t).c2i);
    i = 0 as libc::c_int as gk_idx_t;
    while i < nsymbols as i64 {
        *((*t).i2c).offset(i as isize) = *alphabet.offset(i as isize);
        *((*t).c2i)
            .offset(
                *alphabet.offset(i as isize) as libc::c_int as isize,
            ) = i as libc::c_int;
        i += 1;
        i;
    }
    return t;
}
#[no_mangle]
pub unsafe extern "C" fn gk_seq_ReadGKMODPSSM(
    mut filename: *mut libc::c_char,
) -> *mut gk_seq_t {
    let mut seq: *mut gk_seq_t = 0 as *mut gk_seq_t;
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut ii: gk_idx_t = 0;
    let mut ntokens: size_t = 0;
    let mut nbytes: size_t = 0;
    let mut len: size_t = 0;
    let mut fpin: *mut FILE = 0 as *mut FILE;
    let mut tokens: gk_Tokens_t = gk_Tokens_t {
        ntoks: 0,
        strbuf: 0 as *mut libc::c_char,
        list: 0 as *mut *mut libc::c_char,
    };
    static mut AAORDER: *mut libc::c_char = b"ARNDCQEGHILKMFPSTWYVBZX*\0" as *const u8
        as *const libc::c_char as *mut libc::c_char;
    static mut PSSMWIDTH: libc::c_int = 20 as libc::c_int;
    let mut header: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut line: [libc::c_char; 300000] = [0; 300000];
    let mut converter: *mut gk_i2cc2i_t = 0 as *mut gk_i2cc2i_t;
    header = gk_cmalloc(
        PSSMWIDTH as size_t,
        b"gk_seq_ReadGKMODPSSM: header\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    converter = gk_i2cc2i_create_common(AAORDER);
    gk_getfilestats(filename, &mut len, &mut ntokens, 0 as *mut size_t, &mut nbytes);
    len = len.wrapping_sub(1);
    len;
    seq = gk_malloc(
        ::core::mem::size_of::<gk_seq_t>() as u64,
        b"gk_seq_ReadGKMODPSSM\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    ) as *mut gk_seq_t;
    gk_seq_init(seq);
    (*seq).len = len as libc::c_int;
    (*seq)
        .sequence = gk_imalloc(
        len,
        b"gk_seq_ReadGKMODPSSM\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*seq)
        .pssm = gk_iAllocMatrix(
        len,
        PSSMWIDTH as size_t,
        0 as libc::c_int,
        b"gk_seq_ReadGKMODPSSM\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*seq)
        .psfm = gk_iAllocMatrix(
        len,
        PSSMWIDTH as size_t,
        0 as libc::c_int,
        b"gk_seq_ReadGKMODPSSM\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*seq).nsymbols = PSSMWIDTH;
    (*seq).name = gk_getbasename(filename);
    fpin = gk_fopen(
        filename,
        b"r\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"gk_seq_ReadGKMODPSSM\0" as *const u8 as *const libc::c_char,
    );
    if (fgets(line.as_mut_ptr(), 300000 as libc::c_int - 1 as libc::c_int, fpin))
        .is_null()
    {
        errexit(
            b"Unexpected end of file: %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            filename,
        );
    }
    gk_strtoupper(line.as_mut_ptr());
    gk_strtokenize(
        line.as_mut_ptr(),
        b" \t\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut tokens,
    );
    i = 0 as libc::c_int as gk_idx_t;
    while i < PSSMWIDTH as i64 {
        *header
            .offset(
                i as isize,
            ) = *(*(tokens.list).offset(i as isize)).offset(0 as libc::c_int as isize);
        i += 1;
        i;
    }
    gk_freetokenslist(&mut tokens);
    i = 0 as libc::c_int as gk_idx_t;
    ii = 0 as libc::c_int as gk_idx_t;
    while (ii as u64) < len {
        if (fgets(line.as_mut_ptr(), 300000 as libc::c_int - 1 as libc::c_int, fpin))
            .is_null()
        {
            errexit(
                b"Unexpected end of file: %s\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                filename,
            );
        }
        gk_strtoupper(line.as_mut_ptr());
        gk_strtokenize(
            line.as_mut_ptr(),
            b" \t\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            &mut tokens,
        );
        *((*seq).sequence)
            .offset(
                i as isize,
            ) = *((*converter).c2i)
            .offset(
                *(*(tokens.list).offset(1 as libc::c_int as isize))
                    .offset(0 as libc::c_int as isize) as libc::c_int as isize,
            );
        j = 0 as libc::c_int as gk_idx_t;
        while j < PSSMWIDTH as i64 {
            *(*((*seq).pssm).offset(i as isize))
                .offset(
                    *((*converter).c2i)
                        .offset(*header.offset(j as isize) as libc::c_int as isize)
                        as isize,
                ) = atoi(
                *(tokens.list).offset((2 as libc::c_int as i64 + j) as isize),
            );
            *(*((*seq).psfm).offset(i as isize))
                .offset(
                    *((*converter).c2i)
                        .offset(*header.offset(j as isize) as libc::c_int as isize)
                        as isize,
                ) = atoi(
                *(tokens.list)
                    .offset(
                        ((2 as libc::c_int + PSSMWIDTH) as i64 + j) as isize,
                    ),
            );
            j += 1;
            j;
        }
        gk_freetokenslist(&mut tokens);
        i += 1;
        i;
        ii += 1;
        ii;
    }
    (*seq).len = i as libc::c_int;
    gk_free(
        &mut header as *mut *mut libc::c_char as *mut *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
    gk_fclose(fpin);
    return seq;
}
#[no_mangle]
pub unsafe extern "C" fn gk_seq_free(mut seq: *mut gk_seq_t) {
    gk_iFreeMatrix(&mut (*seq).pssm, (*seq).len as size_t, (*seq).nsymbols as size_t);
    gk_iFreeMatrix(&mut (*seq).psfm, (*seq).len as size_t, (*seq).nsymbols as size_t);
    gk_free(
        &mut (*seq).name as *mut *mut libc::c_char as *mut *mut libc::c_void,
        &mut (*seq).sequence as *mut *mut libc::c_int,
        0 as *mut *mut libc::c_void,
    );
    gk_free(
        &mut seq as *mut *mut gk_seq_t as *mut *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
}
