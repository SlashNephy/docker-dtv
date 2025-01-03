# syntax=docker/dockerfile:1

ARG EPGSTATION_TAG="master@sha256:fd3f3112c0a962fb180885ad34c3a9b81ff704aafd27c011b0f62fd656eaa09f"
ARG NVENCC_TAG="master@sha256:7fb49d30bcee79f4d7cd4f0a2fd977c911f776466f59c5105212781fae1984ad"
ARG QSVENCC_TAG="master@sha256:e1b38a88e68d716c842860d4a23ca40629dd28f356d9ea097d6b49522e50532e"
ARG VCEENCC_TAG="master@sha256:a2f2becdfc5482b5809a05810b6f041f6e65c53f4955ec6053026c7efdb8a8d2"
ARG FFMPEG_TAG="master@sha256:47ceba00ada9927ee3793b47d6e9940bec0c5a10b8e70b522a0aff413f646830"
ARG NODE_VERSION="20"

ARG DEPENDENCIES="curl gnupg"
ARG NVENCC_RUNTIMES="libcuda1-340 libswresample3 libavcodec58 libavformat58 libavfilter7"
ARG QSVENCC_RUNTIMES="intel-media-va-driver-non-free intel-opencl-icd intel-level-zero-gpu level-zero"
ARG RUNTIMES="nodejs ca-certificates ${NVENCC_RUNTIMES} ${QSVENCC_RUNTIMES}"

FROM ghcr.io/slashnephy/epgstation:${EPGSTATION_TAG} AS epgstation
FROM ghcr.io/slashnephy/nvencc:${NVENCC_TAG} AS nvencc
FROM ghcr.io/slashnephy/qsvencc:${QSVENCC_TAG} AS qsvencc
FROM ghcr.io/slashnephy/vceencc:${VCEENCC_TAG} AS vceencc

FROM ghcr.io/slashnephy/ffmpeg-ubuntu:${FFMPEG_TAG}
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

  # Node
  curl -fsSL https://deb.nodesource.com/gpgkey/nodesource-repo.gpg.key | gpg --dearmor -o /usr/share/keyrings/nodesource.gpg
  echo "deb [signed-by=/usr/share/keyrings/nodesource.gpg] https://deb.nodesource.com/node_${NODE_VERSION}.x nodistro main" | tee /etc/apt/sources.list.d/nodesource.list

  # QSVEncC
  curl -fsSL https://repositories.intel.com/graphics/intel-graphics.key | gpg --dearmor -o /usr/share/keyrings/intel-graphics.gpg
  echo 'deb [arch=amd64,i386 signed-by=/usr/share/keyrings/intel-graphics.gpg] https://repositories.intel.com/graphics/ubuntu jammy arc' | tee  /etc/apt/sources.list.d/intel.gpu.jammy.list

  apt-get update
  apt-get install -y --no-install-recommends ${RUNTIMES}

  apt-get remove -y ${DEPENDENCIES}
  apt-get clean
  rm -rf /var/lib/apt/lists/*
EOF

COPY --from=epgstation --chown=$UID:$GID /app/ ./
COPY --from=epgstation --chown=$UID:$GID /app/client/ ./client/
COPY --from=nvencc /usr/bin/nvencc /usr/local/bin/
COPY --from=qsvencc /usr/bin/qsvencc /usr/local/bin/
COPY --from=vceencc /usr/bin/vceencc /usr/local/bin/

USER $USERNAME
ENTRYPOINT [ "node", "dist/index.js" ]
