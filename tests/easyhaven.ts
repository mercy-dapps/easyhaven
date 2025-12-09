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

  const mercy_owner_host = PublicKey.findProgramAddressSync(
    [Buffer.from("owner"), user.publicKey.toBuffer()],
    program.programId
  )[0];

  const mercy_buyer = PublicKey.findProgramAddressSync(
    [Buffer.from("buyer"), user.publicKey.toBuffer()],
    program.programId
  )[0];

  const mercy_user = PublicKey.findProgramAddressSync(
    [Buffer.from("user"), user.publicKey.toBuffer()],
    program.programId
  )[0];

  const allAccounts = {
    user: mercy_user,
    owner: mercy_owner_host,
    userKey: user.publicKey,
    buyer: mercy_buyer,
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

    interest_properties: ["Self-contain", "room and parlour"],
    locations_preferred: ["akobo", "bodija"],
    budgets: "2000000",

    languages_spoken: [],
  };

  it("airdrop", async () => {
    await Promise.all([owner, user].map((a) => airdropSol(a.publicKey, 1e9)));
  });

  it("Create Mercy user", async () => {
    const { name, email, phone_number, location } = mercyUser;

    await program.methods
      .createUser(name, email, phone_number, location)
      .accounts({ ...allAccounts })
      .signers([user])
      .rpc();

    const userAccount = await program.account.user.fetch(mercy_user);

    console.log(`User: ${JSON.stringify(userAccount)}`);
  });

  it("Mercy wants to update her buyer details", async () => {
    const {
      profile_picture,
      bio,
      profession,
      interest_properties,
      locations_preferred,
      budgets,
    } = updateMercyUser;

    await program.methods
      .updateBuyerInfo(
        { female: {} },
        profile_picture ?? null,
        bio ?? null,
        profession ?? null,
        interest_properties ?? null,
        locations_preferred ?? null,
        budgets != null ? Number(budgets) : null
      )
      .accounts({
        ...allAccounts,
      })
      .signers([user])
      .rpc();

    const userAccount = await program.account.user.fetch(mercy_user);
    const buyerAccount = await program.account.buyerInfo.fetch(mercy_buyer);

    console.log(`Buyer: ${JSON.stringify(buyerAccount)}`);
    console.log(`User: ${JSON.stringify(userAccount.gender)}`);
  });

  it("Mercy wants to become a host", async () => {
    await program.methods
      .becomeAHost()
      .accountsPartial({
        ...allAccounts,
      })
      .signers([user])
      .rpc();

    const userAccount = await program.account.user.fetch(mercy_user);

    console.log(`User: ${JSON.stringify(userAccount)}`);
  });

  it("Mercy wants to update her owner details", async () => {
    const {
      profile_picture,
      bio,
      profession,

      languages_spoken,
    } = updateMercyUser;

    await program.methods
      .updateOwnerInfo(
        { male: {} },
        profile_picture ?? null,
        bio ?? null,
        profession ?? null,

        languages_spoken ?? null
      )
      .accounts({
        ...allAccounts,
      })
      .signers([user])
      .rpc();

    const userAccount = await program.account.user.fetch(mercy_user);
    const ownerAccount = await program.account.ownerInfo.fetch(mercy_owner_host);

    console.log(`Owner: ${JSON.stringify(ownerAccount)}`);
    console.log(`User: ${JSON.stringify(userAccount)}`);
  });
});
