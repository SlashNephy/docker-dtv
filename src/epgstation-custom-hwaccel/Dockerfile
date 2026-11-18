# syntax=docker/dockerfile:1

ARG NVENCC_RUNTIMES="libnvidia-compute libswresample4 libavcodec60 libavformat60 libavfilter9"
ARG RUNTIMES="ca-certificates ${NVENCC_RUNTIMES}"

FROM ghcr.io/slashnephy/epgstation:20261116033322@sha256:625a0cc2fb13d446b265b1cefc4aac44c7872f9f67583434a9bcc5e9f0dc17da AS epgstation
FROM ghcr.io/slashnephy/nvencc:20261113153229@sha256:407e4885fc622f170fc4110d756c216ffc5073bb5f5ba43145a906e604ab4b68 AS nvencc
FROM ghcr.io/slashnephy/qsvencc:20261105210348@sha256:ee8293e26d8bf55b61f8604d096d54b3625603067966105f88d14a4fa5b1f4ae AS qsvencc
FROM ghcr.io/slashnephy/vceencc:20260925194753@sha256:da8892a5a81992462bc5747cf52ee5b81ba8ecefbb8cd8384d131d752e9abc80 AS vceencc
FROM node:24.22.2-bookworm-slim@sha256:01d566e8d94d8c619591d78652babfccd3c6992a3dffa9ed5ab39e11cf6c70e9 AS node

FROM ghcr.io/slashnephy/ffmpeg-ubuntu:20261116033545@sha256:33a5d3f231cde7091046a66a9a72119a7d3517c110c4f17c2383a1168a3387ba
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
