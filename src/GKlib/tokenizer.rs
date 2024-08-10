use ::libc;
extern "C" {
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn gk_malloc(nbytes: size_t, msg: *mut libc::c_char) -> *mut libc::c_void;
    fn gk_free(ptr1: *mut *mut libc::c_void, _: ...);
    fn gk_strdup(orgstr: *mut libc::c_char) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gk_Tokens_t {
    pub ntoks: libc::c_int,
    pub strbuf: *mut libc::c_char,
    pub list: *mut *mut libc::c_char,
}
#[no_mangle]
pub unsafe extern "C" fn gk_strtokenize(
    mut str: *mut libc::c_char,
    mut delim: *mut libc::c_char,
    mut tokens: *mut gk_Tokens_t,
) {
    let mut i: libc::c_int = 0;
    let mut ntoks: libc::c_int = 0;
    let mut slen: libc::c_int = 0;
    (*tokens).strbuf = gk_strdup(str);
    slen = strlen(str) as libc::c_int;
    str = (*tokens).strbuf;
    ntoks = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < slen {
        while i < slen
            && !(strchr(delim, *str.offset(i as isize) as libc::c_int)).is_null()
        {
            i += 1;
            i;
        }
        if i == slen {
            break;
        }
        ntoks += 1;
        ntoks;
        while i < slen
            && (strchr(delim, *str.offset(i as isize) as libc::c_int)).is_null()
        {
            i += 1;
            i;
        }
    }
    (*tokens).ntoks = ntoks;
    (*tokens)
        .list = gk_malloc(
        (ntoks as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
        b"strtokenize: tokens->list\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    ) as *mut *mut libc::c_char;
    ntoks = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < slen {
        while i < slen
            && !(strchr(delim, *str.offset(i as isize) as libc::c_int)).is_null()
        {
            let fresh0 = i;
            i = i + 1;
            *str.offset(fresh0 as isize) = '\0' as i32 as libc::c_char;
        }
        if i == slen {
            break;
        }
        let fresh1 = ntoks;
        ntoks = ntoks + 1;
        let ref mut fresh2 = *((*tokens).list).offset(fresh1 as isize);
        *fresh2 = str.offset(i as isize);
        while i < slen
            && (strchr(delim, *str.offset(i as isize) as libc::c_int)).is_null()
        {
            i += 1;
            i;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn gk_freetokenslist(mut tokens: *mut gk_Tokens_t) {
    gk_free(
        &mut (*tokens).list as *mut *mut *mut libc::c_char as *mut libc::c_void
            as *mut *mut libc::c_void,
        &mut (*tokens).strbuf as *mut *mut libc::c_char,
        0 as *mut *mut libc::c_void,
    );
}
