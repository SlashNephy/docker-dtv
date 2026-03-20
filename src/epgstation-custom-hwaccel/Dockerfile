# syntax=docker/dockerfile:1

ARG NVENCC_RUNTIMES="libnvidia-compute libswresample4 libavcodec60 libavformat60 libavfilter9"
ARG RUNTIMES="ca-certificates ${NVENCC_RUNTIMES}"

FROM ghcr.io/slashnephy/epgstation:20260317180857@sha256:9f22bfb54b7eeeaa66e29b38c15bcffc8e0e852f8857ab859ce449ba01ef07cb AS epgstation
FROM ghcr.io/slashnephy/nvencc:20260126030508@sha256:51f6780d557a7888ce8c5e7c62137e05e35148af758344f3174653633121734c AS nvencc
FROM ghcr.io/slashnephy/qsvencc:20260106225130@sha256:03908ff946fbec0895247508be74c5ef956b707e8cb4576d704df387696f420c AS qsvencc
FROM ghcr.io/slashnephy/vceencc:20260221092411@sha256:9653265f792000e2ffe3e11daf5fd2647eef9e4d011f8e7d969138d933ff1110 AS vceencc
FROM node:24.14.0-bookworm-slim@sha256:d8e448a56fc63242f70026718378bd4b00f8c82e78d20eefb199224a4d8e33d8 AS node

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
