# website

[![Build Status](https://travis-ci.com/elba/website.svg?branch=master)](https://travis-ci.com/elba/website)

elba's website and registry backend is growing here. It's now under heavy development.

## Install (Development)

elba's registry backend requires `PostgreSQL` to build. You can install it by package manager or an installer provided by the `PostgreSQL` project. Read more in [Official Guide](https://www.postgresql.org/download/).

Also, since elba's registry backend is written in Rust, you need to install the rust nightly toolchain first, and then run the commands below to start the backend.

```
$ cargo install diesel_cli --no-default-features --features postgres
$ diesel setup
$ cargo run
```

Note that all of the configruations of registry backend is passed in by environment variables. When it starts it reads `.env` file and mash it with the existing environment variables. You would like to change something in that file rather than in the shell enviroment when you do development.

You may also want to build frontend project as well, then run the following commands, and it will output build results to `/public` :

```
cd frontend
yarn build
```

## Architecture

The website project consists of a frontend project (lives in `/frontend`) and a registry api backend. The registry backend only exposes restful apis that serves package upload/downloading and package searching for elba cli program, and it also provides metadata endpoints for frontend app to show package information. 

In current design, the registry backend is not responsible for hosting frontend static files. Instead, the frontend static files is hosted on AWS S3 and is behind CloudFront CDN to improve access quality.

## Deploy

A simplest way to deploy the website would be:

1. Clone this repo into your server.
2. Install Docker-ce and docker-compose.
3. Edit `docker-compose.yml` or `.env` (suit yourself) to fill in enviroment varibles.
4. Run '`cd deploy && bash ./docker-build-image.sh`', then you will have a local docker image with tag `elba/registry:latest`.
5. (Optional) setup a reverse proxy (e.g nginx) to enable `https` access as well as to serve static files in `/public`.
6. Run `docker-compose up`.
7. Setup a remote index repo. You can start with the example `deploy/index-bare-example.tar`, and remember to change the registry url to your real domain.

## Usage

Add remote index url into elba config file (`~/.elba/config.toml`). For example:

```
[indices]
official = "index+git+https://github.com/andylokandy/index.git#master"
```

or use a local index for testing:

```
[indices]
test = "index+git+/etc/elba-registry/index.git#master"
```

And then you are free to play!