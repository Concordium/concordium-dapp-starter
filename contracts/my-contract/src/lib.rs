#![cfg_attr(not(feature = "std"), no_std)]

//! # A Concordium V1 smart contract
use concordium_std::*;

/// Your smart contract state.
#[derive(Serialize, SchemaType)]
pub struct State {
    pub count: u64,
}

/// Your smart contract errors.
#[derive(Debug, PartialEq, Eq, Reject, Serialize, SchemaType)]
pub enum ContractError {
    /// Failed parsing the parameter.
    #[from(ParseError)]
    ParseParams, // Reject code: -1
    /// Error to throw if incrementing by 0.
    IncrementByZero, // // Reject code: -2
}

#[derive(Serialize, SchemaType)]
pub struct InitParameter {
    pub initial_value: u64,
}

/// Init function that creates a new smart contract.
#[init(contract = "my_contract", parameter = "InitParameter")]
fn init(ctx: &InitContext, _state_builder: &mut StateBuilder) -> InitResult<State> {
    let parameter: InitParameter = ctx.parameter_cursor().get()?; // Returns Error::ParseError on failure
    Ok(State {
        count: parameter.initial_value,
    })
}

#[derive(Serialize, SchemaType)]
pub struct IncrementParameter {
    pub increment_by: u64,
}

/// Receive function. The input parameter is the boolean variable `throw_error`.
///  If `throw_error == true`, the receive function will throw a custom error.
///  If `throw_error == false`, the receive function executes successfully.
#[receive(
    contract = "my_contract",
    name = "increment",
    parameter = "IncrementParameter",
    error = "ContractError",
    mutable
)]
fn receive(ctx: &ReceiveContext, host: &mut Host<State>) -> Result<(), ContractError> {
    let parameter: IncrementParameter = ctx.parameter_cursor().get()?; // Returns Error::ParseError on failure.
    ensure!(parameter.increment_by != 0, ContractError::IncrementByZero);
    let state = host.state_mut();
    state.count += parameter.increment_by;
    Ok(())
}

/// View function that returns the content of the state.
#[receive(contract = "my_contract", name = "view", return_value = "State")]
fn view<'b>(_ctx: &ReceiveContext, host: &'b Host<State>) -> ReceiveResult<&'b State> {
    Ok(host.state())
}
