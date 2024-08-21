#!/bin/bash
cd /mindmap-server/guard && /mindmap-server/guard/guard-server &
P1=$!
cd /mindmap-server && echo "mindmap!" && ./target/release/mindmap-server &
P2=$!
nginx -c /mindmap-server/nginx/config/split.conf &
P3=$!
wait $P1 $P2 $P3