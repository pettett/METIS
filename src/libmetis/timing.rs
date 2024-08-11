use ::libc;
use libc::printf;

use super::structure::*;
#[no_mangle]
pub unsafe extern "C" fn libmetis__InitTimers(mut ctrl: *mut ctrl_t) {
    (*ctrl).TotalTmr = 0.0f64;
    (*ctrl).InitPartTmr = 0.0f64;
    (*ctrl).MatchTmr = 0.0f64;
    (*ctrl).ContractTmr = 0.0f64;
    (*ctrl).CoarsenTmr = 0.0f64;
    (*ctrl).UncoarsenTmr = 0.0f64;
    (*ctrl).RefTmr = 0.0f64;
    (*ctrl).ProjectTmr = 0.0f64;
    (*ctrl).SplitTmr = 0.0f64;
    (*ctrl).Aux1Tmr = 0.0f64;
    (*ctrl).Aux2Tmr = 0.0f64;
    (*ctrl).Aux3Tmr = 0.0f64;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__PrintTimers(mut ctrl: *mut ctrl_t) {
    printf(
        b"\nTiming Information -------------------------------------------------\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"\n Multilevel: \t\t %7.3f\0" as *const u8 as *const libc::c_char,
        (*ctrl).TotalTmr,
    );
    printf(
        b"\n     Coarsening: \t\t %7.3f\0" as *const u8 as *const libc::c_char,
        (*ctrl).CoarsenTmr,
    );
    printf(
        b"\n            Matching: \t\t\t %7.3f\0" as *const u8 as *const libc::c_char,
        (*ctrl).MatchTmr,
    );
    printf(
        b"\n            Contract: \t\t\t %7.3f\0" as *const u8 as *const libc::c_char,
        (*ctrl).ContractTmr,
    );
    printf(
        b"\n     Initial Partition: \t %7.3f\0" as *const u8 as *const libc::c_char,
        (*ctrl).InitPartTmr,
    );
    printf(
        b"\n     Uncoarsening: \t\t %7.3f\0" as *const u8 as *const libc::c_char,
        (*ctrl).UncoarsenTmr,
    );
    printf(
        b"\n          Refinement: \t\t\t %7.3f\0" as *const u8 as *const libc::c_char,
        (*ctrl).RefTmr,
    );
    printf(
        b"\n          Projection: \t\t\t %7.3f\0" as *const u8 as *const libc::c_char,
        (*ctrl).ProjectTmr,
    );
    printf(
        b"\n     Splitting: \t\t %7.3f\0" as *const u8 as *const libc::c_char,
        (*ctrl).SplitTmr,
    );
    printf(
        b"\n********************************************************************\n\0" as *const u8
            as *const libc::c_char,
    );
}
