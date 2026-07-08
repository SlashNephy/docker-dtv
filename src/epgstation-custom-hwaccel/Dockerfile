# syntax=docker/dockerfile:1

ARG NVENCC_RUNTIMES="libnvidia-compute libswresample4 libavcodec60 libavformat60 libavfilter9"
ARG RUNTIMES="ca-certificates ${NVENCC_RUNTIMES}"

FROM ghcr.io/slashnephy/epgstation:20260708080312@sha256:ada7dc69bff8efb852482f2d343f0d08f350d5bf0dd20160e1fc1b3ed8539a81 AS epgstation
FROM ghcr.io/slashnephy/nvencc:20260625100104@sha256:c2165cb31647d9d03a139af42afc1831f57ee86a746c5c841dc7d02b01fd03bb AS nvencc
FROM ghcr.io/slashnephy/qsvencc:20260627065121@sha256:9c73f0add2ddd72c0524c95254b20450604797d8e6609313d3402cb0c9a2c7eb AS qsvencc
FROM ghcr.io/slashnephy/vceencc:20260624070533@sha256:eaf67b332bd525325e2dd8a15f2b741460676ddf6c72600b4ba6f03b09012b42 AS vceencc
FROM node:24.18.0-bookworm-slim@sha256:cb4e8f7c443347358b7875e717c29e27bf9befc8f5a26cf18af3c3dec80e58c5 AS node

FROM ghcr.io/slashnephy/ffmpeg-ubuntu:20260625100312@sha256:4169f8c58f987ba1c9c780780cacd234ba79ca688149c1c27643ab7153fefabb
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
