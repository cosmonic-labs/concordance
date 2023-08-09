#!/bin/bash
sed -E '/p:doc|p:desc/d' ./lunar_frontiers.ttl > filtered.ttl
rapper -i turtle -o dot filtered.ttl | dot -Tsvg -omodel.svg -Glabel="Lunar Frontiers"