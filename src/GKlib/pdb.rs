// use ::libc;
// use libc::strcmp;

// use super::memory::{gk_free, gk_malloc};
// extern "C" {
//     pub type _IO_wide_data;
//     pub type _IO_codecvt;
//     pub type _IO_marker;
// }

// pub type __int32_t = libc::c_int;
// pub type __off_t = i64;
// pub type __off64_t = i64;

// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct _IO_FILE {
//     pub _flags: libc::c_int,
//     pub _IO_read_ptr: *mut libc::c_char,
//     pub _IO_read_end: *mut libc::c_char,
//     pub _IO_read_base: *mut libc::c_char,
//     pub _IO_write_base: *mut libc::c_char,
//     pub _IO_write_ptr: *mut libc::c_char,
//     pub _IO_write_end: *mut libc::c_char,
//     pub _IO_buf_base: *mut libc::c_char,
//     pub _IO_buf_end: *mut libc::c_char,
//     pub _IO_save_base: *mut libc::c_char,
//     pub _IO_backup_base: *mut libc::c_char,
//     pub _IO_save_end: *mut libc::c_char,
//     pub _markers: *mut _IO_marker,
//     pub _chain: *mut _IO_FILE,
//     pub _fileno: libc::c_int,
//     pub _flags2: libc::c_int,
//     pub _old_offset: __off_t,
//     pub _cur_column: libc::c_ushort,
//     pub _vtable_offset: libc::c_schar,
//     pub _shortbuf: [libc::c_char; 1],
//     pub _lock: *mut libc::c_void,
//     pub _offset: __off64_t,
//     pub _codecvt: *mut _IO_codecvt,
//     pub _wide_data: *mut _IO_wide_data,
//     pub _freeres_list: *mut _IO_FILE,
//     pub _freeres_buf: *mut libc::c_void,
//     pub __pad5: size_t,
//     pub _mode: libc::c_int,
//     pub _unused2: [libc::c_char; 20],
// }
// pub type _IO_lock_t = ();
// pub type FILE = _IO_FILE;
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct atom {
//     pub serial: libc::c_int,
//     pub name: *mut libc::c_char,
//     pub altLoc: libc::c_char,
//     pub resname: *mut libc::c_char,
//     pub chainid: libc::c_char,
//     pub rserial: libc::c_int,
//     pub icode: libc::c_char,
//     pub element: libc::c_char,
//     pub x: libc::c_double,
//     pub y: libc::c_double,
//     pub z: libc::c_double,
//     pub opcy: libc::c_double,
//     pub tmpt: libc::c_double,
// }
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct center_of_mass {
//     pub name: libc::c_char,
//     pub x: libc::c_double,
//     pub y: libc::c_double,
//     pub z: libc::c_double,
// }
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct pdbf {
//     pub natoms: libc::c_int,
//     pub nresidues: libc::c_int,
//     pub ncas: libc::c_int,
//     pub nbbs: libc::c_int,
//     pub corruption: libc::c_int,
//     pub resSeq: *mut libc::c_char,
//     pub threeresSeq: *mut *mut libc::c_char,
//     pub atoms: *mut atom,
//     pub bbs: *mut *mut atom,
//     pub cas: *mut *mut atom,
//     pub cm: *mut center_of_mass,
// }
// #[inline]
// unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
//     return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
//         *(*__ctype_toupper_loc()).offset(__c as isize)
//     } else {
//         __c
//     };
// }
// #[no_mangle]
// pub unsafe extern "C" fn gk_threetoone(mut res: *mut libc::c_char) -> libc::c_char {
//     *res.offset(0 as libc::c_int as isize) = ({
//         let mut __res: libc::c_int = 0;
//         if ::core::mem::size_of::<libc::c_char>() as u64 > 1 as libc::c_int as u64 {
//             if 0 != 0 {
//                 let mut __c: libc::c_int = *res.offset(0 as libc::c_int as isize) as libc::c_int;
//                 __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
//                     __c
//                 } else {
//                     *(*__ctype_toupper_loc()).offset(__c as isize)
//                 };
//             } else {
//                 __res = toupper(*res.offset(0 as libc::c_int as isize) as libc::c_int);
//             }
//         } else {
//             __res = *(*__ctype_toupper_loc())
//                 .offset(*res.offset(0 as libc::c_int as isize) as libc::c_int as isize)
//                 as char;
//         }
//         __res
//     }) as libc::c_char;
//     *res.offset(1 as libc::c_int as isize) = ({
//         let mut __res: libc::c_int = 0;
//         if ::core::mem::size_of::<libc::c_char>() as u64 > 1 as libc::c_int as u64 {
//             if 0 != 0 {
//                 let mut __c: libc::c_int = *res.offset(1 as libc::c_int as isize) as libc::c_int;
//                 __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
//                     __c
//                 } else {
//                     *(*__ctype_toupper_loc()).offset(__c as isize)
//                 };
//             } else {
//                 __res = toupper(*res.offset(1 as libc::c_int as isize) as libc::c_int);
//             }
//         } else {
//             __res = *(*__ctype_toupper_loc())
//                 .offset(*res.offset(1 as libc::c_int as isize) as libc::c_int as isize);
//         }
//         __res
//     }) as libc::c_char;
//     *res.offset(2 as libc::c_int as isize) = ({
//         let mut __res: libc::c_int = 0;
//         if ::core::mem::size_of::<libc::c_char>() as u64 > 1 as libc::c_int as u64 {
//             if 0 != 0 {
//                 let mut __c: libc::c_int = *res.offset(2 as libc::c_int as isize) as libc::c_int;
//                 __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
//                     __c
//                 } else {
//                     *(*__ctype_toupper_loc()).offset(__c as isize)
//                 };
//             } else {
//                 __res = toupper(*res.offset(2 as libc::c_int as isize) as libc::c_int);
//             }
//         } else {
//             __res = *(*__ctype_toupper_loc())
//                 .offset(*res.offset(2 as libc::c_int as isize) as libc::c_int as isize);
//         }
//         __res
//     }) as libc::c_char;
//     if strcmp(res, b"ALA\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
//         return 'A' as i32 as libc::c_char;
//     } else if strcmp(res, b"CYS\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
//         return 'C' as i32 as libc::c_char;
//     } else if strcmp(res, b"ASP\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
//         return 'D' as i32 as libc::c_char;
//     } else if strcmp(res, b"GLU\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
//         return 'E' as i32 as libc::c_char;
//     } else if strcmp(res, b"PHE\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
//         return 'F' as i32 as libc::c_char;
//     } else if strcmp(res, b"GLY\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
//         return 'G' as i32 as libc::c_char;
//     } else if strcmp(res, b"HIS\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
//         return 'H' as i32 as libc::c_char;
//     } else if strcmp(res, b"ILE\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
//         return 'I' as i32 as libc::c_char;
//     } else if strcmp(res, b"LYS\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
//         return 'K' as i32 as libc::c_char;
//     } else if strcmp(res, b"LEU\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
//         return 'L' as i32 as libc::c_char;
//     } else if strcmp(res, b"MET\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
//         return 'M' as i32 as libc::c_char;
//     } else if strcmp(res, b"ASN\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
//         return 'N' as i32 as libc::c_char;
//     } else if strcmp(res, b"PRO\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
//         return 'P' as i32 as libc::c_char;
//     } else if strcmp(res, b"GLN\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
//         return 'Q' as i32 as libc::c_char;
//     } else if strcmp(res, b"ARG\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
//         return 'R' as i32 as libc::c_char;
//     } else if strcmp(res, b"SER\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
//         return 'S' as i32 as libc::c_char;
//     } else if strcmp(res, b"THR\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
//         return 'T' as i32 as libc::c_char;
//     } else if strcmp(res, b"SCY\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
//         return 'U' as i32 as libc::c_char;
//     } else if strcmp(res, b"VAL\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
//         return 'V' as i32 as libc::c_char;
//     } else if strcmp(res, b"TRP\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
//         return 'W' as i32 as libc::c_char;
//     } else if strcmp(res, b"TYR\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
//         return 'Y' as i32 as libc::c_char;
//     } else {
//         return 'X' as i32 as libc::c_char;
//     };
// }
// #[no_mangle]
// pub unsafe extern "C" fn gk_freepdbf(mut p: *mut pdbf) {
//     let mut i: libc::c_int = 0;
//     if !p.is_null() {
//         gk_free(
//             &mut (*p).resSeq as *mut *mut libc::c_char as *mut *mut libc::c_void,
//             0 as *mut *mut libc::c_void,
//         );
//         i = 0 as libc::c_int;
//         while i < (*p).natoms {
//             gk_free(
//                 &mut (*((*p).atoms).offset(i as isize)).name as *mut *mut libc::c_char
//                     as *mut *mut libc::c_void,
//                 &mut (*((*p).atoms).offset(i as isize)).resname as *mut *mut libc::c_char,
//                 0 as *mut *mut libc::c_void,
//             );
//             i += 1;
//             i;
//         }
//         i = 0 as libc::c_int;
//         while i < (*p).nresidues {
//             gk_free(
//                 &mut *((*p).threeresSeq).offset(i as isize) as *mut *mut libc::c_char
//                     as *mut libc::c_void as *mut *mut libc::c_void,
//                 0 as *mut *mut libc::c_void,
//             );
//             i += 1;
//             i;
//         }
//         gk_free(
//             &mut (*p).bbs as *mut *mut *mut atom as *mut *mut libc::c_void,
//             &mut (*p).cas as *mut *mut *mut atom,
//             &mut (*p).atoms as *mut *mut atom,
//             &mut (*p).cm as *mut *mut center_of_mass,
//             &mut (*p).threeresSeq as *mut *mut *mut libc::c_char,
//             0 as *mut *mut libc::c_void,
//         );
//     }
//     gk_free(
//         &mut p as *mut *mut pdbf as *mut *mut libc::c_void,
//         0 as *mut *mut libc::c_void,
//     );
// }
// #[no_mangle]
// pub unsafe extern "C" fn gk_readpdbfile(mut fname: *mut libc::c_char) -> *mut pdbf {
//     let mut i: libc::c_int = 0 as libc::c_int;
//     let mut res: libc::c_int = 0 as libc::c_int;
//     let mut linetype: [libc::c_char; 6] = [0; 6];
//     let mut aserial: libc::c_int = 0;
//     let mut aname: [libc::c_char; 5] =
//         *::core::mem::transmute::<&[u8; 5], &mut [libc::c_char; 5]>(b"    \0");
//     let mut altLoc: libc::c_char = ' ' as i32 as libc::c_char;
//     let mut rname: [libc::c_char; 4] =
//         *::core::mem::transmute::<&[u8; 4], &mut [libc::c_char; 4]>(b"   \0");
//     let mut chainid: libc::c_char = ' ' as i32 as libc::c_char;
//     let mut oldchainid: libc::c_char = ' ' as i32 as libc::c_char;
//     let mut rserial: libc::c_int = 0;
//     let mut oldRserial: libc::c_int = -(37 as libc::c_int);
//     let mut icode: libc::c_char = ' ' as i32 as libc::c_char;
//     let mut element: libc::c_char = ' ' as i32 as libc::c_char;
//     let mut x: libc::c_double = 0.;
//     let mut y: libc::c_double = 0.;
//     let mut z: libc::c_double = 0.;
//     let mut avgx: libc::c_double = 0.;
//     let mut avgy: libc::c_double = 0.;
//     let mut avgz: libc::c_double = 0.;
//     let mut opcy: libc::c_double = 0.;
//     let mut tmpt: libc::c_double = 0.;
//     let mut line: [libc::c_char; 300000] = [0; 300000];
//     let mut corruption: libc::c_int = 0 as libc::c_int;
//     let mut nresatoms: libc::c_int = 0;
//     let mut atoms: libc::c_int = 0 as libc::c_int;
//     let mut residues: libc::c_int = 0 as libc::c_int;
//     let mut cas: libc::c_int = 0 as libc::c_int;
//     let mut bbs: libc::c_int = 0 as libc::c_int;
//     let mut firstres: libc::c_int = 1 as libc::c_int;
//     let mut toFill: *mut pdbf = gk_malloc(
//         ::core::mem::size_of::<pdbf>() as u64,
//         b"fillme\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
//     ) as *mut pdbf;
//     let mut FPIN: *mut FILE = 0 as *mut FILE;
//     FPIN = gk_fopen(
//         fname,
//         b"r\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
//         fname,
//     );
//     while !(fgets(line.as_mut_ptr(), 256 as libc::c_int, FPIN)).is_null() {
//         sscanf(
//             line.as_mut_ptr(),
//             b"%s \0" as *const u8 as *const libc::c_char,
//             linetype.as_mut_ptr(),
//         );
//         if !(strstr(
//             linetype.as_mut_ptr(),
//             b"ATOM\0" as *const u8 as *const libc::c_char,
//         ))
//         .is_null()
//         {
//             sscanf(
//                 line.as_mut_ptr(),
//                 b"%6s%5d%*1c%4c%1c%3c%*1c%1c%4d%1c%*3c%8lf%8lf%8lf%6lf%6lf %c\n\0" as *const u8
//                     as *const libc::c_char,
//                 linetype.as_mut_ptr(),
//                 &mut aserial as *mut libc::c_int,
//                 aname.as_mut_ptr(),
//                 &mut altLoc as *mut libc::c_char,
//                 rname.as_mut_ptr(),
//                 &mut chainid as *mut libc::c_char,
//                 &mut rserial as *mut libc::c_int,
//                 &mut icode as *mut libc::c_char,
//                 &mut x as *mut libc::c_double,
//                 &mut y as *mut libc::c_double,
//                 &mut z as *mut libc::c_double,
//                 &mut opcy as *mut libc::c_double,
//                 &mut tmpt as *mut libc::c_double,
//                 &mut element as *mut libc::c_char,
//             );
//             libc::sscanf(
//                 linetype.as_mut_ptr(),
//                 b" %s \0" as *const u8 as *const libc::c_char,
//                 linetype.as_mut_ptr(),
//             );
//             sscanf(
//                 aname.as_mut_ptr(),
//                 b" %s \0" as *const u8 as *const libc::c_char,
//                 aname.as_mut_ptr(),
//             );
//             sscanf(
//                 rname.as_mut_ptr(),
//                 b" %s \0" as *const u8 as *const libc::c_char,
//                 rname.as_mut_ptr(),
//             );
//             if altLoc as libc::c_int != ' ' as i32 {
//                 corruption = corruption | 1 as libc::c_int;
//             }
//             if firstres == 1 as libc::c_int {
//                 oldRserial = rserial;
//                 oldchainid = chainid;
//                 residues += 1;
//                 residues;
//                 firstres = 0 as libc::c_int;
//             }
//             if oldRserial != rserial {
//                 residues += 1;
//                 residues;
//                 oldRserial = rserial;
//             }
//             if oldchainid as libc::c_int != chainid as libc::c_int {
//                 corruption = corruption | 8 as libc::c_int;
//             }
//             oldchainid = chainid;
//             atoms += 1;
//             atoms;
//             if strcmp(
//                 aname.as_mut_ptr(),
//                 b"CA\0" as *const u8 as *const libc::c_char,
//             ) == 0 as libc::c_int
//             {
//                 cas += 1;
//                 cas;
//             }
//             if strcmp(
//                 aname.as_mut_ptr(),
//                 b"N\0" as *const u8 as *const libc::c_char,
//             ) == 0 as libc::c_int
//                 || strcmp(
//                     aname.as_mut_ptr(),
//                     b"CA\0" as *const u8 as *const libc::c_char,
//                 ) == 0 as libc::c_int
//                 || strcmp(
//                     aname.as_mut_ptr(),
//                     b"C\0" as *const u8 as *const libc::c_char,
//                 ) == 0 as libc::c_int
//                 || strcmp(
//                     aname.as_mut_ptr(),
//                     b"O\0" as *const u8 as *const libc::c_char,
//                 ) == 0 as libc::c_int
//             {
//                 bbs += 1;
//                 bbs;
//             }
//         } else if !(strstr(
//             linetype.as_mut_ptr(),
//             b"ENDMDL\0" as *const u8 as *const libc::c_char,
//         ))
//         .is_null()
//             || !(strstr(
//                 linetype.as_mut_ptr(),
//                 b"END\0" as *const u8 as *const libc::c_char,
//             ))
//             .is_null()
//             || !(strstr(
//                 linetype.as_mut_ptr(),
//                 b"TER\0" as *const u8 as *const libc::c_char,
//             ))
//             .is_null()
//         {
//             break;
//         }
//     }
//     fclose(FPIN);
//     (*toFill).natoms = atoms;
//     (*toFill).ncas = cas;
//     (*toFill).nbbs = bbs;
//     (*toFill).nresidues = residues;
//     (*toFill).resSeq = gk_malloc(
//         (residues as u64).wrapping_mul(::core::mem::size_of::<libc::c_char>() as u64),
//         b"residue seq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
//     ) as *mut libc::c_char;
//     (*toFill).threeresSeq = gk_malloc(
//         (residues as u64).wrapping_mul(::core::mem::size_of::<*mut libc::c_char>() as u64),
//         b"residue seq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
//     ) as *mut *mut libc::c_char;
//     (*toFill).atoms = gk_malloc(
//         (atoms as u64).wrapping_mul(::core::mem::size_of::<atom>() as u64),
//         b"atoms\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
//     ) as *mut atom;
//     (*toFill).bbs = gk_malloc(
//         (bbs as u64).wrapping_mul(::core::mem::size_of::<*mut atom>() as u64),
//         b"bbs\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
//     ) as *mut *mut atom;
//     (*toFill).cas = gk_malloc(
//         (cas as u64).wrapping_mul(::core::mem::size_of::<*mut atom>() as u64),
//         b"cas\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
//     ) as *mut *mut atom;
//     (*toFill).cm = gk_malloc(
//         (residues as u64).wrapping_mul(::core::mem::size_of::<center_of_mass>() as u64),
//         b"center of mass\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
//     ) as *mut center_of_mass;
//     res = 0 as libc::c_int;
//     firstres = 1 as libc::c_int;
//     cas = 0 as libc::c_int;
//     bbs = 0 as libc::c_int;
//     i = 0 as libc::c_int;
//     avgx = 0.0f64;
//     avgy = 0.0f64;
//     avgz = 0.0f64;
//     nresatoms = 0 as libc::c_int;
//     FPIN = gk_fopen(
//         fname,
//         b"r\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
//         fname,
//     );
//     while !(fgets(line.as_mut_ptr(), 256 as libc::c_int, FPIN)).is_null() {
//         sscanf(
//             line.as_mut_ptr(),
//             b"%s \0" as *const u8 as *const libc::c_char,
//             linetype.as_mut_ptr(),
//         );
//         if !(strstr(
//             linetype.as_mut_ptr(),
//             b"ATOM\0" as *const u8 as *const libc::c_char,
//         ))
//         .is_null()
//         {
//             sscanf(
//                 line.as_mut_ptr(),
//                 b"%6s%5d%*1c%4c%1c%3c%*1c%1c%4d%1c%*3c%8lf%8lf%8lf%6lf%6lf %c\n\0" as *const u8
//                     as *const libc::c_char,
//                 linetype.as_mut_ptr(),
//                 &mut aserial as *mut libc::c_int,
//                 aname.as_mut_ptr(),
//                 &mut altLoc as *mut libc::c_char,
//                 rname.as_mut_ptr(),
//                 &mut chainid as *mut libc::c_char,
//                 &mut rserial as *mut libc::c_int,
//                 &mut icode as *mut libc::c_char,
//                 &mut x as *mut libc::c_double,
//                 &mut y as *mut libc::c_double,
//                 &mut z as *mut libc::c_double,
//                 &mut opcy as *mut libc::c_double,
//                 &mut tmpt as *mut libc::c_double,
//                 &mut element as *mut libc::c_char,
//             );
//             sscanf(
//                 aname.as_mut_ptr(),
//                 b"%s\0" as *const u8 as *const libc::c_char,
//                 aname.as_mut_ptr(),
//             );
//             sscanf(
//                 rname.as_mut_ptr(),
//                 b"%s\0" as *const u8 as *const libc::c_char,
//                 rname.as_mut_ptr(),
//             );
//             if firstres == 1 as libc::c_int {
//                 *((*toFill).resSeq).offset(res as isize) = gk_threetoone(rname.as_mut_ptr());
//                 let ref mut fresh0 = *((*toFill).threeresSeq).offset(res as isize);
//                 *fresh0 = gk_strdup(rname.as_mut_ptr());
//                 oldRserial = rserial;
//                 res += 1;
//                 res;
//                 firstres = 0 as libc::c_int;
//             }
//             if oldRserial != rserial {
//                 (*((*toFill).cm).offset((res - 1 as libc::c_int) as isize)).x =
//                     avgx / nresatoms as libc::c_double;
//                 (*((*toFill).cm).offset((res - 1 as libc::c_int) as isize)).y =
//                     avgy / nresatoms as libc::c_double;
//                 (*((*toFill).cm).offset((res - 1 as libc::c_int) as isize)).z =
//                     avgz / nresatoms as libc::c_double;
//                 avgx = 0.0f64;
//                 avgy = 0.0f64;
//                 avgz = 0.0f64;
//                 nresatoms = 0 as libc::c_int;
//                 (*((*toFill).cm).offset((res - 1 as libc::c_int) as isize)).name =
//                     *((*toFill).resSeq).offset((res - 1 as libc::c_int) as isize);
//                 let ref mut fresh1 = *((*toFill).threeresSeq).offset(res as isize);
//                 *fresh1 = gk_strdup(rname.as_mut_ptr());
//                 *((*toFill).resSeq).offset(res as isize) = gk_threetoone(rname.as_mut_ptr());
//                 res += 1;
//                 res;
//                 oldRserial = rserial;
//             }
//             avgx += x;
//             avgy += y;
//             avgz += z;
//             nresatoms += 1;
//             nresatoms;
//             (*((*toFill).atoms).offset(i as isize)).x = x;
//             (*((*toFill).atoms).offset(i as isize)).y = y;
//             (*((*toFill).atoms).offset(i as isize)).z = z;
//             (*((*toFill).atoms).offset(i as isize)).opcy = opcy;
//             (*((*toFill).atoms).offset(i as isize)).tmpt = tmpt;
//             (*((*toFill).atoms).offset(i as isize)).element = element;
//             (*((*toFill).atoms).offset(i as isize)).serial = aserial;
//             (*((*toFill).atoms).offset(i as isize)).chainid = chainid;
//             (*((*toFill).atoms).offset(i as isize)).altLoc = altLoc;
//             (*((*toFill).atoms).offset(i as isize)).rserial = rserial;
//             (*((*toFill).atoms).offset(i as isize)).icode = icode;
//             let ref mut fresh2 = (*((*toFill).atoms).offset(i as isize)).name;
//             *fresh2 = gk_strdup(aname.as_mut_ptr());
//             let ref mut fresh3 = (*((*toFill).atoms).offset(i as isize)).resname;
//             *fresh3 = gk_strdup(rname.as_mut_ptr());
//             if strcmp(
//                 aname.as_mut_ptr(),
//                 b"CA\0" as *const u8 as *const libc::c_char,
//             ) == 0 as libc::c_int
//             {
//                 let ref mut fresh4 = *((*toFill).cas).offset(cas as isize);
//                 *fresh4 = &mut *((*toFill).atoms).offset(i as isize) as *mut atom;
//                 cas += 1;
//                 cas;
//             }
//             if strcmp(
//                 aname.as_mut_ptr(),
//                 b"N\0" as *const u8 as *const libc::c_char,
//             ) == 0 as libc::c_int
//                 || strcmp(
//                     aname.as_mut_ptr(),
//                     b"CA\0" as *const u8 as *const libc::c_char,
//                 ) == 0 as libc::c_int
//                 || strcmp(
//                     aname.as_mut_ptr(),
//                     b"C\0" as *const u8 as *const libc::c_char,
//                 ) == 0 as libc::c_int
//                 || strcmp(
//                     aname.as_mut_ptr(),
//                     b"O\0" as *const u8 as *const libc::c_char,
//                 ) == 0 as libc::c_int
//             {
//                 let ref mut fresh5 = *((*toFill).bbs).offset(bbs as isize);
//                 *fresh5 = &mut *((*toFill).atoms).offset(i as isize) as *mut atom;
//                 bbs += 1;
//                 bbs;
//             }
//             i += 1;
//             i;
//         } else if !(strstr(
//             linetype.as_mut_ptr(),
//             b"ENDMDL\0" as *const u8 as *const libc::c_char,
//         ))
//         .is_null()
//             || !(strstr(
//                 linetype.as_mut_ptr(),
//                 b"END\0" as *const u8 as *const libc::c_char,
//             ))
//             .is_null()
//             || !(strstr(
//                 linetype.as_mut_ptr(),
//                 b"TER\0" as *const u8 as *const libc::c_char,
//             ))
//             .is_null()
//         {
//             break;
//         }
//     }
//     (*((*toFill).cm).offset((res - 1 as libc::c_int) as isize)).x =
//         avgx / nresatoms as libc::c_double;
//     (*((*toFill).cm).offset((res - 1 as libc::c_int) as isize)).y =
//         avgy / nresatoms as libc::c_double;
//     (*((*toFill).cm).offset((res - 1 as libc::c_int) as isize)).z =
//         avgz / nresatoms as libc::c_double;
//     if cas != residues {
//         printf(
//             b"Number of residues and CA coordinates differs by %d (!)\n\0" as *const u8
//                 as *const libc::c_char,
//             residues - cas,
//         );
//         if cas < residues {
//             corruption = corruption | 2 as libc::c_int;
//         } else if cas > residues {
//             corruption = corruption | 16 as libc::c_int;
//         }
//     }
//     if bbs < residues * 4 as libc::c_int {
//         corruption = corruption | 4 as libc::c_int;
//     } else if bbs > residues * 4 as libc::c_int {
//         corruption = corruption | 32 as libc::c_int;
//     }
//     fclose(FPIN);
//     (*toFill).corruption = corruption;
//     return toFill;
// }
// #[no_mangle]
// pub unsafe extern "C" fn gk_writefastafrompdb(mut pb: *mut pdbf, mut fname: *mut libc::c_char) {
//     let mut i: libc::c_int = 0;
//     let mut FPOUT: *mut FILE = 0 as *mut FILE;
//     FPOUT = gk_fopen(
//         fname,
//         b"w\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
//         fname,
//     );
//     fprintf(
//         FPOUT,
//         b"> %s\n\0" as *const u8 as *const libc::c_char,
//         fname,
//     );
//     i = 0 as libc::c_int;
//     while i < (*pb).nresidues {
//         fprintf(
//             FPOUT,
//             b"%c\0" as *const u8 as *const libc::c_char,
//             *((*pb).resSeq).offset(i as isize) as libc::c_int,
//         );
//         i += 1;
//         i;
//     }
//     fprintf(FPOUT, b"\n\0" as *const u8 as *const libc::c_char);
//     fclose(FPOUT);
// }
// #[no_mangle]
// pub unsafe extern "C" fn gk_writecentersofmass(mut p: *mut pdbf, mut fname: *mut libc::c_char) {
//     let mut i: libc::c_int = 0;
//     let mut FPIN: *mut FILE = 0 as *mut FILE;
//     FPIN = gk_fopen(
//         fname,
//         b"w\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
//         fname,
//     );
//     i = 0 as libc::c_int;
//     while i < (*p).nresidues {
//         fprintf(
//             FPIN,
//             b"%-6s%5d %4s%1c%3s %1c%4d%1c   %8.3lf%8.3lf%8.3lf%6.2f%6.2f\n\0" as *const u8
//                 as *const libc::c_char,
//             b"ATOM  \0" as *const u8 as *const libc::c_char,
//             i,
//             b"CA\0" as *const u8 as *const libc::c_char,
//             ' ' as i32,
//             *((*p).threeresSeq).offset(i as isize),
//             ' ' as i32,
//             i,
//             ' ' as i32,
//             (*((*p).cm).offset(i as isize)).x,
//             (*((*p).cm).offset(i as isize)).y,
//             (*((*p).cm).offset(i as isize)).z,
//             1.0f64,
//             -37.0f64,
//         );
//         i += 1;
//         i;
//     }
//     fclose(FPIN);
// }
// #[no_mangle]
// pub unsafe extern "C" fn gk_writefullatom(mut p: *mut pdbf, mut fname: *mut libc::c_char) {
//     let mut i: libc::c_int = 0;
//     let mut FPIN: *mut FILE = 0 as *mut FILE;
//     FPIN = gk_fopen(
//         fname,
//         b"w\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
//         fname,
//     );
//     i = 0 as libc::c_int;
//     while i < (*p).natoms {
//         fprintf(
//             FPIN,
//             b"%-6s%5d %4s%1c%3s %1c%4d%1c   %8.3lf%8.3lf%8.3lf%6.2f%6.2f\n\0" as *const u8
//                 as *const libc::c_char,
//             b"ATOM  \0" as *const u8 as *const libc::c_char,
//             (*((*p).atoms).offset(i as isize)).serial,
//             (*((*p).atoms).offset(i as isize)).name,
//             (*((*p).atoms).offset(i as isize)).altLoc as libc::c_int,
//             (*((*p).atoms).offset(i as isize)).resname,
//             (*((*p).atoms).offset(i as isize)).chainid as libc::c_int,
//             (*((*p).atoms).offset(i as isize)).rserial,
//             (*((*p).atoms).offset(i as isize)).icode as libc::c_int,
//             (*((*p).atoms).offset(i as isize)).x,
//             (*((*p).atoms).offset(i as isize)).y,
//             (*((*p).atoms).offset(i as isize)).z,
//             (*((*p).atoms).offset(i as isize)).opcy,
//             (*((*p).atoms).offset(i as isize)).tmpt,
//         );
//         i += 1;
//         i;
//     }
//     fclose(FPIN);
// }
// #[no_mangle]
// pub unsafe extern "C" fn gk_writebackbone(mut p: *mut pdbf, mut fname: *mut libc::c_char) {
//     let mut i: libc::c_int = 0;
//     let mut FPIN: *mut FILE = 0 as *mut FILE;
//     FPIN = gk_fopen(
//         fname,
//         b"w\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
//         fname,
//     );
//     i = 0 as libc::c_int;
//     while i < (*p).nbbs {
//         fprintf(
//             FPIN,
//             b"%-6s%5d %4s%1c%3s %1c%4d%1c   %8.3lf%8.3lf%8.3lf%6.2f%6.2f\n\0" as *const u8
//                 as *const libc::c_char,
//             b"ATOM  \0" as *const u8 as *const libc::c_char,
//             (**((*p).bbs).offset(i as isize)).serial,
//             (**((*p).bbs).offset(i as isize)).name,
//             (**((*p).bbs).offset(i as isize)).altLoc as libc::c_int,
//             (**((*p).bbs).offset(i as isize)).resname,
//             (**((*p).bbs).offset(i as isize)).chainid as libc::c_int,
//             (**((*p).bbs).offset(i as isize)).rserial,
//             (**((*p).bbs).offset(i as isize)).icode as libc::c_int,
//             (**((*p).bbs).offset(i as isize)).x,
//             (**((*p).bbs).offset(i as isize)).y,
//             (**((*p).bbs).offset(i as isize)).z,
//             (**((*p).bbs).offset(i as isize)).opcy,
//             (**((*p).bbs).offset(i as isize)).tmpt,
//         );
//         i += 1;
//         i;
//     }
//     fclose(FPIN);
// }
// #[no_mangle]
// pub unsafe extern "C" fn gk_writealphacarbons(mut p: *mut pdbf, mut fname: *mut libc::c_char) {
//     let mut i: libc::c_int = 0;
//     let mut FPIN: *mut FILE = 0 as *mut FILE;
//     FPIN = gk_fopen(
//         fname,
//         b"w\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
//         fname,
//     );
//     i = 0 as libc::c_int;
//     while i < (*p).ncas {
//         fprintf(
//             FPIN,
//             b"%-6s%5d %4s%1c%3s %1c%4d%1c   %8.3lf%8.3lf%8.3lf%6.2f%6.2f\n\0" as *const u8
//                 as *const libc::c_char,
//             b"ATOM  \0" as *const u8 as *const libc::c_char,
//             (**((*p).cas).offset(i as isize)).serial,
//             (**((*p).cas).offset(i as isize)).name,
//             (**((*p).cas).offset(i as isize)).altLoc as libc::c_int,
//             (**((*p).cas).offset(i as isize)).resname,
//             (**((*p).cas).offset(i as isize)).chainid as libc::c_int,
//             (**((*p).cas).offset(i as isize)).rserial,
//             (**((*p).cas).offset(i as isize)).icode as libc::c_int,
//             (**((*p).cas).offset(i as isize)).x,
//             (**((*p).cas).offset(i as isize)).y,
//             (**((*p).cas).offset(i as isize)).z,
//             (**((*p).cas).offset(i as isize)).opcy,
//             (**((*p).cas).offset(i as isize)).tmpt,
//         );
//         i += 1;
//         i;
//     }
//     fclose(FPIN);
// }
// #[no_mangle]
// pub unsafe extern "C" fn gk_showcorruption(mut p: *mut pdbf) {
//     let mut corruption: libc::c_int = (*p).corruption;
//     if corruption & 1 as libc::c_int != 0 {
//         printf(
//             b"Multiple coordinate sets for at least one atom\n\0" as *const u8
//                 as *const libc::c_char,
//         );
//     }
//     if corruption & 2 as libc::c_int != 0 {
//         printf(
//             b"Missing coordiantes for at least one CA atom\n\0" as *const u8 as *const libc::c_char,
//         );
//     }
//     if corruption & 4 as libc::c_int != 0 {
//         printf(
//             b"Missing coordiantes for at least one backbone atom (N,CA,C,O)\n\0" as *const u8
//                 as *const libc::c_char,
//         );
//     }
//     if corruption & 8 as libc::c_int != 0 {
//         printf(
//             b"File contains coordinates for multiple chains\n\0" as *const u8
//                 as *const libc::c_char,
//         );
//     }
//     if corruption & 16 as libc::c_int != 0 {
//         printf(
//             b"Multiple CA atoms found for the same residue (could be alternate locators)\n\0"
//                 as *const u8 as *const libc::c_char,
//         );
//     }
//     if corruption & 16 as libc::c_int != 0 {
//         printf(
//             b"Multiple copies of backbone atoms found for the same residue (could be alternate locators)\n\0"
//                 as *const u8 as *const libc::c_char,
//         );
//     }
// }
