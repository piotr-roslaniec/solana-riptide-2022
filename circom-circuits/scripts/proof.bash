#!/bin/bash

set -x
set -e

snarkjs groth16 prove multiplier2_0001.zkey witness.wtns proof.json public.json

snarkjs groth16 verify verification_key.json public.json proof.json
