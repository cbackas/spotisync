#!/bin/bash

if [[ "${CONTINUOUS_SYNC,,}" == "true" ]]; then
  while true; do
    spotisync
    sleep 20
  done
else
  spotisync
fi
