FROM ubuntu:18.04
LABEL maintainer="Kyle Galloway <kyle.galloway.1993@gmail.com>"

# Install system packages
RUN apt-get update \
    && apt-get install -y \
    git \
    gcc \
    g++ \
    wget \
    cmake

# Build Eigen
# ENV EIGEN_VERSION 3.3.7
ENV EIGEN_VERSION default
RUN wget -q http://bitbucket.org/eigen/eigen/get/${EIGEN_VERSION}.tar.gz -O /tmp/eigen.tar.gz \
   && tar -xzf /tmp/eigen.tar.gz -C /tmp \
   && cp -r /tmp/eigen-eigen-537d95e06d30 /usr/src/eigen3 \
   && cd /usr/src/eigen3 \
   && mkdir build \
   && cd build\
   && cmake .. \
   && make install

# Local code stuff
RUN mkdir /eigen
WORKDIR /eigen
VOLUME /eigen
CMD [ "/bin/bash" ]

