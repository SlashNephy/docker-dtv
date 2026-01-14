# syntax=docker/dockerfile:1

ARG NVENCC_RUNTIMES="libnvidia-compute libswresample4 libavcodec60 libavformat60 libavfilter9"
ARG RUNTIMES="ca-certificates ${NVENCC_RUNTIMES}"

FROM ghcr.io/slashnephy/epgstation:20260113174923@sha256:2d532fd5042aa130039ff72788ea6534b9cd854e9b8d66d5dc9149a895ba610c AS epgstation
FROM ghcr.io/slashnephy/nvencc:20260103153428@sha256:7f8469db22c76c84c315d08336ac00e815cff4a62fa9310e39fdb0b4db0e910a AS nvencc
FROM ghcr.io/slashnephy/qsvencc:20260106225130@sha256:03908ff946fbec0895247508be74c5ef956b707e8cb4576d704df387696f420c AS qsvencc
FROM ghcr.io/slashnephy/vceencc:20260107024438@sha256:53de6739422c9c9ab83d1ec987613eec9172bff79234fdb307369a721b75539a AS vceencc
FROM node:24.12.0-bookworm-slim@sha256:7326fb2dbdce998edd72140946851be64ef4a643e8715e138ca467e8e9d92c99 AS node

FROM ghcr.io/slashnephy/ffmpeg-ubuntu:20260111222957@sha256:15ac7d52fd97dfe98ed2c65ade3a8346ad998a9338caf49340c18efa46e92cf7
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
