# syntax=docker/dockerfile:1

# renovate: datasource=github-tags depName=FFmpeg/FFmpeg
ARG FFMPEG_VERSION="n6.0"

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

FROM --platform=$BUILDPLATFORM public.ecr.aws/bitnami/git:2.44.0@sha256:b91c3ac4915cbf6ba813175a0387bca69a70b52e1b57a22e212322036c47da23 AS download
ARG FFMPEG_VERSION

RUN git clone https://github.com/FFmpeg/FFmpeg -b "${FFMPEG_VERSION}" --recursive

FROM --platform=$TARGETPLATFORM public.ecr.aws/docker/library/alpine:3.18.11@sha256:dd60c75fba961ecc5e918961c713f3c42dd5665171c58f9b2ef5aafe081ad5a0
ARG CPUS
ARG RUNTIMES
ARG DEPENDENCIES

RUN <<EOF
  set -eux

  echo https://dl-cdn.alpinelinux.org/alpine/edge/community >> /etc/apk/repositories
  apk add --no-cache --virtual .build-dependencies ${DEPENDENCIES}
  apk add --no-cache ${RUNTIMES}
EOF

COPY --from=download /FFmpeg/ /tmp/ffmpeg/

WORKDIR /tmp/ffmpeg
RUN <<EOF
  set -eux

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
