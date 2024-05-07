# syntax=docker/dockerfile:1

ARG EPGSTATION_TAG="master@sha256:77cdd08af08d78cd7bb9980e2f4262c58f347cebbf65e919116a72eb3700e492"
ARG FFMPEG_TAG="master@sha256:210a629e16331232994699dfc221f6f225e1f55bc20402dc4661ffeb6574b7a8"
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
