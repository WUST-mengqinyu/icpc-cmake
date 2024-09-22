set(CMAKE_CXX_STANDARD 20)

file(GLOB CONTEST_TYPE_DIRS RELATIVE ${CMAKE_CURRENT_SOURCE_DIR} ${CMAKE_CURRENT_SOURCE_DIR}/*)

foreach(CONTEST_TYPE_DIR ${CONTEST_TYPE_DIRS})
    if(IS_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/${CONTEST_TYPE_DIR})
        file(GLOB CONTEST_DIRS RELATIVE ${CMAKE_CURRENT_SOURCE_DIR}/${CONTEST_TYPE_DIR} ${CMAKE_CURRENT_SOURCE_DIR}/${CONTEST_TYPE_DIR}/*)

        foreach(CONTEST_DIR ${CONTEST_DIRS})
            if(IS_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/${CONTEST_TYPE_DIR}/${CONTEST_DIR})
                file(GLOB PROBLEM_DIRS RELATIVE ${CMAKE_CURRENT_SOURCE_DIR}/${CONTEST_TYPE_DIR}/${CONTEST_DIR} ${CMAKE_CURRENT_SOURCE_DIR}/${CONTEST_TYPE_DIR}/${CONTEST_DIR}/*)

                foreach(PROBLEM_DIR ${PROBLEM_DIRS})
                    if(EXISTS "${CMAKE_CURRENT_SOURCE_DIR}/${CONTEST_TYPE_DIR}/${CONTEST_DIR}/${PROBLEM_DIR}/CMakeLists.txt"
                        AND NOT EXISTS "${CMAKE_CURRENT_SOURCE_DIR}/${CONTEST_TYPE_DIR}/${CONTEST_DIR}/${PROBLEM_DIR}/.cmake-ignore")
                        
                        if(NOT DEFINED ENV{RUNNING_CONTEST} OR "$ENV{RUNNING_CONTEST}" STREQUAL "${CONTEST_TYPE_DIR}${CONTEST_DIR}")
                            add_subdirectory("${CONTEST_TYPE_DIR}/${CONTEST_DIR}/${PROBLEM_DIR}")
                        endif()
                    endif()
                endforeach()
            endif()
        endforeach()
    endif()
endforeach()