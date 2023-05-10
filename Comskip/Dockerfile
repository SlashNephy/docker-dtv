FROM alpine:3.18@sha256:02bb6f428431fbc2809c5d1b41eab5a68350194fb508869a33cb1af4444c9b11

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
