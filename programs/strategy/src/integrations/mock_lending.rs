/// Mock lending protocol integration
/// Simulates depositing into a lending protocol like Kamino or Solend
/// Calculates compounding yield over time

pub struct MockLending {
    /// Deposited amount
    pub deposited: u64,
    /// Accumulated interest
    pub interest_earned: u64,
    /// Interest rate (as annual percentage, e.g., 1050 = 10.50%)
    pub annual_rate: u16,
    /// Deposit timestamp (for calculating compound interest)
    pub deposit_timestamp: i64,
}

impl MockLending {
    pub fn new(annual_rate: u16) -> Self {
        Self {
            deposited: 0,
            interest_earned: 0,
            annual_rate,
            deposit_timestamp: 0,
        }
    }

    pub fn deposit(&mut self, amount: u64, timestamp: i64) -> Result<(), String> {
        if amount == 0 {
            return Err("Deposit amount must be greater than 0".to_string());
        }

        self.deposited = self.deposited.checked_add(amount)
            .ok_or("Overflow in deposit".to_string())?;
        self.deposit_timestamp = timestamp;

        Ok(())
    }

    pub fn withdraw(&mut self, amount: u64) -> Result<u64, String> {
        if amount > self.deposited {
            return Err("Insufficient balance".to_string());
        }

        self.deposited = self.deposited.checked_sub(amount)
            .ok_or("Underflow in withdrawal".to_string())?;

        Ok(amount)
    }

    /// Calculate accrued interest based on time passed
    /// Uses simple interest for MVP (can upgrade to compound)
    pub fn calculate_interest(&self, current_timestamp: i64) -> u64 {
        if self.deposited == 0 {
            return 0;
        }

        let time_passed = current_timestamp.saturating_sub(self.deposit_timestamp) as u64;
        let seconds_per_year = 365 * 24 * 60 * 60;

        // Interest = Principal * Rate * Time / (100 * Year)
        let interest = (self.deposited as u128)
            .checked_mul(self.annual_rate as u128)
            .and_then(|x| x.checked_mul(time_passed as u128))
            .and_then(|x| x.checked_div(100 * seconds_per_year as u128))
            .unwrap_or(0) as u64;

        interest
    }

    /// Accrue interest up to current time
    pub fn accrue_interest(&mut self, current_timestamp: i64) {
        let new_interest = self.calculate_interest(current_timestamp);
        self.interest_earned = self.interest_earned
            .checked_add(new_interest.saturating_sub(self.interest_earned))
            .unwrap_or(u64::MAX);
    }

    /// Get current total value
    pub fn get_balance(&self) -> u64 {
        self.deposited.checked_add(self.interest_earned).unwrap_or(u64::MAX)
    }

    /// Get current APY (annual percentage yield)
    pub fn get_apy(&self) -> u16 {
        self.annual_rate
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lending_deposit() {
        let mut lending = MockLending::new(1050); // 10.5% APY
        lending.deposit(100_000, 1000).unwrap();
        assert_eq!(lending.deposited, 100_000);
    }

    #[test]
    fn test_lending_interest() {
        let mut lending = MockLending::new(1050); // 10.5% APY
        lending.deposit(100_000, 0).unwrap();

        // Calculate interest after 1 year (365 days)
        let one_year_seconds = 365 * 24 * 60 * 60i64;
        let interest = lending.calculate_interest(one_year_seconds);

        // Should be approximately 10500 (100000 * 0.105)
        assert!(interest > 10000 && interest < 11000, "Interest calculation incorrect");
    }

    #[test]
    fn test_lending_withdrawal() {
        let mut lending = MockLending::new(1050);
        lending.deposit(100_000, 0).unwrap();
        let result = lending.withdraw(50_000).unwrap();
        assert_eq!(result, 50_000);
        assert_eq!(lending.deposited, 50_000);
    }
}
