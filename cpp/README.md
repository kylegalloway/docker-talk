# Eigen tutorial

Tutorial can be found at https://eigen.tuxfamily.org/dox/GettingStarted.html.

## Build

```docker build -t eigen_image .```

## Run

```docker run --rm -it -u `id -u`:`id -g` -v "$PWD":/eigen eigen_image /bin/bash -c "./build.sh; build/example1; build/example2"```

This will build and run both examples.

