use ::libc;
extern "C" {
    fn gk_errexit(signum: libc::c_int, _: *mut libc::c_char, _: ...);
}
#[no_mangle]
pub unsafe extern "C" fn encodeblock(mut in_0: *mut libc::c_uchar, mut out: *mut libc::c_uchar) {
    *out.offset(0 as libc::c_int as isize) = (*in_0.offset(0 as libc::c_int as isize)
        as libc::c_int
        >> 2 as libc::c_int) as libc::c_uchar;
    *out.offset(1 as isize) =
        ((*in_0.offset(0 as libc::c_int as isize) as libc::c_int & 0x3 as libc::c_int)
            << 4 as libc::c_int
            | *in_0.offset(1 as isize) as libc::c_int >> 4 as libc::c_int) as libc::c_uchar;
    *out.offset(2 as libc::c_int as isize) =
        ((*in_0.offset(1 as isize) as libc::c_int & 0xf as libc::c_int) << 2 as libc::c_int
            | *in_0.offset(2 as libc::c_int as isize) as libc::c_int >> 6 as libc::c_int)
            as libc::c_uchar;
    *out.offset(3 as libc::c_int as isize) = (*in_0.offset(2 as libc::c_int as isize)
        as libc::c_int
        & 0x3f as libc::c_int) as libc::c_uchar;
    let ref mut fresh0 = *out.offset(0 as libc::c_int as isize);
    *fresh0 = (*fresh0 as libc::c_int + 48 as libc::c_int) as libc::c_uchar;
    let ref mut fresh1 = *out.offset(1 as libc::c_int as isize);
    *fresh1 = (*fresh1 as libc::c_int + 48 as libc::c_int) as libc::c_uchar;
    let ref mut fresh2 = *out.offset(2 as libc::c_int as isize);
    *fresh2 = (*fresh2 as libc::c_int + 48 as libc::c_int) as libc::c_uchar;
    let ref mut fresh3 = *out.offset(3 as libc::c_int as isize);
    *fresh3 = (*fresh3 as libc::c_int + 48 as libc::c_int) as libc::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn decodeblock(mut in_0: *mut libc::c_uchar, mut out: *mut libc::c_uchar) {
    let ref mut fresh4 = *in_0.offset(0 as libc::c_int as isize);
    *fresh4 = (*fresh4 as libc::c_int - 48 as libc::c_int) as libc::c_uchar;
    let ref mut fresh5 = *in_0.offset(1 as isize);
    *fresh5 = (*fresh5 as libc::c_int - 48 as libc::c_int) as libc::c_uchar;
    let ref mut fresh6 = *in_0.offset(2 as libc::c_int as isize);
    *fresh6 = (*fresh6 as libc::c_int - 48 as libc::c_int) as libc::c_uchar;
    let ref mut fresh7 = *in_0.offset(3 as libc::c_int as isize);
    *fresh7 = (*fresh7 as libc::c_int - 48 as libc::c_int) as libc::c_uchar;
    *out.offset(0 as libc::c_int as isize) =
        ((*in_0.offset(0 as libc::c_int as isize) as libc::c_int) << 2 as libc::c_int
            | *in_0.offset(1 as isize) as libc::c_int >> 4 as libc::c_int) as libc::c_uchar;
    *out.offset(1 as isize) = ((*in_0.offset(1 as isize) as libc::c_int) << 4 as libc::c_int
        | *in_0.offset(2 as libc::c_int as isize) as libc::c_int >> 2 as libc::c_int)
        as libc::c_uchar;
    *out.offset(2 as libc::c_int as isize) =
        ((*in_0.offset(2 as libc::c_int as isize) as libc::c_int) << 6 as libc::c_int
            | *in_0.offset(3 as libc::c_int as isize) as libc::c_int) as libc::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn GKEncodeBase64(
    mut nbytes: libc::c_int,
    mut inbuffer: *mut libc::c_uchar,
    mut outbuffer: *mut libc::c_uchar,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if nbytes % 3 as libc::c_int != 0 as libc::c_int {
        gk_errexit(
            15 as libc::c_int,
            b"GKEncodeBase64: Input buffer size should be a multiple of 3! (%d)\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            nbytes,
        );
    }
    j = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < nbytes {
        encodeblock(inbuffer.offset(i as isize), outbuffer.offset(j as isize));
        i += 3 as libc::c_int;
        j += 4 as libc::c_int;
    }
    *outbuffer.offset(j as isize) = '\0' as i32 as libc::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn GKDecodeBase64(
    mut nbytes: libc::c_int,
    mut inbuffer: *mut libc::c_uchar,
    mut outbuffer: *mut libc::c_uchar,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if nbytes % 4 as libc::c_int != 0 as libc::c_int {
        gk_errexit(
            15 as libc::c_int,
            b"GKDecodeBase64: Input buffer size should be a multiple of 4! (%d)\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            nbytes,
        );
    }
    j = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < nbytes {
        decodeblock(inbuffer.offset(i as isize), outbuffer.offset(j as isize));
        i += 4 as libc::c_int;
        j += 3 as libc::c_int;
    }
}
