#!/bin/bash
cd /rover-server/guard && /rover-server/guard/guard-server &
P1=$!
cd /rover-server && echo "rover!" && ./target/release/lighthouse-server &
P2=$!
nginx -c /rover-server/nginx/config/split.conf &
P3=$!
wait $P1 $P2 $P3