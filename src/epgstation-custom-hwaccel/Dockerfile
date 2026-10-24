# syntax=docker/dockerfile:1

ARG NVENCC_RUNTIMES="libnvidia-compute libswresample4 libavcodec60 libavformat60 libavfilter9"
ARG RUNTIMES="ca-certificates ${NVENCC_RUNTIMES}"

FROM ghcr.io/slashnephy/epgstation:20261013235212@sha256:633e4ff0569d53ad6e8b11392b8690799f23500499bd24ec3a8ba423521db120 AS epgstation
FROM ghcr.io/slashnephy/nvencc:20261024144545@sha256:4814328406edc9f063d74919697d9ab0a960e76551ea45bf0a3a06c97b0228c7 AS nvencc
FROM ghcr.io/slashnephy/qsvencc:20261024182229@sha256:06db8f16c320fb3237dee0793ff6e4d080984639bf20d292dff4cfe59b20c981 AS qsvencc
FROM ghcr.io/slashnephy/vceencc:20260925194753@sha256:da8892a5a81992462bc5747cf52ee5b81ba8ecefbb8cd8384d131d752e9abc80 AS vceencc
FROM node:24.21.0-bookworm-slim@sha256:d6aa754f16b3197301076f047b5def2f02ea1dbbc2ca920407d46d7ec7f87b20 AS node

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
