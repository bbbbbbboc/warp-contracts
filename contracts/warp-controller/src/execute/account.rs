use crate::state::{ACCOUNTS, CONFIG};
use crate::ContractError;
use cosmwasm_std::{
    to_binary, CosmosMsg, DepsMut, Env, MessageInfo, ReplyOn, Response, SubMsg, WasmMsg,
};

pub fn create_account(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    // query by sub index (which is a warp account (contract) address)
    // map.get(sub_idx_warp_account_contract_address = sender)
    // item is not null if there's a warp account contract address is the sender, i.e. create_account called by a warp account contract
    let item = ACCOUNTS()
        .idx
        .account
        .item(deps.storage, info.sender.clone());

    // this only happens when warp account (instantiated from warp account template) is the sender calling this create_account fn,
    // maybe this can happen in the future? when one warp account can control other warp account?
    // i.e. warp account is no longer controlled by warp controller + EOA
    if item?.is_some() {
        // really need better naming, need to differ account and warp account
        return Err(ContractError::AccountCannotCreateAccount {});
    }

    // the EOA already has a warp account created, then why won't we fail here?
    // query by primary key, map.get(owner_eoa_address = info.sender)
    // not null if there's a warp account associated with the info sender, i.e. sender already has warp account
    if ACCOUNTS().has(deps.storage, info.sender.clone()) {
        let _account = ACCOUNTS().load(deps.storage, info.sender)?;
        return Err(ContractError::WarpAccountExistInExecuteCreateAccountError {});
        // return Ok(Response::new()
        //     .add_attribute("action", "create_account")
        //     .add_attribute("owner", account.owner)
        //     .add_attribute("account_address", account.account));
    }

    let submsg = SubMsg {
        id: 0,
        msg: CosmosMsg::Wasm(WasmMsg::Instantiate {
            admin: None,
            code_id: config.warp_account_code_id.u64(),
            msg: to_binary(&warp_protocol::account::InstantiateMsg {
                owner: info.sender.to_string(),
            })?,
            funds: vec![],
            label: info.sender.to_string(),
        }),
        gas_limit: None,
        reply_on: ReplyOn::Always,
    };

    Ok(Response::new()
        .add_attribute("action", "create_account")
        .add_submessage(submsg))
}
