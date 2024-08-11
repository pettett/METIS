use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn vfprintf(_: *mut FILE, _: *const libc::c_char, _: ::core::ffi::VaList) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn backtrace(__array: *mut *mut libc::c_void, __size: libc::c_int) -> libc::c_int;
    fn backtrace_symbols(
        __array: *const *mut libc::c_void,
        __size: libc::c_int,
    ) -> *mut *mut libc::c_char;
    fn exit(_: libc::c_int) -> !;
    fn free(_: *mut libc::c_void);
    fn strlen(_: *const libc::c_char) -> u64;
    fn strerror_r(__errnum: libc::c_int, __buf: *mut libc::c_char, __buflen: size_t)
        -> libc::c_int;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn raise(__sig: libc::c_int) -> libc::c_int;
    fn longjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
}
use crate::libmetis::structure::*;
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type __off_t = i64;
pub type __off64_t = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [u64; 16],
}
pub type va_list = __builtin_va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type __sighandler_t = Option<unsafe extern "C" fn(libc::c_int) -> ()>;
pub type __jmp_buf = [i64; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
pub type gksighandler_t = Option<unsafe extern "C" fn(libc::c_int) -> ()>;

#[thread_local]
pub static mut GK_CUR_JBUFS: libc::c_int = -(1);

#[thread_local]
pub static mut gk_jbufs: [jmp_buf; 128] = [[__jmp_buf_tag {
    __jmpbuf: [0; 8],
    __mask_was_saved: 0,
    __saved_mask: __sigset_t { __val: [0; 16] },
}; 1]; 128];

#[thread_local]
pub static mut gk_jbuf: jmp_buf = [__jmp_buf_tag {
    __jmpbuf: [0; 8],
    __mask_was_saved: 0,
    __saved_mask: __sigset_t { __val: [0; 16] },
}; 1];

#[thread_local]
static mut old_SIGMEM_handler: gksighandler_t = None;
#[thread_local]
static mut old_SIGERR_handler: gksighandler_t = None;
#[thread_local]
static mut old_SIGMEM_handlers: [gksighandler_t; 128] = [None; 128];
#[thread_local]
static mut old_SIGERR_handlers: [gksighandler_t; 128] = [None; 128];
static mut gk_exit_on_error: libc::c_int = 1;
#[no_mangle]
pub unsafe extern "C" fn gk_set_exit_on_error(mut value: libc::c_int) {
    gk_exit_on_error = value;
}
#[no_mangle]
pub unsafe extern "C" fn errexit(mut f_str: *mut libc::c_char, mut args: ...) {
    let mut argp: ::core::ffi::VaListImpl;
    argp = args.clone();
    vfprintf(stderr, f_str, argp.as_va_list());
    if strlen(f_str) == 0 as libc::c_int as u64
        || *f_str.offset((strlen(f_str)).wrapping_sub(1 as u64) as isize) as libc::c_int
            != '\n' as i32
    {
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    }
    fflush(stderr);
    if gk_exit_on_error != 0 {
        exit(-(2 as libc::c_int));
    }
}
#[no_mangle]
pub unsafe extern "C" fn gk_errexit(
    mut signum: libc::c_int,
    mut f_str: *mut libc::c_char,
    mut args: ...
) {
    let mut argp: ::core::ffi::VaListImpl;
    argp = args.clone();
    vfprintf(stderr, f_str, argp.as_va_list());
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    fflush(stderr);
    if gk_exit_on_error != 0 {
        raise(signum);
    }
}
#[no_mangle]
pub unsafe extern "C" fn gk_sigtrap() -> libc::c_int {
    if GK_CUR_JBUFS + 1 >= 128 as libc::c_int {
        return 0 as libc::c_int;
    }
    GK_CUR_JBUFS += 1;
    GK_CUR_JBUFS;
    old_SIGMEM_handlers[GK_CUR_JBUFS as usize] = signal(
        6 as libc::c_int,
        Some(gk_sigthrow as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    old_SIGERR_handlers[GK_CUR_JBUFS as usize] = signal(
        15 as libc::c_int,
        Some(gk_sigthrow as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    return 1;
}
#[no_mangle]
pub unsafe extern "C" fn gk_siguntrap() -> libc::c_int {
    if GK_CUR_JBUFS == -(1) {
        return 0 as libc::c_int;
    }
    signal(6 as libc::c_int, old_SIGMEM_handlers[GK_CUR_JBUFS as usize]);
    signal(
        15 as libc::c_int,
        old_SIGERR_handlers[GK_CUR_JBUFS as usize],
    );
    GK_CUR_JBUFS -= 1;
    GK_CUR_JBUFS;
    return 1;
}
#[no_mangle]
pub unsafe extern "C" fn gk_sigthrow(mut signum: libc::c_int) {
    longjmp((gk_jbufs[GK_CUR_JBUFS as usize]).as_mut_ptr(), signum);
}
#[no_mangle]
pub unsafe extern "C" fn gk_SetSignalHandlers() {
    old_SIGMEM_handler = signal(
        6 as libc::c_int,
        Some(gk_NonLocalExit_Handler as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    old_SIGERR_handler = signal(
        15 as libc::c_int,
        Some(gk_NonLocalExit_Handler as unsafe extern "C" fn(libc::c_int) -> ()),
    );
}
#[no_mangle]
pub unsafe extern "C" fn gk_UnsetSignalHandlers() {
    signal(6 as libc::c_int, old_SIGMEM_handler);
    signal(15 as libc::c_int, old_SIGERR_handler);
}
#[no_mangle]
pub unsafe extern "C" fn gk_NonLocalExit_Handler(mut signum: libc::c_int) {
    longjmp(gk_jbuf.as_mut_ptr(), signum);
}
#[no_mangle]
pub unsafe extern "C" fn gk_strerror(mut errnum: libc::c_int) -> *mut libc::c_char {
    #[thread_local]
    static mut buf: [libc::c_char; 1024] = [0; 1024];
    strerror_r(errnum, buf.as_mut_ptr(), 1024 as libc::c_int as size_t);
    buf[1023 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    return buf.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn PrintBackTrace() {
    let mut array: [*mut libc::c_void; 10] = [0 as *mut libc::c_void; 10];
    let mut i: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut strings: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    size = backtrace(array.as_mut_ptr(), 10 as libc::c_int);
    strings = backtrace_symbols(array.as_mut_ptr(), size);
    printf(
        b"Obtained %d stack frames.\n\0" as *const u8 as *const libc::c_char,
        size,
    );
    i = 0 as libc::c_int;
    while i < size {
        printf(
            b"%s\n\0" as *const u8 as *const libc::c_char,
            *strings.offset(i as isize),
        );
        i += 1;
        i;
    }
    free(strings as *mut libc::c_void);
}
