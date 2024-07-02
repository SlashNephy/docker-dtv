# syntax=docker/dockerfile:1

ARG EPGSTATION_TAG="master@sha256:0f57ed3470177a8669392163be23238b36e3f9d739c439a396c17f7c84d6f1cf"
ARG FFMPEG_TAG="master@sha256:32e760c1d5d6cb5254744b8a3a4c5705df955cfe8a616a933a39326a2e7c266e"
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
