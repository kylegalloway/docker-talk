FROM ubuntu:18.04
LABEL maintainer="Kyle Galloway <kyle.galloway@cfdrc.com>"

# Install system packages
RUN apt-get update -qqy \
    && apt-get install -y \
	automake \
	cmake \
	curl \
	g++ \
	gcc \
	gfortran \
	git \
	hdf5-tools \
	libbz2-dev \
	libcurl3-dev \
	libeigen3-dev \
	libexpat1-dev \
	libfftw3-dev \
	libfl-dev \
	libgeographic-dev \
	libhdf5-dev \
	libnetcdf-dev \
	libpng-dev \
	libtool \
	libx11-dev \
	libzip-dev \
	make \
	netcdf-bin \
	pkg-config \
	python \
	qtbase5-dev \
	wget \
    && rm -rf /var/lib/apt/lists/*

ENV LROSE_DIR /lrose_build
RUN mkdir ${LROSE_DIR}
WORKDIR ${LROSE_DIR}

ENV LROSE_VERSION 20190520
ENV LROSE_NAME lrose-core-${LROSE_VERSION}
RUN wget -q https://github.com/NCAR/lrose-core/releases/download/${LROSE_NAME}/${LROSE_NAME}.ubuntu_18.04.amd64.deb -O /tmp/${LROSE_NAME}.ubuntu_18.04.amd64.deb \
    && apt-get install -y /tmp/${LROSE_NAME}.ubuntu_18.04.amd64.deb

ENV SRC_DIR /src
RUN mkdir ${SRC_DIR}
WORKDIR ${SRC_DIR}
VOLUME ${SRC_DIR}

CMD ["/bin/bash"]

