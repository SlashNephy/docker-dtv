# syntax=docker/dockerfile:1@sha256:865e5dd094beca432e8c0a1d5e1c465db5f998dca4e439981029b3b81fb39ed5

ARG EPGSTATION_TAG="master@sha256:1e8c30a5105fe20a8d5497bede7c448aa7950f1cb5cb9538e320e6cb0ce77683"
ARG NVENCC_TAG="master@sha256:8de8a0982a56b6bd0a6cd02358cca67d41d3c4d594ea5f19434e58231ab77140"
ARG QSVENCC_TAG="master@sha256:40e44fcf4808b61b4976787e27cfd1d3ef0de66fd2a4aa8353c55bbea2783a3b"
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
