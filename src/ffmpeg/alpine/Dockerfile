# syntax=docker/dockerfile:1
ARG FFMPEG_VERSION="6.0"
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

FROM --platform=$TARGETPLATFORM debian:stable@sha256:9745edf9e4904fd9ff5fbb2e0c0ba0c28a74e77ca645ed751a0aadff56c439bc AS download-ffmpeg
ARG FFMPEG_VERSION

RUN <<EOF
  set -eux

  apt update
  apt-get install -y --no-install-recommends curl tar bzip2 ca-certificates

  mkdir -p /ffmpeg
  curl "https://ffmpeg.org/releases/ffmpeg-${FFMPEG_VERSION}.tar.bz2" | tar jx --strip-components=1 -C /ffmpeg
EOF

FROM --platform=$TARGETPLATFORM alpine:3.18.3@sha256:7144f7bab3d4c2648d7e59409f15ec52a18006a128c733fcff20d3a4a54ba44a
ARG CPUS
ARG RUNTIMES
ARG DEPENDENCIES

# copy ffmpeg
COPY --from=download-ffmpeg /ffmpeg/ /tmp/ffmpeg/

RUN <<EOF
  set -eux

  # add community repository
  echo https://dl-cdn.alpinelinux.org/alpine/edge/community >> /etc/apk/repositories

  # install build dependencies
  apk add --no-cache --virtual .build-dependencies ${DEPENDENCIES}

  # install runtime
  apk add --no-cache ${RUNTIMES}

  # build ffmpeg
  cd /tmp/ffmpeg/
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

  # clean up
  apk del --purge .build-dependencies
  rm -rf /tmp/ffmpeg
EOF

ENTRYPOINT [ "/usr/local/bin/ffmpeg" ]
