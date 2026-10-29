# syntax=docker/dockerfile:1

FROM --platform=$TARGETPLATFORM ghcr.io/slashnephy/epgstation:20261029052934@sha256:18b0469f63dc3abf2bcb6d3a1e865238cd3b3c3c12b2c230142904d9420e1b54 AS epgstation

FROM --platform=$TARGETPLATFORM node:24.22.0-bookworm-slim@sha256:91acba91b842bec0bf7ea6f5e09c644c7ccad72c4996ed9616d861d1c7cd60d4 AS node

FROM --platform=$TARGETPLATFORM ghcr.io/slashnephy/ffmpeg-debian:20260929104535@sha256:96388cc44d9bf40155ea61b0d8ab36e0cb64736564331d6af0e4e49a282b1d29
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
