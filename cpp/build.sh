#!/bin/bash
# set -x

# Remove and remake build_dir
build_dir=build
echo "Deleting \""${build_dir}\"
rm -r ${build_dir}
mkdir ${build_dir}
cd ${build_dir}

# Build
echo "Building"
cmake ../ && \
    make 


