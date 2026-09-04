# syntax=docker/dockerfile:1

ARG NVENCC_RUNTIMES="libnvidia-compute libswresample4 libavcodec60 libavformat60 libavfilter9"
ARG RUNTIMES="ca-certificates ${NVENCC_RUNTIMES}"

FROM ghcr.io/slashnephy/epgstation:20260904150401@sha256:d50be7ebba70c48c30d8ab2606918d186474aa0448f211a3611713334c783d68 AS epgstation
FROM ghcr.io/slashnephy/nvencc:20260805105256@sha256:8513bb7358a91505a3e20f554a9a1374a95c2d717b8ff3b2b9a440cbcdd991eb AS nvencc
FROM ghcr.io/slashnephy/qsvencc:20260827110912@sha256:c035ec95c0803a17fa0fe569ea70588447c87924a73129f31a6b300a7eea4095 AS qsvencc
FROM ghcr.io/slashnephy/vceencc:20260826002728@sha256:3423a5538dede0d8d635dc39a6a67fe1b4651846bf9e954df0dfc5f573903f1f AS vceencc
FROM node:24.20.0-bookworm-slim@sha256:ba849c60be29959425b8734d57b8b4b7d56f98edd9504c9af091d5281095a71e AS node

FROM ghcr.io/slashnephy/ffmpeg-ubuntu:20260805105446@sha256:603b397428c2a2cd08a3636394a85be71300c577a37850e50f7d60435a00d433
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
