# Casper Bank Contract

A smart contract that exemplifies payable entrypoints and withdrawing funds from contracts.

## Install

First, clone the repository

```bash
git clone https://github.com/dylanireland/casper-bank-contract.git
```

Generate a keypair:

```bash
casper-client keygen keys/
```

Fund this keypair with testnet CSPR using the [Casper Wallet](https://www.casperwallet.io/) at [cspr.live](https://cspr.live).

## Prepare

Move into the smart contract folder:

```bash
cd contract/
```

Compile the contract:

```bash
make prepare
make build-contract
```

Compile the client session code:

```bash
make build-client
```

Adjust *deploy.sh*, *deposit.sh*, *query.sh*, and *withdraw.sh* to your liking.

## Execute

Deploy the bank contract by running:

```bash
chmod +x deploy.sh
./deploy.sh
```

Deposit funds to the `deposit` entrypoint:

```bash
chmod +x deposit.sh
./deposit.sh
```

Check the balance of an account:

```bash
chmod +x query.sh
./query.sh
```

Withdraw funds from the contract:

```bash
chmod +x withdraw.sh
./withdraw.sh
```

