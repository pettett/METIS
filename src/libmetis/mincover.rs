use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn gk_free(ptr1: *mut *mut libc::c_void, _: ...);
    fn libmetis__imalloc(n: size_t, msg: *mut libc::c_char) -> *mut idx_t;
    fn libmetis__ismalloc(n: size_t, ival: idx_t, msg: *mut libc::c_char) -> *mut idx_t;
    fn abs(_: libc::c_int) -> libc::c_int;
}
pub type __int32_t = libc::c_int;
pub type int32_t = __int32_t;
pub type size_t = libc::c_ulong;
pub type idx_t = int32_t;
#[no_mangle]
pub unsafe extern "C" fn libmetis__MinCover(
    mut xadj: *mut idx_t,
    mut adjncy: *mut idx_t,
    mut asize: idx_t,
    mut bsize: idx_t,
    mut cover: *mut idx_t,
    mut csize: *mut idx_t,
) {
    let mut i: idx_t = 0;
    let mut j: idx_t = 0;
    let mut mate: *mut idx_t = 0 as *mut idx_t;
    let mut queue: *mut idx_t = 0 as *mut idx_t;
    let mut flag: *mut idx_t = 0 as *mut idx_t;
    let mut level: *mut idx_t = 0 as *mut idx_t;
    let mut lst: *mut idx_t = 0 as *mut idx_t;
    let mut fptr: idx_t = 0;
    let mut rptr: idx_t = 0;
    let mut lstptr: idx_t = 0;
    let mut row: idx_t = 0;
    let mut maxlevel: idx_t = 0;
    let mut col: idx_t = 0;
    mate = libmetis__ismalloc(
        bsize as size_t,
        -(1 as libc::c_int),
        b"MinCover: mate\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    flag = libmetis__imalloc(
        bsize as size_t,
        b"MinCover: flag\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    level = libmetis__imalloc(
        bsize as size_t,
        b"MinCover: level\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    queue = libmetis__imalloc(
        bsize as size_t,
        b"MinCover: queue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    lst = libmetis__imalloc(
        bsize as size_t,
        b"MinCover: lst\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < asize {
        j = *xadj.offset(i as isize);
        while j < *xadj.offset((i + 1 as libc::c_int) as isize) {
            if *mate.offset(*adjncy.offset(j as isize) as isize) == -(1 as libc::c_int) {
                *mate.offset(i as isize) = *adjncy.offset(j as isize);
                *mate.offset(*adjncy.offset(j as isize) as isize) = i;
                break;
            } else {
                j += 1;
                j;
            }
        }
        i += 1;
        i;
    }
    loop {
        rptr = 0 as libc::c_int;
        fptr = rptr;
        lstptr = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < bsize {
            *level.offset(i as isize) = -(1 as libc::c_int);
            *flag.offset(i as isize) = 0 as libc::c_int;
            i += 1;
            i;
        }
        maxlevel = bsize;
        i = 0 as libc::c_int;
        while i < asize {
            if *mate.offset(i as isize) == -(1 as libc::c_int) {
                let fresh0 = rptr;
                rptr = rptr + 1;
                *queue.offset(fresh0 as isize) = i;
                *level.offset(i as isize) = 0 as libc::c_int;
            }
            i += 1;
            i;
        }
        while fptr != rptr {
            let fresh1 = fptr;
            fptr = fptr + 1;
            row = *queue.offset(fresh1 as isize);
            if *level.offset(row as isize) < maxlevel {
                *flag.offset(row as isize) = 1 as libc::c_int;
                j = *xadj.offset(row as isize);
                while j < *xadj.offset((row + 1 as libc::c_int) as isize) {
                    col = *adjncy.offset(j as isize);
                    if *flag.offset(col as isize) == 0 {
                        *flag.offset(col as isize) = 1 as libc::c_int;
                        if *mate.offset(col as isize) == -(1 as libc::c_int) {
                            maxlevel = *level.offset(row as isize);
                            let fresh2 = lstptr;
                            lstptr = lstptr + 1;
                            *lst.offset(fresh2 as isize) = col;
                        } else {
                            if *flag.offset(*mate.offset(col as isize) as isize) != 0 {
                                printf(
                                    b"\nSomething wrong, flag[%d] is 1\0" as *const u8
                                        as *const libc::c_char,
                                    *mate.offset(col as isize),
                                );
                            }
                            let fresh3 = rptr;
                            rptr = rptr + 1;
                            *queue.offset(fresh3 as isize) = *mate.offset(col as isize);
                            *level
                                .offset(
                                    *mate.offset(col as isize) as isize,
                                ) = *level.offset(row as isize) + 1 as libc::c_int;
                        }
                    }
                    j += 1;
                    j;
                }
            }
        }
        if lstptr == 0 as libc::c_int {
            break;
        }
        i = 0 as libc::c_int;
        while i < lstptr {
            libmetis__MinCover_Augment(
                xadj,
                adjncy,
                *lst.offset(i as isize),
                mate,
                flag,
                level,
                maxlevel,
            );
            i += 1;
            i;
        }
    }
    libmetis__MinCover_Decompose(xadj, adjncy, asize, bsize, mate, cover, csize);
    gk_free(
        &mut mate as *mut *mut idx_t as *mut *mut libc::c_void,
        &mut flag as *mut *mut idx_t,
        &mut level as *mut *mut idx_t,
        &mut queue as *mut *mut idx_t,
        &mut lst as *mut *mut idx_t,
        0 as *mut *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__MinCover_Augment(
    mut xadj: *mut idx_t,
    mut adjncy: *mut idx_t,
    mut col: idx_t,
    mut mate: *mut idx_t,
    mut flag: *mut idx_t,
    mut level: *mut idx_t,
    mut maxlevel: idx_t,
) -> idx_t {
    let mut i: idx_t = 0;
    let mut row: idx_t = -(1 as libc::c_int);
    let mut status: idx_t = 0;
    *flag.offset(col as isize) = 2 as libc::c_int;
    i = *xadj.offset(col as isize);
    while i < *xadj.offset((col + 1 as libc::c_int) as isize) {
        row = *adjncy.offset(i as isize);
        if *flag.offset(row as isize) == 1 as libc::c_int {
            if *level.offset(row as isize) == maxlevel {
                *flag.offset(row as isize) = 2 as libc::c_int;
                if maxlevel != 0 as libc::c_int {
                    status = libmetis__MinCover_Augment(
                        xadj,
                        adjncy,
                        *mate.offset(row as isize),
                        mate,
                        flag,
                        level,
                        maxlevel - 1 as libc::c_int,
                    );
                } else {
                    status = 1 as libc::c_int;
                }
                if status != 0 {
                    *mate.offset(col as isize) = row;
                    *mate.offset(row as isize) = col;
                    return 1 as libc::c_int;
                }
            }
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__MinCover_Decompose(
    mut xadj: *mut idx_t,
    mut adjncy: *mut idx_t,
    mut asize: idx_t,
    mut bsize: idx_t,
    mut mate: *mut idx_t,
    mut cover: *mut idx_t,
    mut csize: *mut idx_t,
) {
    let mut i: idx_t = 0;
    let mut k: idx_t = 0;
    let mut where_0: *mut idx_t = 0 as *mut idx_t;
    let mut card: [idx_t; 10] = [0; 10];
    where_0 = libmetis__imalloc(
        bsize as size_t,
        b"MinCover_Decompose: where\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        card[i as usize] = 0 as libc::c_int;
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < asize {
        *where_0.offset(i as isize) = 2 as libc::c_int;
        i += 1;
        i;
    }
    while i < bsize {
        *where_0.offset(i as isize) = 5 as libc::c_int;
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < asize {
        if *mate.offset(i as isize) == -(1 as libc::c_int) {
            libmetis__MinCover_ColDFS(xadj, adjncy, i, mate, where_0, 10 as libc::c_int);
        }
        i += 1;
        i;
    }
    while i < bsize {
        if *mate.offset(i as isize) == -(1 as libc::c_int) {
            libmetis__MinCover_RowDFS(xadj, adjncy, i, mate, where_0, 20 as libc::c_int);
        }
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < bsize {
        card[*where_0.offset(i as isize) as usize] += 1;
        card[*where_0.offset(i as isize) as usize];
        i += 1;
        i;
    }
    k = 0 as libc::c_int;
    if abs(
        card[1 as libc::c_int as usize] + card[2 as libc::c_int as usize]
            - card[6 as libc::c_int as usize],
    )
        < abs(
            card[1 as libc::c_int as usize] - card[5 as libc::c_int as usize]
                - card[6 as libc::c_int as usize],
        )
    {
        i = 0 as libc::c_int;
        while i < bsize {
            if *where_0.offset(i as isize) == 1 as libc::c_int
                || *where_0.offset(i as isize) == 2 as libc::c_int
                || *where_0.offset(i as isize) == 6 as libc::c_int
            {
                let fresh4 = k;
                k = k + 1;
                *cover.offset(fresh4 as isize) = i;
            }
            i += 1;
            i;
        }
    } else {
        i = 0 as libc::c_int;
        while i < bsize {
            if *where_0.offset(i as isize) == 1 as libc::c_int
                || *where_0.offset(i as isize) == 5 as libc::c_int
                || *where_0.offset(i as isize) == 6 as libc::c_int
            {
                let fresh5 = k;
                k = k + 1;
                *cover.offset(fresh5 as isize) = i;
            }
            i += 1;
            i;
        }
    }
    *csize = k;
    gk_free(
        &mut where_0 as *mut *mut idx_t as *mut *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__MinCover_ColDFS(
    mut xadj: *mut idx_t,
    mut adjncy: *mut idx_t,
    mut root: idx_t,
    mut mate: *mut idx_t,
    mut where_0: *mut idx_t,
    mut flag: idx_t,
) {
    let mut i: idx_t = 0;
    if flag == 10 as libc::c_int {
        if *where_0.offset(root as isize) == 3 as libc::c_int {
            return;
        }
        *where_0.offset(root as isize) = 3 as libc::c_int;
        i = *xadj.offset(root as isize);
        while i < *xadj.offset((root + 1 as libc::c_int) as isize) {
            libmetis__MinCover_ColDFS(
                xadj,
                adjncy,
                *adjncy.offset(i as isize),
                mate,
                where_0,
                20 as libc::c_int,
            );
            i += 1;
            i;
        }
    } else {
        if *where_0.offset(root as isize) == 6 as libc::c_int {
            return;
        }
        *where_0.offset(root as isize) = 6 as libc::c_int;
        if *mate.offset(root as isize) != -(1 as libc::c_int) {
            libmetis__MinCover_ColDFS(
                xadj,
                adjncy,
                *mate.offset(root as isize),
                mate,
                where_0,
                10 as libc::c_int,
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__MinCover_RowDFS(
    mut xadj: *mut idx_t,
    mut adjncy: *mut idx_t,
    mut root: idx_t,
    mut mate: *mut idx_t,
    mut where_0: *mut idx_t,
    mut flag: idx_t,
) {
    let mut i: idx_t = 0;
    if flag == 20 as libc::c_int {
        if *where_0.offset(root as isize) == 4 as libc::c_int {
            return;
        }
        *where_0.offset(root as isize) = 4 as libc::c_int;
        i = *xadj.offset(root as isize);
        while i < *xadj.offset((root + 1 as libc::c_int) as isize) {
            libmetis__MinCover_RowDFS(
                xadj,
                adjncy,
                *adjncy.offset(i as isize),
                mate,
                where_0,
                10 as libc::c_int,
            );
            i += 1;
            i;
        }
    } else {
        if *where_0.offset(root as isize) == 1 as libc::c_int {
            return;
        }
        *where_0.offset(root as isize) = 1 as libc::c_int;
        if *mate.offset(root as isize) != -(1 as libc::c_int) {
            libmetis__MinCover_RowDFS(
                xadj,
                adjncy,
                *mate.offset(root as isize),
                mate,
                where_0,
                20 as libc::c_int,
            );
        }
    };
}
