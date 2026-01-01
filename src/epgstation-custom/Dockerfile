# syntax=docker/dockerfile:1

FROM --platform=$TARGETPLATFORM ghcr.io/slashnephy/epgstation:20251231162821@sha256:2ffff01cf0b13490b9a0813cb1afb206e8bbf90fc0292fb4ace804c22b13b18c AS epgstation

FROM --platform=$TARGETPLATFORM node:24.12.0-bookworm-slim@sha256:b83af04d005d8e3716f542469a28ad2947ba382f6b4a76ddca0827a21446a540 AS node

FROM --platform=$TARGETPLATFORM ghcr.io/slashnephy/ffmpeg-debian:20251231163719@sha256:ffece2f79d00befcfd6998afd1ad4a9d63a193021275842f06a54ae6066f5c6d
WORKDIR /app
ARG USERNAME="app"
ARG GROUPNAME="app"
ARG UID=1000
ARG GID=1000
ENV TZ="Asia/Tokyo"

RUN <<EOF
  set -eux

  groupadd -g "${GID}" "${GROUPNAME}"
  useradd -l -u "${UID}" -g "${GID}" "${USERNAME}"
EOF

COPY --from=epgstation --chown=$UID:$GID /app/ ./
COPY --from=epgstation --chown=$UID:$GID /app/client/ ./client/
COPY --from=node /usr/local/ /usr/local/

USER $USERNAME
ENTRYPOINT [ "node", "dist/index.js" ]
