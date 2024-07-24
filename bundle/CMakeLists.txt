set(CMAKE_CXX_STANDARD 23)


add_executable(bundler bundler.cc)

target_include_directories(bundler PRIVATE ${ctre_SOURCE_DIR}/single-header)
include_directories(${CMAKE_SOURCE_DIR})