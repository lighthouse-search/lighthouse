#!/bin/bash
cd /rover-server/guard && /rover-server/guard/guard-server &
P1=$!
cd /rover-server && echo "lighthouse!" && ./target/release/lighthouse-server &
P2=$!
cd /rover-server/frontend && echo "lighthouse frontend!" && npm run start &
P3=$!
nginx -c /rover-server/nginx/config/split.conf &
P4=$!
wait $P1 $P2 $P3 $P4