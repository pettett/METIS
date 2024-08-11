use ::libc;
extern "C" {
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
}
pub type __int32_t = libc::c_int;
pub type __int64_t = i64;
pub type __ssize_t = i64;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type ssize_t = __ssize_t;
pub type size_t = u64;
pub type gk_idx_t = ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gk_ckv_t {
    pub key: libc::c_char,
    pub val: ssize_t,
}
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
pub struct gk_zkv_t {
    pub key: ssize_t,
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
pub struct gk_skv_t {
    pub key: *mut libc::c_char,
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
pub struct C2RustUnnamed {
    pub _hi: *mut libc::c_char,
    pub _lo: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub _hi: *mut libc::c_char,
    pub _lo: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub _hi: *mut libc::c_int,
    pub _lo: *mut libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub _hi: *mut libc::c_int,
    pub _lo: *mut libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub _hi: *mut libc::c_float,
    pub _lo: *mut libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub _hi: *mut libc::c_float,
    pub _lo: *mut libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub _hi: *mut libc::c_double,
    pub _lo: *mut libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub _hi: *mut libc::c_double,
    pub _lo: *mut libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub _hi: *mut gk_idx_t,
    pub _lo: *mut gk_idx_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub _hi: *mut gk_idx_t,
    pub _lo: *mut gk_idx_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub _hi: *mut gk_ckv_t,
    pub _lo: *mut gk_ckv_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub _hi: *mut gk_ckv_t,
    pub _lo: *mut gk_ckv_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub _hi: *mut gk_ikv_t,
    pub _lo: *mut gk_ikv_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub _hi: *mut gk_ikv_t,
    pub _lo: *mut gk_ikv_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub _hi: *mut gk_i32kv_t,
    pub _lo: *mut gk_i32kv_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub _hi: *mut gk_i32kv_t,
    pub _lo: *mut gk_i32kv_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub _hi: *mut gk_i64kv_t,
    pub _lo: *mut gk_i64kv_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
    pub _hi: *mut gk_i64kv_t,
    pub _lo: *mut gk_i64kv_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
    pub _hi: *mut gk_zkv_t,
    pub _lo: *mut gk_zkv_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_18 {
    pub _hi: *mut gk_zkv_t,
    pub _lo: *mut gk_zkv_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_19 {
    pub _hi: *mut gk_fkv_t,
    pub _lo: *mut gk_fkv_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_20 {
    pub _hi: *mut gk_fkv_t,
    pub _lo: *mut gk_fkv_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_21 {
    pub _hi: *mut gk_dkv_t,
    pub _lo: *mut gk_dkv_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_22 {
    pub _hi: *mut gk_dkv_t,
    pub _lo: *mut gk_dkv_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_23 {
    pub _hi: *mut gk_skv_t,
    pub _lo: *mut gk_skv_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_24 {
    pub _hi: *mut gk_skv_t,
    pub _lo: *mut gk_skv_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_25 {
    pub _hi: *mut gk_idxkv_t,
    pub _lo: *mut gk_idxkv_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_26 {
    pub _hi: *mut gk_idxkv_t,
    pub _lo: *mut gk_idxkv_t,
}
#[no_mangle]
pub unsafe extern "C" fn gk_csorti(mut n: size_t, mut base: *mut libc::c_char) {
    let _base: *mut libc::c_char = base;
    let _elems: size_t = n;
    let mut _hold: libc::c_char = 0;
    if _elems == 0 as libc::c_int as u64 {
        return;
    }
    if _elems > 4 as libc::c_int as u64 {
        let mut _lo: *mut libc::c_char = _base;
        let mut _hi: *mut libc::c_char = _lo
            .offset(_elems as isize)
            .offset(-(1 as isize));
        let mut _stack: [C2RustUnnamed; 64] = [C2RustUnnamed {
            _hi: 0 as *mut libc::c_char,
            _lo: 0 as *mut libc::c_char,
        }; 64];
        let mut _top: *mut C2RustUnnamed = _stack
            .as_mut_ptr()
            .offset(1 as isize);
        while _stack.as_mut_ptr() < _top {
            let mut _left_ptr: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut _right_ptr: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut _mid: *mut libc::c_char = _lo
                .offset(
                    (_hi.offset_from(_lo) as i64 >> 1) as isize,
                );
            if (*_mid as libc::c_int) < *_lo as libc::c_int {
                _hold = *_mid;
                *_mid = *_lo;
                *_lo = _hold;
            }
            if (*_hi as libc::c_int) < *_mid as libc::c_int {
                _hold = *_mid;
                *_mid = *_hi;
                *_hi = _hold;
                if (*_mid as libc::c_int) < *_lo as libc::c_int {
                    _hold = *_mid;
                    *_mid = *_lo;
                    *_lo = _hold;
                }
            }
            _left_ptr = _lo.offset(1 as isize);
            _right_ptr = _hi.offset(-(1 as isize));
            loop {
                while (*_left_ptr as libc::c_int) < *_mid as libc::c_int {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                }
                while (*_mid as libc::c_int) < *_right_ptr as libc::c_int {
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                }
                if _left_ptr < _right_ptr {
                    _hold = *_left_ptr;
                    *_left_ptr = *_right_ptr;
                    *_right_ptr = _hold;
                    if _mid == _left_ptr {
                        _mid = _right_ptr;
                    } else if _mid == _right_ptr {
                        _mid = _left_ptr;
                    }
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                } else if _left_ptr == _right_ptr {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                    break;
                }
                if !(_left_ptr <= _right_ptr) {
                    break;
                }
            }
            if _right_ptr.offset_from(_lo) as i64
                <= 4 as libc::c_int as i64
            {
                if _hi.offset_from(_left_ptr) as i64
                    <= 4 as libc::c_int as i64
                {
                    _top = _top.offset(-1);
                    _top;
                    _lo = (*_top)._lo;
                    _hi = (*_top)._hi;
                } else {
                    _lo = _left_ptr;
                }
            } else if _hi.offset_from(_left_ptr) as i64
                <= 4 as libc::c_int as i64
            {
                _hi = _right_ptr;
            } else if _right_ptr.offset_from(_lo) as i64
                > _hi.offset_from(_left_ptr) as i64
            {
                (*_top)._lo = _lo;
                (*_top)._hi = _right_ptr;
                _top = _top.offset(1);
                _top;
                _lo = _left_ptr;
            } else {
                (*_top)._lo = _left_ptr;
                (*_top)._hi = _hi;
                _top = _top.offset(1);
                _top;
                _hi = _right_ptr;
            }
        }
    }
    let _end_ptr: *mut libc::c_char = _base
        .offset(_elems as isize)
        .offset(-(1 as isize));
    let mut _tmp_ptr: *mut libc::c_char = _base;
    let mut _run_ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut _thresh: *mut libc::c_char = 0 as *mut libc::c_char;
    _thresh = _base.offset(4 as libc::c_int as isize);
    if _thresh > _end_ptr {
        _thresh = _end_ptr;
    }
    _run_ptr = _tmp_ptr.offset(1 as isize);
    while _run_ptr <= _thresh {
        if (*_run_ptr as libc::c_int) < *_tmp_ptr as libc::c_int {
            _tmp_ptr = _run_ptr;
        }
        _run_ptr = _run_ptr.offset(1);
        _run_ptr;
    }
    if _tmp_ptr != _base {
        _hold = *_tmp_ptr;
        *_tmp_ptr = *_base;
        *_base = _hold;
    }
    _run_ptr = _base.offset(1 as isize);
    loop {
        _run_ptr = _run_ptr.offset(1);
        if !(_run_ptr <= _end_ptr) {
            break;
        }
        _tmp_ptr = _run_ptr.offset(-(1 as isize));
        while (*_run_ptr as libc::c_int) < *_tmp_ptr as libc::c_int {
            _tmp_ptr = _tmp_ptr.offset(-1);
            _tmp_ptr;
        }
        _tmp_ptr = _tmp_ptr.offset(1);
        _tmp_ptr;
        if _tmp_ptr != _run_ptr {
            let mut _trav: *mut libc::c_char = _run_ptr
                .offset(1 as isize);
            loop {
                _trav = _trav.offset(-1);
                if !(_trav >= _run_ptr) {
                    break;
                }
                let mut _hi_0: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut _lo_0: *mut libc::c_char = 0 as *mut libc::c_char;
                _hold = *_trav;
                _lo_0 = _trav;
                _hi_0 = _lo_0;
                loop {
                    _lo_0 = _lo_0.offset(-1);
                    if !(_lo_0 >= _tmp_ptr) {
                        break;
                    }
                    *_hi_0 = *_lo_0;
                    _hi_0 = _lo_0;
                }
                *_hi_0 = _hold;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_csortd(mut n: size_t, mut base: *mut libc::c_char) {
    let _base: *mut libc::c_char = base;
    let _elems: size_t = n;
    let mut _hold: libc::c_char = 0;
    if _elems == 0 as libc::c_int as u64 {
        return;
    }
    if _elems > 4 as libc::c_int as u64 {
        let mut _lo: *mut libc::c_char = _base;
        let mut _hi: *mut libc::c_char = _lo
            .offset(_elems as isize)
            .offset(-(1 as isize));
        let mut _stack: [C2RustUnnamed_0; 64] = [C2RustUnnamed_0 {
            _hi: 0 as *mut libc::c_char,
            _lo: 0 as *mut libc::c_char,
        }; 64];
        let mut _top: *mut C2RustUnnamed_0 = _stack
            .as_mut_ptr()
            .offset(1 as isize);
        while _stack.as_mut_ptr() < _top {
            let mut _left_ptr: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut _right_ptr: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut _mid: *mut libc::c_char = _lo
                .offset(
                    (_hi.offset_from(_lo) as i64 >> 1) as isize,
                );
            if *_mid as libc::c_int > *_lo as libc::c_int {
                _hold = *_mid;
                *_mid = *_lo;
                *_lo = _hold;
            }
            if *_hi as libc::c_int > *_mid as libc::c_int {
                _hold = *_mid;
                *_mid = *_hi;
                *_hi = _hold;
                if *_mid as libc::c_int > *_lo as libc::c_int {
                    _hold = *_mid;
                    *_mid = *_lo;
                    *_lo = _hold;
                }
            }
            _left_ptr = _lo.offset(1 as isize);
            _right_ptr = _hi.offset(-(1 as isize));
            loop {
                while *_left_ptr as libc::c_int > *_mid as libc::c_int {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                }
                while *_mid as libc::c_int > *_right_ptr as libc::c_int {
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                }
                if _left_ptr < _right_ptr {
                    _hold = *_left_ptr;
                    *_left_ptr = *_right_ptr;
                    *_right_ptr = _hold;
                    if _mid == _left_ptr {
                        _mid = _right_ptr;
                    } else if _mid == _right_ptr {
                        _mid = _left_ptr;
                    }
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                } else if _left_ptr == _right_ptr {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                    break;
                }
                if !(_left_ptr <= _right_ptr) {
                    break;
                }
            }
            if _right_ptr.offset_from(_lo) as i64
                <= 4 as libc::c_int as i64
            {
                if _hi.offset_from(_left_ptr) as i64
                    <= 4 as libc::c_int as i64
                {
                    _top = _top.offset(-1);
                    _top;
                    _lo = (*_top)._lo;
                    _hi = (*_top)._hi;
                } else {
                    _lo = _left_ptr;
                }
            } else if _hi.offset_from(_left_ptr) as i64
                <= 4 as libc::c_int as i64
            {
                _hi = _right_ptr;
            } else if _right_ptr.offset_from(_lo) as i64
                > _hi.offset_from(_left_ptr) as i64
            {
                (*_top)._lo = _lo;
                (*_top)._hi = _right_ptr;
                _top = _top.offset(1);
                _top;
                _lo = _left_ptr;
            } else {
                (*_top)._lo = _left_ptr;
                (*_top)._hi = _hi;
                _top = _top.offset(1);
                _top;
                _hi = _right_ptr;
            }
        }
    }
    let _end_ptr: *mut libc::c_char = _base
        .offset(_elems as isize)
        .offset(-(1 as isize));
    let mut _tmp_ptr: *mut libc::c_char = _base;
    let mut _run_ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut _thresh: *mut libc::c_char = 0 as *mut libc::c_char;
    _thresh = _base.offset(4 as libc::c_int as isize);
    if _thresh > _end_ptr {
        _thresh = _end_ptr;
    }
    _run_ptr = _tmp_ptr.offset(1 as isize);
    while _run_ptr <= _thresh {
        if *_run_ptr as libc::c_int > *_tmp_ptr as libc::c_int {
            _tmp_ptr = _run_ptr;
        }
        _run_ptr = _run_ptr.offset(1);
        _run_ptr;
    }
    if _tmp_ptr != _base {
        _hold = *_tmp_ptr;
        *_tmp_ptr = *_base;
        *_base = _hold;
    }
    _run_ptr = _base.offset(1 as isize);
    loop {
        _run_ptr = _run_ptr.offset(1);
        if !(_run_ptr <= _end_ptr) {
            break;
        }
        _tmp_ptr = _run_ptr.offset(-(1 as isize));
        while *_run_ptr as libc::c_int > *_tmp_ptr as libc::c_int {
            _tmp_ptr = _tmp_ptr.offset(-1);
            _tmp_ptr;
        }
        _tmp_ptr = _tmp_ptr.offset(1);
        _tmp_ptr;
        if _tmp_ptr != _run_ptr {
            let mut _trav: *mut libc::c_char = _run_ptr
                .offset(1 as isize);
            loop {
                _trav = _trav.offset(-1);
                if !(_trav >= _run_ptr) {
                    break;
                }
                let mut _hi_0: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut _lo_0: *mut libc::c_char = 0 as *mut libc::c_char;
                _hold = *_trav;
                _lo_0 = _trav;
                _hi_0 = _lo_0;
                loop {
                    _lo_0 = _lo_0.offset(-1);
                    if !(_lo_0 >= _tmp_ptr) {
                        break;
                    }
                    *_hi_0 = *_lo_0;
                    _hi_0 = _lo_0;
                }
                *_hi_0 = _hold;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_isorti(mut n: size_t, mut base: *mut libc::c_int) {
    let _base: *mut libc::c_int = base;
    let _elems: size_t = n;
    let mut _hold: libc::c_int = 0;
    if _elems == 0 as libc::c_int as u64 {
        return;
    }
    if _elems > 4 as libc::c_int as u64 {
        let mut _lo: *mut libc::c_int = _base;
        let mut _hi: *mut libc::c_int = _lo
            .offset(_elems as isize)
            .offset(-(1 as isize));
        let mut _stack: [C2RustUnnamed_1; 64] = [C2RustUnnamed_1 {
            _hi: 0 as *mut libc::c_int,
            _lo: 0 as *mut libc::c_int,
        }; 64];
        let mut _top: *mut C2RustUnnamed_1 = _stack
            .as_mut_ptr()
            .offset(1 as isize);
        while _stack.as_mut_ptr() < _top {
            let mut _left_ptr: *mut libc::c_int = 0 as *mut libc::c_int;
            let mut _right_ptr: *mut libc::c_int = 0 as *mut libc::c_int;
            let mut _mid: *mut libc::c_int = _lo
                .offset(
                    (_hi.offset_from(_lo) as i64 >> 1) as isize,
                );
            if *_mid < *_lo {
                _hold = *_mid;
                *_mid = *_lo;
                *_lo = _hold;
            }
            if *_hi < *_mid {
                _hold = *_mid;
                *_mid = *_hi;
                *_hi = _hold;
                if *_mid < *_lo {
                    _hold = *_mid;
                    *_mid = *_lo;
                    *_lo = _hold;
                }
            }
            _left_ptr = _lo.offset(1 as isize);
            _right_ptr = _hi.offset(-(1 as isize));
            loop {
                while *_left_ptr < *_mid {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                }
                while *_mid < *_right_ptr {
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                }
                if _left_ptr < _right_ptr {
                    _hold = *_left_ptr;
                    *_left_ptr = *_right_ptr;
                    *_right_ptr = _hold;
                    if _mid == _left_ptr {
                        _mid = _right_ptr;
                    } else if _mid == _right_ptr {
                        _mid = _left_ptr;
                    }
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                } else if _left_ptr == _right_ptr {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                    break;
                }
                if !(_left_ptr <= _right_ptr) {
                    break;
                }
            }
            if _right_ptr.offset_from(_lo) as i64
                <= 4 as libc::c_int as i64
            {
                if _hi.offset_from(_left_ptr) as i64
                    <= 4 as libc::c_int as i64
                {
                    _top = _top.offset(-1);
                    _top;
                    _lo = (*_top)._lo;
                    _hi = (*_top)._hi;
                } else {
                    _lo = _left_ptr;
                }
            } else if _hi.offset_from(_left_ptr) as i64
                <= 4 as libc::c_int as i64
            {
                _hi = _right_ptr;
            } else if _right_ptr.offset_from(_lo) as i64
                > _hi.offset_from(_left_ptr) as i64
            {
                (*_top)._lo = _lo;
                (*_top)._hi = _right_ptr;
                _top = _top.offset(1);
                _top;
                _lo = _left_ptr;
            } else {
                (*_top)._lo = _left_ptr;
                (*_top)._hi = _hi;
                _top = _top.offset(1);
                _top;
                _hi = _right_ptr;
            }
        }
    }
    let _end_ptr: *mut libc::c_int = _base
        .offset(_elems as isize)
        .offset(-(1 as isize));
    let mut _tmp_ptr: *mut libc::c_int = _base;
    let mut _run_ptr: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut _thresh: *mut libc::c_int = 0 as *mut libc::c_int;
    _thresh = _base.offset(4 as libc::c_int as isize);
    if _thresh > _end_ptr {
        _thresh = _end_ptr;
    }
    _run_ptr = _tmp_ptr.offset(1 as isize);
    while _run_ptr <= _thresh {
        if *_run_ptr < *_tmp_ptr {
            _tmp_ptr = _run_ptr;
        }
        _run_ptr = _run_ptr.offset(1);
        _run_ptr;
    }
    if _tmp_ptr != _base {
        _hold = *_tmp_ptr;
        *_tmp_ptr = *_base;
        *_base = _hold;
    }
    _run_ptr = _base.offset(1 as isize);
    loop {
        _run_ptr = _run_ptr.offset(1);
        if !(_run_ptr <= _end_ptr) {
            break;
        }
        _tmp_ptr = _run_ptr.offset(-(1 as isize));
        while *_run_ptr < *_tmp_ptr {
            _tmp_ptr = _tmp_ptr.offset(-1);
            _tmp_ptr;
        }
        _tmp_ptr = _tmp_ptr.offset(1);
        _tmp_ptr;
        if _tmp_ptr != _run_ptr {
            let mut _trav: *mut libc::c_int = _run_ptr.offset(1 as isize);
            loop {
                _trav = _trav.offset(-1);
                if !(_trav >= _run_ptr) {
                    break;
                }
                let mut _hi_0: *mut libc::c_int = 0 as *mut libc::c_int;
                let mut _lo_0: *mut libc::c_int = 0 as *mut libc::c_int;
                _hold = *_trav;
                _lo_0 = _trav;
                _hi_0 = _lo_0;
                loop {
                    _lo_0 = _lo_0.offset(-1);
                    if !(_lo_0 >= _tmp_ptr) {
                        break;
                    }
                    *_hi_0 = *_lo_0;
                    _hi_0 = _lo_0;
                }
                *_hi_0 = _hold;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_isortd(mut n: size_t, mut base: *mut libc::c_int) {
    let _base: *mut libc::c_int = base;
    let _elems: size_t = n;
    let mut _hold: libc::c_int = 0;
    if _elems == 0 as libc::c_int as u64 {
        return;
    }
    if _elems > 4 as libc::c_int as u64 {
        let mut _lo: *mut libc::c_int = _base;
        let mut _hi: *mut libc::c_int = _lo
            .offset(_elems as isize)
            .offset(-(1 as isize));
        let mut _stack: [C2RustUnnamed_2; 64] = [C2RustUnnamed_2 {
            _hi: 0 as *mut libc::c_int,
            _lo: 0 as *mut libc::c_int,
        }; 64];
        let mut _top: *mut C2RustUnnamed_2 = _stack
            .as_mut_ptr()
            .offset(1 as isize);
        while _stack.as_mut_ptr() < _top {
            let mut _left_ptr: *mut libc::c_int = 0 as *mut libc::c_int;
            let mut _right_ptr: *mut libc::c_int = 0 as *mut libc::c_int;
            let mut _mid: *mut libc::c_int = _lo
                .offset(
                    (_hi.offset_from(_lo) as i64 >> 1) as isize,
                );
            if *_mid > *_lo {
                _hold = *_mid;
                *_mid = *_lo;
                *_lo = _hold;
            }
            if *_hi > *_mid {
                _hold = *_mid;
                *_mid = *_hi;
                *_hi = _hold;
                if *_mid > *_lo {
                    _hold = *_mid;
                    *_mid = *_lo;
                    *_lo = _hold;
                }
            }
            _left_ptr = _lo.offset(1 as isize);
            _right_ptr = _hi.offset(-(1 as isize));
            loop {
                while *_left_ptr > *_mid {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                }
                while *_mid > *_right_ptr {
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                }
                if _left_ptr < _right_ptr {
                    _hold = *_left_ptr;
                    *_left_ptr = *_right_ptr;
                    *_right_ptr = _hold;
                    if _mid == _left_ptr {
                        _mid = _right_ptr;
                    } else if _mid == _right_ptr {
                        _mid = _left_ptr;
                    }
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                } else if _left_ptr == _right_ptr {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                    break;
                }
                if !(_left_ptr <= _right_ptr) {
                    break;
                }
            }
            if _right_ptr.offset_from(_lo) as i64
                <= 4 as libc::c_int as i64
            {
                if _hi.offset_from(_left_ptr) as i64
                    <= 4 as libc::c_int as i64
                {
                    _top = _top.offset(-1);
                    _top;
                    _lo = (*_top)._lo;
                    _hi = (*_top)._hi;
                } else {
                    _lo = _left_ptr;
                }
            } else if _hi.offset_from(_left_ptr) as i64
                <= 4 as libc::c_int as i64
            {
                _hi = _right_ptr;
            } else if _right_ptr.offset_from(_lo) as i64
                > _hi.offset_from(_left_ptr) as i64
            {
                (*_top)._lo = _lo;
                (*_top)._hi = _right_ptr;
                _top = _top.offset(1);
                _top;
                _lo = _left_ptr;
            } else {
                (*_top)._lo = _left_ptr;
                (*_top)._hi = _hi;
                _top = _top.offset(1);
                _top;
                _hi = _right_ptr;
            }
        }
    }
    let _end_ptr: *mut libc::c_int = _base
        .offset(_elems as isize)
        .offset(-(1 as isize));
    let mut _tmp_ptr: *mut libc::c_int = _base;
    let mut _run_ptr: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut _thresh: *mut libc::c_int = 0 as *mut libc::c_int;
    _thresh = _base.offset(4 as libc::c_int as isize);
    if _thresh > _end_ptr {
        _thresh = _end_ptr;
    }
    _run_ptr = _tmp_ptr.offset(1 as isize);
    while _run_ptr <= _thresh {
        if *_run_ptr > *_tmp_ptr {
            _tmp_ptr = _run_ptr;
        }
        _run_ptr = _run_ptr.offset(1);
        _run_ptr;
    }
    if _tmp_ptr != _base {
        _hold = *_tmp_ptr;
        *_tmp_ptr = *_base;
        *_base = _hold;
    }
    _run_ptr = _base.offset(1 as isize);
    loop {
        _run_ptr = _run_ptr.offset(1);
        if !(_run_ptr <= _end_ptr) {
            break;
        }
        _tmp_ptr = _run_ptr.offset(-(1 as isize));
        while *_run_ptr > *_tmp_ptr {
            _tmp_ptr = _tmp_ptr.offset(-1);
            _tmp_ptr;
        }
        _tmp_ptr = _tmp_ptr.offset(1);
        _tmp_ptr;
        if _tmp_ptr != _run_ptr {
            let mut _trav: *mut libc::c_int = _run_ptr.offset(1 as isize);
            loop {
                _trav = _trav.offset(-1);
                if !(_trav >= _run_ptr) {
                    break;
                }
                let mut _hi_0: *mut libc::c_int = 0 as *mut libc::c_int;
                let mut _lo_0: *mut libc::c_int = 0 as *mut libc::c_int;
                _hold = *_trav;
                _lo_0 = _trav;
                _hi_0 = _lo_0;
                loop {
                    _lo_0 = _lo_0.offset(-1);
                    if !(_lo_0 >= _tmp_ptr) {
                        break;
                    }
                    *_hi_0 = *_lo_0;
                    _hi_0 = _lo_0;
                }
                *_hi_0 = _hold;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_fsorti(mut n: size_t, mut base: *mut libc::c_float) {
    let _base: *mut libc::c_float = base;
    let _elems: size_t = n;
    let mut _hold: libc::c_float = 0.;
    if _elems == 0 as libc::c_int as u64 {
        return;
    }
    if _elems > 4 as libc::c_int as u64 {
        let mut _lo: *mut libc::c_float = _base;
        let mut _hi: *mut libc::c_float = _lo
            .offset(_elems as isize)
            .offset(-(1 as isize));
        let mut _stack: [C2RustUnnamed_3; 64] = [C2RustUnnamed_3 {
            _hi: 0 as *mut libc::c_float,
            _lo: 0 as *mut libc::c_float,
        }; 64];
        let mut _top: *mut C2RustUnnamed_3 = _stack
            .as_mut_ptr()
            .offset(1 as isize);
        while _stack.as_mut_ptr() < _top {
            let mut _left_ptr: *mut libc::c_float = 0 as *mut libc::c_float;
            let mut _right_ptr: *mut libc::c_float = 0 as *mut libc::c_float;
            let mut _mid: *mut libc::c_float = _lo
                .offset(
                    (_hi.offset_from(_lo) as i64 >> 1) as isize,
                );
            if *_mid < *_lo {
                _hold = *_mid;
                *_mid = *_lo;
                *_lo = _hold;
            }
            if *_hi < *_mid {
                _hold = *_mid;
                *_mid = *_hi;
                *_hi = _hold;
                if *_mid < *_lo {
                    _hold = *_mid;
                    *_mid = *_lo;
                    *_lo = _hold;
                }
            }
            _left_ptr = _lo.offset(1 as isize);
            _right_ptr = _hi.offset(-(1 as isize));
            loop {
                while *_left_ptr < *_mid {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                }
                while *_mid < *_right_ptr {
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                }
                if _left_ptr < _right_ptr {
                    _hold = *_left_ptr;
                    *_left_ptr = *_right_ptr;
                    *_right_ptr = _hold;
                    if _mid == _left_ptr {
                        _mid = _right_ptr;
                    } else if _mid == _right_ptr {
                        _mid = _left_ptr;
                    }
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                } else if _left_ptr == _right_ptr {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                    break;
                }
                if !(_left_ptr <= _right_ptr) {
                    break;
                }
            }
            if _right_ptr.offset_from(_lo) as i64
                <= 4 as libc::c_int as i64
            {
                if _hi.offset_from(_left_ptr) as i64
                    <= 4 as libc::c_int as i64
                {
                    _top = _top.offset(-1);
                    _top;
                    _lo = (*_top)._lo;
                    _hi = (*_top)._hi;
                } else {
                    _lo = _left_ptr;
                }
            } else if _hi.offset_from(_left_ptr) as i64
                <= 4 as libc::c_int as i64
            {
                _hi = _right_ptr;
            } else if _right_ptr.offset_from(_lo) as i64
                > _hi.offset_from(_left_ptr) as i64
            {
                (*_top)._lo = _lo;
                (*_top)._hi = _right_ptr;
                _top = _top.offset(1);
                _top;
                _lo = _left_ptr;
            } else {
                (*_top)._lo = _left_ptr;
                (*_top)._hi = _hi;
                _top = _top.offset(1);
                _top;
                _hi = _right_ptr;
            }
        }
    }
    let _end_ptr: *mut libc::c_float = _base
        .offset(_elems as isize)
        .offset(-(1 as isize));
    let mut _tmp_ptr: *mut libc::c_float = _base;
    let mut _run_ptr: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut _thresh: *mut libc::c_float = 0 as *mut libc::c_float;
    _thresh = _base.offset(4 as libc::c_int as isize);
    if _thresh > _end_ptr {
        _thresh = _end_ptr;
    }
    _run_ptr = _tmp_ptr.offset(1 as isize);
    while _run_ptr <= _thresh {
        if *_run_ptr < *_tmp_ptr {
            _tmp_ptr = _run_ptr;
        }
        _run_ptr = _run_ptr.offset(1);
        _run_ptr;
    }
    if _tmp_ptr != _base {
        _hold = *_tmp_ptr;
        *_tmp_ptr = *_base;
        *_base = _hold;
    }
    _run_ptr = _base.offset(1 as isize);
    loop {
        _run_ptr = _run_ptr.offset(1);
        if !(_run_ptr <= _end_ptr) {
            break;
        }
        _tmp_ptr = _run_ptr.offset(-(1 as isize));
        while *_run_ptr < *_tmp_ptr {
            _tmp_ptr = _tmp_ptr.offset(-1);
            _tmp_ptr;
        }
        _tmp_ptr = _tmp_ptr.offset(1);
        _tmp_ptr;
        if _tmp_ptr != _run_ptr {
            let mut _trav: *mut libc::c_float = _run_ptr
                .offset(1 as isize);
            loop {
                _trav = _trav.offset(-1);
                if !(_trav >= _run_ptr) {
                    break;
                }
                let mut _hi_0: *mut libc::c_float = 0 as *mut libc::c_float;
                let mut _lo_0: *mut libc::c_float = 0 as *mut libc::c_float;
                _hold = *_trav;
                _lo_0 = _trav;
                _hi_0 = _lo_0;
                loop {
                    _lo_0 = _lo_0.offset(-1);
                    if !(_lo_0 >= _tmp_ptr) {
                        break;
                    }
                    *_hi_0 = *_lo_0;
                    _hi_0 = _lo_0;
                }
                *_hi_0 = _hold;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_fsortd(mut n: size_t, mut base: *mut libc::c_float) {
    let _base: *mut libc::c_float = base;
    let _elems: size_t = n;
    let mut _hold: libc::c_float = 0.;
    if _elems == 0 as libc::c_int as u64 {
        return;
    }
    if _elems > 4 as libc::c_int as u64 {
        let mut _lo: *mut libc::c_float = _base;
        let mut _hi: *mut libc::c_float = _lo
            .offset(_elems as isize)
            .offset(-(1 as isize));
        let mut _stack: [C2RustUnnamed_4; 64] = [C2RustUnnamed_4 {
            _hi: 0 as *mut libc::c_float,
            _lo: 0 as *mut libc::c_float,
        }; 64];
        let mut _top: *mut C2RustUnnamed_4 = _stack
            .as_mut_ptr()
            .offset(1 as isize);
        while _stack.as_mut_ptr() < _top {
            let mut _left_ptr: *mut libc::c_float = 0 as *mut libc::c_float;
            let mut _right_ptr: *mut libc::c_float = 0 as *mut libc::c_float;
            let mut _mid: *mut libc::c_float = _lo
                .offset(
                    (_hi.offset_from(_lo) as i64 >> 1) as isize,
                );
            if *_mid > *_lo {
                _hold = *_mid;
                *_mid = *_lo;
                *_lo = _hold;
            }
            if *_hi > *_mid {
                _hold = *_mid;
                *_mid = *_hi;
                *_hi = _hold;
                if *_mid > *_lo {
                    _hold = *_mid;
                    *_mid = *_lo;
                    *_lo = _hold;
                }
            }
            _left_ptr = _lo.offset(1 as isize);
            _right_ptr = _hi.offset(-(1 as isize));
            loop {
                while *_left_ptr > *_mid {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                }
                while *_mid > *_right_ptr {
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                }
                if _left_ptr < _right_ptr {
                    _hold = *_left_ptr;
                    *_left_ptr = *_right_ptr;
                    *_right_ptr = _hold;
                    if _mid == _left_ptr {
                        _mid = _right_ptr;
                    } else if _mid == _right_ptr {
                        _mid = _left_ptr;
                    }
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                } else if _left_ptr == _right_ptr {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                    break;
                }
                if !(_left_ptr <= _right_ptr) {
                    break;
                }
            }
            if _right_ptr.offset_from(_lo) as i64
                <= 4 as libc::c_int as i64
            {
                if _hi.offset_from(_left_ptr) as i64
                    <= 4 as libc::c_int as i64
                {
                    _top = _top.offset(-1);
                    _top;
                    _lo = (*_top)._lo;
                    _hi = (*_top)._hi;
                } else {
                    _lo = _left_ptr;
                }
            } else if _hi.offset_from(_left_ptr) as i64
                <= 4 as libc::c_int as i64
            {
                _hi = _right_ptr;
            } else if _right_ptr.offset_from(_lo) as i64
                > _hi.offset_from(_left_ptr) as i64
            {
                (*_top)._lo = _lo;
                (*_top)._hi = _right_ptr;
                _top = _top.offset(1);
                _top;
                _lo = _left_ptr;
            } else {
                (*_top)._lo = _left_ptr;
                (*_top)._hi = _hi;
                _top = _top.offset(1);
                _top;
                _hi = _right_ptr;
            }
        }
    }
    let _end_ptr: *mut libc::c_float = _base
        .offset(_elems as isize)
        .offset(-(1 as isize));
    let mut _tmp_ptr: *mut libc::c_float = _base;
    let mut _run_ptr: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut _thresh: *mut libc::c_float = 0 as *mut libc::c_float;
    _thresh = _base.offset(4 as libc::c_int as isize);
    if _thresh > _end_ptr {
        _thresh = _end_ptr;
    }
    _run_ptr = _tmp_ptr.offset(1 as isize);
    while _run_ptr <= _thresh {
        if *_run_ptr > *_tmp_ptr {
            _tmp_ptr = _run_ptr;
        }
        _run_ptr = _run_ptr.offset(1);
        _run_ptr;
    }
    if _tmp_ptr != _base {
        _hold = *_tmp_ptr;
        *_tmp_ptr = *_base;
        *_base = _hold;
    }
    _run_ptr = _base.offset(1 as isize);
    loop {
        _run_ptr = _run_ptr.offset(1);
        if !(_run_ptr <= _end_ptr) {
            break;
        }
        _tmp_ptr = _run_ptr.offset(-(1 as isize));
        while *_run_ptr > *_tmp_ptr {
            _tmp_ptr = _tmp_ptr.offset(-1);
            _tmp_ptr;
        }
        _tmp_ptr = _tmp_ptr.offset(1);
        _tmp_ptr;
        if _tmp_ptr != _run_ptr {
            let mut _trav: *mut libc::c_float = _run_ptr
                .offset(1 as isize);
            loop {
                _trav = _trav.offset(-1);
                if !(_trav >= _run_ptr) {
                    break;
                }
                let mut _hi_0: *mut libc::c_float = 0 as *mut libc::c_float;
                let mut _lo_0: *mut libc::c_float = 0 as *mut libc::c_float;
                _hold = *_trav;
                _lo_0 = _trav;
                _hi_0 = _lo_0;
                loop {
                    _lo_0 = _lo_0.offset(-1);
                    if !(_lo_0 >= _tmp_ptr) {
                        break;
                    }
                    *_hi_0 = *_lo_0;
                    _hi_0 = _lo_0;
                }
                *_hi_0 = _hold;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_dsorti(mut n: size_t, mut base: *mut libc::c_double) {
    let _base: *mut libc::c_double = base;
    let _elems: size_t = n;
    let mut _hold: libc::c_double = 0.;
    if _elems == 0 as libc::c_int as u64 {
        return;
    }
    if _elems > 4 as libc::c_int as u64 {
        let mut _lo: *mut libc::c_double = _base;
        let mut _hi: *mut libc::c_double = _lo
            .offset(_elems as isize)
            .offset(-(1 as isize));
        let mut _stack: [C2RustUnnamed_5; 64] = [C2RustUnnamed_5 {
            _hi: 0 as *mut libc::c_double,
            _lo: 0 as *mut libc::c_double,
        }; 64];
        let mut _top: *mut C2RustUnnamed_5 = _stack
            .as_mut_ptr()
            .offset(1 as isize);
        while _stack.as_mut_ptr() < _top {
            let mut _left_ptr: *mut libc::c_double = 0 as *mut libc::c_double;
            let mut _right_ptr: *mut libc::c_double = 0 as *mut libc::c_double;
            let mut _mid: *mut libc::c_double = _lo
                .offset(
                    (_hi.offset_from(_lo) as i64 >> 1) as isize,
                );
            if *_mid < *_lo {
                _hold = *_mid;
                *_mid = *_lo;
                *_lo = _hold;
            }
            if *_hi < *_mid {
                _hold = *_mid;
                *_mid = *_hi;
                *_hi = _hold;
                if *_mid < *_lo {
                    _hold = *_mid;
                    *_mid = *_lo;
                    *_lo = _hold;
                }
            }
            _left_ptr = _lo.offset(1 as isize);
            _right_ptr = _hi.offset(-(1 as isize));
            loop {
                while *_left_ptr < *_mid {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                }
                while *_mid < *_right_ptr {
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                }
                if _left_ptr < _right_ptr {
                    _hold = *_left_ptr;
                    *_left_ptr = *_right_ptr;
                    *_right_ptr = _hold;
                    if _mid == _left_ptr {
                        _mid = _right_ptr;
                    } else if _mid == _right_ptr {
                        _mid = _left_ptr;
                    }
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                } else if _left_ptr == _right_ptr {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                    break;
                }
                if !(_left_ptr <= _right_ptr) {
                    break;
                }
            }
            if _right_ptr.offset_from(_lo) as i64
                <= 4 as libc::c_int as i64
            {
                if _hi.offset_from(_left_ptr) as i64
                    <= 4 as libc::c_int as i64
                {
                    _top = _top.offset(-1);
                    _top;
                    _lo = (*_top)._lo;
                    _hi = (*_top)._hi;
                } else {
                    _lo = _left_ptr;
                }
            } else if _hi.offset_from(_left_ptr) as i64
                <= 4 as libc::c_int as i64
            {
                _hi = _right_ptr;
            } else if _right_ptr.offset_from(_lo) as i64
                > _hi.offset_from(_left_ptr) as i64
            {
                (*_top)._lo = _lo;
                (*_top)._hi = _right_ptr;
                _top = _top.offset(1);
                _top;
                _lo = _left_ptr;
            } else {
                (*_top)._lo = _left_ptr;
                (*_top)._hi = _hi;
                _top = _top.offset(1);
                _top;
                _hi = _right_ptr;
            }
        }
    }
    let _end_ptr: *mut libc::c_double = _base
        .offset(_elems as isize)
        .offset(-(1 as isize));
    let mut _tmp_ptr: *mut libc::c_double = _base;
    let mut _run_ptr: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut _thresh: *mut libc::c_double = 0 as *mut libc::c_double;
    _thresh = _base.offset(4 as libc::c_int as isize);
    if _thresh > _end_ptr {
        _thresh = _end_ptr;
    }
    _run_ptr = _tmp_ptr.offset(1 as isize);
    while _run_ptr <= _thresh {
        if *_run_ptr < *_tmp_ptr {
            _tmp_ptr = _run_ptr;
        }
        _run_ptr = _run_ptr.offset(1);
        _run_ptr;
    }
    if _tmp_ptr != _base {
        _hold = *_tmp_ptr;
        *_tmp_ptr = *_base;
        *_base = _hold;
    }
    _run_ptr = _base.offset(1 as isize);
    loop {
        _run_ptr = _run_ptr.offset(1);
        if !(_run_ptr <= _end_ptr) {
            break;
        }
        _tmp_ptr = _run_ptr.offset(-(1 as isize));
        while *_run_ptr < *_tmp_ptr {
            _tmp_ptr = _tmp_ptr.offset(-1);
            _tmp_ptr;
        }
        _tmp_ptr = _tmp_ptr.offset(1);
        _tmp_ptr;
        if _tmp_ptr != _run_ptr {
            let mut _trav: *mut libc::c_double = _run_ptr
                .offset(1 as isize);
            loop {
                _trav = _trav.offset(-1);
                if !(_trav >= _run_ptr) {
                    break;
                }
                let mut _hi_0: *mut libc::c_double = 0 as *mut libc::c_double;
                let mut _lo_0: *mut libc::c_double = 0 as *mut libc::c_double;
                _hold = *_trav;
                _lo_0 = _trav;
                _hi_0 = _lo_0;
                loop {
                    _lo_0 = _lo_0.offset(-1);
                    if !(_lo_0 >= _tmp_ptr) {
                        break;
                    }
                    *_hi_0 = *_lo_0;
                    _hi_0 = _lo_0;
                }
                *_hi_0 = _hold;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_dsortd(mut n: size_t, mut base: *mut libc::c_double) {
    let _base: *mut libc::c_double = base;
    let _elems: size_t = n;
    let mut _hold: libc::c_double = 0.;
    if _elems == 0 as libc::c_int as u64 {
        return;
    }
    if _elems > 4 as libc::c_int as u64 {
        let mut _lo: *mut libc::c_double = _base;
        let mut _hi: *mut libc::c_double = _lo
            .offset(_elems as isize)
            .offset(-(1 as isize));
        let mut _stack: [C2RustUnnamed_6; 64] = [C2RustUnnamed_6 {
            _hi: 0 as *mut libc::c_double,
            _lo: 0 as *mut libc::c_double,
        }; 64];
        let mut _top: *mut C2RustUnnamed_6 = _stack
            .as_mut_ptr()
            .offset(1 as isize);
        while _stack.as_mut_ptr() < _top {
            let mut _left_ptr: *mut libc::c_double = 0 as *mut libc::c_double;
            let mut _right_ptr: *mut libc::c_double = 0 as *mut libc::c_double;
            let mut _mid: *mut libc::c_double = _lo
                .offset(
                    (_hi.offset_from(_lo) as i64 >> 1) as isize,
                );
            if *_mid > *_lo {
                _hold = *_mid;
                *_mid = *_lo;
                *_lo = _hold;
            }
            if *_hi > *_mid {
                _hold = *_mid;
                *_mid = *_hi;
                *_hi = _hold;
                if *_mid > *_lo {
                    _hold = *_mid;
                    *_mid = *_lo;
                    *_lo = _hold;
                }
            }
            _left_ptr = _lo.offset(1 as isize);
            _right_ptr = _hi.offset(-(1 as isize));
            loop {
                while *_left_ptr > *_mid {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                }
                while *_mid > *_right_ptr {
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                }
                if _left_ptr < _right_ptr {
                    _hold = *_left_ptr;
                    *_left_ptr = *_right_ptr;
                    *_right_ptr = _hold;
                    if _mid == _left_ptr {
                        _mid = _right_ptr;
                    } else if _mid == _right_ptr {
                        _mid = _left_ptr;
                    }
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                } else if _left_ptr == _right_ptr {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                    break;
                }
                if !(_left_ptr <= _right_ptr) {
                    break;
                }
            }
            if _right_ptr.offset_from(_lo) as i64
                <= 4 as libc::c_int as i64
            {
                if _hi.offset_from(_left_ptr) as i64
                    <= 4 as libc::c_int as i64
                {
                    _top = _top.offset(-1);
                    _top;
                    _lo = (*_top)._lo;
                    _hi = (*_top)._hi;
                } else {
                    _lo = _left_ptr;
                }
            } else if _hi.offset_from(_left_ptr) as i64
                <= 4 as libc::c_int as i64
            {
                _hi = _right_ptr;
            } else if _right_ptr.offset_from(_lo) as i64
                > _hi.offset_from(_left_ptr) as i64
            {
                (*_top)._lo = _lo;
                (*_top)._hi = _right_ptr;
                _top = _top.offset(1);
                _top;
                _lo = _left_ptr;
            } else {
                (*_top)._lo = _left_ptr;
                (*_top)._hi = _hi;
                _top = _top.offset(1);
                _top;
                _hi = _right_ptr;
            }
        }
    }
    let _end_ptr: *mut libc::c_double = _base
        .offset(_elems as isize)
        .offset(-(1 as isize));
    let mut _tmp_ptr: *mut libc::c_double = _base;
    let mut _run_ptr: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut _thresh: *mut libc::c_double = 0 as *mut libc::c_double;
    _thresh = _base.offset(4 as libc::c_int as isize);
    if _thresh > _end_ptr {
        _thresh = _end_ptr;
    }
    _run_ptr = _tmp_ptr.offset(1 as isize);
    while _run_ptr <= _thresh {
        if *_run_ptr > *_tmp_ptr {
            _tmp_ptr = _run_ptr;
        }
        _run_ptr = _run_ptr.offset(1);
        _run_ptr;
    }
    if _tmp_ptr != _base {
        _hold = *_tmp_ptr;
        *_tmp_ptr = *_base;
        *_base = _hold;
    }
    _run_ptr = _base.offset(1 as isize);
    loop {
        _run_ptr = _run_ptr.offset(1);
        if !(_run_ptr <= _end_ptr) {
            break;
        }
        _tmp_ptr = _run_ptr.offset(-(1 as isize));
        while *_run_ptr > *_tmp_ptr {
            _tmp_ptr = _tmp_ptr.offset(-1);
            _tmp_ptr;
        }
        _tmp_ptr = _tmp_ptr.offset(1);
        _tmp_ptr;
        if _tmp_ptr != _run_ptr {
            let mut _trav: *mut libc::c_double = _run_ptr
                .offset(1 as isize);
            loop {
                _trav = _trav.offset(-1);
                if !(_trav >= _run_ptr) {
                    break;
                }
                let mut _hi_0: *mut libc::c_double = 0 as *mut libc::c_double;
                let mut _lo_0: *mut libc::c_double = 0 as *mut libc::c_double;
                _hold = *_trav;
                _lo_0 = _trav;
                _hi_0 = _lo_0;
                loop {
                    _lo_0 = _lo_0.offset(-1);
                    if !(_lo_0 >= _tmp_ptr) {
                        break;
                    }
                    *_hi_0 = *_lo_0;
                    _hi_0 = _lo_0;
                }
                *_hi_0 = _hold;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_idxsorti(mut n: size_t, mut base: *mut gk_idx_t) {
    let _base: *mut gk_idx_t = base;
    let _elems: size_t = n;
    let mut _hold: gk_idx_t = 0;
    if _elems == 0 as libc::c_int as u64 {
        return;
    }
    if _elems > 4 as libc::c_int as u64 {
        let mut _lo: *mut gk_idx_t = _base;
        let mut _hi: *mut gk_idx_t = _lo
            .offset(_elems as isize)
            .offset(-(1 as isize));
        let mut _stack: [C2RustUnnamed_7; 64] = [C2RustUnnamed_7 {
            _hi: 0 as *mut gk_idx_t,
            _lo: 0 as *mut gk_idx_t,
        }; 64];
        let mut _top: *mut C2RustUnnamed_7 = _stack
            .as_mut_ptr()
            .offset(1 as isize);
        while _stack.as_mut_ptr() < _top {
            let mut _left_ptr: *mut gk_idx_t = 0 as *mut gk_idx_t;
            let mut _right_ptr: *mut gk_idx_t = 0 as *mut gk_idx_t;
            let mut _mid: *mut gk_idx_t = _lo
                .offset(
                    (_hi.offset_from(_lo) as i64 >> 1) as isize,
                );
            if *_mid < *_lo {
                _hold = *_mid;
                *_mid = *_lo;
                *_lo = _hold;
            }
            if *_hi < *_mid {
                _hold = *_mid;
                *_mid = *_hi;
                *_hi = _hold;
                if *_mid < *_lo {
                    _hold = *_mid;
                    *_mid = *_lo;
                    *_lo = _hold;
                }
            }
            _left_ptr = _lo.offset(1 as isize);
            _right_ptr = _hi.offset(-(1 as isize));
            loop {
                while *_left_ptr < *_mid {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                }
                while *_mid < *_right_ptr {
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                }
                if _left_ptr < _right_ptr {
                    _hold = *_left_ptr;
                    *_left_ptr = *_right_ptr;
                    *_right_ptr = _hold;
                    if _mid == _left_ptr {
                        _mid = _right_ptr;
                    } else if _mid == _right_ptr {
                        _mid = _left_ptr;
                    }
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                } else if _left_ptr == _right_ptr {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                    break;
                }
                if !(_left_ptr <= _right_ptr) {
                    break;
                }
            }
            if _right_ptr.offset_from(_lo) as i64
                <= 4 as libc::c_int as i64
            {
                if _hi.offset_from(_left_ptr) as i64
                    <= 4 as libc::c_int as i64
                {
                    _top = _top.offset(-1);
                    _top;
                    _lo = (*_top)._lo;
                    _hi = (*_top)._hi;
                } else {
                    _lo = _left_ptr;
                }
            } else if _hi.offset_from(_left_ptr) as i64
                <= 4 as libc::c_int as i64
            {
                _hi = _right_ptr;
            } else if _right_ptr.offset_from(_lo) as i64
                > _hi.offset_from(_left_ptr) as i64
            {
                (*_top)._lo = _lo;
                (*_top)._hi = _right_ptr;
                _top = _top.offset(1);
                _top;
                _lo = _left_ptr;
            } else {
                (*_top)._lo = _left_ptr;
                (*_top)._hi = _hi;
                _top = _top.offset(1);
                _top;
                _hi = _right_ptr;
            }
        }
    }
    let _end_ptr: *mut gk_idx_t = _base
        .offset(_elems as isize)
        .offset(-(1 as isize));
    let mut _tmp_ptr: *mut gk_idx_t = _base;
    let mut _run_ptr: *mut gk_idx_t = 0 as *mut gk_idx_t;
    let mut _thresh: *mut gk_idx_t = 0 as *mut gk_idx_t;
    _thresh = _base.offset(4 as libc::c_int as isize);
    if _thresh > _end_ptr {
        _thresh = _end_ptr;
    }
    _run_ptr = _tmp_ptr.offset(1 as isize);
    while _run_ptr <= _thresh {
        if *_run_ptr < *_tmp_ptr {
            _tmp_ptr = _run_ptr;
        }
        _run_ptr = _run_ptr.offset(1);
        _run_ptr;
    }
    if _tmp_ptr != _base {
        _hold = *_tmp_ptr;
        *_tmp_ptr = *_base;
        *_base = _hold;
    }
    _run_ptr = _base.offset(1 as isize);
    loop {
        _run_ptr = _run_ptr.offset(1);
        if !(_run_ptr <= _end_ptr) {
            break;
        }
        _tmp_ptr = _run_ptr.offset(-(1 as isize));
        while *_run_ptr < *_tmp_ptr {
            _tmp_ptr = _tmp_ptr.offset(-1);
            _tmp_ptr;
        }
        _tmp_ptr = _tmp_ptr.offset(1);
        _tmp_ptr;
        if _tmp_ptr != _run_ptr {
            let mut _trav: *mut gk_idx_t = _run_ptr.offset(1 as isize);
            loop {
                _trav = _trav.offset(-1);
                if !(_trav >= _run_ptr) {
                    break;
                }
                let mut _hi_0: *mut gk_idx_t = 0 as *mut gk_idx_t;
                let mut _lo_0: *mut gk_idx_t = 0 as *mut gk_idx_t;
                _hold = *_trav;
                _lo_0 = _trav;
                _hi_0 = _lo_0;
                loop {
                    _lo_0 = _lo_0.offset(-1);
                    if !(_lo_0 >= _tmp_ptr) {
                        break;
                    }
                    *_hi_0 = *_lo_0;
                    _hi_0 = _lo_0;
                }
                *_hi_0 = _hold;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_idxsortd(mut n: size_t, mut base: *mut gk_idx_t) {
    let _base: *mut gk_idx_t = base;
    let _elems: size_t = n;
    let mut _hold: gk_idx_t = 0;
    if _elems == 0 as libc::c_int as u64 {
        return;
    }
    if _elems > 4 as libc::c_int as u64 {
        let mut _lo: *mut gk_idx_t = _base;
        let mut _hi: *mut gk_idx_t = _lo
            .offset(_elems as isize)
            .offset(-(1 as isize));
        let mut _stack: [C2RustUnnamed_8; 64] = [C2RustUnnamed_8 {
            _hi: 0 as *mut gk_idx_t,
            _lo: 0 as *mut gk_idx_t,
        }; 64];
        let mut _top: *mut C2RustUnnamed_8 = _stack
            .as_mut_ptr()
            .offset(1 as isize);
        while _stack.as_mut_ptr() < _top {
            let mut _left_ptr: *mut gk_idx_t = 0 as *mut gk_idx_t;
            let mut _right_ptr: *mut gk_idx_t = 0 as *mut gk_idx_t;
            let mut _mid: *mut gk_idx_t = _lo
                .offset(
                    (_hi.offset_from(_lo) as i64 >> 1) as isize,
                );
            if *_mid > *_lo {
                _hold = *_mid;
                *_mid = *_lo;
                *_lo = _hold;
            }
            if *_hi > *_mid {
                _hold = *_mid;
                *_mid = *_hi;
                *_hi = _hold;
                if *_mid > *_lo {
                    _hold = *_mid;
                    *_mid = *_lo;
                    *_lo = _hold;
                }
            }
            _left_ptr = _lo.offset(1 as isize);
            _right_ptr = _hi.offset(-(1 as isize));
            loop {
                while *_left_ptr > *_mid {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                }
                while *_mid > *_right_ptr {
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                }
                if _left_ptr < _right_ptr {
                    _hold = *_left_ptr;
                    *_left_ptr = *_right_ptr;
                    *_right_ptr = _hold;
                    if _mid == _left_ptr {
                        _mid = _right_ptr;
                    } else if _mid == _right_ptr {
                        _mid = _left_ptr;
                    }
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                } else if _left_ptr == _right_ptr {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                    break;
                }
                if !(_left_ptr <= _right_ptr) {
                    break;
                }
            }
            if _right_ptr.offset_from(_lo) as i64
                <= 4 as libc::c_int as i64
            {
                if _hi.offset_from(_left_ptr) as i64
                    <= 4 as libc::c_int as i64
                {
                    _top = _top.offset(-1);
                    _top;
                    _lo = (*_top)._lo;
                    _hi = (*_top)._hi;
                } else {
                    _lo = _left_ptr;
                }
            } else if _hi.offset_from(_left_ptr) as i64
                <= 4 as libc::c_int as i64
            {
                _hi = _right_ptr;
            } else if _right_ptr.offset_from(_lo) as i64
                > _hi.offset_from(_left_ptr) as i64
            {
                (*_top)._lo = _lo;
                (*_top)._hi = _right_ptr;
                _top = _top.offset(1);
                _top;
                _lo = _left_ptr;
            } else {
                (*_top)._lo = _left_ptr;
                (*_top)._hi = _hi;
                _top = _top.offset(1);
                _top;
                _hi = _right_ptr;
            }
        }
    }
    let _end_ptr: *mut gk_idx_t = _base
        .offset(_elems as isize)
        .offset(-(1 as isize));
    let mut _tmp_ptr: *mut gk_idx_t = _base;
    let mut _run_ptr: *mut gk_idx_t = 0 as *mut gk_idx_t;
    let mut _thresh: *mut gk_idx_t = 0 as *mut gk_idx_t;
    _thresh = _base.offset(4 as libc::c_int as isize);
    if _thresh > _end_ptr {
        _thresh = _end_ptr;
    }
    _run_ptr = _tmp_ptr.offset(1 as isize);
    while _run_ptr <= _thresh {
        if *_run_ptr > *_tmp_ptr {
            _tmp_ptr = _run_ptr;
        }
        _run_ptr = _run_ptr.offset(1);
        _run_ptr;
    }
    if _tmp_ptr != _base {
        _hold = *_tmp_ptr;
        *_tmp_ptr = *_base;
        *_base = _hold;
    }
    _run_ptr = _base.offset(1 as isize);
    loop {
        _run_ptr = _run_ptr.offset(1);
        if !(_run_ptr <= _end_ptr) {
            break;
        }
        _tmp_ptr = _run_ptr.offset(-(1 as isize));
        while *_run_ptr > *_tmp_ptr {
            _tmp_ptr = _tmp_ptr.offset(-1);
            _tmp_ptr;
        }
        _tmp_ptr = _tmp_ptr.offset(1);
        _tmp_ptr;
        if _tmp_ptr != _run_ptr {
            let mut _trav: *mut gk_idx_t = _run_ptr.offset(1 as isize);
            loop {
                _trav = _trav.offset(-1);
                if !(_trav >= _run_ptr) {
                    break;
                }
                let mut _hi_0: *mut gk_idx_t = 0 as *mut gk_idx_t;
                let mut _lo_0: *mut gk_idx_t = 0 as *mut gk_idx_t;
                _hold = *_trav;
                _lo_0 = _trav;
                _hi_0 = _lo_0;
                loop {
                    _lo_0 = _lo_0.offset(-1);
                    if !(_lo_0 >= _tmp_ptr) {
                        break;
                    }
                    *_hi_0 = *_lo_0;
                    _hi_0 = _lo_0;
                }
                *_hi_0 = _hold;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_ckvsorti(mut n: size_t, mut base: *mut gk_ckv_t) {
    let _base: *mut gk_ckv_t = base;
    let _elems: size_t = n;
    let mut _hold: gk_ckv_t = gk_ckv_t { key: 0, val: 0 };
    if _elems == 0 as libc::c_int as u64 {
        return;
    }
    if _elems > 4 as libc::c_int as u64 {
        let mut _lo: *mut gk_ckv_t = _base;
        let mut _hi: *mut gk_ckv_t = _lo
            .offset(_elems as isize)
            .offset(-(1 as isize));
        let mut _stack: [C2RustUnnamed_9; 64] = [C2RustUnnamed_9 {
            _hi: 0 as *mut gk_ckv_t,
            _lo: 0 as *mut gk_ckv_t,
        }; 64];
        let mut _top: *mut C2RustUnnamed_9 = _stack
            .as_mut_ptr()
            .offset(1 as isize);
        while _stack.as_mut_ptr() < _top {
            let mut _left_ptr: *mut gk_ckv_t = 0 as *mut gk_ckv_t;
            let mut _right_ptr: *mut gk_ckv_t = 0 as *mut gk_ckv_t;
            let mut _mid: *mut gk_ckv_t = _lo
                .offset(
                    (_hi.offset_from(_lo) as i64 >> 1) as isize,
                );
            if ((*_mid).key as libc::c_int) < (*_lo).key as libc::c_int {
                _hold = *_mid;
                *_mid = *_lo;
                *_lo = _hold;
            }
            if ((*_hi).key as libc::c_int) < (*_mid).key as libc::c_int {
                _hold = *_mid;
                *_mid = *_hi;
                *_hi = _hold;
                if ((*_mid).key as libc::c_int) < (*_lo).key as libc::c_int {
                    _hold = *_mid;
                    *_mid = *_lo;
                    *_lo = _hold;
                }
            }
            _left_ptr = _lo.offset(1 as isize);
            _right_ptr = _hi.offset(-(1 as isize));
            loop {
                while ((*_left_ptr).key as libc::c_int) < (*_mid).key as libc::c_int {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                }
                while ((*_mid).key as libc::c_int) < (*_right_ptr).key as libc::c_int {
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                }
                if _left_ptr < _right_ptr {
                    _hold = *_left_ptr;
                    *_left_ptr = *_right_ptr;
                    *_right_ptr = _hold;
                    if _mid == _left_ptr {
                        _mid = _right_ptr;
                    } else if _mid == _right_ptr {
                        _mid = _left_ptr;
                    }
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                } else if _left_ptr == _right_ptr {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                    break;
                }
                if !(_left_ptr <= _right_ptr) {
                    break;
                }
            }
            if _right_ptr.offset_from(_lo) as i64
                <= 4 as libc::c_int as i64
            {
                if _hi.offset_from(_left_ptr) as i64
                    <= 4 as libc::c_int as i64
                {
                    _top = _top.offset(-1);
                    _top;
                    _lo = (*_top)._lo;
                    _hi = (*_top)._hi;
                } else {
                    _lo = _left_ptr;
                }
            } else if _hi.offset_from(_left_ptr) as i64
                <= 4 as libc::c_int as i64
            {
                _hi = _right_ptr;
            } else if _right_ptr.offset_from(_lo) as i64
                > _hi.offset_from(_left_ptr) as i64
            {
                (*_top)._lo = _lo;
                (*_top)._hi = _right_ptr;
                _top = _top.offset(1);
                _top;
                _lo = _left_ptr;
            } else {
                (*_top)._lo = _left_ptr;
                (*_top)._hi = _hi;
                _top = _top.offset(1);
                _top;
                _hi = _right_ptr;
            }
        }
    }
    let _end_ptr: *mut gk_ckv_t = _base
        .offset(_elems as isize)
        .offset(-(1 as isize));
    let mut _tmp_ptr: *mut gk_ckv_t = _base;
    let mut _run_ptr: *mut gk_ckv_t = 0 as *mut gk_ckv_t;
    let mut _thresh: *mut gk_ckv_t = 0 as *mut gk_ckv_t;
    _thresh = _base.offset(4 as libc::c_int as isize);
    if _thresh > _end_ptr {
        _thresh = _end_ptr;
    }
    _run_ptr = _tmp_ptr.offset(1 as isize);
    while _run_ptr <= _thresh {
        if ((*_run_ptr).key as libc::c_int) < (*_tmp_ptr).key as libc::c_int {
            _tmp_ptr = _run_ptr;
        }
        _run_ptr = _run_ptr.offset(1);
        _run_ptr;
    }
    if _tmp_ptr != _base {
        _hold = *_tmp_ptr;
        *_tmp_ptr = *_base;
        *_base = _hold;
    }
    _run_ptr = _base.offset(1 as isize);
    loop {
        _run_ptr = _run_ptr.offset(1);
        if !(_run_ptr <= _end_ptr) {
            break;
        }
        _tmp_ptr = _run_ptr.offset(-(1 as isize));
        while ((*_run_ptr).key as libc::c_int) < (*_tmp_ptr).key as libc::c_int {
            _tmp_ptr = _tmp_ptr.offset(-1);
            _tmp_ptr;
        }
        _tmp_ptr = _tmp_ptr.offset(1);
        _tmp_ptr;
        if _tmp_ptr != _run_ptr {
            let mut _trav: *mut gk_ckv_t = _run_ptr.offset(1 as isize);
            loop {
                _trav = _trav.offset(-1);
                if !(_trav >= _run_ptr) {
                    break;
                }
                let mut _hi_0: *mut gk_ckv_t = 0 as *mut gk_ckv_t;
                let mut _lo_0: *mut gk_ckv_t = 0 as *mut gk_ckv_t;
                _hold = *_trav;
                _lo_0 = _trav;
                _hi_0 = _lo_0;
                loop {
                    _lo_0 = _lo_0.offset(-1);
                    if !(_lo_0 >= _tmp_ptr) {
                        break;
                    }
                    *_hi_0 = *_lo_0;
                    _hi_0 = _lo_0;
                }
                *_hi_0 = _hold;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_ckvsortd(mut n: size_t, mut base: *mut gk_ckv_t) {
    let _base: *mut gk_ckv_t = base;
    let _elems: size_t = n;
    let mut _hold: gk_ckv_t = gk_ckv_t { key: 0, val: 0 };
    if _elems == 0 as libc::c_int as u64 {
        return;
    }
    if _elems > 4 as libc::c_int as u64 {
        let mut _lo: *mut gk_ckv_t = _base;
        let mut _hi: *mut gk_ckv_t = _lo
            .offset(_elems as isize)
            .offset(-(1 as isize));
        let mut _stack: [C2RustUnnamed_10; 64] = [C2RustUnnamed_10 {
            _hi: 0 as *mut gk_ckv_t,
            _lo: 0 as *mut gk_ckv_t,
        }; 64];
        let mut _top: *mut C2RustUnnamed_10 = _stack
            .as_mut_ptr()
            .offset(1 as isize);
        while _stack.as_mut_ptr() < _top {
            let mut _left_ptr: *mut gk_ckv_t = 0 as *mut gk_ckv_t;
            let mut _right_ptr: *mut gk_ckv_t = 0 as *mut gk_ckv_t;
            let mut _mid: *mut gk_ckv_t = _lo
                .offset(
                    (_hi.offset_from(_lo) as i64 >> 1) as isize,
                );
            if (*_mid).key as libc::c_int > (*_lo).key as libc::c_int {
                _hold = *_mid;
                *_mid = *_lo;
                *_lo = _hold;
            }
            if (*_hi).key as libc::c_int > (*_mid).key as libc::c_int {
                _hold = *_mid;
                *_mid = *_hi;
                *_hi = _hold;
                if (*_mid).key as libc::c_int > (*_lo).key as libc::c_int {
                    _hold = *_mid;
                    *_mid = *_lo;
                    *_lo = _hold;
                }
            }
            _left_ptr = _lo.offset(1 as isize);
            _right_ptr = _hi.offset(-(1 as isize));
            loop {
                while (*_left_ptr).key as libc::c_int > (*_mid).key as libc::c_int {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                }
                while (*_mid).key as libc::c_int > (*_right_ptr).key as libc::c_int {
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                }
                if _left_ptr < _right_ptr {
                    _hold = *_left_ptr;
                    *_left_ptr = *_right_ptr;
                    *_right_ptr = _hold;
                    if _mid == _left_ptr {
                        _mid = _right_ptr;
                    } else if _mid == _right_ptr {
                        _mid = _left_ptr;
                    }
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                } else if _left_ptr == _right_ptr {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                    break;
                }
                if !(_left_ptr <= _right_ptr) {
                    break;
                }
            }
            if _right_ptr.offset_from(_lo) as i64
                <= 4 as libc::c_int as i64
            {
                if _hi.offset_from(_left_ptr) as i64
                    <= 4 as libc::c_int as i64
                {
                    _top = _top.offset(-1);
                    _top;
                    _lo = (*_top)._lo;
                    _hi = (*_top)._hi;
                } else {
                    _lo = _left_ptr;
                }
            } else if _hi.offset_from(_left_ptr) as i64
                <= 4 as libc::c_int as i64
            {
                _hi = _right_ptr;
            } else if _right_ptr.offset_from(_lo) as i64
                > _hi.offset_from(_left_ptr) as i64
            {
                (*_top)._lo = _lo;
                (*_top)._hi = _right_ptr;
                _top = _top.offset(1);
                _top;
                _lo = _left_ptr;
            } else {
                (*_top)._lo = _left_ptr;
                (*_top)._hi = _hi;
                _top = _top.offset(1);
                _top;
                _hi = _right_ptr;
            }
        }
    }
    let _end_ptr: *mut gk_ckv_t = _base
        .offset(_elems as isize)
        .offset(-(1 as isize));
    let mut _tmp_ptr: *mut gk_ckv_t = _base;
    let mut _run_ptr: *mut gk_ckv_t = 0 as *mut gk_ckv_t;
    let mut _thresh: *mut gk_ckv_t = 0 as *mut gk_ckv_t;
    _thresh = _base.offset(4 as libc::c_int as isize);
    if _thresh > _end_ptr {
        _thresh = _end_ptr;
    }
    _run_ptr = _tmp_ptr.offset(1 as isize);
    while _run_ptr <= _thresh {
        if (*_run_ptr).key as libc::c_int > (*_tmp_ptr).key as libc::c_int {
            _tmp_ptr = _run_ptr;
        }
        _run_ptr = _run_ptr.offset(1);
        _run_ptr;
    }
    if _tmp_ptr != _base {
        _hold = *_tmp_ptr;
        *_tmp_ptr = *_base;
        *_base = _hold;
    }
    _run_ptr = _base.offset(1 as isize);
    loop {
        _run_ptr = _run_ptr.offset(1);
        if !(_run_ptr <= _end_ptr) {
            break;
        }
        _tmp_ptr = _run_ptr.offset(-(1 as isize));
        while (*_run_ptr).key as libc::c_int > (*_tmp_ptr).key as libc::c_int {
            _tmp_ptr = _tmp_ptr.offset(-1);
            _tmp_ptr;
        }
        _tmp_ptr = _tmp_ptr.offset(1);
        _tmp_ptr;
        if _tmp_ptr != _run_ptr {
            let mut _trav: *mut gk_ckv_t = _run_ptr.offset(1 as isize);
            loop {
                _trav = _trav.offset(-1);
                if !(_trav >= _run_ptr) {
                    break;
                }
                let mut _hi_0: *mut gk_ckv_t = 0 as *mut gk_ckv_t;
                let mut _lo_0: *mut gk_ckv_t = 0 as *mut gk_ckv_t;
                _hold = *_trav;
                _lo_0 = _trav;
                _hi_0 = _lo_0;
                loop {
                    _lo_0 = _lo_0.offset(-1);
                    if !(_lo_0 >= _tmp_ptr) {
                        break;
                    }
                    *_hi_0 = *_lo_0;
                    _hi_0 = _lo_0;
                }
                *_hi_0 = _hold;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_ikvsorti(mut n: size_t, mut base: *mut gk_ikv_t) {
    let _base: *mut gk_ikv_t = base;
    let _elems: size_t = n;
    let mut _hold: gk_ikv_t = gk_ikv_t { key: 0, val: 0 };
    if _elems == 0 as libc::c_int as u64 {
        return;
    }
    if _elems > 4 as libc::c_int as u64 {
        let mut _lo: *mut gk_ikv_t = _base;
        let mut _hi: *mut gk_ikv_t = _lo
            .offset(_elems as isize)
            .offset(-(1 as isize));
        let mut _stack: [C2RustUnnamed_11; 64] = [C2RustUnnamed_11 {
            _hi: 0 as *mut gk_ikv_t,
            _lo: 0 as *mut gk_ikv_t,
        }; 64];
        let mut _top: *mut C2RustUnnamed_11 = _stack
            .as_mut_ptr()
            .offset(1 as isize);
        while _stack.as_mut_ptr() < _top {
            let mut _left_ptr: *mut gk_ikv_t = 0 as *mut gk_ikv_t;
            let mut _right_ptr: *mut gk_ikv_t = 0 as *mut gk_ikv_t;
            let mut _mid: *mut gk_ikv_t = _lo
                .offset(
                    (_hi.offset_from(_lo) as i64 >> 1) as isize,
                );
            if (*_mid).key < (*_lo).key {
                _hold = *_mid;
                *_mid = *_lo;
                *_lo = _hold;
            }
            if (*_hi).key < (*_mid).key {
                _hold = *_mid;
                *_mid = *_hi;
                *_hi = _hold;
                if (*_mid).key < (*_lo).key {
                    _hold = *_mid;
                    *_mid = *_lo;
                    *_lo = _hold;
                }
            }
            _left_ptr = _lo.offset(1 as isize);
            _right_ptr = _hi.offset(-(1 as isize));
            loop {
                while (*_left_ptr).key < (*_mid).key {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                }
                while (*_mid).key < (*_right_ptr).key {
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                }
                if _left_ptr < _right_ptr {
                    _hold = *_left_ptr;
                    *_left_ptr = *_right_ptr;
                    *_right_ptr = _hold;
                    if _mid == _left_ptr {
                        _mid = _right_ptr;
                    } else if _mid == _right_ptr {
                        _mid = _left_ptr;
                    }
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                } else if _left_ptr == _right_ptr {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                    break;
                }
                if !(_left_ptr <= _right_ptr) {
                    break;
                }
            }
            if _right_ptr.offset_from(_lo) as i64
                <= 4 as libc::c_int as i64
            {
                if _hi.offset_from(_left_ptr) as i64
                    <= 4 as libc::c_int as i64
                {
                    _top = _top.offset(-1);
                    _top;
                    _lo = (*_top)._lo;
                    _hi = (*_top)._hi;
                } else {
                    _lo = _left_ptr;
                }
            } else if _hi.offset_from(_left_ptr) as i64
                <= 4 as libc::c_int as i64
            {
                _hi = _right_ptr;
            } else if _right_ptr.offset_from(_lo) as i64
                > _hi.offset_from(_left_ptr) as i64
            {
                (*_top)._lo = _lo;
                (*_top)._hi = _right_ptr;
                _top = _top.offset(1);
                _top;
                _lo = _left_ptr;
            } else {
                (*_top)._lo = _left_ptr;
                (*_top)._hi = _hi;
                _top = _top.offset(1);
                _top;
                _hi = _right_ptr;
            }
        }
    }
    let _end_ptr: *mut gk_ikv_t = _base
        .offset(_elems as isize)
        .offset(-(1 as isize));
    let mut _tmp_ptr: *mut gk_ikv_t = _base;
    let mut _run_ptr: *mut gk_ikv_t = 0 as *mut gk_ikv_t;
    let mut _thresh: *mut gk_ikv_t = 0 as *mut gk_ikv_t;
    _thresh = _base.offset(4 as libc::c_int as isize);
    if _thresh > _end_ptr {
        _thresh = _end_ptr;
    }
    _run_ptr = _tmp_ptr.offset(1 as isize);
    while _run_ptr <= _thresh {
        if (*_run_ptr).key < (*_tmp_ptr).key {
            _tmp_ptr = _run_ptr;
        }
        _run_ptr = _run_ptr.offset(1);
        _run_ptr;
    }
    if _tmp_ptr != _base {
        _hold = *_tmp_ptr;
        *_tmp_ptr = *_base;
        *_base = _hold;
    }
    _run_ptr = _base.offset(1 as isize);
    loop {
        _run_ptr = _run_ptr.offset(1);
        if !(_run_ptr <= _end_ptr) {
            break;
        }
        _tmp_ptr = _run_ptr.offset(-(1 as isize));
        while (*_run_ptr).key < (*_tmp_ptr).key {
            _tmp_ptr = _tmp_ptr.offset(-1);
            _tmp_ptr;
        }
        _tmp_ptr = _tmp_ptr.offset(1);
        _tmp_ptr;
        if _tmp_ptr != _run_ptr {
            let mut _trav: *mut gk_ikv_t = _run_ptr.offset(1 as isize);
            loop {
                _trav = _trav.offset(-1);
                if !(_trav >= _run_ptr) {
                    break;
                }
                let mut _hi_0: *mut gk_ikv_t = 0 as *mut gk_ikv_t;
                let mut _lo_0: *mut gk_ikv_t = 0 as *mut gk_ikv_t;
                _hold = *_trav;
                _lo_0 = _trav;
                _hi_0 = _lo_0;
                loop {
                    _lo_0 = _lo_0.offset(-1);
                    if !(_lo_0 >= _tmp_ptr) {
                        break;
                    }
                    *_hi_0 = *_lo_0;
                    _hi_0 = _lo_0;
                }
                *_hi_0 = _hold;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_ikvsortd(mut n: size_t, mut base: *mut gk_ikv_t) {
    let _base: *mut gk_ikv_t = base;
    let _elems: size_t = n;
    let mut _hold: gk_ikv_t = gk_ikv_t { key: 0, val: 0 };
    if _elems == 0 as libc::c_int as u64 {
        return;
    }
    if _elems > 4 as libc::c_int as u64 {
        let mut _lo: *mut gk_ikv_t = _base;
        let mut _hi: *mut gk_ikv_t = _lo
            .offset(_elems as isize)
            .offset(-(1 as isize));
        let mut _stack: [C2RustUnnamed_12; 64] = [C2RustUnnamed_12 {
            _hi: 0 as *mut gk_ikv_t,
            _lo: 0 as *mut gk_ikv_t,
        }; 64];
        let mut _top: *mut C2RustUnnamed_12 = _stack
            .as_mut_ptr()
            .offset(1 as isize);
        while _stack.as_mut_ptr() < _top {
            let mut _left_ptr: *mut gk_ikv_t = 0 as *mut gk_ikv_t;
            let mut _right_ptr: *mut gk_ikv_t = 0 as *mut gk_ikv_t;
            let mut _mid: *mut gk_ikv_t = _lo
                .offset(
                    (_hi.offset_from(_lo) as i64 >> 1) as isize,
                );
            if (*_mid).key > (*_lo).key {
                _hold = *_mid;
                *_mid = *_lo;
                *_lo = _hold;
            }
            if (*_hi).key > (*_mid).key {
                _hold = *_mid;
                *_mid = *_hi;
                *_hi = _hold;
                if (*_mid).key > (*_lo).key {
                    _hold = *_mid;
                    *_mid = *_lo;
                    *_lo = _hold;
                }
            }
            _left_ptr = _lo.offset(1 as isize);
            _right_ptr = _hi.offset(-(1 as isize));
            loop {
                while (*_left_ptr).key > (*_mid).key {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                }
                while (*_mid).key > (*_right_ptr).key {
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                }
                if _left_ptr < _right_ptr {
                    _hold = *_left_ptr;
                    *_left_ptr = *_right_ptr;
                    *_right_ptr = _hold;
                    if _mid == _left_ptr {
                        _mid = _right_ptr;
                    } else if _mid == _right_ptr {
                        _mid = _left_ptr;
                    }
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                } else if _left_ptr == _right_ptr {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                    break;
                }
                if !(_left_ptr <= _right_ptr) {
                    break;
                }
            }
            if _right_ptr.offset_from(_lo) as i64
                <= 4 as libc::c_int as i64
            {
                if _hi.offset_from(_left_ptr) as i64
                    <= 4 as libc::c_int as i64
                {
                    _top = _top.offset(-1);
                    _top;
                    _lo = (*_top)._lo;
                    _hi = (*_top)._hi;
                } else {
                    _lo = _left_ptr;
                }
            } else if _hi.offset_from(_left_ptr) as i64
                <= 4 as libc::c_int as i64
            {
                _hi = _right_ptr;
            } else if _right_ptr.offset_from(_lo) as i64
                > _hi.offset_from(_left_ptr) as i64
            {
                (*_top)._lo = _lo;
                (*_top)._hi = _right_ptr;
                _top = _top.offset(1);
                _top;
                _lo = _left_ptr;
            } else {
                (*_top)._lo = _left_ptr;
                (*_top)._hi = _hi;
                _top = _top.offset(1);
                _top;
                _hi = _right_ptr;
            }
        }
    }
    let _end_ptr: *mut gk_ikv_t = _base
        .offset(_elems as isize)
        .offset(-(1 as isize));
    let mut _tmp_ptr: *mut gk_ikv_t = _base;
    let mut _run_ptr: *mut gk_ikv_t = 0 as *mut gk_ikv_t;
    let mut _thresh: *mut gk_ikv_t = 0 as *mut gk_ikv_t;
    _thresh = _base.offset(4 as libc::c_int as isize);
    if _thresh > _end_ptr {
        _thresh = _end_ptr;
    }
    _run_ptr = _tmp_ptr.offset(1 as isize);
    while _run_ptr <= _thresh {
        if (*_run_ptr).key > (*_tmp_ptr).key {
            _tmp_ptr = _run_ptr;
        }
        _run_ptr = _run_ptr.offset(1);
        _run_ptr;
    }
    if _tmp_ptr != _base {
        _hold = *_tmp_ptr;
        *_tmp_ptr = *_base;
        *_base = _hold;
    }
    _run_ptr = _base.offset(1 as isize);
    loop {
        _run_ptr = _run_ptr.offset(1);
        if !(_run_ptr <= _end_ptr) {
            break;
        }
        _tmp_ptr = _run_ptr.offset(-(1 as isize));
        while (*_run_ptr).key > (*_tmp_ptr).key {
            _tmp_ptr = _tmp_ptr.offset(-1);
            _tmp_ptr;
        }
        _tmp_ptr = _tmp_ptr.offset(1);
        _tmp_ptr;
        if _tmp_ptr != _run_ptr {
            let mut _trav: *mut gk_ikv_t = _run_ptr.offset(1 as isize);
            loop {
                _trav = _trav.offset(-1);
                if !(_trav >= _run_ptr) {
                    break;
                }
                let mut _hi_0: *mut gk_ikv_t = 0 as *mut gk_ikv_t;
                let mut _lo_0: *mut gk_ikv_t = 0 as *mut gk_ikv_t;
                _hold = *_trav;
                _lo_0 = _trav;
                _hi_0 = _lo_0;
                loop {
                    _lo_0 = _lo_0.offset(-1);
                    if !(_lo_0 >= _tmp_ptr) {
                        break;
                    }
                    *_hi_0 = *_lo_0;
                    _hi_0 = _lo_0;
                }
                *_hi_0 = _hold;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_i32kvsorti(mut n: size_t, mut base: *mut gk_i32kv_t) {
    let _base: *mut gk_i32kv_t = base;
    let _elems: size_t = n;
    let mut _hold: gk_i32kv_t = gk_i32kv_t { key: 0, val: 0 };
    if _elems == 0 as libc::c_int as u64 {
        return;
    }
    if _elems > 4 as libc::c_int as u64 {
        let mut _lo: *mut gk_i32kv_t = _base;
        let mut _hi: *mut gk_i32kv_t = _lo
            .offset(_elems as isize)
            .offset(-(1 as isize));
        let mut _stack: [C2RustUnnamed_13; 64] = [C2RustUnnamed_13 {
            _hi: 0 as *mut gk_i32kv_t,
            _lo: 0 as *mut gk_i32kv_t,
        }; 64];
        let mut _top: *mut C2RustUnnamed_13 = _stack
            .as_mut_ptr()
            .offset(1 as isize);
        while _stack.as_mut_ptr() < _top {
            let mut _left_ptr: *mut gk_i32kv_t = 0 as *mut gk_i32kv_t;
            let mut _right_ptr: *mut gk_i32kv_t = 0 as *mut gk_i32kv_t;
            let mut _mid: *mut gk_i32kv_t = _lo
                .offset(
                    (_hi.offset_from(_lo) as i64 >> 1) as isize,
                );
            if (*_mid).key < (*_lo).key {
                _hold = *_mid;
                *_mid = *_lo;
                *_lo = _hold;
            }
            if (*_hi).key < (*_mid).key {
                _hold = *_mid;
                *_mid = *_hi;
                *_hi = _hold;
                if (*_mid).key < (*_lo).key {
                    _hold = *_mid;
                    *_mid = *_lo;
                    *_lo = _hold;
                }
            }
            _left_ptr = _lo.offset(1 as isize);
            _right_ptr = _hi.offset(-(1 as isize));
            loop {
                while (*_left_ptr).key < (*_mid).key {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                }
                while (*_mid).key < (*_right_ptr).key {
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                }
                if _left_ptr < _right_ptr {
                    _hold = *_left_ptr;
                    *_left_ptr = *_right_ptr;
                    *_right_ptr = _hold;
                    if _mid == _left_ptr {
                        _mid = _right_ptr;
                    } else if _mid == _right_ptr {
                        _mid = _left_ptr;
                    }
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                } else if _left_ptr == _right_ptr {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                    break;
                }
                if !(_left_ptr <= _right_ptr) {
                    break;
                }
            }
            if _right_ptr.offset_from(_lo) as i64
                <= 4 as libc::c_int as i64
            {
                if _hi.offset_from(_left_ptr) as i64
                    <= 4 as libc::c_int as i64
                {
                    _top = _top.offset(-1);
                    _top;
                    _lo = (*_top)._lo;
                    _hi = (*_top)._hi;
                } else {
                    _lo = _left_ptr;
                }
            } else if _hi.offset_from(_left_ptr) as i64
                <= 4 as libc::c_int as i64
            {
                _hi = _right_ptr;
            } else if _right_ptr.offset_from(_lo) as i64
                > _hi.offset_from(_left_ptr) as i64
            {
                (*_top)._lo = _lo;
                (*_top)._hi = _right_ptr;
                _top = _top.offset(1);
                _top;
                _lo = _left_ptr;
            } else {
                (*_top)._lo = _left_ptr;
                (*_top)._hi = _hi;
                _top = _top.offset(1);
                _top;
                _hi = _right_ptr;
            }
        }
    }
    let _end_ptr: *mut gk_i32kv_t = _base
        .offset(_elems as isize)
        .offset(-(1 as isize));
    let mut _tmp_ptr: *mut gk_i32kv_t = _base;
    let mut _run_ptr: *mut gk_i32kv_t = 0 as *mut gk_i32kv_t;
    let mut _thresh: *mut gk_i32kv_t = 0 as *mut gk_i32kv_t;
    _thresh = _base.offset(4 as libc::c_int as isize);
    if _thresh > _end_ptr {
        _thresh = _end_ptr;
    }
    _run_ptr = _tmp_ptr.offset(1 as isize);
    while _run_ptr <= _thresh {
        if (*_run_ptr).key < (*_tmp_ptr).key {
            _tmp_ptr = _run_ptr;
        }
        _run_ptr = _run_ptr.offset(1);
        _run_ptr;
    }
    if _tmp_ptr != _base {
        _hold = *_tmp_ptr;
        *_tmp_ptr = *_base;
        *_base = _hold;
    }
    _run_ptr = _base.offset(1 as isize);
    loop {
        _run_ptr = _run_ptr.offset(1);
        if !(_run_ptr <= _end_ptr) {
            break;
        }
        _tmp_ptr = _run_ptr.offset(-(1 as isize));
        while (*_run_ptr).key < (*_tmp_ptr).key {
            _tmp_ptr = _tmp_ptr.offset(-1);
            _tmp_ptr;
        }
        _tmp_ptr = _tmp_ptr.offset(1);
        _tmp_ptr;
        if _tmp_ptr != _run_ptr {
            let mut _trav: *mut gk_i32kv_t = _run_ptr.offset(1 as isize);
            loop {
                _trav = _trav.offset(-1);
                if !(_trav >= _run_ptr) {
                    break;
                }
                let mut _hi_0: *mut gk_i32kv_t = 0 as *mut gk_i32kv_t;
                let mut _lo_0: *mut gk_i32kv_t = 0 as *mut gk_i32kv_t;
                _hold = *_trav;
                _lo_0 = _trav;
                _hi_0 = _lo_0;
                loop {
                    _lo_0 = _lo_0.offset(-1);
                    if !(_lo_0 >= _tmp_ptr) {
                        break;
                    }
                    *_hi_0 = *_lo_0;
                    _hi_0 = _lo_0;
                }
                *_hi_0 = _hold;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_i32kvsortd(mut n: size_t, mut base: *mut gk_i32kv_t) {
    let _base: *mut gk_i32kv_t = base;
    let _elems: size_t = n;
    let mut _hold: gk_i32kv_t = gk_i32kv_t { key: 0, val: 0 };
    if _elems == 0 as libc::c_int as u64 {
        return;
    }
    if _elems > 4 as libc::c_int as u64 {
        let mut _lo: *mut gk_i32kv_t = _base;
        let mut _hi: *mut gk_i32kv_t = _lo
            .offset(_elems as isize)
            .offset(-(1 as isize));
        let mut _stack: [C2RustUnnamed_14; 64] = [C2RustUnnamed_14 {
            _hi: 0 as *mut gk_i32kv_t,
            _lo: 0 as *mut gk_i32kv_t,
        }; 64];
        let mut _top: *mut C2RustUnnamed_14 = _stack
            .as_mut_ptr()
            .offset(1 as isize);
        while _stack.as_mut_ptr() < _top {
            let mut _left_ptr: *mut gk_i32kv_t = 0 as *mut gk_i32kv_t;
            let mut _right_ptr: *mut gk_i32kv_t = 0 as *mut gk_i32kv_t;
            let mut _mid: *mut gk_i32kv_t = _lo
                .offset(
                    (_hi.offset_from(_lo) as i64 >> 1) as isize,
                );
            if (*_mid).key > (*_lo).key {
                _hold = *_mid;
                *_mid = *_lo;
                *_lo = _hold;
            }
            if (*_hi).key > (*_mid).key {
                _hold = *_mid;
                *_mid = *_hi;
                *_hi = _hold;
                if (*_mid).key > (*_lo).key {
                    _hold = *_mid;
                    *_mid = *_lo;
                    *_lo = _hold;
                }
            }
            _left_ptr = _lo.offset(1 as isize);
            _right_ptr = _hi.offset(-(1 as isize));
            loop {
                while (*_left_ptr).key > (*_mid).key {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                }
                while (*_mid).key > (*_right_ptr).key {
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                }
                if _left_ptr < _right_ptr {
                    _hold = *_left_ptr;
                    *_left_ptr = *_right_ptr;
                    *_right_ptr = _hold;
                    if _mid == _left_ptr {
                        _mid = _right_ptr;
                    } else if _mid == _right_ptr {
                        _mid = _left_ptr;
                    }
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                } else if _left_ptr == _right_ptr {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                    break;
                }
                if !(_left_ptr <= _right_ptr) {
                    break;
                }
            }
            if _right_ptr.offset_from(_lo) as i64
                <= 4 as libc::c_int as i64
            {
                if _hi.offset_from(_left_ptr) as i64
                    <= 4 as libc::c_int as i64
                {
                    _top = _top.offset(-1);
                    _top;
                    _lo = (*_top)._lo;
                    _hi = (*_top)._hi;
                } else {
                    _lo = _left_ptr;
                }
            } else if _hi.offset_from(_left_ptr) as i64
                <= 4 as libc::c_int as i64
            {
                _hi = _right_ptr;
            } else if _right_ptr.offset_from(_lo) as i64
                > _hi.offset_from(_left_ptr) as i64
            {
                (*_top)._lo = _lo;
                (*_top)._hi = _right_ptr;
                _top = _top.offset(1);
                _top;
                _lo = _left_ptr;
            } else {
                (*_top)._lo = _left_ptr;
                (*_top)._hi = _hi;
                _top = _top.offset(1);
                _top;
                _hi = _right_ptr;
            }
        }
    }
    let _end_ptr: *mut gk_i32kv_t = _base
        .offset(_elems as isize)
        .offset(-(1 as isize));
    let mut _tmp_ptr: *mut gk_i32kv_t = _base;
    let mut _run_ptr: *mut gk_i32kv_t = 0 as *mut gk_i32kv_t;
    let mut _thresh: *mut gk_i32kv_t = 0 as *mut gk_i32kv_t;
    _thresh = _base.offset(4 as libc::c_int as isize);
    if _thresh > _end_ptr {
        _thresh = _end_ptr;
    }
    _run_ptr = _tmp_ptr.offset(1 as isize);
    while _run_ptr <= _thresh {
        if (*_run_ptr).key > (*_tmp_ptr).key {
            _tmp_ptr = _run_ptr;
        }
        _run_ptr = _run_ptr.offset(1);
        _run_ptr;
    }
    if _tmp_ptr != _base {
        _hold = *_tmp_ptr;
        *_tmp_ptr = *_base;
        *_base = _hold;
    }
    _run_ptr = _base.offset(1 as isize);
    loop {
        _run_ptr = _run_ptr.offset(1);
        if !(_run_ptr <= _end_ptr) {
            break;
        }
        _tmp_ptr = _run_ptr.offset(-(1 as isize));
        while (*_run_ptr).key > (*_tmp_ptr).key {
            _tmp_ptr = _tmp_ptr.offset(-1);
            _tmp_ptr;
        }
        _tmp_ptr = _tmp_ptr.offset(1);
        _tmp_ptr;
        if _tmp_ptr != _run_ptr {
            let mut _trav: *mut gk_i32kv_t = _run_ptr.offset(1 as isize);
            loop {
                _trav = _trav.offset(-1);
                if !(_trav >= _run_ptr) {
                    break;
                }
                let mut _hi_0: *mut gk_i32kv_t = 0 as *mut gk_i32kv_t;
                let mut _lo_0: *mut gk_i32kv_t = 0 as *mut gk_i32kv_t;
                _hold = *_trav;
                _lo_0 = _trav;
                _hi_0 = _lo_0;
                loop {
                    _lo_0 = _lo_0.offset(-1);
                    if !(_lo_0 >= _tmp_ptr) {
                        break;
                    }
                    *_hi_0 = *_lo_0;
                    _hi_0 = _lo_0;
                }
                *_hi_0 = _hold;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_i64kvsorti(mut n: size_t, mut base: *mut gk_i64kv_t) {
    let _base: *mut gk_i64kv_t = base;
    let _elems: size_t = n;
    let mut _hold: gk_i64kv_t = gk_i64kv_t { key: 0, val: 0 };
    if _elems == 0 as libc::c_int as u64 {
        return;
    }
    if _elems > 4 as libc::c_int as u64 {
        let mut _lo: *mut gk_i64kv_t = _base;
        let mut _hi: *mut gk_i64kv_t = _lo
            .offset(_elems as isize)
            .offset(-(1 as isize));
        let mut _stack: [C2RustUnnamed_15; 64] = [C2RustUnnamed_15 {
            _hi: 0 as *mut gk_i64kv_t,
            _lo: 0 as *mut gk_i64kv_t,
        }; 64];
        let mut _top: *mut C2RustUnnamed_15 = _stack
            .as_mut_ptr()
            .offset(1 as isize);
        while _stack.as_mut_ptr() < _top {
            let mut _left_ptr: *mut gk_i64kv_t = 0 as *mut gk_i64kv_t;
            let mut _right_ptr: *mut gk_i64kv_t = 0 as *mut gk_i64kv_t;
            let mut _mid: *mut gk_i64kv_t = _lo
                .offset(
                    (_hi.offset_from(_lo) as i64 >> 1) as isize,
                );
            if (*_mid).key < (*_lo).key {
                _hold = *_mid;
                *_mid = *_lo;
                *_lo = _hold;
            }
            if (*_hi).key < (*_mid).key {
                _hold = *_mid;
                *_mid = *_hi;
                *_hi = _hold;
                if (*_mid).key < (*_lo).key {
                    _hold = *_mid;
                    *_mid = *_lo;
                    *_lo = _hold;
                }
            }
            _left_ptr = _lo.offset(1 as isize);
            _right_ptr = _hi.offset(-(1 as isize));
            loop {
                while (*_left_ptr).key < (*_mid).key {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                }
                while (*_mid).key < (*_right_ptr).key {
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                }
                if _left_ptr < _right_ptr {
                    _hold = *_left_ptr;
                    *_left_ptr = *_right_ptr;
                    *_right_ptr = _hold;
                    if _mid == _left_ptr {
                        _mid = _right_ptr;
                    } else if _mid == _right_ptr {
                        _mid = _left_ptr;
                    }
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                } else if _left_ptr == _right_ptr {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                    break;
                }
                if !(_left_ptr <= _right_ptr) {
                    break;
                }
            }
            if _right_ptr.offset_from(_lo) as i64
                <= 4 as libc::c_int as i64
            {
                if _hi.offset_from(_left_ptr) as i64
                    <= 4 as libc::c_int as i64
                {
                    _top = _top.offset(-1);
                    _top;
                    _lo = (*_top)._lo;
                    _hi = (*_top)._hi;
                } else {
                    _lo = _left_ptr;
                }
            } else if _hi.offset_from(_left_ptr) as i64
                <= 4 as libc::c_int as i64
            {
                _hi = _right_ptr;
            } else if _right_ptr.offset_from(_lo) as i64
                > _hi.offset_from(_left_ptr) as i64
            {
                (*_top)._lo = _lo;
                (*_top)._hi = _right_ptr;
                _top = _top.offset(1);
                _top;
                _lo = _left_ptr;
            } else {
                (*_top)._lo = _left_ptr;
                (*_top)._hi = _hi;
                _top = _top.offset(1);
                _top;
                _hi = _right_ptr;
            }
        }
    }
    let _end_ptr: *mut gk_i64kv_t = _base
        .offset(_elems as isize)
        .offset(-(1 as isize));
    let mut _tmp_ptr: *mut gk_i64kv_t = _base;
    let mut _run_ptr: *mut gk_i64kv_t = 0 as *mut gk_i64kv_t;
    let mut _thresh: *mut gk_i64kv_t = 0 as *mut gk_i64kv_t;
    _thresh = _base.offset(4 as libc::c_int as isize);
    if _thresh > _end_ptr {
        _thresh = _end_ptr;
    }
    _run_ptr = _tmp_ptr.offset(1 as isize);
    while _run_ptr <= _thresh {
        if (*_run_ptr).key < (*_tmp_ptr).key {
            _tmp_ptr = _run_ptr;
        }
        _run_ptr = _run_ptr.offset(1);
        _run_ptr;
    }
    if _tmp_ptr != _base {
        _hold = *_tmp_ptr;
        *_tmp_ptr = *_base;
        *_base = _hold;
    }
    _run_ptr = _base.offset(1 as isize);
    loop {
        _run_ptr = _run_ptr.offset(1);
        if !(_run_ptr <= _end_ptr) {
            break;
        }
        _tmp_ptr = _run_ptr.offset(-(1 as isize));
        while (*_run_ptr).key < (*_tmp_ptr).key {
            _tmp_ptr = _tmp_ptr.offset(-1);
            _tmp_ptr;
        }
        _tmp_ptr = _tmp_ptr.offset(1);
        _tmp_ptr;
        if _tmp_ptr != _run_ptr {
            let mut _trav: *mut gk_i64kv_t = _run_ptr.offset(1 as isize);
            loop {
                _trav = _trav.offset(-1);
                if !(_trav >= _run_ptr) {
                    break;
                }
                let mut _hi_0: *mut gk_i64kv_t = 0 as *mut gk_i64kv_t;
                let mut _lo_0: *mut gk_i64kv_t = 0 as *mut gk_i64kv_t;
                _hold = *_trav;
                _lo_0 = _trav;
                _hi_0 = _lo_0;
                loop {
                    _lo_0 = _lo_0.offset(-1);
                    if !(_lo_0 >= _tmp_ptr) {
                        break;
                    }
                    *_hi_0 = *_lo_0;
                    _hi_0 = _lo_0;
                }
                *_hi_0 = _hold;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_i64kvsortd(mut n: size_t, mut base: *mut gk_i64kv_t) {
    let _base: *mut gk_i64kv_t = base;
    let _elems: size_t = n;
    let mut _hold: gk_i64kv_t = gk_i64kv_t { key: 0, val: 0 };
    if _elems == 0 as libc::c_int as u64 {
        return;
    }
    if _elems > 4 as libc::c_int as u64 {
        let mut _lo: *mut gk_i64kv_t = _base;
        let mut _hi: *mut gk_i64kv_t = _lo
            .offset(_elems as isize)
            .offset(-(1 as isize));
        let mut _stack: [C2RustUnnamed_16; 64] = [C2RustUnnamed_16 {
            _hi: 0 as *mut gk_i64kv_t,
            _lo: 0 as *mut gk_i64kv_t,
        }; 64];
        let mut _top: *mut C2RustUnnamed_16 = _stack
            .as_mut_ptr()
            .offset(1 as isize);
        while _stack.as_mut_ptr() < _top {
            let mut _left_ptr: *mut gk_i64kv_t = 0 as *mut gk_i64kv_t;
            let mut _right_ptr: *mut gk_i64kv_t = 0 as *mut gk_i64kv_t;
            let mut _mid: *mut gk_i64kv_t = _lo
                .offset(
                    (_hi.offset_from(_lo) as i64 >> 1) as isize,
                );
            if (*_mid).key > (*_lo).key {
                _hold = *_mid;
                *_mid = *_lo;
                *_lo = _hold;
            }
            if (*_hi).key > (*_mid).key {
                _hold = *_mid;
                *_mid = *_hi;
                *_hi = _hold;
                if (*_mid).key > (*_lo).key {
                    _hold = *_mid;
                    *_mid = *_lo;
                    *_lo = _hold;
                }
            }
            _left_ptr = _lo.offset(1 as isize);
            _right_ptr = _hi.offset(-(1 as isize));
            loop {
                while (*_left_ptr).key > (*_mid).key {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                }
                while (*_mid).key > (*_right_ptr).key {
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                }
                if _left_ptr < _right_ptr {
                    _hold = *_left_ptr;
                    *_left_ptr = *_right_ptr;
                    *_right_ptr = _hold;
                    if _mid == _left_ptr {
                        _mid = _right_ptr;
                    } else if _mid == _right_ptr {
                        _mid = _left_ptr;
                    }
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                } else if _left_ptr == _right_ptr {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                    break;
                }
                if !(_left_ptr <= _right_ptr) {
                    break;
                }
            }
            if _right_ptr.offset_from(_lo) as i64
                <= 4 as libc::c_int as i64
            {
                if _hi.offset_from(_left_ptr) as i64
                    <= 4 as libc::c_int as i64
                {
                    _top = _top.offset(-1);
                    _top;
                    _lo = (*_top)._lo;
                    _hi = (*_top)._hi;
                } else {
                    _lo = _left_ptr;
                }
            } else if _hi.offset_from(_left_ptr) as i64
                <= 4 as libc::c_int as i64
            {
                _hi = _right_ptr;
            } else if _right_ptr.offset_from(_lo) as i64
                > _hi.offset_from(_left_ptr) as i64
            {
                (*_top)._lo = _lo;
                (*_top)._hi = _right_ptr;
                _top = _top.offset(1);
                _top;
                _lo = _left_ptr;
            } else {
                (*_top)._lo = _left_ptr;
                (*_top)._hi = _hi;
                _top = _top.offset(1);
                _top;
                _hi = _right_ptr;
            }
        }
    }
    let _end_ptr: *mut gk_i64kv_t = _base
        .offset(_elems as isize)
        .offset(-(1 as isize));
    let mut _tmp_ptr: *mut gk_i64kv_t = _base;
    let mut _run_ptr: *mut gk_i64kv_t = 0 as *mut gk_i64kv_t;
    let mut _thresh: *mut gk_i64kv_t = 0 as *mut gk_i64kv_t;
    _thresh = _base.offset(4 as libc::c_int as isize);
    if _thresh > _end_ptr {
        _thresh = _end_ptr;
    }
    _run_ptr = _tmp_ptr.offset(1 as isize);
    while _run_ptr <= _thresh {
        if (*_run_ptr).key > (*_tmp_ptr).key {
            _tmp_ptr = _run_ptr;
        }
        _run_ptr = _run_ptr.offset(1);
        _run_ptr;
    }
    if _tmp_ptr != _base {
        _hold = *_tmp_ptr;
        *_tmp_ptr = *_base;
        *_base = _hold;
    }
    _run_ptr = _base.offset(1 as isize);
    loop {
        _run_ptr = _run_ptr.offset(1);
        if !(_run_ptr <= _end_ptr) {
            break;
        }
        _tmp_ptr = _run_ptr.offset(-(1 as isize));
        while (*_run_ptr).key > (*_tmp_ptr).key {
            _tmp_ptr = _tmp_ptr.offset(-1);
            _tmp_ptr;
        }
        _tmp_ptr = _tmp_ptr.offset(1);
        _tmp_ptr;
        if _tmp_ptr != _run_ptr {
            let mut _trav: *mut gk_i64kv_t = _run_ptr.offset(1 as isize);
            loop {
                _trav = _trav.offset(-1);
                if !(_trav >= _run_ptr) {
                    break;
                }
                let mut _hi_0: *mut gk_i64kv_t = 0 as *mut gk_i64kv_t;
                let mut _lo_0: *mut gk_i64kv_t = 0 as *mut gk_i64kv_t;
                _hold = *_trav;
                _lo_0 = _trav;
                _hi_0 = _lo_0;
                loop {
                    _lo_0 = _lo_0.offset(-1);
                    if !(_lo_0 >= _tmp_ptr) {
                        break;
                    }
                    *_hi_0 = *_lo_0;
                    _hi_0 = _lo_0;
                }
                *_hi_0 = _hold;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_zkvsorti(mut n: size_t, mut base: *mut gk_zkv_t) {
    let _base: *mut gk_zkv_t = base;
    let _elems: size_t = n;
    let mut _hold: gk_zkv_t = gk_zkv_t { key: 0, val: 0 };
    if _elems == 0 as libc::c_int as u64 {
        return;
    }
    if _elems > 4 as libc::c_int as u64 {
        let mut _lo: *mut gk_zkv_t = _base;
        let mut _hi: *mut gk_zkv_t = _lo
            .offset(_elems as isize)
            .offset(-(1 as isize));
        let mut _stack: [C2RustUnnamed_17; 64] = [C2RustUnnamed_17 {
            _hi: 0 as *mut gk_zkv_t,
            _lo: 0 as *mut gk_zkv_t,
        }; 64];
        let mut _top: *mut C2RustUnnamed_17 = _stack
            .as_mut_ptr()
            .offset(1 as isize);
        while _stack.as_mut_ptr() < _top {
            let mut _left_ptr: *mut gk_zkv_t = 0 as *mut gk_zkv_t;
            let mut _right_ptr: *mut gk_zkv_t = 0 as *mut gk_zkv_t;
            let mut _mid: *mut gk_zkv_t = _lo
                .offset(
                    (_hi.offset_from(_lo) as i64 >> 1) as isize,
                );
            if (*_mid).key < (*_lo).key {
                _hold = *_mid;
                *_mid = *_lo;
                *_lo = _hold;
            }
            if (*_hi).key < (*_mid).key {
                _hold = *_mid;
                *_mid = *_hi;
                *_hi = _hold;
                if (*_mid).key < (*_lo).key {
                    _hold = *_mid;
                    *_mid = *_lo;
                    *_lo = _hold;
                }
            }
            _left_ptr = _lo.offset(1 as isize);
            _right_ptr = _hi.offset(-(1 as isize));
            loop {
                while (*_left_ptr).key < (*_mid).key {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                }
                while (*_mid).key < (*_right_ptr).key {
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                }
                if _left_ptr < _right_ptr {
                    _hold = *_left_ptr;
                    *_left_ptr = *_right_ptr;
                    *_right_ptr = _hold;
                    if _mid == _left_ptr {
                        _mid = _right_ptr;
                    } else if _mid == _right_ptr {
                        _mid = _left_ptr;
                    }
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                } else if _left_ptr == _right_ptr {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                    break;
                }
                if !(_left_ptr <= _right_ptr) {
                    break;
                }
            }
            if _right_ptr.offset_from(_lo) as i64
                <= 4 as libc::c_int as i64
            {
                if _hi.offset_from(_left_ptr) as i64
                    <= 4 as libc::c_int as i64
                {
                    _top = _top.offset(-1);
                    _top;
                    _lo = (*_top)._lo;
                    _hi = (*_top)._hi;
                } else {
                    _lo = _left_ptr;
                }
            } else if _hi.offset_from(_left_ptr) as i64
                <= 4 as libc::c_int as i64
            {
                _hi = _right_ptr;
            } else if _right_ptr.offset_from(_lo) as i64
                > _hi.offset_from(_left_ptr) as i64
            {
                (*_top)._lo = _lo;
                (*_top)._hi = _right_ptr;
                _top = _top.offset(1);
                _top;
                _lo = _left_ptr;
            } else {
                (*_top)._lo = _left_ptr;
                (*_top)._hi = _hi;
                _top = _top.offset(1);
                _top;
                _hi = _right_ptr;
            }
        }
    }
    let _end_ptr: *mut gk_zkv_t = _base
        .offset(_elems as isize)
        .offset(-(1 as isize));
    let mut _tmp_ptr: *mut gk_zkv_t = _base;
    let mut _run_ptr: *mut gk_zkv_t = 0 as *mut gk_zkv_t;
    let mut _thresh: *mut gk_zkv_t = 0 as *mut gk_zkv_t;
    _thresh = _base.offset(4 as libc::c_int as isize);
    if _thresh > _end_ptr {
        _thresh = _end_ptr;
    }
    _run_ptr = _tmp_ptr.offset(1 as isize);
    while _run_ptr <= _thresh {
        if (*_run_ptr).key < (*_tmp_ptr).key {
            _tmp_ptr = _run_ptr;
        }
        _run_ptr = _run_ptr.offset(1);
        _run_ptr;
    }
    if _tmp_ptr != _base {
        _hold = *_tmp_ptr;
        *_tmp_ptr = *_base;
        *_base = _hold;
    }
    _run_ptr = _base.offset(1 as isize);
    loop {
        _run_ptr = _run_ptr.offset(1);
        if !(_run_ptr <= _end_ptr) {
            break;
        }
        _tmp_ptr = _run_ptr.offset(-(1 as isize));
        while (*_run_ptr).key < (*_tmp_ptr).key {
            _tmp_ptr = _tmp_ptr.offset(-1);
            _tmp_ptr;
        }
        _tmp_ptr = _tmp_ptr.offset(1);
        _tmp_ptr;
        if _tmp_ptr != _run_ptr {
            let mut _trav: *mut gk_zkv_t = _run_ptr.offset(1 as isize);
            loop {
                _trav = _trav.offset(-1);
                if !(_trav >= _run_ptr) {
                    break;
                }
                let mut _hi_0: *mut gk_zkv_t = 0 as *mut gk_zkv_t;
                let mut _lo_0: *mut gk_zkv_t = 0 as *mut gk_zkv_t;
                _hold = *_trav;
                _lo_0 = _trav;
                _hi_0 = _lo_0;
                loop {
                    _lo_0 = _lo_0.offset(-1);
                    if !(_lo_0 >= _tmp_ptr) {
                        break;
                    }
                    *_hi_0 = *_lo_0;
                    _hi_0 = _lo_0;
                }
                *_hi_0 = _hold;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_zkvsortd(mut n: size_t, mut base: *mut gk_zkv_t) {
    let _base: *mut gk_zkv_t = base;
    let _elems: size_t = n;
    let mut _hold: gk_zkv_t = gk_zkv_t { key: 0, val: 0 };
    if _elems == 0 as libc::c_int as u64 {
        return;
    }
    if _elems > 4 as libc::c_int as u64 {
        let mut _lo: *mut gk_zkv_t = _base;
        let mut _hi: *mut gk_zkv_t = _lo
            .offset(_elems as isize)
            .offset(-(1 as isize));
        let mut _stack: [C2RustUnnamed_18; 64] = [C2RustUnnamed_18 {
            _hi: 0 as *mut gk_zkv_t,
            _lo: 0 as *mut gk_zkv_t,
        }; 64];
        let mut _top: *mut C2RustUnnamed_18 = _stack
            .as_mut_ptr()
            .offset(1 as isize);
        while _stack.as_mut_ptr() < _top {
            let mut _left_ptr: *mut gk_zkv_t = 0 as *mut gk_zkv_t;
            let mut _right_ptr: *mut gk_zkv_t = 0 as *mut gk_zkv_t;
            let mut _mid: *mut gk_zkv_t = _lo
                .offset(
                    (_hi.offset_from(_lo) as i64 >> 1) as isize,
                );
            if (*_mid).key > (*_lo).key {
                _hold = *_mid;
                *_mid = *_lo;
                *_lo = _hold;
            }
            if (*_hi).key > (*_mid).key {
                _hold = *_mid;
                *_mid = *_hi;
                *_hi = _hold;
                if (*_mid).key > (*_lo).key {
                    _hold = *_mid;
                    *_mid = *_lo;
                    *_lo = _hold;
                }
            }
            _left_ptr = _lo.offset(1 as isize);
            _right_ptr = _hi.offset(-(1 as isize));
            loop {
                while (*_left_ptr).key > (*_mid).key {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                }
                while (*_mid).key > (*_right_ptr).key {
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                }
                if _left_ptr < _right_ptr {
                    _hold = *_left_ptr;
                    *_left_ptr = *_right_ptr;
                    *_right_ptr = _hold;
                    if _mid == _left_ptr {
                        _mid = _right_ptr;
                    } else if _mid == _right_ptr {
                        _mid = _left_ptr;
                    }
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                } else if _left_ptr == _right_ptr {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                    break;
                }
                if !(_left_ptr <= _right_ptr) {
                    break;
                }
            }
            if _right_ptr.offset_from(_lo) as i64
                <= 4 as libc::c_int as i64
            {
                if _hi.offset_from(_left_ptr) as i64
                    <= 4 as libc::c_int as i64
                {
                    _top = _top.offset(-1);
                    _top;
                    _lo = (*_top)._lo;
                    _hi = (*_top)._hi;
                } else {
                    _lo = _left_ptr;
                }
            } else if _hi.offset_from(_left_ptr) as i64
                <= 4 as libc::c_int as i64
            {
                _hi = _right_ptr;
            } else if _right_ptr.offset_from(_lo) as i64
                > _hi.offset_from(_left_ptr) as i64
            {
                (*_top)._lo = _lo;
                (*_top)._hi = _right_ptr;
                _top = _top.offset(1);
                _top;
                _lo = _left_ptr;
            } else {
                (*_top)._lo = _left_ptr;
                (*_top)._hi = _hi;
                _top = _top.offset(1);
                _top;
                _hi = _right_ptr;
            }
        }
    }
    let _end_ptr: *mut gk_zkv_t = _base
        .offset(_elems as isize)
        .offset(-(1 as isize));
    let mut _tmp_ptr: *mut gk_zkv_t = _base;
    let mut _run_ptr: *mut gk_zkv_t = 0 as *mut gk_zkv_t;
    let mut _thresh: *mut gk_zkv_t = 0 as *mut gk_zkv_t;
    _thresh = _base.offset(4 as libc::c_int as isize);
    if _thresh > _end_ptr {
        _thresh = _end_ptr;
    }
    _run_ptr = _tmp_ptr.offset(1 as isize);
    while _run_ptr <= _thresh {
        if (*_run_ptr).key > (*_tmp_ptr).key {
            _tmp_ptr = _run_ptr;
        }
        _run_ptr = _run_ptr.offset(1);
        _run_ptr;
    }
    if _tmp_ptr != _base {
        _hold = *_tmp_ptr;
        *_tmp_ptr = *_base;
        *_base = _hold;
    }
    _run_ptr = _base.offset(1 as isize);
    loop {
        _run_ptr = _run_ptr.offset(1);
        if !(_run_ptr <= _end_ptr) {
            break;
        }
        _tmp_ptr = _run_ptr.offset(-(1 as isize));
        while (*_run_ptr).key > (*_tmp_ptr).key {
            _tmp_ptr = _tmp_ptr.offset(-1);
            _tmp_ptr;
        }
        _tmp_ptr = _tmp_ptr.offset(1);
        _tmp_ptr;
        if _tmp_ptr != _run_ptr {
            let mut _trav: *mut gk_zkv_t = _run_ptr.offset(1 as isize);
            loop {
                _trav = _trav.offset(-1);
                if !(_trav >= _run_ptr) {
                    break;
                }
                let mut _hi_0: *mut gk_zkv_t = 0 as *mut gk_zkv_t;
                let mut _lo_0: *mut gk_zkv_t = 0 as *mut gk_zkv_t;
                _hold = *_trav;
                _lo_0 = _trav;
                _hi_0 = _lo_0;
                loop {
                    _lo_0 = _lo_0.offset(-1);
                    if !(_lo_0 >= _tmp_ptr) {
                        break;
                    }
                    *_hi_0 = *_lo_0;
                    _hi_0 = _lo_0;
                }
                *_hi_0 = _hold;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_fkvsorti(mut n: size_t, mut base: *mut gk_fkv_t) {
    let _base: *mut gk_fkv_t = base;
    let _elems: size_t = n;
    let mut _hold: gk_fkv_t = gk_fkv_t { key: 0., val: 0 };
    if _elems == 0 as libc::c_int as u64 {
        return;
    }
    if _elems > 4 as libc::c_int as u64 {
        let mut _lo: *mut gk_fkv_t = _base;
        let mut _hi: *mut gk_fkv_t = _lo
            .offset(_elems as isize)
            .offset(-(1 as isize));
        let mut _stack: [C2RustUnnamed_19; 64] = [C2RustUnnamed_19 {
            _hi: 0 as *mut gk_fkv_t,
            _lo: 0 as *mut gk_fkv_t,
        }; 64];
        let mut _top: *mut C2RustUnnamed_19 = _stack
            .as_mut_ptr()
            .offset(1 as isize);
        while _stack.as_mut_ptr() < _top {
            let mut _left_ptr: *mut gk_fkv_t = 0 as *mut gk_fkv_t;
            let mut _right_ptr: *mut gk_fkv_t = 0 as *mut gk_fkv_t;
            let mut _mid: *mut gk_fkv_t = _lo
                .offset(
                    (_hi.offset_from(_lo) as i64 >> 1) as isize,
                );
            if (*_mid).key < (*_lo).key {
                _hold = *_mid;
                *_mid = *_lo;
                *_lo = _hold;
            }
            if (*_hi).key < (*_mid).key {
                _hold = *_mid;
                *_mid = *_hi;
                *_hi = _hold;
                if (*_mid).key < (*_lo).key {
                    _hold = *_mid;
                    *_mid = *_lo;
                    *_lo = _hold;
                }
            }
            _left_ptr = _lo.offset(1 as isize);
            _right_ptr = _hi.offset(-(1 as isize));
            loop {
                while (*_left_ptr).key < (*_mid).key {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                }
                while (*_mid).key < (*_right_ptr).key {
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                }
                if _left_ptr < _right_ptr {
                    _hold = *_left_ptr;
                    *_left_ptr = *_right_ptr;
                    *_right_ptr = _hold;
                    if _mid == _left_ptr {
                        _mid = _right_ptr;
                    } else if _mid == _right_ptr {
                        _mid = _left_ptr;
                    }
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                } else if _left_ptr == _right_ptr {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                    break;
                }
                if !(_left_ptr <= _right_ptr) {
                    break;
                }
            }
            if _right_ptr.offset_from(_lo) as i64
                <= 4 as libc::c_int as i64
            {
                if _hi.offset_from(_left_ptr) as i64
                    <= 4 as libc::c_int as i64
                {
                    _top = _top.offset(-1);
                    _top;
                    _lo = (*_top)._lo;
                    _hi = (*_top)._hi;
                } else {
                    _lo = _left_ptr;
                }
            } else if _hi.offset_from(_left_ptr) as i64
                <= 4 as libc::c_int as i64
            {
                _hi = _right_ptr;
            } else if _right_ptr.offset_from(_lo) as i64
                > _hi.offset_from(_left_ptr) as i64
            {
                (*_top)._lo = _lo;
                (*_top)._hi = _right_ptr;
                _top = _top.offset(1);
                _top;
                _lo = _left_ptr;
            } else {
                (*_top)._lo = _left_ptr;
                (*_top)._hi = _hi;
                _top = _top.offset(1);
                _top;
                _hi = _right_ptr;
            }
        }
    }
    let _end_ptr: *mut gk_fkv_t = _base
        .offset(_elems as isize)
        .offset(-(1 as isize));
    let mut _tmp_ptr: *mut gk_fkv_t = _base;
    let mut _run_ptr: *mut gk_fkv_t = 0 as *mut gk_fkv_t;
    let mut _thresh: *mut gk_fkv_t = 0 as *mut gk_fkv_t;
    _thresh = _base.offset(4 as libc::c_int as isize);
    if _thresh > _end_ptr {
        _thresh = _end_ptr;
    }
    _run_ptr = _tmp_ptr.offset(1 as isize);
    while _run_ptr <= _thresh {
        if (*_run_ptr).key < (*_tmp_ptr).key {
            _tmp_ptr = _run_ptr;
        }
        _run_ptr = _run_ptr.offset(1);
        _run_ptr;
    }
    if _tmp_ptr != _base {
        _hold = *_tmp_ptr;
        *_tmp_ptr = *_base;
        *_base = _hold;
    }
    _run_ptr = _base.offset(1 as isize);
    loop {
        _run_ptr = _run_ptr.offset(1);
        if !(_run_ptr <= _end_ptr) {
            break;
        }
        _tmp_ptr = _run_ptr.offset(-(1 as isize));
        while (*_run_ptr).key < (*_tmp_ptr).key {
            _tmp_ptr = _tmp_ptr.offset(-1);
            _tmp_ptr;
        }
        _tmp_ptr = _tmp_ptr.offset(1);
        _tmp_ptr;
        if _tmp_ptr != _run_ptr {
            let mut _trav: *mut gk_fkv_t = _run_ptr.offset(1 as isize);
            loop {
                _trav = _trav.offset(-1);
                if !(_trav >= _run_ptr) {
                    break;
                }
                let mut _hi_0: *mut gk_fkv_t = 0 as *mut gk_fkv_t;
                let mut _lo_0: *mut gk_fkv_t = 0 as *mut gk_fkv_t;
                _hold = *_trav;
                _lo_0 = _trav;
                _hi_0 = _lo_0;
                loop {
                    _lo_0 = _lo_0.offset(-1);
                    if !(_lo_0 >= _tmp_ptr) {
                        break;
                    }
                    *_hi_0 = *_lo_0;
                    _hi_0 = _lo_0;
                }
                *_hi_0 = _hold;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_fkvsortd(mut n: size_t, mut base: *mut gk_fkv_t) {
    let _base: *mut gk_fkv_t = base;
    let _elems: size_t = n;
    let mut _hold: gk_fkv_t = gk_fkv_t { key: 0., val: 0 };
    if _elems == 0 as libc::c_int as u64 {
        return;
    }
    if _elems > 4 as libc::c_int as u64 {
        let mut _lo: *mut gk_fkv_t = _base;
        let mut _hi: *mut gk_fkv_t = _lo
            .offset(_elems as isize)
            .offset(-(1 as isize));
        let mut _stack: [C2RustUnnamed_20; 64] = [C2RustUnnamed_20 {
            _hi: 0 as *mut gk_fkv_t,
            _lo: 0 as *mut gk_fkv_t,
        }; 64];
        let mut _top: *mut C2RustUnnamed_20 = _stack
            .as_mut_ptr()
            .offset(1 as isize);
        while _stack.as_mut_ptr() < _top {
            let mut _left_ptr: *mut gk_fkv_t = 0 as *mut gk_fkv_t;
            let mut _right_ptr: *mut gk_fkv_t = 0 as *mut gk_fkv_t;
            let mut _mid: *mut gk_fkv_t = _lo
                .offset(
                    (_hi.offset_from(_lo) as i64 >> 1) as isize,
                );
            if (*_mid).key > (*_lo).key {
                _hold = *_mid;
                *_mid = *_lo;
                *_lo = _hold;
            }
            if (*_hi).key > (*_mid).key {
                _hold = *_mid;
                *_mid = *_hi;
                *_hi = _hold;
                if (*_mid).key > (*_lo).key {
                    _hold = *_mid;
                    *_mid = *_lo;
                    *_lo = _hold;
                }
            }
            _left_ptr = _lo.offset(1 as isize);
            _right_ptr = _hi.offset(-(1 as isize));
            loop {
                while (*_left_ptr).key > (*_mid).key {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                }
                while (*_mid).key > (*_right_ptr).key {
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                }
                if _left_ptr < _right_ptr {
                    _hold = *_left_ptr;
                    *_left_ptr = *_right_ptr;
                    *_right_ptr = _hold;
                    if _mid == _left_ptr {
                        _mid = _right_ptr;
                    } else if _mid == _right_ptr {
                        _mid = _left_ptr;
                    }
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                } else if _left_ptr == _right_ptr {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                    break;
                }
                if !(_left_ptr <= _right_ptr) {
                    break;
                }
            }
            if _right_ptr.offset_from(_lo) as i64
                <= 4 as libc::c_int as i64
            {
                if _hi.offset_from(_left_ptr) as i64
                    <= 4 as libc::c_int as i64
                {
                    _top = _top.offset(-1);
                    _top;
                    _lo = (*_top)._lo;
                    _hi = (*_top)._hi;
                } else {
                    _lo = _left_ptr;
                }
            } else if _hi.offset_from(_left_ptr) as i64
                <= 4 as libc::c_int as i64
            {
                _hi = _right_ptr;
            } else if _right_ptr.offset_from(_lo) as i64
                > _hi.offset_from(_left_ptr) as i64
            {
                (*_top)._lo = _lo;
                (*_top)._hi = _right_ptr;
                _top = _top.offset(1);
                _top;
                _lo = _left_ptr;
            } else {
                (*_top)._lo = _left_ptr;
                (*_top)._hi = _hi;
                _top = _top.offset(1);
                _top;
                _hi = _right_ptr;
            }
        }
    }
    let _end_ptr: *mut gk_fkv_t = _base
        .offset(_elems as isize)
        .offset(-(1 as isize));
    let mut _tmp_ptr: *mut gk_fkv_t = _base;
    let mut _run_ptr: *mut gk_fkv_t = 0 as *mut gk_fkv_t;
    let mut _thresh: *mut gk_fkv_t = 0 as *mut gk_fkv_t;
    _thresh = _base.offset(4 as libc::c_int as isize);
    if _thresh > _end_ptr {
        _thresh = _end_ptr;
    }
    _run_ptr = _tmp_ptr.offset(1 as isize);
    while _run_ptr <= _thresh {
        if (*_run_ptr).key > (*_tmp_ptr).key {
            _tmp_ptr = _run_ptr;
        }
        _run_ptr = _run_ptr.offset(1);
        _run_ptr;
    }
    if _tmp_ptr != _base {
        _hold = *_tmp_ptr;
        *_tmp_ptr = *_base;
        *_base = _hold;
    }
    _run_ptr = _base.offset(1 as isize);
    loop {
        _run_ptr = _run_ptr.offset(1);
        if !(_run_ptr <= _end_ptr) {
            break;
        }
        _tmp_ptr = _run_ptr.offset(-(1 as isize));
        while (*_run_ptr).key > (*_tmp_ptr).key {
            _tmp_ptr = _tmp_ptr.offset(-1);
            _tmp_ptr;
        }
        _tmp_ptr = _tmp_ptr.offset(1);
        _tmp_ptr;
        if _tmp_ptr != _run_ptr {
            let mut _trav: *mut gk_fkv_t = _run_ptr.offset(1 as isize);
            loop {
                _trav = _trav.offset(-1);
                if !(_trav >= _run_ptr) {
                    break;
                }
                let mut _hi_0: *mut gk_fkv_t = 0 as *mut gk_fkv_t;
                let mut _lo_0: *mut gk_fkv_t = 0 as *mut gk_fkv_t;
                _hold = *_trav;
                _lo_0 = _trav;
                _hi_0 = _lo_0;
                loop {
                    _lo_0 = _lo_0.offset(-1);
                    if !(_lo_0 >= _tmp_ptr) {
                        break;
                    }
                    *_hi_0 = *_lo_0;
                    _hi_0 = _lo_0;
                }
                *_hi_0 = _hold;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_dkvsorti(mut n: size_t, mut base: *mut gk_dkv_t) {
    let _base: *mut gk_dkv_t = base;
    let _elems: size_t = n;
    let mut _hold: gk_dkv_t = gk_dkv_t { key: 0., val: 0 };
    if _elems == 0 as libc::c_int as u64 {
        return;
    }
    if _elems > 4 as libc::c_int as u64 {
        let mut _lo: *mut gk_dkv_t = _base;
        let mut _hi: *mut gk_dkv_t = _lo
            .offset(_elems as isize)
            .offset(-(1 as isize));
        let mut _stack: [C2RustUnnamed_21; 64] = [C2RustUnnamed_21 {
            _hi: 0 as *mut gk_dkv_t,
            _lo: 0 as *mut gk_dkv_t,
        }; 64];
        let mut _top: *mut C2RustUnnamed_21 = _stack
            .as_mut_ptr()
            .offset(1 as isize);
        while _stack.as_mut_ptr() < _top {
            let mut _left_ptr: *mut gk_dkv_t = 0 as *mut gk_dkv_t;
            let mut _right_ptr: *mut gk_dkv_t = 0 as *mut gk_dkv_t;
            let mut _mid: *mut gk_dkv_t = _lo
                .offset(
                    (_hi.offset_from(_lo) as i64 >> 1) as isize,
                );
            if (*_mid).key < (*_lo).key {
                _hold = *_mid;
                *_mid = *_lo;
                *_lo = _hold;
            }
            if (*_hi).key < (*_mid).key {
                _hold = *_mid;
                *_mid = *_hi;
                *_hi = _hold;
                if (*_mid).key < (*_lo).key {
                    _hold = *_mid;
                    *_mid = *_lo;
                    *_lo = _hold;
                }
            }
            _left_ptr = _lo.offset(1 as isize);
            _right_ptr = _hi.offset(-(1 as isize));
            loop {
                while (*_left_ptr).key < (*_mid).key {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                }
                while (*_mid).key < (*_right_ptr).key {
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                }
                if _left_ptr < _right_ptr {
                    _hold = *_left_ptr;
                    *_left_ptr = *_right_ptr;
                    *_right_ptr = _hold;
                    if _mid == _left_ptr {
                        _mid = _right_ptr;
                    } else if _mid == _right_ptr {
                        _mid = _left_ptr;
                    }
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                } else if _left_ptr == _right_ptr {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                    break;
                }
                if !(_left_ptr <= _right_ptr) {
                    break;
                }
            }
            if _right_ptr.offset_from(_lo) as i64
                <= 4 as libc::c_int as i64
            {
                if _hi.offset_from(_left_ptr) as i64
                    <= 4 as libc::c_int as i64
                {
                    _top = _top.offset(-1);
                    _top;
                    _lo = (*_top)._lo;
                    _hi = (*_top)._hi;
                } else {
                    _lo = _left_ptr;
                }
            } else if _hi.offset_from(_left_ptr) as i64
                <= 4 as libc::c_int as i64
            {
                _hi = _right_ptr;
            } else if _right_ptr.offset_from(_lo) as i64
                > _hi.offset_from(_left_ptr) as i64
            {
                (*_top)._lo = _lo;
                (*_top)._hi = _right_ptr;
                _top = _top.offset(1);
                _top;
                _lo = _left_ptr;
            } else {
                (*_top)._lo = _left_ptr;
                (*_top)._hi = _hi;
                _top = _top.offset(1);
                _top;
                _hi = _right_ptr;
            }
        }
    }
    let _end_ptr: *mut gk_dkv_t = _base
        .offset(_elems as isize)
        .offset(-(1 as isize));
    let mut _tmp_ptr: *mut gk_dkv_t = _base;
    let mut _run_ptr: *mut gk_dkv_t = 0 as *mut gk_dkv_t;
    let mut _thresh: *mut gk_dkv_t = 0 as *mut gk_dkv_t;
    _thresh = _base.offset(4 as libc::c_int as isize);
    if _thresh > _end_ptr {
        _thresh = _end_ptr;
    }
    _run_ptr = _tmp_ptr.offset(1 as isize);
    while _run_ptr <= _thresh {
        if (*_run_ptr).key < (*_tmp_ptr).key {
            _tmp_ptr = _run_ptr;
        }
        _run_ptr = _run_ptr.offset(1);
        _run_ptr;
    }
    if _tmp_ptr != _base {
        _hold = *_tmp_ptr;
        *_tmp_ptr = *_base;
        *_base = _hold;
    }
    _run_ptr = _base.offset(1 as isize);
    loop {
        _run_ptr = _run_ptr.offset(1);
        if !(_run_ptr <= _end_ptr) {
            break;
        }
        _tmp_ptr = _run_ptr.offset(-(1 as isize));
        while (*_run_ptr).key < (*_tmp_ptr).key {
            _tmp_ptr = _tmp_ptr.offset(-1);
            _tmp_ptr;
        }
        _tmp_ptr = _tmp_ptr.offset(1);
        _tmp_ptr;
        if _tmp_ptr != _run_ptr {
            let mut _trav: *mut gk_dkv_t = _run_ptr.offset(1 as isize);
            loop {
                _trav = _trav.offset(-1);
                if !(_trav >= _run_ptr) {
                    break;
                }
                let mut _hi_0: *mut gk_dkv_t = 0 as *mut gk_dkv_t;
                let mut _lo_0: *mut gk_dkv_t = 0 as *mut gk_dkv_t;
                _hold = *_trav;
                _lo_0 = _trav;
                _hi_0 = _lo_0;
                loop {
                    _lo_0 = _lo_0.offset(-1);
                    if !(_lo_0 >= _tmp_ptr) {
                        break;
                    }
                    *_hi_0 = *_lo_0;
                    _hi_0 = _lo_0;
                }
                *_hi_0 = _hold;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_dkvsortd(mut n: size_t, mut base: *mut gk_dkv_t) {
    let _base: *mut gk_dkv_t = base;
    let _elems: size_t = n;
    let mut _hold: gk_dkv_t = gk_dkv_t { key: 0., val: 0 };
    if _elems == 0 as libc::c_int as u64 {
        return;
    }
    if _elems > 4 as libc::c_int as u64 {
        let mut _lo: *mut gk_dkv_t = _base;
        let mut _hi: *mut gk_dkv_t = _lo
            .offset(_elems as isize)
            .offset(-(1 as isize));
        let mut _stack: [C2RustUnnamed_22; 64] = [C2RustUnnamed_22 {
            _hi: 0 as *mut gk_dkv_t,
            _lo: 0 as *mut gk_dkv_t,
        }; 64];
        let mut _top: *mut C2RustUnnamed_22 = _stack
            .as_mut_ptr()
            .offset(1 as isize);
        while _stack.as_mut_ptr() < _top {
            let mut _left_ptr: *mut gk_dkv_t = 0 as *mut gk_dkv_t;
            let mut _right_ptr: *mut gk_dkv_t = 0 as *mut gk_dkv_t;
            let mut _mid: *mut gk_dkv_t = _lo
                .offset(
                    (_hi.offset_from(_lo) as i64 >> 1) as isize,
                );
            if (*_mid).key > (*_lo).key {
                _hold = *_mid;
                *_mid = *_lo;
                *_lo = _hold;
            }
            if (*_hi).key > (*_mid).key {
                _hold = *_mid;
                *_mid = *_hi;
                *_hi = _hold;
                if (*_mid).key > (*_lo).key {
                    _hold = *_mid;
                    *_mid = *_lo;
                    *_lo = _hold;
                }
            }
            _left_ptr = _lo.offset(1 as isize);
            _right_ptr = _hi.offset(-(1 as isize));
            loop {
                while (*_left_ptr).key > (*_mid).key {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                }
                while (*_mid).key > (*_right_ptr).key {
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                }
                if _left_ptr < _right_ptr {
                    _hold = *_left_ptr;
                    *_left_ptr = *_right_ptr;
                    *_right_ptr = _hold;
                    if _mid == _left_ptr {
                        _mid = _right_ptr;
                    } else if _mid == _right_ptr {
                        _mid = _left_ptr;
                    }
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                } else if _left_ptr == _right_ptr {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                    break;
                }
                if !(_left_ptr <= _right_ptr) {
                    break;
                }
            }
            if _right_ptr.offset_from(_lo) as i64
                <= 4 as libc::c_int as i64
            {
                if _hi.offset_from(_left_ptr) as i64
                    <= 4 as libc::c_int as i64
                {
                    _top = _top.offset(-1);
                    _top;
                    _lo = (*_top)._lo;
                    _hi = (*_top)._hi;
                } else {
                    _lo = _left_ptr;
                }
            } else if _hi.offset_from(_left_ptr) as i64
                <= 4 as libc::c_int as i64
            {
                _hi = _right_ptr;
            } else if _right_ptr.offset_from(_lo) as i64
                > _hi.offset_from(_left_ptr) as i64
            {
                (*_top)._lo = _lo;
                (*_top)._hi = _right_ptr;
                _top = _top.offset(1);
                _top;
                _lo = _left_ptr;
            } else {
                (*_top)._lo = _left_ptr;
                (*_top)._hi = _hi;
                _top = _top.offset(1);
                _top;
                _hi = _right_ptr;
            }
        }
    }
    let _end_ptr: *mut gk_dkv_t = _base
        .offset(_elems as isize)
        .offset(-(1 as isize));
    let mut _tmp_ptr: *mut gk_dkv_t = _base;
    let mut _run_ptr: *mut gk_dkv_t = 0 as *mut gk_dkv_t;
    let mut _thresh: *mut gk_dkv_t = 0 as *mut gk_dkv_t;
    _thresh = _base.offset(4 as libc::c_int as isize);
    if _thresh > _end_ptr {
        _thresh = _end_ptr;
    }
    _run_ptr = _tmp_ptr.offset(1 as isize);
    while _run_ptr <= _thresh {
        if (*_run_ptr).key > (*_tmp_ptr).key {
            _tmp_ptr = _run_ptr;
        }
        _run_ptr = _run_ptr.offset(1);
        _run_ptr;
    }
    if _tmp_ptr != _base {
        _hold = *_tmp_ptr;
        *_tmp_ptr = *_base;
        *_base = _hold;
    }
    _run_ptr = _base.offset(1 as isize);
    loop {
        _run_ptr = _run_ptr.offset(1);
        if !(_run_ptr <= _end_ptr) {
            break;
        }
        _tmp_ptr = _run_ptr.offset(-(1 as isize));
        while (*_run_ptr).key > (*_tmp_ptr).key {
            _tmp_ptr = _tmp_ptr.offset(-1);
            _tmp_ptr;
        }
        _tmp_ptr = _tmp_ptr.offset(1);
        _tmp_ptr;
        if _tmp_ptr != _run_ptr {
            let mut _trav: *mut gk_dkv_t = _run_ptr.offset(1 as isize);
            loop {
                _trav = _trav.offset(-1);
                if !(_trav >= _run_ptr) {
                    break;
                }
                let mut _hi_0: *mut gk_dkv_t = 0 as *mut gk_dkv_t;
                let mut _lo_0: *mut gk_dkv_t = 0 as *mut gk_dkv_t;
                _hold = *_trav;
                _lo_0 = _trav;
                _hi_0 = _lo_0;
                loop {
                    _lo_0 = _lo_0.offset(-1);
                    if !(_lo_0 >= _tmp_ptr) {
                        break;
                    }
                    *_hi_0 = *_lo_0;
                    _hi_0 = _lo_0;
                }
                *_hi_0 = _hold;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_skvsorti(mut n: size_t, mut base: *mut gk_skv_t) {
    let _base: *mut gk_skv_t = base;
    let _elems: size_t = n;
    let mut _hold: gk_skv_t = gk_skv_t {
        key: 0 as *mut libc::c_char,
        val: 0,
    };
    if _elems == 0 as libc::c_int as u64 {
        return;
    }
    if _elems > 4 as libc::c_int as u64 {
        let mut _lo: *mut gk_skv_t = _base;
        let mut _hi: *mut gk_skv_t = _lo
            .offset(_elems as isize)
            .offset(-(1 as isize));
        let mut _stack: [C2RustUnnamed_23; 64] = [C2RustUnnamed_23 {
            _hi: 0 as *mut gk_skv_t,
            _lo: 0 as *mut gk_skv_t,
        }; 64];
        let mut _top: *mut C2RustUnnamed_23 = _stack
            .as_mut_ptr()
            .offset(1 as isize);
        while _stack.as_mut_ptr() < _top {
            let mut _left_ptr: *mut gk_skv_t = 0 as *mut gk_skv_t;
            let mut _right_ptr: *mut gk_skv_t = 0 as *mut gk_skv_t;
            let mut _mid: *mut gk_skv_t = _lo
                .offset(
                    (_hi.offset_from(_lo) as i64 >> 1) as isize,
                );
            if strcmp((*_mid).key, (*_lo).key) < 0 as libc::c_int {
                _hold = *_mid;
                *_mid = *_lo;
                *_lo = _hold;
            }
            if strcmp((*_hi).key, (*_mid).key) < 0 as libc::c_int {
                _hold = *_mid;
                *_mid = *_hi;
                *_hi = _hold;
                if strcmp((*_mid).key, (*_lo).key) < 0 as libc::c_int {
                    _hold = *_mid;
                    *_mid = *_lo;
                    *_lo = _hold;
                }
            }
            _left_ptr = _lo.offset(1 as isize);
            _right_ptr = _hi.offset(-(1 as isize));
            loop {
                while strcmp((*_left_ptr).key, (*_mid).key) < 0 as libc::c_int {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                }
                while strcmp((*_mid).key, (*_right_ptr).key) < 0 as libc::c_int {
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                }
                if _left_ptr < _right_ptr {
                    _hold = *_left_ptr;
                    *_left_ptr = *_right_ptr;
                    *_right_ptr = _hold;
                    if _mid == _left_ptr {
                        _mid = _right_ptr;
                    } else if _mid == _right_ptr {
                        _mid = _left_ptr;
                    }
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                } else if _left_ptr == _right_ptr {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                    break;
                }
                if !(_left_ptr <= _right_ptr) {
                    break;
                }
            }
            if _right_ptr.offset_from(_lo) as i64
                <= 4 as libc::c_int as i64
            {
                if _hi.offset_from(_left_ptr) as i64
                    <= 4 as libc::c_int as i64
                {
                    _top = _top.offset(-1);
                    _top;
                    _lo = (*_top)._lo;
                    _hi = (*_top)._hi;
                } else {
                    _lo = _left_ptr;
                }
            } else if _hi.offset_from(_left_ptr) as i64
                <= 4 as libc::c_int as i64
            {
                _hi = _right_ptr;
            } else if _right_ptr.offset_from(_lo) as i64
                > _hi.offset_from(_left_ptr) as i64
            {
                (*_top)._lo = _lo;
                (*_top)._hi = _right_ptr;
                _top = _top.offset(1);
                _top;
                _lo = _left_ptr;
            } else {
                (*_top)._lo = _left_ptr;
                (*_top)._hi = _hi;
                _top = _top.offset(1);
                _top;
                _hi = _right_ptr;
            }
        }
    }
    let _end_ptr: *mut gk_skv_t = _base
        .offset(_elems as isize)
        .offset(-(1 as isize));
    let mut _tmp_ptr: *mut gk_skv_t = _base;
    let mut _run_ptr: *mut gk_skv_t = 0 as *mut gk_skv_t;
    let mut _thresh: *mut gk_skv_t = 0 as *mut gk_skv_t;
    _thresh = _base.offset(4 as libc::c_int as isize);
    if _thresh > _end_ptr {
        _thresh = _end_ptr;
    }
    _run_ptr = _tmp_ptr.offset(1 as isize);
    while _run_ptr <= _thresh {
        if strcmp((*_run_ptr).key, (*_tmp_ptr).key) < 0 as libc::c_int {
            _tmp_ptr = _run_ptr;
        }
        _run_ptr = _run_ptr.offset(1);
        _run_ptr;
    }
    if _tmp_ptr != _base {
        _hold = *_tmp_ptr;
        *_tmp_ptr = *_base;
        *_base = _hold;
    }
    _run_ptr = _base.offset(1 as isize);
    loop {
        _run_ptr = _run_ptr.offset(1);
        if !(_run_ptr <= _end_ptr) {
            break;
        }
        _tmp_ptr = _run_ptr.offset(-(1 as isize));
        while strcmp((*_run_ptr).key, (*_tmp_ptr).key) < 0 as libc::c_int {
            _tmp_ptr = _tmp_ptr.offset(-1);
            _tmp_ptr;
        }
        _tmp_ptr = _tmp_ptr.offset(1);
        _tmp_ptr;
        if _tmp_ptr != _run_ptr {
            let mut _trav: *mut gk_skv_t = _run_ptr.offset(1 as isize);
            loop {
                _trav = _trav.offset(-1);
                if !(_trav >= _run_ptr) {
                    break;
                }
                let mut _hi_0: *mut gk_skv_t = 0 as *mut gk_skv_t;
                let mut _lo_0: *mut gk_skv_t = 0 as *mut gk_skv_t;
                _hold = *_trav;
                _lo_0 = _trav;
                _hi_0 = _lo_0;
                loop {
                    _lo_0 = _lo_0.offset(-1);
                    if !(_lo_0 >= _tmp_ptr) {
                        break;
                    }
                    *_hi_0 = *_lo_0;
                    _hi_0 = _lo_0;
                }
                *_hi_0 = _hold;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_skvsortd(mut n: size_t, mut base: *mut gk_skv_t) {
    let _base: *mut gk_skv_t = base;
    let _elems: size_t = n;
    let mut _hold: gk_skv_t = gk_skv_t {
        key: 0 as *mut libc::c_char,
        val: 0,
    };
    if _elems == 0 as libc::c_int as u64 {
        return;
    }
    if _elems > 4 as libc::c_int as u64 {
        let mut _lo: *mut gk_skv_t = _base;
        let mut _hi: *mut gk_skv_t = _lo
            .offset(_elems as isize)
            .offset(-(1 as isize));
        let mut _stack: [C2RustUnnamed_24; 64] = [C2RustUnnamed_24 {
            _hi: 0 as *mut gk_skv_t,
            _lo: 0 as *mut gk_skv_t,
        }; 64];
        let mut _top: *mut C2RustUnnamed_24 = _stack
            .as_mut_ptr()
            .offset(1 as isize);
        while _stack.as_mut_ptr() < _top {
            let mut _left_ptr: *mut gk_skv_t = 0 as *mut gk_skv_t;
            let mut _right_ptr: *mut gk_skv_t = 0 as *mut gk_skv_t;
            let mut _mid: *mut gk_skv_t = _lo
                .offset(
                    (_hi.offset_from(_lo) as i64 >> 1) as isize,
                );
            if strcmp((*_mid).key, (*_lo).key) > 0 as libc::c_int {
                _hold = *_mid;
                *_mid = *_lo;
                *_lo = _hold;
            }
            if strcmp((*_hi).key, (*_mid).key) > 0 as libc::c_int {
                _hold = *_mid;
                *_mid = *_hi;
                *_hi = _hold;
                if strcmp((*_mid).key, (*_lo).key) > 0 as libc::c_int {
                    _hold = *_mid;
                    *_mid = *_lo;
                    *_lo = _hold;
                }
            }
            _left_ptr = _lo.offset(1 as isize);
            _right_ptr = _hi.offset(-(1 as isize));
            loop {
                while strcmp((*_left_ptr).key, (*_mid).key) > 0 as libc::c_int {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                }
                while strcmp((*_mid).key, (*_right_ptr).key) > 0 as libc::c_int {
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                }
                if _left_ptr < _right_ptr {
                    _hold = *_left_ptr;
                    *_left_ptr = *_right_ptr;
                    *_right_ptr = _hold;
                    if _mid == _left_ptr {
                        _mid = _right_ptr;
                    } else if _mid == _right_ptr {
                        _mid = _left_ptr;
                    }
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                } else if _left_ptr == _right_ptr {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                    break;
                }
                if !(_left_ptr <= _right_ptr) {
                    break;
                }
            }
            if _right_ptr.offset_from(_lo) as i64
                <= 4 as libc::c_int as i64
            {
                if _hi.offset_from(_left_ptr) as i64
                    <= 4 as libc::c_int as i64
                {
                    _top = _top.offset(-1);
                    _top;
                    _lo = (*_top)._lo;
                    _hi = (*_top)._hi;
                } else {
                    _lo = _left_ptr;
                }
            } else if _hi.offset_from(_left_ptr) as i64
                <= 4 as libc::c_int as i64
            {
                _hi = _right_ptr;
            } else if _right_ptr.offset_from(_lo) as i64
                > _hi.offset_from(_left_ptr) as i64
            {
                (*_top)._lo = _lo;
                (*_top)._hi = _right_ptr;
                _top = _top.offset(1);
                _top;
                _lo = _left_ptr;
            } else {
                (*_top)._lo = _left_ptr;
                (*_top)._hi = _hi;
                _top = _top.offset(1);
                _top;
                _hi = _right_ptr;
            }
        }
    }
    let _end_ptr: *mut gk_skv_t = _base
        .offset(_elems as isize)
        .offset(-(1 as isize));
    let mut _tmp_ptr: *mut gk_skv_t = _base;
    let mut _run_ptr: *mut gk_skv_t = 0 as *mut gk_skv_t;
    let mut _thresh: *mut gk_skv_t = 0 as *mut gk_skv_t;
    _thresh = _base.offset(4 as libc::c_int as isize);
    if _thresh > _end_ptr {
        _thresh = _end_ptr;
    }
    _run_ptr = _tmp_ptr.offset(1 as isize);
    while _run_ptr <= _thresh {
        if strcmp((*_run_ptr).key, (*_tmp_ptr).key) > 0 as libc::c_int {
            _tmp_ptr = _run_ptr;
        }
        _run_ptr = _run_ptr.offset(1);
        _run_ptr;
    }
    if _tmp_ptr != _base {
        _hold = *_tmp_ptr;
        *_tmp_ptr = *_base;
        *_base = _hold;
    }
    _run_ptr = _base.offset(1 as isize);
    loop {
        _run_ptr = _run_ptr.offset(1);
        if !(_run_ptr <= _end_ptr) {
            break;
        }
        _tmp_ptr = _run_ptr.offset(-(1 as isize));
        while strcmp((*_run_ptr).key, (*_tmp_ptr).key) > 0 as libc::c_int {
            _tmp_ptr = _tmp_ptr.offset(-1);
            _tmp_ptr;
        }
        _tmp_ptr = _tmp_ptr.offset(1);
        _tmp_ptr;
        if _tmp_ptr != _run_ptr {
            let mut _trav: *mut gk_skv_t = _run_ptr.offset(1 as isize);
            loop {
                _trav = _trav.offset(-1);
                if !(_trav >= _run_ptr) {
                    break;
                }
                let mut _hi_0: *mut gk_skv_t = 0 as *mut gk_skv_t;
                let mut _lo_0: *mut gk_skv_t = 0 as *mut gk_skv_t;
                _hold = *_trav;
                _lo_0 = _trav;
                _hi_0 = _lo_0;
                loop {
                    _lo_0 = _lo_0.offset(-1);
                    if !(_lo_0 >= _tmp_ptr) {
                        break;
                    }
                    *_hi_0 = *_lo_0;
                    _hi_0 = _lo_0;
                }
                *_hi_0 = _hold;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_idxkvsorti(mut n: size_t, mut base: *mut gk_idxkv_t) {
    let _base: *mut gk_idxkv_t = base;
    let _elems: size_t = n;
    let mut _hold: gk_idxkv_t = gk_idxkv_t { key: 0, val: 0 };
    if _elems == 0 as libc::c_int as u64 {
        return;
    }
    if _elems > 4 as libc::c_int as u64 {
        let mut _lo: *mut gk_idxkv_t = _base;
        let mut _hi: *mut gk_idxkv_t = _lo
            .offset(_elems as isize)
            .offset(-(1 as isize));
        let mut _stack: [C2RustUnnamed_25; 64] = [C2RustUnnamed_25 {
            _hi: 0 as *mut gk_idxkv_t,
            _lo: 0 as *mut gk_idxkv_t,
        }; 64];
        let mut _top: *mut C2RustUnnamed_25 = _stack
            .as_mut_ptr()
            .offset(1 as isize);
        while _stack.as_mut_ptr() < _top {
            let mut _left_ptr: *mut gk_idxkv_t = 0 as *mut gk_idxkv_t;
            let mut _right_ptr: *mut gk_idxkv_t = 0 as *mut gk_idxkv_t;
            let mut _mid: *mut gk_idxkv_t = _lo
                .offset(
                    (_hi.offset_from(_lo) as i64 >> 1) as isize,
                );
            if (*_mid).key < (*_lo).key {
                _hold = *_mid;
                *_mid = *_lo;
                *_lo = _hold;
            }
            if (*_hi).key < (*_mid).key {
                _hold = *_mid;
                *_mid = *_hi;
                *_hi = _hold;
                if (*_mid).key < (*_lo).key {
                    _hold = *_mid;
                    *_mid = *_lo;
                    *_lo = _hold;
                }
            }
            _left_ptr = _lo.offset(1 as isize);
            _right_ptr = _hi.offset(-(1 as isize));
            loop {
                while (*_left_ptr).key < (*_mid).key {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                }
                while (*_mid).key < (*_right_ptr).key {
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                }
                if _left_ptr < _right_ptr {
                    _hold = *_left_ptr;
                    *_left_ptr = *_right_ptr;
                    *_right_ptr = _hold;
                    if _mid == _left_ptr {
                        _mid = _right_ptr;
                    } else if _mid == _right_ptr {
                        _mid = _left_ptr;
                    }
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                } else if _left_ptr == _right_ptr {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                    break;
                }
                if !(_left_ptr <= _right_ptr) {
                    break;
                }
            }
            if _right_ptr.offset_from(_lo) as i64
                <= 4 as libc::c_int as i64
            {
                if _hi.offset_from(_left_ptr) as i64
                    <= 4 as libc::c_int as i64
                {
                    _top = _top.offset(-1);
                    _top;
                    _lo = (*_top)._lo;
                    _hi = (*_top)._hi;
                } else {
                    _lo = _left_ptr;
                }
            } else if _hi.offset_from(_left_ptr) as i64
                <= 4 as libc::c_int as i64
            {
                _hi = _right_ptr;
            } else if _right_ptr.offset_from(_lo) as i64
                > _hi.offset_from(_left_ptr) as i64
            {
                (*_top)._lo = _lo;
                (*_top)._hi = _right_ptr;
                _top = _top.offset(1);
                _top;
                _lo = _left_ptr;
            } else {
                (*_top)._lo = _left_ptr;
                (*_top)._hi = _hi;
                _top = _top.offset(1);
                _top;
                _hi = _right_ptr;
            }
        }
    }
    let _end_ptr: *mut gk_idxkv_t = _base
        .offset(_elems as isize)
        .offset(-(1 as isize));
    let mut _tmp_ptr: *mut gk_idxkv_t = _base;
    let mut _run_ptr: *mut gk_idxkv_t = 0 as *mut gk_idxkv_t;
    let mut _thresh: *mut gk_idxkv_t = 0 as *mut gk_idxkv_t;
    _thresh = _base.offset(4 as libc::c_int as isize);
    if _thresh > _end_ptr {
        _thresh = _end_ptr;
    }
    _run_ptr = _tmp_ptr.offset(1 as isize);
    while _run_ptr <= _thresh {
        if (*_run_ptr).key < (*_tmp_ptr).key {
            _tmp_ptr = _run_ptr;
        }
        _run_ptr = _run_ptr.offset(1);
        _run_ptr;
    }
    if _tmp_ptr != _base {
        _hold = *_tmp_ptr;
        *_tmp_ptr = *_base;
        *_base = _hold;
    }
    _run_ptr = _base.offset(1 as isize);
    loop {
        _run_ptr = _run_ptr.offset(1);
        if !(_run_ptr <= _end_ptr) {
            break;
        }
        _tmp_ptr = _run_ptr.offset(-(1 as isize));
        while (*_run_ptr).key < (*_tmp_ptr).key {
            _tmp_ptr = _tmp_ptr.offset(-1);
            _tmp_ptr;
        }
        _tmp_ptr = _tmp_ptr.offset(1);
        _tmp_ptr;
        if _tmp_ptr != _run_ptr {
            let mut _trav: *mut gk_idxkv_t = _run_ptr.offset(1 as isize);
            loop {
                _trav = _trav.offset(-1);
                if !(_trav >= _run_ptr) {
                    break;
                }
                let mut _hi_0: *mut gk_idxkv_t = 0 as *mut gk_idxkv_t;
                let mut _lo_0: *mut gk_idxkv_t = 0 as *mut gk_idxkv_t;
                _hold = *_trav;
                _lo_0 = _trav;
                _hi_0 = _lo_0;
                loop {
                    _lo_0 = _lo_0.offset(-1);
                    if !(_lo_0 >= _tmp_ptr) {
                        break;
                    }
                    *_hi_0 = *_lo_0;
                    _hi_0 = _lo_0;
                }
                *_hi_0 = _hold;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn gk_idxkvsortd(mut n: size_t, mut base: *mut gk_idxkv_t) {
    let _base: *mut gk_idxkv_t = base;
    let _elems: size_t = n;
    let mut _hold: gk_idxkv_t = gk_idxkv_t { key: 0, val: 0 };
    if _elems == 0 as libc::c_int as u64 {
        return;
    }
    if _elems > 4 as libc::c_int as u64 {
        let mut _lo: *mut gk_idxkv_t = _base;
        let mut _hi: *mut gk_idxkv_t = _lo
            .offset(_elems as isize)
            .offset(-(1 as isize));
        let mut _stack: [C2RustUnnamed_26; 64] = [C2RustUnnamed_26 {
            _hi: 0 as *mut gk_idxkv_t,
            _lo: 0 as *mut gk_idxkv_t,
        }; 64];
        let mut _top: *mut C2RustUnnamed_26 = _stack
            .as_mut_ptr()
            .offset(1 as isize);
        while _stack.as_mut_ptr() < _top {
            let mut _left_ptr: *mut gk_idxkv_t = 0 as *mut gk_idxkv_t;
            let mut _right_ptr: *mut gk_idxkv_t = 0 as *mut gk_idxkv_t;
            let mut _mid: *mut gk_idxkv_t = _lo
                .offset(
                    (_hi.offset_from(_lo) as i64 >> 1) as isize,
                );
            if (*_mid).key > (*_lo).key {
                _hold = *_mid;
                *_mid = *_lo;
                *_lo = _hold;
            }
            if (*_hi).key > (*_mid).key {
                _hold = *_mid;
                *_mid = *_hi;
                *_hi = _hold;
                if (*_mid).key > (*_lo).key {
                    _hold = *_mid;
                    *_mid = *_lo;
                    *_lo = _hold;
                }
            }
            _left_ptr = _lo.offset(1 as isize);
            _right_ptr = _hi.offset(-(1 as isize));
            loop {
                while (*_left_ptr).key > (*_mid).key {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                }
                while (*_mid).key > (*_right_ptr).key {
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                }
                if _left_ptr < _right_ptr {
                    _hold = *_left_ptr;
                    *_left_ptr = *_right_ptr;
                    *_right_ptr = _hold;
                    if _mid == _left_ptr {
                        _mid = _right_ptr;
                    } else if _mid == _right_ptr {
                        _mid = _left_ptr;
                    }
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                } else if _left_ptr == _right_ptr {
                    _left_ptr = _left_ptr.offset(1);
                    _left_ptr;
                    _right_ptr = _right_ptr.offset(-1);
                    _right_ptr;
                    break;
                }
                if !(_left_ptr <= _right_ptr) {
                    break;
                }
            }
            if _right_ptr.offset_from(_lo) as i64
                <= 4 as libc::c_int as i64
            {
                if _hi.offset_from(_left_ptr) as i64
                    <= 4 as libc::c_int as i64
                {
                    _top = _top.offset(-1);
                    _top;
                    _lo = (*_top)._lo;
                    _hi = (*_top)._hi;
                } else {
                    _lo = _left_ptr;
                }
            } else if _hi.offset_from(_left_ptr) as i64
                <= 4 as libc::c_int as i64
            {
                _hi = _right_ptr;
            } else if _right_ptr.offset_from(_lo) as i64
                > _hi.offset_from(_left_ptr) as i64
            {
                (*_top)._lo = _lo;
                (*_top)._hi = _right_ptr;
                _top = _top.offset(1);
                _top;
                _lo = _left_ptr;
            } else {
                (*_top)._lo = _left_ptr;
                (*_top)._hi = _hi;
                _top = _top.offset(1);
                _top;
                _hi = _right_ptr;
            }
        }
    }
    let _end_ptr: *mut gk_idxkv_t = _base
        .offset(_elems as isize)
        .offset(-(1 as isize));
    let mut _tmp_ptr: *mut gk_idxkv_t = _base;
    let mut _run_ptr: *mut gk_idxkv_t = 0 as *mut gk_idxkv_t;
    let mut _thresh: *mut gk_idxkv_t = 0 as *mut gk_idxkv_t;
    _thresh = _base.offset(4 as libc::c_int as isize);
    if _thresh > _end_ptr {
        _thresh = _end_ptr;
    }
    _run_ptr = _tmp_ptr.offset(1 as isize);
    while _run_ptr <= _thresh {
        if (*_run_ptr).key > (*_tmp_ptr).key {
            _tmp_ptr = _run_ptr;
        }
        _run_ptr = _run_ptr.offset(1);
        _run_ptr;
    }
    if _tmp_ptr != _base {
        _hold = *_tmp_ptr;
        *_tmp_ptr = *_base;
        *_base = _hold;
    }
    _run_ptr = _base.offset(1 as isize);
    loop {
        _run_ptr = _run_ptr.offset(1);
        if !(_run_ptr <= _end_ptr) {
            break;
        }
        _tmp_ptr = _run_ptr.offset(-(1 as isize));
        while (*_run_ptr).key > (*_tmp_ptr).key {
            _tmp_ptr = _tmp_ptr.offset(-1);
            _tmp_ptr;
        }
        _tmp_ptr = _tmp_ptr.offset(1);
        _tmp_ptr;
        if _tmp_ptr != _run_ptr {
            let mut _trav: *mut gk_idxkv_t = _run_ptr.offset(1 as isize);
            loop {
                _trav = _trav.offset(-1);
                if !(_trav >= _run_ptr) {
                    break;
                }
                let mut _hi_0: *mut gk_idxkv_t = 0 as *mut gk_idxkv_t;
                let mut _lo_0: *mut gk_idxkv_t = 0 as *mut gk_idxkv_t;
                _hold = *_trav;
                _lo_0 = _trav;
                _hi_0 = _lo_0;
                loop {
                    _lo_0 = _lo_0.offset(-1);
                    if !(_lo_0 >= _tmp_ptr) {
                        break;
                    }
                    *_hi_0 = *_lo_0;
                    _hi_0 = _lo_0;
                }
                *_hi_0 = _hold;
            }
        }
    };
}
