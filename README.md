# solana-private-key-converter

A small Rust command-line tool that converts a Solana keypair file into its **Base58** private-key string. Point it at the `[u8, …]` JSON array that `solana-keygen` writes (e.g. `wallet.json`) and it prints the Base58 form — the format wallet apps such as Phantom expect when importing a private key. It is a one-way converter — it generates nothing and has no randomness; it simply re-encodes a keypair you already have.

> **This tool prints a private key in plain text.** Anyone who sees that output — on your screen, in your shell history, in a log, or in a screenshot — gains full, irreversible control of the wallet and any funds in it. Run it only on a machine you trust, and never share or paste the output anywhere you do not control.
>
> **⚠️ Educational use only — see the [Disclaimer](#disclaimer) before using this software.**

## What it does

Given the path to a keypair JSON file, it Base58-encodes the raw bytes and prints:

- `base58_full_<N>bytes` — the entire file, Base58-encoded
- `base58_seed_32bytes` — for a 64-byte Solana keypair, the 32-byte seed (the first half), Base58-encoded as well

It accepts a **64-byte** Solana CLI keypair or a **32-byte** seed; any other length, or malformed JSON, is rejected with a clear error. The tool only **reads** the file you pass it — it never writes, generates, transmits, or stores anything — and it prints a warning to stderr before showing the key.

## Prerequisites

- Rust — a recent stable toolchain (`edition = "2024"`; 1.85+)
- A Solana keypair JSON file — e.g. one created with `solana-keygen new --outfile wallet.json`

## Usage

```bash
# Build
cargo build --release

# Convert a keypair file — pass the path as the only argument
cargo run --release -- wallet.json
```

The Base58 strings are printed to stdout; the safety warning is printed to stderr.

### Example

```
$ cargo run --release -- wallet.json
WARNING: You are about to print a private key. Do NOT share this output.
base58_full_64bytes: <88-character Base58 string>
base58_seed_32bytes: <44-character Base58 string>
```

A 32-byte seed file instead prints a single `base58_full_32bytes` line.

## Disclaimer

**This software is provided for educational and informational purposes only.**

- **It exposes private keys.** This tool prints the Base58 private key of whatever keypair file you give it. A private key grants full, irreversible control of its wallet and any funds it holds — treat both the input file and the printed output as you would the keys to a safe.
- **You are solely responsible** for the keypair files you supply, for the machine you run this on, and for keeping the output secret — including from shell history, logs, screen shares, and screenshots. Run it only on a trusted machine.
- **No warranty.** This software is provided "AS IS", without warranty of any kind, express or implied. It may contain bugs and may behave incorrectly — including through software defects or malformed input.
- **No liability.** To the maximum extent permitted by law, the author(s) and contributors shall not be liable for any claim, damages, or other liability — including loss of funds — arising from or in connection with the use of, or inability to use, this software.

By using, running, modifying, or distributing this software, you acknowledge that you have read and understood this disclaimer and accept full responsibility for the outcomes.

## Author

**Taki Hades Baker Alyasri**

## License

MIT — see the [Disclaimer](#disclaimer) above. The MIT license's "AS IS", no-warranty, and no-liability terms apply to all use of this software.
