FROM rust:latest
LABEL maintainer="Kyle Galloway <kyle.galloway@cfdrc.com>"

# Get updates and basic packages
RUN apt-get update -qqy \
    && apt-get install -qqy --no-install-recommends \
        libgtk-3-dev \
    && rm -rf /var/lib/apt/lists/*

ENV SRC_DIR /src
RUN mkdir $SRC_DIR
WORKDIR $SRC_DIR
VOLUME $SRC_DIR

CMD ["/bin/bash"]

