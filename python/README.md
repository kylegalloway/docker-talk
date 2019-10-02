# Django tutorial site

A simple site based on the [Django tutorial](https://docs.djangoproject.com/en/2.2/intro/tutorial01/).

## Build with Docker

### Build Docker image

```sh
docker build -t py_image -f Dockerfile .
```

### Build application

```sh
docker run -it --rm --name=py -v "$PWD":/src -p 8000:8000 -u `id -u`:`id -g` py_image
```

