#!/bin/bash
set -ev

# Build registry bianry
docker-compose -f docker-compose.build.yml up --exit-code-from registry-builder --abort-on-container-exit

rm -rf .build/
mkdir .build/

# Copy targets from builder container
docker container cp registry-builder:/app/target/x86_64-unknown-linux-musl/release/elba-registry .build/elba-registry
docker container cp registry-builder:/root/.cargo/bin/diesel .build/diesel

# Include files for production image
cp -r ../migrations .build
cp -r ./registry_entrypoint.sh .build
# Suppress error message if file doesn't exist
cp -r ../.env .build || :

# Prepare an initial bare index (for local testing)
mkdir .build/tmp
tar -xf index-bare-example.tar -C .build/tmp

# Build production image
docker build --no-cache -t elba/registry:latest .build/ -f Dockerfile.run
