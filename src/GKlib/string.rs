use ::c2rust_bitfields;
use ::libc;
extern "C" {
    pub type re_dfa_t;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn mktime(__tp: *mut tm) -> time_t;
    fn strftime(
        __s: *mut libc::c_char,
        __maxsize: size_t,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> size_t;
    fn strptime(
        __s: *const libc::c_char,
        __fmt: *const libc::c_char,
        __tp: *mut tm,
    ) -> *mut libc::c_char;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: u64) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: u64) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> u64;
    fn regcomp(
        __preg: *mut regex_t,
        __pattern: *const libc::c_char,
        __cflags: libc::c_int,
    ) -> libc::c_int;
    fn regexec(
        __preg: *const regex_t,
        __String: *const libc::c_char,
        __nmatch: size_t,
        __pmatch: *mut regmatch_t,
        __eflags: libc::c_int,
    ) -> libc::c_int;
    fn regerror(
        __errcode: libc::c_int,
        __preg: *const regex_t,
        __errbuf: *mut libc::c_char,
        __errbuf_size: size_t,
    ) -> size_t;
    fn regfree(__preg: *mut regex_t);
    fn gk_cmalloc(n: size_t, msg: *mut libc::c_char) -> *mut libc::c_char;
    fn gk_malloc(nbytes: size_t, msg: *mut libc::c_char) -> *mut libc::c_void;
    fn gk_realloc(
        oldptr: *mut libc::c_void,
        nbytes: size_t,
        msg: *mut libc::c_char,
    ) -> *mut libc::c_void;
    fn gk_free(ptr1: *mut *mut libc::c_void, _: ...);
}
pub type __int32_t = libc::c_int;
pub type __time_t = i64;
pub type __ssize_t = i64;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub __tm_gmtoff: i64,
    pub __tm_zone: *const libc::c_char,
}
pub type __re_long_size_t = u64;
pub type reg_syntax_t = u64;
pub type C2RustUnnamed = libc::c_int;
pub const _REG_ERPAREN: C2RustUnnamed = 16;
pub const _REG_ESIZE: C2RustUnnamed = 15;
pub const _REG_EEND: C2RustUnnamed = 14;
pub const _REG_BADRPT: C2RustUnnamed = 13;
pub const _REG_ESPACE: C2RustUnnamed = 12;
pub const _REG_ERANGE: C2RustUnnamed = 11;
pub const _REG_BADBR: C2RustUnnamed = 10;
pub const _REG_EBRACE: C2RustUnnamed = 9;
pub const _REG_EPAREN: C2RustUnnamed = 8;
pub const _REG_EBRACK: C2RustUnnamed = 7;
pub const _REG_ESUBREG: C2RustUnnamed = 6;
pub const _REG_EESCAPE: C2RustUnnamed = 5;
pub const _REG_ECTYPE: C2RustUnnamed = 4;
pub const _REG_ECOLLATE: C2RustUnnamed = 3;
pub const _REG_BADPAT: C2RustUnnamed = 2;
pub const _REG_NOMATCH: C2RustUnnamed = 1;
pub const _REG_NOERROR: C2RustUnnamed = 0;
pub const _REG_ENOSYS: C2RustUnnamed = -1;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct re_pattern_buffer {
    pub __buffer: *mut re_dfa_t,
    pub __allocated: __re_long_size_t,
    pub __used: __re_long_size_t,
    pub __syntax: reg_syntax_t,
    pub __fastmap: *mut libc::c_char,
    pub __translate: *mut libc::c_uchar,
    pub re_nsub: size_t,
    #[bitfield(name = "__can_be_null", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "__regs_allocated", ty = "libc::c_uint", bits = "1..=2")]
    #[bitfield(name = "__fastmap_accurate", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "__no_sub", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "__not_bol", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "__not_eol", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "__newline_anchor", ty = "libc::c_uint", bits = "7..=7")]
    pub __can_be_null___regs_allocated___fastmap_accurate___no_sub___not_bol___not_eol___newline_anchor:
        [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type regex_t = re_pattern_buffer;
pub type regoff_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regmatch_t {
    pub rm_so: regoff_t,
    pub rm_eo: regoff_t,
}
pub type gk_idx_t = ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gk_StringMap_t {
    pub name: *mut libc::c_char,
    pub id: libc::c_int,
}
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_strchr_replace(
    mut str: *mut libc::c_char,
    mut fromlist: *mut libc::c_char,
    mut tolist: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut k: gk_idx_t = 0;
    let mut len: size_t = 0;
    let mut fromlen: size_t = 0;
    let mut tolen: size_t = 0;
    len = strlen(str);
    fromlen = strlen(fromlist);
    tolen = strlen(tolist);
    j = 0 as libc::c_int as gk_idx_t;
    i = j;
    while (i as u64) < len {
        k = 0 as libc::c_int as gk_idx_t;
        while (k as u64) < fromlen {
            if *str.offset(i as isize) as libc::c_int == *fromlist.offset(k as isize) as libc::c_int
            {
                if (k as u64) < tolen {
                    let fresh0 = j;
                    j = j + 1;
                    *str.offset(fresh0 as isize) = *tolist.offset(k as isize);
                }
                break;
            } else {
                k += 1;
                k;
            }
        }
        if k as u64 == fromlen {
            let fresh1 = j;
            j = j + 1;
            *str.offset(fresh1 as isize) = *str.offset(i as isize);
        }
        i += 1;
        i;
    }
    *str.offset(j as isize) = '\0' as i32 as libc::c_char;
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn gk_strstr_replace(
    mut str: *mut libc::c_char,
    mut pattern: *mut libc::c_char,
    mut replacement: *mut libc::c_char,
    mut options: *mut libc::c_char,
    mut new_str: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut i: gk_idx_t = 0;
    let mut j: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    let mut flags: libc::c_int = 0;
    let mut global: libc::c_int = 0;
    let mut nmatches: libc::c_int = 0;
    let mut len: size_t = 0;
    let mut rlen: size_t = 0;
    let mut nlen: size_t = 0;
    let mut offset: size_t = 0;
    let mut noffset: size_t = 0;
    let mut re: regex_t = regex_t {
        __buffer: 0 as *mut re_dfa_t,
        __allocated: 0,
        __used: 0,
        __syntax: 0,
        __fastmap: 0 as *mut libc::c_char,
        __translate: 0 as *mut libc::c_uchar,
        re_nsub: 0,
        __can_be_null___regs_allocated___fastmap_accurate___no_sub___not_bol___not_eol___newline_anchor: [0; 1],
        c2rust_padding: [0; 7],
    };
    let mut matches: [regmatch_t; 10] = [regmatch_t { rm_so: 0, rm_eo: 0 }; 10];
    flags = 1;
    if !(strchr(options, 'i' as i32)).is_null() {
        flags = flags | (1) << 1;
    }
    global = if !(strchr(options, 'g' as i32)).is_null() {
        1
    } else {
        0 as libc::c_int
    };
    rc = regcomp(&mut re, pattern, flags);
    if rc != 0 as libc::c_int {
        len = regerror(
            rc,
            &mut re,
            0 as *mut libc::c_char,
            0 as libc::c_int as size_t,
        );
        *new_str = gk_cmalloc(
            len,
            b"gk_strstr_replace: new_str\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        regerror(rc, &mut re, *new_str, len);
        return 0 as libc::c_int;
    }
    len = strlen(str);
    nlen = (2 as libc::c_int as u64).wrapping_mul(len);
    noffset = 0 as libc::c_int as size_t;
    *new_str = gk_cmalloc(
        nlen.wrapping_add(1 as u64),
        b"gk_strstr_replace: new_str\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    rlen = strlen(replacement);
    offset = 0 as libc::c_int as size_t;
    nmatches = 0 as libc::c_int;
    loop {
        rc = regexec(
            &mut re,
            str.offset(offset as isize),
            10 as libc::c_int as size_t,
            matches.as_mut_ptr(),
            0 as libc::c_int,
        );
        if rc == _REG_ESPACE as libc::c_int {
            gk_free(
                new_str as *mut *mut libc::c_void,
                0 as *mut *mut libc::c_void,
            );
            *new_str = gk_strdup(
                b"regexec ran out of memory.\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            regfree(&mut re);
            return 0 as libc::c_int;
        } else if rc == _REG_NOMATCH as libc::c_int {
            if nlen.wrapping_sub(noffset) < len.wrapping_sub(offset) {
                nlen = (nlen as u64).wrapping_add(
                    len.wrapping_sub(offset)
                        .wrapping_sub(nlen.wrapping_sub(noffset)),
                ) as size_t as size_t;
                *new_str = gk_realloc(
                    *new_str as *mut libc::c_void,
                    nlen.wrapping_add(1 as u64)
                        .wrapping_mul(::core::mem::size_of::<libc::c_char>() as u64),
                    b"gk_strstr_replace: new_str\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ) as *mut libc::c_char;
            }
            strcpy(
                (*new_str).offset(noffset as isize),
                str.offset(offset as isize),
            );
            noffset = (noffset as u64).wrapping_add(len.wrapping_sub(offset)) as size_t as size_t;
            break;
        } else {
            nmatches += 1;
            nmatches;
            if matches[0].rm_so > 0 as libc::c_int {
                if nlen.wrapping_sub(noffset) < matches[0].rm_so as u64 {
                    nlen = (nlen as u64).wrapping_add(
                        (matches[0].rm_so as u64).wrapping_sub(nlen.wrapping_sub(noffset)),
                    ) as size_t as size_t;
                    *new_str = gk_realloc(
                        *new_str as *mut libc::c_void,
                        nlen.wrapping_add(1 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_char>() as u64),
                        b"gk_strstr_replace: new_str\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    ) as *mut libc::c_char;
                }
                strncpy(
                    (*new_str).offset(noffset as isize),
                    str.offset(offset as isize),
                    matches[0].rm_so as u64,
                );
                noffset =
                    (noffset as u64).wrapping_add(matches[0].rm_so as u64) as size_t as size_t;
            }
            i = 0 as libc::c_int as gk_idx_t;
            while (i as u64) < rlen {
                match *replacement.offset(i as isize) as libc::c_int {
                    92 => {
                        if ((i + 1 as i64) as u64) < rlen {
                            if nlen.wrapping_sub(noffset) < 1 as u64 {
                                nlen = (nlen as u64).wrapping_add(nlen.wrapping_add(1 as u64))
                                    as size_t as size_t;
                                *new_str =
                                    gk_realloc(
                                        *new_str as *mut libc::c_void,
                                        nlen.wrapping_add(1 as u64).wrapping_mul(
                                            ::core::mem::size_of::<libc::c_char>() as u64,
                                        ),
                                        b"gk_strstr_replace: new_str\0" as *const u8
                                            as *const libc::c_char
                                            as *mut libc::c_char,
                                    ) as *mut libc::c_char;
                            }
                            i += 1;
                            let fresh2 = noffset;
                            noffset = noffset.wrapping_add(1);
                            **new_str.offset(fresh2 as isize) = *replacement.offset(i as isize);
                        } else {
                            gk_free(
                                new_str as *mut *mut libc::c_void,
                                0 as *mut *mut libc::c_void,
                            );
                            *new_str = gk_strdup(
                                b"Error in replacement string. Missing character following ''.\0"
                                    as *const u8
                                    as *const libc::c_char
                                    as *mut libc::c_char,
                            );
                            regfree(&mut re);
                            return 0 as libc::c_int;
                        }
                    }
                    36 => {
                        if ((i + 1 as i64) as u64) < rlen {
                            i += 1;
                            j = *replacement.offset(i as isize) as libc::c_int - '0' as i32;
                            if j < 0 as libc::c_int || j > 9 as libc::c_int {
                                gk_free(
                                    new_str as *mut *mut libc::c_void,
                                    0 as *mut *mut libc::c_void,
                                );
                                *new_str = gk_strdup(
                                    b"Error in captured subexpression specification.\0" as *const u8
                                        as *const libc::c_char
                                        as *mut libc::c_char,
                                );
                                regfree(&mut re);
                                return 0 as libc::c_int;
                            }
                            if nlen.wrapping_sub(noffset)
                                < (matches[j as usize].rm_eo - matches[j as usize].rm_so) as u64
                            {
                                nlen = (nlen as u64).wrapping_add(nlen.wrapping_add(
                                    (matches[j as usize].rm_eo - matches[j as usize].rm_so) as u64,
                                )) as size_t as size_t;
                                *new_str =
                                    gk_realloc(
                                        *new_str as *mut libc::c_void,
                                        nlen.wrapping_add(1 as u64).wrapping_mul(
                                            ::core::mem::size_of::<libc::c_char>() as u64,
                                        ),
                                        b"gk_strstr_replace: new_str\0" as *const u8
                                            as *const libc::c_char
                                            as *mut libc::c_char,
                                    ) as *mut libc::c_char;
                            }
                            strncpy(
                                (*new_str).offset(noffset as isize),
                                str.offset(offset as isize)
                                    .offset(matches[j as usize].rm_so as isize),
                                matches[j as usize].rm_eo as u64,
                            );
                            noffset = (noffset as u64).wrapping_add(
                                (matches[j as usize].rm_eo - matches[j as usize].rm_so) as u64,
                            ) as size_t as size_t;
                        } else {
                            gk_free(
                                new_str as *mut *mut libc::c_void,
                                0 as *mut *mut libc::c_void,
                            );
                            *new_str = gk_strdup(
                                b"Error in replacement string. Missing subexpression number folloing '$'.\0"
                                    as *const u8 as *const libc::c_char as *mut libc::c_char,
                            );
                            regfree(&mut re);
                            return 0 as libc::c_int;
                        }
                    }
                    _ => {
                        if nlen.wrapping_sub(noffset) < 1 as u64 {
                            nlen = (nlen as u64).wrapping_add(nlen.wrapping_add(1 as u64)) as size_t
                                as size_t;
                            *new_str = gk_realloc(
                                *new_str as *mut libc::c_void,
                                nlen.wrapping_add(1 as u64)
                                    .wrapping_mul(::core::mem::size_of::<libc::c_char>() as u64),
                                b"gk_strstr_replace: new_str\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            ) as *mut libc::c_char;
                        }
                        let fresh3 = noffset;
                        noffset = noffset.wrapping_add(1);
                        *(*new_str).offset(fresh3 as isize) = *replacement.offset(i as isize);
                    }
                }
                i += 1;
                i;
            }
            offset = (offset as u64).wrapping_add(matches[0].rm_eo as u64) as size_t as size_t;
            if global == 0 {
                if nlen.wrapping_sub(noffset) < len.wrapping_sub(offset) {
                    nlen = (nlen as u64).wrapping_add(
                        len.wrapping_sub(offset)
                            .wrapping_sub(nlen.wrapping_sub(noffset)),
                    ) as size_t as size_t;
                    *new_str = gk_realloc(
                        *new_str as *mut libc::c_void,
                        nlen.wrapping_add(1 as u64)
                            .wrapping_mul(::core::mem::size_of::<libc::c_char>() as u64),
                        b"gk_strstr_replace: new_str\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    ) as *mut libc::c_char;
                }
                strcpy(
                    (*new_str).offset(noffset as isize),
                    str.offset(offset as isize),
                );
                noffset =
                    (noffset as u64).wrapping_add(len.wrapping_sub(offset)) as size_t as size_t;
            }
            if !(global != 0) {
                break;
            }
        }
    }
    *(*new_str).offset(noffset as isize) = '\0' as i32 as libc::c_char;
    regfree(&mut re);
    return nmatches + 1;
}
#[no_mangle]
pub unsafe extern "C" fn gk_strtprune(
    mut str: *mut libc::c_char,
    mut rmlist: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut len: size_t = 0;
    len = strlen(rmlist);
    i = (strlen(str)).wrapping_sub(1 as u64) as gk_idx_t;
    while i >= 0 as libc::c_int as i64 {
        j = 0 as libc::c_int as gk_idx_t;
        while (j as u64) < len {
            if *str.offset(i as isize) as libc::c_int == *rmlist.offset(j as isize) as libc::c_int {
                break;
            }
            j += 1;
            j;
        }
        if j as u64 == len {
            break;
        }
        i -= 1;
        i;
    }
    *str.offset((i + 1 as i64) as isize) = '\0' as i32 as libc::c_char;
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn gk_strhprune(
    mut str: *mut libc::c_char,
    mut rmlist: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut len: size_t = 0;
    len = strlen(rmlist);
    i = 0 as libc::c_int as gk_idx_t;
    while *str.offset(i as isize) != 0 {
        j = 0 as libc::c_int as gk_idx_t;
        while (j as u64) < len {
            if *str.offset(i as isize) as libc::c_int == *rmlist.offset(j as isize) as libc::c_int {
                break;
            }
            j += 1;
            j;
        }
        if j as u64 == len {
            break;
        }
        i += 1;
        i;
    }
    if i > 0 as libc::c_int as i64 {
        j = 0 as libc::c_int as gk_idx_t;
        while *str.offset(i as isize) != 0 {
            *str.offset(j as isize) = *str.offset(i as isize);
            i += 1;
            i;
            j += 1;
            j;
        }
        *str.offset(j as isize) = '\0' as i32 as libc::c_char;
    }
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn gk_strtoupper(mut str: *mut libc::c_char) -> *mut libc::c_char {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while *str.offset(i as isize) as libc::c_int != '\0' as i32 {
        *str.offset(i as isize) = ({
            let mut __res: libc::c_int = 0;
            if ::core::mem::size_of::<libc::c_char>() as u64 > 1 as u64 {
                if 0 != 0 {
                    let mut __c: libc::c_int = *str.offset(i as isize) as libc::c_int;
                    __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*__ctype_toupper_loc()).offset(__c as isize)
                    };
                } else {
                    __res = toupper(*str.offset(i as isize) as libc::c_int);
                }
            } else {
                __res = *(*__ctype_toupper_loc())
                    .offset(*str.offset(i as isize) as libc::c_int as isize);
            }
            __res
        }) as libc::c_char;
        i += 1;
        i;
    }
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn gk_strtolower(mut str: *mut libc::c_char) -> *mut libc::c_char {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while *str.offset(i as isize) as libc::c_int != '\0' as i32 {
        *str.offset(i as isize) = ({
            let mut __res: libc::c_int = 0;
            if ::core::mem::size_of::<libc::c_char>() as u64 > 1 as u64 {
                if 0 != 0 {
                    let mut __c: libc::c_int = *str.offset(i as isize) as libc::c_int;
                    __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*__ctype_tolower_loc()).offset(__c as isize)
                    };
                } else {
                    __res = tolower(*str.offset(i as isize) as libc::c_int);
                }
            } else {
                __res = *(*__ctype_tolower_loc())
                    .offset(*str.offset(i as isize) as libc::c_int as isize);
            }
            __res
        }) as libc::c_char;
        i += 1;
        i;
    }
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn gk_strdup(mut orgstr: *mut libc::c_char) -> *mut libc::c_char {
    let mut len: libc::c_int = 0;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    if !orgstr.is_null() {
        len = (strlen(orgstr)).wrapping_add(1 as u64) as libc::c_int;
        str = gk_malloc(
            (len as u64).wrapping_mul(::core::mem::size_of::<libc::c_char>() as u64),
            b"gk_strdup: str\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) as *mut libc::c_char;
        strcpy(str, orgstr);
    }
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn gk_strcasecmp(
    mut s1: *mut libc::c_char,
    mut s2: *mut libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0 as libc::c_int;
    if strlen(s1) != strlen(s2) {
        return 0 as libc::c_int;
    }
    while *s1.offset(i as isize) as libc::c_int != '\0' as i32 {
        if ({
            let mut __res: libc::c_int = 0;
            if ::core::mem::size_of::<libc::c_char>() as u64 > 1 as u64 {
                if 0 != 0 {
                    let mut __c: libc::c_int = *s1.offset(i as isize) as libc::c_int;
                    __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*__ctype_tolower_loc()).offset(__c as isize)
                    };
                } else {
                    __res = tolower(*s1.offset(i as isize) as libc::c_int);
                }
            } else {
                __res = *(*__ctype_tolower_loc())
                    .offset(*s1.offset(i as isize) as libc::c_int as isize);
            }
            __res
        }) != ({
            let mut __res: libc::c_int = 0;
            if ::core::mem::size_of::<libc::c_char>() as u64 > 1 as u64 {
                if 0 != 0 {
                    let mut __c: libc::c_int = *s2.offset(i as isize) as libc::c_int;
                    __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*__ctype_tolower_loc()).offset(__c as isize)
                    };
                } else {
                    __res = tolower(*s2.offset(i as isize) as libc::c_int);
                }
            } else {
                __res = *(*__ctype_tolower_loc())
                    .offset(*s2.offset(i as isize) as libc::c_int as isize);
            }
            __res
        }) {
            return 0 as libc::c_int;
        }
        i += 1;
        i;
    }
    return 1;
}
#[no_mangle]
pub unsafe extern "C" fn gk_strrcmp(
    mut s1: *mut libc::c_char,
    mut s2: *mut libc::c_char,
) -> libc::c_int {
    let mut i1: libc::c_int = (strlen(s1)).wrapping_sub(1 as u64) as libc::c_int;
    let mut i2: libc::c_int = (strlen(s2)).wrapping_sub(1 as u64) as libc::c_int;
    while i1 >= 0 as libc::c_int && i2 >= 0 as libc::c_int {
        if *s1.offset(i1 as isize) as libc::c_int != *s2.offset(i2 as isize) as libc::c_int {
            return *s1.offset(i1 as isize) as libc::c_int - *s2.offset(i2 as isize) as libc::c_int;
        }
        i1 -= 1;
        i1;
        i2 -= 1;
        i2;
    }
    if i1 < i2 {
        return -(1);
    }
    if i1 > i2 {
        return 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gk_time2str(mut time: time_t) -> *mut libc::c_char {
    static mut datestr: [libc::c_char; 128] = [0; 128];
    let mut tm: *mut tm = 0 as *mut tm;
    tm = localtime(&mut time);
    if strftime(
        datestr.as_mut_ptr(),
        128 as libc::c_int as size_t,
        b"%m/%d/%Y %H:%M:%S\0" as *const u8 as *const libc::c_char,
        tm,
    ) == 0 as libc::c_int as u64
    {
        return 0 as *mut libc::c_char;
    } else {
        return datestr.as_mut_ptr();
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_str2time(mut str: *mut libc::c_char) -> time_t {
    let mut time: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        __tm_gmtoff: 0,
        __tm_zone: 0 as *const libc::c_char,
    };
    let mut rtime: time_t = 0;
    memset(
        &mut time as *mut tm as *mut libc::c_void,
        '\0' as i32,
        ::core::mem::size_of::<tm>() as u64,
    );
    if (strptime(
        str,
        b"%m/%d/%Y %H:%M:%S\0" as *const u8 as *const libc::c_char,
        &mut time,
    ))
    .is_null()
    {
        return -(1) as time_t;
    }
    rtime = mktime(&mut time);
    return if rtime < 0 as libc::c_int as i64 {
        0 as libc::c_int as i64
    } else {
        rtime
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_GetStringID(
    mut strmap: *mut gk_StringMap_t,
    mut key: *mut libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while !((*strmap.offset(i as isize)).name).is_null() {
        if gk_strcasecmp(key, (*strmap.offset(i as isize)).name) != 0 {
            return (*strmap.offset(i as isize)).id;
        }
        i += 1;
        i;
    }
    return -(1);
}
