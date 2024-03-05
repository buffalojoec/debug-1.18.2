#![cfg(feature = "test-sbf")]

use {
    repro_1_18_2::NUM_KEYS,
    solana_program_test::*,
    solana_sdk::{
        instruction::{AccountMeta, Instruction},
        message::Message,
        pubkey::Pubkey,
        signature::Signer,
        signer::keypair::Keypair,
        transaction::Transaction,
    },
    std::collections::HashMap,
};

#[tokio::test]
async fn test() {
    let program_test = ProgramTest::new("repro_1_18_2", repro_1_18_2::id(), None);
    let mut context = program_test.start_with_context().await;

    let recent_blockhash = context.last_blockhash;
    let payer = context.payer.insecure_clone();
    let keypairs: HashMap<Pubkey, Keypair> = (0..NUM_KEYS)
        .map(|_| Keypair::new())
        .map(|keypair| (keypair.pubkey(), keypair))
        .collect();

    let instruction = {
        let mut metas = vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new_readonly(solana_program::system_program::id(), false),
        ];
        metas.extend(
            keypairs
                .iter()
                .map(|(pubkey, _)| AccountMeta::new(*pubkey, true)),
        );
        Instruction::new_with_bytes(repro_1_18_2::id(), &[], metas)
    };

    let message = {
        let mut message = Message::new(&[instruction], Some(&payer.pubkey()));
        message.recent_blockhash = recent_blockhash;
        message
    };

    let transaction = {
        let message_data = message.serialize();
        let signatures = message
            .signer_keys()
            .iter()
            .map(|pubkey| {
                if let Some(keypair) = keypairs.get(pubkey) {
                    return keypair.try_sign_message(&message_data).unwrap();
                } else {
                    return payer.try_sign_message(&message_data).unwrap();
                }
            })
            .collect::<Vec<_>>();
        let mut transaction = Transaction::new_unsigned(message);
        transaction.signatures = signatures;
        transaction
    };

    context
        .banks_client
        .process_transaction(transaction)
        .await
        .unwrap();
}
