use ::libc;
extern "C" {
    fn raise(__sig: libc::c_int) -> libc::c_int;
    fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;
    #[thread_local]
    static mut gk_cur_jbufs: libc::c_int;
    #[thread_local]
    static mut gk_jbufs: [jmp_buf; 0];
    fn gk_malloc_init() -> libc::c_int;
    fn gk_malloc_cleanup(showstats: libc::c_int);
    fn gk_free(ptr1: *mut *mut libc::c_void, _: ...);
    fn gk_sigtrap() -> libc::c_int;
    fn gk_siguntrap() -> libc::c_int;
    fn METIS_PartGraphRecursive(
        nvtxs: *mut idx_t,
        ncon: *mut idx_t,
        xadj: *mut idx_t,
        adjncy: *mut idx_t,
        vwgt: *mut idx_t,
        vsize: *mut idx_t,
        adjwgt: *mut idx_t,
        nparts: *mut idx_t,
        tpwgts: *mut real_t,
        ubvec: *mut real_t,
        options: *mut idx_t,
        edgecut: *mut idx_t,
        part: *mut idx_t,
    ) -> libc::c_int;
    fn METIS_PartGraphKway(
        nvtxs: *mut idx_t,
        ncon: *mut idx_t,
        xadj: *mut idx_t,
        adjncy: *mut idx_t,
        vwgt: *mut idx_t,
        vsize: *mut idx_t,
        adjwgt: *mut idx_t,
        nparts: *mut idx_t,
        tpwgts: *mut real_t,
        ubvec: *mut real_t,
        options: *mut idx_t,
        edgecut: *mut idx_t,
        part: *mut idx_t,
    ) -> libc::c_int;
    fn METIS_MeshToDual(
        ne: *mut idx_t,
        nn: *mut idx_t,
        eptr: *mut idx_t,
        eind: *mut idx_t,
        ncommon: *mut idx_t,
        numflag: *mut idx_t,
        r_xadj: *mut *mut idx_t,
        r_adjncy: *mut *mut idx_t,
    ) -> libc::c_int;
    fn METIS_MeshToNodal(
        ne: *mut idx_t,
        nn: *mut idx_t,
        eptr: *mut idx_t,
        eind: *mut idx_t,
        numflag: *mut idx_t,
        r_xadj: *mut *mut idx_t,
        r_adjncy: *mut *mut idx_t,
    ) -> libc::c_int;
    fn libmetis__metis_rcode(sigrval: libc::c_int) -> libc::c_int;
    fn METIS_Free(ptr: *mut libc::c_void) -> libc::c_int;
    fn libmetis__ChangeMesh2FNumbering2(
        ne: idx_t,
        nn: idx_t,
        ptr: *mut idx_t,
        ind: *mut idx_t,
        epart: *mut idx_t,
        npart: *mut idx_t,
    );
    fn libmetis__iargmax(n: size_t, x: *mut idx_t) -> size_t;
    fn libmetis__iset(n: size_t, val: idx_t, x: *mut idx_t) -> *mut idx_t;
    fn libmetis__imalloc(n: size_t, msg: *mut libc::c_char) -> *mut idx_t;
    fn libmetis__ismalloc(n: size_t, ival: idx_t, msg: *mut libc::c_char) -> *mut idx_t;
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
pub type real_t = libc::c_float;
pub const METIS_OPTION_NUMBERING: C2RustUnnamed_0 = 17;
pub const METIS_OK: C2RustUnnamed = 1;
pub const METIS_PTYPE_KWAY: C2RustUnnamed_1 = 1;
pub const METIS_OPTION_PTYPE: C2RustUnnamed_0 = 0;
pub const METIS_ERROR_MEMORY: C2RustUnnamed = -3;
pub type C2RustUnnamed = libc::c_int;
pub const METIS_ERROR: C2RustUnnamed = -4;
pub const METIS_ERROR_INPUT: C2RustUnnamed = -2;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const METIS_OPTION_UBVEC: C2RustUnnamed_0 = 24;
pub const METIS_OPTION_GTYPE: C2RustUnnamed_0 = 23;
pub const METIS_OPTION_BALANCE: C2RustUnnamed_0 = 22;
pub const METIS_OPTION_NOOUTPUT: C2RustUnnamed_0 = 21;
pub const METIS_OPTION_NCOMMON: C2RustUnnamed_0 = 20;
pub const METIS_OPTION_TPWGTS: C2RustUnnamed_0 = 19;
pub const METIS_OPTION_HELP: C2RustUnnamed_0 = 18;
pub const METIS_OPTION_UFACTOR: C2RustUnnamed_0 = 16;
pub const METIS_OPTION_NSEPS: C2RustUnnamed_0 = 15;
pub const METIS_OPTION_PFACTOR: C2RustUnnamed_0 = 14;
pub const METIS_OPTION_CCORDER: C2RustUnnamed_0 = 13;
pub const METIS_OPTION_COMPRESS: C2RustUnnamed_0 = 12;
pub const METIS_OPTION_CONTIG: C2RustUnnamed_0 = 11;
pub const METIS_OPTION_MINCONN: C2RustUnnamed_0 = 10;
pub const METIS_OPTION_NO2HOP: C2RustUnnamed_0 = 9;
pub const METIS_OPTION_SEED: C2RustUnnamed_0 = 8;
pub const METIS_OPTION_NCUTS: C2RustUnnamed_0 = 7;
pub const METIS_OPTION_NITER: C2RustUnnamed_0 = 6;
pub const METIS_OPTION_DBGLVL: C2RustUnnamed_0 = 5;
pub const METIS_OPTION_RTYPE: C2RustUnnamed_0 = 4;
pub const METIS_OPTION_IPTYPE: C2RustUnnamed_0 = 3;
pub const METIS_OPTION_CTYPE: C2RustUnnamed_0 = 2;
pub const METIS_OPTION_OBJTYPE: C2RustUnnamed_0 = 1;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const METIS_PTYPE_RB: C2RustUnnamed_1 = 0;
#[no_mangle]
pub unsafe extern "C" fn METIS_PartMeshNodal(
    mut ne: *mut idx_t,
    mut nn: *mut idx_t,
    mut eptr: *mut idx_t,
    mut eind: *mut idx_t,
    mut vwgt: *mut idx_t,
    mut vsize: *mut idx_t,
    mut nparts: *mut idx_t,
    mut tpwgts: *mut real_t,
    mut options: *mut idx_t,
    mut objval: *mut idx_t,
    mut epart: *mut idx_t,
    mut npart: *mut idx_t,
) -> libc::c_int {
    let mut sigrval: libc::c_int = 0 as libc::c_int;
    let mut renumber: libc::c_int = 0 as libc::c_int;
    let mut ptype: libc::c_int = 0;
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut ncon: idx_t = 1 as libc::c_int;
    let mut pnumflag: idx_t = 0 as libc::c_int;
    let mut rstatus: libc::c_int = METIS_OK as libc::c_int;
    if gk_malloc_init() == 0 {
        return METIS_ERROR_MEMORY as libc::c_int;
    }
    gk_sigtrap();
    sigrval = _setjmp(
        (*gk_jbufs.as_mut_ptr().offset(gk_cur_jbufs as isize)).as_mut_ptr(),
    );
    if !(sigrval != 0 as libc::c_int) {
        renumber = if options.is_null()
            || *options.offset(METIS_OPTION_NUMBERING as libc::c_int as isize)
                == -(1 as libc::c_int)
        {
            0 as libc::c_int
        } else {
            *options.offset(METIS_OPTION_NUMBERING as libc::c_int as isize)
        };
        ptype = if options.is_null()
            || *options.offset(METIS_OPTION_PTYPE as libc::c_int as isize)
                == -(1 as libc::c_int)
        {
            METIS_PTYPE_KWAY as libc::c_int
        } else {
            *options.offset(METIS_OPTION_PTYPE as libc::c_int as isize)
        };
        if renumber != 0 {
            libmetis__ChangeMesh2CNumbering(*ne, eptr, eind);
            *options
                .offset(
                    METIS_OPTION_NUMBERING as libc::c_int as isize,
                ) = 0 as libc::c_int;
        }
        rstatus = METIS_MeshToNodal(
            ne,
            nn,
            eptr,
            eind,
            &mut pnumflag,
            &mut xadj,
            &mut adjncy,
        );
        if rstatus != METIS_OK as libc::c_int {
            raise(15 as libc::c_int);
        }
        if ptype == METIS_PTYPE_KWAY as libc::c_int {
            rstatus = METIS_PartGraphKway(
                nn,
                &mut ncon,
                xadj,
                adjncy,
                vwgt,
                vsize,
                0 as *mut idx_t,
                nparts,
                tpwgts,
                0 as *mut real_t,
                options,
                objval,
                npart,
            );
        } else {
            rstatus = METIS_PartGraphRecursive(
                nn,
                &mut ncon,
                xadj,
                adjncy,
                vwgt,
                vsize,
                0 as *mut idx_t,
                nparts,
                tpwgts,
                0 as *mut real_t,
                options,
                objval,
                npart,
            );
        }
        if rstatus != METIS_OK as libc::c_int {
            raise(15 as libc::c_int);
        }
        libmetis__InduceRowPartFromColumnPart(
            *ne,
            eptr,
            eind,
            epart,
            npart,
            *nparts,
            tpwgts,
        );
    }
    if renumber != 0 {
        libmetis__ChangeMesh2FNumbering2(*ne, *nn, eptr, eind, epart, npart);
        *options
            .offset(METIS_OPTION_NUMBERING as libc::c_int as isize) = 1 as libc::c_int;
    }
    METIS_Free(xadj as *mut libc::c_void);
    METIS_Free(adjncy as *mut libc::c_void);
    gk_siguntrap();
    gk_malloc_cleanup(0 as libc::c_int);
    return libmetis__metis_rcode(sigrval);
}
#[no_mangle]
pub unsafe extern "C" fn METIS_PartMeshDual(
    mut ne: *mut idx_t,
    mut nn: *mut idx_t,
    mut eptr: *mut idx_t,
    mut eind: *mut idx_t,
    mut vwgt: *mut idx_t,
    mut vsize: *mut idx_t,
    mut ncommon: *mut idx_t,
    mut nparts: *mut idx_t,
    mut tpwgts: *mut real_t,
    mut options: *mut idx_t,
    mut objval: *mut idx_t,
    mut epart: *mut idx_t,
    mut npart: *mut idx_t,
) -> libc::c_int {
    let mut sigrval: libc::c_int = 0 as libc::c_int;
    let mut renumber: libc::c_int = 0 as libc::c_int;
    let mut ptype: libc::c_int = 0;
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut xadj: *mut idx_t = 0 as *mut idx_t;
    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut nptr: *mut idx_t = 0 as *mut idx_t;
    let mut nind: *mut idx_t = 0 as *mut idx_t;
    let mut ncon: idx_t = 1 as libc::c_int;
    let mut pnumflag: idx_t = 0 as libc::c_int;
    let mut rstatus: libc::c_int = METIS_OK as libc::c_int;
    if gk_malloc_init() == 0 {
        return METIS_ERROR_MEMORY as libc::c_int;
    }
    gk_sigtrap();
    sigrval = _setjmp(
        (*gk_jbufs.as_mut_ptr().offset(gk_cur_jbufs as isize)).as_mut_ptr(),
    );
    if !(sigrval != 0 as libc::c_int) {
        renumber = if options.is_null()
            || *options.offset(METIS_OPTION_NUMBERING as libc::c_int as isize)
                == -(1 as libc::c_int)
        {
            0 as libc::c_int
        } else {
            *options.offset(METIS_OPTION_NUMBERING as libc::c_int as isize)
        };
        ptype = if options.is_null()
            || *options.offset(METIS_OPTION_PTYPE as libc::c_int as isize)
                == -(1 as libc::c_int)
        {
            METIS_PTYPE_KWAY as libc::c_int
        } else {
            *options.offset(METIS_OPTION_PTYPE as libc::c_int as isize)
        };
        if renumber != 0 {
            libmetis__ChangeMesh2CNumbering(*ne, eptr, eind);
            *options
                .offset(
                    METIS_OPTION_NUMBERING as libc::c_int as isize,
                ) = 0 as libc::c_int;
        }
        rstatus = METIS_MeshToDual(
            ne,
            nn,
            eptr,
            eind,
            ncommon,
            &mut pnumflag,
            &mut xadj,
            &mut adjncy,
        );
        if rstatus != METIS_OK as libc::c_int {
            raise(15 as libc::c_int);
        }
        if ptype == METIS_PTYPE_KWAY as libc::c_int {
            rstatus = METIS_PartGraphKway(
                ne,
                &mut ncon,
                xadj,
                adjncy,
                vwgt,
                vsize,
                0 as *mut idx_t,
                nparts,
                tpwgts,
                0 as *mut real_t,
                options,
                objval,
                epart,
            );
        } else {
            rstatus = METIS_PartGraphRecursive(
                ne,
                &mut ncon,
                xadj,
                adjncy,
                vwgt,
                vsize,
                0 as *mut idx_t,
                nparts,
                tpwgts,
                0 as *mut real_t,
                options,
                objval,
                epart,
            );
        }
        if rstatus != METIS_OK as libc::c_int {
            raise(15 as libc::c_int);
        }
        nptr = libmetis__ismalloc(
            (*nn + 1 as libc::c_int) as size_t,
            0 as libc::c_int,
            b"METIS_PartMeshDual: nptr\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        nind = libmetis__imalloc(
            *eptr.offset(*ne as isize) as size_t,
            b"METIS_PartMeshDual: nind\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        i = 0 as libc::c_int;
        while i < *ne {
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
        while i < *nn {
            let ref mut fresh1 = *nptr.offset(i as isize);
            *fresh1 += *nptr.offset((i - 1 as libc::c_int) as isize);
            i += 1;
            i;
        }
        i = *nn;
        while i > 0 as libc::c_int {
            *nptr.offset(i as isize) = *nptr.offset((i - 1 as libc::c_int) as isize);
            i -= 1;
            i;
        }
        *nptr.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < *ne {
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
        i = *nn;
        while i > 0 as libc::c_int {
            *nptr.offset(i as isize) = *nptr.offset((i - 1 as libc::c_int) as isize);
            i -= 1;
            i;
        }
        *nptr.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
        libmetis__InduceRowPartFromColumnPart(
            *nn,
            nptr,
            nind,
            npart,
            epart,
            *nparts,
            tpwgts,
        );
        gk_free(
            &mut nptr as *mut *mut idx_t as *mut *mut libc::c_void,
            &mut nind as *mut *mut idx_t,
            0 as *mut *mut libc::c_void,
        );
    }
    if renumber != 0 {
        libmetis__ChangeMesh2FNumbering2(*ne, *nn, eptr, eind, epart, npart);
        *options
            .offset(METIS_OPTION_NUMBERING as libc::c_int as isize) = 1 as libc::c_int;
    }
    METIS_Free(xadj as *mut libc::c_void);
    METIS_Free(adjncy as *mut libc::c_void);
    gk_siguntrap();
    gk_malloc_cleanup(0 as libc::c_int);
    return libmetis__metis_rcode(sigrval);
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__InduceRowPartFromColumnPart(
    mut nrows: idx_t,
    mut rowptr: *mut idx_t,
    mut rowind: *mut idx_t,
    mut rpart: *mut idx_t,
    mut cpart: *mut idx_t,
    mut nparts: idx_t,
    mut tpwgts: *mut real_t,
) {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut k: idx_t = 0;
    let mut me: idx_t = 0;
    let mut nnbrs: idx_t = 0;
    let mut pwgts: *mut idx_t = 0 as *mut idx_t;
    let mut nbrdom: *mut idx_t = 0 as *mut idx_t;
    let mut nbrwgt: *mut idx_t = 0 as *mut idx_t;
    let mut nbrmrk: *mut idx_t = 0 as *mut idx_t;
    let mut itpwgts: *mut idx_t = 0 as *mut idx_t;
    pwgts = libmetis__ismalloc(
        nparts as size_t,
        0 as libc::c_int,
        b"InduceRowPartFromColumnPart: pwgts\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    nbrdom = libmetis__ismalloc(
        nparts as size_t,
        0 as libc::c_int,
        b"InduceRowPartFromColumnPart: nbrdom\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    nbrwgt = libmetis__ismalloc(
        nparts as size_t,
        0 as libc::c_int,
        b"InduceRowPartFromColumnPart: nbrwgt\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    nbrmrk = libmetis__ismalloc(
        nparts as size_t,
        -(1 as libc::c_int),
        b"InduceRowPartFromColumnPart: nbrmrk\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    libmetis__iset(nrows as size_t, -(1 as libc::c_int), rpart);
    itpwgts = libmetis__imalloc(
        nparts as size_t,
        b"InduceRowPartFromColumnPart: itpwgts\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    if tpwgts.is_null() {
        libmetis__iset(nparts as size_t, 1 as libc::c_int + nrows / nparts, itpwgts);
    } else {
        i = 0 as libc::c_int;
        while i < nparts {
            *itpwgts
                .offset(
                    i as isize,
                ) = (1 as libc::c_int as libc::c_float
                + nrows as libc::c_float * *tpwgts.offset(i as isize)) as idx_t;
            i += 1;
            i;
        }
    }
    i = 0 as libc::c_int;
    while i < nrows {
        if *rowptr.offset((i + 1 as libc::c_int) as isize) - *rowptr.offset(i as isize)
            == 0 as libc::c_int
        {
            *rpart.offset(i as isize) = -(2 as libc::c_int);
        } else {
            me = *cpart
                .offset(*rowind.offset(*rowptr.offset(i as isize) as isize) as isize);
            j = *rowptr.offset(i as isize) + 1 as libc::c_int;
            while j < *rowptr.offset((i + 1 as libc::c_int) as isize) {
                if *cpart.offset(*rowind.offset(j as isize) as isize) != me {
                    break;
                }
                j += 1;
                j;
            }
            if j == *rowptr.offset((i + 1 as libc::c_int) as isize) {
                *rpart.offset(i as isize) = me;
                let ref mut fresh4 = *pwgts.offset(me as isize);
                *fresh4 += 1;
                *fresh4;
            }
        }
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < nrows {
        if *rpart.offset(i as isize) == -(1 as libc::c_int) {
            nnbrs = 0 as libc::c_int;
            j = *rowptr.offset(i as isize);
            while j < *rowptr.offset((i + 1 as libc::c_int) as isize) {
                me = *cpart.offset(*rowind.offset(j as isize) as isize);
                if *nbrmrk.offset(me as isize) == -(1 as libc::c_int) {
                    *nbrdom.offset(nnbrs as isize) = me;
                    *nbrwgt.offset(nnbrs as isize) = 1 as libc::c_int;
                    let fresh5 = nnbrs;
                    nnbrs = nnbrs + 1;
                    *nbrmrk.offset(me as isize) = fresh5;
                } else {
                    let ref mut fresh6 = *nbrwgt
                        .offset(*nbrmrk.offset(me as isize) as isize);
                    *fresh6 += 1;
                    *fresh6;
                }
                j += 1;
                j;
            }
            *rpart
                .offset(
                    i as isize,
                ) = *nbrdom.offset(libmetis__iargmax(nnbrs as size_t, nbrwgt) as isize);
            if *pwgts.offset(*rpart.offset(i as isize) as isize)
                > *itpwgts.offset(*rpart.offset(i as isize) as isize)
            {
                j = 0 as libc::c_int;
                while j < nnbrs {
                    if *pwgts.offset(*nbrdom.offset(j as isize) as isize)
                        < *itpwgts.offset(*nbrdom.offset(j as isize) as isize)
                        || *pwgts.offset(*nbrdom.offset(j as isize) as isize)
                            - *itpwgts.offset(*nbrdom.offset(j as isize) as isize)
                            < *pwgts.offset(*rpart.offset(i as isize) as isize)
                                - *itpwgts.offset(*rpart.offset(i as isize) as isize)
                    {
                        *rpart.offset(i as isize) = *nbrdom.offset(j as isize);
                        break;
                    } else {
                        j += 1;
                        j;
                    }
                }
            }
            let ref mut fresh7 = *pwgts.offset(*rpart.offset(i as isize) as isize);
            *fresh7 += 1;
            *fresh7;
            j = 0 as libc::c_int;
            while j < nnbrs {
                *nbrmrk
                    .offset(*nbrdom.offset(j as isize) as isize) = -(1 as libc::c_int);
                j += 1;
                j;
            }
        }
        i += 1;
        i;
    }
    gk_free(
        &mut pwgts as *mut *mut idx_t as *mut *mut libc::c_void,
        &mut nbrdom as *mut *mut idx_t,
        &mut nbrwgt as *mut *mut idx_t,
        &mut nbrmrk as *mut *mut idx_t,
        &mut itpwgts as *mut *mut idx_t,
        0 as *mut *mut libc::c_void,
    );
}
