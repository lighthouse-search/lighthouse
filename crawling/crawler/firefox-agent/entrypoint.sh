#!/bin/sh
# Copy the extension into a writable location and template the crawl-back URL.
# The container's root FS is mounted read-only at runtime (see k8s manifest),
# so any mutation has to happen on an emptyDir-backed mount.
set -eu

EXT_SRC=/app/extension-src
EXT_DST=${EXTENSION_DIR:-/tmp/extension}
TARGET_HOST=${lighthouse_host:-http://lighthouse-service.production.svc.cluster.local}

rm -rf "$EXT_DST"
mkdir -p "$EXT_DST"
cp -r "$EXT_SRC"/. "$EXT_DST"/

# borderify.js ships with a localhost URL for dev; rewrite to the configured
# in-cluster service so the content script can POST discovered URLs back to
# /api/native-v1/crawler/index.
if [ -f "$EXT_DST/borderify.js" ]; then
  # Use a delimiter that won't appear in URLs.
  sed -i "s|http://127.0.0.1:4459|${TARGET_HOST}|g" "$EXT_DST/borderify.js"
fi

export EXTENSION_DIR="$EXT_DST"
exec python /app/index.py
