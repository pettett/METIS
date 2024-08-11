#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(path_statements)]
#![allow(unused)]
#![feature(c_variadic)]
#![feature(extern_types)]
#![feature(thread_local)]

#[macro_use]
extern crate c2rust_bitfields;
extern crate libc;

pub mod GKlib {
    pub mod b64;
    pub mod blas;
    pub mod csr;
    pub mod error;
    pub mod evaluate;
    pub mod fkvkselect;
    pub mod fs;
    pub mod getopt;
    pub mod gkregex;
    pub mod graph;
    pub mod htable;
    pub mod io;
    pub mod itemsets;
    pub mod mcore;
    pub mod memory;
    pub mod omp;
    pub mod pdb;
    pub mod pqueue;
    pub mod random;
    pub mod rw;
    pub mod seq;
    pub mod sort;
    pub mod string;
    pub mod timers;
    pub mod tokenizer;
    pub mod util;
} // mod GKlib
pub mod libmetis {
    pub mod auxapi;
    pub mod balance;
    pub mod bucketsort;
    pub mod checkgraph;
    pub mod coarsen;
    pub mod compress;
    pub mod contig;
    pub mod debug;
    pub mod fm;
    pub mod fortran;
    pub mod gklib;
    pub mod graph;
    pub mod initpart;
    pub mod kmetis;
    pub mod kwayfm;
    pub mod kwayrefine;
    pub mod mcutil;
    pub mod mesh;
    pub mod meshpart;
    pub mod minconn;
    pub mod mincover;
    pub mod mmd;
    pub mod ometis;
    pub mod options;
    pub mod parmetis;
    pub mod pmetis;
    pub mod refine;
    pub mod separator;
    pub mod sfm;
    pub mod srefine;
    pub mod stat;
    pub mod structure;
    pub mod timing;
    pub mod util;
    pub mod wspace;
} // mod libmetis
pub mod programs {
    pub mod cmdline_gpmetis;
    pub mod cmdline_m2gmetis;
    pub mod cmdline_mpmetis;
    pub mod cmdline_ndmetis;
    pub mod cmpfillin;
    pub mod io;
    pub mod smbfactor;
    pub mod stat;
} // mod programs
