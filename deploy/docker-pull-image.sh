#!/bin/bash
set -ev

REGISTRY_URL="${AWS_ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com"
SOURCE_IMAGE="${REGISTRY_URL}/elba/registry"
SOURCE_IMAGE_LATEST="${SOURCE_IMAGE}:latest"
TARGET_IMAGE_LATEST="elba/registry:latest"

# docker login to aws ecr
export AWS_ACCESS_KEY_ID=${AWS_ACCESS_KEY}
export AWS_SECRET_ACCESS_KEY=${AWS_SECRET_KEY}
export AWS_DEFAULT_REGION=${AWS_REGION}
$(aws ecr get-login --no-include-email)

# pull latest image
docker pull ${SOURCE_IMAGE_LATEST}
docker image tag ${SOURCE_IMAGE_LATEST} ${TARGET_IMAGE_LATEST}
