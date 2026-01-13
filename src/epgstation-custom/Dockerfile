# syntax=docker/dockerfile:1

FROM --platform=$TARGETPLATFORM ghcr.io/slashnephy/epgstation:20260113174923@sha256:2d532fd5042aa130039ff72788ea6534b9cd854e9b8d66d5dc9149a895ba610c AS epgstation

FROM --platform=$TARGETPLATFORM node:24.12.0-bookworm-slim@sha256:732887b2c52d7251840a19d203afc68a61532b1ff8b5bf3b76efd49ded39e908 AS node

FROM --platform=$TARGETPLATFORM ghcr.io/slashnephy/ffmpeg-debian:20260111222730@sha256:e247b9f96a3f67869d2b4071fe4361cbedeb8ade984e7d66dd379a33ce17f2d6
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
