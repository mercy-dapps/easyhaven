import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Easyhaven } from "../target/types/easyhaven";
import { Keypair, PublicKey } from "@solana/web3.js";
import { randomBytes } from "crypto";

describe("easyhaven", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider();

  const program = anchor.workspace.easyhaven as Program<Easyhaven>;

  const [user2, user] = Array.from({ length: 2 }, () => Keypair.generate());

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

  const seed = new anchor.BN(randomBytes(8));
  const seed2 = new anchor.BN(randomBytes(8));

  const mercy_owner_host = PublicKey.findProgramAddressSync(
    [Buffer.from("owner"), user.publicKey.toBuffer()],
    program.programId
  )[0];

  const mercy_buyer = PublicKey.findProgramAddressSync(
    [Buffer.from("buyer"), user.publicKey.toBuffer()],
    program.programId
  )[0];

  const mercy_buyer2 = PublicKey.findProgramAddressSync(
    [Buffer.from("buyer"), user2.publicKey.toBuffer()],
    program.programId
  )[0];

  const mercy_user = PublicKey.findProgramAddressSync(
    [Buffer.from("user"), user.publicKey.toBuffer()],
    program.programId
  )[0];

  const mercy_user2 = PublicKey.findProgramAddressSync(
    [Buffer.from("user"), user2.publicKey.toBuffer()],
    program.programId
  )[0];

  const mercy_property = PublicKey.findProgramAddressSync(
    [
      Buffer.from("property"),
      user.publicKey.toBuffer(),
      seed.toArrayLike(Buffer, "le", 8),
    ],
    program.programId
  )[0];

  const mercy_property2 = PublicKey.findProgramAddressSync(
    [
      Buffer.from("property"),
      user.publicKey.toBuffer(),
      seed2.toArrayLike(Buffer, "le", 8),
    ],
    program.programId
  )[0];

  const allAccounts = {
    user: mercy_user,
    owner: mercy_owner_host,
    userKey: user.publicKey,
    buyer: mercy_buyer,
    property: mercy_property,
  };

  const mercyUser = {
    name: "Mercy User",
    email: "mercy@user.com",
    phone_number: "08012345678",
    location: "Abuja, Nigeria",
  };

  const mercyProperty = {
    name: "A roof self contain",
    details:
      "A conducive and neat room self contain located in the heart of Ibadan city",
    price: 200,
    location: "Ibadan, Nigeria",
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
    await Promise.all([user2, user].map((a) => airdropSol(a.publicKey, 1e9)));
  });

  it("Create users", async () => {
    const { name, email, phone_number, location } = mercyUser;

    await program.methods
      .createUser(name, email, phone_number, location)
      .accounts({ ...allAccounts })
      .signers([user])
      .rpc();

    await program.methods
      .createUser("user2", "user2@mail.com", "09012345678", "Lagos, Nigeria")
      .accounts({
        userKey: user2.publicKey,
      })
      .signers([user2])
      .rpc();

    const userAccount = await program.account.user.fetch(mercy_user);
    const userAccount2 = await program.account.user.fetch(mercy_user2);

    console.log(
      `Created users: ${JSON.stringify(userAccount)}, ${JSON.stringify(
        userAccount2
      )}`
    );
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

    await program.methods
      .updateBuyerInfo(
        { male: {} },
        "",
        "user2 bio",
        "Artist",
        ["Duplex"],
        ["Ikoyi"],
        10000000
      )
      .accountsPartial({
        user: mercy_user2,
        buyer: mercy_buyer2,
        userKey: user2.publicKey,
      })
      .signers([user2])
      .rpc();

    const userAccount = await program.account.user.fetch(mercy_user);
    const buyerAccount = await program.account.buyerInfo.fetch(mercy_buyer);

    console.log(`Buyer: ${JSON.stringify(buyerAccount)}`);
    console.log(`User: ${JSON.stringify(userAccount.gender)}`);

    const userAccount2 = await program.account.user.fetch(mercy_user2);
    const buyerAccount2 = await program.account.buyerInfo.fetch(mercy_buyer2);

    console.log(`Buyer: ${JSON.stringify(buyerAccount2)}`);
    console.log(`User: ${JSON.stringify(userAccount2.gender)}`);
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

    console.log(`Become host: ${JSON.stringify(userAccount)}`);
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
    const ownerAccount = await program.account.ownerInfo.fetch(
      mercy_owner_host
    );

    console.log(`Owner: ${JSON.stringify(ownerAccount)}`);
    console.log(`User: ${JSON.stringify(userAccount)}`);
  });

  it("Create property", async () => {
    const { name, price, location, details } = mercyProperty;

    await program.methods
      .createProperty(
        seed,
        name,
        details,
        price,
        location,
        { rental: {} },
        { crypto: {} }
      )
      .accounts({ ...allAccounts })
      .signers([user])
      .rpc();

    const propertyAccount = await program.account.property.fetch(
      mercy_property
    );

    console.log(`Property: ${JSON.stringify(propertyAccount)}`);

    await program.methods
      .createProperty(
        seed2,
        name,
        details,
        price,
        location,
        { rental: {} },
        { crypto: {} }
      )
      .accountsPartial({
        user: mercy_user,
        owner: mercy_owner_host,
        userKey: user.publicKey,
        property: mercy_property2,
      })
      .signers([user])
      .rpc();
  });

  it("Approve property", async () => {
    await program.methods
      .approveProperty()
      .accounts({ ...allAccounts })
      .signers([user])
      .rpc();

    const appovedProperty = await program.account.property.fetch(
      mercy_property
    );

    console.log(`Approved property: ${JSON.stringify(appovedProperty)}`);
  });

  it("Review property", async () => {
    await program.methods
      .reviewProperty("This is a great property!")
      .accountsPartial({
        user: mercy_user2,
        property: mercy_property,
        userKey: user2.publicKey,
      })
      .signers([user2])
      .rpc();

    const reviewedProperty = await program.account.property.fetch(
      mercy_property
    );

    console.log(`Approved property: ${JSON.stringify(reviewedProperty)}`);
  });

  it("Rate a property", async () => {
    await program.methods
      .rateProperty(4)
      .accountsPartial({
        user: mercy_user2,
        property: mercy_property,
        userKey: user2.publicKey,
      })
      .signers([user2])
      .rpc();

    const ratedProperty = await program.account.property.fetch(mercy_property);

    console.log(`Rated property: ${JSON.stringify(ratedProperty)}`);
  });

  it("Like a property", async () => {
    await program.methods
      .likeProperty(seed)
      .accountsPartial({
        user: mercy_user2,
        property: mercy_property,
        userKey: user2.publicKey,
      })
      .signers([user2])
      .rpc();

    const likedProperty = await program.account.property.fetch(mercy_property);

    console.log(`liked property: ${JSON.stringify(likedProperty)}`);

    await program.methods
      .likeProperty(seed)
      .accountsPartial({
        user: mercy_user,
        property: mercy_property,
        userKey: user.publicKey,
      })
      .signers([user])
      .rpc();
  });

  it("Save a property", async () => {
    await program.methods
      .saveProperty(seed)
      .accountsPartial({
        user: mercy_user2,
        property: mercy_property,
        userKey: user2.publicKey,
      })
      .signers([user2])
      .rpc();

    const savedProperty = await program.account.property.fetch(mercy_property);

    console.log(`Saved property: ${JSON.stringify(savedProperty)}`);
  });

  it("fetch liked property", async () => {
    const allProperty = await program.account.property.all();

    console.log(`All property: ${JSON.stringify(allProperty.length)}`);

    const liked = allProperty.filter((p) =>
      p.account.likedPubkey.find(
        (l) => l.toBase58() === user2.publicKey.toBase58()
      )
    );

    console.log(
      `Liked property: $${JSON.stringify(liked)}, count: ${liked.length}`
    );
  });
});
