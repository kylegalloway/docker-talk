# Rust GTK counter

Created using `cargo new ./rust-gtk-counter --vcs git --bin --edition 2018 -v --color always`

Following the example [here](https://turbomack.github.io/posts/2019-07-28-rust-vs-gui.html).

## Build with Docker

### Build Docker image

`docker build -t rust-gtk-counter_image .`

### Build application

```docker run --rm -it -v "$PWD":/src -u `id -u`:`id -g` rust-gtk-counter_image /bin/bash -c "cargo build"```

### Run

Due to constraints, this app cannot be run from within a docker container.
However, you can build it inside the container and then run the app from your local computer.
To do this, run `./target/debug/rust-gtk-counter`

