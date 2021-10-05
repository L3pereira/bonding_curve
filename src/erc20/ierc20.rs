use crate::common::*;

// use async_trait::async_trait;

// #[async_trait]
pub trait IERC20 {


    ///
    /// Returns the amount of tokens in existence.
    ///
    fn total_supply(&mut self) -> Result<&mut TotalSupply, ERC20Error>;


    ///
    /// Returns the amount of tokens owned by `account`.
    ///
    fn balance_of(&mut self, address: &Address) -> Result<&mut Amount, ERC20Error>;


    ///
    ///Moves `amount` tokens from the caller's account to `recipient`.
    ///
    ///Returns a boolean value indicating whether the operation succeeded.
    ///
    ///Emits a {Transfer} event.
    ///
    fn transfer(&mut self, address: &Address, amount: Amount) -> Result<bool, ERC20Error>;


    ///
    ///@dev Returns the remaining number of tokens that `spender` will be
    ///allowed to spend on behalf of `owner` through {transferFrom}. This is
    ///zero by default.
    ///
    ///This value changes when {approve} or {transferFrom} are called.
    ///
    fn allowance(&self, owner: &Address, spender: &Address)  -> Result<Amount, ERC20Error>;


    ////
    ///Sets `amount` as the allowance of `spender` over the caller's tokens.
    ///
    ///Returns a boolean value indicating whether the operation succeeded.
    ///
    ///IMPORTANT: Beware that changing an allowance with this method brings the risk
    ///that someone may use both the old and the new allowance by unfortunate
    ///transaction ordering. One possible solution to mitigate this race
    ///condition is to first reduce the spender's allowance to 0 and set the
    ///desired value afterwards:
    ///https://github.com/ethereum/EIPs/issues/20#issuecomment-263524729
    ///
    ///Emits an {Approval} event.
    ///
    fn approve(&mut self, spender: &Address, amount: Amount) -> Result<bool, ERC20Error>;
    

    ///
    ///@dev Moves `amount` tokens from `sender` to `recipient` using the
    ///allowance mechanism. `amount` is then deducted from the caller's
    ///allowance.
    ///
    ///Returns a boolean value indicating whether the operation succeeded.
    ///
    ///Emits a {Transfer} event.
    ///
    fn transfer_from(&mut self, sender: &Address, recipient: &Address,  amount: Amount) -> Result<bool, ERC20Error>;

    // async fn transfer_event(&self, from: &Address, to: &Address, value: Amount);

    // async fn approval_event(&self, owner: &Address, spender: &Address, value: Amount);

}