cmake_minimum_required (VERSION 3.1)
project (eigen-tutorial VERSION 1.0
			DESCRIPTION "Simple tutorial examples from Eigen"
		        LANGUAGES CXX)

find_package (Eigen3 3.3 REQUIRED NO_MODULE)

add_executable (example1 example1.cpp)
target_compile_features(example1 PRIVATE cxx_std_14)
target_link_libraries (example1 Eigen3::Eigen)

add_executable (example2 example2.cpp)
target_compile_features(example2 PRIVATE cxx_std_14)
target_link_libraries (example2 Eigen3::Eigen)

