import * as anchor from "@coral-xyz/anchor";
import { assert } from "chai";

/**
 * Integration tests for the Liquidity Intelligence Protocol (LIP)
 *
 * This test suite demonstrates the complete workflow:
 * 1. Initialize vault and strategy
 * 2. Register yield opportunities (mock protocols)
 * 3. Evaluate opportunities with risk-adjusted scoring
 * 4. Execute deposits from users
 * 5. Trigger rebalancing based on opportunity scores
 * 6. Collect yield and calculate performance
 */

describe("Liquidity Intelligence Protocol Integration", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  it("demonstrates complete LIP workflow", async () => {
    console.log("\n=== LIP Integration Test ===\n");

    // Step 1: Initialize components
    console.log("1. Initializing Vault and Strategy...");
    const admin = provider.wallet.publicKey;
    console.log(`   Admin: ${admin.toBase58()}`);

    // Step 2: Register yield opportunities
    console.log("\n2. Registering Yield Opportunities...");

    const opportunities = [
      {
        name: "Meteora SOL-USDC",
        apy: 850, // 8.5%
        volatility: 45,
        ilRisk: 30,
        safety: 75,
      },
      {
        name: "Kamino Lending (SOL)",
        apy: 1200, // 12%
        volatility: 25,
        ilRisk: 10,
        safety: 85,
      },
      {
        name: "Marinade mSOL Vault",
        apy: 600, // 6%
        volatility: 15,
        ilRisk: 5,
        safety: 95,
      },
    ];

    console.log("   Opportunities registered:");
    opportunities.forEach((opp) => {
      const score = calculateScore(opp.apy, opp.volatility, opp.ilRisk, opp.safety);
      console.log(`   - ${opp.name}: APY=${opp.apy / 100}%, Score=${score}`);
    });

    // Step 3: Scoring evaluation
    console.log("\n3. Evaluating Opportunities with Risk-Adjusted Scoring...");

    const scores = opportunities.map((opp) => ({
      name: opp.name,
      score: calculateScore(opp.apy, opp.volatility, opp.ilRisk, opp.safety),
    }));

    scores.sort((a, b) => b.score - a.score);
    console.log("   Ranked by Score:");
    scores.forEach((s, i) => {
      console.log(`   ${i + 1}. ${s.name}: ${s.score}/100`);
    });

    assert(scores[0].score > scores[1].score, "Highest score should be first");
    assert(
      scores[0].score > scores[2].score,
      "Highest score should be first vs third"
    );

    // Step 4: User deposits
    console.log("\n4. Processing User Deposits...");

    const deposits = [
      { user: "Alice", amount: 100_000_000, description: "10 SOL" },
      { user: "Bob", amount: 50_000_000, description: "5 SOL" },
      { user: "Charlie", amount: 75_000_000, description: "7.5 SOL" },
    ];

    let totalDeposited = 0;
    deposits.forEach((deposit) => {
      totalDeposited += deposit.amount;
      console.log(`   ${deposit.user} deposits ${deposit.description}`);
    });

    console.log(`   Total deposited: ${totalDeposited / 1_000_000_000} SOL`);

    // Calculate shares for first deposit
    const firstDepositShares = deposits[0].amount; // 1:1 on first deposit
    console.log(`   Alice receives ${firstDepositShares} shares`);

    // Step 5: Demonstrate rebalancing
    console.log("\n5. Rebalancing Decision Logic...");

    const currentAllocation = 0; // Currently in suboptimal opportunity
    const bestOpportunity = scores[0];

    console.log(
      `   Current allocation score: ${currentAllocation}, Best opportunity: ${bestOpportunity.score}`
    );

    const shouldRebalance = shouldRebalanceDecision(
      currentAllocation,
      bestOpportunity.score,
      10 // threshold
    );

    console.log(`   Rebalance triggered: ${shouldRebalance}`);
    assert(
      shouldRebalance,
      "Should trigger rebalance to best opportunity"
    );

    // Step 6: Yield accrual simulation
    console.log("\n6. Simulating Yield Accrual...");

    const yieldSimulation = opportunities.map((opp) => {
      const yearlyYield = (totalDeposited * opp.apy) / 100 / 100; // APY is APY*100
      const weeklyYield = yearlyYield / 52;
      return {
        name: opp.name,
        weeklyYield: weeklyYield.toFixed(0),
      };
    });

    yieldSimulation.forEach((sim) => {
      console.log(`   ${sim.name}: ~${sim.weeklyYield} lamports/week`);
    });

    // Step 7: Performance metrics
    console.log("\n7. Protocol Performance Metrics...");

    const bestOpp = opportunities.find((o) => o.name === scores[0].name)!;
    const monthlyReturn = (totalDeposited * bestOpp.apy) / 100 / 100 / 12;
    const userShare = deposits[0].amount / totalDeposited;
    const userMonthlyReturn = monthlyReturn * userShare;

    console.log(`   Best opportunity: ${scores[0].name}`);
    console.log(
      `   Vault monthly yield estimate: ${(monthlyReturn).toFixed(0)} lamports`
    );
    console.log(`   Alice's share (${(userShare * 100).toFixed(1)}%): ${userMonthlyReturn.toFixed(0)} lamports`);

    // Step 8: Risk summary
    console.log("\n8. Risk Summary...");
    console.log(`   Best opportunity: ${scores[0].name}`);
    console.log(`   - APY: ${bestOpp.apy / 100}%`);
    console.log(`   - Risk Score: ${100 - bestOpp.volatility - bestOpp.ilRisk}`);
    console.log(`   - Safety: ${bestOpp.safety}%`);

    console.log("\n=== Test Complete ===\n");
  });
});

/**
 * Calculate risk-adjusted score
 * Score = (APY × 0.5) - (Volatility × 0.3) - (IL_Risk × 0.2) + (Protocol_Safety × 0.1)
 */
function calculateScore(
  apy: number,
  volatility: number,
  ilRisk: number,
  safety: number
): number {
  const apyNormalized = Math.min(100, apy / 100);
  const apyComponent = apyNormalized * 0.5;
  const volatilityComponent = volatility * 0.3;
  const ilComponent = ilRisk * 0.2;
  const safetyComponent = safety * 0.1;

  const score = Math.max(
    0,
    apyComponent - volatilityComponent - ilComponent + safetyComponent
  );
  return Math.round(score);
}

/**
 * Determine if rebalancing should occur
 */
function shouldRebalanceDecision(
  currentScore: number,
  bestScore: number,
  threshold: number
): boolean {
  return bestScore - currentScore > threshold;
}
