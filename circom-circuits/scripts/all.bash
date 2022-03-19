#!/bin/bash
 
./scripts/compile.bash && \
./scripts/witness.bash && \
./scripts/ceremony.bash && \
./scripts/proof.bash && \
echo All is well!
