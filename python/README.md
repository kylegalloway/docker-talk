# Python Radar Reader

A small project that reads radar files using the [LRose library](https://github.com/NCAR/lrose-core).

## Build with Docker

### Build Docker image

`docker build -t reader_image .`

### Build application

```docker run --rm -it -v "$PWD":/src -u `id -u`:`id -g` reader_image /bin/bash```

