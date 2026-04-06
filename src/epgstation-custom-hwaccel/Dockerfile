# syntax=docker/dockerfile:1

ARG NVENCC_RUNTIMES="libnvidia-compute libswresample4 libavcodec60 libavformat60 libavfilter9"
ARG RUNTIMES="ca-certificates ${NVENCC_RUNTIMES}"

FROM ghcr.io/slashnephy/epgstation:20260329013617@sha256:ac10e68b1bbbc327be6934aadd0c574e8efa125e6da9017cd5e213bb9948bf21 AS epgstation
FROM ghcr.io/slashnephy/nvencc:20260320143306@sha256:01a2933ad66e32f8f92bceea6f003099dc990f286e274e86690c8b02c5f3b628 AS nvencc
FROM ghcr.io/slashnephy/qsvencc:20260106225130@sha256:03908ff946fbec0895247508be74c5ef956b707e8cb4576d704df387696f420c AS qsvencc
FROM ghcr.io/slashnephy/vceencc:20260406061005@sha256:d7542bd1dd00e5238eca3b3fdcc69e94a9f068b6a9bccc7c3e9bc9e8e48b6f9a AS vceencc
FROM node:24.14.1-bookworm-slim@sha256:06e5c9f86bfa0aaa7163cf37a5eaa8805f16b9acb48e3f85645b09d459fc2a9f AS node

FROM ghcr.io/slashnephy/ffmpeg-ubuntu:20260320143557@sha256:67d4d98401068e29e2af2e8f0ebdb1277a8f277aa5fd56412ee6c6b79363bd07
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
