FROM alpine:3.18@sha256:82d1e9d7ed48a7523bdebc18cf6290bdb97b82302a8a9c27d4fe885949ea94d1

RUN apk --no-cache add python3 python3-dev bash build-base linux-headers \
    && pip3 install --upgrade pip \
    && pip3 install aiohttp psutil discord.py

COPY app.py /app.py
WORKDIR /

ENTRYPOINT ["python3", "/app.py"]
