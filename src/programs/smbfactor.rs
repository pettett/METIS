use std::slice;

use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn gk_free(ptr1: *mut *mut libc::c_void, _: ...);
    fn errexit(_: *mut libc::c_char, _: ...);
    fn libmetis__imalloc(n: size_t, msg: *mut libc::c_char) -> *mut idx_t;
    fn libmetis__ismalloc(n: size_t, ival: idx_t, msg: *mut libc::c_char) -> *mut idx_t;
}
pub type __int32_t = libc::c_int;
pub type int32_t = __int32_t;
pub type size_t = u64;
pub type idx_t = int32_t;
pub type real_t = libc::c_float;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ckrinfo_t {
    pub id: idx_t,
    pub ed: idx_t,
    pub nnbrs: idx_t,
    pub inbr: idx_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vkrinfo_t {
    pub nid: idx_t,
    pub ned: idx_t,
    pub gv: idx_t,
    pub nnbrs: idx_t,
    pub inbr: idx_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nrinfo_t {
    pub edegrees: [idx_t; 2],
}

use crate::libmetis::structure::*;

#[no_mangle]
pub unsafe extern "C" fn ComputeFillIn(
    mut graph: *mut graph_t,
    mut perm: *mut idx_t,
    mut iperm: *mut idx_t,
    mut r_maxlnz: *mut size_t,
    mut r_opc: *mut size_t,
) {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut k: idx_t = 0;
    let mut nvtxs: idx_t = 0;
    let mut maxlnz: idx_t = 0;
    let mut maxsub: idx_t = 0;

    let mut adjncy: *mut idx_t = 0 as *mut idx_t;
    let mut xlnz: *mut idx_t = 0 as *mut idx_t;
    let mut xnzsub: *mut idx_t = 0 as *mut idx_t;
    let mut nzsub: *mut idx_t = 0 as *mut idx_t;
    let mut opc: size_t = 0;
    nvtxs = (*graph).nvtxs;
    let mut xadj = &mut (*graph).xadj;
    adjncy = (*graph).adjncy;
    maxsub = 8 as libc::c_int * (nvtxs + xadj[nvtxs as usize]);
    i = 0 as libc::c_int;
    while i < xadj[nvtxs as usize] {
        let ref mut fresh0 = *adjncy.offset(i as isize);
        *fresh0 += 1;
        *fresh0;
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < nvtxs + 1 {
        let ref mut fresh1 = xadj[i as usize];
        *fresh1 += 1;
        *fresh1;
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < nvtxs {
        let ref mut fresh2 = *iperm.offset(i as isize);
        *fresh2 += 1;
        *fresh2;
        let ref mut fresh3 = *perm.offset(i as isize);
        *fresh3 += 1;
        *fresh3;
        i += 1;
        i;
    }
    xlnz = libmetis__imalloc(
        (nvtxs + 2 as libc::c_int) as size_t,
        b"ComputeFillIn: xlnz\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    xnzsub = libmetis__imalloc(
        (nvtxs + 2 as libc::c_int) as size_t,
        b"ComputeFillIn: xnzsub\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    nzsub = libmetis__imalloc(
        (maxsub + 1) as size_t,
        b"ComputeFillIn: nzsub\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if smbfct(
        nvtxs,
        xadj,
        adjncy,
        perm,
        iperm,
        xlnz,
        &mut maxlnz,
        xnzsub,
        nzsub,
        &mut maxsub,
    ) != 0
    {
        printf(b"Realocating nzsub...\n\0" as *const u8 as *const libc::c_char);
        gk_free(
            &mut nzsub as *mut *mut idx_t as *mut *mut libc::c_void,
            0 as *mut *mut libc::c_void,
        );
        maxsub *= 2 as libc::c_int;
        nzsub = libmetis__imalloc(
            (maxsub + 1) as size_t,
            b"ComputeFillIn: nzsub\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if smbfct(
            nvtxs,
            xadj,
            adjncy,
            perm,
            iperm,
            xlnz,
            &mut maxlnz,
            xnzsub,
            nzsub,
            &mut maxsub,
        ) != 0
        {
            errexit(
                b"MAXSUB is too small!\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
    }
    i = 0 as libc::c_int;
    while i < nvtxs {
        let ref mut fresh4 = *xlnz.offset(i as isize);
        *fresh4 -= 1;
        *fresh4;
        i += 1;
        i;
    }
    opc = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int;
    while i < nvtxs {
        opc = (opc as u64).wrapping_add(
            ((*xlnz.offset((i + 1) as isize) - *xlnz.offset(i as isize))
                * (*xlnz.offset((i + 1) as isize) - *xlnz.offset(i as isize))
                - (*xlnz.offset((i + 1) as isize) - *xlnz.offset(i as isize))) as u64,
        ) as size_t as size_t;
        i += 1;
        i;
    }
    *r_maxlnz = maxlnz as size_t;
    *r_opc = opc;
    gk_free(
        &mut xlnz as *mut *mut idx_t as *mut *mut libc::c_void,
        &mut xnzsub as *mut *mut idx_t,
        &mut nzsub as *mut *mut idx_t,
        0 as *mut *mut libc::c_void,
    );
    i = 0 as libc::c_int;
    while i < nvtxs {
        let ref mut fresh5 = *iperm.offset(i as isize);
        *fresh5 -= 1;
        *fresh5;
        let ref mut fresh6 = *perm.offset(i as isize);
        *fresh6 -= 1;
        *fresh6;
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < nvtxs + 1 {
        let ref mut fresh7 = xadj[i as usize];
        *fresh7 -= 1;
        *fresh7;
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < xadj[nvtxs as usize] {
        let ref mut fresh8 = *adjncy.offset(i as isize);
        *fresh8 -= 1;
        *fresh8;
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn smbfct(
    mut neqns: idx_t,
    mut xadj: &mut [idx_t],
    mut adjncy: *mut idx_t,
    mut perm: *mut idx_t,
    mut invp: *mut idx_t,
    mut xlnz: *mut idx_t,
    mut maxlnz: *mut idx_t,
    mut xnzsub: *mut idx_t,
    mut nzsub: *mut idx_t,
    mut maxsub: *mut idx_t,
) -> idx_t {
    let mut current_block: u64;
    let mut node: idx_t = 0;
    let mut rchm: idx_t = 0;
    let mut mrgk: idx_t = 0;
    let mut lmax: idx_t = 0;
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut k: idx_t = 0;
    let mut m: idx_t = 0;
    let mut nabor: idx_t = 0;
    let mut nzbeg: idx_t = 0;
    let mut nzend: idx_t = 0;
    let mut kxsub: idx_t = 0;
    let mut jstop: idx_t = 0;
    let mut jstrt: idx_t = 0;
    let mut mrkflg: idx_t = 0;
    let mut inz: idx_t = 0;
    let mut knz: idx_t = 0;
    let mut flag: idx_t = 0;
    let mut mrglnk: *mut idx_t = 0 as *mut idx_t;
    let mut marker: *mut idx_t = 0 as *mut idx_t;
    let mut rchlnk: *mut idx_t = 0 as *mut idx_t;
    rchlnk = libmetis__ismalloc(
        (neqns + 1) as size_t,
        0 as libc::c_int,
        b"smbfct: rchlnk\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    marker = libmetis__ismalloc(
        (neqns + 1) as size_t,
        0 as libc::c_int,
        b"smbfct: marker\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    mrglnk = libmetis__ismalloc(
        (neqns + 1) as size_t,
        0 as libc::c_int,
        b"smbfct: mgrlnk\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    marker = marker.offset(-1);
    marker;
    mrglnk = mrglnk.offset(-1);
    mrglnk;
    rchlnk = rchlnk.offset(-1);
    rchlnk;
    nzsub = nzsub.offset(-1);
    nzsub;
    xnzsub = xnzsub.offset(-1);
    xnzsub;
    xlnz = xlnz.offset(-1);
    xlnz;
    invp = invp.offset(-1);
    invp;
    perm = perm.offset(-1);
    perm;
    adjncy = adjncy.offset(-1);
    adjncy;
    xadj = slice::from_raw_parts_mut(xadj.as_mut_ptr().offset(-1), xadj.len() + 1);

    flag = 0 as libc::c_int;
    nzbeg = 1;
    nzend = 0 as libc::c_int;
    *xlnz.offset(1 as isize) = 1;
    k = 1;
    while k <= neqns {
        *xnzsub.offset(k as isize) = nzend;
        node = *perm.offset(k as isize);
        knz = 0 as libc::c_int;
        mrgk = *mrglnk.offset(k as isize);
        mrkflg = 0 as libc::c_int;
        *marker.offset(k as isize) = k;
        if mrgk != 0 as libc::c_int {
            *marker.offset(k as isize) = *marker.offset(mrgk as isize);
        }
        if xadj[(node as usize)] >= xadj[((node + 1) as usize)] {
            *xlnz.offset((k + 1) as isize) = *xlnz.offset(k as isize);
        } else {
            *rchlnk.offset(k as isize) = neqns + 1;
            j = xadj[(node as usize)];
            while j < xadj[((node + 1) as usize)] {
                nabor = *invp.offset(*adjncy.offset(j as isize) as isize);
                if !(nabor <= k) {
                    rchm = k;
                    loop {
                        m = rchm;
                        rchm = *rchlnk.offset(m as isize);
                        if !(rchm <= nabor) {
                            break;
                        }
                    }
                    knz += 1;
                    knz;
                    *rchlnk.offset(m as isize) = nabor;
                    *rchlnk.offset(nabor as isize) = rchm;
                    if *marker.offset(nabor as isize) != *marker.offset(k as isize) {
                        mrkflg = 1;
                    }
                }
                j += 1;
                j;
            }
            lmax = 0 as libc::c_int;
            if mrkflg != 0 as libc::c_int
                || mrgk == 0 as libc::c_int
                || *mrglnk.offset(mrgk as isize) != 0 as libc::c_int
            {
                i = k;
                loop {
                    i = *mrglnk.offset(i as isize);
                    if !(i != 0 as libc::c_int) {
                        break;
                    }
                    inz = *xlnz.offset((i + 1) as isize) - (*xlnz.offset(i as isize) + 1);
                    jstrt = *xnzsub.offset(i as isize) + 1;
                    jstop = *xnzsub.offset(i as isize) + inz;
                    if inz > lmax {
                        lmax = inz;
                        *xnzsub.offset(k as isize) = jstrt;
                    }
                    rchm = k;
                    j = jstrt;
                    while j <= jstop {
                        nabor = *nzsub.offset(j as isize);
                        loop {
                            m = rchm;
                            rchm = *rchlnk.offset(m as isize);
                            if !(rchm < nabor) {
                                break;
                            }
                        }
                        if rchm != nabor {
                            knz += 1;
                            knz;
                            *rchlnk.offset(m as isize) = nabor;
                            *rchlnk.offset(nabor as isize) = rchm;
                            rchm = nabor;
                        }
                        j += 1;
                        j;
                    }
                }
                if !(knz == lmax) {
                    if nzbeg > nzend {
                        current_block = 3693981863110393028;
                    } else {
                        i = *rchlnk.offset(k as isize);
                        jstrt = nzbeg;
                        loop {
                            if !(jstrt <= nzend) {
                                current_block = 3693981863110393028;
                                break;
                            }
                            if *nzsub.offset(jstrt as isize) < i {
                                jstrt += 1;
                                jstrt;
                            } else if *nzsub.offset(jstrt as isize) == i {
                                current_block = 9703207952471510684;
                                break;
                            } else {
                                current_block = 3693981863110393028;
                                break;
                            }
                        }
                        match current_block {
                            3693981863110393028 => {}
                            _ => {
                                *xnzsub.offset(k as isize) = jstrt;
                                j = jstrt;
                                loop {
                                    if !(j <= nzend) {
                                        current_block = 1352918242886884122;
                                        break;
                                    }
                                    if *nzsub.offset(j as isize) != i {
                                        current_block = 3693981863110393028;
                                        break;
                                    }
                                    i = *rchlnk.offset(i as isize);
                                    if i > neqns {
                                        current_block = 11690585760747073978;
                                        break;
                                    }
                                    j += 1;
                                    j;
                                }
                                match current_block {
                                    11690585760747073978 => {}
                                    3693981863110393028 => {}
                                    _ => {
                                        nzend = jstrt - 1;
                                        current_block = 3693981863110393028;
                                    }
                                }
                            }
                        }
                    }
                    match current_block {
                        11690585760747073978 => {}
                        _ => {
                            nzbeg = nzend + 1;
                            nzend += knz;
                            if nzend >= *maxsub {
                                flag = 1;
                                break;
                            } else {
                                i = k;
                                j = nzbeg;
                                while j <= nzend {
                                    i = *rchlnk.offset(i as isize);
                                    *nzsub.offset(j as isize) = i;
                                    *marker.offset(i as isize) = k;
                                    j += 1;
                                    j;
                                }
                                *xnzsub.offset(k as isize) = nzbeg;
                                *marker.offset(k as isize) = k;
                            }
                        }
                    }
                }
            } else {
                *xnzsub.offset(k as isize) = *xnzsub.offset(mrgk as isize) + 1;
                knz = *xlnz.offset((mrgk + 1) as isize) - (*xlnz.offset(mrgk as isize) + 1);
            }
            if knz > 1 {
                kxsub = *xnzsub.offset(k as isize);
                i = *nzsub.offset(kxsub as isize);
                *mrglnk.offset(k as isize) = *mrglnk.offset(i as isize);
                *mrglnk.offset(i as isize) = k;
            }
            *xlnz.offset((k + 1) as isize) = *xlnz.offset(k as isize) + knz;
        }
        k += 1;
        k;
    }
    if flag == 0 as libc::c_int {
        *maxlnz = *xlnz.offset(neqns as isize) - 1;
        *maxsub = *xnzsub.offset(neqns as isize);
        *xnzsub.offset((neqns + 1) as isize) = *xnzsub.offset(neqns as isize);
    }
    marker = marker.offset(1);
    mrglnk = mrglnk.offset(1);
    rchlnk = rchlnk.offset(1);
    nzsub = nzsub.offset(1);
    xnzsub = xnzsub.offset(1);
    xlnz = xlnz.offset(1);
    invp = invp.offset(1);
    perm = perm.offset(1);
    adjncy = adjncy.offset(1);
    xadj = slice::from_raw_parts_mut(xadj.as_mut_ptr().offset(1), xadj.len() - 1);
    gk_free(
        &mut rchlnk as *mut *mut idx_t as *mut *mut libc::c_void,
        &mut mrglnk as *mut *mut idx_t,
        &mut marker as *mut *mut idx_t,
        0 as *mut *mut libc::c_void,
    );
    return flag;
}
