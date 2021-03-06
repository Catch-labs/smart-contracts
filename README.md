# Catch smart-contracts

This Repo contains all the smart contracts related to Catch and tests related to them.

It is basically divided into 3 contracts : 

## 1. Fungible Token
` Testnet Contract : Will be Updated Later!`

```
Catch tokens are the Fungible Tokens that will be utilised for the purpose of this project

This Contract would contain all the logic related to FT & FT Reward Distribution functionality that will be in conjuction with NFT Contract

All the In-Game Objectives and their Metadata ipfs links are to be hardcoded into the contract and validated before being deployed
```

## 2. Non-Fungible Token
` Testnet Contract : Will be Updated Later!`

```
This Contract would contain all the logic related to NFT & In-Game Achievements functionality that will be in conjuction with FT Contract

It would also track player achievements and their NFT's

It would also handle business side of things for them to upload and get featured in the Catch Map

For any user to hold Catch NFT's they have to mandatorily go through the KYC process for Legal Compliances and will be able to create sub-account Eg : username.catchlabs.near

```

## 3. Market
` Testnet Contract : Will be Updated Later!`

```
This Contract will basically handle all the logic regarding NFT marketplace

It would also handle swapping, trading of NFT's and much more
```

All Contracts follow the Near Standards for smart-contracts with slight appropriate Modifications

You can go the appropriate Contract folders to find their appropriate Readme files.

User flow for the contract is also explained in the readme itself.

### Build Wasm 

> To build Wasm files of contracts, go the appropriate contract folder and then execute the below command

```console
./build.sh
```

### Sandbox tests

> Executing below commands will run a local blockchain and run sandbox-tests for all 3 contracts

```console
./mock_chain.sh
./sand_test.sh
```

