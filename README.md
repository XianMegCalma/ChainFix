# ChainFix

Blockchain-based dormitory maintenance request and escrow payment system built on Stellar Soroban.

---

## Problem

Dormitory tenants often report maintenance problems through chat apps where complaints can be ignored, deleted, or disputed later.

---

## Solution

ChainFix Dorm records maintenance requests on Stellar and uses Soroban smart contracts to automatically release repair payments after completion confirmation.

---

## Timeline

Hackathon MVP:
- Day 1: Smart contract logic
- Day 2: Frontend integration
- Day 3: Wallet connection + demo flow

---

## Stellar Features Used

- Soroban Smart Contracts
- USDC Transfers
- Trustlines
- On-chain Event Logging

---

## Vision and Purpose

Create transparent and tamper-proof maintenance coordination between tenants, landlords, and repair workers while enabling secure escrow payments.

---

## Prerequisites

- Rust
- Soroban CLI
- Stellar Testnet Account

Install Soroban CLI:

```bash
cargo install soroban-cli

## Contract ID 
CA7J3QFCF3LGYEZYGZ3WPR5RF64BSIBWB3PGYTWVGQLYRNZ5LMIDMVCF