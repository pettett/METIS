use ::libc;
extern "C" {
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
    fn METIS_PartMeshNodal(
        ne: *mut idx_t,
        nn: *mut idx_t,
        eptr: *mut idx_t,
        eind: *mut idx_t,
        vwgt: *mut idx_t,
        vsize: *mut idx_t,
        nparts: *mut idx_t,
        tpwgts: *mut real_t,
        options: *mut idx_t,
        objval: *mut idx_t,
        epart: *mut idx_t,
        npart: *mut idx_t,
    ) -> libc::c_int;
    fn METIS_PartMeshDual(
        ne: *mut idx_t,
        nn: *mut idx_t,
        eptr: *mut idx_t,
        eind: *mut idx_t,
        vwgt: *mut idx_t,
        vsize: *mut idx_t,
        ncommon: *mut idx_t,
        nparts: *mut idx_t,
        tpwgts: *mut real_t,
        options: *mut idx_t,
        objval: *mut idx_t,
        epart: *mut idx_t,
        npart: *mut idx_t,
    ) -> libc::c_int;
    fn METIS_NodeND(
        nvtxs: *mut idx_t,
        xadj: *mut idx_t,
        adjncy: *mut idx_t,
        vwgt: *mut idx_t,
        options: *mut idx_t,
        perm: *mut idx_t,
        iperm: *mut idx_t,
    ) -> libc::c_int;
    fn METIS_Free(ptr: *mut libc::c_void) -> libc::c_int;
    fn METIS_SetDefaultOptions(options: *mut idx_t) -> libc::c_int;
}
pub type __int32_t = libc::c_int;
pub type int32_t = __int32_t;
pub type idx_t = int32_t;
pub type real_t = libc::c_float;
#[no_mangle]
pub unsafe extern "C" fn metis_partgraphrecursive(
    mut nvtxs: *mut idx_t,
    mut ncon: *mut idx_t,
    mut xadj: *mut idx_t,
    mut adjncy: *mut idx_t,
    mut vwgt: *mut idx_t,
    mut vsize: *mut idx_t,
    mut adjwgt: *mut idx_t,
    mut nparts: *mut idx_t,
    mut tpwgts: *mut real_t,
    mut ubvec: *mut real_t,
    mut options: *mut idx_t,
    mut edgecut: *mut idx_t,
    mut part: *mut idx_t,
) -> libc::c_int {
    return METIS_PartGraphRecursive(
        nvtxs,
        ncon,
        xadj,
        adjncy,
        vwgt,
        vsize,
        adjwgt,
        nparts,
        tpwgts,
        ubvec,
        options,
        edgecut,
        part,
    );
}
#[no_mangle]
pub unsafe extern "C" fn metis_partgraphrecursive_(
    mut nvtxs: *mut idx_t,
    mut ncon: *mut idx_t,
    mut xadj: *mut idx_t,
    mut adjncy: *mut idx_t,
    mut vwgt: *mut idx_t,
    mut vsize: *mut idx_t,
    mut adjwgt: *mut idx_t,
    mut nparts: *mut idx_t,
    mut tpwgts: *mut real_t,
    mut ubvec: *mut real_t,
    mut options: *mut idx_t,
    mut edgecut: *mut idx_t,
    mut part: *mut idx_t,
) -> libc::c_int {
    return METIS_PartGraphRecursive(
        nvtxs,
        ncon,
        xadj,
        adjncy,
        vwgt,
        vsize,
        adjwgt,
        nparts,
        tpwgts,
        ubvec,
        options,
        edgecut,
        part,
    );
}
#[no_mangle]
pub unsafe extern "C" fn metis_partgraphrecursive__(
    mut nvtxs: *mut idx_t,
    mut ncon: *mut idx_t,
    mut xadj: *mut idx_t,
    mut adjncy: *mut idx_t,
    mut vwgt: *mut idx_t,
    mut vsize: *mut idx_t,
    mut adjwgt: *mut idx_t,
    mut nparts: *mut idx_t,
    mut tpwgts: *mut real_t,
    mut ubvec: *mut real_t,
    mut options: *mut idx_t,
    mut edgecut: *mut idx_t,
    mut part: *mut idx_t,
) -> libc::c_int {
    return METIS_PartGraphRecursive(
        nvtxs,
        ncon,
        xadj,
        adjncy,
        vwgt,
        vsize,
        adjwgt,
        nparts,
        tpwgts,
        ubvec,
        options,
        edgecut,
        part,
    );
}
#[no_mangle]
pub unsafe extern "C" fn METIS_PARTGRAPHRECURSIVE(
    mut nvtxs: *mut idx_t,
    mut ncon: *mut idx_t,
    mut xadj: *mut idx_t,
    mut adjncy: *mut idx_t,
    mut vwgt: *mut idx_t,
    mut vsize: *mut idx_t,
    mut adjwgt: *mut idx_t,
    mut nparts: *mut idx_t,
    mut tpwgts: *mut real_t,
    mut ubvec: *mut real_t,
    mut options: *mut idx_t,
    mut edgecut: *mut idx_t,
    mut part: *mut idx_t,
) -> libc::c_int {
    return METIS_PartGraphRecursive(
        nvtxs,
        ncon,
        xadj,
        adjncy,
        vwgt,
        vsize,
        adjwgt,
        nparts,
        tpwgts,
        ubvec,
        options,
        edgecut,
        part,
    );
}
#[no_mangle]
pub unsafe extern "C" fn metis_partgraphkway__(
    mut nvtxs: *mut idx_t,
    mut ncon: *mut idx_t,
    mut xadj: *mut idx_t,
    mut adjncy: *mut idx_t,
    mut vwgt: *mut idx_t,
    mut vsize: *mut idx_t,
    mut adjwgt: *mut idx_t,
    mut nparts: *mut idx_t,
    mut tpwgts: *mut real_t,
    mut ubvec: *mut real_t,
    mut options: *mut idx_t,
    mut edgecut: *mut idx_t,
    mut part: *mut idx_t,
) -> libc::c_int {
    return METIS_PartGraphKway(
        nvtxs,
        ncon,
        xadj,
        adjncy,
        vwgt,
        vsize,
        adjwgt,
        nparts,
        tpwgts,
        ubvec,
        options,
        edgecut,
        part,
    );
}
#[no_mangle]
pub unsafe extern "C" fn METIS_PARTGRAPHKWAY(
    mut nvtxs: *mut idx_t,
    mut ncon: *mut idx_t,
    mut xadj: *mut idx_t,
    mut adjncy: *mut idx_t,
    mut vwgt: *mut idx_t,
    mut vsize: *mut idx_t,
    mut adjwgt: *mut idx_t,
    mut nparts: *mut idx_t,
    mut tpwgts: *mut real_t,
    mut ubvec: *mut real_t,
    mut options: *mut idx_t,
    mut edgecut: *mut idx_t,
    mut part: *mut idx_t,
) -> libc::c_int {
    return METIS_PartGraphKway(
        nvtxs,
        ncon,
        xadj,
        adjncy,
        vwgt,
        vsize,
        adjwgt,
        nparts,
        tpwgts,
        ubvec,
        options,
        edgecut,
        part,
    );
}
#[no_mangle]
pub unsafe extern "C" fn metis_partgraphkway(
    mut nvtxs: *mut idx_t,
    mut ncon: *mut idx_t,
    mut xadj: *mut idx_t,
    mut adjncy: *mut idx_t,
    mut vwgt: *mut idx_t,
    mut vsize: *mut idx_t,
    mut adjwgt: *mut idx_t,
    mut nparts: *mut idx_t,
    mut tpwgts: *mut real_t,
    mut ubvec: *mut real_t,
    mut options: *mut idx_t,
    mut edgecut: *mut idx_t,
    mut part: *mut idx_t,
) -> libc::c_int {
    return METIS_PartGraphKway(
        nvtxs,
        ncon,
        xadj,
        adjncy,
        vwgt,
        vsize,
        adjwgt,
        nparts,
        tpwgts,
        ubvec,
        options,
        edgecut,
        part,
    );
}
#[no_mangle]
pub unsafe extern "C" fn metis_partgraphkway_(
    mut nvtxs: *mut idx_t,
    mut ncon: *mut idx_t,
    mut xadj: *mut idx_t,
    mut adjncy: *mut idx_t,
    mut vwgt: *mut idx_t,
    mut vsize: *mut idx_t,
    mut adjwgt: *mut idx_t,
    mut nparts: *mut idx_t,
    mut tpwgts: *mut real_t,
    mut ubvec: *mut real_t,
    mut options: *mut idx_t,
    mut edgecut: *mut idx_t,
    mut part: *mut idx_t,
) -> libc::c_int {
    return METIS_PartGraphKway(
        nvtxs,
        ncon,
        xadj,
        adjncy,
        vwgt,
        vsize,
        adjwgt,
        nparts,
        tpwgts,
        ubvec,
        options,
        edgecut,
        part,
    );
}
#[no_mangle]
pub unsafe extern "C" fn METIS_MESHTODUAL(
    mut ne: *mut idx_t,
    mut nn: *mut idx_t,
    mut eptr: *mut idx_t,
    mut eind: *mut idx_t,
    mut ncommon: *mut idx_t,
    mut numflag: *mut idx_t,
    mut r_xadj: *mut *mut idx_t,
    mut r_adjncy: *mut *mut idx_t,
) -> libc::c_int {
    return METIS_MeshToDual(ne, nn, eptr, eind, ncommon, numflag, r_xadj, r_adjncy);
}
#[no_mangle]
pub unsafe extern "C" fn metis_meshtodual(
    mut ne: *mut idx_t,
    mut nn: *mut idx_t,
    mut eptr: *mut idx_t,
    mut eind: *mut idx_t,
    mut ncommon: *mut idx_t,
    mut numflag: *mut idx_t,
    mut r_xadj: *mut *mut idx_t,
    mut r_adjncy: *mut *mut idx_t,
) -> libc::c_int {
    return METIS_MeshToDual(ne, nn, eptr, eind, ncommon, numflag, r_xadj, r_adjncy);
}
#[no_mangle]
pub unsafe extern "C" fn metis_meshtodual_(
    mut ne: *mut idx_t,
    mut nn: *mut idx_t,
    mut eptr: *mut idx_t,
    mut eind: *mut idx_t,
    mut ncommon: *mut idx_t,
    mut numflag: *mut idx_t,
    mut r_xadj: *mut *mut idx_t,
    mut r_adjncy: *mut *mut idx_t,
) -> libc::c_int {
    return METIS_MeshToDual(ne, nn, eptr, eind, ncommon, numflag, r_xadj, r_adjncy);
}
#[no_mangle]
pub unsafe extern "C" fn metis_meshtodual__(
    mut ne: *mut idx_t,
    mut nn: *mut idx_t,
    mut eptr: *mut idx_t,
    mut eind: *mut idx_t,
    mut ncommon: *mut idx_t,
    mut numflag: *mut idx_t,
    mut r_xadj: *mut *mut idx_t,
    mut r_adjncy: *mut *mut idx_t,
) -> libc::c_int {
    return METIS_MeshToDual(ne, nn, eptr, eind, ncommon, numflag, r_xadj, r_adjncy);
}
#[no_mangle]
pub unsafe extern "C" fn METIS_MESHTONODAL(
    mut ne: *mut idx_t,
    mut nn: *mut idx_t,
    mut eptr: *mut idx_t,
    mut eind: *mut idx_t,
    mut numflag: *mut idx_t,
    mut r_xadj: *mut *mut idx_t,
    mut r_adjncy: *mut *mut idx_t,
) -> libc::c_int {
    return METIS_MeshToNodal(ne, nn, eptr, eind, numflag, r_xadj, r_adjncy);
}
#[no_mangle]
pub unsafe extern "C" fn metis_meshtonodal(
    mut ne: *mut idx_t,
    mut nn: *mut idx_t,
    mut eptr: *mut idx_t,
    mut eind: *mut idx_t,
    mut numflag: *mut idx_t,
    mut r_xadj: *mut *mut idx_t,
    mut r_adjncy: *mut *mut idx_t,
) -> libc::c_int {
    return METIS_MeshToNodal(ne, nn, eptr, eind, numflag, r_xadj, r_adjncy);
}
#[no_mangle]
pub unsafe extern "C" fn metis_meshtonodal_(
    mut ne: *mut idx_t,
    mut nn: *mut idx_t,
    mut eptr: *mut idx_t,
    mut eind: *mut idx_t,
    mut numflag: *mut idx_t,
    mut r_xadj: *mut *mut idx_t,
    mut r_adjncy: *mut *mut idx_t,
) -> libc::c_int {
    return METIS_MeshToNodal(ne, nn, eptr, eind, numflag, r_xadj, r_adjncy);
}
#[no_mangle]
pub unsafe extern "C" fn metis_meshtonodal__(
    mut ne: *mut idx_t,
    mut nn: *mut idx_t,
    mut eptr: *mut idx_t,
    mut eind: *mut idx_t,
    mut numflag: *mut idx_t,
    mut r_xadj: *mut *mut idx_t,
    mut r_adjncy: *mut *mut idx_t,
) -> libc::c_int {
    return METIS_MeshToNodal(ne, nn, eptr, eind, numflag, r_xadj, r_adjncy);
}
#[no_mangle]
pub unsafe extern "C" fn metis_partmeshnodal_(
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
    return METIS_PartMeshNodal(
        ne,
        nn,
        eptr,
        eind,
        vwgt,
        vsize,
        nparts,
        tpwgts,
        options,
        objval,
        epart,
        npart,
    );
}
#[no_mangle]
pub unsafe extern "C" fn metis_partmeshnodal__(
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
    return METIS_PartMeshNodal(
        ne,
        nn,
        eptr,
        eind,
        vwgt,
        vsize,
        nparts,
        tpwgts,
        options,
        objval,
        epart,
        npart,
    );
}
#[no_mangle]
pub unsafe extern "C" fn metis_partmeshnodal(
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
    return METIS_PartMeshNodal(
        ne,
        nn,
        eptr,
        eind,
        vwgt,
        vsize,
        nparts,
        tpwgts,
        options,
        objval,
        epart,
        npart,
    );
}
#[no_mangle]
pub unsafe extern "C" fn METIS_PARTMESHNODAL(
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
    return METIS_PartMeshNodal(
        ne,
        nn,
        eptr,
        eind,
        vwgt,
        vsize,
        nparts,
        tpwgts,
        options,
        objval,
        epart,
        npart,
    );
}
#[no_mangle]
pub unsafe extern "C" fn METIS_PARTMESHDUAL(
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
    return METIS_PartMeshDual(
        ne,
        nn,
        eptr,
        eind,
        vwgt,
        vsize,
        ncommon,
        nparts,
        tpwgts,
        options,
        objval,
        epart,
        npart,
    );
}
#[no_mangle]
pub unsafe extern "C" fn metis_partmeshdual(
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
    return METIS_PartMeshDual(
        ne,
        nn,
        eptr,
        eind,
        vwgt,
        vsize,
        ncommon,
        nparts,
        tpwgts,
        options,
        objval,
        epart,
        npart,
    );
}
#[no_mangle]
pub unsafe extern "C" fn metis_partmeshdual_(
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
    return METIS_PartMeshDual(
        ne,
        nn,
        eptr,
        eind,
        vwgt,
        vsize,
        ncommon,
        nparts,
        tpwgts,
        options,
        objval,
        epart,
        npart,
    );
}
#[no_mangle]
pub unsafe extern "C" fn metis_partmeshdual__(
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
    return METIS_PartMeshDual(
        ne,
        nn,
        eptr,
        eind,
        vwgt,
        vsize,
        ncommon,
        nparts,
        tpwgts,
        options,
        objval,
        epart,
        npart,
    );
}
#[no_mangle]
pub unsafe extern "C" fn METIS_NODEND(
    mut nvtxs: *mut idx_t,
    mut xadj: *mut idx_t,
    mut adjncy: *mut idx_t,
    mut vwgt: *mut idx_t,
    mut options: *mut idx_t,
    mut perm: *mut idx_t,
    mut iperm: *mut idx_t,
) -> libc::c_int {
    return METIS_NodeND(nvtxs, xadj, adjncy, vwgt, options, perm, iperm);
}
#[no_mangle]
pub unsafe extern "C" fn metis_nodend(
    mut nvtxs: *mut idx_t,
    mut xadj: *mut idx_t,
    mut adjncy: *mut idx_t,
    mut vwgt: *mut idx_t,
    mut options: *mut idx_t,
    mut perm: *mut idx_t,
    mut iperm: *mut idx_t,
) -> libc::c_int {
    return METIS_NodeND(nvtxs, xadj, adjncy, vwgt, options, perm, iperm);
}
#[no_mangle]
pub unsafe extern "C" fn metis_nodend_(
    mut nvtxs: *mut idx_t,
    mut xadj: *mut idx_t,
    mut adjncy: *mut idx_t,
    mut vwgt: *mut idx_t,
    mut options: *mut idx_t,
    mut perm: *mut idx_t,
    mut iperm: *mut idx_t,
) -> libc::c_int {
    return METIS_NodeND(nvtxs, xadj, adjncy, vwgt, options, perm, iperm);
}
#[no_mangle]
pub unsafe extern "C" fn metis_nodend__(
    mut nvtxs: *mut idx_t,
    mut xadj: *mut idx_t,
    mut adjncy: *mut idx_t,
    mut vwgt: *mut idx_t,
    mut options: *mut idx_t,
    mut perm: *mut idx_t,
    mut iperm: *mut idx_t,
) -> libc::c_int {
    return METIS_NodeND(nvtxs, xadj, adjncy, vwgt, options, perm, iperm);
}
#[no_mangle]
pub unsafe extern "C" fn METIS_FREE(mut ptr: *mut libc::c_void) -> libc::c_int {
    return METIS_Free(ptr);
}
#[no_mangle]
pub unsafe extern "C" fn metis_free(mut ptr: *mut libc::c_void) -> libc::c_int {
    return METIS_Free(ptr);
}
#[no_mangle]
pub unsafe extern "C" fn metis_free_(mut ptr: *mut libc::c_void) -> libc::c_int {
    return METIS_Free(ptr);
}
#[no_mangle]
pub unsafe extern "C" fn metis_free__(mut ptr: *mut libc::c_void) -> libc::c_int {
    return METIS_Free(ptr);
}
#[no_mangle]
pub unsafe extern "C" fn METIS_SETDEFAULTOPTIONS(
    mut options: *mut idx_t,
) -> libc::c_int {
    return METIS_SetDefaultOptions(options);
}
#[no_mangle]
pub unsafe extern "C" fn metis_setdefaultoptions(
    mut options: *mut idx_t,
) -> libc::c_int {
    return METIS_SetDefaultOptions(options);
}
#[no_mangle]
pub unsafe extern "C" fn metis_setdefaultoptions_(
    mut options: *mut idx_t,
) -> libc::c_int {
    return METIS_SetDefaultOptions(options);
}
#[no_mangle]
pub unsafe extern "C" fn metis_setdefaultoptions__(
    mut options: *mut idx_t,
) -> libc::c_int {
    return METIS_SetDefaultOptions(options);
}
