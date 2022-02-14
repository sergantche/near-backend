// First, import some helper libraries. `shelljs` is included in the
// devDependencies of the root project, which is why it's available here. It
// makes it easy to use *NIX-style scripting (which works on Linux distros,
// macOS, and Unix systems) on Windows as well.
const sh = require("shelljs");
const contractName =
  process.env.CONTRACT_NAME ||
  fs.readFileSync("./neardev/dev-account").toString();

console.log("Try to craft a new hero ...");
sh.exec(
  `near view ${contractName} nft_supply_for_owner '{"account_id": "sergantche.testnet"}'`
);
sh.exec(
  `near call ${contractName} craft_new_hero '{"username": "sergantche.testnet"}' --deposit-yocto 9000000000000000000000 --account-id ${contractName} --gas 300000000000000`
);
sh.exec(
  `near view ${contractName} nft_supply_for_owner '{"account_id": "sergantche.testnet"}'`
);
sh.exec(
  `near view ${contractName} nft_tokens_for_owner '{"account_id": "sergantche.testnet"}'`
);

// exit script with the same code as the build command
process.exit();
