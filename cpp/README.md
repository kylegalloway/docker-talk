# Eigen tutorial

Tutorial can be found at https://eigen.tuxfamily.org/dox/GettingStarted.html.

## Build

```docker build --rm -t eigen .```

## Run

```docker run --rm -it -u `id -u`:`id -g` -v "$PWD":/eigen eigen /bin/bash -c "./build.sh; build/example1;"```

`build/example1` can be replaced with `build/example2` or `build/example3` if you want to try the other examples.

