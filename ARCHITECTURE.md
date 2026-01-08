# Liquidity Intelligence Protocol - Architecture

## System Design

The LIP protocol consists of two main smart contract programs working in coordination:

```
┌─────────────────────────────────────────────────────────────┐
│                    Liquidity Intelligence Protocol          │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌──────────────────────────────────────────────────────┐   │
│  │         Vault Program (Fund Management)              │   │
│  ├──────────────────────────────────────────────────────┤   │
│  │ • User Deposits/Withdrawals                          │   │
│  │ • Share Token Minting (LP Tokens)                    │   │
│  │ • Yield Compounding                                  │   │
│  │ • Performance Fee Collection                         │   │
│  └──────────────────────────────────────────────────────┘   │
│                           ▲                                 │
│                           │ CPI Calls                       │
│                           │                                 │
│  ┌──────────────────────────────────────────────────────┐   │
│  │    Strategy Engine (Opportunity Evaluation)          │   │
│  ├──────────────────────────────────────────────────────┤   │
│  │ • Opportunity Registration                           │   │
│  │ • Risk-Adjusted Scoring                              │   │
│  │ • Rebalancing Execution                              │   │
│  │ • Performance Monitoring                             │   │
│  └──────────────────────────────────────────────────────┘   │
│                           │                                 │
│                           │ Aggregates from                 │
│                           ▼                                 │
│  ┌──────────────────────────────────────────────────────┐   │
│  │        Protocol Integrations (via CPI)               │   │
│  ├──────────────────────────────────────────────────────┤   │
│  │ • Meteora (AMM)                                      │   │
│  │ • Jupiter (DEX Aggregator)                           │   │
│  │ • Kamino (Lending/Yield)                             │   │
│  │ • Hyperliquid (Perpetuals)                           │   │
│  │ • Other DeFi Protocols                               │   │
│  └──────────────────────────────────────────────────────┘   │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

## Account Model

### Vault Program Accounts

#### 1. Vault Account
Primary account storing vault-level state.

```rust
pub struct Vault {
    pub total_assets: u64,           // All deposited SOL + yield
    pub total_shares: u64,           // LP tokens in circulation
    pub admin: Pubkey,               // Vault controller
    pub strategy_allocation: u8,     // Current allocation %
    pub total_yield: u64,            // All-time yield accrued
    pub accumulated_fees: u64,       // Fees collected
    pub num_users: u64,              // User count
    pub bump: u8,                    // PDA seed
}
```

**Relationships:**
- 1 Vault → N UserPositions
- 1 Vault → 1 StrategyState
- Vault is a PDA derived from seeds: `[b"vault", admin_address]`

#### 2. UserPosition Accounts
Each user has one position account per vault.

```rust
pub struct UserPosition {
    pub user: Pubkey,                // User wallet
    pub vault: Pubkey,               // Associated vault
    pub shares: u64,                 // LP tokens held
    pub deposit_timestamp: i64,      // Deposit time
    pub total_deposited: u64,        // Total amount deposited
    pub bump: u8,                    // PDA seed
}
```

**Derivation:** `[b"user_position", vault_address, user_address]`

#### 3. StrategyOpportunity Accounts
Tracks each yield opportunity.

```rust
pub struct StrategyOpportunity {
    pub protocol_name: [u8; 32],    // Protocol identifier
    pub apy: u16,                    // Annual % yield * 100
    pub volatility: u8,              // Risk score
    pub il_risk: u8,                 // Impermanent loss risk
    pub safety_score: u8,            // Protocol safety
    pub score: u16,                  // Calculated score
    pub active: bool,                // Is active
    pub bump: u8,                    // PDA seed
}
```

### Strategy Program Accounts

#### 1. StrategyState Account
Strategy engine configuration and state.

```rust
pub struct StrategyState {
    pub vault: Pubkey,               // Associated vault
    pub best_opportunity: u8,        // Index of best opportunity
    pub num_opportunities: u8,       // Total registered
    pub rebalance_threshold: u16,    // Score difference threshold
    pub last_rebalance: i64,         // Last rebalance timestamp
    pub deployed_value: u64,         // Capital currently deployed
    pub bump: u8,                    // PDA seed
}
```

#### 2. Opportunity Account
Individual yield opportunity details.

```rust
pub struct Opportunity {
    pub protocol_id: [u8; 32],      // Protocol identifier
    pub apy: u16,                    // Annual % yield * 100
    pub volatility: u8,              // 0-100 risk score
    pub il_risk: u8,                 // 0-100 IL risk
    pub safety_score: u8,            // 0-100 safety score
    pub score: u16,                  // Risk-adjusted score
    pub active: bool,                // Is opportunity active
    pub last_updated: i64,           // Last update time
    pub bump: u8,                    // PDA seed
}
```

#### 3. DeployedPosition Account
Tracks deployed capital in opportunities.

```rust
pub struct DeployedPosition {
    pub opportunity: Pubkey,         // Target opportunity
    pub amount: u64,                 // Amount deployed
    pub deployment_timestamp: i64,   // When deployed
    pub yield_earned: u64,           // Accumulated yield
    pub bump: u8,                    // PDA seed
}
```

## Data Flow

### Deposit Flow

```
User sends transaction:
  deposit(vault_address, amount)
          │
          ▼
    Vault program validates
    ├─ Amount > 0
    ├─ User has sufficient SOL
    └─ Vault exists
          │
          ▼
    Calculate shares to mint
    shares = amount * total_shares / total_assets
             (or amount if vault empty)
          │
          ▼
    Transfer SOL to vault account
          │
          ▼
    Create/Update UserPosition
    ├─ shares += calculated shares
    └─ Update timestamp (if new)
          │
          ▼
    Update Vault state
    ├─ total_assets += amount
    ├─ total_shares += shares
    └─ num_users++ (if new)
          │
          ▼
    Return receipt with shares
