# syntax=docker/dockerfile:1

ARG NVENCC_RUNTIMES="libnvidia-compute libswresample4 libavcodec60 libavformat60 libavfilter9"
ARG RUNTIMES="ca-certificates ${NVENCC_RUNTIMES}"

FROM ghcr.io/slashnephy/epgstation:20261002113551@sha256:982ad20712a202434e9c23181898166044d2d302017cbaaef3fa785eaf16be3b AS epgstation
FROM ghcr.io/slashnephy/nvencc:20260927190503@sha256:e63b1e9ac0602ffa80a5d67c50e434072c82a546d893179bcf9d16cc50f828af AS nvencc
FROM ghcr.io/slashnephy/qsvencc:20260926190940@sha256:c3b4c7b7bd3fd7885310911f4a916f74450b438eae9f35989cd67ecf6748d64d AS qsvencc
FROM ghcr.io/slashnephy/vceencc:20260925194753@sha256:da8892a5a81992462bc5747cf52ee5b81ba8ecefbb8cd8384d131d752e9abc80 AS vceencc
FROM node:24.21.0-bookworm-slim@sha256:0e0ff40c39bc087845bfb27465a0df4ea419520094bc35842ff83dd8cbe6f9b6 AS node

FROM ghcr.io/slashnephy/ffmpeg-ubuntu:20261007184034@sha256:52818abeb24276f5d95dfe27655267976104a1037cd98f5800ef34a2ee5f14ab
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
