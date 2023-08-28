# syntax=docker/dockerfile:1

# renovate: datasource=github-tags depName=FFmpeg/FFmpeg
ARG FFMPEG_VERSION="n6.0"

ARG CPUS=4
ARG X264_RUNTIMES="libx264-160"
ARG X264_DEPENDENCIES="libx264-dev"
ARG X265_RUNTIMES="libx265-192 libnuma1"
ARG X265_DEPENDENCIES="libx265-dev libnuma-dev"
ARG FDK_AAC_RUNTIMES="libfdk-aac2"
ARG FDK_AAC_DEPENDENCIES="libfdk-aac-dev"
ARG WEBM_RUNTIMES="libvpx6 libvorbisenc2"
ARG WEBM_DEPENDENCIES="libvpx-dev libvorbis-dev"
ARG ASS_RUNTIMES="libass9"
ARG ASS_DEPENDENCIES="libass-dev"
ARG ARIBB24_RUNTIMES="libaribb24-0"
ARG ARIBB24_DEPENDENCIES="libaribb24-dev"
ARG VA_API_RUNTIMES="libva2 libva-drm2"
ARG VA_API_DEPENDENCIES="libva-dev"
ARG RUNTIMES="${X264_RUNTIMES} ${X265_RUNTIMES} ${FDK_AAC_RUNTIMES} ${WEBM_RUNTIMES} ${ASS_RUNTIMES} ${ARIBB24_RUNTIMES} ${VA_API_RUNTIMES}"
ARG ADDITIONAL_DEPENDENCIES="build-essential software-properties-common"
ARG DEPENDENCIES="${ADDITIONAL_DEPENDENCIES} ${X264_DEPENDENCIES} ${X265_DEPENDENCIES} ${FDK_AAC_DEPENDENCIES} ${WEBM_DEPENDENCIES} ${ASS_DEPENDENCIES} ${ARIBB24_DEPENDENCIES} ${VA_API_DEPENDENCIES}"

FROM --platform=$BUILDPLATFORM debian:stable@sha256:9745edf9e4904fd9ff5fbb2e0c0ba0c28a74e77ca645ed751a0aadff56c439bc AS downloader

RUN <<EOF
  set -eux

  apt-get update
  apt-get install -y --no-install-recommends git ca-certificates
EOF

FROM --platform=$BUILDPLATFORM downloader AS download-ffmpeg
ARG FFMPEG_VERSION

RUN git clone https://github.com/FFmpeg/FFmpeg -b "${FFMPEG_VERSION}" --recursive

FROM --platform=$TARGETPLATFORM debian:11.7-slim@sha256:61386e11b5256efa33823cbfafd668dd651dbce810b24a8fb7b2e32fa7f65a85
ARG CPUS
ARG RUNTIMES
ARG DEPENDENCIES

# copy ffmpeg
COPY --from=download-ffmpeg /FFmpeg/ /tmp/ffmpeg/

ENV DEBIAN_FRONTEND="noninteractive"
RUN <<EOF
  set -eux

  # add non-free repository
  # TODO: software-properties-common broken on Debian 12
  apt-get update
  apt-get install -y software-properties-common
  apt-add-repository non-free
  apt-get update

  # install build dependencies and runtimes
  apt-get install -y --no-install-recommends ${RUNTIMES} ${DEPENDENCIES}

  # build ffmpeg
  cd /tmp/ffmpeg
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

  # clean up
  apt-get remove -y ${DEPENDENCIES}
  apt-get autoremove -y
  apt-get clean
  rm -rf \
    /var/lib/apt/lists/* \
    /tmp/ffmpeg
EOF

ENTRYPOINT [ "/usr/local/bin/ffmpeg" ]
