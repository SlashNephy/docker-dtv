# syntax=docker/dockerfile:1

ARG NVENCC_RUNTIMES="libnvidia-compute libswresample4 libavcodec60 libavformat60 libavfilter9"
ARG RUNTIMES="ca-certificates ${NVENCC_RUNTIMES}"

FROM ghcr.io/slashnephy/epgstation:20260625054037@sha256:e72c2cdb3340376386aa01e9e83686f556c55c09b6641f0fccf5e5ac710df90f AS epgstation
FROM ghcr.io/slashnephy/nvencc:20260625100104@sha256:c2165cb31647d9d03a139af42afc1831f57ee86a746c5c841dc7d02b01fd03bb AS nvencc
FROM ghcr.io/slashnephy/qsvencc:20260424224934@sha256:3e4f09b5d9ad940993dc68a7453154c8885648533f2ea5c1fd95d672dce50436 AS qsvencc
FROM ghcr.io/slashnephy/vceencc:20260624070533@sha256:eaf67b332bd525325e2dd8a15f2b741460676ddf6c72600b4ba6f03b09012b42 AS vceencc
FROM node:24.16.0-bookworm-slim@sha256:2c87ef9bd3c6a3bd4b472b4bec2ce9d16354b0c574f736c476489d09f560a203 AS node

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
