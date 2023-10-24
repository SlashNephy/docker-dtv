# syntax=docker/dockerfile:1

ARG EPGSTATION_TAG="master"
ARG NVENCC_TAG="master"
ARG QSVENCC_TAG="master"
ARG VCEENCC_TAG="master"
ARG FFMPEG_TAG="master"
ARG NODE_VERSION="20"

ARG DEPENDENCIES="ca-certificates curl gnupg"
ARG NVENCC_RUNTIMES="libcuda1-340 libswresample3 libavcodec58 libavformat58 libavfilter7"
ARG QSVENCC_RUNTIMES="intel-media-va-driver-non-free intel-opencl-icd intel-level-zero-gpu level-zero"
ARG RUNTIMES="nodejs ${NVENCC_RUNTIMES} ${QSVENCC_RUNTIMES}"

FROM ghcr.io/slashnephy/epgstation:${EPGSTATION_TAG} AS epgstation
FROM ghcr.io/slashnephy/nvencc:${NVENCC_TAG} AS nvencc
FROM ghcr.io/slashnephy/qsvencc:${QSVENCC_TAG} AS qsvencc
FROM ghcr.io/slashnephy/vceencc:${VCEENCC_TAG} AS vceencc

FROM ghcr.io/slashnephy/ffmpeg-ubuntu:${FFMPEG_TAG}
ARG NODE_VERSION
ARG DEPENDENCIES
ARG RUNTIMES

ENV DEBIAN_FRONTEND="noninteractive"
RUN <<EOF
  set -eux

  apt-get update
  apt-get install -y --no-install-recommends ${DEPENDENCIES}

  # Node
  curl -fsSL https://deb.nodesource.com/gpgkey/nodesource-repo.gpg.key | gpg --dearmor -o /usr/share/keyrings/nodesource.gpg
  echo "deb [signed-by=/usr/share/keyrings/nodesource.gpg] https://deb.nodesource.com/node_${NODE_VERSION}.x nodistro main" | tee /etc/apt/sources.list.d/nodesource.list

  # QSVEncC
  curl -fsSL https://repositories.intel.com/graphics/intel-graphics.key | gpg --dearmor -o /usr/share/keyrings/intel-graphics.gpg
  echo 'deb [arch=amd64,i386 signed-by=/usr/share/keyrings/intel-graphics.gpg] https://repositories.intel.com/graphics/ubuntu jammy arc' | tee  /etc/apt/sources.list.d/intel.gpu.jammy.list

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
COPY --from=nvencc /usr/bin/nvencc /usr/local/bin/
COPY --from=qsvencc /usr/bin/qsvencc /usr/local/bin/
COPY --from=vceencc /usr/bin/vceencc /usr/local/bin/

ENV TZ="Asia/Tokyo"

USER $USERNAME
ENTRYPOINT [ "node", "dist/index.js" ]
