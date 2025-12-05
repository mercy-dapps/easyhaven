import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Easyhaven } from "../target/types/easyhaven";
import { Keypair, PublicKey } from "@solana/web3.js";

describe("easyhaven", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider();

  const program = anchor.workspace.easyhaven as Program<Easyhaven>;

  const [owner, buyer] = Array.from({ length: 2 }, () => Keypair.generate());

  async function airdropSol(publicKey, amount) {
    let airdropTx = await provider.connection.requestAirdrop(publicKey, amount);
    await confirmTransaction(airdropTx);
  }

  async function confirmTransaction(tx) {
    const latestBlockHash = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: tx,
    });
  }

  const mercy_owner = PublicKey.findProgramAddressSync(
    [Buffer.from("owner"), owner.publicKey.toBuffer()],
    program.programId
  )[0];

  const ownerAccounts = {
    propertyOwner: mercy_owner,
    owner: owner.publicKey,
  };

  const mercyOwner = {
    name: "Mercy Adams",
    email: "mercy@owner.com",
    phone_number: "08065980493",
    location: "Ibadan, Nigeria",
  };

  it("airdrop", async () => {
    await Promise.all([owner, buyer].map((a) => airdropSol(a.publicKey, 1e9)));
  });

  it("Create Mercy Owner", async () => {
    const { name, email, phone_number, location } = mercyOwner;

    await program.methods
      .initializeOwner(name, email, phone_number, location)
      .accounts({ ...ownerAccounts })
      .signers([owner])
      .rpc();

    const ownerAccount = await program.account.propertyOwner.fetch(mercy_owner);

    console.log(`Owner: ${JSON.stringify(ownerAccount)}`);
  });
});
