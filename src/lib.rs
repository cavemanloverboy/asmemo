#[cfg(test)]
mod tests {
    use litesvm::LiteSVM;
    use solana_sdk::{instruction::Instruction, message::Message, native_token::LAMPORTS_PER_SOL, pubkey::Pubkey, signature::Keypair, signer::Signer, transaction::Transaction};


    const ID: Pubkey = solana_sdk::pubkey!("asmemoasmemoasmemoasmemoasmemoasmemoasmemoa");

    #[test]
    fn test_valid_ascii() { 
        let mut svm = LiteSVM::new().with_sigverify(false).with_blockhash_check(false);
        svm.add_program_from_file(ID, "deploy/asmemo.so").unwrap();
        let keypair = Keypair::new();
        let payer = keypair.pubkey();
        svm.airdrop(&payer, 100 *LAMPORTS_PER_SOL).unwrap();


        const MEMO: &str = "why does spl memo use 36000 cus to print len 60 msg of ascii";
        let instruction = Instruction::new_with_bytes(
            ID,
            MEMO.as_bytes(),
            vec![]
        );

        let message = Message::new(&[instruction], Some(&payer));
        let transaction = Transaction::new_unsigned(message);
        
        let result = svm.send_transaction(
            transaction,
        ).unwrap();
        for log in result.logs {
            println!("    {log}");
        }
    }
}