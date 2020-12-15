FROM alpine:3.11

RUN apk --no-cache add python3 python3-dev bash build-base linux-headers \
    && pip3 install --upgrade pip \
    && pip3 install aiohttp psutil discord.py

COPY app.py /app.py
WORKDIR /

ENTRYPOINT ["python3", "/app.py"]
