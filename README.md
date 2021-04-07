# Image-Uploader
A sharex compatible image uploader built for speed.

## Features

- web ui with login and uploader
- Normal HTTP uploader
- Uploader that supports forms
- Full ShareX support (example file)
- Automatic file compression
- Speed and Efficiency (Rust btw)

## Env File

it will auto load an env in the same directory, or if there is none it will just get env vars. All config vars are very important 

```env
BASE_URL="url used in json response"
AUTH_TOKEN="api token for rest api"
AUTH_USER="username for web"
AUTH_PASSWORD="password for web"
NAME="name of your service"
URL="0.0.0.0"
PORT=6969 # Whatever port you want
```

## Running in Docker

Simple single lined docker command

```shell
docker run -d -v ./static:/usr/local/bin/static -p 6969:6969 --expose 6969 --env-file .env ghcr.io/daggy1234/image-uploader
```

And there you have a running instance. 

The image doesn't support ARM so you can use the binaries instead

## Running the Binaries

1) Choose the architecture. Those should be in the releases
2) Download the file (wget , curl) based on architecture
3)  Extract the \*.tar.gz file and cd into the folder
4)  Create the .env file for config
5)  `./image-uploader` and look for any logs!


## Contributing

Just fork the repo and run with cargo. for Docker testing there is a dev.Dockerfile that uses Cargo chef. Cargo chef instructions below

```shell
cargo install cargo-chef
cargo chef prepare --recipe-path recipe.json
```


## Issue or bugs

Report em here or on the discord

https://server.daggy.tech
