# Sovena Phase 1 — Lock and Release Schedule

This document defines the deterministic release schedules
for locked Sovena supply in Phase 1.

All schedules are enforced entirely by smart contract logic.
No manual intervention or administrative control exists.

---

## Time Definitions

To ensure determinism and auditability, time is defined as follows:

- One year = 365 days
- One month = 30 days

These values are fixed constants.
Calendar-based variation is intentionally excluded.

---

## Genesis Time

The reference point for all schedules is the Phase 1 mainnet deployment time.

Genesis time is defined as:
- The block timestamp at which the lock contract is instantiated on mainnet

Testnet deployments do not activate real schedules
and are not considered part of the genesis timeline.

---

## Developer Allocation

Total allocation: 30,000,000 SOV

Purpose:
- Long-term alignment
- Elimination of short-term incentive distortion
- Enforced non-dominance in early circulation

### Lock Structure

- Cliff period: 3 years
- Release duration: 7 years
- Release frequency: yearly
- Total release events: 7

No tokens are released during the first three years.

### Release Amounts

- Annual release: 4,285,714 SOV
- Final remainder: 2 SOV
- Remainder is released together with the final tranche

Total released after all periods:
- 30,000,000 SOV

---

## Reserve Allocation

Total allocation: 90,000,000 SOV

Purpose:
- Ecosystem operations
- Infrastructure support
- Liquidity provisioning
- Long-term continuity

This allocation is not a discretionary treasury.

### Lock Structure

- Cliff period: none
- Release duration: 10 years
- Release frequency: monthly
- Total release events: 120

### Release Amounts

- Monthly release: 750,000 SOV
- Remainder: none

Released amounts accumulate if not claimed.
Unclaimed tokens are never forfeited.

---

## Claim Mechanism

Released tokens are not transferred automatically.

A release becomes effective only when a claim transaction is executed.

Key properties:
- Anyone may trigger a claim transaction
- Released tokens are always sent to the predefined beneficiary address
- The caller has no control over the destination
- Claims are cumulative and non-expiring

Failure to claim does not alter the schedule
and does not create additional entitlement.

---

## Immutability

Once deployed on mainnet:

- Schedules cannot be modified
- Beneficiaries cannot be changed
- Release rates cannot be accelerated or paused
- No emergency withdrawal is possible

If a schedule is incorrect, the only remedy is redeployment.

This constraint is intentional.
