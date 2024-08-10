use ::libc;
extern "C" {
    fn gk_ikvmalloc(n: size_t, msg: *mut libc::c_char) -> *mut gk_ikv_t;
    fn gk_malloc(nbytes: size_t, msg: *mut libc::c_char) -> *mut libc::c_void;
    fn gk_free(ptr1: *mut *mut libc::c_void, _: ...);
    fn gk_errexit(signum: libc::c_int, _: *mut libc::c_char, _: ...);
}
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gk_ikv_t {
    pub key: libc::c_int,
    pub val: ssize_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gk_HTable_t {
    pub nelements: libc::c_int,
    pub htsize: libc::c_int,
    pub harray: *mut gk_ikv_t,
}
#[no_mangle]
pub unsafe extern "C" fn HTable_Create(mut nelements: libc::c_int) -> *mut gk_HTable_t {
    let mut htable: *mut gk_HTable_t = 0 as *mut gk_HTable_t;
    htable = gk_malloc(
        ::core::mem::size_of::<gk_HTable_t>() as libc::c_ulong,
        b"HTable_Create: htable\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    ) as *mut gk_HTable_t;
    (*htable)
        .harray = gk_ikvmalloc(
        nelements as size_t,
        b"HTable_Create: harray\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*htable).nelements = nelements;
    HTable_Reset(htable);
    return htable;
}
#[no_mangle]
pub unsafe extern "C" fn HTable_Reset(mut htable: *mut gk_HTable_t) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*htable).nelements {
        (*((*htable).harray).offset(i as isize)).key = -(1 as libc::c_int);
        i += 1;
        i;
    }
    (*htable).htsize = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn HTable_Resize(
    mut htable: *mut gk_HTable_t,
    mut nelements: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut old_nelements: libc::c_int = 0;
    let mut old_harray: *mut gk_ikv_t = 0 as *mut gk_ikv_t;
    old_nelements = (*htable).nelements;
    old_harray = (*htable).harray;
    (*htable).nelements = nelements;
    (*htable).htsize = 0 as libc::c_int;
    (*htable)
        .harray = gk_ikvmalloc(
        nelements as size_t,
        b"HTable_Resize: harray\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < nelements {
        (*((*htable).harray).offset(i as isize)).key = -(1 as libc::c_int);
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < old_nelements {
        if (*old_harray.offset(i as isize)).key != -(1 as libc::c_int) {
            HTable_Insert(
                htable,
                (*old_harray.offset(i as isize)).key,
                (*old_harray.offset(i as isize)).val as libc::c_int,
            );
        }
        i += 1;
        i;
    }
    gk_free(
        &mut old_harray as *mut *mut gk_ikv_t as *mut *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn HTable_Insert(
    mut htable: *mut gk_HTable_t,
    mut key: libc::c_int,
    mut val: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut first: libc::c_int = 0;
    if (*htable).htsize > (*htable).nelements / 2 as libc::c_int {
        HTable_Resize(htable, 2 as libc::c_int * (*htable).nelements);
    }
    first = HTable_HFunction((*htable).nelements, key);
    i = first;
    while i < (*htable).nelements {
        if (*((*htable).harray).offset(i as isize)).key == -(1 as libc::c_int)
            || (*((*htable).harray).offset(i as isize)).key == -(2 as libc::c_int)
        {
            (*((*htable).harray).offset(i as isize)).key = key;
            (*((*htable).harray).offset(i as isize)).val = val as ssize_t;
            (*htable).htsize += 1;
            (*htable).htsize;
            return;
        }
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < first {
        if (*((*htable).harray).offset(i as isize)).key == -(1 as libc::c_int)
            || (*((*htable).harray).offset(i as isize)).key == -(2 as libc::c_int)
        {
            (*((*htable).harray).offset(i as isize)).key = key;
            (*((*htable).harray).offset(i as isize)).val = val as ssize_t;
            (*htable).htsize += 1;
            (*htable).htsize;
            return;
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn HTable_Delete(
    mut htable: *mut gk_HTable_t,
    mut key: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut first: libc::c_int = 0;
    first = HTable_HFunction((*htable).nelements, key);
    i = first;
    while i < (*htable).nelements {
        if (*((*htable).harray).offset(i as isize)).key == key {
            (*((*htable).harray).offset(i as isize)).key = -(2 as libc::c_int);
            (*htable).htsize -= 1;
            (*htable).htsize;
            return;
        }
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < first {
        if (*((*htable).harray).offset(i as isize)).key == key {
            (*((*htable).harray).offset(i as isize)).key = -(2 as libc::c_int);
            (*htable).htsize -= 1;
            (*htable).htsize;
            return;
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn HTable_Search(
    mut htable: *mut gk_HTable_t,
    mut key: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut first: libc::c_int = 0;
    first = HTable_HFunction((*htable).nelements, key);
    i = first;
    while i < (*htable).nelements {
        if (*((*htable).harray).offset(i as isize)).key == key {
            return (*((*htable).harray).offset(i as isize)).val as libc::c_int
        } else if (*((*htable).harray).offset(i as isize)).key == -(1 as libc::c_int) {
            return -(1 as libc::c_int)
        }
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < first {
        if (*((*htable).harray).offset(i as isize)).key == key {
            return (*((*htable).harray).offset(i as isize)).val as libc::c_int
        } else if (*((*htable).harray).offset(i as isize)).key == -(1 as libc::c_int) {
            return -(1 as libc::c_int)
        }
        i += 1;
        i;
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn HTable_GetNext(
    mut htable: *mut gk_HTable_t,
    mut key: libc::c_int,
    mut r_val: *mut libc::c_int,
    mut type_0: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    static mut first: libc::c_int = 0;
    static mut last: libc::c_int = 0;
    if type_0 == 1 as libc::c_int {
        last = HTable_HFunction((*htable).nelements, key);
        first = last;
    }
    if first > last {
        i = first;
        while i < (*htable).nelements {
            if (*((*htable).harray).offset(i as isize)).key == key {
                *r_val = (*((*htable).harray).offset(i as isize)).val as libc::c_int;
                first = i + 1 as libc::c_int;
                return 1 as libc::c_int;
            } else if (*((*htable).harray).offset(i as isize)).key == -(1 as libc::c_int)
            {
                return -(1 as libc::c_int)
            }
            i += 1;
            i;
        }
        first = 0 as libc::c_int;
    }
    i = first;
    while i < last {
        if (*((*htable).harray).offset(i as isize)).key == key {
            *r_val = (*((*htable).harray).offset(i as isize)).val as libc::c_int;
            first = i + 1 as libc::c_int;
            return 1 as libc::c_int;
        } else if (*((*htable).harray).offset(i as isize)).key == -(1 as libc::c_int) {
            return -(1 as libc::c_int)
        }
        i += 1;
        i;
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn HTable_SearchAndDelete(
    mut htable: *mut gk_HTable_t,
    mut key: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut first: libc::c_int = 0;
    first = HTable_HFunction((*htable).nelements, key);
    i = first;
    while i < (*htable).nelements {
        if (*((*htable).harray).offset(i as isize)).key == key {
            (*((*htable).harray).offset(i as isize)).key = -(2 as libc::c_int);
            (*htable).htsize -= 1;
            (*htable).htsize;
            return (*((*htable).harray).offset(i as isize)).val as libc::c_int;
        } else if (*((*htable).harray).offset(i as isize)).key == -(1 as libc::c_int) {
            gk_errexit(
                15 as libc::c_int,
                b"HTable_SearchAndDelete: Failed to find the key!\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < first {
        if (*((*htable).harray).offset(i as isize)).key == key {
            (*((*htable).harray).offset(i as isize)).key = -(2 as libc::c_int);
            (*htable).htsize -= 1;
            (*htable).htsize;
            return (*((*htable).harray).offset(i as isize)).val as libc::c_int;
        } else if (*((*htable).harray).offset(i as isize)).key == -(1 as libc::c_int) {
            gk_errexit(
                15 as libc::c_int,
                b"HTable_SearchAndDelete: Failed to find the key!\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
        i += 1;
        i;
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn HTable_Destroy(mut htable: *mut gk_HTable_t) {
    gk_free(
        &mut (*htable).harray as *mut *mut gk_ikv_t as *mut *mut libc::c_void,
        &mut htable as *mut *mut gk_HTable_t,
        0 as *mut *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn HTable_HFunction(
    mut nelements: libc::c_int,
    mut key: libc::c_int,
) -> libc::c_int {
    return key % nelements;
}
