# Liquidity Intelligence Protocol (LIP)

A smart yield optimizer for Solana that intelligently allocates capital across DeFi protocols based on risk-adjusted returns.

## Overview

LIP brings **intelligent, risk-aware yield optimization** to Solana DeFi. Unlike passive strategies or simple APY-chasing, LIP actively balances returns against risks like impermanent loss and volatility, making it suitable for both conservative and aggressive users.

### Key Innovation

Unlike existing yield aggregators that use static strategies, LIP introduces:
1. **On-chain Intelligence Engine**: Lightweight scoring system that evaluates and ranks yield opportunities in real-time
2. **Cross-Protocol Yield Routing**: Seamlessly moves liquidity between spot AMMs (Meteora), aggregators (Jupiter), lending (Kamino-style), and simulated perps exposure
3. **Risk-Weighted Optimization**: Balances APY with volatility, impermanent loss risk, and protocol safety scores
4. **Auto-Compounding with Gas Efficiency**: Batches operations to minimize transaction costs

### Compared to Reference Protocols

| Protocol | LIP |
|----------|-----|
| **vs Jupiter** | Not just swap aggregation, but full position management and yield optimization |
| **vs Meteora** | Not just an AMM, but a meta-layer that allocates TO AMMs based on yield |
| **vs Kamino** | More dynamic rebalancing with risk scoring; works across multiple protocols |
| **vs Hyperliquid** | Includes perps exposure as one strategy option, not the only focus |

## Risk-Adjusted Scoring Formula

```
Score = (APY × 50%) - (Volatility × 30%) - (IL Risk × 20%) + (Safety × 10%)
```

**Explanation:**
- **APY (50% weight)**: Primary driver - higher returns are prioritized
- **Volatility (30% weight)**: Penalizes price fluctuation risk
- **IL Risk (20% weight)**: Penalizes impermanent loss from LPing
- **Safety (10% weight)**: Rewards established, safer protocols

## Architecture

### Core Programs

#### Vault Program
- User deposit/withdrawal management
- Share token (LP token) minting
- Performance fee collection (10% of yield)
- Yield compounding

#### Strategy Engine
- Evaluates yield opportunities in real-time
- Calculates risk-adjusted scores
- Executes rebalancing when better opportunities exist
- Manages capital allocation

## Quick Start

### Build

```bash
anchor build
```

### Test

```bash
anchor test
```

### Deploy (Devnet)

```bash
anchor deploy --provider.cluster devnet
```

## Project Structure

```
blockchain/
├── programs/
│   ├── vault/
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── state.rs
│   │       ├── errors.rs
│   │       └── instructions/
│   │           ├── initialize.rs
│   │           ├── deposit.rs
│   │           ├── withdraw.rs
│   │           └── harvest.rs
│   └── strategy/
│       └── src/
│           ├── lib.rs
│           ├── state.rs
│           ├── scoring.rs
│           ├── errors.rs
│           ├── instructions/
│           ├── integrations/
│           │   ├── mock_amm.rs
│           │   └── mock_lending.rs
├── tests/
│   ├── vault.ts
│   ├── strategy.ts
│   └── integration.ts
└── migrations/
    └── deploy.ts
```

## User Flow

1. User deposits SOL into a vault
2. Receives LP tokens representing their share
3. Strategy engine evaluates yield opportunities
4. Capital deployed to highest-scoring opportunity
5. Yield accrues and compounds
6. User can withdraw after 24-hour time lock

## Testing

Run the comprehensive test suite:

```bash
# All tests
anchor test

# Specific test
anchor test -- --grep "Integration"
```

Tests cover:
- ✅ Vault initialization and share accounting
- ✅ Deposits and withdrawals
- ✅ Risk-adjusted scoring
- ✅ Rebalancing logic
- ✅ End-to-end integration flows

## Smart Contract Interfaces

### Vault Instructions

- `initialize()` - Create a new vault
- `deposit(amount)` - Deposit SOL and receive shares
- `withdraw(shares)` - Redeem shares for SOL
- `harvest(yield_amount)` - Collect and distribute yield

### Strategy Instructions

- `initialize(rebalance_threshold)` - Initialize strategy
- `register_opportunity(...)` - Register new yield opportunity
- `evaluate(...)` - Update opportunity metrics
- `rebalance()` - Execute rebalancing

## Example Scoring

### Conservative Strategy (Kamino Lending)
- APY: 10%
- Volatility: 20%
- IL Risk: 5%
- Safety: 95%
- **Score: 9/10** ✓

### Aggressive Strategy (Volatile LP)
- APY: 200%
- Volatility: 85%
- IL Risk: 60%
- Safety: 40%
- **Score: 4/10** ⚠️

## Key Features

- **Multiple Strategy Types**: Conservative, Balanced, Aggressive
- **Automatic Rebalancing**: Moves capital when opportunities improve
- **Performance Fees**: 10% of net yield to protocol
- **Time Lock**: 24-hour withdrawal lock prevents gaming
- **Mock Integrations**: Test protocols for MVP development

## Future Enhancements

- Real protocol integrations (Jupiter, Meteora, Kamino, Hyperliquid)
- Multi-asset support (USDC, mSOL, etc.)
- Oracle integration for live data
- Governance token
- Web UI
- Advanced ML strategies

## Security Notes

This is an MVP implementation. Production use requires:
- Formal security audit
- Insurance mechanisms
- Decentralized governance
- Emergency pause functionality

## Resources

- [Anchor Documentation](https://www.anchor-lang.com/)
- [Solana Documentation](https://docs.solana.com/)

---

**LIP: Intelligent Yield Optimization for Solana** 🚀
