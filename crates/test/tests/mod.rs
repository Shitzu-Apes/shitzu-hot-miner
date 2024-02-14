mod util;

use util::*;

use near_sdk::NearToken;

#[tokio::test]
async fn test_all() -> anyhow::Result<()> {
    let fee = 10_000_000_000_000_000_000;
    let (worker, owner, contract, test_token) = initialize_contracts(fee).await?;

    let user_a = worker.dev_create_account().await?;
    let user_b = worker.dev_create_account().await?;

    tokio::try_join!(
        call::storage_deposit(&contract, &user_a, None, None),
        call::storage_deposit(&contract, &user_b, None, None),
        call::storage_deposit(
            &test_token,
            contract.as_account(),
            None,
            Some(NearToken::from_millinear(100)),
        ),
        call::storage_deposit(
            &test_token,
            &owner,
            None,
            Some(NearToken::from_millinear(100)),
        ),
        call::storage_deposit(
            &test_token,
            &user_a,
            None,
            Some(NearToken::from_millinear(100)),
        ),
        call::storage_deposit(
            &test_token,
            &user_b,
            None,
            Some(NearToken::from_millinear(100)),
        )
    )?;
    call::mint_tokens(&test_token, user_a.id(), fee * 10).await?;
    call::mint_tokens(&test_token, user_b.id(), fee * 5).await?;

    let balance = view::ft_balance_of(&test_token, user_a.id()).await?;
    assert_eq!(balance.0, fee * 10);
    let balance = view::ft_balance_of(&test_token, user_b.id()).await?;
    assert_eq!(balance.0, fee * 5);

    call::deposit(&user_a, test_token.id(), contract.id(), fee).await?;
    call::deposit(&user_b, test_token.id(), contract.id(), fee * 5).await?;

    let balance = view::ft_balance_of(&test_token, user_a.id()).await?;
    assert_eq!(balance.0, fee * 9);
    let balance = view::ft_balance_of(&test_token, user_b.id()).await?;
    assert_eq!(balance.0, 0);
    let balance = view::ft_balance_of(&test_token, contract.id()).await?;
    assert_eq!(balance.0, fee * 6);
    let balance = view::get_balance(&contract, user_a.id()).await?;
    assert_eq!(balance.0, fee);
    let balance = view::get_balance(&contract, user_b.id()).await?;
    assert_eq!(balance.0, fee * 5);

    call::withdraw(&contract, &user_a, fee).await?;
    call::withdraw(&contract, &user_b, fee * 2).await?;

    let balance = view::ft_balance_of(&test_token, user_a.id()).await?;
    assert_eq!(balance.0, fee * 10);
    let balance = view::ft_balance_of(&test_token, user_b.id()).await?;
    assert_eq!(balance.0, fee * 2);
    let balance = view::ft_balance_of(&test_token, contract.id()).await?;
    assert_eq!(balance.0, fee * 3);
    let balance = view::get_balance(&contract, user_a.id()).await?;
    assert_eq!(balance.0, 0);
    let balance = view::get_balance(&contract, user_b.id()).await?;
    assert_eq!(balance.0, fee * 3);
    let fees = view::get_collected_fees(&contract).await?;
    assert_eq!(fees.0, 0);

    call::deposit(&user_b, test_token.id(), contract.id(), fee * 2).await?;
    let res = call::deduct_fee(&contract, user_a.id()).await;
    assert!(res.is_err());

    call::deduct_fee(&contract, user_b.id()).await?;
    call::deduct_fee(&contract, user_b.id()).await?;
    call::deduct_fee(&contract, user_b.id()).await?;
    call::deduct_fee(&contract, user_b.id()).await?;
    call::deduct_fee(&contract, user_b.id()).await?;

    let balance = view::ft_balance_of(&test_token, user_a.id()).await?;
    assert_eq!(balance.0, fee * 10);
    let balance = view::ft_balance_of(&test_token, user_b.id()).await?;
    assert_eq!(balance.0, 0);
    let balance = view::ft_balance_of(&test_token, contract.id()).await?;
    assert_eq!(balance.0, fee * 5);
    let balance = view::get_balance(&contract, user_a.id()).await?;
    assert_eq!(balance.0, 0);
    let balance = view::get_balance(&contract, user_b.id()).await?;
    assert_eq!(balance.0, 0);
    let fees = view::get_collected_fees(&contract).await?;
    assert_eq!(fees.0, fee * 5);

    call::refund_fee(&contract, user_b.id()).await?;

    let balance = view::get_balance(&contract, user_a.id()).await?;
    assert_eq!(balance.0, 0);
    let balance = view::get_balance(&contract, user_b.id()).await?;
    assert_eq!(balance.0, fee);
    let fees = view::get_collected_fees(&contract).await?;
    assert_eq!(fees.0, fee * 4);

    let res = call::claim_fees(&contract).await;
    assert!(res.is_err());

    let balance = view::ft_balance_of(&test_token, user_a.id()).await?;
    assert_eq!(balance.0, fee * 10);
    let balance = view::ft_balance_of(&test_token, user_b.id()).await?;
    assert_eq!(balance.0, 0);
    let balance = view::ft_balance_of(&test_token, contract.id()).await?;
    assert_eq!(balance.0, fee * 5);
    let balance = view::ft_balance_of(&test_token, owner.id()).await?;
    assert_eq!(balance.0, 0);
    let balance = view::get_balance(&contract, user_a.id()).await?;
    assert_eq!(balance.0, 0);
    let balance = view::get_balance(&contract, user_b.id()).await?;
    assert_eq!(balance.0, fee);
    let fees = view::get_collected_fees(&contract).await?;
    assert_eq!(fees.0, fee * 4);

    call::mint_tokens(&test_token, user_b.id(), fee * 5).await?;
    call::deposit(&user_b, test_token.id(), contract.id(), fee * 5).await?;
    call::deduct_fee(&contract, user_b.id()).await?;
    call::deduct_fee(&contract, user_b.id()).await?;
    call::deduct_fee(&contract, user_b.id()).await?;
    call::claim_fees(&contract).await?;

    let balance = view::ft_balance_of(&test_token, user_a.id()).await?;
    assert_eq!(balance.0, fee * 10);
    let balance = view::ft_balance_of(&test_token, user_b.id()).await?;
    assert_eq!(balance.0, 0);
    let balance = view::ft_balance_of(&test_token, contract.id()).await?;
    assert_eq!(balance.0, fee * 8);
    let balance = view::ft_balance_of(&test_token, owner.id()).await?;
    assert_eq!(balance.0, fee * 2);
    let balance = view::get_balance(&contract, user_a.id()).await?;
    assert_eq!(balance.0, 0);
    let balance = view::get_balance(&contract, user_b.id()).await?;
    assert_eq!(balance.0, fee * 3);
    let fees = view::get_collected_fees(&contract).await?;
    assert_eq!(fees.0, fee * 5);

    Ok(())
}
