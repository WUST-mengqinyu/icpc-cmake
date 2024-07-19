cmake_minimum_required(VERSION 3.19.2)

project(icpc)

set(CMAKE_CXX_STANDARD 20)

add_definitions(-Dbadcw)
set(PROJECT_DIR ${CMAKE_SOURCE_DIR})
configure_file(consts.h.in ${CMAKE_CURRENT_SOURCE_DIR}/consts.h @ONLY)

# add_subdirectory(src)
add_executable(main main.cc testlib.h consts.h)
include_directories(${CMAKE_SOURCE_DIR})

file(GLOB_RECURSE CC_SRC_LIST ${CMAKE_CURRENT_LIST_DIR}/src/*.cc)

foreach(v ${CC_SRC_LIST})
    string(REGEX MATCH "src/.*" relative_path ${v})
    string(REGEX REPLACE "src/" "" target_name ${relative_path})
    string(REGEX REPLACE ".cc" "" target_name ${target_name})
    add_executable(bin_${target_name} ${v})
endforeach()
