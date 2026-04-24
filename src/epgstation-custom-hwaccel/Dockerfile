# syntax=docker/dockerfile:1

ARG NVENCC_RUNTIMES="libnvidia-compute libswresample4 libavcodec60 libavformat60 libavfilter9"
ARG RUNTIMES="ca-certificates ${NVENCC_RUNTIMES}"

FROM ghcr.io/slashnephy/epgstation:20260423055222@sha256:b56f48f6bd0976fad171fe2e4a169e907dfeb3a112e48066cedb607819490249 AS epgstation
FROM ghcr.io/slashnephy/nvencc:20260424054635@sha256:53bd31b824ba471dccf9015f4fc9a6eb02150a483dcd4e384cda5902287c72c0 AS nvencc
FROM ghcr.io/slashnephy/qsvencc:20260424224934@sha256:3e4f09b5d9ad940993dc68a7453154c8885648533f2ea5c1fd95d672dce50436 AS qsvencc
FROM ghcr.io/slashnephy/vceencc:20260419224513@sha256:a79a4b5ffe032569b88228d54887bd2bd35cff2364e8b18430395537e6bf3465 AS vceencc
FROM node:24.15.0-bookworm-slim@sha256:03eae3ef7e88a9de535496fb488d67e02b9d96a063a8967bae657744ecd513f2 AS node

FROM ghcr.io/slashnephy/ffmpeg-ubuntu:20260424054934@sha256:18140cd3336c1e59d0fad0e1a80c4e4cce58f5d954424c7fbbd82cd4fd3f28bb
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
