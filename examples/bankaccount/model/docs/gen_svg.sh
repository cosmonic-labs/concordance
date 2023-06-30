#!/bin/bash
sed -E '/p:doc|p:desc/d' ../../bankaccount-model.ttl > filtered.ttl
rapper -i turtle -o dot filtered.ttl | dot -Tsvg -omodel.svg -Glabel="Bank Account Example"