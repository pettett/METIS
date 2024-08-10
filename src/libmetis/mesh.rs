use ::libc;
extern "C" {
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;
    #[thread_local]
    static mut gk_cur_jbufs: libc::c_int;
    #[thread_local]
    static mut gk_jbufs: [jmp_buf; 0];
    fn gk_malloc_init() -> libc::c_int;
    fn gk_malloc_cleanup(showstats: libc::c_int);
    fn gk_malloc(nbytes: size_t, msg: *mut libc::c_char) -> *mut libc::c_void;
    fn gk_free(ptr1: *mut *mut libc::c_void, _: ...);
    fn gk_errexit(signum: libc::c_int, _: *mut libc::c_char, _: ...);
    fn gk_sigtrap() -> libc::c_int;
    fn gk_siguntrap() -> libc::c_int;
    fn libmetis__metis_rcode(sigrval: libc::c_int) -> libc::c_int;
    fn libmetis__ChangeMesh2FNumbering(
        n: idx_t,
        ptr: *mut idx_t,
        ind: *mut idx_t,
        nvtxs: idx_t,
        xadj: *mut idx_t,
        adjncy: *mut idx_t,
    );
    fn libmetis__imalloc(n: size_t, msg: *mut libc::c_char) -> *mut idx_t;
    fn libmetis__ismalloc(n: size_t, ival: idx_t, msg: *mut libc::c_char) -> *mut idx_t;
    fn libmetis__iset(n: size_t, val: idx_t, x: *mut idx_t) -> *mut idx_t;
    fn libmetis__ChangeMesh2CNumbering(n: idx_t, ptr: *mut idx_t, ind: *mut idx_t);
}
pub type __int32_t = libc::c_int;
pub type int32_t = __int32_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type __jmp_buf = [libc::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
pub type idx_t = int32_t;
pub const METIS_ERROR_MEMORY: C2RustUnnamed = -3;
pub type C2RustUnnamed = libc::c_int;
pub const METIS_ERROR: C2RustUnnamed = -4;
pub const METIS_ERROR_INPUT: C2RustUnnamed = -2;
pub const METIS_OK: C2RustUnnamed = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mesh_t {
    pub ne: idx_t,
    pub nn: idx_t,
    pub ncon: idx_t,
    pub eptr: *mut idx_t,
    pub eind: *mut idx_t,
    pub ewgt: *mut idx_t,
}
#[no_mangle]
pub unsafe extern "C" fn METIS_MeshToDual(
    mut ne: *mut idx_t,
    mut nn: *mut idx_t,
    mut eptr: *mut idx_t,
    mut eind: *mut idx_t,
    mut ncommon: *mut idx_t,
    mut numflag: *mut idx_t,
    mut r_xadj: *mut *mut idx_t,
    mut r_adjncy: *mut *mut idx_t,
) -> libc::c_int {
    let mut sigrval: libc::c_int = 0 as libc::c_int;
    let mut renumber: libc::c_int = 0 as libc::c_int;
    if gk_malloc_init() == 0 {
        return METIS_ERROR_MEMORY as libc::c_int;
    }
    gk_sigtrap();
    sigrval = _setjmp(
        (*gk_jbufs.as_mut_ptr().offset(gk_cur_jbufs as isize)).as_mut_ptr(),
    );
    if !(sigrval != 0 as libc::c_int) {
        if *numflag == 1 as libc::c_int {
            libmetis__ChangeMesh2CNumbering(*ne, eptr, eind);
            renumber = 1 as libc::c_int;
        }
        *r_adjncy = 0 as *mut idx_t;
        *r_xadj = *r_adjncy;
        libmetis__CreateGraphDual(*ne, *nn, eptr, eind, *ncommon, r_xadj, r_adjncy);
    }
    if renumber != 0 {
        libmetis__ChangeMesh2FNumbering(*ne, eptr, eind, *ne, *r_xadj, *r_adjncy);
    }
    gk_siguntrap();
    gk_malloc_cleanup(0 as libc::c_int);
    if sigrval != 0 as libc::c_int {
        if !(*r_xadj).is_null() {
            free(*r_xadj as *mut libc::c_void);
        }
        if !(*r_adjncy).is_null() {
            free(*r_adjncy as *mut libc::c_void);
        }
        *r_adjncy = 0 as *mut idx_t;
        *r_xadj = *r_adjncy;
    }
    return libmetis__metis_rcode(sigrval);
}
#[no_mangle]
pub unsafe extern "C" fn METIS_MeshToNodal(
    mut ne: *mut idx_t,
    mut nn: *mut idx_t,
    mut eptr: *mut idx_t,
    mut eind: *mut idx_t,
    mut numflag: *mut idx_t,
    mut r_xadj: *mut *mut idx_t,
    mut r_adjncy: *mut *mut idx_t,
) -> libc::c_int {
    let mut sigrval: libc::c_int = 0 as libc::c_int;
    let mut renumber: libc::c_int = 0 as libc::c_int;
    if gk_malloc_init() == 0 {
        return METIS_ERROR_MEMORY as libc::c_int;
    }
    gk_sigtrap();
    sigrval = _setjmp(
        (*gk_jbufs.as_mut_ptr().offset(gk_cur_jbufs as isize)).as_mut_ptr(),
    );
    if !(sigrval != 0 as libc::c_int) {
        if *numflag == 1 as libc::c_int {
            libmetis__ChangeMesh2CNumbering(*ne, eptr, eind);
            renumber = 1 as libc::c_int;
        }
        *r_adjncy = 0 as *mut idx_t;
        *r_xadj = *r_adjncy;
        libmetis__CreateGraphNodal(*ne, *nn, eptr, eind, r_xadj, r_adjncy);
    }
    if renumber != 0 {
        libmetis__ChangeMesh2FNumbering(*ne, eptr, eind, *nn, *r_xadj, *r_adjncy);
    }
    gk_siguntrap();
    gk_malloc_cleanup(0 as libc::c_int);
    if sigrval != 0 as libc::c_int {
        if !(*r_xadj).is_null() {
            free(*r_xadj as *mut libc::c_void);
        }
        if !(*r_adjncy).is_null() {
            free(*r_adjncy as *mut libc::c_void);
        }
        *r_adjncy = 0 as *mut idx_t;
        *r_xadj = *r_adjncy;
    }
    return libmetis__metis_rcode(sigrval);
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__CreateGraphDual(
    mut ne: idx_t,
    mut nn: idx_t,
    mut eptr: *mut idx_t,
    mut eind: *mut idx_t,
    mut ncommon: idx_t,
    mut r_xadj: *mut *mut idx_t,
    mut r_adjncy: *mut *mut idx_t,
) {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut nnbrs: idx_t = 0;
    let mut nptr: *mut idx_t = 0 as *mut idx_t;
    let mut nind: *mut idx_t = 0 as *mut idx_t;
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut marker: *mut idx_t = 0 as *mut idx_t;
    let mut nbrs: *mut idx_t = 0 as *mut idx_t;
    if ncommon < 1 as libc::c_int {
        printf(
            b"  Increased ncommon to 1, as it was initially %d\n\0" as *const u8
                as *const libc::c_char,
            ncommon,
        );
        ncommon = 1 as libc::c_int;
    }
    nptr = libmetis__ismalloc(
        (nn + 1 as libc::c_int) as size_t,
        0 as libc::c_int,
        b"CreateGraphDual: nptr\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    nind = libmetis__imalloc(
        *eptr.offset(ne as isize) as size_t,
        b"CreateGraphDual: nind\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < ne {
        j = *eptr.offset(i as isize);
        while j < *eptr.offset((i + 1 as libc::c_int) as isize) {
            let ref mut fresh0 = *nptr.offset(*eind.offset(j as isize) as isize);
            *fresh0 += 1;
            *fresh0;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    i = 1 as libc::c_int;
    while i < nn {
        let ref mut fresh1 = *nptr.offset(i as isize);
        *fresh1 += *nptr.offset((i - 1 as libc::c_int) as isize);
        i += 1;
        i;
    }
    i = nn;
    while i > 0 as libc::c_int {
        *nptr.offset(i as isize) = *nptr.offset((i - 1 as libc::c_int) as isize);
        i -= 1;
        i;
    }
    *nptr.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < ne {
        j = *eptr.offset(i as isize);
        while j < *eptr.offset((i + 1 as libc::c_int) as isize) {
            let ref mut fresh2 = *nptr.offset(*eind.offset(j as isize) as isize);
            let fresh3 = *fresh2;
            *fresh2 = *fresh2 + 1;
            *nind.offset(fresh3 as isize) = i;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    i = nn;
    while i > 0 as libc::c_int {
        *nptr.offset(i as isize) = *nptr.offset((i - 1 as libc::c_int) as isize);
        i -= 1;
        i;
    }
    *nptr.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    xadj = malloc(
        ((ne + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<idx_t>() as libc::c_ulong),
    ) as *mut idx_t;
    if xadj.is_null() {
        gk_errexit(
            6 as libc::c_int,
            b"***Failed to allocate memory for xadj.\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
    }
    *r_xadj = xadj;
    libmetis__iset((ne + 1 as libc::c_int) as size_t, 0 as libc::c_int, xadj);
    marker = libmetis__ismalloc(
        ne as size_t,
        0 as libc::c_int,
        b"CreateGraphDual: marker\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    nbrs = libmetis__imalloc(
        ne as size_t,
        b"CreateGraphDual: nbrs\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < ne {
        *xadj
            .offset(
                i as isize,
            ) = libmetis__FindCommonElements(
            i,
            *eptr.offset((i + 1 as libc::c_int) as isize) - *eptr.offset(i as isize),
            eind.offset(*eptr.offset(i as isize) as isize),
            nptr,
            nind,
            eptr,
            ncommon,
            marker,
            nbrs,
        );
        i += 1;
        i;
    }
    i = 1 as libc::c_int;
    while i < ne {
        let ref mut fresh4 = *xadj.offset(i as isize);
        *fresh4 += *xadj.offset((i - 1 as libc::c_int) as isize);
        i += 1;
        i;
    }
    i = ne;
    while i > 0 as libc::c_int {
        *xadj.offset(i as isize) = *xadj.offset((i - 1 as libc::c_int) as isize);
        i -= 1;
        i;
    }
    *xadj.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    adjncy = malloc(
        (*xadj.offset(ne as isize) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<idx_t>() as libc::c_ulong),
    ) as *mut idx_t;
    if adjncy.is_null() {
        free(xadj as *mut libc::c_void);
        *r_xadj = 0 as *mut idx_t;
        gk_errexit(
            6 as libc::c_int,
            b"***Failed to allocate memory for adjncy.\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
    }
    *r_adjncy = adjncy;
    i = 0 as libc::c_int;
    while i < ne {
        nnbrs = libmetis__FindCommonElements(
            i,
            *eptr.offset((i + 1 as libc::c_int) as isize) - *eptr.offset(i as isize),
            eind.offset(*eptr.offset(i as isize) as isize),
            nptr,
            nind,
            eptr,
            ncommon,
            marker,
            nbrs,
        );
        j = 0 as libc::c_int;
        while j < nnbrs {
            let ref mut fresh5 = *xadj.offset(i as isize);
            let fresh6 = *fresh5;
            *fresh5 = *fresh5 + 1;
            *adjncy.offset(fresh6 as isize) = *nbrs.offset(j as isize);
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    i = ne;
    while i > 0 as libc::c_int {
        *xadj.offset(i as isize) = *xadj.offset((i - 1 as libc::c_int) as isize);
        i -= 1;
        i;
    }
    *xadj.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    gk_free(
        &mut nptr as *mut *mut idx_t as *mut *mut libc::c_void,
        &mut nind as *mut *mut idx_t,
        &mut marker as *mut *mut idx_t,
        &mut nbrs as *mut *mut idx_t,
        0 as *mut *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__FindCommonElements(
    mut qid: idx_t,
    mut elen: idx_t,
    mut eind: *mut idx_t,
    mut nptr: *mut idx_t,
    mut nind: *mut idx_t,
    mut eptr: *mut idx_t,
    mut ncommon: idx_t,
    mut marker: *mut idx_t,
    mut nbrs: *mut idx_t,
) -> idx_t {
    let mut i: idx_t = 0;
    let mut ii: idx_t = 0;
    let mut j: idx_t = 0;
    let mut jj: idx_t = 0;
    let mut k: idx_t = 0;
    let mut l: idx_t = 0;
    let mut overlap: idx_t = 0;
    k = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < elen {
        j = *eind.offset(i as isize);
        ii = *nptr.offset(j as isize);
        while ii < *nptr.offset((j + 1 as libc::c_int) as isize) {
            jj = *nind.offset(ii as isize);
            if *marker.offset(jj as isize) == 0 as libc::c_int {
                let fresh7 = k;
                k = k + 1;
                *nbrs.offset(fresh7 as isize) = jj;
            }
            let ref mut fresh8 = *marker.offset(jj as isize);
            *fresh8 += 1;
            *fresh8;
            ii += 1;
            ii;
        }
        i += 1;
        i;
    }
    if *marker.offset(qid as isize) == 0 as libc::c_int {
        let fresh9 = k;
        k = k + 1;
        *nbrs.offset(fresh9 as isize) = qid;
    }
    *marker.offset(qid as isize) = 0 as libc::c_int;
    j = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < k {
        l = *nbrs.offset(i as isize);
        overlap = *marker.offset(l as isize);
        if overlap >= ncommon || overlap >= elen - 1 as libc::c_int
            || overlap
                >= *eptr.offset((l + 1 as libc::c_int) as isize)
                    - *eptr.offset(l as isize) - 1 as libc::c_int
        {
            let fresh10 = j;
            j = j + 1;
            *nbrs.offset(fresh10 as isize) = l;
        }
        *marker.offset(l as isize) = 0 as libc::c_int;
        i += 1;
        i;
    }
    return j;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__CreateGraphNodal(
    mut ne: idx_t,
    mut nn: idx_t,
    mut eptr: *mut idx_t,
    mut eind: *mut idx_t,
    mut r_xadj: *mut *mut idx_t,
    mut r_adjncy: *mut *mut idx_t,
) {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut nnbrs: idx_t = 0;
    let mut nptr: *mut idx_t = 0 as *mut idx_t;
    let mut nind: *mut idx_t = 0 as *mut idx_t;
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut marker: *mut idx_t = 0 as *mut idx_t;
    let mut nbrs: *mut idx_t = 0 as *mut idx_t;
    nptr = libmetis__ismalloc(
        (nn + 1 as libc::c_int) as size_t,
        0 as libc::c_int,
        b"CreateGraphNodal: nptr\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    nind = libmetis__imalloc(
        *eptr.offset(ne as isize) as size_t,
        b"CreateGraphNodal: nind\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < ne {
        j = *eptr.offset(i as isize);
        while j < *eptr.offset((i + 1 as libc::c_int) as isize) {
            let ref mut fresh11 = *nptr.offset(*eind.offset(j as isize) as isize);
            *fresh11 += 1;
            *fresh11;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    i = 1 as libc::c_int;
    while i < nn {
        let ref mut fresh12 = *nptr.offset(i as isize);
        *fresh12 += *nptr.offset((i - 1 as libc::c_int) as isize);
        i += 1;
        i;
    }
    i = nn;
    while i > 0 as libc::c_int {
        *nptr.offset(i as isize) = *nptr.offset((i - 1 as libc::c_int) as isize);
        i -= 1;
        i;
    }
    *nptr.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < ne {
        j = *eptr.offset(i as isize);
        while j < *eptr.offset((i + 1 as libc::c_int) as isize) {
            let ref mut fresh13 = *nptr.offset(*eind.offset(j as isize) as isize);
            let fresh14 = *fresh13;
            *fresh13 = *fresh13 + 1;
            *nind.offset(fresh14 as isize) = i;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    i = nn;
    while i > 0 as libc::c_int {
        *nptr.offset(i as isize) = *nptr.offset((i - 1 as libc::c_int) as isize);
        i -= 1;
        i;
    }
    *nptr.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    xadj = malloc(
        ((nn + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<idx_t>() as libc::c_ulong),
    ) as *mut idx_t;
    if xadj.is_null() {
        gk_errexit(
            6 as libc::c_int,
            b"***Failed to allocate memory for xadj.\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
    }
    *r_xadj = xadj;
    libmetis__iset((nn + 1 as libc::c_int) as size_t, 0 as libc::c_int, xadj);
    marker = libmetis__ismalloc(
        nn as size_t,
        0 as libc::c_int,
        b"CreateGraphNodal: marker\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    nbrs = libmetis__imalloc(
        nn as size_t,
        b"CreateGraphNodal: nbrs\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < nn {
        *xadj
            .offset(
                i as isize,
            ) = libmetis__FindCommonNodes(
            i,
            *nptr.offset((i + 1 as libc::c_int) as isize) - *nptr.offset(i as isize),
            nind.offset(*nptr.offset(i as isize) as isize),
            eptr,
            eind,
            marker,
            nbrs,
        );
        i += 1;
        i;
    }
    i = 1 as libc::c_int;
    while i < nn {
        let ref mut fresh15 = *xadj.offset(i as isize);
        *fresh15 += *xadj.offset((i - 1 as libc::c_int) as isize);
        i += 1;
        i;
    }
    i = nn;
    while i > 0 as libc::c_int {
        *xadj.offset(i as isize) = *xadj.offset((i - 1 as libc::c_int) as isize);
        i -= 1;
        i;
    }
    *xadj.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    adjncy = malloc(
        (*xadj.offset(nn as isize) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<idx_t>() as libc::c_ulong),
    ) as *mut idx_t;
    if adjncy.is_null() {
        free(xadj as *mut libc::c_void);
        *r_xadj = 0 as *mut idx_t;
        gk_errexit(
            6 as libc::c_int,
            b"***Failed to allocate memory for adjncy.\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
    }
    *r_adjncy = adjncy;
    i = 0 as libc::c_int;
    while i < nn {
        nnbrs = libmetis__FindCommonNodes(
            i,
            *nptr.offset((i + 1 as libc::c_int) as isize) - *nptr.offset(i as isize),
            nind.offset(*nptr.offset(i as isize) as isize),
            eptr,
            eind,
            marker,
            nbrs,
        );
        j = 0 as libc::c_int;
        while j < nnbrs {
            let ref mut fresh16 = *xadj.offset(i as isize);
            let fresh17 = *fresh16;
            *fresh16 = *fresh16 + 1;
            *adjncy.offset(fresh17 as isize) = *nbrs.offset(j as isize);
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    i = nn;
    while i > 0 as libc::c_int {
        *xadj.offset(i as isize) = *xadj.offset((i - 1 as libc::c_int) as isize);
        i -= 1;
        i;
    }
    *xadj.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    gk_free(
        &mut nptr as *mut *mut idx_t as *mut *mut libc::c_void,
        &mut nind as *mut *mut idx_t,
        &mut marker as *mut *mut idx_t,
        &mut nbrs as *mut *mut idx_t,
        0 as *mut *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__FindCommonNodes(
    mut qid: idx_t,
    mut nelmnts: idx_t,
    mut elmntids: *mut idx_t,
    mut eptr: *mut idx_t,
    mut eind: *mut idx_t,
    mut marker: *mut idx_t,
    mut nbrs: *mut idx_t,
) -> idx_t {
    let mut i: idx_t = 0;
    let mut ii: idx_t = 0;
    let mut j: idx_t = 0;
    let mut jj: idx_t = 0;
    let mut k: idx_t = 0;
    *marker.offset(qid as isize) = 1 as libc::c_int;
    k = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < nelmnts {
        j = *elmntids.offset(i as isize);
        ii = *eptr.offset(j as isize);
        while ii < *eptr.offset((j + 1 as libc::c_int) as isize) {
            jj = *eind.offset(ii as isize);
            if *marker.offset(jj as isize) == 0 as libc::c_int {
                let fresh18 = k;
                k = k + 1;
                *nbrs.offset(fresh18 as isize) = jj;
                *marker.offset(jj as isize) = 1 as libc::c_int;
            }
            ii += 1;
            ii;
        }
        i += 1;
        i;
    }
    *marker.offset(qid as isize) = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < k {
        *marker.offset(*nbrs.offset(i as isize) as isize) = 0 as libc::c_int;
        i += 1;
        i;
    }
    return k;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__CreateMesh() -> *mut mesh_t {
    let mut mesh: *mut mesh_t = 0 as *mut mesh_t;
    mesh = gk_malloc(
        ::core::mem::size_of::<mesh_t>() as libc::c_ulong,
        b"CreateMesh: mesh\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut mesh_t;
    libmetis__InitMesh(mesh);
    return mesh;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__InitMesh(mut mesh: *mut mesh_t) {
    memset(
        mesh as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<mesh_t>() as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__FreeMesh(mut r_mesh: *mut *mut mesh_t) {
    let mut mesh: *mut mesh_t = *r_mesh;
    gk_free(
        &mut (*mesh).eptr as *mut *mut idx_t as *mut *mut libc::c_void,
        &mut (*mesh).eind as *mut *mut idx_t,
        &mut (*mesh).ewgt as *mut *mut idx_t,
        &mut mesh as *mut *mut mesh_t,
        0 as *mut *mut libc::c_void,
    );
    *r_mesh = 0 as *mut mesh_t;
}
