# syntax=docker/dockerfile:1

ARG NVENCC_RUNTIMES="libnvidia-compute libswresample4 libavcodec60 libavformat60 libavfilter9"
ARG RUNTIMES="ca-certificates ${NVENCC_RUNTIMES}"

FROM ghcr.io/slashnephy/epgstation:20260208175110@sha256:b445505355e07343af8cca479aa0482e0e551013ef7de0e4b7bb5843c3712a5c AS epgstation
FROM ghcr.io/slashnephy/nvencc:20260126030508@sha256:51f6780d557a7888ce8c5e7c62137e05e35148af758344f3174653633121734c AS nvencc
FROM ghcr.io/slashnephy/qsvencc:20260106225130@sha256:03908ff946fbec0895247508be74c5ef956b707e8cb4576d704df387696f420c AS qsvencc
FROM ghcr.io/slashnephy/vceencc:20260122113829@sha256:6065111b7777fd26fa00d2091567db4eb42c48f2a4c85c8e3f3b5d807212568f AS vceencc
FROM node:24.13.1-bookworm-slim@sha256:a81a03dd965b4052269a57fac857004022b522a4bf06e7a739e25e18bce45af2 AS node

FROM ghcr.io/slashnephy/ffmpeg-ubuntu:20260208175341@sha256:c8dc70c05683a3499a55f3de16a62343d84fe80dfb253ea0f9c1f85a4447be24
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
