use crate::*;

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct BankAccountAggregateState {
    pub balance: u32, // CENTS
    pub min_balance: u32,
    pub account_number: String,
    pub customer_id: String,
    pub reserved_funds: HashMap<String, u32>, // wire_transfer_id -> amount
}

impl BankAccountAggregateState {
    /// Returns the regular balance minus the sum of transfer holds
    pub fn available_balance(&self) -> u32 {
        self.balance
            .checked_sub(self.reserved_funds.values().sum::<u32>())
            .unwrap_or(0)
    }

    /// Returns the total amount of funds on hold
    pub fn total_reserved(&self) -> u32 {
        self.reserved_funds.values().sum::<u32>()
    }

    /// Releases the funds associated with a wire transfer hold. Has no affect on actual balance, only available
    pub fn release_funds(self, reservation_id: &str) -> Self {
        let mut new_state = self.clone();
        new_state.reserved_funds.remove(reservation_id);

        new_state
    }

    /// Adds a reservation hold for a given wire transfer. Has no affect on actual balance, only available
    pub fn reserve_funds(self, reservation_id: &str, amount: u32) -> Self {
        let mut new_state = self.clone();
        new_state
            .reserved_funds
            .insert(reservation_id.to_string(), amount);
        new_state
    }

    /// Commits held funds. Subtracts held funds from balance. Note: A more realistic banking
    /// app might emit an overdrawn/overdraft event if the new balance is less than 0. Here we
    /// just floor the balance at 0. Also note that overcommits shouldn't happen because we reject
    /// attempts to hold beyond available funds
    pub fn commit_funds(self, reservation_id: &str) -> Self {
        let mut new_state = self.clone();
        let amount = new_state.reserved_funds.remove(reservation_id).unwrap_or(0);
        new_state.balance = new_state.balance.checked_sub(amount).unwrap_or(0);
        new_state
    }

    /// Withdraws a given amount of funds
    pub fn withdraw(self, amount: u32) -> Self {
        let mut new_state = self.clone();
        new_state.balance = new_state.balance.checked_sub(amount).unwrap_or(0);
        new_state
    }

    /// Deposits a given amount of funds. Ceilings at u32::MAX
    pub fn deposit(self, amount: u32) -> Self {
        let mut new_state = self.clone();
        new_state.balance = new_state
            .balance
            .checked_add(amount)
            .unwrap_or(new_state.balance);
        new_state
    }
}
