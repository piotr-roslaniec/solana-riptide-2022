#!/bin/bash

set -x
set -e

cd multiplier2_js
node generate_witness.js multiplier2.wasm ../input.json ../witness.wtns