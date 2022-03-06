const anchor = require('@project-serum/anchor');

const main = async()=>{
  console.log("startingTest")

  // this is to set the anchor net eg local, devnet or mainnet
  // env() use the setting we set in solana config get, 
  anchor.setProvider(anchor.Provider.env());

  // anchor.workspace will auto compile the code in libs.rs and get it deployed in the provider given
  // note structure and naming of the folder is very impt
  const program = anchor.workspace.Myepicproject;

  //
  const tx = await program.rpc.startStuffOff();

  console.log("📝 Your transaction signature", tx);
}

const runMain = async () => {
  try {
    await main();
    process.exit(0);
  } catch (error) {
    console.error(error);
    process.exit(1);
  }
};

runMain();