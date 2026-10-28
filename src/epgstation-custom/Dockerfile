# syntax=docker/dockerfile:1

FROM --platform=$TARGETPLATFORM ghcr.io/slashnephy/epgstation:20261013235212@sha256:633e4ff0569d53ad6e8b11392b8690799f23500499bd24ec3a8ba423521db120 AS epgstation

FROM --platform=$TARGETPLATFORM node:26.12.0-bookworm-slim@sha256:742a0fe7320e09e0321222b68165e6ac731d30470e7ea7f98f9bfc3a6e1034fe AS node

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
