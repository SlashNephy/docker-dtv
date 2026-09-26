# syntax=docker/dockerfile:1

ARG NVENCC_RUNTIMES="libnvidia-compute libswresample4 libavcodec60 libavformat60 libavfilter9"
ARG RUNTIMES="ca-certificates ${NVENCC_RUNTIMES}"

FROM ghcr.io/slashnephy/epgstation:20260918135847@sha256:5759d812aa0d9e5a3f7560380e9d3f4d901a8deb0c852ab1914c9528db68455f AS epgstation
FROM ghcr.io/slashnephy/nvencc:20260805105256@sha256:8513bb7358a91505a3e20f554a9a1374a95c2d717b8ff3b2b9a440cbcdd991eb AS nvencc
FROM ghcr.io/slashnephy/qsvencc:20260917210444@sha256:e21b9c0ddc6b456768be79f07882e56dec0bfdf6cc6d1767b458e011adad314e AS qsvencc
FROM ghcr.io/slashnephy/vceencc:20260925194753@sha256:da8892a5a81992462bc5747cf52ee5b81ba8ecefbb8cd8384d131d752e9abc80 AS vceencc
FROM node:24.21.0-bookworm-slim@sha256:0e0ff40c39bc087845bfb27465a0df4ea419520094bc35842ff83dd8cbe6f9b6 AS node

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
