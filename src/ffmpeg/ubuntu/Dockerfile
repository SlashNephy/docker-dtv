# syntax=docker/dockerfile:1

# renovate: datasource=github-tags depName=FFmpeg/FFmpeg versioning=regex:^n(?<major>\d+)\.(?<minor>\d+)\.(?<patch>\d+)$
ARG FFMPEG_VERSION="n6.1.4"
# renovate: datasource=github-tags depName=FFmpeg/nv-codec-headers versioning=regex:^n(?<major>\d+)\.(?<minor>\d+)\.(?<patch>\d+)\.(?<build>\d+)$
ARG NV_CODEC_HEADERS_VERSION="n13.0.19.0"
# renovate: datasource=github-tags depName=GPUOpen-LibrariesAndSDKs/AMF
ARG AMF_VERSION="v1.5.0"

ARG CPUS=4
ARG X264_RUNTIMES="libx264-164"
ARG X264_DEPENDENCIES="libx264-dev"
ARG X265_RUNTIMES="libx265-199 libnuma1"
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
FROM --platform=$BUILDPLATFORM alpine/git:2.52.0@sha256:d86f367afb53d022acc4377741e7334bc20add161bb10234272b91b459b4b7d8 AS download-nv-codec-headers
ARG NV_CODEC_HEADERS_VERSION

RUN git clone https://github.com/FFmpeg/nv-codec-headers -b "${NV_CODEC_HEADERS_VERSION}" --recursive /app

# download AMF
FROM --platform=$BUILDPLATFORM alpine/git:2.52.0@sha256:d86f367afb53d022acc4377741e7334bc20add161bb10234272b91b459b4b7d8 AS download-amf
ARG AMF_VERSION

RUN git clone https://github.com/GPUOpen-LibrariesAndSDKs/AMF -b "${AMF_VERSION}" --recursive /app

# download ffmpeg
FROM --platform=$BUILDPLATFORM alpine/git:2.52.0@sha256:d86f367afb53d022acc4377741e7334bc20add161bb10234272b91b459b4b7d8 AS download-ffmpeg
ARG FFMPEG_VERSION

RUN git clone https://github.com/FFmpeg/FFmpeg -b "${FFMPEG_VERSION}" --recursive /app

FROM --platform=$TARGETPLATFORM nvidia/cuda:13.1.0-devel-ubuntu24.04@sha256:7f32ae6e575abb29f2dacf6c75fe94a262bb48dcc5196ac833ced59d9fde8107 AS build
ARG CPUS
ARG DEPENDENCIES

SHELL ["/usr/bin/bash", "-euxc"]
RUN <<EOF
  apt-get update
  apt-get install -y --no-install-recommends ${DEPENDENCIES}

  apt-get clean
  rm -rf /var/lib/apt/lists/*
EOF

COPY --from=download-nv-codec-headers /app/ /tmp/nv-codec-headers/
WORKDIR /tmp/nv-codec-headers
RUN make install

COPY --from=download-amf /app/amf/public/include/ /usr/local/include/AMF/

COPY --from=download-ffmpeg /app/ /tmp/ffmpeg/
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

FROM --platform=$TARGETPLATFORM nvidia/cuda:13.1.0-base-ubuntu24.04@sha256:ae7ce81a6af05a9a7ad3fe86115cb34db2d024e9c303959679e8889c45aeba73
ARG RUNTIMES
ENV LANG="C.UTF-8"
ENV NVIDIA_VISIBLE_DEVICES="all"
ENV NVIDIA_DRIVER_CAPABILITIES="video,compute,utility"

SHELL ["/usr/bin/bash", "-euxc"]
RUN <<EOF
  apt-get update
  apt-get install -y --no-install-recommends ${RUNTIMES}

  apt-get clean
  rm -rf /var/lib/apt/lists/*
EOF

COPY --from=build /usr/local/bin/ffmpeg /usr/local/bin/ffmpeg
COPY --from=build /usr/local/bin/ffprobe /usr/local/bin/ffprobe

ENTRYPOINT [ "/usr/local/bin/ffmpeg" ]
