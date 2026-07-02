# syntax=docker/dockerfile:1

# renovate: datasource=github-tags depName=FFmpeg/FFmpeg versioning=regex:^n(?<major>\d+)\.(?<minor>\d+)\.(?<patch>\d+)$
ARG FFMPEG_VERSION="n8.1.2"

ARG CPUS=4
ARG X264_RUNTIMES="x264-libs"
ARG X264_DEPENDENCIES="x264-dev"
ARG X265_RUNTIMES="x265-libs numactl"
ARG X265_DEPENDENCIES="x265-dev numactl-dev"
ARG FDK_AAC_RUNTIMES="fdk-aac"
ARG FDK_AAC_DEPENDENCIES="fdk-aac-dev"
ARG WEBM_RUNTIMES="libvpx libvorbis"
ARG WEBM_DEPENDENCIES="libvpx-dev libvorbis-dev"
ARG ASS_RUNTIMES="libass"
ARG ASS_DEPENDENCIES="libass-dev"
ARG VA_API_RUNTIMES="libva"
ARG VA_API_DEPENDENCIES="libva-dev"
ARG RUNTIMES="${X264_RUNTIMES} ${X265_RUNTIMES} ${FDK_AAC_RUNTIMES} ${WEBM_RUNTIMES} ${ASS_RUNTIMES} ${VA_API_RUNTIMES}"
ARG ADDITIONAL_DEPENDENCIES="build-base"
ARG DEPENDENCIES="${ADDITIONAL_DEPENDENCIES} ${X264_DEPENDENCIES} ${X265_DEPENDENCIES} ${FDK_AAC_DEPENDENCIES} ${WEBM_DEPENDENCIES} ${ASS_DEPENDENCIES} ${VA_API_DEPENDENCIES}"

FROM --platform=$BUILDPLATFORM alpine/git:v2.54.0@sha256:e043be20669db13cbcfb6190192babee4cf2dca98709bb0c2d08ca2d35a0a06a AS download
ARG FFMPEG_VERSION

RUN git clone https://github.com/FFmpeg/FFmpeg -b "${FFMPEG_VERSION}" --recursive /app

FROM --platform=$TARGETPLATFORM alpine:3.24.1@sha256:28bd5fe8b56d1bd048e5babf5b10710ebe0bae67db86916198a6eec434943f8b
ARG CPUS
ARG RUNTIMES
ARG DEPENDENCIES

SHELL ["/bin/ash", "-euxc"]
RUN <<EOF
  echo https://dl-cdn.alpinelinux.org/alpine/edge/community >> /etc/apk/repositories
  apk add --no-cache --virtual .build-dependencies ${DEPENDENCIES}
  apk add --no-cache ${RUNTIMES}
EOF

COPY --from=download /app/ /tmp/ffmpeg/

WORKDIR /tmp/ffmpeg
SHELL ["/bin/ash", "-euxc"]
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
    `# VA-API` \
    --enable-vaapi
  make -j${CPUS}
  make install

  apk del --purge .build-dependencies
  rm -rf /tmp/ffmpeg
EOF

WORKDIR /
ENTRYPOINT [ "/usr/local/bin/ffmpeg" ]
