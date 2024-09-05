cmake_minimum_required(VERSION 3.19.2)

project(icpc)

set(CMAKE_CXX_STANDARD 20)
set(COMPILE_EXT true)
add_definitions(-Dbadcw)

set(PROJECT_DIR ${CMAKE_SOURCE_DIR})
set(BUNDLE_OUT_DIR ${PROJECT_DIR}/bin_output)
set(BIN_PATH bin)
set(ICPC_EXECUTABLES_LIST "" CACHE INTERNAL "List of icpc targets")
set(ICPC_MAIN_SOURCE "" CACHE INTERNAL "List of icpc main sources")

function(add_icpc_executable target_name main_source)
    set(full_main_source_path "${CMAKE_CURRENT_SOURCE_DIR}/${main_source}")
    add_executable(${target_name} ${full_main_source_path} ${ARGN})
    set_property(GLOBAL APPEND PROPERTY ICPC_EXECUTABLES_LIST "${target_name}")
    set_property(GLOBAL APPEND PROPERTY ICPC_MAIN_SOURCE "${full_main_source_path}")
endfunction(add_icpc_executable)

# ext
if(${COMPILE_EXT})

    # third party
    if(DEFINED ENV{ENABLE_THIRD_PARTY})
        message(status "ENABLE_THIRD_PARTY is " $ENV{ENABLE_THIRD_PARTY})

        set(gtest_force_shared_crt ON CACHE BOOL "" FORCE)
        set(THIRD_PARTY_DIR third_party)
        file(GLOB THRID_PARTY ${THIRD_PARTY_DIR}/*)
        foreach(v ${THRID_PARTY})
            string(REGEX MATCH "${THIRD_PARTY_DIR}/.*" relative_path ${v})
            string(REGEX REPLACE "${THIRD_PARTY_DIR}/" "" dir_name ${relative_path})
            add_subdirectory(${v})
            set(${dir_name}_SOURCE_DIR ${v})
        endforeach()
        # add_subdirectory(minify)

        # testing
        if(DEFINED ENV{ENABLE_TESTING})
            message(status "ENABLE_TESTING is " $ENV{ENABLE_TESTING})
            add_subdirectory(tests)
            enable_testing()
        endif()

        # archive TODO
        if(DEFINED ENV{ENABLE_ARCHIVE})
            message(status "ENABLE_ARCHIVE is " $ENV{ENABLE_ARCHIVE})
            add_subdirectory(archive)
        endif()
    endif()
    
    # bins
    if(DEFINED ENV{ENABLE_BINS})
        message(status "ENABLE_BINS is " $ENV{ENABLE_BINS})
        file(GLOB_RECURSE CC_SRC_LIST ${CMAKE_CURRENT_LIST_DIR}/${BIN_PATH}/*.cc)
        set_directory_properties(PROPERTIES CMAKE_CONFIGURE_DEPENDS ${CMAKE_HOME_DIRECTORY}/${BIN_PATH})
        foreach(v ${CC_SRC_LIST})
            string(REGEX MATCH "${BIN_PATH}/.*" relative_path ${v})
            string(REGEX REPLACE "${BIN_PATH}/" "" target_name ${relative_path})
            string(REGEX REPLACE ".cc" "" target_name ${target_name})
            add_icpc_executable(bin_${target_name} ${relative_path})
        endforeach()
    endif()


endif()

# main
add_icpc_executable(main main.cc)

get_property(icpc_executable_list GLOBAL PROPERTY ICPC_EXECUTABLES_LIST)
get_property(icpc_main_source GLOBAL PROPERTY ICPC_MAIN_SOURCE)
add_subdirectory(tools)
configure_file(consts.h.in ${CMAKE_CURRENT_SOURCE_DIR}/consts.h @ONLY)
include_directories(${CMAKE_SOURCE_DIR})