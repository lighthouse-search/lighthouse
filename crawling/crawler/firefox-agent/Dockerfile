# Headless Firefox crawler — runs in an isolated, ephemeral pod.
# See kubernetes/30-lighthouse.yaml (namespace: crawler-isolated) for the
# isolation policy this image is designed to live inside: read-only root FS,
# non-root user, all capabilities dropped, no service-account token, strict
# NetworkPolicy egress.
#
# Build context: the REPO ROOT so the COPY paths can reach crawling/...
#   docker build -f crawling/crawler/firefox-agent/Dockerfile \
#     -t ghcr.io/lighthouse-search/lighthouse-crawler:latest .

# --- geckodriver fetcher (build-only; not in final image) -------------------
FROM --platform=linux/amd64 debian:bookworm-slim AS gecko
ARG GECKODRIVER_VERSION=v0.34.0
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates wget && \
    wget -qO /tmp/gecko.tar.gz \
      "https://github.com/mozilla/geckodriver/releases/download/${GECKODRIVER_VERSION}/geckodriver-${GECKODRIVER_VERSION}-linux64.tar.gz" && \
    tar -xzf /tmp/gecko.tar.gz -C /usr/local/bin && \
    chmod +x /usr/local/bin/geckodriver

# --- runtime ----------------------------------------------------------------
FROM --platform=linux/amd64 python:3.12-slim AS runtime

ENV PYTHONDONTWRITEBYTECODE=1 \
    PYTHONUNBUFFERED=1 \
    MOZ_HEADLESS=1 \
    HOME=/home/crawler

# Only what's needed at runtime — no compilers, no wget/curl, no shells beyond
# /bin/sh. Smaller attack surface for the unsandboxed pages we'll be loading.
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
      firefox-esr \
      ca-certificates \
      tini \
      libgtk-3-0 \
      libdbus-glib-1-2 \
      libasound2 && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

COPY --from=gecko /usr/local/bin/geckodriver /usr/local/bin/geckodriver

# Non-root user — UID matches runAsUser in the k8s manifest.
RUN groupadd -r -g 10001 crawler && \
    useradd  -r -u 10001 -g crawler -m -d /home/crawler -s /sbin/nologin crawler

WORKDIR /app

COPY crawling/crawler/firefox-agent/requirements.txt /app/requirements.txt
RUN pip install --no-cache-dir -r /app/requirements.txt

COPY crawling/crawler/firefox-agent/index.py       /app/index.py
COPY crawling/crawler/firefox-agent/entrypoint.sh  /app/entrypoint.sh
COPY crawling/crawler/extension                    /app/extension-src

RUN chmod +x /app/entrypoint.sh && \
    chown -R crawler:crawler /app

USER crawler
ENTRYPOINT ["/usr/bin/tini", "--", "/app/entrypoint.sh"]
