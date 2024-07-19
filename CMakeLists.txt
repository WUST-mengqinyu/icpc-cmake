cmake_minimum_required(VERSION 3.19.2)

project(icpc)

set(CMAKE_CXX_STANDARD 20)

add_definitions(-Dbadcw)
set(PROJECT_DIR ${CMAKE_SOURCE_DIR})
configure_file(consts.h.in ${CMAKE_CURRENT_SOURCE_DIR}/consts.h @ONLY)

add_executable(main main.cc testlib.h)
include_directories(.)

#add_executable(A A.cc)
#add_executable(B B.cc)
#add_executable(C C.cc)
#add_executable(D D.cc)
#add_executable(E E.cc)
#add_executable(F F.cc)
#add_executable(G G.cc)
#add_executable(H H.cc)
#add_executable(I I.cc)
