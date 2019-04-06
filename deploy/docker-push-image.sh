#!/bin/bash
set -ev

REGISTRY_URL="${AWS_ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com"
SOURCE_IMAGE="elba/registry"
TARGET_IMAGE="${REGISTRY_URL}/elba/registry"
# lets make sure we always have access to latest image
TARGET_IMAGE_LATEST="${TARGET_IMAGE}:latest"
TIMESTAMP="$(date '+%Y%m%d%H%M%S')"
# using datetime as part of a version for versioned image
VERSION="${TIMESTAMP}-${TRAVIS_COMMIT}"
# using specific version as well
# it is useful if you want to reference this particular version
TARGET_IMAGE_VERSIONED="${TARGET_IMAGE}:${VERSION}"

# Push image to ECR
###################

# docker login to aws ecr
export AWS_ACCESS_KEY_ID=${AWS_ACCESS_KEY}
export AWS_SECRET_ACCESS_KEY=${AWS_SECRET_KEY}
export AWS_DEFAULT_REGION=${AWS_REGION}
$(aws ecr get-login --no-include-email)

# update latest version
docker tag ${SOURCE_IMAGE} ${TARGET_IMAGE_LATEST}
docker push ${TARGET_IMAGE_LATEST}

# push new version
docker tag ${SOURCE_IMAGE} ${TARGET_IMAGE_VERSIONED}
docker push ${TARGET_IMAGE_VERSIONED}
