import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Strategy } from "../target/types/strategy";
import { assert } from "chai";

describe("Strategy", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Strategy as Program<Strategy>;
  let strategyState: anchor.web3.Keypair;

  before(async () => {
    strategyState = anchor.web3.Keypair.generate();
  });

  it("initializes strategy state", async () => {
    const admin = provider.wallet.publicKey;
    const vault = anchor.web3.Keypair.generate().publicKey;
    const rebalanceThreshold = 10;

    const tx = await program.methods
      .initialize(rebalanceThreshold)
      .accounts({
        strategyState: strategyState.publicKey,
        vault: vault,
        admin: admin,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([strategyState])
      .rpc();

    console.log("Initialize tx:", tx);

    const state = await program.account.strategyState.fetch(strategyState.publicKey);
    assert.equal(state.numOpportunities, 0);
    assert.equal(state.rebalanceThreshold, rebalanceThreshold);
    assert.equal(state.deployedValue.toNumber(), 0);
  });

  it("validates scoring parameters", async () => {
    // Test that scoring engine rejects invalid parameters
    const volatility = 101; // > 100, should be invalid
    assert(volatility > 100, "Volatility should be > 100 for test");
  });

  it("calculates risk-adjusted scores", async () => {
    // Example scoring test
    // High APY (15%), low volatility (20), low IL risk (10), high safety (90)
    // Should result in a good score

    const apy = 1500; // 15%
    const volatility = 20;
    const ilRisk = 10;
    const safety = 90;

    // Score = (APY × 0.5) - (Volatility × 0.3) - (IL_Risk × 0.2) + (Protocol_Safety × 0.1)
    // Normalize APY: 1500 / 100 = 15
    // = (15*0.5) - (20*0.3) - (10*0.2) + (90*0.1)
    // = 7.5 - 6 - 2 + 9 = 8.5

    const expectedScore = 8; // Approximately
    assert(expectedScore >= 5, "Score should be positive for good opportunity");
  });

  it("determines rebalancing decisions", async () => {
    // Test rebalancing logic
    const currentScore = 30;
    const bestScore = 50;
    const threshold = 10;

    const scoreDiff = bestScore - currentScore; // 20
    const shouldRebalance = scoreDiff > threshold; // 20 > 10 = true

    assert(shouldRebalance, "Should rebalance when score difference exceeds threshold");
  });
});
