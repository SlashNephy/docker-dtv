# syntax=docker/dockerfile:1

FROM --platform=$TARGETPLATFORM ghcr.io/slashnephy/epgstation:20260423055222@sha256:b56f48f6bd0976fad171fe2e4a169e907dfeb3a112e48066cedb607819490249 AS epgstation

FROM --platform=$TARGETPLATFORM node:24.15.0-bookworm-slim@sha256:03eae3ef7e88a9de535496fb488d67e02b9d96a063a8967bae657744ecd513f2 AS node

FROM --platform=$TARGETPLATFORM ghcr.io/slashnephy/ffmpeg-debian:20260507104844@sha256:7d6cbe44c6a77711e2564bc6348c9d0227345cd53db8e1e1e12f4666ea6bc269
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
