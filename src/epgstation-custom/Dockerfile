# syntax=docker/dockerfile:1

ARG EPGSTATION_TAG="master@sha256:2ffff01cf0b13490b9a0813cb1afb206e8bbf90fc0292fb4ace804c22b13b18c"
ARG FFMPEG_TAG="master@sha256:467132d11bc7cc09fca6f67b3ff543cbd415c5b0cf1c5e216119091d1d9cc069"
ARG NODE_VERSION="20"

ARG DEPENDENCIES="ca-certificates curl gnupg"
ARG RUNTIMES="nodejs"

FROM --platform=$TARGETPLATFORM ghcr.io/slashnephy/epgstation:${EPGSTATION_TAG} AS epgstation

FROM --platform=$TARGETPLATFORM ghcr.io/slashnephy/ffmpeg-debian:${FFMPEG_TAG}
WORKDIR /app
ARG NODE_VERSION
ARG DEPENDENCIES
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
  apt-get install -y --no-install-recommends ${DEPENDENCIES}

  curl -fsSL https://deb.nodesource.com/gpgkey/nodesource-repo.gpg.key | gpg --dearmor -o /usr/share/keyrings/nodesource.gpg
  echo "deb [signed-by=/usr/share/keyrings/nodesource.gpg] https://deb.nodesource.com/node_${NODE_VERSION}.x nodistro main" | tee /etc/apt/sources.list.d/nodesource.list

  apt-get update
  apt-get install -y --no-install-recommends ${RUNTIMES}

  apt-get remove -y ${DEPENDENCIES}
  apt-get autoremove -y
  apt-get clean
  rm -rf /var/lib/apt/lists/*
EOF

COPY --from=epgstation --chown=$UID:$GID /app/ ./
COPY --from=epgstation --chown=$UID:$GID /app/client/ ./client/

USER $USERNAME
ENTRYPOINT [ "node", "dist/index.js" ]
