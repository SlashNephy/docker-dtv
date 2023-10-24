# syntax=docker/dockerfile:1

ARG EPGSTATION_TAG="master@sha256:fd942190343e8ea2adbd9b13bb34a168a7790ccc3fb2868abc829426e1c69f54"
ARG FFMPEG_TAG="master@sha256:6d55cc571a43a58600e347c5b7502a00945db4ca65093615e1ba0beb623e8ff4"
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
