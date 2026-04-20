# syntax=docker/dockerfile:1

FROM --platform=$TARGETPLATFORM ghcr.io/slashnephy/epgstation:20260420014950@sha256:1bbc90387d9fa1932adc699f1a08bad2a79355e25dec61db956c26e36b104e94 AS epgstation

FROM --platform=$TARGETPLATFORM node:24.15.0-bookworm-slim@sha256:879b21aec4a1ad820c27ccd565e7c7ed955f24b92e6694556154f251e4bdb240 AS node

FROM --platform=$TARGETPLATFORM ghcr.io/slashnephy/ffmpeg-debian:20260223013927@sha256:dfdc5b3ad84a1ea24bf387f4e1302ca28378b330e2f9131e808bc25e2a6610f6
WORKDIR /app
ARG USERNAME="app"
ARG GROUPNAME="app"
ARG UID=1000
ARG GID=1000
ENV TZ="Asia/Tokyo"

SHELL ["/usr/bin/bash", "-euxc"]
RUN <<EOF
  groupadd -g "${GID}" "${GROUPNAME}"
  useradd -l -u "${UID}" -g "${GID}" "${USERNAME}"
EOF

COPY --from=epgstation --chown=$UID:$GID /app/ ./
COPY --from=epgstation --chown=$UID:$GID /app/client/ ./client/
COPY --from=node /usr/local/ /usr/local/

USER $USERNAME
ENTRYPOINT [ "node", "dist/index.js" ]
