// First, import some helper libraries. `shelljs` is included in the
// devDependencies of the root project, which is why it's available here. It
// makes it easy to use *NIX-style scripting (which works on Linux distros,
// macOS, and Unix systems) on Windows as well.
const sh = require("shelljs");

// Parse near call response and return result of contract function call
const getResult = (res) => {
  return res.stdout.split(/\r?\n/).slice(-2, -1)[0];
};

const main = async () => {
  const contractName =
    process.env.CONTRACT_NAME ||
    fs.readFileSync("./neardev/dev-account").toString();

  console.log("Try to craft a new hero ...");
  sh.exec(
    `near view ${contractName} nft_supply_for_owner '{"account_id": "sergantche.testnet"}'`
  );
  let res = await sh.exec(
    `near call ${contractName} craft_new_hero '{"username": "sergantche.testnet"}' --deposit-yocto 9000000000000000000000 --account-id ${contractName} --gas 300000000000000`
  );
  const tokenId = getResult(res);

  // Get new hero statistics
  res = await sh.exec(
    `near view ${contractName} get_stats '{"token_id": "${tokenId}"}'`
  );

  // Exit script with the same code as the build command
  process.exit();
};

// Run tests
main();
