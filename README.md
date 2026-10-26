# Credential revocation on Solana

Checking whether a certificate or credential has been revoked usually means asking a server: a CRL download or an OCSP responder. That server has to stay online, stay trusted, and stay up to date. When it is down, verifiers either fail or quietly skip the check.

This prototype moves revocation status onto Solana. An issuer revokes a credential by writing a small record on-chain, and anyone can check it publicly, with no server to run and no wallet needed to verify.

**Live demo:** https://mehmetxeyd.github.io/solana-credential-revocation/ (Solana devnet, needs the Phantom wallet in devnet mode for issuing and revoking; verifying needs no wallet)

**Program (devnet):** `HNGzq89ne642qkLDCszQ56Y6PKGgBSRTpmPHG3JRYcdX`

## How it works

1. **Issue.** The issuer's Solana wallet signs a credential (a small JSON document) with its Ed25519 key. The wallet's public key is the trust anchor. Nothing is stored on-chain at this point.
2. **Revoke.** The issuer sends one transaction to the Anchor program. It creates a record at an address derived from `issuer + SHA-256(credential)`. Only the issuer's signature can create it.
3. **Verify.** Anyone checks the issuer's signature, then looks up the derived address. If the record exists, the credential is revoked. The record also stores the time of revocation.

Because the record's address is fully determined by the issuer and the credential hash, verifiers never need to trust or query a particular server. They only read public chain state.

## Why Solana

- Revocation status becomes a public, always-available registry that nobody has to host.
- Writes are cheap and fast, so revoking costs a fraction of a cent.
- Solana's native signature scheme is Ed25519, so the issuer's wallet key signs credentials directly, with no extra key infrastructure.

## What is in this repo

| Path | What it is |
|---|---|
| `index.html` | The whole web app (issue, verify, revoke). Plain HTML and JavaScript, using `@solana/web3.js` and `tweetnacl`. |
| `program/lib.rs` | The Anchor program with one instruction, `revoke`, which creates the revocation record. |

## Try it

1. Install the Phantom wallet, turn on Testnet Mode, select Solana Devnet, and get devnet SOL from https://faucet.solana.com.
2. Open the live demo and connect Phantom.
3. **Issue** a credential and approve the signature.
4. **Verify** it. It should show Valid.
5. **Revoke** it and approve the transaction.
6. **Verify** again. It should now show Revoked.

## Limitations and next steps

This is a one-day prototype, so it deliberately stays small.

- Verifiers currently trust the issuer address inside the credential. A next step is an on-chain issuer registry, so a verifier can check that the issuer is a known organisation, such as a university.
- No expiry dates, batch revocation, or privacy features yet.
- Devnet only. No audit.
- Integration with existing X.509 and CRL tooling is a natural extension, for example anchoring a private CA's revocations on-chain.
- Agent access: verification could be exposed as a tool so AI agents can check credentials themselves.

## Built with

Anchor (Rust) on Solana devnet, `@solana/web3.js`, `tweetnacl`, and the Phantom wallet.
