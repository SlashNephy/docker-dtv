# syntax=docker/dockerfile:1

ARG NVENCC_RUNTIMES="libnvidia-compute libswresample4 libavcodec60 libavformat60 libavfilter9"
ARG RUNTIMES="ca-certificates ${NVENCC_RUNTIMES}"

FROM ghcr.io/slashnephy/epgstation:20261029052934@sha256:18b0469f63dc3abf2bcb6d3a1e865238cd3b3c3c12b2c230142904d9420e1b54 AS epgstation
FROM ghcr.io/slashnephy/nvencc:20261024144545@sha256:4814328406edc9f063d74919697d9ab0a960e76551ea45bf0a3a06c97b0228c7 AS nvencc
FROM ghcr.io/slashnephy/qsvencc:20261024182229@sha256:06db8f16c320fb3237dee0793ff6e4d080984639bf20d292dff4cfe59b20c981 AS qsvencc
FROM ghcr.io/slashnephy/vceencc:20260925194753@sha256:da8892a5a81992462bc5747cf52ee5b81ba8ecefbb8cd8384d131d752e9abc80 AS vceencc
FROM node:24.22.0-bookworm-slim@sha256:91acba91b842bec0bf7ea6f5e09c644c7ccad72c4996ed9616d861d1c7cd60d4 AS node

FROM ghcr.io/slashnephy/ffmpeg-ubuntu:20261024144907@sha256:89edc79a3d4e033753e2e6b2bdba741d0391b046319933bb333ea17442a46156
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
