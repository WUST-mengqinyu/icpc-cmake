set(CMAKE_CXX_STANDARD 23)

configure_file(src/consts.rs.in ${CMAKE_CURRENT_SOURCE_DIR}/src/consts.rs @ONLY)

# rebundler when icpc_main_source file changed
list(LENGTH icpc_main_source icpc_main_source_len)
math(EXPR icpc_main_source_len "${len} - 1")
foreach(idx RANGE ${icpc_main_source_len})
    list(GET icpc_executable_list ${idx} exec_name)
    list(GET icpc_main_source ${idx} source_file)

    add_custom_command(
      OUTPUT ${PROJECT_DIR}/bin_output/${exec_name}.cc
      COMMAND icpc-bundler gen --exec-name ${exec_name} --exec-path ${source_file} && gcc -fpreprocessed -dD -E ${PROJECT_DIR}/bin_output/${exec_name}.cc > ${PROJECT_DIR}/bin_output/${exec_name}_min.cc
      DEPENDS ${source_file}
      COMMENT "Executing bundler because the specific file was modified."
    )

    add_custom_target(run_my_executable_on_file_change_${exec_name} ALL
        DEPENDS ${PROJECT_DIR}/bin_output/${exec_name}.cc
    )
endforeach()