# syntax=docker/dockerfile:1

# renovate: datasource=github-tags depName=FFmpeg/FFmpeg versioning=regex:^n(?<major>\d+)\.(?<minor>\d+)\.(?<patch>\d+)$
ARG FFMPEG_VERSION="n8.0.1"

ARG CPUS=4
ARG X264_RUNTIMES="libx264-164"
ARG X264_DEPENDENCIES="libx264-dev"
ARG X265_RUNTIMES="libx265-215 libnuma1"
ARG X265_DEPENDENCIES="libx265-dev libnuma-dev"
ARG FDK_AAC_RUNTIMES="libfdk-aac2"
ARG FDK_AAC_DEPENDENCIES="libfdk-aac-dev"
ARG WEBM_RUNTIMES="libvpx9 libvorbisenc2"
ARG WEBM_DEPENDENCIES="libvpx-dev libvorbis-dev"
ARG ASS_RUNTIMES="libass9"
ARG ASS_DEPENDENCIES="libass-dev"
ARG ARIBB24_RUNTIMES="libaribb24-0"
ARG ARIBB24_DEPENDENCIES="libaribb24-dev"
ARG VA_API_RUNTIMES="libva2 libva-drm2"
ARG VA_API_DEPENDENCIES="libva-dev"
ARG RUNTIMES="${X264_RUNTIMES} ${X265_RUNTIMES} ${FDK_AAC_RUNTIMES} ${WEBM_RUNTIMES} ${ASS_RUNTIMES} ${ARIBB24_RUNTIMES} ${VA_API_RUNTIMES}"
ARG ADDITIONAL_DEPENDENCIES="build-essential"
ARG DEPENDENCIES="${ADDITIONAL_DEPENDENCIES} ${X264_DEPENDENCIES} ${X265_DEPENDENCIES} ${FDK_AAC_DEPENDENCIES} ${WEBM_DEPENDENCIES} ${ASS_DEPENDENCIES} ${ARIBB24_DEPENDENCIES} ${VA_API_DEPENDENCIES}"

FROM --platform=$BUILDPLATFORM alpine/git:2.52.0@sha256:d86f367afb53d022acc4377741e7334bc20add161bb10234272b91b459b4b7d8 AS download
ARG FFMPEG_VERSION

RUN git clone https://github.com/FFmpeg/FFmpeg -b "${FFMPEG_VERSION}" --recursive /app

FROM --platform=$TARGETPLATFORM debian:trixie-20251229-slim@sha256:4bcb9db66237237d03b55b969271728dd3d955eaaa254b9db8a3db94550b1885 AS build
ARG CPUS
ARG DEPENDENCIES

SHELL ["/usr/bin/bash", "-euxc"]
RUN <<EOF
  echo 'deb http://deb.debian.org/debian/ stable contrib non-free non-free-firmware' | tee /etc/apt/sources.list
  apt-get update
  apt-get install -y --no-install-recommends ${DEPENDENCIES}

  apt-get clean
  rm -rf /var/lib/apt/lists/*
EOF

COPY --from=download /app/ /tmp/ffmpeg/

WORKDIR /tmp/ffmpeg
SHELL ["/usr/bin/bash", "-euxc"]
RUN <<EOF
  ./configure \
    --extra-libs="-lpthread -lm -lz" \
    --enable-small \
    --disable-shared \
    --disable-debug \
    --disable-doc \
    --disable-ffplay \
    --disable-x86asm \
    `# x264` \
    --enable-libx264 \
    --enable-gpl \
    `# x265` \
    --enable-libx265 \
    `# fdk-aac` \
    --enable-libfdk-aac \
    --enable-nonfree \
    `# WebM` \
    --enable-libvpx \
    --enable-libvorbis \
    `# ASS` \
    --enable-libass \
    `# ARIB STD-B24` \
    --enable-libaribb24 \
    --enable-version3 \
    `# VA-API` \
    --enable-vaapi
  make -j${CPUS}
  make install
EOF

FROM --platform=$TARGETPLATFORM debian:trixie-20251229-slim@sha256:4bcb9db66237237d03b55b969271728dd3d955eaaa254b9db8a3db94550b1885
ARG RUNTIMES

SHELL ["/usr/bin/bash", "-euxc"]
RUN <<EOF
  echo 'deb http://deb.debian.org/debian/ stable contrib non-free non-free-firmware' | tee /etc/apt/sources.list
  apt-get update
  apt-get install -y --no-install-recommends ${RUNTIMES}

  apt-get clean
  rm -rf /var/lib/apt/lists/*
EOF

COPY --from=build /usr/local/bin/ffmpeg /usr/local/bin/ffmpeg
COPY --from=build /usr/local/bin/ffprobe /usr/local/bin/ffprobe

WORKDIR /
ENTRYPOINT [ "/usr/local/bin/ffmpeg" ]
