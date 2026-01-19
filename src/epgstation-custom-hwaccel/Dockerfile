# syntax=docker/dockerfile:1

ARG NVENCC_RUNTIMES="libnvidia-compute libswresample4 libavcodec60 libavformat60 libavfilter9"
ARG RUNTIMES="ca-certificates ${NVENCC_RUNTIMES}"

FROM ghcr.io/slashnephy/epgstation:20260118223249@sha256:fc5d554a5583323940b794c8ee1a6be1b25ed765fec4a7a3447bcfd16d1ea9a4 AS epgstation
FROM ghcr.io/slashnephy/nvencc:20260103153428@sha256:7f8469db22c76c84c315d08336ac00e815cff4a62fa9310e39fdb0b4db0e910a AS nvencc
FROM ghcr.io/slashnephy/qsvencc:20260106225130@sha256:03908ff946fbec0895247508be74c5ef956b707e8cb4576d704df387696f420c AS qsvencc
FROM ghcr.io/slashnephy/vceencc:20260107024438@sha256:53de6739422c9c9ab83d1ec987613eec9172bff79234fdb307369a721b75539a AS vceencc
FROM node:24.13.0-bookworm-slim@sha256:bf22df20270b654c4e9da59d8d4a3516cce6ba2852e159b27288d645b7a7eedc AS node

FROM ghcr.io/slashnephy/ffmpeg-ubuntu:20260118223501@sha256:fb7266fac57b579caef25cc58124a039e461d8920a42b55a595238e0a9402f76
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