```

### Rebalancing Flow

```
Strategy Evaluator (off-chain or keeper)
          │
          ▼
    Poll current opportunities
    ├─ APY data
    ├─ Volatility metrics
    ├─ IL risk factors
    └─ Safety scores
          │
          ▼
    Call evaluate() for each opportunity
    ├─ Update metrics
    ├─ Recalculate scores
    └─ Update best opportunity
          │
          ▼
    Check rebalance conditions
    ├─ Best score > current + threshold
    ├─ Cooldown passed (1 hour)
    └─ Capital available to move
          │
          ▼
    If conditions met: Call rebalance()
    ├─ Withdraw from old opportunity
    ├─ Execute CPI to vault program
    ├─ Deploy to new opportunity
    └─ Update strategy state
          │
          ▼
    Emit rebalancing event
    (with old and new allocations)
```

### Yield Collection Flow

```
Keeper/Admin monitors vault
          │
          ▼
    Collect yield from all deployed positions
    ├─ Query each opportunity's earned yield
    ├─ Calculate net (yield - IL)
    └─ Tally total
          │
          ▼
    Call harvest(yield_amount)
          │
          ▼
    Vault program:
    ├─ Calculate fee = yield * 10%
    ├─ Net yield = yield - fee
    ├─ total_assets += net_yield
    ├─ accumulated_fees += fee
    └─ Shares price increases (assets up, shares same)
          │
          ▼
    All users' positions now worth more
    (no share count change, just higher per-share value)
```

## Scoring System

The core innovation: Risk-adjusted scoring that balances returns with risks.

### Formula

```
Score = (APY × 0.50)
        - (Volatility × 0.30)
        - (IL_Risk × 0.20)
        + (Protocol_Safety × 0.10)
```

### Component Breakdown

#### 1. APY Component (50% weight)
- **Input**: Annual percentage yield (as percentage * 100)
- **Normalization**: Capped at 100% for scoring
- **Formula**: `APY_normalized * 0.5`
- **Rationale**: Primary goal is yield; higher returns prioritized

#### 2. Volatility Component (30% weight)
- **Input**: Price volatility score (0-100, where 100 = most volatile)
- **Formula**: `Volatility * 0.3` (subtracted from score)
- **Rationale**: Higher volatility increases risk of losses

#### 3. IL Risk Component (20% weight)
- **Input**: Impermanent loss risk (0-100)
- **Formula**: `IL_Risk * 0.2` (subtracted from score)
- **Rationale**: IL from LPing can eat into gains

#### 4. Safety Component (10% weight)
- **Input**: Protocol safety score (0-100, where 100 = safest)
- **Formula**: `Safety * 0.1` (added to score)
- **Rationale**: Established protocols deserve a premium

### Example Calculations

**Opportunity A: Conservative Lending**
```
APY: 1050 (10.5%) → Normalized: 10
Volatility: 20
IL Risk: 5
Safety: 95

Score = (10 × 0.5) - (20 × 0.3) - (5 × 0.2) + (95 × 0.1)
       = 5 - 6 - 1 + 9.5
       = 7.5 → Score: 8/100
```

**Opportunity B: Risky Volatile LP**
```
APY: 30000 (300%) → Normalized: 100
Volatility: 95
IL Risk: 80
Safety: 20

Score = (100 × 0.5) - (95 × 0.3) - (80 × 0.2) + (20 × 0.1)
       = 50 - 28.5 - 16 + 2
       = 7.5 → Score: 8/100
