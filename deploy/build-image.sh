#!/bin/sh
set -e

# Build backend bianry
docker-compose -f docker-compose-build.yml up --exit-code-from backend-builder

mkdir -p .build/

# Copy targets from builder container
docker container cp backend-builder:/app/target/x86_64-unknown-linux-musl/release/elba-backend ./.build/elba-backend
docker container cp backend-builder:/root/.cargo/bin/diesel ./.build/diesel

# Include files for production image
cp -r ../migrations ./.build
cp -r ../.env ./.build
cp -r ../.env.override ./.build
cp -r ./backend_entrypoint.sh ./.build

# Build production image
docker build --no-cache -t elba/backend:latest . -f Dockerfile.run

rm .build/ -rf
