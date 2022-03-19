# circom-circuits

## Installation

Follow official installation [instructions](https://docs.circom.io/getting-started/installation/#installing-dependencies).

Warning: There is an issue with one of the `snarkjs` dependencies. For the time being, use a version provided with this repo:

```bash
npm i -g snarkjs-0.4.15.tgz
```

## Usage

Run all steps in one go:

```bash
./scripts/all.bash
```

### Step by step

Compile circuit with:

```bash
./scripts/compile.bash
```

Compute the witness:

```bash
./scripts/witness.bash
```

Perform the trusted ceremony:

```bash
./scripts/ceremony.bash
```

Compute and verify the proof:

```bash
./scripts/proof.bash
```
