# syntax=docker/dockerfile:1

ARG NVENCC_RUNTIMES="libnvidia-compute libswresample4 libavcodec60 libavformat60 libavfilter9"
ARG RUNTIMES="ca-certificates ${NVENCC_RUNTIMES}"

FROM ghcr.io/slashnephy/epgstation:20260125214518@sha256:67d60ec9df6917a9c29c8da87b1892d920f8deb5d2f1685076b82f95619bee16 AS epgstation
FROM ghcr.io/slashnephy/nvencc:20260119021414@sha256:1042c10d4532cd9b2c113687d0e59831ed12b55e1585751c1bfca3321d7e548d AS nvencc
FROM ghcr.io/slashnephy/qsvencc:20260106225130@sha256:03908ff946fbec0895247508be74c5ef956b707e8cb4576d704df387696f420c AS qsvencc
FROM ghcr.io/slashnephy/vceencc:20260122113829@sha256:6065111b7777fd26fa00d2091567db4eb42c48f2a4c85c8e3f3b5d807212568f AS vceencc
FROM node:24.13.0-bookworm-slim@sha256:bf22df20270b654c4e9da59d8d4a3516cce6ba2852e159b27288d645b7a7eedc AS node

FROM ghcr.io/slashnephy/ffmpeg-ubuntu:20260126030739@sha256:2da408ac1b8024e3be94319ba31e8541b9dea30ac33bbac7ce6cd263bfd48970
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
