# syntax=docker/dockerfile:1

# renovate: datasource=github-tags depName=l3tnun/EPGStation
ARG EPGSTATION_VERSION="v2.10.0"

ARG COMMIT_ID="unknown"

ARG DEPENDENCIES="build-essential python3"

FROM --platform=$BUILDPLATFORM public.ecr.aws/bitnami/git:2.44.0@sha256:30b4e6611de1031f38cf01fbbd8f2f0a503e5cf7b03634efe36ca3f855a9e89e AS source
ARG EPGSTATION_VERSION

RUN git clone https://github.com/l3tnun/EPGStation -b "${EPGSTATION_VERSION}" /app

FROM --platform=$BUILDPLATFORM public.ecr.aws/bitnami/git:2.44.0@sha256:30b4e6611de1031f38cf01fbbd8f2f0a503e5cf7b03634efe36ca3f855a9e89e AS patch-source
WORKDIR /app

COPY --from=source /app/ ./
COPY ./*.patch /tmp/

RUN git apply --numstat --summary --check --apply --ignore-whitespace -v /tmp/*.patch

FROM --platform=$BUILDPLATFORM public.ecr.aws/debian/debian:stable-slim@sha256:01d9267d7b0ca810310802fb08cb14579479928ae2e701de9205d829f10aeb94 AS jq

# hadolint ignore=DL3009
RUN apt-get update && apt-get install -y --no-install-recommends jq

FROM --platform=$BUILDPLATFORM jq AS patch-version
WORKDIR /app
ARG COMMIT_ID

COPY --from=source /app/package.json ./package.json
COPY --from=source /app/client/package.json ./client/package.json

SHELL ["/bin/bash", "-euxc"]
RUN <<EOF
  cp /usr/share/zoneinfo/Asia/Tokyo /etc/localtime

  jq ".version=\"$(jq -r .version ./package.json)-$(date '+%Y%m%d-%H%M%S')-${COMMIT_ID:0:7}\"" ./package.json > /tmp/server.package.json
  jq ".version=\"$(jq -r .version ./client/package.json)-$(date '+%Y%m%d-%H%M%S')-${COMMIT_ID:0:7}\"" ./client/package.json > /tmp/client.package.json

  mv -f /tmp/server.package.json ./package.json
  mv -f /tmp/client.package.json ./client/package.json
EOF

FROM --platform=$BUILDPLATFORM public.ecr.aws/docker/library/node:20.19.6-bookworm-slim@sha256:a270640213033657548b7ed57852709a0d2ee3b277f902f912367216aa8e9520 AS build-client
WORKDIR /app/client

COPY --from=source /app/client/package*.json ./
RUN npm install

COPY --from=patch-source /app/ ../
RUN npm run build

FROM --platform=$TARGETPLATFORM public.ecr.aws/docker/library/node:20.19.6-bookworm-slim@sha256:a270640213033657548b7ed57852709a0d2ee3b277f902f912367216aa8e9520 AS build-server
WORKDIR /app
ARG DEPENDENCIES

RUN apt-get update && apt-get install -y --no-install-recommends ${DEPENDENCIES}

COPY --from=source /app/package*.json ./
RUN npm install --no-save

COPY --from=patch-source /app/ ./
RUN rm -rf client && npm run build-server

FROM --platform=$TARGETPLATFORM public.ecr.aws/docker/library/node:20.19.6-bookworm-slim@sha256:a270640213033657548b7ed57852709a0d2ee3b277f902f912367216aa8e9520
WORKDIR /app

COPY --from=build-server /app/ ./
COPY --from=build-client /app/client/ ./client/
COPY --from=patch-version /app/package.json ./package.json
COPY --from=patch-version /app/client/package.json ./client/package.json

ENTRYPOINT ["node", "dist/index.js"]
