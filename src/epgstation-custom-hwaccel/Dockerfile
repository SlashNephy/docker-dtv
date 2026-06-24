# syntax=docker/dockerfile:1

ARG NVENCC_RUNTIMES="libnvidia-compute libswresample4 libavcodec60 libavformat60 libavfilter9"
ARG RUNTIMES="ca-certificates ${NVENCC_RUNTIMES}"

FROM ghcr.io/slashnephy/epgstation:20260622155551@sha256:09694500f332f67bc190c168c14932237d1398d8e63de36534fe15b2cd3f0b67 AS epgstation
FROM ghcr.io/slashnephy/nvencc:20260424054635@sha256:53bd31b824ba471dccf9015f4fc9a6eb02150a483dcd4e384cda5902287c72c0 AS nvencc
FROM ghcr.io/slashnephy/qsvencc:20260424224934@sha256:3e4f09b5d9ad940993dc68a7453154c8885648533f2ea5c1fd95d672dce50436 AS qsvencc
FROM ghcr.io/slashnephy/vceencc:20260624070533@sha256:eaf67b332bd525325e2dd8a15f2b741460676ddf6c72600b4ba6f03b09012b42 AS vceencc
FROM node:24.16.0-bookworm-slim@sha256:2c87ef9bd3c6a3bd4b472b4bec2ce9d16354b0c574f736c476489d09f560a203 AS node

FROM ghcr.io/slashnephy/ffmpeg-ubuntu:20260624185759@sha256:70b1f4839489bbb11e50d9dca8511690fc0bf36176c1b585eb720c731217da90
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
