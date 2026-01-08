/// Risk-adjusted yield scoring logic
///
/// Score = (APY × 0.5) - (Volatility × 0.3) - (IL_Risk × 0.2) + (Protocol_Safety × 0.1)
///
/// This creates a balanced approach:
/// - Prioritizes higher APY (50% weight)
/// - Reduces score for volatility (30% weight)
/// - Reduces score for IL risk (20% weight)
/// - Rewards safer protocols (10% weight bonus)

pub struct ScoringEngine;

impl ScoringEngine {
    /// Calculate risk-adjusted score for an opportunity
    pub fn calculate_score(
        apy: u16,        // APY as percentage * 100 (e.g., 1050 = 10.50%)
        volatility: u8,  // 0-100
        il_risk: u8,     // 0-100
        safety_score: u8, // 0-100
    ) -> u16 {
        // Normalize APY from percentage * 100 to 0-100 scale
        // Cap at 100% for scoring purposes
        let apy_normalized = std::cmp::min(100, (apy / 100) as u8);

        let apy_component = (apy_normalized as u16) * 50 / 100;
        let volatility_component = (volatility as u16) * 30 / 100;
        let il_component = (il_risk as u16) * 20 / 100;
        let safety_component = (safety_score as u16) * 10 / 100;

        let score = apy_component
            .saturating_sub(volatility_component)
            .saturating_sub(il_component)
            .saturating_add(safety_component);

        // Ensure score is in reasonable range (0-100)
        std::cmp::min(100, score)
    }

    /// Determine if rebalancing is needed
    pub fn should_rebalance(
        current_score: u16,
        best_score: u16,
        threshold: u16,
    ) -> bool {
        let score_diff = best_score.saturating_sub(current_score);
        score_diff > threshold
    }

    /// Calculate allocation based on score
    /// Returns percentage allocation (0-100)
    pub fn calculate_allocation(
        score: u16,
        total_score: u16,
    ) -> u8 {
        if total_score == 0 {
            return 0;
        }

        let allocation = ((score as u32 * 100) / total_score as u32) as u8;
        std::cmp::min(100, allocation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scoring_high_apy_low_risk() {
        // High APY (15%), low volatility (20), low IL risk (10), high safety (90)
        let score = ScoringEngine::calculate_score(1500, 20, 10, 90);
        // Expected: (15*0.5) - (20*0.3) - (10*0.2) + (90*0.1)
        // = 7.5 - 6 - 2 + 9 = 8.5 -> ~8
        assert!(score > 5, "Score should reward good opportunities");
    }

    #[test]
    fn test_scoring_low_apy_high_risk() {
        // Low APY (2%), high volatility (80), high IL risk (70), low safety (20)
        let score = ScoringEngine::calculate_score(200, 80, 70, 20);
        // This should result in a low score
        assert!(score < 20, "Score should penalize risky opportunities");
    }

    #[test]
    fn test_rebalance_decision() {
        assert!(ScoringEngine::should_rebalance(30, 50, 10));
        assert!(!ScoringEngine::should_rebalance(45, 50, 10));
    }
}
