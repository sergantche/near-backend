## Description

The contract serves as a registry for NFT tokens and implements NEAR standarts. Each NFT represents a hero with parameters: rarity, name, stars, experience, maximum_level.
To craft an NFT hero without NEAR authentication, you can use the API server.

### Contract Interface

Change Methods:

1. `update_hero_stats(token_id, stars, experience, maximum_level)` - update hero parameters for `token_id` hero;
1. `craft_new_hero(username) -> token_id` - mint NFT using a probabilistic algorithm and transfer it to `username` account;

Read-only Methods:

1. `get_stats(token_id) -> (stars, experience, maximum_level, rarity)` - return hero stats.

### API server

Deployed to Google cloud, has contract level account credentials for now.

HTTPS Endpoints:

1. `GET /api/ -> hello_message`;
1. `GET /api/craft-hero?nearid=<account_id> -> token_id` mints NFT for specified account `account_id`;

## Development

### Local development

Install dependencies:

```bash
yarn
```

Build contract:

```bash
yarn build
```

Deploy contract from dev account:

```bash
yarn dev:deploy
```

or build and deploy simultaniously:

```bash
yarn dev:build:deploy
```

Run acceptance tests:

```bash
yarn test
```

### Testnet contract deployment

To deploy the contract to NEAR account set appropriate values for `CONTRACT` and `MASTER_ACCOUNT` in `./scripts/deploy.sh` file. Then run:

```bash
yarn deploy:contract
```

To run tests set `CONTRACT_NAME` in `./config/testnet-account.env` and run:

```bash
yarn testnet:test
```

### API server deployment

Start API server locally bound to dev contract:

```bash
yarn dev:server
```

Run acceptance tests for the API service:

```bash
yarn test:server
```

Deploy to Google cloud:

```bash
yarn deploy:server
```
