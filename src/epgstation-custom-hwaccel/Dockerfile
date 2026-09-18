# syntax=docker/dockerfile:1

ARG NVENCC_RUNTIMES="libnvidia-compute libswresample4 libavcodec60 libavformat60 libavfilter9"
ARG RUNTIMES="ca-certificates ${NVENCC_RUNTIMES}"

FROM ghcr.io/slashnephy/epgstation:20260909164345@sha256:ecbd4d29f6b04d9e21afa01c6508e18c2c024df30922e7291164c2a45766efea AS epgstation
FROM ghcr.io/slashnephy/nvencc:20260805105256@sha256:8513bb7358a91505a3e20f554a9a1374a95c2d717b8ff3b2b9a440cbcdd991eb AS nvencc
FROM ghcr.io/slashnephy/qsvencc:20260917210444@sha256:e21b9c0ddc6b456768be79f07882e56dec0bfdf6cc6d1767b458e011adad314e AS qsvencc
FROM ghcr.io/slashnephy/vceencc:20260917093606@sha256:014b0c67d0330290102abc0dcdcea0fdf0a74e8fed3c255e50658b3b5767769a AS vceencc
FROM node:24.20.0-bookworm-slim@sha256:ba849c60be29959425b8734d57b8b4b7d56f98edd9504c9af091d5281095a71e AS node

FROM ghcr.io/slashnephy/ffmpeg-ubuntu:20260909164601@sha256:aa1b33325e4b9e4a41680623ca5000ceae7b606a8bd27c1f5370540a6d01d265
WORKDIR /app
ARG RUNTIMES
ENV TZ="Asia/Tokyo"

SHELL ["/usr/bin/bash", "-euxc"]
RUN <<EOF
  apt-get update
  apt-get install -y --no-install-recommends ${RUNTIMES}

  apt-get clean
  rm -rf /var/lib/apt/lists/*
EOF

COPY --from=epgstation --chown=ubuntu:ubuntu /app/ ./
COPY --from=epgstation --chown=ubuntu:ubuntu /app/client/ ./client/
COPY --from=nvencc /usr/bin/nvencc /usr/local/bin/
COPY --from=qsvencc /usr/bin/qsvencc /usr/local/bin/
COPY --from=vceencc /usr/bin/vceencc /usr/local/bin/
COPY --from=node /usr/local/ /usr/local/

USER ubuntu
ENTRYPOINT [ "node", "dist/index.js" ]
