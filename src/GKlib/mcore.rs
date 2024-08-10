use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn gk_malloc(nbytes: size_t, msg: *mut libc::c_char) -> *mut libc::c_void;
    fn gk_free(ptr1: *mut *mut libc::c_void, _: ...);
    fn errexit(_: *mut libc::c_char, _: ...);
    fn gk_errexit(signum: libc::c_int, _: *mut libc::c_char, _: ...);
}
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gk_mop_t {
    pub type_0: libc::c_int,
    pub nbytes: ssize_t,
    pub ptr: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gk_mcore_t {
    pub coresize: size_t,
    pub corecpos: size_t,
    pub core: *mut libc::c_void,
    pub nmops: size_t,
    pub cmop: size_t,
    pub mops: *mut gk_mop_t,
    pub num_callocs: size_t,
    pub num_hallocs: size_t,
    pub size_callocs: size_t,
    pub size_hallocs: size_t,
    pub cur_callocs: size_t,
    pub cur_hallocs: size_t,
    pub max_callocs: size_t,
    pub max_hallocs: size_t,
}
#[no_mangle]
pub unsafe extern "C" fn gk_mcoreCreate(mut coresize: size_t) -> *mut gk_mcore_t {
    let mut mcore: *mut gk_mcore_t = 0 as *mut gk_mcore_t;
    mcore = gk_malloc(
        ::core::mem::size_of::<gk_mcore_t>() as libc::c_ulong,
        b"gk_mcoreCreate: mcore\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    ) as *mut gk_mcore_t;
    memset(
        mcore as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<gk_mcore_t>() as libc::c_ulong,
    );
    (*mcore).coresize = coresize;
    (*mcore).corecpos = 0 as libc::c_int as size_t;
    (*mcore)
        .core = if coresize == 0 as libc::c_int as libc::c_ulong {
        0 as *mut libc::c_void
    } else {
        gk_malloc(
            (*mcore).coresize,
            b"gk_mcoreCreate: core\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        )
    };
    (*mcore).nmops = 2048 as libc::c_int as size_t;
    (*mcore).cmop = 0 as libc::c_int as size_t;
    (*mcore)
        .mops = gk_malloc(
        ((*mcore).nmops)
            .wrapping_mul(::core::mem::size_of::<gk_mop_t>() as libc::c_ulong),
        b"gk_mcoreCreate: mcore->mops\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    ) as *mut gk_mop_t;
    return mcore;
}
#[no_mangle]
pub unsafe extern "C" fn gk_gkmcoreCreate() -> *mut gk_mcore_t {
    let mut mcore: *mut gk_mcore_t = 0 as *mut gk_mcore_t;
    mcore = malloc(::core::mem::size_of::<gk_mcore_t>() as libc::c_ulong)
        as *mut gk_mcore_t;
    if mcore.is_null() {
        return 0 as *mut gk_mcore_t;
    }
    memset(
        mcore as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<gk_mcore_t>() as libc::c_ulong,
    );
    (*mcore).nmops = 2048 as libc::c_int as size_t;
    (*mcore).cmop = 0 as libc::c_int as size_t;
    (*mcore)
        .mops = malloc(
        ((*mcore).nmops)
            .wrapping_mul(::core::mem::size_of::<gk_mop_t>() as libc::c_ulong),
    ) as *mut gk_mop_t;
    if ((*mcore).mops).is_null() {
        free(mcore as *mut libc::c_void);
        return 0 as *mut gk_mcore_t;
    }
    return mcore;
}
#[no_mangle]
pub unsafe extern "C" fn gk_mcoreDestroy(
    mut r_mcore: *mut *mut gk_mcore_t,
    mut showstats: libc::c_int,
) {
    let mut mcore: *mut gk_mcore_t = *r_mcore;
    if mcore.is_null() {
        return;
    }
    if showstats != 0 {
        printf(
            b"\n gk_mcore statistics\n           coresize: %12zu         nmops: %12zu  cmop: %6zu\n        num_callocs: %12zu   num_hallocs: %12zu\n       size_callocs: %12zu  size_hallocs: %12zu\n        cur_callocs: %12zu   cur_hallocs: %12zu\n        max_callocs: %12zu   max_hallocs: %12zu\n\0"
                as *const u8 as *const libc::c_char,
            (*mcore).coresize,
            (*mcore).nmops,
            (*mcore).cmop,
            (*mcore).num_callocs,
            (*mcore).num_hallocs,
            (*mcore).size_callocs,
            (*mcore).size_hallocs,
            (*mcore).cur_callocs,
            (*mcore).cur_hallocs,
            (*mcore).max_callocs,
            (*mcore).max_hallocs,
        );
    }
    if (*mcore).cur_callocs != 0 as libc::c_int as libc::c_ulong
        || (*mcore).cur_hallocs != 0 as libc::c_int as libc::c_ulong
        || (*mcore).cmop != 0 as libc::c_int as libc::c_ulong
    {
        printf(
            b"***Warning: mcore memory was not fully freed when destroyed.\n cur_callocs: %6zu  cur_hallocs: %6zu cmop: %6zu\n\0"
                as *const u8 as *const libc::c_char,
            (*mcore).cur_callocs,
            (*mcore).cur_hallocs,
            (*mcore).cmop,
        );
    }
    gk_free(
        &mut (*mcore).core as *mut *mut libc::c_void,
        &mut (*mcore).mops as *mut *mut gk_mop_t,
        &mut mcore as *mut *mut gk_mcore_t,
        0 as *mut *mut libc::c_void,
    );
    *r_mcore = 0 as *mut gk_mcore_t;
}
#[no_mangle]
pub unsafe extern "C" fn gk_gkmcoreDestroy(
    mut r_mcore: *mut *mut gk_mcore_t,
    mut showstats: libc::c_int,
) {
    let mut mcore: *mut gk_mcore_t = *r_mcore;
    if mcore.is_null() {
        return;
    }
    if showstats != 0 {
        printf(
            b"\n gk_mcore statistics\n         nmops: %12zu  cmop: %6zu\n   num_hallocs: %12zu\n  size_hallocs: %12zu\n   cur_hallocs: %12zu\n   max_hallocs: %12zu\n\0"
                as *const u8 as *const libc::c_char,
            (*mcore).nmops,
            (*mcore).cmop,
            (*mcore).num_hallocs,
            (*mcore).size_hallocs,
            (*mcore).cur_hallocs,
            (*mcore).max_hallocs,
        );
    }
    if (*mcore).cur_hallocs != 0 as libc::c_int as libc::c_ulong
        || (*mcore).cmop != 0 as libc::c_int as libc::c_ulong
    {
        printf(
            b"***Warning: mcore memory was not fully freed when destroyed.\n cur_hallocs: %6zu cmop: %6zu\n\0"
                as *const u8 as *const libc::c_char,
            (*mcore).cur_hallocs,
            (*mcore).cmop,
        );
    }
    free((*mcore).mops as *mut libc::c_void);
    free(mcore as *mut libc::c_void);
    *r_mcore = 0 as *mut gk_mcore_t;
}
#[no_mangle]
pub unsafe extern "C" fn gk_mcoreMalloc(
    mut mcore: *mut gk_mcore_t,
    mut nbytes: size_t,
) -> *mut libc::c_void {
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    nbytes = (nbytes as libc::c_ulong)
        .wrapping_add(
            if nbytes.wrapping_rem(8 as libc::c_int as libc::c_ulong)
                == 0 as libc::c_int as libc::c_ulong
            {
                0 as libc::c_int as libc::c_ulong
            } else {
                (8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(nbytes.wrapping_rem(8 as libc::c_int as libc::c_ulong))
            },
        ) as size_t as size_t;
    if ((*mcore).corecpos).wrapping_add(nbytes) < (*mcore).coresize {
        ptr = ((*mcore).core as *mut libc::c_char).offset((*mcore).corecpos as isize)
            as *mut libc::c_void;
        (*mcore)
            .corecpos = ((*mcore).corecpos as libc::c_ulong).wrapping_add(nbytes)
            as size_t as size_t;
        gk_mcoreAdd(mcore, 2 as libc::c_int, nbytes, ptr);
    } else {
        ptr = gk_malloc(
            nbytes,
            b"gk_mcoremalloc: ptr\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        gk_mcoreAdd(mcore, 3 as libc::c_int, nbytes, ptr);
    }
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn gk_mcorePush(mut mcore: *mut gk_mcore_t) {
    gk_mcoreAdd(
        mcore,
        1 as libc::c_int,
        0 as libc::c_int as size_t,
        0 as *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gk_gkmcorePush(mut mcore: *mut gk_mcore_t) {
    gk_gkmcoreAdd(
        mcore,
        1 as libc::c_int,
        0 as libc::c_int as size_t,
        0 as *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gk_mcorePop(mut mcore: *mut gk_mcore_t) {
    while (*mcore).cmop > 0 as libc::c_int as libc::c_ulong {
        (*mcore).cmop = ((*mcore).cmop).wrapping_sub(1);
        (*mcore).cmop;
        match (*((*mcore).mops).offset((*mcore).cmop as isize)).type_0 {
            1 => {
                break;
            }
            2 => {
                if (*mcore).corecpos
                    < (*((*mcore).mops).offset((*mcore).cmop as isize)).nbytes
                        as libc::c_ulong
                {
                    errexit(
                        b"Internal Error: wspace's core is about to be over-freed [%zu, %zu, %zd]\n\0"
                            as *const u8 as *const libc::c_char as *mut libc::c_char,
                        (*mcore).coresize,
                        (*mcore).corecpos,
                        (*((*mcore).mops).offset((*mcore).cmop as isize)).nbytes,
                    );
                }
                (*mcore)
                    .corecpos = ((*mcore).corecpos as libc::c_ulong)
                    .wrapping_sub(
                        (*((*mcore).mops).offset((*mcore).cmop as isize)).nbytes
                            as libc::c_ulong,
                    ) as size_t as size_t;
                (*mcore)
                    .cur_callocs = ((*mcore).cur_callocs as libc::c_ulong)
                    .wrapping_sub(
                        (*((*mcore).mops).offset((*mcore).cmop as isize)).nbytes
                            as libc::c_ulong,
                    ) as size_t as size_t;
            }
            3 => {
                gk_free(
                    &mut (*((*mcore).mops).offset((*mcore).cmop as isize)).ptr
                        as *mut *mut libc::c_void,
                    0 as *mut *mut libc::c_void,
                );
                (*mcore)
                    .cur_hallocs = ((*mcore).cur_hallocs as libc::c_ulong)
                    .wrapping_sub(
                        (*((*mcore).mops).offset((*mcore).cmop as isize)).nbytes
                            as libc::c_ulong,
                    ) as size_t as size_t;
            }
            _ => {
                gk_errexit(
                    6 as libc::c_int,
                    b"Unknown mop type of %d\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    (*((*mcore).mops).offset((*mcore).cmop as isize)).type_0,
                );
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn gk_gkmcorePop(mut mcore: *mut gk_mcore_t) {
    while (*mcore).cmop > 0 as libc::c_int as libc::c_ulong {
        (*mcore).cmop = ((*mcore).cmop).wrapping_sub(1);
        (*mcore).cmop;
        match (*((*mcore).mops).offset((*mcore).cmop as isize)).type_0 {
            1 => {
                break;
            }
            3 => {
                free((*((*mcore).mops).offset((*mcore).cmop as isize)).ptr);
                (*mcore)
                    .cur_hallocs = ((*mcore).cur_hallocs as libc::c_ulong)
                    .wrapping_sub(
                        (*((*mcore).mops).offset((*mcore).cmop as isize)).nbytes
                            as libc::c_ulong,
                    ) as size_t as size_t;
            }
            _ => {
                gk_errexit(
                    6 as libc::c_int,
                    b"Unknown mop type of %d\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    (*((*mcore).mops).offset((*mcore).cmop as isize)).type_0,
                );
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn gk_mcoreAdd(
    mut mcore: *mut gk_mcore_t,
    mut type_0: libc::c_int,
    mut nbytes: size_t,
    mut ptr: *mut libc::c_void,
) {
    if (*mcore).cmop == (*mcore).nmops {
        (*mcore)
            .nmops = ((*mcore).nmops as libc::c_ulong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
        (*mcore)
            .mops = realloc(
            (*mcore).mops as *mut libc::c_void,
            ((*mcore).nmops)
                .wrapping_mul(::core::mem::size_of::<gk_mop_t>() as libc::c_ulong),
        ) as *mut gk_mop_t;
        if ((*mcore).mops).is_null() {
            gk_errexit(
                6 as libc::c_int,
                b"***Memory allocation for gkmcore failed.\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
    }
    (*((*mcore).mops).offset((*mcore).cmop as isize)).type_0 = type_0;
    (*((*mcore).mops).offset((*mcore).cmop as isize)).nbytes = nbytes as ssize_t;
    let ref mut fresh0 = (*((*mcore).mops).offset((*mcore).cmop as isize)).ptr;
    *fresh0 = ptr;
    (*mcore).cmop = ((*mcore).cmop).wrapping_add(1);
    (*mcore).cmop;
    match type_0 {
        1 => {}
        2 => {
            (*mcore).num_callocs = ((*mcore).num_callocs).wrapping_add(1);
            (*mcore).num_callocs;
            (*mcore)
                .size_callocs = ((*mcore).size_callocs as libc::c_ulong)
                .wrapping_add(nbytes) as size_t as size_t;
            (*mcore)
                .cur_callocs = ((*mcore).cur_callocs as libc::c_ulong)
                .wrapping_add(nbytes) as size_t as size_t;
            if (*mcore).max_callocs < (*mcore).cur_callocs {
                (*mcore).max_callocs = (*mcore).cur_callocs;
            }
        }
        3 => {
            (*mcore).num_hallocs = ((*mcore).num_hallocs).wrapping_add(1);
            (*mcore).num_hallocs;
            (*mcore)
                .size_hallocs = ((*mcore).size_hallocs as libc::c_ulong)
                .wrapping_add(nbytes) as size_t as size_t;
            (*mcore)
                .cur_hallocs = ((*mcore).cur_hallocs as libc::c_ulong)
                .wrapping_add(nbytes) as size_t as size_t;
            if (*mcore).max_hallocs < (*mcore).cur_hallocs {
                (*mcore).max_hallocs = (*mcore).cur_hallocs;
            }
        }
        _ => {
            gk_errexit(
                6 as libc::c_int,
                b"Incorrect mcore type operation.\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_gkmcoreAdd(
    mut mcore: *mut gk_mcore_t,
    mut type_0: libc::c_int,
    mut nbytes: size_t,
    mut ptr: *mut libc::c_void,
) {
    if (*mcore).cmop == (*mcore).nmops {
        (*mcore)
            .nmops = ((*mcore).nmops as libc::c_ulong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
        (*mcore)
            .mops = realloc(
            (*mcore).mops as *mut libc::c_void,
            ((*mcore).nmops)
                .wrapping_mul(::core::mem::size_of::<gk_mop_t>() as libc::c_ulong),
        ) as *mut gk_mop_t;
        if ((*mcore).mops).is_null() {
            gk_errexit(
                6 as libc::c_int,
                b"***Memory allocation for gkmcore failed.\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
    }
    (*((*mcore).mops).offset((*mcore).cmop as isize)).type_0 = type_0;
    (*((*mcore).mops).offset((*mcore).cmop as isize)).nbytes = nbytes as ssize_t;
    let ref mut fresh1 = (*((*mcore).mops).offset((*mcore).cmop as isize)).ptr;
    *fresh1 = ptr;
    (*mcore).cmop = ((*mcore).cmop).wrapping_add(1);
    (*mcore).cmop;
    match type_0 {
        1 => {}
        3 => {
            (*mcore).num_hallocs = ((*mcore).num_hallocs).wrapping_add(1);
            (*mcore).num_hallocs;
            (*mcore)
                .size_hallocs = ((*mcore).size_hallocs as libc::c_ulong)
                .wrapping_add(nbytes) as size_t as size_t;
            (*mcore)
                .cur_hallocs = ((*mcore).cur_hallocs as libc::c_ulong)
                .wrapping_add(nbytes) as size_t as size_t;
            if (*mcore).max_hallocs < (*mcore).cur_hallocs {
                (*mcore).max_hallocs = (*mcore).cur_hallocs;
            }
        }
        _ => {
            gk_errexit(
                6 as libc::c_int,
                b"Incorrect mcore type operation.\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_mcoreDel(
    mut mcore: *mut gk_mcore_t,
    mut ptr: *mut libc::c_void,
) {
    let mut i: libc::c_int = 0;
    i = ((*mcore).cmop).wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    while i >= 0 as libc::c_int {
        if (*((*mcore).mops).offset(i as isize)).type_0 == 1 as libc::c_int {
            gk_errexit(
                6 as libc::c_int,
                b"Could not find pointer %p in mcore\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ptr,
            );
        }
        if (*((*mcore).mops).offset(i as isize)).ptr == ptr {
            if (*((*mcore).mops).offset(i as isize)).type_0 != 3 as libc::c_int {
                gk_errexit(
                    6 as libc::c_int,
                    b"Trying to delete a non-HEAP mop.\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
            }
            (*mcore)
                .cur_hallocs = ((*mcore).cur_hallocs as libc::c_ulong)
                .wrapping_sub(
                    (*((*mcore).mops).offset(i as isize)).nbytes as libc::c_ulong,
                ) as size_t as size_t;
            (*mcore).cmop = ((*mcore).cmop).wrapping_sub(1);
            *((*mcore).mops)
                .offset(i as isize) = *((*mcore).mops).offset((*mcore).cmop as isize);
            return;
        }
        i -= 1;
        i;
    }
    gk_errexit(
        6 as libc::c_int,
        b"mcoreDel should never have been here!\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gk_gkmcoreDel(
    mut mcore: *mut gk_mcore_t,
    mut ptr: *mut libc::c_void,
) {
    let mut i: libc::c_int = 0;
    i = ((*mcore).cmop).wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    while i >= 0 as libc::c_int {
        if (*((*mcore).mops).offset(i as isize)).type_0 == 1 as libc::c_int {
            gk_errexit(
                6 as libc::c_int,
                b"Could not find pointer %p in mcore\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ptr,
            );
        }
        if (*((*mcore).mops).offset(i as isize)).ptr == ptr {
            if (*((*mcore).mops).offset(i as isize)).type_0 != 3 as libc::c_int {
                gk_errexit(
                    6 as libc::c_int,
                    b"Trying to delete a non-HEAP mop.\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
            }
            (*mcore)
                .cur_hallocs = ((*mcore).cur_hallocs as libc::c_ulong)
                .wrapping_sub(
                    (*((*mcore).mops).offset(i as isize)).nbytes as libc::c_ulong,
                ) as size_t as size_t;
            (*mcore).cmop = ((*mcore).cmop).wrapping_sub(1);
            *((*mcore).mops)
                .offset(i as isize) = *((*mcore).mops).offset((*mcore).cmop as isize);
            return;
        }
        i -= 1;
        i;
    }
    gk_errexit(
        6 as libc::c_int,
        b"gkmcoreDel should never have been here!\n\0" as *const u8
            as *const libc::c_char as *mut libc::c_char,
    );
}
