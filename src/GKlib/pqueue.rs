use ::libc;
extern "C" {
    fn gk_idxsmalloc(n: size_t, ival: gk_idx_t, msg: *mut libc::c_char) -> *mut gk_idx_t;
    fn gk_ikvmalloc(n: size_t, msg: *mut libc::c_char) -> *mut gk_ikv_t;
    fn gk_i32kvmalloc(n: size_t, msg: *mut libc::c_char) -> *mut gk_i32kv_t;
    fn gk_i64kvmalloc(n: size_t, msg: *mut libc::c_char) -> *mut gk_i64kv_t;
    fn gk_fkvmalloc(n: size_t, msg: *mut libc::c_char) -> *mut gk_fkv_t;
    fn gk_dkvmalloc(n: size_t, msg: *mut libc::c_char) -> *mut gk_dkv_t;
    fn gk_idxkvmalloc(n: size_t, msg: *mut libc::c_char) -> *mut gk_idxkv_t;
    fn gk_malloc(nbytes: size_t, msg: *mut libc::c_char) -> *mut libc::c_void;
    fn gk_free(ptr1: *mut *mut libc::c_void, _: ...);
}
pub type __int32_t = i32;
pub type __int64_t = i64;
pub type __ssize_t = i64;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type ssize_t = __ssize_t;
pub type size_t = u64;
pub type gk_idx_t = ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gk_ikv_t {
    pub key: libc::c_int,
    pub val: ssize_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gk_i32kv_t {
    pub key: int32_t,
    pub val: ssize_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gk_i64kv_t {
    pub key: int64_t,
    pub val: ssize_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gk_fkv_t {
    pub key: libc::c_float,
    pub val: ssize_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gk_dkv_t {
    pub key: libc::c_double,
    pub val: ssize_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gk_idxkv_t {
    pub key: gk_idx_t,
    pub val: gk_idx_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gk_ipq_t {
    pub nnodes: gk_idx_t,
    pub maxnodes: gk_idx_t,
    pub heap: *mut gk_ikv_t,
    pub locator: *mut gk_idx_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gk_i32pq_t {
    pub nnodes: gk_idx_t,
    pub maxnodes: gk_idx_t,
    pub heap: *mut gk_i32kv_t,
    pub locator: *mut gk_idx_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gk_i64pq_t {
    pub nnodes: gk_idx_t,
    pub maxnodes: gk_idx_t,
    pub heap: *mut gk_i64kv_t,
    pub locator: *mut gk_idx_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gk_fpq_t {
    pub nnodes: gk_idx_t,
    pub maxnodes: gk_idx_t,
    pub heap: *mut gk_fkv_t,
    pub locator: *mut gk_idx_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gk_dpq_t {
    pub nnodes: gk_idx_t,
    pub maxnodes: gk_idx_t,
    pub heap: *mut gk_dkv_t,
    pub locator: *mut gk_idx_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gk_idxpq_t {
    pub nnodes: gk_idx_t,
    pub maxnodes: gk_idx_t,
    pub heap: *mut gk_idxkv_t,
    pub locator: *mut gk_idx_t,
}
#[no_mangle]
pub unsafe extern "C" fn gk_ipqCheckHeap(mut queue: *mut gk_ipq_t) -> libc::c_int {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut nnodes: size_t = 0;
    let mut locator: *mut gk_idx_t = 0 as *mut gk_idx_t;
    let mut heap: *mut gk_ikv_t = 0 as *mut gk_ikv_t;
    heap = (*queue).heap;
    locator = (*queue).locator;
    nnodes = (*queue).nnodes as size_t;
    if nnodes == 0 as libc::c_int as u64 {
        return 1;
    }
    i = 1 as gk_idx_t;
    while (i as u64) < nnodes {
        i += 1;
        i;
    }
    i = 1 as gk_idx_t;
    while (i as u64) < nnodes {
        i += 1;
        i;
    }
    i = 0 as libc::c_int as gk_idx_t;
    j = i;
    while i < (*queue).maxnodes {
        if *locator.offset(i as isize) != -(1) as i64 {
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    return 1;
}
#[no_mangle]
pub unsafe extern "C" fn gk_ipqInit(mut queue: *mut gk_ipq_t, mut maxnodes: size_t) {
    (*queue).nnodes = 0 as libc::c_int as gk_idx_t;
    (*queue).maxnodes = maxnodes as gk_idx_t;
    (*queue).heap = gk_ikvmalloc(
        maxnodes,
        b"gk_PQInit: heap\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (*queue).locator = gk_idxsmalloc(
        maxnodes,
        -(1) as gk_idx_t,
        b"gk_PQInit: locator\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gk_ipqReset(mut queue: *mut gk_ipq_t) {
    let mut i: gk_idx_t = 0;
    let mut locator: *mut gk_idx_t = (*queue).locator;
    let mut heap: *mut gk_ikv_t = (*queue).heap;
    i = (*queue).nnodes - 1 as i64;
    while i >= 0 as libc::c_int as i64 {
        *locator.offset((*heap.offset(i as isize)).val as isize) = -(1) as gk_idx_t;
        i -= 1;
        i;
    }
    (*queue).nnodes = 0 as libc::c_int as gk_idx_t;
}
#[no_mangle]
pub unsafe extern "C" fn gk_ipqFree(mut queue: *mut gk_ipq_t) {
    if queue.is_null() {
        return;
    }
    gk_free(
        &mut (*queue).heap as *mut *mut gk_ikv_t as *mut *mut libc::c_void,
        &mut (*queue).locator as *mut *mut gk_idx_t,
        0 as *mut *mut libc::c_void,
    );
    (*queue).maxnodes = 0 as libc::c_int as gk_idx_t;
}
#[no_mangle]
pub unsafe extern "C" fn gk_ipqDestroy(mut queue: *mut gk_ipq_t) {
    if queue.is_null() {
        return;
    }
    gk_ipqFree(queue);
    gk_free(
        &mut queue as *mut *mut gk_ipq_t as *mut *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gk_ipqLength(mut queue: *mut gk_ipq_t) -> size_t {
    return (*queue).nnodes as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn gk_ipqInsert(
    mut queue: *mut gk_ipq_t,
    mut node: gk_idx_t,
    mut key: libc::c_int,
) -> libc::c_int {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut locator: *mut gk_idx_t = (*queue).locator;
    let mut heap: *mut gk_ikv_t = (*queue).heap;
    let fresh0 = (*queue).nnodes;
    (*queue).nnodes = (*queue).nnodes + 1;
    i = fresh0;
    while i > 0 as libc::c_int as i64 {
        j = i - 1 as i64 >> 1;
        if !(key > (*heap.offset(j as isize)).key) {
            break;
        }
        *heap.offset(i as isize) = *heap.offset(j as isize);
        *locator.offset((*heap.offset(i as isize)).val as isize) = i;
        i = j;
    }
    (*heap.offset(i as isize)).key = key;
    (*heap.offset(i as isize)).val = node;
    *locator.offset(node as isize) = i;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gk_ipqDelete(mut queue: *mut gk_ipq_t, mut node: gk_idx_t) -> libc::c_int {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut nnodes: gk_idx_t = 0;
    let mut newkey: libc::c_int = 0;
    let mut oldkey: libc::c_int = 0;
    let mut locator: *mut gk_idx_t = (*queue).locator;
    let mut heap: *mut gk_ikv_t = (*queue).heap;
    i = *locator.offset(node as isize);
    *locator.offset(node as isize) = -(1) as gk_idx_t;
    (*queue).nnodes -= 1;
    if (*queue).nnodes > 0 as libc::c_int as i64
        && (*heap.offset((*queue).nnodes as isize)).val != node
    {
        node = (*heap.offset((*queue).nnodes as isize)).val;
        newkey = (*heap.offset((*queue).nnodes as isize)).key;
        oldkey = (*heap.offset(i as isize)).key;
        if newkey > oldkey {
            while i > 0 as libc::c_int as i64 {
                j = i - 1 as i64 >> 1;
                if !(newkey > (*heap.offset(j as isize)).key) {
                    break;
                }
                *heap.offset(i as isize) = *heap.offset(j as isize);
                *locator.offset((*heap.offset(i as isize)).val as isize) = i;
                i = j;
            }
        } else {
            nnodes = (*queue).nnodes;
            loop {
                j = (i << 1) + 1 as i64;
                if !(j < nnodes) {
                    break;
                }
                if (*heap.offset(j as isize)).key > newkey {
                    if (j + 1 as i64) < nnodes
                        && (*heap.offset((j + 1 as i64) as isize)).key
                            > (*heap.offset(j as isize)).key
                    {
                        j += 1;
                        j;
                    }
                    *heap.offset(i as isize) = *heap.offset(j as isize);
                    *locator.offset((*heap.offset(i as isize)).val as isize) = i;
                    i = j;
                } else {
                    if !((j + 1 as i64) < nnodes
                        && (*heap.offset((j + 1 as i64) as isize)).key
                            > newkey)
                    {
                        break;
                    }
                    j += 1;
                    j;
                    *heap.offset(i as isize) = *heap.offset(j as isize);
                    *locator.offset((*heap.offset(i as isize)).val as isize) = i;
                    i = j;
                }
            }
        }
        (*heap.offset(i as isize)).key = newkey;
        (*heap.offset(i as isize)).val = node;
        *locator.offset(node as isize) = i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gk_ipqUpdate(
    mut queue: *mut gk_ipq_t,
    mut node: gk_idx_t,
    mut newkey: libc::c_int,
) {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut nnodes: gk_idx_t = 0;
    let mut oldkey: libc::c_int = 0;
    let mut locator: *mut gk_idx_t = (*queue).locator;
    let mut heap: *mut gk_ikv_t = (*queue).heap;
    oldkey = (*heap.offset(*locator.offset(node as isize) as isize)).key;
    i = *locator.offset(node as isize);
    if newkey > oldkey {
        while i > 0 as libc::c_int as i64 {
            j = i - 1 as i64 >> 1;
            if !(newkey > (*heap.offset(j as isize)).key) {
                break;
            }
            *heap.offset(i as isize) = *heap.offset(j as isize);
            *locator.offset((*heap.offset(i as isize)).val as isize) = i;
            i = j;
        }
    } else {
        nnodes = (*queue).nnodes;
        loop {
            j = (i << 1) + 1 as i64;
            if !(j < nnodes) {
                break;
            }
            if (*heap.offset(j as isize)).key > newkey {
                if (j + 1 as i64) < nnodes
                    && (*heap.offset((j + 1 as i64) as isize)).key
                        > (*heap.offset(j as isize)).key
                {
                    j += 1;
                    j;
                }
                *heap.offset(i as isize) = *heap.offset(j as isize);
                *locator.offset((*heap.offset(i as isize)).val as isize) = i;
                i = j;
            } else {
                if !((j + 1 as i64) < nnodes
                    && (*heap.offset((j + 1 as i64) as isize)).key > newkey)
                {
                    break;
                }
                j += 1;
                j;
                *heap.offset(i as isize) = *heap.offset(j as isize);
                *locator.offset((*heap.offset(i as isize)).val as isize) = i;
                i = j;
            }
        }
    }
    (*heap.offset(i as isize)).key = newkey;
    (*heap.offset(i as isize)).val = node;
    *locator.offset(node as isize) = i;
}
#[no_mangle]
pub unsafe extern "C" fn gk_ipqCreate(mut maxnodes: size_t) -> *mut gk_ipq_t {
    let mut queue: *mut gk_ipq_t = 0 as *mut gk_ipq_t;
    queue = gk_malloc(
        ::core::mem::size_of::<gk_ipq_t>() as u64,
        b"gk_pqCreate: queue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut gk_ipq_t;
    gk_ipqInit(queue, maxnodes);
    return queue;
}
#[no_mangle]
pub unsafe extern "C" fn gk_ipqSeeTopKey(mut queue: *mut gk_ipq_t) -> libc::c_int {
    return if (*queue).nnodes == 0 as libc::c_int as i64 {
        2147483647 as libc::c_int
    } else {
        (*((*queue).heap).offset(0 as libc::c_int as isize)).key
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_ipqSeeKey(mut queue: *mut gk_ipq_t, mut node: gk_idx_t) -> libc::c_int {
    let mut locator: *mut gk_idx_t = 0 as *mut gk_idx_t;
    let mut heap: *mut gk_ikv_t = 0 as *mut gk_ikv_t;
    heap = (*queue).heap;
    locator = (*queue).locator;
    return (*heap.offset(*locator.offset(node as isize) as isize)).key;
}
#[no_mangle]
pub unsafe extern "C" fn gk_ipqGetTop(mut queue: *mut gk_ipq_t) -> gk_idx_t {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut locator: *mut gk_idx_t = 0 as *mut gk_idx_t;
    let mut heap: *mut gk_ikv_t = 0 as *mut gk_ikv_t;
    let mut vtx: gk_idx_t = 0;
    let mut node: gk_idx_t = 0;
    let mut key: libc::c_int = 0;
    if (*queue).nnodes == 0 as libc::c_int as i64 {
        return -(1) as gk_idx_t;
    }
    (*queue).nnodes -= 1;
    (*queue).nnodes;
    heap = (*queue).heap;
    locator = (*queue).locator;
    vtx = (*heap.offset(0 as libc::c_int as isize)).val;
    *locator.offset(vtx as isize) = -(1) as gk_idx_t;
    i = (*queue).nnodes;
    if i > 0 as libc::c_int as i64 {
        key = (*heap.offset(i as isize)).key;
        node = (*heap.offset(i as isize)).val;
        i = 0 as libc::c_int as gk_idx_t;
        loop {
            j = 2 as libc::c_int as i64 * i + 1 as i64;
            if !(j < (*queue).nnodes) {
                break;
            }
            if (*heap.offset(j as isize)).key > key {
                if (j + 1 as i64) < (*queue).nnodes
                    && (*heap.offset((j + 1 as i64) as isize)).key
                        > (*heap.offset(j as isize)).key
                {
                    j = j + 1 as i64;
                }
                *heap.offset(i as isize) = *heap.offset(j as isize);
                *locator.offset((*heap.offset(i as isize)).val as isize) = i;
                i = j;
            } else {
                if !((j + 1 as i64) < (*queue).nnodes
                    && (*heap.offset((j + 1 as i64) as isize)).key > key)
                {
                    break;
                }
                j = j + 1 as i64;
                *heap.offset(i as isize) = *heap.offset(j as isize);
                *locator.offset((*heap.offset(i as isize)).val as isize) = i;
                i = j;
            }
        }
        (*heap.offset(i as isize)).key = key;
        (*heap.offset(i as isize)).val = node;
        *locator.offset(node as isize) = i;
    }
    return vtx;
}
#[no_mangle]
pub unsafe extern "C" fn gk_ipqSeeTopVal(mut queue: *mut gk_ipq_t) -> gk_idx_t {
    return if (*queue).nnodes == 0 as libc::c_int as i64 {
        -(1) as i64
    } else {
        (*((*queue).heap).offset(0 as libc::c_int as isize)).val
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_i32pqReset(mut queue: *mut gk_i32pq_t) {
    let mut i: gk_idx_t = 0;
    let mut locator: *mut gk_idx_t = (*queue).locator;
    let mut heap: *mut gk_i32kv_t = (*queue).heap;
    i = (*queue).nnodes - 1 as i64;
    while i >= 0 as libc::c_int as i64 {
        *locator.offset((*heap.offset(i as isize)).val as isize) = -(1) as gk_idx_t;
        i -= 1;
        i;
    }
    (*queue).nnodes = 0 as libc::c_int as gk_idx_t;
}
#[no_mangle]
pub unsafe extern "C" fn gk_i32pqInit(mut queue: *mut gk_i32pq_t, mut maxnodes: size_t) {
    (*queue).nnodes = 0 as libc::c_int as gk_idx_t;
    (*queue).maxnodes = maxnodes as gk_idx_t;
    (*queue).heap = gk_i32kvmalloc(
        maxnodes,
        b"gk_PQInit: heap\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (*queue).locator = gk_idxsmalloc(
        maxnodes,
        -(1) as gk_idx_t,
        b"gk_PQInit: locator\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gk_i32pqCheckHeap(mut queue: *mut gk_i32pq_t) -> libc::c_int {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut nnodes: size_t = 0;
    let mut locator: *mut gk_idx_t = 0 as *mut gk_idx_t;
    let mut heap: *mut gk_i32kv_t = 0 as *mut gk_i32kv_t;
    heap = (*queue).heap;
    locator = (*queue).locator;
    nnodes = (*queue).nnodes as size_t;
    if nnodes == 0 as libc::c_int as u64 {
        return 1;
    }
    i = 1 as gk_idx_t;
    while (i as u64) < nnodes {
        i += 1;
        i;
    }
    i = 1 as gk_idx_t;
    while (i as u64) < nnodes {
        i += 1;
        i;
    }
    i = 0 as libc::c_int as gk_idx_t;
    j = i;
    while i < (*queue).maxnodes {
        if *locator.offset(i as isize) != -(1) as i64 {
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    return 1;
}
#[no_mangle]
pub unsafe extern "C" fn gk_i32pqFree(mut queue: *mut gk_i32pq_t) {
    if queue.is_null() {
        return;
    }
    gk_free(
        &mut (*queue).heap as *mut *mut gk_i32kv_t as *mut *mut libc::c_void,
        &mut (*queue).locator as *mut *mut gk_idx_t,
        0 as *mut *mut libc::c_void,
    );
    (*queue).maxnodes = 0 as libc::c_int as gk_idx_t;
}
#[no_mangle]
pub unsafe extern "C" fn gk_i32pqDestroy(mut queue: *mut gk_i32pq_t) {
    if queue.is_null() {
        return;
    }
    gk_i32pqFree(queue);
    gk_free(
        &mut queue as *mut *mut gk_i32pq_t as *mut *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gk_i32pqLength(mut queue: *mut gk_i32pq_t) -> size_t {
    return (*queue).nnodes as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn gk_i32pqInsert(
    mut queue: *mut gk_i32pq_t,
    mut node: gk_idx_t,
    mut key: int32_t,
) -> libc::c_int {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut locator: *mut gk_idx_t = (*queue).locator;
    let mut heap: *mut gk_i32kv_t = (*queue).heap;
    let fresh1 = (*queue).nnodes;
    (*queue).nnodes = (*queue).nnodes + 1;
    i = fresh1;
    while i > 0 as libc::c_int as i64 {
        j = i - 1 as i64 >> 1;
        if !(key > (*heap.offset(j as isize)).key) {
            break;
        }
        *heap.offset(i as isize) = *heap.offset(j as isize);
        *locator.offset((*heap.offset(i as isize)).val as isize) = i;
        i = j;
    }
    (*heap.offset(i as isize)).key = key;
    (*heap.offset(i as isize)).val = node;
    *locator.offset(node as isize) = i;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gk_i32pqDelete(
    mut queue: *mut gk_i32pq_t,
    mut node: gk_idx_t,
) -> libc::c_int {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut nnodes: gk_idx_t = 0;
    let mut newkey: int32_t = 0;
    let mut oldkey: int32_t = 0;
    let mut locator: *mut gk_idx_t = (*queue).locator;
    let mut heap: *mut gk_i32kv_t = (*queue).heap;
    i = *locator.offset(node as isize);
    *locator.offset(node as isize) = -(1) as gk_idx_t;
    (*queue).nnodes -= 1;
    if (*queue).nnodes > 0 as libc::c_int as i64
        && (*heap.offset((*queue).nnodes as isize)).val != node
    {
        node = (*heap.offset((*queue).nnodes as isize)).val;
        newkey = (*heap.offset((*queue).nnodes as isize)).key;
        oldkey = (*heap.offset(i as isize)).key;
        if newkey > oldkey {
            while i > 0 as libc::c_int as i64 {
                j = i - 1 as i64 >> 1;
                if !(newkey > (*heap.offset(j as isize)).key) {
                    break;
                }
                *heap.offset(i as isize) = *heap.offset(j as isize);
                *locator.offset((*heap.offset(i as isize)).val as isize) = i;
                i = j;
            }
        } else {
            nnodes = (*queue).nnodes;
            loop {
                j = (i << 1) + 1 as i64;
                if !(j < nnodes) {
                    break;
                }
                if (*heap.offset(j as isize)).key > newkey {
                    if (j + 1 as i64) < nnodes
                        && (*heap.offset((j + 1 as i64) as isize)).key
                            > (*heap.offset(j as isize)).key
                    {
                        j += 1;
                        j;
                    }
                    *heap.offset(i as isize) = *heap.offset(j as isize);
                    *locator.offset((*heap.offset(i as isize)).val as isize) = i;
                    i = j;
                } else {
                    if !((j + 1 as i64) < nnodes
                        && (*heap.offset((j + 1 as i64) as isize)).key
                            > newkey)
                    {
                        break;
                    }
                    j += 1;
                    j;
                    *heap.offset(i as isize) = *heap.offset(j as isize);
                    *locator.offset((*heap.offset(i as isize)).val as isize) = i;
                    i = j;
                }
            }
        }
        (*heap.offset(i as isize)).key = newkey;
        (*heap.offset(i as isize)).val = node;
        *locator.offset(node as isize) = i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gk_i32pqUpdate(
    mut queue: *mut gk_i32pq_t,
    mut node: gk_idx_t,
    mut newkey: int32_t,
) {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut nnodes: gk_idx_t = 0;
    let mut oldkey: int32_t = 0;
    let mut locator: *mut gk_idx_t = (*queue).locator;
    let mut heap: *mut gk_i32kv_t = (*queue).heap;
    oldkey = (*heap.offset(*locator.offset(node as isize) as isize)).key;
    i = *locator.offset(node as isize);
    if newkey > oldkey {
        while i > 0 as libc::c_int as i64 {
            j = i - 1 as i64 >> 1;
            if !(newkey > (*heap.offset(j as isize)).key) {
                break;
            }
            *heap.offset(i as isize) = *heap.offset(j as isize);
            *locator.offset((*heap.offset(i as isize)).val as isize) = i;
            i = j;
        }
    } else {
        nnodes = (*queue).nnodes;
        loop {
            j = (i << 1) + 1 as i64;
            if !(j < nnodes) {
                break;
            }
            if (*heap.offset(j as isize)).key > newkey {
                if (j + 1 as i64) < nnodes
                    && (*heap.offset((j + 1 as i64) as isize)).key
                        > (*heap.offset(j as isize)).key
                {
                    j += 1;
                    j;
                }
                *heap.offset(i as isize) = *heap.offset(j as isize);
                *locator.offset((*heap.offset(i as isize)).val as isize) = i;
                i = j;
            } else {
                if !((j + 1 as i64) < nnodes
                    && (*heap.offset((j + 1 as i64) as isize)).key > newkey)
                {
                    break;
                }
                j += 1;
                j;
                *heap.offset(i as isize) = *heap.offset(j as isize);
                *locator.offset((*heap.offset(i as isize)).val as isize) = i;
                i = j;
            }
        }
    }
    (*heap.offset(i as isize)).key = newkey;
    (*heap.offset(i as isize)).val = node;
    *locator.offset(node as isize) = i;
}
#[no_mangle]
pub unsafe extern "C" fn gk_i32pqCreate(mut maxnodes: size_t) -> *mut gk_i32pq_t {
    let mut queue: *mut gk_i32pq_t = 0 as *mut gk_i32pq_t;
    queue = gk_malloc(
        ::core::mem::size_of::<gk_i32pq_t>() as u64,
        b"gk_pqCreate: queue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut gk_i32pq_t;
    gk_i32pqInit(queue, maxnodes);
    return queue;
}
#[no_mangle]
pub unsafe extern "C" fn gk_i32pqSeeKey(mut queue: *mut gk_i32pq_t, mut node: gk_idx_t) -> int32_t {
    let mut locator: *mut gk_idx_t = 0 as *mut gk_idx_t;
    let mut heap: *mut gk_i32kv_t = 0 as *mut gk_i32kv_t;
    heap = (*queue).heap;
    locator = (*queue).locator;
    return (*heap.offset(*locator.offset(node as isize) as isize)).key;
}
#[no_mangle]
pub unsafe extern "C" fn gk_i32pqSeeTopKey(mut queue: *mut gk_i32pq_t) -> int32_t {
    return if (*queue).nnodes == 0 as libc::c_int as i64 {
        2147483647 as libc::c_int
    } else {
        (*((*queue).heap).offset(0 as libc::c_int as isize)).key
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_i32pqSeeTopVal(mut queue: *mut gk_i32pq_t) -> gk_idx_t {
    return if (*queue).nnodes == 0 as libc::c_int as i64 {
        -(1) as i64
    } else {
        (*((*queue).heap).offset(0 as libc::c_int as isize)).val
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_i32pqGetTop(mut queue: *mut gk_i32pq_t) -> gk_idx_t {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut locator: *mut gk_idx_t = 0 as *mut gk_idx_t;
    let mut heap: *mut gk_i32kv_t = 0 as *mut gk_i32kv_t;
    let mut vtx: gk_idx_t = 0;
    let mut node: gk_idx_t = 0;
    let mut key: int32_t = 0;
    if (*queue).nnodes == 0 as libc::c_int as i64 {
        return -(1) as gk_idx_t;
    }
    (*queue).nnodes -= 1;
    (*queue).nnodes;
    heap = (*queue).heap;
    locator = (*queue).locator;
    vtx = (*heap.offset(0 as libc::c_int as isize)).val;
    *locator.offset(vtx as isize) = -(1) as gk_idx_t;
    i = (*queue).nnodes;
    if i > 0 as libc::c_int as i64 {
        key = (*heap.offset(i as isize)).key;
        node = (*heap.offset(i as isize)).val;
        i = 0 as libc::c_int as gk_idx_t;
        loop {
            j = 2 as libc::c_int as i64 * i + 1 as i64;
            if !(j < (*queue).nnodes) {
                break;
            }
            if (*heap.offset(j as isize)).key > key {
                if (j + 1 as i64) < (*queue).nnodes
                    && (*heap.offset((j + 1 as i64) as isize)).key
                        > (*heap.offset(j as isize)).key
                {
                    j = j + 1 as i64;
                }
                *heap.offset(i as isize) = *heap.offset(j as isize);
                *locator.offset((*heap.offset(i as isize)).val as isize) = i;
                i = j;
            } else {
                if !((j + 1 as i64) < (*queue).nnodes
                    && (*heap.offset((j + 1 as i64) as isize)).key > key)
                {
                    break;
                }
                j = j + 1 as i64;
                *heap.offset(i as isize) = *heap.offset(j as isize);
                *locator.offset((*heap.offset(i as isize)).val as isize) = i;
                i = j;
            }
        }
        (*heap.offset(i as isize)).key = key;
        (*heap.offset(i as isize)).val = node;
        *locator.offset(node as isize) = i;
    }
    return vtx;
}
#[no_mangle]
pub unsafe extern "C" fn gk_i64pqCheckHeap(mut queue: *mut gk_i64pq_t) -> libc::c_int {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut nnodes: size_t = 0;
    let mut locator: *mut gk_idx_t = 0 as *mut gk_idx_t;
    let mut heap: *mut gk_i64kv_t = 0 as *mut gk_i64kv_t;
    heap = (*queue).heap;
    locator = (*queue).locator;
    nnodes = (*queue).nnodes as size_t;
    if nnodes == 0 as libc::c_int as u64 {
        return 1;
    }
    i = 1 as gk_idx_t;
    while (i as u64) < nnodes {
        i += 1;
        i;
    }
    i = 1 as gk_idx_t;
    while (i as u64) < nnodes {
        i += 1;
        i;
    }
    i = 0 as libc::c_int as gk_idx_t;
    j = i;
    while i < (*queue).maxnodes {
        if *locator.offset(i as isize) != -(1) as i64 {
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    return 1;
}
#[no_mangle]
pub unsafe extern "C" fn gk_i64pqInit(mut queue: *mut gk_i64pq_t, mut maxnodes: size_t) {
    (*queue).nnodes = 0 as libc::c_int as gk_idx_t;
    (*queue).maxnodes = maxnodes as gk_idx_t;
    (*queue).heap = gk_i64kvmalloc(
        maxnodes,
        b"gk_PQInit: heap\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (*queue).locator = gk_idxsmalloc(
        maxnodes,
        -(1) as gk_idx_t,
        b"gk_PQInit: locator\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gk_i64pqReset(mut queue: *mut gk_i64pq_t) {
    let mut i: gk_idx_t = 0;
    let mut locator: *mut gk_idx_t = (*queue).locator;
    let mut heap: *mut gk_i64kv_t = (*queue).heap;
    i = (*queue).nnodes - 1 as i64;
    while i >= 0 as libc::c_int as i64 {
        *locator.offset((*heap.offset(i as isize)).val as isize) = -(1) as gk_idx_t;
        i -= 1;
        i;
    }
    (*queue).nnodes = 0 as libc::c_int as gk_idx_t;
}
#[no_mangle]
pub unsafe extern "C" fn gk_i64pqFree(mut queue: *mut gk_i64pq_t) {
    if queue.is_null() {
        return;
    }
    gk_free(
        &mut (*queue).heap as *mut *mut gk_i64kv_t as *mut *mut libc::c_void,
        &mut (*queue).locator as *mut *mut gk_idx_t,
        0 as *mut *mut libc::c_void,
    );
    (*queue).maxnodes = 0 as libc::c_int as gk_idx_t;
}
#[no_mangle]
pub unsafe extern "C" fn gk_i64pqDestroy(mut queue: *mut gk_i64pq_t) {
    if queue.is_null() {
        return;
    }
    gk_i64pqFree(queue);
    gk_free(
        &mut queue as *mut *mut gk_i64pq_t as *mut *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gk_i64pqLength(mut queue: *mut gk_i64pq_t) -> size_t {
    return (*queue).nnodes as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn gk_i64pqInsert(
    mut queue: *mut gk_i64pq_t,
    mut node: gk_idx_t,
    mut key: int64_t,
) -> libc::c_int {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut locator: *mut gk_idx_t = (*queue).locator;
    let mut heap: *mut gk_i64kv_t = (*queue).heap;
    let fresh2 = (*queue).nnodes;
    (*queue).nnodes = (*queue).nnodes + 1;
    i = fresh2;
    while i > 0 as libc::c_int as i64 {
        j = i - 1 as i64 >> 1;
        if !(key > (*heap.offset(j as isize)).key) {
            break;
        }
        *heap.offset(i as isize) = *heap.offset(j as isize);
        *locator.offset((*heap.offset(i as isize)).val as isize) = i;
        i = j;
    }
    (*heap.offset(i as isize)).key = key;
    (*heap.offset(i as isize)).val = node;
    *locator.offset(node as isize) = i;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gk_i64pqDelete(
    mut queue: *mut gk_i64pq_t,
    mut node: gk_idx_t,
) -> libc::c_int {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut nnodes: gk_idx_t = 0;
    let mut newkey: int64_t = 0;
    let mut oldkey: int64_t = 0;
    let mut locator: *mut gk_idx_t = (*queue).locator;
    let mut heap: *mut gk_i64kv_t = (*queue).heap;
    i = *locator.offset(node as isize);
    *locator.offset(node as isize) = -(1) as gk_idx_t;
    (*queue).nnodes -= 1;
    if (*queue).nnodes > 0 as libc::c_int as i64
        && (*heap.offset((*queue).nnodes as isize)).val != node
    {
        node = (*heap.offset((*queue).nnodes as isize)).val;
        newkey = (*heap.offset((*queue).nnodes as isize)).key;
        oldkey = (*heap.offset(i as isize)).key;
        if newkey > oldkey {
            while i > 0 as libc::c_int as i64 {
                j = i - 1 as i64 >> 1;
                if !(newkey > (*heap.offset(j as isize)).key) {
                    break;
                }
                *heap.offset(i as isize) = *heap.offset(j as isize);
                *locator.offset((*heap.offset(i as isize)).val as isize) = i;
                i = j;
            }
        } else {
            nnodes = (*queue).nnodes;
            loop {
                j = (i << 1) + 1 as i64;
                if !(j < nnodes) {
                    break;
                }
                if (*heap.offset(j as isize)).key > newkey {
                    if (j + 1 as i64) < nnodes
                        && (*heap.offset((j + 1 as i64) as isize)).key
                            > (*heap.offset(j as isize)).key
                    {
                        j += 1;
                        j;
                    }
                    *heap.offset(i as isize) = *heap.offset(j as isize);
                    *locator.offset((*heap.offset(i as isize)).val as isize) = i;
                    i = j;
                } else {
                    if !((j + 1 as i64) < nnodes
                        && (*heap.offset((j + 1 as i64) as isize)).key
                            > newkey)
                    {
                        break;
                    }
                    j += 1;
                    j;
                    *heap.offset(i as isize) = *heap.offset(j as isize);
                    *locator.offset((*heap.offset(i as isize)).val as isize) = i;
                    i = j;
                }
            }
        }
        (*heap.offset(i as isize)).key = newkey;
        (*heap.offset(i as isize)).val = node;
        *locator.offset(node as isize) = i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gk_i64pqUpdate(
    mut queue: *mut gk_i64pq_t,
    mut node: gk_idx_t,
    mut newkey: int64_t,
) {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut nnodes: gk_idx_t = 0;
    let mut oldkey: int64_t = 0;
    let mut locator: *mut gk_idx_t = (*queue).locator;
    let mut heap: *mut gk_i64kv_t = (*queue).heap;
    oldkey = (*heap.offset(*locator.offset(node as isize) as isize)).key;
    i = *locator.offset(node as isize);
    if newkey > oldkey {
        while i > 0 as libc::c_int as i64 {
            j = i - 1 as i64 >> 1;
            if !(newkey > (*heap.offset(j as isize)).key) {
                break;
            }
            *heap.offset(i as isize) = *heap.offset(j as isize);
            *locator.offset((*heap.offset(i as isize)).val as isize) = i;
            i = j;
        }
    } else {
        nnodes = (*queue).nnodes;
        loop {
            j = (i << 1) + 1 as i64;
            if !(j < nnodes) {
                break;
            }
            if (*heap.offset(j as isize)).key > newkey {
                if (j + 1 as i64) < nnodes
                    && (*heap.offset((j + 1 as i64) as isize)).key
                        > (*heap.offset(j as isize)).key
                {
                    j += 1;
                    j;
                }
                *heap.offset(i as isize) = *heap.offset(j as isize);
                *locator.offset((*heap.offset(i as isize)).val as isize) = i;
                i = j;
            } else {
                if !((j + 1 as i64) < nnodes
                    && (*heap.offset((j + 1 as i64) as isize)).key > newkey)
                {
                    break;
                }
                j += 1;
                j;
                *heap.offset(i as isize) = *heap.offset(j as isize);
                *locator.offset((*heap.offset(i as isize)).val as isize) = i;
                i = j;
            }
        }
    }
    (*heap.offset(i as isize)).key = newkey;
    (*heap.offset(i as isize)).val = node;
    *locator.offset(node as isize) = i;
}
#[no_mangle]
pub unsafe extern "C" fn gk_i64pqCreate(mut maxnodes: size_t) -> *mut gk_i64pq_t {
    let mut queue: *mut gk_i64pq_t = 0 as *mut gk_i64pq_t;
    queue = gk_malloc(
        ::core::mem::size_of::<gk_i64pq_t>() as u64,
        b"gk_pqCreate: queue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut gk_i64pq_t;
    gk_i64pqInit(queue, maxnodes);
    return queue;
}
#[no_mangle]
pub unsafe extern "C" fn gk_i64pqSeeKey(mut queue: *mut gk_i64pq_t, mut node: gk_idx_t) -> int64_t {
    let mut locator: *mut gk_idx_t = 0 as *mut gk_idx_t;
    let mut heap: *mut gk_i64kv_t = 0 as *mut gk_i64kv_t;
    heap = (*queue).heap;
    locator = (*queue).locator;
    return (*heap.offset(*locator.offset(node as isize) as isize)).key;
}
#[no_mangle]
pub unsafe extern "C" fn gk_i64pqSeeTopKey(mut queue: *mut gk_i64pq_t) -> int64_t {
    return if (*queue).nnodes == 0 as libc::c_int as i64 {
        9223372036854775807 as i64
    } else {
        (*((*queue).heap).offset(0 as libc::c_int as isize)).key as i64
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_i64pqSeeTopVal(mut queue: *mut gk_i64pq_t) -> gk_idx_t {
    return if (*queue).nnodes == 0 as libc::c_int as i64 {
        -(1) as i64
    } else {
        (*((*queue).heap).offset(0 as libc::c_int as isize)).val
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_i64pqGetTop(mut queue: *mut gk_i64pq_t) -> gk_idx_t {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut locator: *mut gk_idx_t = 0 as *mut gk_idx_t;
    let mut heap: *mut gk_i64kv_t = 0 as *mut gk_i64kv_t;
    let mut vtx: gk_idx_t = 0;
    let mut node: gk_idx_t = 0;
    let mut key: int64_t = 0;
    if (*queue).nnodes == 0 as libc::c_int as i64 {
        return -(1) as gk_idx_t;
    }
    (*queue).nnodes -= 1;
    (*queue).nnodes;
    heap = (*queue).heap;
    locator = (*queue).locator;
    vtx = (*heap.offset(0 as libc::c_int as isize)).val;
    *locator.offset(vtx as isize) = -(1) as gk_idx_t;
    i = (*queue).nnodes;
    if i > 0 as libc::c_int as i64 {
        key = (*heap.offset(i as isize)).key;
        node = (*heap.offset(i as isize)).val;
        i = 0 as libc::c_int as gk_idx_t;
        loop {
            j = 2 as libc::c_int as i64 * i + 1 as i64;
            if !(j < (*queue).nnodes) {
                break;
            }
            if (*heap.offset(j as isize)).key > key {
                if (j + 1 as i64) < (*queue).nnodes
                    && (*heap.offset((j + 1 as i64) as isize)).key
                        > (*heap.offset(j as isize)).key
                {
                    j = j + 1 as i64;
                }
                *heap.offset(i as isize) = *heap.offset(j as isize);
                *locator.offset((*heap.offset(i as isize)).val as isize) = i;
                i = j;
            } else {
                if !((j + 1 as i64) < (*queue).nnodes
                    && (*heap.offset((j + 1 as i64) as isize)).key > key)
                {
                    break;
                }
                j = j + 1 as i64;
                *heap.offset(i as isize) = *heap.offset(j as isize);
                *locator.offset((*heap.offset(i as isize)).val as isize) = i;
                i = j;
            }
        }
        (*heap.offset(i as isize)).key = key;
        (*heap.offset(i as isize)).val = node;
        *locator.offset(node as isize) = i;
    }
    return vtx;
}
#[no_mangle]
pub unsafe extern "C" fn gk_fpqInit(mut queue: *mut gk_fpq_t, mut maxnodes: size_t) {
    (*queue).nnodes = 0 as libc::c_int as gk_idx_t;
    (*queue).maxnodes = maxnodes as gk_idx_t;
    (*queue).heap = gk_fkvmalloc(
        maxnodes,
        b"gk_PQInit: heap\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (*queue).locator = gk_idxsmalloc(
        maxnodes,
        -(1) as gk_idx_t,
        b"gk_PQInit: locator\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gk_fpqReset(mut queue: *mut gk_fpq_t) {
    let mut i: gk_idx_t = 0;
    let mut locator: *mut gk_idx_t = (*queue).locator;
    let mut heap: *mut gk_fkv_t = (*queue).heap;
    i = (*queue).nnodes - 1 as i64;
    while i >= 0 as libc::c_int as i64 {
        *locator.offset((*heap.offset(i as isize)).val as isize) = -(1) as gk_idx_t;
        i -= 1;
        i;
    }
    (*queue).nnodes = 0 as libc::c_int as gk_idx_t;
}
#[no_mangle]
pub unsafe extern "C" fn gk_fpqCheckHeap(mut queue: *mut gk_fpq_t) -> libc::c_int {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut nnodes: size_t = 0;
    let mut locator: *mut gk_idx_t = 0 as *mut gk_idx_t;
    let mut heap: *mut gk_fkv_t = 0 as *mut gk_fkv_t;
    heap = (*queue).heap;
    locator = (*queue).locator;
    nnodes = (*queue).nnodes as size_t;
    if nnodes == 0 as libc::c_int as u64 {
        return 1;
    }
    i = 1 as gk_idx_t;
    while (i as u64) < nnodes {
        i += 1;
        i;
    }
    i = 1 as gk_idx_t;
    while (i as u64) < nnodes {
        i += 1;
        i;
    }
    i = 0 as libc::c_int as gk_idx_t;
    j = i;
    while i < (*queue).maxnodes {
        if *locator.offset(i as isize) != -(1) as i64 {
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    return 1;
}
#[no_mangle]
pub unsafe extern "C" fn gk_fpqFree(mut queue: *mut gk_fpq_t) {
    if queue.is_null() {
        return;
    }
    gk_free(
        &mut (*queue).heap as *mut *mut gk_fkv_t as *mut *mut libc::c_void,
        &mut (*queue).locator as *mut *mut gk_idx_t,
        0 as *mut *mut libc::c_void,
    );
    (*queue).maxnodes = 0 as libc::c_int as gk_idx_t;
}
#[no_mangle]
pub unsafe extern "C" fn gk_fpqDestroy(mut queue: *mut gk_fpq_t) {
    if queue.is_null() {
        return;
    }
    gk_fpqFree(queue);
    gk_free(
        &mut queue as *mut *mut gk_fpq_t as *mut *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gk_fpqLength(mut queue: *mut gk_fpq_t) -> size_t {
    return (*queue).nnodes as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn gk_fpqInsert(
    mut queue: *mut gk_fpq_t,
    mut node: gk_idx_t,
    mut key: libc::c_float,
) -> libc::c_int {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut locator: *mut gk_idx_t = (*queue).locator;
    let mut heap: *mut gk_fkv_t = (*queue).heap;
    let fresh3 = (*queue).nnodes;
    (*queue).nnodes = (*queue).nnodes + 1;
    i = fresh3;
    while i > 0 as libc::c_int as i64 {
        j = i - 1 as i64 >> 1;
        if !(key > (*heap.offset(j as isize)).key) {
            break;
        }
        *heap.offset(i as isize) = *heap.offset(j as isize);
        *locator.offset((*heap.offset(i as isize)).val as isize) = i;
        i = j;
    }
    (*heap.offset(i as isize)).key = key;
    (*heap.offset(i as isize)).val = node;
    *locator.offset(node as isize) = i;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gk_fpqDelete(mut queue: *mut gk_fpq_t, mut node: gk_idx_t) -> libc::c_int {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut nnodes: gk_idx_t = 0;
    let mut newkey: libc::c_float = 0.;
    let mut oldkey: libc::c_float = 0.;
    let mut locator: *mut gk_idx_t = (*queue).locator;
    let mut heap: *mut gk_fkv_t = (*queue).heap;
    i = *locator.offset(node as isize);
    *locator.offset(node as isize) = -(1) as gk_idx_t;
    (*queue).nnodes -= 1;
    if (*queue).nnodes > 0 as libc::c_int as i64
        && (*heap.offset((*queue).nnodes as isize)).val != node
    {
        node = (*heap.offset((*queue).nnodes as isize)).val;
        newkey = (*heap.offset((*queue).nnodes as isize)).key;
        oldkey = (*heap.offset(i as isize)).key;
        if newkey > oldkey {
            while i > 0 as libc::c_int as i64 {
                j = i - 1 as i64 >> 1;
                if !(newkey > (*heap.offset(j as isize)).key) {
                    break;
                }
                *heap.offset(i as isize) = *heap.offset(j as isize);
                *locator.offset((*heap.offset(i as isize)).val as isize) = i;
                i = j;
            }
        } else {
            nnodes = (*queue).nnodes;
            loop {
                j = (i << 1) + 1 as i64;
                if !(j < nnodes) {
                    break;
                }
                if (*heap.offset(j as isize)).key > newkey {
                    if (j + 1 as i64) < nnodes
                        && (*heap.offset((j + 1 as i64) as isize)).key
                            > (*heap.offset(j as isize)).key
                    {
                        j += 1;
                        j;
                    }
                    *heap.offset(i as isize) = *heap.offset(j as isize);
                    *locator.offset((*heap.offset(i as isize)).val as isize) = i;
                    i = j;
                } else {
                    if !((j + 1 as i64) < nnodes
                        && (*heap.offset((j + 1 as i64) as isize)).key
                            > newkey)
                    {
                        break;
                    }
                    j += 1;
                    j;
                    *heap.offset(i as isize) = *heap.offset(j as isize);
                    *locator.offset((*heap.offset(i as isize)).val as isize) = i;
                    i = j;
                }
            }
        }
        (*heap.offset(i as isize)).key = newkey;
        (*heap.offset(i as isize)).val = node;
        *locator.offset(node as isize) = i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gk_fpqUpdate(
    mut queue: *mut gk_fpq_t,
    mut node: gk_idx_t,
    mut newkey: libc::c_float,
) {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut nnodes: gk_idx_t = 0;
    let mut oldkey: libc::c_float = 0.;
    let mut locator: *mut gk_idx_t = (*queue).locator;
    let mut heap: *mut gk_fkv_t = (*queue).heap;
    oldkey = (*heap.offset(*locator.offset(node as isize) as isize)).key;
    i = *locator.offset(node as isize);
    if newkey > oldkey {
        while i > 0 as libc::c_int as i64 {
            j = i - 1 as i64 >> 1;
            if !(newkey > (*heap.offset(j as isize)).key) {
                break;
            }
            *heap.offset(i as isize) = *heap.offset(j as isize);
            *locator.offset((*heap.offset(i as isize)).val as isize) = i;
            i = j;
        }
    } else {
        nnodes = (*queue).nnodes;
        loop {
            j = (i << 1) + 1 as i64;
            if !(j < nnodes) {
                break;
            }
            if (*heap.offset(j as isize)).key > newkey {
                if (j + 1 as i64) < nnodes
                    && (*heap.offset((j + 1 as i64) as isize)).key
                        > (*heap.offset(j as isize)).key
                {
                    j += 1;
                    j;
                }
                *heap.offset(i as isize) = *heap.offset(j as isize);
                *locator.offset((*heap.offset(i as isize)).val as isize) = i;
                i = j;
            } else {
                if !((j + 1 as i64) < nnodes
                    && (*heap.offset((j + 1 as i64) as isize)).key > newkey)
                {
                    break;
                }
                j += 1;
                j;
                *heap.offset(i as isize) = *heap.offset(j as isize);
                *locator.offset((*heap.offset(i as isize)).val as isize) = i;
                i = j;
            }
        }
    }
    (*heap.offset(i as isize)).key = newkey;
    (*heap.offset(i as isize)).val = node;
    *locator.offset(node as isize) = i;
}
#[no_mangle]
pub unsafe extern "C" fn gk_fpqCreate(mut maxnodes: size_t) -> *mut gk_fpq_t {
    let mut queue: *mut gk_fpq_t = 0 as *mut gk_fpq_t;
    queue = gk_malloc(
        ::core::mem::size_of::<gk_fpq_t>() as u64,
        b"gk_pqCreate: queue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut gk_fpq_t;
    gk_fpqInit(queue, maxnodes);
    return queue;
}
#[no_mangle]
pub unsafe extern "C" fn gk_fpqSeeKey(
    mut queue: *mut gk_fpq_t,
    mut node: gk_idx_t,
) -> libc::c_float {
    let mut locator: *mut gk_idx_t = 0 as *mut gk_idx_t;
    let mut heap: *mut gk_fkv_t = 0 as *mut gk_fkv_t;
    heap = (*queue).heap;
    locator = (*queue).locator;
    return (*heap.offset(*locator.offset(node as isize) as isize)).key;
}
#[no_mangle]
pub unsafe extern "C" fn gk_fpqSeeTopKey(mut queue: *mut gk_fpq_t) -> libc::c_float {
    return if (*queue).nnodes == 0 as libc::c_int as i64 {
        3.40282347e+38f32
    } else {
        (*((*queue).heap).offset(0 as libc::c_int as isize)).key
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_fpqGetTop(mut queue: *mut gk_fpq_t) -> gk_idx_t {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut locator: *mut gk_idx_t = 0 as *mut gk_idx_t;
    let mut heap: *mut gk_fkv_t = 0 as *mut gk_fkv_t;
    let mut vtx: gk_idx_t = 0;
    let mut node: gk_idx_t = 0;
    let mut key: libc::c_float = 0.;
    if (*queue).nnodes == 0 as libc::c_int as i64 {
        return -(1) as gk_idx_t;
    }
    (*queue).nnodes -= 1;
    (*queue).nnodes;
    heap = (*queue).heap;
    locator = (*queue).locator;
    vtx = (*heap.offset(0 as libc::c_int as isize)).val;
    *locator.offset(vtx as isize) = -(1) as gk_idx_t;
    i = (*queue).nnodes;
    if i > 0 as libc::c_int as i64 {
        key = (*heap.offset(i as isize)).key;
        node = (*heap.offset(i as isize)).val;
        i = 0 as libc::c_int as gk_idx_t;
        loop {
            j = 2 as libc::c_int as i64 * i + 1 as i64;
            if !(j < (*queue).nnodes) {
                break;
            }
            if (*heap.offset(j as isize)).key > key {
                if (j + 1 as i64) < (*queue).nnodes
                    && (*heap.offset((j + 1 as i64) as isize)).key
                        > (*heap.offset(j as isize)).key
                {
                    j = j + 1 as i64;
                }
                *heap.offset(i as isize) = *heap.offset(j as isize);
                *locator.offset((*heap.offset(i as isize)).val as isize) = i;
                i = j;
            } else {
                if !((j + 1 as i64) < (*queue).nnodes
                    && (*heap.offset((j + 1 as i64) as isize)).key > key)
                {
                    break;
                }
                j = j + 1 as i64;
                *heap.offset(i as isize) = *heap.offset(j as isize);
                *locator.offset((*heap.offset(i as isize)).val as isize) = i;
                i = j;
            }
        }
        (*heap.offset(i as isize)).key = key;
        (*heap.offset(i as isize)).val = node;
        *locator.offset(node as isize) = i;
    }
    return vtx;
}
#[no_mangle]
pub unsafe extern "C" fn gk_fpqSeeTopVal(mut queue: *mut gk_fpq_t) -> gk_idx_t {
    return if (*queue).nnodes == 0 as libc::c_int as i64 {
        -(1) as i64
    } else {
        (*((*queue).heap).offset(0 as libc::c_int as isize)).val
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_dpqFree(mut queue: *mut gk_dpq_t) {
    if queue.is_null() {
        return;
    }
    gk_free(
        &mut (*queue).heap as *mut *mut gk_dkv_t as *mut *mut libc::c_void,
        &mut (*queue).locator as *mut *mut gk_idx_t,
        0 as *mut *mut libc::c_void,
    );
    (*queue).maxnodes = 0 as libc::c_int as gk_idx_t;
}
#[no_mangle]
pub unsafe extern "C" fn gk_dpqUpdate(
    mut queue: *mut gk_dpq_t,
    mut node: gk_idx_t,
    mut newkey: libc::c_double,
) {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut nnodes: gk_idx_t = 0;
    let mut oldkey: libc::c_double = 0.;
    let mut locator: *mut gk_idx_t = (*queue).locator;
    let mut heap: *mut gk_dkv_t = (*queue).heap;
    oldkey = (*heap.offset(*locator.offset(node as isize) as isize)).key;
    i = *locator.offset(node as isize);
    if newkey > oldkey {
        while i > 0 as libc::c_int as i64 {
            j = i - 1 as i64 >> 1;
            if !(newkey > (*heap.offset(j as isize)).key) {
                break;
            }
            *heap.offset(i as isize) = *heap.offset(j as isize);
            *locator.offset((*heap.offset(i as isize)).val as isize) = i;
            i = j;
        }
    } else {
        nnodes = (*queue).nnodes;
        loop {
            j = (i << 1) + 1 as i64;
            if !(j < nnodes) {
                break;
            }
            if (*heap.offset(j as isize)).key > newkey {
                if (j + 1 as i64) < nnodes
                    && (*heap.offset((j + 1 as i64) as isize)).key
                        > (*heap.offset(j as isize)).key
                {
                    j += 1;
                    j;
                }
                *heap.offset(i as isize) = *heap.offset(j as isize);
                *locator.offset((*heap.offset(i as isize)).val as isize) = i;
                i = j;
            } else {
                if !((j + 1 as i64) < nnodes
                    && (*heap.offset((j + 1 as i64) as isize)).key > newkey)
                {
                    break;
                }
                j += 1;
                j;
                *heap.offset(i as isize) = *heap.offset(j as isize);
                *locator.offset((*heap.offset(i as isize)).val as isize) = i;
                i = j;
            }
        }
    }
    (*heap.offset(i as isize)).key = newkey;
    (*heap.offset(i as isize)).val = node;
    *locator.offset(node as isize) = i;
}
#[no_mangle]
pub unsafe extern "C" fn gk_dpqInit(mut queue: *mut gk_dpq_t, mut maxnodes: size_t) {
    (*queue).nnodes = 0 as libc::c_int as gk_idx_t;
    (*queue).maxnodes = maxnodes as gk_idx_t;
    (*queue).heap = gk_dkvmalloc(
        maxnodes,
        b"gk_PQInit: heap\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (*queue).locator = gk_idxsmalloc(
        maxnodes,
        -(1) as gk_idx_t,
        b"gk_PQInit: locator\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gk_dpqReset(mut queue: *mut gk_dpq_t) {
    let mut i: gk_idx_t = 0;
    let mut locator: *mut gk_idx_t = (*queue).locator;
    let mut heap: *mut gk_dkv_t = (*queue).heap;
    i = (*queue).nnodes - 1 as i64;
    while i >= 0 as libc::c_int as i64 {
        *locator.offset((*heap.offset(i as isize)).val as isize) = -(1) as gk_idx_t;
        i -= 1;
        i;
    }
    (*queue).nnodes = 0 as libc::c_int as gk_idx_t;
}
#[no_mangle]
pub unsafe extern "C" fn gk_dpqCheckHeap(mut queue: *mut gk_dpq_t) -> libc::c_int {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut nnodes: size_t = 0;
    let mut locator: *mut gk_idx_t = 0 as *mut gk_idx_t;
    let mut heap: *mut gk_dkv_t = 0 as *mut gk_dkv_t;
    heap = (*queue).heap;
    locator = (*queue).locator;
    nnodes = (*queue).nnodes as size_t;
    if nnodes == 0 as libc::c_int as u64 {
        return 1;
    }
    i = 1 as gk_idx_t;
    while (i as u64) < nnodes {
        i += 1;
        i;
    }
    i = 1 as gk_idx_t;
    while (i as u64) < nnodes {
        i += 1;
        i;
    }
    i = 0 as libc::c_int as gk_idx_t;
    j = i;
    while i < (*queue).maxnodes {
        if *locator.offset(i as isize) != -(1) as i64 {
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    return 1;
}
#[no_mangle]
pub unsafe extern "C" fn gk_dpqDestroy(mut queue: *mut gk_dpq_t) {
    if queue.is_null() {
        return;
    }
    gk_dpqFree(queue);
    gk_free(
        &mut queue as *mut *mut gk_dpq_t as *mut *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gk_dpqLength(mut queue: *mut gk_dpq_t) -> size_t {
    return (*queue).nnodes as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn gk_dpqInsert(
    mut queue: *mut gk_dpq_t,
    mut node: gk_idx_t,
    mut key: libc::c_double,
) -> libc::c_int {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut locator: *mut gk_idx_t = (*queue).locator;
    let mut heap: *mut gk_dkv_t = (*queue).heap;
    let fresh4 = (*queue).nnodes;
    (*queue).nnodes = (*queue).nnodes + 1;
    i = fresh4;
    while i > 0 as libc::c_int as i64 {
        j = i - 1 as i64 >> 1;
        if !(key > (*heap.offset(j as isize)).key) {
            break;
        }
        *heap.offset(i as isize) = *heap.offset(j as isize);
        *locator.offset((*heap.offset(i as isize)).val as isize) = i;
        i = j;
    }
    (*heap.offset(i as isize)).key = key;
    (*heap.offset(i as isize)).val = node;
    *locator.offset(node as isize) = i;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gk_dpqDelete(mut queue: *mut gk_dpq_t, mut node: gk_idx_t) -> libc::c_int {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut nnodes: gk_idx_t = 0;
    let mut newkey: libc::c_double = 0.;
    let mut oldkey: libc::c_double = 0.;
    let mut locator: *mut gk_idx_t = (*queue).locator;
    let mut heap: *mut gk_dkv_t = (*queue).heap;
    i = *locator.offset(node as isize);
    *locator.offset(node as isize) = -(1) as gk_idx_t;
    (*queue).nnodes -= 1;
    if (*queue).nnodes > 0 as libc::c_int as i64
        && (*heap.offset((*queue).nnodes as isize)).val != node
    {
        node = (*heap.offset((*queue).nnodes as isize)).val;
        newkey = (*heap.offset((*queue).nnodes as isize)).key;
        oldkey = (*heap.offset(i as isize)).key;
        if newkey > oldkey {
            while i > 0 as libc::c_int as i64 {
                j = i - 1 as i64 >> 1;
                if !(newkey > (*heap.offset(j as isize)).key) {
                    break;
                }
                *heap.offset(i as isize) = *heap.offset(j as isize);
                *locator.offset((*heap.offset(i as isize)).val as isize) = i;
                i = j;
            }
        } else {
            nnodes = (*queue).nnodes;
            loop {
                j = (i << 1) + 1 as i64;
                if !(j < nnodes) {
                    break;
                }
                if (*heap.offset(j as isize)).key > newkey {
                    if (j + 1 as i64) < nnodes
                        && (*heap.offset((j + 1 as i64) as isize)).key
                            > (*heap.offset(j as isize)).key
                    {
                        j += 1;
                        j;
                    }
                    *heap.offset(i as isize) = *heap.offset(j as isize);
                    *locator.offset((*heap.offset(i as isize)).val as isize) = i;
                    i = j;
                } else {
                    if !((j + 1 as i64) < nnodes
                        && (*heap.offset((j + 1 as i64) as isize)).key
                            > newkey)
                    {
                        break;
                    }
                    j += 1;
                    j;
                    *heap.offset(i as isize) = *heap.offset(j as isize);
                    *locator.offset((*heap.offset(i as isize)).val as isize) = i;
                    i = j;
                }
            }
        }
        (*heap.offset(i as isize)).key = newkey;
        (*heap.offset(i as isize)).val = node;
        *locator.offset(node as isize) = i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gk_dpqCreate(mut maxnodes: size_t) -> *mut gk_dpq_t {
    let mut queue: *mut gk_dpq_t = 0 as *mut gk_dpq_t;
    queue = gk_malloc(
        ::core::mem::size_of::<gk_dpq_t>() as u64,
        b"gk_pqCreate: queue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut gk_dpq_t;
    gk_dpqInit(queue, maxnodes);
    return queue;
}
#[no_mangle]
pub unsafe extern "C" fn gk_dpqSeeTopKey(mut queue: *mut gk_dpq_t) -> libc::c_double {
    return if (*queue).nnodes == 0 as libc::c_int as i64 {
        1.7976931348623157e+308f64
    } else {
        (*((*queue).heap).offset(0 as libc::c_int as isize)).key
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_dpqSeeKey(
    mut queue: *mut gk_dpq_t,
    mut node: gk_idx_t,
) -> libc::c_double {
    let mut locator: *mut gk_idx_t = 0 as *mut gk_idx_t;
    let mut heap: *mut gk_dkv_t = 0 as *mut gk_dkv_t;
    heap = (*queue).heap;
    locator = (*queue).locator;
    return (*heap.offset(*locator.offset(node as isize) as isize)).key;
}
#[no_mangle]
pub unsafe extern "C" fn gk_dpqGetTop(mut queue: *mut gk_dpq_t) -> gk_idx_t {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut locator: *mut gk_idx_t = 0 as *mut gk_idx_t;
    let mut heap: *mut gk_dkv_t = 0 as *mut gk_dkv_t;
    let mut vtx: gk_idx_t = 0;
    let mut node: gk_idx_t = 0;
    let mut key: libc::c_double = 0.;
    if (*queue).nnodes == 0 as libc::c_int as i64 {
        return -(1) as gk_idx_t;
    }
    (*queue).nnodes -= 1;
    (*queue).nnodes;
    heap = (*queue).heap;
    locator = (*queue).locator;
    vtx = (*heap.offset(0 as libc::c_int as isize)).val;
    *locator.offset(vtx as isize) = -(1) as gk_idx_t;
    i = (*queue).nnodes;
    if i > 0 as libc::c_int as i64 {
        key = (*heap.offset(i as isize)).key;
        node = (*heap.offset(i as isize)).val;
        i = 0 as libc::c_int as gk_idx_t;
        loop {
            j = 2 as libc::c_int as i64 * i + 1 as i64;
            if !(j < (*queue).nnodes) {
                break;
            }
            if (*heap.offset(j as isize)).key > key {
                if (j + 1 as i64) < (*queue).nnodes
                    && (*heap.offset((j + 1 as i64) as isize)).key
                        > (*heap.offset(j as isize)).key
                {
                    j = j + 1 as i64;
                }
                *heap.offset(i as isize) = *heap.offset(j as isize);
                *locator.offset((*heap.offset(i as isize)).val as isize) = i;
                i = j;
            } else {
                if !((j + 1 as i64) < (*queue).nnodes
                    && (*heap.offset((j + 1 as i64) as isize)).key > key)
                {
                    break;
                }
                j = j + 1 as i64;
                *heap.offset(i as isize) = *heap.offset(j as isize);
                *locator.offset((*heap.offset(i as isize)).val as isize) = i;
                i = j;
            }
        }
        (*heap.offset(i as isize)).key = key;
        (*heap.offset(i as isize)).val = node;
        *locator.offset(node as isize) = i;
    }
    return vtx;
}
#[no_mangle]
pub unsafe extern "C" fn gk_dpqSeeTopVal(mut queue: *mut gk_dpq_t) -> gk_idx_t {
    return if (*queue).nnodes == 0 as libc::c_int as i64 {
        -(1) as i64
    } else {
        (*((*queue).heap).offset(0 as libc::c_int as isize)).val
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_idxpqCheckHeap(mut queue: *mut gk_idxpq_t) -> libc::c_int {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut nnodes: size_t = 0;
    let mut locator: *mut gk_idx_t = 0 as *mut gk_idx_t;
    let mut heap: *mut gk_idxkv_t = 0 as *mut gk_idxkv_t;
    heap = (*queue).heap;
    locator = (*queue).locator;
    nnodes = (*queue).nnodes as size_t;
    if nnodes == 0 as libc::c_int as u64 {
        return 1;
    }
    i = 1 as gk_idx_t;
    while (i as u64) < nnodes {
        i += 1;
        i;
    }
    i = 1 as gk_idx_t;
    while (i as u64) < nnodes {
        i += 1;
        i;
    }
    i = 0 as libc::c_int as gk_idx_t;
    j = i;
    while i < (*queue).maxnodes {
        if *locator.offset(i as isize) != -(1) as i64 {
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    return 1;
}
#[no_mangle]
pub unsafe extern "C" fn gk_idxpqInit(mut queue: *mut gk_idxpq_t, mut maxnodes: size_t) {
    (*queue).nnodes = 0 as libc::c_int as gk_idx_t;
    (*queue).maxnodes = maxnodes as gk_idx_t;
    (*queue).heap = gk_idxkvmalloc(
        maxnodes,
        b"gk_PQInit: heap\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (*queue).locator = gk_idxsmalloc(
        maxnodes,
        -(1) as gk_idx_t,
        b"gk_PQInit: locator\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gk_idxpqReset(mut queue: *mut gk_idxpq_t) {
    let mut i: gk_idx_t = 0;
    let mut locator: *mut gk_idx_t = (*queue).locator;
    let mut heap: *mut gk_idxkv_t = (*queue).heap;
    i = (*queue).nnodes - 1 as i64;
    while i >= 0 as libc::c_int as i64 {
        *locator.offset((*heap.offset(i as isize)).val as isize) = -(1) as gk_idx_t;
        i -= 1;
        i;
    }
    (*queue).nnodes = 0 as libc::c_int as gk_idx_t;
}
#[no_mangle]
pub unsafe extern "C" fn gk_idxpqFree(mut queue: *mut gk_idxpq_t) {
    if queue.is_null() {
        return;
    }
    gk_free(
        &mut (*queue).heap as *mut *mut gk_idxkv_t as *mut *mut libc::c_void,
        &mut (*queue).locator as *mut *mut gk_idx_t,
        0 as *mut *mut libc::c_void,
    );
    (*queue).maxnodes = 0 as libc::c_int as gk_idx_t;
}
#[no_mangle]
pub unsafe extern "C" fn gk_idxpqDestroy(mut queue: *mut gk_idxpq_t) {
    if queue.is_null() {
        return;
    }
    gk_idxpqFree(queue);
    gk_free(
        &mut queue as *mut *mut gk_idxpq_t as *mut *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gk_idxpqLength(mut queue: *mut gk_idxpq_t) -> size_t {
    return (*queue).nnodes as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn gk_idxpqInsert(
    mut queue: *mut gk_idxpq_t,
    mut node: gk_idx_t,
    mut key: gk_idx_t,
) -> libc::c_int {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut locator: *mut gk_idx_t = (*queue).locator;
    let mut heap: *mut gk_idxkv_t = (*queue).heap;
    let fresh5 = (*queue).nnodes;
    (*queue).nnodes = (*queue).nnodes + 1;
    i = fresh5;
    while i > 0 as libc::c_int as i64 {
        j = i - 1 as i64 >> 1;
        if !(key > (*heap.offset(j as isize)).key) {
            break;
        }
        *heap.offset(i as isize) = *heap.offset(j as isize);
        *locator.offset((*heap.offset(i as isize)).val as isize) = i;
        i = j;
    }
    (*heap.offset(i as isize)).key = key;
    (*heap.offset(i as isize)).val = node;
    *locator.offset(node as isize) = i;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gk_idxpqDelete(
    mut queue: *mut gk_idxpq_t,
    mut node: gk_idx_t,
) -> libc::c_int {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut nnodes: gk_idx_t = 0;
    let mut newkey: gk_idx_t = 0;
    let mut oldkey: gk_idx_t = 0;
    let mut locator: *mut gk_idx_t = (*queue).locator;
    let mut heap: *mut gk_idxkv_t = (*queue).heap;
    i = *locator.offset(node as isize);
    *locator.offset(node as isize) = -(1) as gk_idx_t;
    (*queue).nnodes -= 1;
    if (*queue).nnodes > 0 as libc::c_int as i64
        && (*heap.offset((*queue).nnodes as isize)).val != node
    {
        node = (*heap.offset((*queue).nnodes as isize)).val;
        newkey = (*heap.offset((*queue).nnodes as isize)).key;
        oldkey = (*heap.offset(i as isize)).key;
        if newkey > oldkey {
            while i > 0 as libc::c_int as i64 {
                j = i - 1 as i64 >> 1;
                if !(newkey > (*heap.offset(j as isize)).key) {
                    break;
                }
                *heap.offset(i as isize) = *heap.offset(j as isize);
                *locator.offset((*heap.offset(i as isize)).val as isize) = i;
                i = j;
            }
        } else {
            nnodes = (*queue).nnodes;
            loop {
                j = (i << 1) + 1 as i64;
                if !(j < nnodes) {
                    break;
                }
                if (*heap.offset(j as isize)).key > newkey {
                    if (j + 1 as i64) < nnodes
                        && (*heap.offset((j + 1 as i64) as isize)).key
                            > (*heap.offset(j as isize)).key
                    {
                        j += 1;
                        j;
                    }
                    *heap.offset(i as isize) = *heap.offset(j as isize);
                    *locator.offset((*heap.offset(i as isize)).val as isize) = i;
                    i = j;
                } else {
                    if !((j + 1 as i64) < nnodes
                        && (*heap.offset((j + 1 as i64) as isize)).key
                            > newkey)
                    {
                        break;
                    }
                    j += 1;
                    j;
                    *heap.offset(i as isize) = *heap.offset(j as isize);
                    *locator.offset((*heap.offset(i as isize)).val as isize) = i;
                    i = j;
                }
            }
        }
        (*heap.offset(i as isize)).key = newkey;
        (*heap.offset(i as isize)).val = node;
        *locator.offset(node as isize) = i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gk_idxpqUpdate(
    mut queue: *mut gk_idxpq_t,
    mut node: gk_idx_t,
    mut newkey: gk_idx_t,
) {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut nnodes: gk_idx_t = 0;
    let mut oldkey: gk_idx_t = 0;
    let mut locator: *mut gk_idx_t = (*queue).locator;
    let mut heap: *mut gk_idxkv_t = (*queue).heap;
    oldkey = (*heap.offset(*locator.offset(node as isize) as isize)).key;
    i = *locator.offset(node as isize);
    if newkey > oldkey {
        while i > 0 as libc::c_int as i64 {
            j = i - 1 as i64 >> 1;
            if !(newkey > (*heap.offset(j as isize)).key) {
                break;
            }
            *heap.offset(i as isize) = *heap.offset(j as isize);
            *locator.offset((*heap.offset(i as isize)).val as isize) = i;
            i = j;
        }
    } else {
        nnodes = (*queue).nnodes;
        loop {
            j = (i << 1) + 1 as i64;
            if !(j < nnodes) {
                break;
            }
            if (*heap.offset(j as isize)).key > newkey {
                if (j + 1 as i64) < nnodes
                    && (*heap.offset((j + 1 as i64) as isize)).key
                        > (*heap.offset(j as isize)).key
                {
                    j += 1;
                    j;
                }
                *heap.offset(i as isize) = *heap.offset(j as isize);
                *locator.offset((*heap.offset(i as isize)).val as isize) = i;
                i = j;
            } else {
                if !((j + 1 as i64) < nnodes
                    && (*heap.offset((j + 1 as i64) as isize)).key > newkey)
                {
                    break;
                }
                j += 1;
                j;
                *heap.offset(i as isize) = *heap.offset(j as isize);
                *locator.offset((*heap.offset(i as isize)).val as isize) = i;
                i = j;
            }
        }
    }
    (*heap.offset(i as isize)).key = newkey;
    (*heap.offset(i as isize)).val = node;
    *locator.offset(node as isize) = i;
}
#[no_mangle]
pub unsafe extern "C" fn gk_idxpqCreate(mut maxnodes: size_t) -> *mut gk_idxpq_t {
    let mut queue: *mut gk_idxpq_t = 0 as *mut gk_idxpq_t;
    queue = gk_malloc(
        ::core::mem::size_of::<gk_idxpq_t>() as u64,
        b"gk_pqCreate: queue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut gk_idxpq_t;
    gk_idxpqInit(queue, maxnodes);
    return queue;
}
#[no_mangle]
pub unsafe extern "C" fn gk_idxpqSeeKey(
    mut queue: *mut gk_idxpq_t,
    mut node: gk_idx_t,
) -> gk_idx_t {
    let mut locator: *mut gk_idx_t = 0 as *mut gk_idx_t;
    let mut heap: *mut gk_idxkv_t = 0 as *mut gk_idxkv_t;
    heap = (*queue).heap;
    locator = (*queue).locator;
    return (*heap.offset(*locator.offset(node as isize) as isize)).key;
}
#[no_mangle]
pub unsafe extern "C" fn gk_idxpqSeeTopKey(mut queue: *mut gk_idxpq_t) -> gk_idx_t {
    return (if (*queue).nnodes == 0 as libc::c_int as i64 {
        (0xFFFFFFFFFFFFFFFF as u64 >> 1).wrapping_sub(2 as libc::c_int as u64)
    } else {
        (*((*queue).heap).offset(0 as libc::c_int as isize)).key as u64
    }) as gk_idx_t;
}
#[no_mangle]
pub unsafe extern "C" fn gk_idxpqGetTop(mut queue: *mut gk_idxpq_t) -> gk_idx_t {
    let mut i: gk_idx_t = 0;
    let mut j: gk_idx_t = 0;
    let mut locator: *mut gk_idx_t = 0 as *mut gk_idx_t;
    let mut heap: *mut gk_idxkv_t = 0 as *mut gk_idxkv_t;
    let mut vtx: gk_idx_t = 0;
    let mut node: gk_idx_t = 0;
    let mut key: gk_idx_t = 0;
    if (*queue).nnodes == 0 as libc::c_int as i64 {
        return -(1) as gk_idx_t;
    }
    (*queue).nnodes -= 1;
    (*queue).nnodes;
    heap = (*queue).heap;
    locator = (*queue).locator;
    vtx = (*heap.offset(0 as libc::c_int as isize)).val;
    *locator.offset(vtx as isize) = -(1) as gk_idx_t;
    i = (*queue).nnodes;
    if i > 0 as libc::c_int as i64 {
        key = (*heap.offset(i as isize)).key;
        node = (*heap.offset(i as isize)).val;
        i = 0 as libc::c_int as gk_idx_t;
        loop {
            j = 2 as libc::c_int as i64 * i + 1 as i64;
            if !(j < (*queue).nnodes) {
                break;
            }
            if (*heap.offset(j as isize)).key > key {
                if (j + 1 as i64) < (*queue).nnodes
                    && (*heap.offset((j + 1 as i64) as isize)).key
                        > (*heap.offset(j as isize)).key
                {
                    j = j + 1 as i64;
                }
                *heap.offset(i as isize) = *heap.offset(j as isize);
                *locator.offset((*heap.offset(i as isize)).val as isize) = i;
                i = j;
            } else {
                if !((j + 1 as i64) < (*queue).nnodes
                    && (*heap.offset((j + 1 as i64) as isize)).key > key)
                {
                    break;
                }
                j = j + 1 as i64;
                *heap.offset(i as isize) = *heap.offset(j as isize);
                *locator.offset((*heap.offset(i as isize)).val as isize) = i;
                i = j;
            }
        }
        (*heap.offset(i as isize)).key = key;
        (*heap.offset(i as isize)).val = node;
        *locator.offset(node as isize) = i;
    }
    return vtx;
}
#[no_mangle]
pub unsafe extern "C" fn gk_idxpqSeeTopVal(mut queue: *mut gk_idxpq_t) -> gk_idx_t {
    return if (*queue).nnodes == 0 as libc::c_int as i64 {
        -(1) as i64
    } else {
        (*((*queue).heap).offset(0 as libc::c_int as isize)).val
    };
}
