#!/bin/bash
cd /rover-server/guard && /rover-server/guard/guard-server &
P1=$!
# The axum server binds the container's exposed port (80) directly, so no
# reverse proxy is needed in front of it.
cd /rover-server && echo "lighthouse!" && lighthouse_port=80 ./target/release/lighthouse-server &
P2=$!
cd /rover-server/frontend/lighthouse && echo "lighthouse frontend!" && npm run start &
P3=$!
wait $P1 $P2 $P3
