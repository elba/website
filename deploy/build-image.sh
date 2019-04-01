#!/bin/sh
set -e

# Build backend bianry
docker-compose -f docker-compose-build.yml up

mkdir -p .tmp/build/

# Copy targets from builder container
docker container cp backend-builder:/app/target/x86_64-unknown-linux-musl/release/elba-backend ./.tmp/build/elba-backend
docker container cp backend-builder:/root/.cargo/bin/diesel ./.tmp/build/diesel

cp -r ../migrations ./.tmp/build

# Build production image
docker build --no-cache -t elba/backend:latest . -f Dockerfile.run

rm .tmp/ -rf
