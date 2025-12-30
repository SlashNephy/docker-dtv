# syntax=docker/dockerfile:1

# renovate: datasource=github-tags depName=FFmpeg/FFmpeg
ARG FFMPEG_VERSION="n6.1.4"
# renovate: datasource=github-tags depName=FFmpeg/nv-codec-headers
ARG NV_CODEC_HEADERS_VERSION="n12.1.14.0"
# renovate: datasource=github-tags depName=GPUOpen-LibrariesAndSDKs/AMF
ARG AMF_VERSION="v1.4.33"

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

# download nv-codec-headers
FROM --platform=$BUILDPLATFORM public.ecr.aws/bitnami/git:2.52.0@sha256:22ee857a29be6e98994fe50d9d9730de8ba2535caff6e4d3a1fef48dabaa3713 AS download-nv-codec-headers
ARG NV_CODEC_HEADERS_VERSION

RUN git clone https://github.com/FFmpeg/nv-codec-headers -b "${NV_CODEC_HEADERS_VERSION}" --recursive

# download AMF
FROM --platform=$BUILDPLATFORM public.ecr.aws/bitnami/git:2.52.0@sha256:22ee857a29be6e98994fe50d9d9730de8ba2535caff6e4d3a1fef48dabaa3713 AS download-amf
ARG AMF_VERSION

RUN git clone https://github.com/GPUOpen-LibrariesAndSDKs/AMF -b "${AMF_VERSION}" --recursive

# download ffmpeg
FROM --platform=$BUILDPLATFORM public.ecr.aws/bitnami/git:2.52.0@sha256:22ee857a29be6e98994fe50d9d9730de8ba2535caff6e4d3a1fef48dabaa3713 AS download-ffmpeg
ARG FFMPEG_VERSION

RUN git clone https://github.com/FFmpeg/FFmpeg -b "${FFMPEG_VERSION}" --recursive

FROM --platform=$TARGETPLATFORM nvidia/cuda:12.2.0-devel-ubuntu22.04@sha256:c4e81887e4aa9f13b1119337323cba89601319ecb282383b879c4ba50510fd17 AS build
ARG CPUS
ARG RUNTIMES
ARG DEPENDENCIES

RUN <<EOF
  set -eux

  apt-get update
  apt-get install -y --no-install-recommends curl gpg-agent software-properties-common
  curl -o - https://repositories.intel.com/graphics/intel-graphics.key | apt-key add -
  apt-add-repository 'deb [arch=amd64] https://repositories.intel.com/graphics/ubuntu focal main'

  apt-get update
  apt-get install -y --no-install-recommends ${RUNTIMES} ${DEPENDENCIES}
EOF

# copy nv-codec-headers
COPY --from=download-nv-codec-headers /nv-codec-headers/ /tmp/nv-codec-headers/
WORKDIR /tmp/nv-codec-headers
RUN make install && rm -rf /tmp/nv-codec-headers

# copy AMF
COPY --from=download-amf /AMF/amf/public/include/ /usr/local/include/AMF/

# copy ffmpeg
COPY --from=download-ffmpeg /FFmpeg/ /tmp/ffmpeg/
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

FROM --platform=$TARGETPLATFORM nvidia/cuda:12.2.0-base-ubuntu22.04@sha256:ecdf8549dd5f12609e365217a64dedde26ecda26da8f3ff3f82def6749f53051
ARG RUNTIMES
ENV LANG="C.UTF-8"
ENV NVIDIA_VISIBLE_DEVICES="all"
ENV NVIDIA_DRIVER_CAPABILITIES="video,compute,utility"

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
