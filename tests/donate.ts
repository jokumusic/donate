import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { publicKey } from "@project-serum/anchor/dist/cjs/utils";
import { BN } from "bn.js";
import { expect } from "chai";
import { Donate } from "../target/types/donate";


describe("", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Donate as Program<Donate>;  
  const [summaryPda, summaryPdaBump] = publicKey
  .findProgramAddressSync([anchor.utils.bytes.utf8.encode("donation-summary"), program.programId.toBuffer()], program.programId);

  describe("[Initialize]", () =>{
    it("Setup Donation Summary PDA", async () => {
      const nullableSummary = await program.account.donationSummary.fetchNullable(summaryPda);
      if(nullableSummary != null) {
        console.log("Summary PDA already exists. Skipping initialization...");
        return;
      }

      const tx = await program.methods
        .setup()
        .accounts({
          summary: summaryPda,
          payer: provider.publicKey, 
        })
        .rpc();

        const summary = await program.account.donationSummary.fetch(summaryPda);    
        expect(summary.donations.toNumber()).to.equal(0);
        expect(summary.total.toNumber()).to.equal(0);
        
        console.log("Your transaction signature", tx);
        //console.log("summary=", summary);
    });
});


describe("[Update]", () =>{
  it("Send Donation", async () => {

    const summaryBeforeDonation  = await program.account.donationSummary.fetch(summaryPda);
    const [donorPda, donorPdaBump] = publicKey
    .findProgramAddressSync([
        anchor.utils.bytes.utf8.encode("donation"), 
        anchor.utils.bytes.utf8.encode(summaryBeforeDonation.donations.toNumber().toString()),
        provider.publicKey.toBuffer()
      ], program.programId);
  
    const donationAmount = Math.floor(Math.random() * 200) + 1;
    const tx = await program.methods
      .donate(donationAmount)
      .accounts({
        donation: donorPda,
        donor: provider.publicKey,
        summary: summaryPda,
      })
      .rpc();      

      const donation = await program.account.donation.fetch(donorPda);  
      const summaryAfterDonation = await program.account.donationSummary.fetch(summaryPda);    
      expect(summaryAfterDonation.donations.toNumber()).to.equal(summaryBeforeDonation.donations.toNumber() + 1);
      expect(summaryAfterDonation.total.toNumber()).to.equal(summaryBeforeDonation.total.toNumber() + donationAmount);
      
      console.log("Your transaction signature", tx);
      console.log("donation=", donation);
      console.log("summary (total is ", summaryAfterDonation.total.toNumber(), "} = ", summaryAfterDonation);
  });
});

});
