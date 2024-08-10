use ::libc;
pub type __int32_t = libc::c_int;
pub type int32_t = __int32_t;
pub type idx_t = int32_t;
#[no_mangle]
pub unsafe extern "C" fn libmetis__Change2CNumbering(
    mut nvtxs: idx_t,
    mut xadj: *mut idx_t,
    mut adjncy: *mut idx_t,
) {
    let mut i: idx_t = 0;
    i = 0 as libc::c_int;
    while i <= nvtxs {
        let ref mut fresh0 = *xadj.offset(i as isize);
        *fresh0 -= 1;
        *fresh0;
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < *xadj.offset(nvtxs as isize) {
        let ref mut fresh1 = *adjncy.offset(i as isize);
        *fresh1 -= 1;
        *fresh1;
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__Change2FNumbering(
    mut nvtxs: idx_t,
    mut xadj: *mut idx_t,
    mut adjncy: *mut idx_t,
    mut vector: *mut idx_t,
) {
    let mut i: idx_t = 0;
    i = 0 as libc::c_int;
    while i < nvtxs {
        let ref mut fresh2 = *vector.offset(i as isize);
        *fresh2 += 1;
        *fresh2;
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < *xadj.offset(nvtxs as isize) {
        let ref mut fresh3 = *adjncy.offset(i as isize);
        *fresh3 += 1;
        *fresh3;
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i <= nvtxs {
        let ref mut fresh4 = *xadj.offset(i as isize);
        *fresh4 += 1;
        *fresh4;
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__Change2FNumbering2(
    mut nvtxs: idx_t,
    mut xadj: *mut idx_t,
    mut adjncy: *mut idx_t,
) {
    let mut i: idx_t = 0;
    let mut nedges: idx_t = 0;
    nedges = *xadj.offset(nvtxs as isize);
    i = 0 as libc::c_int;
    while i < nedges {
        let ref mut fresh5 = *adjncy.offset(i as isize);
        *fresh5 += 1;
        *fresh5;
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i <= nvtxs {
        let ref mut fresh6 = *xadj.offset(i as isize);
        *fresh6 += 1;
        *fresh6;
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__Change2FNumberingOrder(
    mut nvtxs: idx_t,
    mut xadj: *mut idx_t,
    mut adjncy: *mut idx_t,
    mut v1: *mut idx_t,
    mut v2: *mut idx_t,
) {
    let mut i: idx_t = 0;
    let mut nedges: idx_t = 0;
    i = 0 as libc::c_int;
    while i < nvtxs {
        let ref mut fresh7 = *v1.offset(i as isize);
        *fresh7 += 1;
        *fresh7;
        let ref mut fresh8 = *v2.offset(i as isize);
        *fresh8 += 1;
        *fresh8;
        i += 1;
        i;
    }
    nedges = *xadj.offset(nvtxs as isize);
    i = 0 as libc::c_int;
    while i < nedges {
        let ref mut fresh9 = *adjncy.offset(i as isize);
        *fresh9 += 1;
        *fresh9;
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i <= nvtxs {
        let ref mut fresh10 = *xadj.offset(i as isize);
        *fresh10 += 1;
        *fresh10;
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__ChangeMesh2CNumbering(
    mut n: idx_t,
    mut ptr: *mut idx_t,
    mut ind: *mut idx_t,
) {
    let mut i: idx_t = 0;
    i = 0 as libc::c_int;
    while i <= n {
        let ref mut fresh11 = *ptr.offset(i as isize);
        *fresh11 -= 1;
        *fresh11;
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < *ptr.offset(n as isize) {
        let ref mut fresh12 = *ind.offset(i as isize);
        *fresh12 -= 1;
        *fresh12;
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__ChangeMesh2FNumbering(
    mut n: idx_t,
    mut ptr: *mut idx_t,
    mut ind: *mut idx_t,
    mut nvtxs: idx_t,
    mut xadj: *mut idx_t,
    mut adjncy: *mut idx_t,
) {
    let mut i: idx_t = 0;
    i = 0 as libc::c_int;
    while i < *ptr.offset(n as isize) {
        let ref mut fresh13 = *ind.offset(i as isize);
        *fresh13 += 1;
        *fresh13;
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i <= n {
        let ref mut fresh14 = *ptr.offset(i as isize);
        *fresh14 += 1;
        *fresh14;
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < *xadj.offset(nvtxs as isize) {
        let ref mut fresh15 = *adjncy.offset(i as isize);
        *fresh15 += 1;
        *fresh15;
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i <= nvtxs {
        let ref mut fresh16 = *xadj.offset(i as isize);
        *fresh16 += 1;
        *fresh16;
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__ChangeMesh2FNumbering2(
    mut ne: idx_t,
    mut nn: idx_t,
    mut ptr: *mut idx_t,
    mut ind: *mut idx_t,
    mut epart: *mut idx_t,
    mut npart: *mut idx_t,
) {
    let mut i: idx_t = 0;
    i = 0 as libc::c_int;
    while i < *ptr.offset(ne as isize) {
        let ref mut fresh17 = *ind.offset(i as isize);
        *fresh17 += 1;
        *fresh17;
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i <= ne {
        let ref mut fresh18 = *ptr.offset(i as isize);
        *fresh18 += 1;
        *fresh18;
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < ne {
        let ref mut fresh19 = *epart.offset(i as isize);
        *fresh19 += 1;
        *fresh19;
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < nn {
        let ref mut fresh20 = *npart.offset(i as isize);
        *fresh20 += 1;
        *fresh20;
        i += 1;
        i;
    }
}
