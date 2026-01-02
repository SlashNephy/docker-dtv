# syntax=docker/dockerfile:1

ARG NVENCC_RUNTIMES="libcuda1-340 libswresample3 libavcodec58 libavformat58 libavfilter7"
ARG RUNTIMES="ca-certificates ${NVENCC_RUNTIMES}"

FROM ghcr.io/slashnephy/epgstation:20260101191808@sha256:0ccb684756e2350727271bfd14cfdabe29544c97355f98999d1178de3d9a5fa5 AS epgstation
FROM ghcr.io/slashnephy/nvencc:20251231163544@sha256:197e9ed7200ff91ad4f0eeee976cede81ef19f9ac4327c05bf2ccf27b674e6df AS nvencc
FROM ghcr.io/slashnephy/qsvencc:20251231164915@sha256:0a93400effef7d73d4e6f1a14e135b318d48b3cf29bb4b8902c63c9f862a334f AS qsvencc
FROM ghcr.io/slashnephy/vceencc:20251231175402@sha256:eaccea9f77bdf250bf33c4f301916c8adcd48cdf833ac9c059cf3e389ad4db63 AS vceencc
FROM node:24.12.0-bookworm-slim@sha256:b83af04d005d8e3716f542469a28ad2947ba382f6b4a76ddca0827a21446a540 AS node

FROM ghcr.io/slashnephy/ffmpeg-ubuntu:20260101155658@sha256:9faad15dffdb215a549d8c2c122ca4951ba796fd107cbc8b13081f1161aa48d1
WORKDIR /app
ARG RUNTIMES
ARG USERNAME="app"
ARG GROUPNAME="app"
ARG UID=1000
ARG GID=1000
ENV TZ="Asia/Tokyo"

RUN <<EOF
  set -eux

  groupadd -g "${GID}" "${GROUPNAME}"
  useradd -l -u "${UID}" -g "${GID}" "${USERNAME}"

  apt-get update
  apt-get install -y --no-install-recommends ${RUNTIMES}

  apt-get clean
  rm -rf /var/lib/apt/lists/*
EOF

COPY --from=epgstation --chown=$UID:$GID /app/ ./
COPY --from=epgstation --chown=$UID:$GID /app/client/ ./client/
COPY --from=nvencc /usr/bin/nvencc /usr/local/bin/
COPY --from=qsvencc /usr/bin/qsvencc /usr/local/bin/
COPY --from=vceencc /usr/bin/vceencc /usr/local/bin/
COPY --from=node /usr/local/ /usr/local/

USER $USERNAME
ENTRYPOINT [ "node", "dist/index.js" ]
