FROM alpine:3.18@sha256:7144f7bab3d4c2648d7e59409f15ec52a18006a128c733fcff20d3a4a54ba44a

RUN apk --no-cache add python python3 python3-dev ffmpeg tzdata bash \
    && apk --no-cache add --virtual=builddeps \
        autoconf \
        automake \
        libtool \
        git \
        ffmpeg-dev \
        wget \
        tar \
        build-base \
    && pip3 install --upgrade pip \
    && pip3 install aiohttp \
    \
    && cd /tmp \
    && wget http://prdownloads.sourceforge.net/argtable/argtable2-13.tar.gz \
    && tar xzf argtable2-13.tar.gz \
    && cd argtable2-13 \
    && ./configure \
    && make \
    && make install \
    \
    && cd /tmp \
    && git clone git://github.com/erikkaashoek/Comskip.git \
    && cd Comskip \
    && ./autogen.sh \
    && ./configure \
    && make \
    && make install \
    \
    && apk del builddeps \
    && rm -rf /var/cache/apk/* \
        /tmp/* \
        /tmp/.[!.]*

COPY app.py /app.py
WORKDIR /

ENTRYPOINT ["python3", "/app.py"]
