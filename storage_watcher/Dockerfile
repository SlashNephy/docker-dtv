FROM alpine:3.18@sha256:7144f7bab3d4c2648d7e59409f15ec52a18006a128c733fcff20d3a4a54ba44a

RUN apk --no-cache add python3 python3-dev bash build-base linux-headers \
    && pip3 install --upgrade pip \
    && pip3 install aiohttp psutil discord.py

COPY app.py /app.py
WORKDIR /

ENTRYPOINT ["python3", "/app.py"]
