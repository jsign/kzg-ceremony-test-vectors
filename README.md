# Ethereum KZG Powers of Tau Test Vectors

This repository contains a program that generates a `BatchContribution` from the [initialContribution.json](https://github.com/ethereum/kzg-ceremony-specs/blob/master/initialContribution.json) from the spec. The generation is done using the [reference implementation](https://github.com/crate-crypto/small-powers-of-taureference) ceremony client.


This can help KZG ceremony client implementations to validate that their batch contribution logic is working as expected. This is possible since `f(initialContribution, subCeremonySecrets) -> newBatchContribution` is deterministic.

Clients can also cross-validate their implementations with the devnet sequencer, but that is slower to validate than a unit test and also implies building the API flow logic.


## How should I use this?
In your client implementation, create a test that:
- Loads `initialContribution.json`
- Execute a contribution with fixed secrets `[0x111100, 0x221100, 0x331100, 0x441100]`. Each secret is a big-endian byte representation of the n-th sub ceremony `x` (private key).
- Serialize your new batch contribution, and compare it with `updatedContribution.json`.

If they match, then your KZG ceremony client generates a batch contribution that matches the reference implementation, so there's a high chance that you're not missing any details.

Soon I'll include a link on how this test vector is used in a Go client implementation.

## How do I regenerate the `updatedContribution.json`?
Simply run `cargo run`. The `updateContribution.json` file in the repository is just a cached version.

```bash
$ cargo run
   Compiling kzg-ceremony-test-vectors v0.1.0 (/home/ignacio/code/kzg-ceremony-test-vectors)
    Finished dev [unoptimized + debuginfo] target(s) in 0.70s
     Running `target/debug/kzg-ceremony-test-vectors`
unmarshaling initial contribution file... 
done.
parsing initial contribution...
done.
fixed secrets for test vector: ["0x111100", "0x221100", "0x331100", "0x441100"]
updating contribution... 
done.
```

## Why only one test vector?
Today running the program generates the output for the fixed secrets `[0x111100, 0x221100, 0x331100, 0x441100]` (one for each sub-ceremony). If the output of your client matches the expected output, that's high signal that your implementation is correct since getting that by chance is impossible.

However, we could change the code in this repo to generate `(N * #sub-ceremonies`) random test vectors and `N` `updatedContribution.json` outputs.

If you're interested in generating the output for specific secrets, you can easily change that in `main.rs`.

