# warp-contracts

## Overview
Warp let you automate anything that can be defined by cosmos native msg or cw msg in a decentralized manner. In short, it maintains an on chain queue that stores all the jobs waiting to be executed, each job has a condition, when the condition is met, the msg of the job can be executed. Any one can run a bot (called keeper) to execute jobs and get reward.

## Repo structure
There are 2 contracts
### warp-account
Anyone creating jobs on warp will create an account on warp, this account is like a smart contract wallet, it holds the tokens for job reward and what the output of a job (eg: a limit order job will trade your LUNA for ASTRO), job creators can deposit and withdraw token from the their account.

this account contract is like cw20 contract that can be instantiated for every user (every token in the case of cw20). user send tx to warp-controller contract that instantiates an account contract.

the account itself has no function at all (except the init, where we specify the EOA creator and warp-controller). execute does nothing, query returns nothing, it's just an address that can hold tokens. so it's just like a wallet (smart contract wallet, controlled by user address instead of private key), all the send, withdraw function is done by warp-controller

### warp-controller
This handles account creation and job creation.
