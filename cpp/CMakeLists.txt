cmake_minimum_required (VERSION 3.1)
project (eigen-tutorial)

find_package (Eigen3 3.3 REQUIRED NO_MODULE)

add_executable (example1 example1.cpp)
add_executable (example2 example2.cpp)

target_compile_options(example1 PRIVATE -march=native -std=c++14)
target_compile_options(example2 PRIVATE -march=native -std=c++14)

target_link_libraries (example1 Eigen3::Eigen)
target_link_libraries (example2 Eigen3::Eigen)

