#!/bin/bash

set -x
set -e

circom multiplier2.circom --wasm --r1cs