```

Despite vastly different APY, both score similarly - showing proper risk adjustment.

## State Transitions

### Vault States

```
┌─────────────────────┐
│   Not Initialized   │
└──────────┬──────────┘
           │ initialize()
           ▼
┌──────────────────────┐
│   Initialized        │
│   (No deposits)      │
└──────────┬──────────┘
           │ deposit()
           ▼
┌──────────────────────┐
│   Active             │
│   (Has deposits)     │ ◄──┐
└──────────┬──────────┘     │
           │ harvest()      │ deposit()/
           │ withdraw()     │ withdraw()
           └─────────────────┘
```

### User Position States

```
┌──────────────────────┐
│   Not Positioned     │
└──────────┬──────────┘
           │ deposit()
           ▼
┌──────────────────────┐
│   Positioned         │
│   (Has shares)       │ ◄──┐
└──────────┬──────────┘     │
           │ withdraw()     │ deposit()/
           │ harvest()      │ withdraw()
           │                └────────────┘
           ▼ (all shares withdrawn)
┌──────────────────────┐
│   Closed             │
│   (No shares)        │
└──────────────────────┘
```

### Strategy Rebalancing States

```
┌──────────────────────────┐
│   Current Best           │
│   Score: 50              │
└──────────┬───────────────┘
           │ New opportunity
           │ emerges with
           │ Score: 65
           ▼
┌──────────────────────────┐
│   Rebalance Pending      │
│   Difference: 15 > 10    │
│   (Threshold)            │
└──────────┬───────────────┘
           │ rebalance()
           │ called
           ▼
┌──────────────────────────┐
│   Executing Rebalance    │
│   Withdraw from old      │
│   Deploy to new          │
└──────────┬───────────────┘
           │
           ▼
┌──────────────────────────┐
│   New Best: Score 65     │
│   Cooldown started       │
│   (Locked for 1 hour)    │
└──────────────────────────┘
```

## Security Model

### Access Control

1. **Vault Owner/Admin**
   - Initialize vault
   - Harvest yield
   - Collect fees

2. **Vault Users**
   - Deposit/withdraw own funds
   - Cannot withdraw within 24 hours

3. **Strategy Executor**
   - Register opportunities
   - Evaluate opportunities
   - Execute rebalancing

### Validation

All instructions validate:
- ✅ Signer authority (owns position being modified)
- ✅ Account ownership (correct program owns accounts)
- ✅ Account state (expected values in expected ranges)
- ✅ Amount validations (no zero amounts, no overflows)
- ✅ Time locks (cooldowns, withdrawal delays)

### Anti-Patterns Prevented

| Attack | Prevention |
|--------|-----------|
| Reentrancy | Solana's runtime prevents by design |
| Double-spending shares | Share math is atomic |
| Front-run rebalancing | Threshold prevents gaming |
| Flash loans | Time lock on new deposits |
| Sybil attack | Each position tracked separately |

## Gas Optimization

### Batch Operations
- Multiple deposits processed in single transaction
- Yield harvested and distributed in one operation
- Rebalancing combines withdrawal + deployment

### Account Minimization
- One vault account per vault
- One position account per user per vault
- One opportunity account per opportunity
- No redundant state duplication

### Computation Efficiency
- Scoring is O(1) per opportunity
- No complex calculations in hot path
- Time-based checks use timestamps, not counters

## Integration Patterns

### Adding New Protocol

1. **Create Integration Module**
   ```rust
   pub mod my_protocol {
       pub struct MyProtocolIntegration { ... }
       impl ProtocolIntegration for MyProtocolIntegration {
           fn deposit(&mut self, amount: u64) -> Result<(), Box<dyn Error>> { ... }
           fn withdraw(&mut self, amount: u64) -> Result<(), Box<dyn Error>> { ... }
           fn calculate_yield(&self) -> u64 { ... }
       }
   }
   ```

2. **Create Opportunity Account**
   - Store protocol metadata
   - Track current APY, volatility, IL risk

3. **Register Opportunity**
   - Call `register_opportunity()` with protocol metrics
   - System automatically scores it

4. **Monitor and Update**
   - Off-chain keeper monitors protocol metrics
   - Calls `evaluate()` with updated values
   - System recalculates score

## Performance Characteristics

### Time Complexity
- Deposit: O(1)
- Withdraw: O(1)
- Evaluate: O(1) per opportunity
- Rebalance: O(n) where n = number of opportunities

### Space Complexity
- Vault: ~120 bytes
- User position: ~80 bytes
- Opportunity: ~80 bytes
- Linear growth with users and opportunities

### Transaction Costs
- Simple deposit: ~5K compute units
- Deposit + rebalance: ~20K compute units
- Harvest: ~10K compute units

---

*Architecture designed for scalability, security, and risk-aware yield optimization.*
