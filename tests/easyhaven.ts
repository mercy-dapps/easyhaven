import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Easyhaven } from "../target/types/easyhaven";
import { Keypair, PublicKey } from "@solana/web3.js";

describe("easyhaven", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider();

  const program = anchor.workspace.easyhaven as Program<Easyhaven>;

  const [owner, user] = Array.from({ length: 2 }, () => Keypair.generate());

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

  const mercy_user = PublicKey.findProgramAddressSync(
    [Buffer.from("user"), user.publicKey.toBuffer()],
    program.programId
  )[0];

  const ownerAccounts = {
    propertyOwner: mercy_owner,
    owner: owner.publicKey,
    user: mercy_user,
    signer: user.publicKey,
  };

  const mercyOwner = {
    name: "Mercy Adams",
    email: "mercy@owner.com",
    phone_number: "08065980493",
    location: "Ibadan, Nigeria",
  };

  const mercyUser = {
    name: "Mercy User",
    email: "mercy@user.com",
    phone_number: "08012345678",
    location: "Abuja, Nigeria",
  };

  const updateMercyUser = {
    gender: "Female",
    profile_picture: "",
    bio: "A young beautiful girl",
    profession: "Smart Contract Engineer",
    languages_spoken: ["English", "Yoruba", "Spanish"],
    budgets: "2000000",
  };

  it("airdrop", async () => {
    await Promise.all([owner, user].map((a) => airdropSol(a.publicKey, 1e9)));
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

  it("Create Mercy user", async () => {
    const { name, email, phone_number, location } = mercyUser;

    await program.methods
      .initializeUser(name, email, phone_number, location)
      .accounts({ ...ownerAccounts })
      .signers([user])
      .rpc();

    const userAccount = await program.account.user.fetch(mercy_user);

    console.log(`User: ${JSON.stringify(userAccount)}`);
  });

  it("Mercy wants to become a host", async () => {
    await program.methods
      .becomeAHost()
      .accounts({
        user: mercy_user,
      })
      .rpc();

    const userAccount = await program.account.user.fetch(mercy_user);

    console.log(`User: ${JSON.stringify(userAccount)}`);
  });

  // it("Mercy wants to update her details", async () => {
  //   const {
  //     gender,
  //     profile_picture,
  //     bio,
  //     profession,
  //     languages_spoken,
  //     budgets,
  //   } = updateMercyUser;

  //   await program.methods
  //     .updateUser(
  //       { female: {} },
  //       profile_picture ?? null,
  //       bio ?? null,
  //       profession ?? null,
  //       languages_spoken ?? null,
  //       budgets != null ? Number(budgets) : null
  //     )
  //     .accounts({
  //       user: mercy_user,
  //     })
  //     .rpc();

  //   const userAccount = await program.account.user.fetch(mercy_user);

  //   console.log(`User: ${JSON.stringify(userAccount)}`);
  // });
});
