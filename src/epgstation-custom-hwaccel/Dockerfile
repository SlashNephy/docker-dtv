# syntax=docker/dockerfile:1

ARG NVENCC_RUNTIMES="libnvidia-compute libswresample4 libavcodec60 libavformat60 libavfilter9"
ARG RUNTIMES="ca-certificates ${NVENCC_RUNTIMES}"

FROM ghcr.io/slashnephy/epgstation:20260702122901@sha256:337736fc4b19b841f1fbbcb61143e82a6946a414a1128fa9a5ed0a3ebde9b23d AS epgstation
FROM ghcr.io/slashnephy/nvencc:20260625100104@sha256:c2165cb31647d9d03a139af42afc1831f57ee86a746c5c841dc7d02b01fd03bb AS nvencc
FROM ghcr.io/slashnephy/qsvencc:20260627065121@sha256:9c73f0add2ddd72c0524c95254b20450604797d8e6609313d3402cb0c9a2c7eb AS qsvencc
FROM ghcr.io/slashnephy/vceencc:20260624070533@sha256:eaf67b332bd525325e2dd8a15f2b741460676ddf6c72600b4ba6f03b09012b42 AS vceencc
FROM node:24.18.0-bookworm-slim@sha256:b31e7a42fdf8b8aa5f5ed477c72d694301273f1069c5a2f71d53c6482e99a2fc AS node

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
