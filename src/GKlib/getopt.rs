use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: u64,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> u64;
}
pub type __off_t = i64;
pub type __off64_t = i64;
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
pub struct gk_option {
    pub name: *mut libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub const REQUIRE_ORDER: C2RustUnnamed = 0;
pub type C2RustUnnamed = libc::c_uint;
pub const RETURN_IN_ORDER: C2RustUnnamed = 2;
pub const PERMUTE: C2RustUnnamed = 1;
#[no_mangle]
pub static mut gk_optarg: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut gk_optind: libc::c_int = 1;
#[no_mangle]
pub static mut gk_opterr: libc::c_int = 1;
#[no_mangle]
pub static mut gk_optopt: libc::c_int = '?' as i32;
#[no_mangle]
pub static mut gk_getopt_initialized: libc::c_int = 0;
static mut nextchar: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut posixly_correct: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut ordering: C2RustUnnamed = REQUIRE_ORDER;
static mut first_nonopt: libc::c_int = 0;
static mut last_nonopt: libc::c_int = 0;
unsafe extern "C" fn exchange(mut argv: *mut *mut libc::c_char) {
    let mut bottom: libc::c_int = first_nonopt;
    let mut middle: libc::c_int = last_nonopt;
    let mut top: libc::c_int = gk_optind;
    let mut tem: *mut libc::c_char = 0 as *mut libc::c_char;
    while top > middle && middle > bottom {
        if top - middle > middle - bottom {
            let mut len: libc::c_int = middle - bottom;
            let mut i: libc::c_int = 0;
            i = 0 as libc::c_int;
            while i < len {
                tem = *argv.offset((bottom + i) as isize);
                let ref mut fresh0 = *argv.offset((bottom + i) as isize);
                *fresh0 = *argv.offset((top - (middle - bottom) + i) as isize);
                let ref mut fresh1 = *argv
                    .offset((top - (middle - bottom) + i) as isize);
                *fresh1 = tem;
                i += 1;
                i;
            }
            top -= len;
        } else {
            let mut len_0: libc::c_int = top - middle;
            let mut i_0: libc::c_int = 0;
            i_0 = 0 as libc::c_int;
            while i_0 < len_0 {
                tem = *argv.offset((bottom + i_0) as isize);
                let ref mut fresh2 = *argv.offset((bottom + i_0) as isize);
                *fresh2 = *argv.offset((middle + i_0) as isize);
                let ref mut fresh3 = *argv.offset((middle + i_0) as isize);
                *fresh3 = tem;
                i_0 += 1;
                i_0;
            }
            bottom += len_0;
        }
    }
    first_nonopt += gk_optind - last_nonopt;
    last_nonopt = gk_optind;
}
unsafe extern "C" fn gk_getopt_initialize(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut optstring: *mut libc::c_char,
) -> *mut libc::c_char {
    last_nonopt = gk_optind;
    first_nonopt = last_nonopt;
    nextchar = 0 as *mut libc::c_char;
    posixly_correct = getenv(b"POSIXLY_CORRECT\0" as *const u8 as *const libc::c_char);
    if *optstring.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32 {
        ordering = RETURN_IN_ORDER;
        optstring = optstring.offset(1);
        optstring;
    } else if *optstring.offset(0 as libc::c_int as isize) as libc::c_int == '+' as i32 {
        ordering = REQUIRE_ORDER;
        optstring = optstring.offset(1);
        optstring;
    } else if !posixly_correct.is_null() {
        ordering = REQUIRE_ORDER;
    } else {
        ordering = PERMUTE;
    }
    return optstring;
}
unsafe extern "C" fn gk_getopt_internal(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut optstring: *mut libc::c_char,
    mut longopts: *mut gk_option,
    mut longind: *mut libc::c_int,
    mut long_only: libc::c_int,
) -> libc::c_int {
    let mut print_errors: libc::c_int = gk_opterr;
    if *optstring.offset(0 as libc::c_int as isize) as libc::c_int == ':' as i32 {
        print_errors = 0 as libc::c_int;
    }
    if argc < 1 {
        return -(1);
    }
    gk_optarg = 0 as *mut libc::c_char;
    if gk_optind == 0 as libc::c_int || gk_getopt_initialized == 0 {
        if gk_optind == 0 as libc::c_int {
            gk_optind = 1;
        }
        optstring = gk_getopt_initialize(argc, argv, optstring);
        gk_getopt_initialized = 1;
    }
    if nextchar.is_null() || *nextchar as libc::c_int == '\0' as i32 {
        if last_nonopt > gk_optind {
            last_nonopt = gk_optind;
        }
        if first_nonopt > gk_optind {
            first_nonopt = gk_optind;
        }
        if ordering as libc::c_uint == PERMUTE as libc::c_int as libc::c_uint {
            if first_nonopt != last_nonopt && last_nonopt != gk_optind {
                exchange(argv);
            } else if last_nonopt != gk_optind {
                first_nonopt = gk_optind;
            }
            while gk_optind < argc
                && (*(*argv.offset(gk_optind as isize)).offset(0 as libc::c_int as isize)
                    as libc::c_int != '-' as i32
                    || *(*argv.offset(gk_optind as isize))
                        .offset(1 as isize) as libc::c_int == '\0' as i32)
            {
                gk_optind += 1;
                gk_optind;
            }
            last_nonopt = gk_optind;
        }
        if gk_optind != argc
            && strcmp(
                *argv.offset(gk_optind as isize),
                b"--\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            gk_optind += 1;
            gk_optind;
            if first_nonopt != last_nonopt && last_nonopt != gk_optind {
                exchange(argv);
            } else if first_nonopt == last_nonopt {
                first_nonopt = gk_optind;
            }
            last_nonopt = argc;
            gk_optind = argc;
        }
        if gk_optind == argc {
            if first_nonopt != last_nonopt {
                gk_optind = first_nonopt;
            }
            return -(1);
        }
        if *(*argv.offset(gk_optind as isize)).offset(0 as libc::c_int as isize)
            as libc::c_int != '-' as i32
            || *(*argv.offset(gk_optind as isize)).offset(1 as isize)
                as libc::c_int == '\0' as i32
        {
            if ordering as libc::c_uint == REQUIRE_ORDER as libc::c_int as libc::c_uint {
                return -(1);
            }
            let fresh4 = gk_optind;
            gk_optind = gk_optind + 1;
            gk_optarg = *argv.offset(fresh4 as isize);
            return 1;
        }
        nextchar = (*argv.offset(gk_optind as isize))
            .offset(1 as isize)
            .offset(
                (!longopts.is_null()
                    && *(*argv.offset(gk_optind as isize))
                        .offset(1 as isize) as libc::c_int == '-' as i32)
                    as libc::c_int as isize,
            );
    }
    if !longopts.is_null()
        && (*(*argv.offset(gk_optind as isize)).offset(1 as isize)
            as libc::c_int == '-' as i32
            || long_only != 0
                && (*(*argv.offset(gk_optind as isize)).offset(2 as libc::c_int as isize)
                    as libc::c_int != 0
                    || (strchr(
                        optstring,
                        *(*argv.offset(gk_optind as isize))
                            .offset(1 as isize) as libc::c_int,
                    ))
                        .is_null()))
    {
        let mut nameend: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut p: *mut gk_option = 0 as *mut gk_option;
        let mut pfound: *mut gk_option = 0 as *mut gk_option;
        let mut exact: libc::c_int = 0 as libc::c_int;
        let mut ambig: libc::c_int = 0 as libc::c_int;
        let mut indfound: libc::c_int = -(1);
        let mut option_index: libc::c_int = 0;
        nameend = nextchar;
        while *nameend as libc::c_int != 0 && *nameend as libc::c_int != '=' as i32 {
            nameend = nameend.offset(1);
            nameend;
        }
        p = longopts;
        option_index = 0 as libc::c_int;
        while !((*p).name).is_null() {
            if strncmp(
                (*p).name,
                nextchar,
                nameend.offset_from(nextchar) as i64 as u64,
            ) == 0
            {
                if nameend.offset_from(nextchar) as i64 as libc::c_uint
                    == strlen((*p).name) as libc::c_uint
                {
                    pfound = p;
                    indfound = option_index;
                    exact = 1;
                    break;
                } else if pfound.is_null() {
                    pfound = p;
                    indfound = option_index;
                } else if long_only != 0 || (*pfound).has_arg != (*p).has_arg
                    || (*pfound).flag != (*p).flag || (*pfound).val != (*p).val
                {
                    ambig = 1;
                }
            }
            p = p.offset(1);
            p;
            option_index += 1;
            option_index;
        }
        if ambig != 0 && exact == 0 {
            if print_errors != 0 {
                fprintf(
                    stderr,
                    b"%s: option `%s' is ambiguous\n\0" as *const u8
                        as *const libc::c_char,
                    *argv.offset(0 as libc::c_int as isize),
                    *argv.offset(gk_optind as isize),
                );
            }
            nextchar = nextchar.offset(strlen(nextchar) as isize);
            gk_optind += 1;
            gk_optind;
            gk_optopt = 0 as libc::c_int;
            return '?' as i32;
        }
        if !pfound.is_null() {
            option_index = indfound;
            gk_optind += 1;
            gk_optind;
            if *nameend != 0 {
                if (*pfound).has_arg != 0 {
                    gk_optarg = nameend.offset(1 as isize);
                } else {
                    if print_errors != 0 {
                        if *(*argv.offset((gk_optind - 1) as isize))
                            .offset(1 as isize) as libc::c_int
                            == '-' as i32
                        {
                            fprintf(
                                stderr,
                                b"%s: option `--%s' doesn't allow an argument\n\0"
                                    as *const u8 as *const libc::c_char,
                                *argv.offset(0 as libc::c_int as isize),
                                (*pfound).name,
                            );
                        } else {
                            fprintf(
                                stderr,
                                b"%s: option `%c%s' doesn't allow an argument\n\0"
                                    as *const u8 as *const libc::c_char,
                                *argv.offset(0 as libc::c_int as isize),
                                *(*argv.offset((gk_optind - 1) as isize))
                                    .offset(0 as libc::c_int as isize) as libc::c_int,
                                (*pfound).name,
                            );
                        }
                    }
                    nextchar = nextchar.offset(strlen(nextchar) as isize);
                    gk_optopt = (*pfound).val;
                    return '?' as i32;
                }
            } else if (*pfound).has_arg == 1 {
                if gk_optind < argc {
                    let fresh5 = gk_optind;
                    gk_optind = gk_optind + 1;
                    gk_optarg = *argv.offset(fresh5 as isize);
                } else {
                    if print_errors != 0 {
                        fprintf(
                            stderr,
                            b"%s: option `%s' requires an argument\n\0" as *const u8
                                as *const libc::c_char,
                            *argv.offset(0 as libc::c_int as isize),
                            *argv.offset((gk_optind - 1) as isize),
                        );
                    }
                    nextchar = nextchar.offset(strlen(nextchar) as isize);
                    gk_optopt = (*pfound).val;
                    return if *optstring.offset(0 as libc::c_int as isize) as libc::c_int
                        == ':' as i32
                    {
                        ':' as i32
                    } else {
                        '?' as i32
                    };
                }
            }
            nextchar = nextchar.offset(strlen(nextchar) as isize);
            if !longind.is_null() {
                *longind = option_index;
            }
            if !((*pfound).flag).is_null() {
                *(*pfound).flag = (*pfound).val;
                return 0 as libc::c_int;
            }
            return (*pfound).val;
        }
        if long_only == 0
            || *(*argv.offset(gk_optind as isize)).offset(1 as isize)
                as libc::c_int == '-' as i32
            || (strchr(optstring, *nextchar as libc::c_int)).is_null()
        {
            if print_errors != 0 {
                if *(*argv.offset(gk_optind as isize)).offset(1 as isize)
                    as libc::c_int == '-' as i32
                {
                    fprintf(
                        stderr,
                        b"%s: unrecognized option `--%s'\n\0" as *const u8
                            as *const libc::c_char,
                        *argv.offset(0 as libc::c_int as isize),
                        nextchar,
                    );
                } else {
                    fprintf(
                        stderr,
                        b"%s: unrecognized option `%c%s'\n\0" as *const u8
                            as *const libc::c_char,
                        *argv.offset(0 as libc::c_int as isize),
                        *(*argv.offset(gk_optind as isize))
                            .offset(0 as libc::c_int as isize) as libc::c_int,
                        nextchar,
                    );
                }
            }
            nextchar = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            gk_optind += 1;
            gk_optind;
            gk_optopt = 0 as libc::c_int;
            return '?' as i32;
        }
    }
    let fresh6 = nextchar;
    nextchar = nextchar.offset(1);
    let mut c: libc::c_char = *fresh6;
    let mut temp: *mut libc::c_char = strchr(optstring, c as libc::c_int);
    if *nextchar as libc::c_int == '\0' as i32 {
        gk_optind += 1;
        gk_optind;
    }
    if temp.is_null() || c as libc::c_int == ':' as i32 {
        if print_errors != 0 {
            if !posixly_correct.is_null() {
                fprintf(
                    stderr,
                    b"%s: illegal option -- %c\n\0" as *const u8 as *const libc::c_char,
                    *argv.offset(0 as libc::c_int as isize),
                    c as libc::c_int,
                );
            } else {
                fprintf(
                    stderr,
                    b"%s: invalid option -- %c\n\0" as *const u8 as *const libc::c_char,
                    *argv.offset(0 as libc::c_int as isize),
                    c as libc::c_int,
                );
            }
        }
        gk_optopt = c as libc::c_int;
        return '?' as i32;
    }
    if *temp.offset(0 as libc::c_int as isize) as libc::c_int == 'W' as i32
        && *temp.offset(1 as isize) as libc::c_int == ';' as i32
    {
        let mut nameend_0: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut p_0: *mut gk_option = 0 as *mut gk_option;
        let mut pfound_0: *mut gk_option = 0 as *mut gk_option;
        let mut exact_0: libc::c_int = 0 as libc::c_int;
        let mut ambig_0: libc::c_int = 0 as libc::c_int;
        let mut indfound_0: libc::c_int = 0 as libc::c_int;
        let mut option_index_0: libc::c_int = 0;
        if *nextchar as libc::c_int != '\0' as i32 {
            gk_optarg = nextchar;
            gk_optind += 1;
            gk_optind;
        } else if gk_optind == argc {
            if print_errors != 0 {
                fprintf(
                    stderr,
                    b"%s: option requires an argument -- %c\n\0" as *const u8
                        as *const libc::c_char,
                    *argv.offset(0 as libc::c_int as isize),
                    c as libc::c_int,
                );
            }
            gk_optopt = c as libc::c_int;
            if *optstring.offset(0 as libc::c_int as isize) as libc::c_int == ':' as i32
            {
                c = ':' as i32 as libc::c_char;
            } else {
                c = '?' as i32 as libc::c_char;
            }
            return c as libc::c_int;
        } else {
            let fresh7 = gk_optind;
            gk_optind = gk_optind + 1;
            gk_optarg = *argv.offset(fresh7 as isize);
        }
        nameend_0 = gk_optarg;
        nextchar = nameend_0;
        while *nameend_0 as libc::c_int != 0 && *nameend_0 as libc::c_int != '=' as i32 {
            nameend_0 = nameend_0.offset(1);
            nameend_0;
        }
        p_0 = longopts;
        option_index_0 = 0 as libc::c_int;
        while !((*p_0).name).is_null() {
            if strncmp(
                (*p_0).name,
                nextchar,
                nameend_0.offset_from(nextchar) as i64 as u64,
            ) == 0
            {
                if nameend_0.offset_from(nextchar) as i64 as libc::c_uint
                    as u64 == strlen((*p_0).name)
                {
                    pfound_0 = p_0;
                    indfound_0 = option_index_0;
                    exact_0 = 1;
                    break;
                } else if pfound_0.is_null() {
                    pfound_0 = p_0;
                    indfound_0 = option_index_0;
                } else {
                    ambig_0 = 1;
                }
            }
            p_0 = p_0.offset(1);
            p_0;
            option_index_0 += 1;
            option_index_0;
        }
        if ambig_0 != 0 && exact_0 == 0 {
            if print_errors != 0 {
                fprintf(
                    stderr,
                    b"%s: option `-W %s' is ambiguous\n\0" as *const u8
                        as *const libc::c_char,
                    *argv.offset(0 as libc::c_int as isize),
                    *argv.offset(gk_optind as isize),
                );
            }
            nextchar = nextchar.offset(strlen(nextchar) as isize);
            gk_optind += 1;
            gk_optind;
            return '?' as i32;
        }
        if !pfound_0.is_null() {
            option_index_0 = indfound_0;
            if *nameend_0 != 0 {
                if (*pfound_0).has_arg != 0 {
                    gk_optarg = nameend_0.offset(1 as isize);
                } else {
                    if print_errors != 0 {
                        fprintf(
                            stderr,
                            b"%s: option `-W %s' doesn't allow an argument\n\0"
                                as *const u8 as *const libc::c_char,
                            *argv.offset(0 as libc::c_int as isize),
                            (*pfound_0).name,
                        );
                    }
                    nextchar = nextchar.offset(strlen(nextchar) as isize);
                    return '?' as i32;
                }
            } else if (*pfound_0).has_arg == 1 {
                if gk_optind < argc {
                    let fresh8 = gk_optind;
                    gk_optind = gk_optind + 1;
                    gk_optarg = *argv.offset(fresh8 as isize);
                } else {
                    if print_errors != 0 {
                        fprintf(
                            stderr,
                            b"%s: option `%s' requires an argument\n\0" as *const u8
                                as *const libc::c_char,
                            *argv.offset(0 as libc::c_int as isize),
                            *argv.offset((gk_optind - 1) as isize),
                        );
                    }
                    nextchar = nextchar.offset(strlen(nextchar) as isize);
                    return if *optstring.offset(0 as libc::c_int as isize) as libc::c_int
                        == ':' as i32
                    {
                        ':' as i32
                    } else {
                        '?' as i32
                    };
                }
            }
            nextchar = nextchar.offset(strlen(nextchar) as isize);
            if !longind.is_null() {
                *longind = option_index_0;
            }
            if !((*pfound_0).flag).is_null() {
                *(*pfound_0).flag = (*pfound_0).val;
                return 0 as libc::c_int;
            }
            return (*pfound_0).val;
        }
        nextchar = 0 as *mut libc::c_char;
        return 'W' as i32;
    }
    if *temp.offset(1 as isize) as libc::c_int == ':' as i32 {
        if *temp.offset(2 as libc::c_int as isize) as libc::c_int == ':' as i32 {
            if *nextchar as libc::c_int != '\0' as i32 {
                gk_optarg = nextchar;
                gk_optind += 1;
                gk_optind;
            } else {
                gk_optarg = 0 as *mut libc::c_char;
            }
            nextchar = 0 as *mut libc::c_char;
        } else {
            if *nextchar as libc::c_int != '\0' as i32 {
                gk_optarg = nextchar;
                gk_optind += 1;
                gk_optind;
            } else if gk_optind == argc {
                if print_errors != 0 {
                    fprintf(
                        stderr,
                        b"%s: option requires an argument -- %c\n\0" as *const u8
                            as *const libc::c_char,
                        *argv.offset(0 as libc::c_int as isize),
                        c as libc::c_int,
                    );
                }
                gk_optopt = c as libc::c_int;
                if *optstring.offset(0 as libc::c_int as isize) as libc::c_int
                    == ':' as i32
                {
                    c = ':' as i32 as libc::c_char;
                } else {
                    c = '?' as i32 as libc::c_char;
                }
            } else {
                let fresh9 = gk_optind;
                gk_optind = gk_optind + 1;
                gk_optarg = *argv.offset(fresh9 as isize);
            }
            nextchar = 0 as *mut libc::c_char;
        }
    }
    return c as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gk_getopt(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut options: *mut libc::c_char,
) -> libc::c_int {
    return gk_getopt_internal(
        argc,
        argv,
        options,
        0 as *mut gk_option,
        0 as *mut libc::c_int,
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gk_getopt_long(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut options: *mut libc::c_char,
    mut long_options: *mut gk_option,
    mut opt_index: *mut libc::c_int,
) -> libc::c_int {
    return gk_getopt_internal(
        argc,
        argv,
        options,
        long_options,
        opt_index,
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gk_getopt_long_only(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut options: *mut libc::c_char,
    mut long_options: *mut gk_option,
    mut opt_index: *mut libc::c_int,
) -> libc::c_int {
    return gk_getopt_internal(
        argc,
        argv,
        options,
        long_options,
        opt_index,
        1,
    );
}
