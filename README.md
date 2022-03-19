# solana-riptide-2022

Verify ZKP in Solana programs

## Overview

This repo contains the following sources:

`./circom-circuits` - ZKP circuits implemented with [`circom`](https://github.com/iden3/circom/tree/master/circom)

`./solana-verifier` - Solana program to verify `circom` proofs

That also includes forked dependencies:

`./ffjavascript` - fork of [`ffjavascript`](https://github.com/iden3/ffjavascript); contains [fixes](https://github.com/iden3/ffjavascript/pull/30) for `./snarkjs`

`./getrandom` - fork of [`getrandom`](https://github.com/rust-random/getrandom); contains fixes for `arkworks-rs`, which is being used in `./solana-verifier`

`./snarkjs` - fork of [`snarkjs`](https://github.com/iden3/snarkjs); contains a Solana verifier [template](snarkjs/templates/verifier_groth16_solana.rs.ejs)

## Status

As of now, the project doesn't compile. The breaking issue is with making contributions in `snarkjs` for bls12-381. To reproduce, run `cd circom-circuits && ./scripts/all.bash`

Error:

```
[ERROR] snarkJS: Error: Curve not supported: 4002409555221667610661788685990436838824310417315709160196267947217350388461752218928110433526561144555515641069568
```

### What was accomplished?

- Implemented verifier [program](solana-verifier/programs/solana-verifier/src/verifier.rs) and corresponding [template for snakrjs](snarkjs/templates/verifier_groth16_solana.rs.ejs).
- Fixed some issues in the `snarkjs` toolchain

### What is left do do?
- Implement the rest of the verifier program using `arkwork-rs` stack
- Finish the verifier template for `snarkjs` and update the CLI scripts, etc.