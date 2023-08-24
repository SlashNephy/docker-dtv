# syntax=docker/dockerfile:1
ARG VERSION="snapshot"
ARG NV_CODEC_HEADERS_REF="master"
ARG AMF_REF="master"
ARG CPUS=4
ARG X264_RUNTIMES="libx264-163"
ARG X264_DEPENDENCIES="libx264-dev"
ARG X265_RUNTIMES="libx265-199 libnuma1"
ARG X265_DEPENDENCIES="libx265-dev libnuma-dev"
ARG FDK_AAC_RUNTIMES="libfdk-aac2"
ARG FDK_AAC_DEPENDENCIES="libfdk-aac-dev"
ARG WEBM_RUNTIMES="libvpx7 libvorbisenc2"
ARG WEBM_DEPENDENCIES="libvpx-dev libvorbis-dev"
ARG ASS_RUNTIMES="libass9"
ARG ASS_DEPENDENCIES="libass-dev"
ARG ARIBB24_RUNTIMES="libaribb24-0"
ARG ARIBB24_DEPENDENCIES="libaribb24-dev"
ARG VA_API_RUNTIMES="libva2 libva-drm2"
ARG VA_API_DEPENDENCIES="libva-dev"
ARG NVIDIA_NVENC_RUNTIMES=""
ARG NVIDIA_NVENC_DEPENDENCIES="nasm yasm"
ARG INTEL_QSV_RUNTIMES="libmfx1 libva-x11-2"
ARG INTEL_QSV_DEPENDENCIES="libmfx-dev"
# TODO: AMD AMF not working in this image currently. Add some runtimes.
ARG AMD_AMF_RUNTIMES=""
ARG AMD_AMF_DEPENDENCIES=""
ARG RUNTIMES="${X264_RUNTIMES} ${X265_RUNTIMES} ${FDK_AAC_RUNTIMES} ${WEBM_RUNTIMES} ${ASS_RUNTIMES} ${ARIBB24_RUNTIMES} ${VA_API_RUNTIMES} ${NVIDIA_NVENC_RUNTIMES} ${INTEL_QSV_RUNTIMES} ${AMD_AMF_RUNTIMES}"
ARG ADDITIONAL_DEPENDENCIES="build-essential"
ARG DEPENDENCIES="${ADDITIONAL_DEPENDENCIES} ${X264_DEPENDENCIES} ${X265_DEPENDENCIES} ${FDK_AAC_DEPENDENCIES} ${WEBM_DEPENDENCIES} ${ASS_DEPENDENCIES} ${ARIBB24_DEPENDENCIES} ${VA_API_DEPENDENCIES} ${NVIDIA_NVENC_DEPENDENCIES} ${INTEL_QSV_DEPENDENCIES} ${AMD_AMF_DEPENDENCIES}"

FROM debian:stable@sha256:9745edf9e4904fd9ff5fbb2e0c0ba0c28a74e77ca645ed751a0aadff56c439bc AS downloader

RUN <<EOF
  set -eux

  apt update
  apt-get install -y --no-install-recommends git curl tar bzip2 ca-certificates
EOF

# download nv-codec-headers
FROM downloader AS download-nv-codec-headers
ARG NV_CODEC_HEADERS_REF

RUN git clone https://git.videolan.org/git/ffmpeg/nv-codec-headers.git -b "${NV_CODEC_HEADERS_REF}" --recursive

# download AMF
FROM downloader AS download-amf
ARG AMF_REF

RUN git clone https://github.com/GPUOpen-LibrariesAndSDKs/AMF -b "${AMF_REF}" --recursive

# download ffmpeg
FROM downloader AS download-ffmpeg
ARG VERSION

RUN <<EOF
  set -eux

  mkdir -p /ffmpeg
  curl "https://ffmpeg.org/releases/ffmpeg-${VERSION}.tar.bz2" | tar jx --strip-components=1 -C /ffmpeg
EOF

FROM nvidia/cuda:12.2.0-devel-ubuntu22.04@sha256:7e81c70fb55bfc54e6711700eb721bb94159a3cd546620b37210d12c17914fa6 AS build
ARG CPUS
ARG RUNTIMES
ARG DEPENDENCIES

# copy nv-codec-headers
COPY --from=download-nv-codec-headers /nv-codec-headers/ /tmp/nv-codec-headers/
RUN <<EOF
  set -eux

  cd /tmp/nv-codec-headers
  make install
  rm -rf /tmp/nv-codec-headers
EOF

# copy AMF
COPY --from=download-amf /AMF/amf/public/include/ /usr/local/include/AMF/

# copy ffmpeg
COPY --from=download-ffmpeg /ffmpeg/ /tmp/ffmpeg/

ENV DEBIAN_FRONTEND="noninteractive"
RUN <<EOF
  set -eux

  # install build dependencies and runtimes
  apt-get update
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
    --enable-vaapi \
    `# NVIDIA NVEnc` \
    --enable-cuda \
    --enable-nvenc \
    --extra-cflags="-I/usr/local/cuda/include" \
    --extra-ldflags="-L/usr/local/cuda/lib64" \
    `# Intel QSV` \
    --enable-libmfx \
    `# AMD AMF` \
    --enable-amf
  make -j${CPUS}
  make install
EOF

FROM nvidia/cuda:12.2.0-base-ubuntu22.04@sha256:1069ccd2910506f68e1d7c0907a32aaa877b8038d1aa24cb7ffb2d2a85d725c7
ARG RUNTIMES
ENV LANG="C.UTF-8"
ENV NVIDIA_VISIBLE_DEVICES="all"
ENV NVIDIA_DRIVER_CAPABILITIES="video,compute,utility"

ENV DEBIAN_FRONTEND="noninteractive"
RUN <<EOF
  set -eux

  # install runtimes
  apt-get update
  apt-get install -y --no-install-recommends ${RUNTIMES}

  # clean up
  apt-get clean
  rm -rf /var/lib/apt/lists/*
EOF

COPY --from=build /usr/local/bin/ffmpeg /usr/local/bin/ffmpeg
COPY --from=build /usr/local/bin/ffprobe /usr/local/bin/ffprobe

ENTRYPOINT [ "/usr/local/bin/ffmpeg" ]
