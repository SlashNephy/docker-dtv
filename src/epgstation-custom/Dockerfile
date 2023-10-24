# syntax=docker/dockerfile:1

ARG EPGSTATION_TAG="master"
ARG FFMPEG_TAG="master"
ARG NODE_VERSION="20"

ARG DEPENDENCIES="ca-certificates curl gnupg"
ARG RUNTIMES="nodejs"

FROM --platform=$TARGETPLATFORM ghcr.io/slashnephy/epgstation:${EPGSTATION_TAG} AS epgstation

FROM --platform=$TARGETPLATFORM ghcr.io/slashnephy/ffmpeg-debian:${FFMPEG_TAG}
ARG NODE_VERSION
ARG DEPENDENCIES
ARG RUNTIMES

ENV DEBIAN_FRONTEND="noninteractive"
RUN <<EOF
  set -eux

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

ARG USERNAME="app"
ARG GROUPNAME="app"
ARG UID=1000
ARG GID=1000
RUN <<EOF
  groupadd -g "${GID}" "${GROUPNAME}"
  useradd -u "${UID}" -g "${GID}" "${USERNAME}"
EOF

WORKDIR /app
COPY --from=epgstation --chown=$UID:$GID /app/ ./
COPY --from=epgstation --chown=$UID:$GID /app/client/ ./client/

ENV TZ="Asia/Tokyo"

USER $USERNAME
ENTRYPOINT [ "node", "dist/index.js" ]
