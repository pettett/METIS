use ::libc;
extern "C" {
    fn gk_iincset(
        n: size_t,
        baseval: libc::c_int,
        x: *mut libc::c_int,
    ) -> *mut libc::c_int;
    fn gk_iargmax(n: size_t, x: *mut libc::c_int) -> size_t;
    fn gk_imalloc(n: size_t, msg: *mut libc::c_char) -> *mut libc::c_int;
    fn gk_ismalloc(
        n: size_t,
        ival: libc::c_int,
        msg: *mut libc::c_char,
    ) -> *mut libc::c_int;
    fn gk_iset(n: size_t, val: libc::c_int, x: *mut libc::c_int) -> *mut libc::c_int;
    fn gk_icopy(n: size_t, a: *mut libc::c_int, b: *mut libc::c_int) -> *mut libc::c_int;
    fn gk_zmalloc(n: size_t, msg: *mut libc::c_char) -> *mut ssize_t;
    fn gk_zcopy(n: size_t, a: *mut ssize_t, b: *mut ssize_t) -> *mut ssize_t;
    fn gk_ikvmalloc(n: size_t, msg: *mut libc::c_char) -> *mut gk_ikv_t;
    fn gk_free(ptr1: *mut *mut libc::c_void, _: ...);
    fn gk_ikvsorti(_: size_t, _: *mut gk_ikv_t);
    fn gk_csr_Create() -> *mut gk_csr_t;
    fn gk_csr_Free(mat: *mut *mut gk_csr_t);
    fn gk_csr_CreateIndex(mat: *mut gk_csr_t, what: libc::c_int);
}
pub type __int32_t = libc::c_int;
pub type __ssize_t = i64;
pub type int32_t = __int32_t;
pub type ssize_t = __ssize_t;
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gk_ikv_t {
    pub key: libc::c_int,
    pub val: ssize_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gk_csr_t {
    pub nrows: int32_t,
    pub ncols: int32_t,
    pub rowptr: *mut ssize_t,
    pub colptr: *mut ssize_t,
    pub rowind: *mut int32_t,
    pub colind: *mut int32_t,
    pub rowids: *mut int32_t,
    pub colids: *mut int32_t,
    pub rowval: *mut libc::c_float,
    pub colval: *mut libc::c_float,
    pub rnorms: *mut libc::c_float,
    pub cnorms: *mut libc::c_float,
    pub rsums: *mut libc::c_float,
    pub csums: *mut libc::c_float,
    pub rsizes: *mut libc::c_float,
    pub csizes: *mut libc::c_float,
    pub rvols: *mut libc::c_float,
    pub cvols: *mut libc::c_float,
    pub rwgts: *mut libc::c_float,
    pub cwgts: *mut libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct isparams_t {
    pub minfreq: libc::c_int,
    pub maxfreq: libc::c_int,
    pub minlen: libc::c_int,
    pub maxlen: libc::c_int,
    pub tnitems: libc::c_int,
    pub callback: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_int,
            *mut libc::c_int,
            libc::c_int,
            *mut libc::c_int,
        ) -> (),
    >,
    pub stateptr: *mut libc::c_void,
    pub rmarker: *mut libc::c_int,
    pub cand: *mut gk_ikv_t,
}
#[no_mangle]
pub unsafe extern "C" fn gk_find_frequent_itemsets(
    mut ntrans: libc::c_int,
    mut tranptr: *mut ssize_t,
    mut tranind: *mut libc::c_int,
    mut minfreq: libc::c_int,
    mut maxfreq: libc::c_int,
    mut minlen: libc::c_int,
    mut maxlen: libc::c_int,
    mut process_itemset: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_int,
            *mut libc::c_int,
            libc::c_int,
            *mut libc::c_int,
        ) -> (),
    >,
    mut stateptr: *mut libc::c_void,
) {
    let mut i: ssize_t = 0;
    let mut mat: *mut gk_csr_t = 0 as *mut gk_csr_t;
    let mut pmat: *mut gk_csr_t = 0 as *mut gk_csr_t;
    let mut params: isparams_t = isparams_t {
        minfreq: 0,
        maxfreq: 0,
        minlen: 0,
        maxlen: 0,
        tnitems: 0,
        callback: None,
        stateptr: 0 as *mut libc::c_void,
        rmarker: 0 as *mut libc::c_int,
        cand: 0 as *mut gk_ikv_t,
    };
    let mut pattern: *mut libc::c_int = 0 as *mut libc::c_int;
    mat = gk_csr_Create();
    (*mat).nrows = ntrans;
    (*mat)
        .ncols = *tranind
        .offset(gk_iargmax(*tranptr.offset(ntrans as isize) as size_t, tranind) as isize)
        + 1 as libc::c_int;
    (*mat)
        .rowptr = gk_zcopy(
        (ntrans + 1 as libc::c_int) as size_t,
        tranptr,
        gk_zmalloc(
            (ntrans + 1 as libc::c_int) as size_t,
            b"gk_find_frequent_itemsets: mat.rowptr\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        ),
    );
    (*mat)
        .rowind = gk_icopy(
        *tranptr.offset(ntrans as isize) as size_t,
        tranind,
        gk_imalloc(
            *tranptr.offset(ntrans as isize) as size_t,
            b"gk_find_frequent_itemsets: mat.rowind\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        ),
    );
    (*mat)
        .colids = gk_iincset(
        (*mat).ncols as size_t,
        0 as libc::c_int,
        gk_imalloc(
            (*mat).ncols as size_t,
            b"gk_find_frequent_itemsets: mat.colids\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        ),
    );
    params.minfreq = minfreq;
    params.maxfreq = if maxfreq == -(1 as libc::c_int) { (*mat).nrows } else { maxfreq };
    params.minlen = minlen;
    params.maxlen = if maxlen == -(1 as libc::c_int) { (*mat).ncols } else { maxlen };
    params.tnitems = (*mat).ncols;
    params.callback = process_itemset;
    params.stateptr = stateptr;
    params
        .rmarker = gk_ismalloc(
        (*mat).nrows as size_t,
        0 as libc::c_int,
        b"gk_find_frequent_itemsets: rmarker\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    params
        .cand = gk_ikvmalloc(
        (*mat).ncols as size_t,
        b"gk_find_frequent_itemsets: cand\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    gk_csr_CreateIndex(mat, 2 as libc::c_int);
    pmat = itemsets_project_matrix(&mut params, mat, -(1 as libc::c_int));
    gk_csr_Free(&mut mat);
    pattern = gk_imalloc(
        (*pmat).ncols as size_t,
        b"gk_find_frequent_itemsets: pattern\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    itemsets_find_frequent_itemsets(&mut params, pmat, 0 as libc::c_int, pattern);
    gk_csr_Free(&mut pmat);
    gk_free(
        &mut pattern as *mut *mut libc::c_int as *mut *mut libc::c_void,
        &mut params.rmarker as *mut *mut libc::c_int,
        &mut params.cand as *mut *mut gk_ikv_t,
        0 as *mut *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn itemsets_find_frequent_itemsets(
    mut params: *mut isparams_t,
    mut mat: *mut gk_csr_t,
    mut preflen: libc::c_int,
    mut prefix: *mut libc::c_int,
) {
    let mut i: ssize_t = 0;
    let mut cmat: *mut gk_csr_t = 0 as *mut gk_csr_t;
    i = 0 as libc::c_int as ssize_t;
    while i < (*mat).ncols as i64 {
        *prefix.offset(preflen as isize) = *((*mat).colids).offset(i as isize);
        if preflen + 1 as libc::c_int >= (*params).minlen {
            (Some(((*params).callback).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                (*params).stateptr,
                preflen + 1 as libc::c_int,
                prefix,
                (*((*mat).colptr).offset((i + 1 as libc::c_int as i64) as isize)
                    - *((*mat).colptr).offset(i as isize)) as libc::c_int,
                ((*mat).colind).offset(*((*mat).colptr).offset(i as isize) as isize),
            );
        }
        if (preflen + 1 as libc::c_int) < (*params).maxlen {
            cmat = itemsets_project_matrix(params, mat, i as libc::c_int);
            itemsets_find_frequent_itemsets(
                params,
                cmat,
                preflen + 1 as libc::c_int,
                prefix,
            );
            gk_csr_Free(&mut cmat);
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn itemsets_project_matrix(
    mut params: *mut isparams_t,
    mut mat: *mut gk_csr_t,
    mut cid: libc::c_int,
) -> *mut gk_csr_t {
    let mut i: ssize_t = 0;
    let mut j: ssize_t = 0;
    let mut k: ssize_t = 0;
    let mut ii: ssize_t = 0;
    let mut pnnz: ssize_t = 0;
    let mut nrows: libc::c_int = 0;
    let mut ncols: libc::c_int = 0;
    let mut pnrows: libc::c_int = 0;
    let mut pncols: libc::c_int = 0;
    let mut colptr: *mut ssize_t = 0 as *mut ssize_t;
    let mut pcolptr: *mut ssize_t = 0 as *mut ssize_t;
    let mut colind: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut colids: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut pcolind: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut pcolids: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut rmarker: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut pmat: *mut gk_csr_t = 0 as *mut gk_csr_t;
    let mut cand: *mut gk_ikv_t = 0 as *mut gk_ikv_t;
    nrows = (*mat).nrows;
    ncols = (*mat).ncols;
    colptr = (*mat).colptr;
    colind = (*mat).colind;
    colids = (*mat).colids;
    rmarker = (*params).rmarker;
    cand = (*params).cand;
    pmat = gk_csr_Create();
    pnrows = (if cid == -(1 as libc::c_int) {
        nrows as i64
    } else {
        *colptr.offset((cid + 1 as libc::c_int) as isize) - *colptr.offset(cid as isize)
    }) as libc::c_int;
    (*pmat).nrows = pnrows;
    if cid == -(1 as libc::c_int) {
        gk_iset(nrows as size_t, 1 as libc::c_int, rmarker);
    } else {
        i = *colptr.offset(cid as isize);
        while i < *colptr.offset((cid + 1 as libc::c_int) as isize) {
            *rmarker.offset(*colind.offset(i as isize) as isize) = 1 as libc::c_int;
            i += 1;
            i;
        }
    }
    pncols = 0 as libc::c_int;
    pnnz = 0 as libc::c_int as ssize_t;
    i = (cid + 1 as libc::c_int) as ssize_t;
    while i < ncols as i64 {
        k = 0 as libc::c_int as ssize_t;
        j = *colptr.offset(i as isize);
        while j < *colptr.offset((i + 1 as libc::c_int as i64) as isize) {
            k += *rmarker.offset(*colind.offset(j as isize) as isize) as i64;
            j += 1;
            j;
        }
        if k >= (*params).minfreq as i64
            && k <= (*params).maxfreq as i64
        {
            (*cand.offset(pncols as isize)).val = i;
            let fresh0 = pncols;
            pncols = pncols + 1;
            (*cand.offset(fresh0 as isize)).key = k as libc::c_int;
            pnnz += k;
        }
        i += 1;
        i;
    }
    gk_ikvsorti(pncols as size_t, cand);
    (*pmat).ncols = pncols;
    pcolids = gk_imalloc(
        pncols as size_t,
        b"itemsets_project_matrix: pcolids\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*pmat).colids = pcolids;
    pcolptr = gk_zmalloc(
        (pncols + 1 as libc::c_int) as size_t,
        b"itemsets_project_matrix: pcolptr\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*pmat).colptr = pcolptr;
    pcolind = gk_imalloc(
        pnnz as size_t,
        b"itemsets_project_matrix: pcolind\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*pmat).colind = pcolind;
    *pcolptr.offset(0 as libc::c_int as isize) = 0 as libc::c_int as ssize_t;
    pnnz = 0 as libc::c_int as ssize_t;
    ii = 0 as libc::c_int as ssize_t;
    while ii < pncols as i64 {
        i = (*cand.offset(ii as isize)).val;
        j = *colptr.offset(i as isize);
        while j < *colptr.offset((i + 1 as libc::c_int as i64) as isize) {
            if *rmarker.offset(*colind.offset(j as isize) as isize) != 0 {
                let fresh1 = pnnz;
                pnnz = pnnz + 1;
                *pcolind.offset(fresh1 as isize) = *colind.offset(j as isize);
            }
            j += 1;
            j;
        }
        *pcolids.offset(ii as isize) = *colids.offset(i as isize);
        *pcolptr.offset((ii + 1 as libc::c_int as i64) as isize) = pnnz;
        ii += 1;
        ii;
    }
    if cid == -(1 as libc::c_int) {
        gk_iset(nrows as size_t, 0 as libc::c_int, rmarker);
    } else {
        i = *colptr.offset(cid as isize);
        while i < *colptr.offset((cid + 1 as libc::c_int) as isize) {
            *rmarker.offset(*colind.offset(i as isize) as isize) = 0 as libc::c_int;
            i += 1;
            i;
        }
    }
    return pmat;
}
