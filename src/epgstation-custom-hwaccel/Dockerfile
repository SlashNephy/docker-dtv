# syntax=docker/dockerfile:1

ARG NVENCC_RUNTIMES="libnvidia-compute libswresample4 libavcodec60 libavformat60 libavfilter9"
ARG RUNTIMES="ca-certificates ${NVENCC_RUNTIMES}"

FROM ghcr.io/slashnephy/epgstation:20260715021633@sha256:e06940fee925a5ed56161044841db5735405a4d30533167e93f507ceb476cf3c AS epgstation
FROM ghcr.io/slashnephy/nvencc:20260625100104@sha256:c2165cb31647d9d03a139af42afc1831f57ee86a746c5c841dc7d02b01fd03bb AS nvencc
FROM ghcr.io/slashnephy/qsvencc:20260709221435@sha256:85935a794d14a9a97727e4c22497886b6e6867b65db72cbfaf7e8d3f20f50186 AS qsvencc
FROM ghcr.io/slashnephy/vceencc:20260709175827@sha256:4d7d8b5da8bab727d1fe58c4c36e35831add5ae3e51bb64c18e250b40693bf3a AS vceencc
FROM node:24.18.0-bookworm-slim@sha256:6f7b03f7c2c8e2e784dcf9295400527b9b1270fd37b7e9a7285cf83b6951452d AS node

FROM ghcr.io/slashnephy/ffmpeg-ubuntu:20260722160527@sha256:1f1fd5dccb1010033be44c06c6d4f228bfd61afaaee259c30abaddc589a3b4e6
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
