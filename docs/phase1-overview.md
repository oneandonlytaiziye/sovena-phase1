# Sovena Phase 1 — Technical Overview

Phase 1 establishes the initial on-chain representation of Sovena
within an existing privacy-native environment.

This phase is not a blockchain launch.
It does not involve mining, block rewards, or monetary issuance.

Its sole purpose is to enforce monetary discipline at genesis
and to provide a verifiable foundation for future sovereign deployment.

---

## Scope of Phase 1

Phase 1 is limited in scope by design.

Included:
- Fixed-supply token deployment
- Privacy by default at the token level
- On-chain enforced supply distribution
- Immutable time-lock and vesting logic

Excluded:
- Mining or validator incentives
- Block rewards
- Governance mechanisms
- Monetary policy adjustments
- Exchange or liquidity systems

Phase 1 introduces no discretionary controls.

---

## Host Environment

Sovena Phase 1 is deployed on Secret Network.

The chosen environment provides:
- Encrypted contract state
- Confidential balances
- Confidential transfer amounts
- Opt-in disclosure through viewing keys or permits

Privacy is enforced at the protocol level and not implemented as an external layer.

---

## Token Characteristics

- Token standard: SNIP-20
- Supply model: Fixed supply, single mint
- Privacy model: Confidential by default

The token contract contains no mint function after deployment.

All tokens are created once and distributed according to predefined rules.

---

## Supply Distribution

Total supply: 128,000,000 SOV

Distribution at genesis:

- 8,000,000 SOV  
  Initial circulation

- 30,000,000 SOV  
  Developer allocation, locked

- 90,000,000 SOV  
  Reserve allocation, locked and released deterministically

No additional supply can be created under any circumstances.

---

## Time Enforcement

All time-based rules in Phase 1 are enforced on-chain.

Time is measured using fixed durations:
- One year = 365 days
- One month = 30 days

Calendar-based variability is intentionally avoided.

The effective genesis time for Phase 1 is defined as the block timestamp
at mainnet contract deployment.

Testnet deployments are used solely for validation
and do not activate time-based release schedules.

---

## Relationship to Later Phases

Phase 1 does not attempt to replicate the final sovereign network.

It exists to:
- Lock supply rules
- Prove enforceability
- Provide a migration reference for Phase 3

Phase 3 introduces native consensus, mining, and block-level issuance.
Those mechanisms are explicitly out of scope for Phase 1.
