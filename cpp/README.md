# Eigen tutorial

Tutorial can be found at https://eigen.tuxfamily.org/dox/GettingStarted.html.

## Build

```docker build --rm -t eigen .```

## Run

```docker run --rm -it -u `id -u`:`id -g` -v "$PWD":/eigen eigen /bin/bash -c "./build.sh; build/example1; build/example2"```

This will build and run both examples.

