# CMake generated Testfile for 
# Source directory: /home/maxwell/Documents/metis_in_rust/METIS
# Build directory: /home/maxwell/Documents/metis_in_rust/METIS
# 
# This file includes the relevant testing commands required for 
# testing this directory and lists subdirectories to be tested as well.
add_test([=[GraphCheck]=] "/home/maxwell/Documents/metis_in_rust/METIS/graphchk" "/home/maxwell/Documents/metis_in_rust/METIS/src/tests/tiny_01.graph")
set_tests_properties([=[GraphCheck]=] PROPERTIES  _BACKTRACE_TRIPLES "/home/maxwell/Documents/metis_in_rust/METIS/CMakeLists.txt;47;add_test;/home/maxwell/Documents/metis_in_rust/METIS/CMakeLists.txt;0;")
subdirs("src/include")
subdirs("src/libmetis")
subdirs("src/programs")
