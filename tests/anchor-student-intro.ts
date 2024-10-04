import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorStudentIntro } from "../target/types/anchor_student_intro";
import { expect } from "chai";

describe("anchor-student-intro", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // let's create default values to be added
  const student = {
    name: "Weebad Neqqa",
    
    short_message: "I am gonna be fucking rich!",
  }


  const program = anchor.workspace.AnchorStudentIntro as Program<AnchorStudentIntro>;


  // Derive the student PDA
  const [studentPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from(student.name), provider.wallet.publicKey.toBuffer()],
    program.programId,
  );

  
  it("Student Details ADDED to records successfully!", async () => {

    // Add the test here
    const tx = await program.methods
      .addStudentDetails(student.name, student.short_message)
      .rpc();

    const account = await program.account.studentIntroState.fetch(studentPda);
    expect(student.name == account.name);
    expect(student.short_message == account.shortMessage);
    expect(account.student == provider.wallet.publicKey);
  });


  it("Student Details Updated successfully!", async () => {
    // write the new short message
    const new_short_message = "I will fucking murder you!";

    const tx = await program.methods
      .updateStudentDetails(student.name, new_short_message)
      .rpc();

    const account = await program.account.studentIntroState.fetch(studentPda);
    expect(student.name == account.name);
    expect(new_short_message == account.shortMessage);
    expect(account.student == provider.wallet.publicKey);
  });

  
  it("Student Details in Records cleared successfully!", async () => {

    // delete student account details
    const tx = await program.methods
      .closeStudentDetails(student.name)
      .rpc();
  })
});
