use goose::prelude::*;
use goose_eggs::{validate_and_load_static_assets, Validate};

async fn loadtest_index(user: &mut GooseUser) -> TransactionResult {
    let goose = user.get("/bems/v1/buildings/health").await.unwrap();
    let validate = &Validate::builder().status(200).text("status OK 200").build();

    validate_and_load_static_assets(user, goose, &validate)
        .await
        .unwrap();

    Ok(())
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    GooseAttack::initialize()
        .unwrap()
        .register_scenario(
            scenario!("LoadtestTransactions").register_transaction(transaction!(loadtest_index)),
        )
        .execute()
        .await
        .unwrap();
}

//13:15