use std::slice;

use ::libc;
pub type __int32_t = libc::c_int;
pub type int32_t = __int32_t;
pub type idx_t = int32_t;
#[no_mangle]
pub unsafe extern "C" fn libmetis__genmmd(
    mut neqns: idx_t,
    mut xadj: &mut [idx_t],
    mut adjncy: *mut idx_t,
    mut invp: *mut idx_t,
    mut perm: *mut idx_t,
    mut delta: idx_t,
    mut head: *mut idx_t,
    mut qsize: *mut idx_t,
    mut list: *mut idx_t,
    mut marker: *mut idx_t,
    mut maxint: idx_t,
    mut ncsub: *mut idx_t,
) {
    let mut ehead: idx_t = 0;
    let mut i: idx_t = 0;
    let mut mdeg: idx_t = 0;
    let mut mdlmt: idx_t = 0;
    let mut mdeg_node: idx_t = 0;
    let mut nextmd: idx_t = 0;
    let mut num: idx_t = 0;
    let mut tag: idx_t = 0;
    if neqns <= 0 as libc::c_int {
        return;
    }

    // Code here is from fortran and indexes from 1
    xadj = slice::from_raw_parts_mut(xadj.as_mut_ptr().offset(-1), xadj.len() + 1);

    adjncy = adjncy.offset(-1);
    adjncy;
    invp = invp.offset(-1);
    invp;
    perm = perm.offset(-1);
    perm;
    head = head.offset(-1);
    head;
    qsize = qsize.offset(-1);
    qsize;
    list = list.offset(-1);
    list;
    marker = marker.offset(-1);
    marker;
    *ncsub = 0 as libc::c_int;
    libmetis__mmdint(neqns, xadj, adjncy, head, invp, perm, qsize, list, marker);
    num = 1;
    nextmd = *head.offset(1 as isize);
    while nextmd > 0 as libc::c_int {
        mdeg_node = nextmd;
        nextmd = *invp.offset(mdeg_node as isize);
        *marker.offset(mdeg_node as isize) = maxint;
        *invp.offset(mdeg_node as isize) = -num;
        num = num + 1;
    }
    if !(num > neqns) {
        tag = 1;
        *head.offset(1 as isize) = 0 as libc::c_int;
        mdeg = 2 as libc::c_int;
        's_88: loop {
            while *head.offset(mdeg as isize) <= 0 as libc::c_int {
                mdeg += 1;
                mdeg;
            }
            mdlmt = mdeg + delta;
            ehead = 0 as libc::c_int;
            '_n500: loop {
                mdeg_node = *head.offset(mdeg as isize);
                while mdeg_node <= 0 as libc::c_int {
                    mdeg += 1;
                    mdeg;
                    if mdeg > mdlmt {
                        break '_n500;
                    }
                    mdeg_node = *head.offset(mdeg as isize);
                }
                nextmd = *invp.offset(mdeg_node as isize);
                *head.offset(mdeg as isize) = nextmd;
                if nextmd > 0 as libc::c_int {
                    *perm.offset(nextmd as isize) = -mdeg;
                }
                *invp.offset(mdeg_node as isize) = -num;
                *ncsub += mdeg + *qsize.offset(mdeg_node as isize) - 2 as libc::c_int;
                if num + *qsize.offset(mdeg_node as isize) > neqns {
                    break 's_88;
                }
                tag += 1;
                tag;
                if tag >= maxint {
                    tag = 1;
                    i = 1;
                    while i <= neqns {
                        if *marker.offset(i as isize) < maxint {
                            *marker.offset(i as isize) = 0 as libc::c_int;
                        }
                        i += 1;
                        i;
                    }
                }
                libmetis__mmdelm(
                    mdeg_node, xadj, adjncy, head, invp, perm, qsize, list, marker, maxint, tag,
                );
                num += *qsize.offset(mdeg_node as isize);
                *list.offset(mdeg_node as isize) = ehead;
                ehead = mdeg_node;
                if !(delta >= 0 as libc::c_int) {
                    break;
                }
            }
            if num > neqns {
                break;
            }
            libmetis__mmdupd(
                ehead, neqns, xadj, adjncy, delta, &mut mdeg, head, invp, perm, qsize, list,
                marker, maxint, &mut tag,
            );
        }
    }
    libmetis__mmdnum(neqns, perm, invp, qsize);
    xadj = slice::from_raw_parts_mut(xadj.as_mut_ptr().offset(1), xadj.len() - 1);

    adjncy = adjncy.offset(1);
    adjncy;
    invp = invp.offset(1);
    invp;
    perm = perm.offset(1);
    perm;
    head = head.offset(1);
    head;
    qsize = qsize.offset(1);
    qsize;
    list = list.offset(1);
    list;
    marker = marker.offset(1);
    marker;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__mmdelm(
    mut mdeg_node: idx_t,
    mut xadj: &mut [idx_t],
    mut adjncy: *mut idx_t,
    mut head: *mut idx_t,
    mut forward: *mut idx_t,
    mut backward: *mut idx_t,
    mut qsize: *mut idx_t,
    mut list: *mut idx_t,
    mut marker: *mut idx_t,
    mut maxint: idx_t,
    mut tag: idx_t,
) {
    let mut element: idx_t = 0;
    let mut i: idx_t = 0;
    let mut istop: idx_t = 0;
    let mut istart: idx_t = 0;
    let mut j: idx_t = 0;
    let mut jstop: idx_t = 0;
    let mut jstart: idx_t = 0;
    let mut link: idx_t = 0;
    let mut nabor: idx_t = 0;
    let mut node: idx_t = 0;
    let mut npv: idx_t = 0;
    let mut nqnbrs: idx_t = 0;
    let mut nxnode: idx_t = 0;
    let mut pvnode: idx_t = 0;
    let mut rlmt: idx_t = 0;
    let mut rloc: idx_t = 0;
    let mut rnode: idx_t = 0;
    let mut xqnbr: idx_t = 0;
    *marker.offset(mdeg_node as isize) = tag;
    istart = xadj[(mdeg_node as usize)];
    istop = xadj[((mdeg_node + 1) as usize)] - 1;
    element = 0 as libc::c_int;
    rloc = istart;
    rlmt = istop;
    i = istart;
    while i <= istop {
        nabor = *adjncy.offset(i as isize);
        if nabor == 0 as libc::c_int {
            break;
        }
        if *marker.offset(nabor as isize) < tag {
            *marker.offset(nabor as isize) = tag;
            if *forward.offset(nabor as isize) < 0 as libc::c_int {
                *list.offset(nabor as isize) = element;
                element = nabor;
            } else {
                *adjncy.offset(rloc as isize) = nabor;
                rloc += 1;
                rloc;
            }
        }
        i += 1;
        i;
    }
    while element > 0 as libc::c_int {
        *adjncy.offset(rlmt as isize) = -element;
        link = element;
        '_n400: loop {
            jstart = xadj[(link as usize)];
            jstop = xadj[((link + 1) as usize)] - 1;
            j = jstart;
            loop {
                if !(j <= jstop) {
                    break '_n400;
                }
                node = *adjncy.offset(j as isize);
                link = -node;
                if node < 0 as libc::c_int {
                    break;
                }
                if node == 0 as libc::c_int {
                    break '_n400;
                }
                if *marker.offset(node as isize) < tag
                    && *forward.offset(node as isize) >= 0 as libc::c_int
                {
                    *marker.offset(node as isize) = tag;
                    while rloc >= rlmt {
                        link = -*adjncy.offset(rlmt as isize);
                        rloc = xadj[(link as usize)];
                        rlmt = xadj[((link + 1) as usize)] - 1;
                    }
                    *adjncy.offset(rloc as isize) = node;
                    rloc += 1;
                    rloc;
                }
                j += 1;
                j;
            }
        }
        element = *list.offset(element as isize);
    }
    if rloc <= rlmt {
        *adjncy.offset(rloc as isize) = 0 as libc::c_int;
    }
    link = mdeg_node;
    '_n1100: loop {
        istart = xadj[(link as usize)];
        istop = xadj[((link + 1) as usize)] - 1;
        i = istart;
        loop {
            if !(i <= istop) {
                break '_n1100;
            }
            rnode = *adjncy.offset(i as isize);
            link = -rnode;
            if rnode < 0 as libc::c_int {
                break;
            }
            if rnode == 0 as libc::c_int {
                return;
            }
            pvnode = *backward.offset(rnode as isize);
            if pvnode != 0 as libc::c_int && pvnode != -maxint {
                nxnode = *forward.offset(rnode as isize);
                if nxnode > 0 as libc::c_int {
                    *backward.offset(nxnode as isize) = pvnode;
                }
                if pvnode > 0 as libc::c_int {
                    *forward.offset(pvnode as isize) = nxnode;
                }
                npv = -pvnode;
                if pvnode < 0 as libc::c_int {
                    *head.offset(npv as isize) = nxnode;
                }
            }
            jstart = xadj[(rnode as usize)];
            jstop = xadj[((rnode + 1) as usize)] - 1;
            xqnbr = jstart;
            j = jstart;
            while j <= jstop {
                nabor = *adjncy.offset(j as isize);
                if nabor == 0 as libc::c_int {
                    break;
                }
                if *marker.offset(nabor as isize) < tag {
                    *adjncy.offset(xqnbr as isize) = nabor;
                    xqnbr += 1;
                    xqnbr;
                }
                j += 1;
                j;
            }
            nqnbrs = xqnbr - jstart;
            if nqnbrs <= 0 as libc::c_int {
                let ref mut fresh0 = *qsize.offset(mdeg_node as isize);
                *fresh0 += *qsize.offset(rnode as isize);
                *qsize.offset(rnode as isize) = 0 as libc::c_int;
                *marker.offset(rnode as isize) = maxint;
                *forward.offset(rnode as isize) = -mdeg_node;
                *backward.offset(rnode as isize) = -maxint;
            } else {
                *forward.offset(rnode as isize) = nqnbrs + 1;
                *backward.offset(rnode as isize) = 0 as libc::c_int;
                *adjncy.offset(xqnbr as isize) = mdeg_node;
                xqnbr += 1;
                xqnbr;
                if xqnbr <= jstop {
                    *adjncy.offset(xqnbr as isize) = 0 as libc::c_int;
                }
            }
            i += 1;
            i;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__mmdint(
    mut neqns: idx_t,
    mut xadj: &mut [idx_t],
    mut adjncy: *mut idx_t,
    mut head: *mut idx_t,
    mut forward: *mut idx_t,
    mut backward: *mut idx_t,
    mut qsize: *mut idx_t,
    mut list: *mut idx_t,
    mut marker: *mut idx_t,
) -> idx_t {
    let mut fnode: idx_t = 0;
    let mut ndeg: idx_t = 0;
    let mut node: idx_t = 0;
    node = 1;
    while node <= neqns {
        *head.offset(node as isize) = 0 as libc::c_int;
        *qsize.offset(node as isize) = 1;
        *marker.offset(node as isize) = 0 as libc::c_int;
        *list.offset(node as isize) = 0 as libc::c_int;
        node += 1;
        node;
    }
    node = 1;
    while node <= neqns {
        ndeg = xadj[((node + 1) as usize)] - xadj[(node as usize)];
        if ndeg == 0 as libc::c_int {
            ndeg = 1;
        }
        fnode = *head.offset(ndeg as isize);
        *forward.offset(node as isize) = fnode;
        *head.offset(ndeg as isize) = node;
        if fnode > 0 as libc::c_int {
            *backward.offset(fnode as isize) = node;
        }
        *backward.offset(node as isize) = -ndeg;
        node += 1;
        node;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__mmdnum(
    mut neqns: idx_t,
    mut perm: *mut idx_t,
    mut invp: *mut idx_t,
    mut qsize: *mut idx_t,
) {
    let mut father: idx_t = 0;
    let mut nextf: idx_t = 0;
    let mut node: idx_t = 0;
    let mut nqsize: idx_t = 0;
    let mut num: idx_t = 0;
    let mut root: idx_t = 0;
    node = 1;
    while node <= neqns {
        nqsize = *qsize.offset(node as isize);
        if nqsize <= 0 as libc::c_int {
            *perm.offset(node as isize) = *invp.offset(node as isize);
        }
        if nqsize > 0 as libc::c_int {
            *perm.offset(node as isize) = -*invp.offset(node as isize);
        }
        node += 1;
        node;
    }
    node = 1;
    while node <= neqns {
        if *perm.offset(node as isize) <= 0 as libc::c_int {
            father = node;
            while *perm.offset(father as isize) <= 0 as libc::c_int {
                father = -*perm.offset(father as isize);
            }
            root = father;
            num = *perm.offset(root as isize) + 1;
            *invp.offset(node as isize) = -num;
            *perm.offset(root as isize) = num;
            father = node;
            nextf = -*perm.offset(father as isize);
            while nextf > 0 as libc::c_int {
                *perm.offset(father as isize) = -root;
                father = nextf;
                nextf = -*perm.offset(father as isize);
            }
        }
        node += 1;
        node;
    }
    node = 1;
    while node <= neqns {
        num = -*invp.offset(node as isize);
        *invp.offset(node as isize) = num;
        *perm.offset(num as isize) = node;
        node += 1;
        node;
    }
}
#[no_mangle]
pub unsafe extern "C" fn libmetis__mmdupd(
    mut ehead: idx_t,
    mut neqns: idx_t,
    mut xadj: &mut [idx_t],
    mut adjncy: *mut idx_t,
    mut delta: idx_t,
    mut mdeg: *mut idx_t,
    mut head: *mut idx_t,
    mut forward: *mut idx_t,
    mut backward: *mut idx_t,
    mut qsize: *mut idx_t,
    mut list: *mut idx_t,
    mut marker: *mut idx_t,
    mut maxint: idx_t,
    mut tag: *mut idx_t,
) {
    let mut current_block: u64;
    let mut deg: idx_t = 0;
    let mut deg0: idx_t = 0;
    let mut element: idx_t = 0;
    let mut enode: idx_t = 0;
    let mut fnode: idx_t = 0;
    let mut i: idx_t = 0;
    let mut iq2: idx_t = 0;
    let mut istop: idx_t = 0;
    let mut istart: idx_t = 0;
    let mut j: idx_t = 0;
    let mut jstop: idx_t = 0;
    let mut jstart: idx_t = 0;
    let mut link: idx_t = 0;
    let mut mdeg0: idx_t = 0;
    let mut mtag: idx_t = 0;
    let mut nabor: idx_t = 0;
    let mut node: idx_t = 0;
    let mut q2head: idx_t = 0;
    let mut qxhead: idx_t = 0;
    mdeg0 = *mdeg + delta;
    element = ehead;
    loop {
        if element <= 0 as libc::c_int {
            return;
        }
        mtag = *tag + mdeg0;
        if mtag >= maxint {
            *tag = 1;
            i = 1;
            while i <= neqns {
                if *marker.offset(i as isize) < maxint {
                    *marker.offset(i as isize) = 0 as libc::c_int;
                }
                i += 1;
                i;
            }
            mtag = *tag + mdeg0;
        }
        q2head = 0 as libc::c_int;
        qxhead = 0 as libc::c_int;
        deg0 = 0 as libc::c_int;
        link = element;
        '_n400: loop {
            istart = xadj[(link as usize)];
            istop = xadj[((link + 1) as usize)] - 1;
            i = istart;
            loop {
                if !(i <= istop) {
                    break '_n400;
                }
                enode = *adjncy.offset(i as isize);
                link = -enode;
                if enode < 0 as libc::c_int {
                    break;
                }
                if enode == 0 as libc::c_int {
                    break '_n400;
                }
                if *qsize.offset(enode as isize) != 0 as libc::c_int {
                    deg0 += *qsize.offset(enode as isize);
                    *marker.offset(enode as isize) = mtag;
                    if *backward.offset(enode as isize) == 0 as libc::c_int {
                        if *forward.offset(enode as isize) != 2 as libc::c_int {
                            *list.offset(enode as isize) = qxhead;
                            qxhead = enode;
                        } else {
                            *list.offset(enode as isize) = q2head;
                            q2head = enode;
                        }
                    }
                }
                i += 1;
                i;
            }
        }
        enode = q2head;
        iq2 = 1;
        '_n900: loop {
            if enode <= 0 as libc::c_int {
                enode = qxhead;
                iq2 = 0 as libc::c_int;
                current_block = 1867586960090983514;
            } else if *backward.offset(enode as isize) != 0 as libc::c_int {
                current_block = 17454654331926439509;
            } else {
                *tag += 1;
                *tag;
                deg = deg0;
                istart = xadj[(enode as usize)];
                nabor = *adjncy.offset(istart as isize);
                if nabor == element {
                    nabor = *adjncy.offset((istart + 1) as isize);
                }
                link = nabor;
                if *forward.offset(nabor as isize) >= 0 as libc::c_int {
                    deg += *qsize.offset(nabor as isize);
                } else {
                    '_n1000: loop {
                        istart = xadj[(link as usize)];
                        istop = xadj[((link + 1) as usize)] - 1;
                        i = istart;
                        while i <= istop {
                            node = *adjncy.offset(i as isize);
                            link = -node;
                            if node != enode {
                                if node < 0 as libc::c_int {
                                    continue '_n1000;
                                }
                                if node == 0 as libc::c_int {
                                    break '_n1000;
                                }
                                if *qsize.offset(node as isize) != 0 as libc::c_int {
                                    if *marker.offset(node as isize) < *tag {
                                        *marker.offset(node as isize) = *tag;
                                        deg += *qsize.offset(node as isize);
                                    } else if *backward.offset(node as isize) == 0 as libc::c_int {
                                        if *forward.offset(node as isize) == 2 as libc::c_int {
                                            let ref mut fresh1 = *qsize.offset(enode as isize);
                                            *fresh1 += *qsize.offset(node as isize);
                                            *qsize.offset(node as isize) = 0 as libc::c_int;
                                            *marker.offset(node as isize) = maxint;
                                            *forward.offset(node as isize) = -enode;
                                            *backward.offset(node as isize) = -maxint;
                                        } else if *backward.offset(node as isize)
                                            == 0 as libc::c_int
                                        {
                                            *backward.offset(node as isize) = -maxint;
                                        }
                                    }
                                }
                            }
                            i += 1;
                            i;
                        }
                        break;
                    }
                }
                current_block = 8022740728492411272;
            }
            loop {
                match current_block {
                    17454654331926439509 => {
                        enode = *list.offset(enode as isize);
                        if iq2 == 1 {
                            break;
                        } else {
                            current_block = 1867586960090983514;
                        }
                    }
                    8022740728492411272 => {
                        deg = deg - *qsize.offset(enode as isize) + 1;
                        fnode = *head.offset(deg as isize);
                        *forward.offset(enode as isize) = fnode;
                        *backward.offset(enode as isize) = -deg;
                        if fnode > 0 as libc::c_int {
                            *backward.offset(fnode as isize) = enode;
                        }
                        *head.offset(deg as isize) = enode;
                        if deg < *mdeg {
                            *mdeg = deg;
                        }
                        current_block = 17454654331926439509;
                    }
                    _ => {
                        if enode <= 0 as libc::c_int {
                            break '_n900;
                        }
                        if *backward.offset(enode as isize) != 0 as libc::c_int {
                            current_block = 17454654331926439509;
                            continue;
                        }
                        *tag += 1;
                        *tag;
                        deg = deg0;
                        istart = xadj[(enode as usize)];
                        istop = xadj[((enode + 1) as usize)] - 1;
                        i = istart;
                        while i <= istop {
                            nabor = *adjncy.offset(i as isize);
                            if nabor == 0 as libc::c_int {
                                break;
                            }
                            if *marker.offset(nabor as isize) < *tag {
                                *marker.offset(nabor as isize) = *tag;
                                link = nabor;
                                if *forward.offset(nabor as isize) >= 0 as libc::c_int {
                                    deg += *qsize.offset(nabor as isize);
                                } else {
                                    '_n1700: loop {
                                        jstart = xadj[(link as usize)];
                                        jstop = xadj[((link + 1) as usize)]
                                            - 1;
                                        j = jstart;
                                        loop {
                                            if !(j <= jstop) {
                                                break '_n1700;
                                            }
                                            node = *adjncy.offset(j as isize);
                                            link = -node;
                                            if node < 0 as libc::c_int {
                                                break;
                                            }
                                            if node == 0 as libc::c_int {
                                                break '_n1700;
                                            }
                                            if *marker.offset(node as isize) < *tag {
                                                *marker.offset(node as isize) = *tag;
                                                deg += *qsize.offset(node as isize);
                                            }
                                            j += 1;
                                            j;
                                        }
                                    }
                                }
                            }
                            i += 1;
                            i;
                        }
                        current_block = 8022740728492411272;
                    }
                }
            }
        }
        *tag = mtag;
        element = *list.offset(element as isize);
    }
}
