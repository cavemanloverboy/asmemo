#[cfg(test)]
mod tests {
    use litesvm::LiteSVM;
    use solana_sdk::{account::Account, instruction::{AccountMeta, Instruction, InstructionError}, message::Message, native_token::LAMPORTS_PER_SOL, pubkey::Pubkey, signature::Keypair, signer::Signer, transaction::{Transaction, TransactionError}};


    const ID: Pubkey = solana_sdk::pubkey!("asmemoasmemoasmemoasmemoasmemoasmemoasmemoa");

    #[test]
    fn test_valid_ascii_no_accounts() { 
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

    #[test]
    fn test_valid_ascii_two_accounts() { 
        let mut svm = LiteSVM::new().with_sigverify(false).with_blockhash_check(false);
        svm.add_program_from_file(ID, "deploy/asmemo.so").unwrap();
        let keypair = Keypair::new();
        let payer = keypair.pubkey();
        svm.airdrop(&payer, 100 *LAMPORTS_PER_SOL).unwrap();

        // Initialize second account with data
        let first_account = payer;
        let second_account = Pubkey::new_unique();
        svm.set_account(
            second_account,
            Account::new_data(LAMPORTS_PER_SOL, &[42_u8; 17], &Pubkey::default()).unwrap()).unwrap();

        const MEMO: &str = "why does spl memo use 36000 cus to print len 60 msg of ascii";
        let instruction = Instruction::new_with_bytes(
            ID,
            MEMO.as_bytes(),
            vec![AccountMeta::new_readonly(first_account, true), AccountMeta::new_readonly(second_account, true)]
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

    #[test]
    fn test_fail_nonsigner() { 
        let mut svm = LiteSVM::new().with_sigverify(false).with_blockhash_check(false);
        svm.add_program_from_file(ID, "deploy/asmemo.so").unwrap();
        let keypair = Keypair::new();
        let payer = keypair.pubkey();
        svm.airdrop(&payer, 100 *LAMPORTS_PER_SOL).unwrap();

        // Initialize second account with data
        let second_account = Pubkey::new_unique();
        const MEMO: &str = "why does spl memo use 36000 cus to print len 60 msg of ascii";
        let instruction = Instruction::new_with_bytes(
            ID,
            MEMO.as_bytes(),
            vec![AccountMeta::new_readonly(second_account, false)]
        );

        let message = Message::new(&[instruction], Some(&payer));
        let transaction = Transaction::new_unsigned(message);
        
        let result = svm.send_transaction(
            transaction,
        ).unwrap_err();
    
        assert!(matches!(result.err, TransactionError::InstructionError(0, InstructionError::Custom(69))));
      
    }

    // TODO fail duplicate account
    #[test]
    fn test_fail_duplicate() { 
        let mut svm = LiteSVM::new().with_sigverify(false).with_blockhash_check(false);
        svm.add_program_from_file(ID, "deploy/asmemo.so").unwrap();
        let keypair = Keypair::new();
        let payer = keypair.pubkey();
        svm.airdrop(&payer, 100 *LAMPORTS_PER_SOL).unwrap();

        // Initialize second account with data
        let second_account = Pubkey::new_unique();
        const MEMO: &str = "why does spl memo use 36000 cus to print len 60 msg of ascii";
        let instruction = Instruction::new_with_bytes(
            ID,
            MEMO.as_bytes(),
            vec![AccountMeta::new_readonly(second_account, true); 2]
        );

        let message = Message::new(&[instruction], Some(&payer));
        let transaction = Transaction::new_unsigned(message);
        
        let result = svm.send_transaction(
            transaction,
        ).unwrap_err();
    
        assert!(matches!(result.err, TransactionError::InstructionError(0, InstructionError::Custom(420))));
      
    }
}



