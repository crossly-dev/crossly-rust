# \CBXApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_cbx_accrual**](CBXApi.md#create_cbx_accrual) | **POST** /v1/cbx/accruals | Record cashback a user earned, in cents.
[**create_cbx_accrual_purchase**](CBXApi.md#create_cbx_accrual_purchase) | **POST** /v1/cbx/accruals/purchase | Accrue cashback for an order at the resolved rate.
[**create_cbx_accrual_reverse**](CBXApi.md#create_cbx_accrual_reverse) | **POST** /v1/cbx/accruals/{accrualId}/reverse | Claw back a pending accrual — a refund, a cancellation, fraud.
[**create_cbx_ad_credit_purchase**](CBXApi.md#create_cbx_ad_credit_purchase) | **POST** /v1/cbx/ad-credit/purchase | Claim ad credit against a CBX transfer you sent.
[**create_cbx_ad_credit_quote**](CBXApi.md#create_cbx_ad_credit_quote) | **POST** /v1/cbx/ad-credit/quote | What a given number of tokens buys in ad credit.
[**create_cbx_ad_credit_spend**](CBXApi.md#create_cbx_ad_credit_spend) | **POST** /v1/cbx/ad-credit/spend | Consume credit for a billing period.
[**create_cbx_boost**](CBXApi.md#create_cbx_boost) | **POST** /v1/cbx/boosts | Fund elevated cashback on matching items.
[**create_cbx_boost_pause**](CBXApi.md#create_cbx_boost_pause) | **POST** /v1/cbx/boosts/{boostId}/pause | Stop a boost from matching further orders.
[**create_cbx_campaign**](CBXApi.md#create_cbx_campaign) | **POST** /v1/cbx/campaigns | Create a campaign in draft.
[**create_cbx_campaign_approve**](CBXApi.md#create_cbx_campaign_approve) | **POST** /v1/cbx/campaigns/{campaignId}/approve | Approve the previewed recipient list.
[**create_cbx_campaign_execute**](CBXApi.md#create_cbx_campaign_execute) | **POST** /v1/cbx/campaigns/{campaignId}/execute | Pay an approved campaign.
[**create_cbx_campaign_preview**](CBXApi.md#create_cbx_campaign_preview) | **POST** /v1/cbx/campaigns/{campaignId}/preview | Compute the recipient list without paying it.
[**create_cbx_claim**](CBXApi.md#create_cbx_claim) | **POST** /v1/cbx/claims | Reserve a claim. Debits the balance and queues the transfer.
[**create_cbx_claim_quote**](CBXApi.md#create_cbx_claim_quote) | **POST** /v1/cbx/claims/quote | What a claim would cost, without committing to it.
[**create_cbx_claim_send**](CBXApi.md#create_cbx_claim_send) | **POST** /v1/cbx/claims/{claimId}/send | Send a reserved claim on chain.
[**create_cbx_credit_draw**](CBXApi.md#create_cbx_credit_draw) | **POST** /v1/cbx/credit/draw | Draw against a line, receiving grant credit.
[**create_cbx_credit_freeze**](CBXApi.md#create_cbx_credit_freeze) | **POST** /v1/cbx/credit/freeze | Stop new draws. Leaves the drawn balance on its terms.
[**create_cbx_credit_refresh**](CBXApi.md#create_cbx_credit_refresh) | **POST** /v1/cbx/credit/refresh | Recompute a limit from trading history and stake.
[**create_cbx_credit_repay**](CBXApi.md#create_cbx_credit_repay) | **POST** /v1/cbx/credit/repay | Apply a repayment to a line.
[**create_cbx_disbursement_rule**](CBXApi.md#create_cbx_disbursement_rule) | **POST** /v1/cbx/disbursement-rules | Fire a distribution when the events pool crosses a threshold.
[**create_cbx_disbursement_rule_active**](CBXApi.md#create_cbx_disbursement_rule_active) | **POST** /v1/cbx/disbursement-rules/{ruleId}/active | Enable or disable a rule.
[**create_cbx_disbursement_rule_check**](CBXApi.md#create_cbx_disbursement_rule_check) | **POST** /v1/cbx/disbursement-rules/{ruleId}/check | Evaluate a rule now. Fires it if every gate passes.
[**create_cbx_earn_tier**](CBXApi.md#create_cbx_earn_tier) | **POST** /v1/cbx/earn-tiers | Define an earn term.
[**create_cbx_rate_quote**](CBXApi.md#create_cbx_rate_quote) | **POST** /v1/cbx/rates/quote | What would this order earn, and why.
[**create_cbx_redemption**](CBXApi.md#create_cbx_redemption) | **POST** /v1/cbx/redemptions | Pay for a service in CBX.
[**create_cbx_redemption_quote**](CBXApi.md#create_cbx_redemption_quote) | **POST** /v1/cbx/redemptions/quote | What a service costs in tokens right now.
[**create_cbx_revenue_sweep**](CBXApi.md#create_cbx_revenue_sweep) | **POST** /v1/cbx/revenue/sweep | Move accrued revenue from the reserve to your revenue wallet.
[**create_cbx_spend**](CBXApi.md#create_cbx_spend) | **POST** /v1/cbx/spends | Redeem a user's CBX against an order.
[**create_cbx_spend_reverse**](CBXApi.md#create_cbx_spend_reverse) | **POST** /v1/cbx/spends/{externalId}/reverse | Refund a spend — give the tokens back and claw the skim back.
[**create_cbx_stake_tier**](CBXApi.md#create_cbx_stake_tier) | **POST** /v1/cbx/stake-tiers | Define a staking tier.
[**create_cbx_subject**](CBXApi.md#create_cbx_subject) | **POST** /v1/cbx/subjects | Map one of your user ids to a CBX subject.
[**create_cbx_subject_grant**](CBXApi.md#create_cbx_subject_grant) | **POST** /v1/cbx/subjects/{subjectId}/grants | Issue grant credit — in-platform, non-withdrawable.
[**create_cbx_subject_spend_plan**](CBXApi.md#create_cbx_subject_spend_plan) | **POST** /v1/cbx/subjects/{subjectId}/spend-plan | Which balances would pay for a spend, and in what order.
[**create_cbx_subject_stake**](CBXApi.md#create_cbx_subject_stake) | **POST** /v1/cbx/subjects/{subjectId}/stake | Lock a subject's tokens for a tier.
[**create_cbx_subject_stake_unstake**](CBXApi.md#create_cbx_subject_stake_unstake) | **POST** /v1/cbx/subjects/{subjectId}/stake/unstake | Start the cooldown. Tokens unlock when it elapses.
[**create_cbx_wallet_challenge**](CBXApi.md#create_cbx_wallet_challenge) | **POST** /v1/cbx/wallets/challenge | Start wallet verification. Returns a message for the user to sign.
[**create_cbx_wallet_payment_confirm**](CBXApi.md#create_cbx_wallet_payment_confirm) | **POST** /v1/cbx/wallet-payments/confirm | Present the signature. Returns a ship / do-not-ship decision.
[**create_cbx_wallet_payment_quote**](CBXApi.md#create_cbx_wallet_payment_quote) | **POST** /v1/cbx/wallet-payments/quote | Build a transfer for the buyer to sign themselves.
[**create_cbx_wallet_payment_resolve**](CBXApi.md#create_cbx_wallet_payment_resolve) | **POST** /v1/cbx/wallet-payments/{paymentId}/resolve | A human decides on a held payment.
[**create_cbx_wallet_verify**](CBXApi.md#create_cbx_wallet_verify) | **POST** /v1/cbx/wallets/verify | Complete wallet verification with the user's signature.
[**get_cbx_ad_credit**](CBXApi.md#get_cbx_ad_credit) | **GET** /v1/cbx/ad-credit | Unspent advertising credit, in cents.
[**get_cbx_claim**](CBXApi.md#get_cbx_claim) | **GET** /v1/cbx/claims/{claimId} | A claim's current state.
[**get_cbx_credit**](CBXApi.md#get_cbx_credit) | **GET** /v1/cbx/credit | A seller's wholesale credit line.
[**get_cbx_me**](CBXApi.md#get_cbx_me) | **GET** /v1/cbx/me | Identity check — which merchant this key belongs to, and its terms.
[**get_cbx_pool**](CBXApi.md#get_cbx_pool) | **GET** /v1/cbx/pool | Your events-pool balance.
[**get_cbx_revenue**](CBXApi.md#get_cbx_revenue) | **GET** /v1/cbx/revenue | Operator revenue accrued and not yet withdrawn.
[**get_cbx_subject_balance**](CBXApi.md#get_cbx_subject_balance) | **GET** /v1/cbx/subjects/{subjectId}/balance | What a subject holds: pending cents and available CBX.
[**get_cbx_subject_balance_by_subject_id**](CBXApi.md#get_cbx_subject_balance_by_subject_id) | **GET** /v1/cbx/subjects/{subjectId}/balances | All three balances a subject holds.
[**get_cbx_subject_spent**](CBXApi.md#get_cbx_subject_spent) | **GET** /v1/cbx/subjects/{subjectId}/spent | Total CBX a subject has spent in your marketplace.
[**get_cbx_subject_stake**](CBXApi.md#get_cbx_subject_stake) | **GET** /v1/cbx/subjects/{subjectId}/stake | A subject's staking state and spendable balance.
[**get_cbx_subject_wallet**](CBXApi.md#get_cbx_subject_wallet) | **GET** /v1/cbx/subjects/{subjectId}/wallet | The verified payout address for a subject, if any.
[**get_cbx_treasury**](CBXApi.md#get_cbx_treasury) | **GET** /v1/cbx/treasury | Your most recent reserve reconciliation.
[**list_cbx_ad_credit_ledger**](CBXApi.md#list_cbx_ad_credit_ledger) | **GET** /v1/cbx/ad-credit/ledger | Ad-credit movements, newest first.
[**list_cbx_boosts**](CBXApi.md#list_cbx_boosts) | **GET** /v1/cbx/boosts | Your funded cashback boosts, newest first.
[**list_cbx_campaign_payouts**](CBXApi.md#list_cbx_campaign_payouts) | **GET** /v1/cbx/campaigns/{campaignId}/payouts | What a campaign actually paid, with the weight behind each amount.
[**list_cbx_campaigns**](CBXApi.md#list_cbx_campaigns) | **GET** /v1/cbx/campaigns | Your campaigns, newest first.
[**list_cbx_disbursement_progress**](CBXApi.md#list_cbx_disbursement_progress) | **GET** /v1/cbx/disbursement-progress | How close each rule is to firing — the public counter.
[**list_cbx_disbursement_rules**](CBXApi.md#list_cbx_disbursement_rules) | **GET** /v1/cbx/disbursement-rules | Threshold rules that fire community distributions.
[**list_cbx_earn_tiers**](CBXApi.md#list_cbx_earn_tiers) | **GET** /v1/cbx/earn-tiers | Earn terms on offer — longer maturation, higher rate.
[**list_cbx_redemption_services**](CBXApi.md#list_cbx_redemption_services) | **GET** /v1/cbx/redemptions/services | Services payable in CBX, and the discount each carries.
[**list_cbx_stake_tiers**](CBXApi.md#list_cbx_stake_tiers) | **GET** /v1/cbx/stake-tiers | Staking tiers — what locking tokens buys.
[**list_cbx_subject_grants**](CBXApi.md#list_cbx_subject_grants) | **GET** /v1/cbx/subjects/{subjectId}/grants | Live grants, soonest-expiring first.
[**list_cbx_subject_ledger**](CBXApi.md#list_cbx_subject_ledger) | **GET** /v1/cbx/subjects/{subjectId}/ledger | A subject's CBX ledger, newest first.
[**list_cbx_wallet_payment_review**](CBXApi.md#list_cbx_wallet_payment_review) | **GET** /v1/cbx/wallet-payments/review | Payments held for a human — the ops queue.



## create_cbx_accrual

> crate::models::CreateCbxAccrualResponse create_cbx_accrual()
Record cashback a user earned, in cents.

The accrual is PENDING until its maturation window closes, then converts to CBX at that moment's market price. It is denominated in cents the whole time it is pending, deliberately: quoting a token quantity up front and buying later would leave the reserve short for the length of the window. Send sourceExternalId and a retry is a no-op rather than a double credit.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateCbxAccrualResponse**](CreateCbxAccrualResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_cbx_accrual_purchase

> crate::models::CreateCbxAccrualPurchaseResponse create_cbx_accrual_purchase()
Accrue cashback for an order at the resolved rate.

Prefer this over POST /accruals when you want us to do the rate maths — it resolves the tier, the boost and the stake boost, charges your boost budget in the same transaction, and records what rate was actually granted. If a matching boost cannot fund the order, the accrual falls back to your un-boosted rate and `boostBudgetExhausted` is true. Idempotent on externalId: a retried webhook neither accrues twice nor charges your budget twice.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateCbxAccrualPurchaseResponse**](CreateCbxAccrualPurchaseResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_cbx_accrual_reverse

> crate::models::CreateCbxAccrualReverseResponse create_cbx_accrual_reverse(accrual_id)
Claw back a pending accrual — a refund, a cancellation, fraud.

Only works while the accrual is still pending. Once it has converted, the value is tokens in somebody's balance and this is the wrong operation: reversing then is a debit against that balance, which is a different act with different consequences and is not something an integration key can do. This is why the maturation window has to be at least as long as your refund window.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**accrual_id** | **String** |  | [required] |

### Return type

[**crate::models::CreateCbxAccrualReverseResponse**](CreateCbxAccrualReverseResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_cbx_ad_credit_purchase

> crate::models::CreateCbxAdCreditPurchaseResponse create_cbx_ad_credit_purchase()
Claim ad credit against a CBX transfer you sent.

Send CBX to our revenue wallet yourself, then present the signature. We read the actual balance delta at FINALIZED commitment — a confirmed transaction can still be dropped by a fork, and this grants real credit. One signature can be claimed exactly once. Priced at the spot when the claim is processed, not when you signed: pricing at send time would let somebody hold signed transfers and claim only the ones that moved in their favour. A 409 means the transaction has not finalized yet and you should retry; a 400 means it will never be claimable (failed, wrong mint, wrong destination).

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateCbxAdCreditPurchaseResponse**](CreateCbxAdCreditPurchaseResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_cbx_ad_credit_quote

> crate::models::CreateCbxAdCreditQuoteResponse create_cbx_ad_credit_quote()
What a given number of tokens buys in ad credit.

Credit is 1:1 with the market value of the tokens at confirmation. Refuses with 409 when there is no fresh price — pricing an entire prepaid budget off a guess is not something to do quietly.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateCbxAdCreditQuoteResponse**](CreateCbxAdCreditQuoteResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_cbx_ad_credit_spend

> crate::models::CreateCbxAdCreditSpendResponse create_cbx_ad_credit_spend()
Consume credit for a billing period.

Spends what the balance covers and reports the rest as `shortfallCents` — the campaign should stop there rather than running on credit that does not exist. 20% of what is spent moves to the community events pool and 80% is operator revenue; the split happens on SPEND rather than at purchase, because the pool's share is earned when the advertising is actually delivered. Idempotent on externalId: pass your billing-period id so a retried run does not consume the same credit twice.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateCbxAdCreditSpendResponse**](CreateCbxAdCreditSpendResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_cbx_boost

> crate::models::CreateCbxBoostResponse create_cbx_boost()
Fund elevated cashback on matching items.

Performance marketing priced in CBX: the spend lands as a durable balance the buyer returns to use, rather than a one-time price cut they pocket. A boost REPLACES the base or tier rate rather than adding to it — you are stating the total you will pay, and it is priced against your margin. Targeting is matched by equality on one facet; most specific wins (sku > collection > category > all). Kinds: all, category, sku, collection.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateCbxBoostResponse**](CreateCbxBoostResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_cbx_boost_pause

> crate::models::CreateCbxBoostPauseResponse create_cbx_boost_pause(boost_id)
Stop a boost from matching further orders.

Pausing does not refund anything — already-accrued cashback is a promise already made to a buyer, and unwinding it would take back cashback somebody was shown at checkout. The remaining budget simply stops being spendable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**boost_id** | **String** |  | [required] |

### Return type

[**crate::models::CreateCbxBoostPauseResponse**](CreateCbxBoostPauseResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_cbx_campaign

> crate::models::CreateCbxCampaignResponse create_cbx_campaign()
Create a campaign in draft.

Nothing is paid until it is previewed, approved and executed. Available metrics: accruals_count, accrued_cents, spend_count, spend_base_units — all of them measure activity inside the window. `capPerSubject` is worth setting on a proportional campaign: without it one large participant can take almost the whole pool, which makes the event pointless for everybody else.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateCbxCampaignResponse**](CreateCbxCampaignResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_cbx_campaign_approve

> crate::models::CreateCbxCampaignApproveResponse create_cbx_campaign_approve(campaign_id)
Approve the previewed recipient list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | **String** |  | [required] |

### Return type

[**crate::models::CreateCbxCampaignApproveResponse**](CreateCbxCampaignApproveResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_cbx_campaign_execute

> crate::models::CreateCbxCampaignExecuteResponse create_cbx_campaign_execute(campaign_id)
Pay an approved campaign.

Recomputes the list and refuses if the hash no longer matches the approved one. Payouts credit balances directly rather than transferring on chain, so a distribution to ten thousand recipients costs one internal move and is reversible if it was computed wrong.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | **String** |  | [required] |

### Return type

[**crate::models::CreateCbxCampaignExecuteResponse**](CreateCbxCampaignExecuteResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_cbx_campaign_preview

> crate::models::CreateCbxCampaignPreviewResponse create_cbx_campaign_preview(campaign_id)
Compute the recipient list without paying it.

Returns every recipient and amount, plus a hash of the list. Re-previewing invalidates any prior approval by design — the approver signed off on a specific list, and if it has changed they have not approved what would now happen.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | **String** |  | [required] |

### Return type

[**crate::models::CreateCbxCampaignPreviewResponse**](CreateCbxCampaignPreviewResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_cbx_claim

> crate::models::CreateCbxClaimResponse create_cbx_claim()
Reserve a claim. Debits the balance and queues the transfer.

The balance is debited here, before anything is sent, which is the only safe order: sending first and debiting after means a crash between the two pays somebody and never charges them, and that is unrecoverable. A crash after this leaves a reserved balance and a pending claim, which is recoverable by looking at the chain. Send the same idempotencyKey to retry safely.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateCbxClaimResponse**](CreateCbxClaimResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_cbx_claim_quote

> crate::models::CreateCbxClaimQuoteResponse create_cbx_claim_quote()
What a claim would cost, without committing to it.

Every fee at cost, so a confirm screen can show the breakdown before the user agrees. The network fee is passed through at actual cost and includes the one-time account rent when the recipient has no token account yet — that rent is a recoverable deposit on an account the USER owns, not a fee we keep.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateCbxClaimQuoteResponse**](CreateCbxClaimQuoteResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_cbx_claim_send

> crate::models::CreateCbxClaimSendResponse create_cbx_claim_send(claim_id)
Send a reserved claim on chain.

Re-checks every precondition rather than trusting the reservation: the delegation may have been revoked, the fee payer may have run dry, the address may have been flagged since. A response of `unconfirmed` means the transfer may have landed but confirmation was not observed — do NOT retry it, it needs reconciliation against the chain first.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**claim_id** | **String** |  | [required] |

### Return type

[**crate::models::CreateCbxClaimSendResponse**](CreateCbxClaimSendResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_cbx_credit_draw

> crate::models::CreateCbxCreditDrawResponse create_cbx_credit_draw()
Draw against a line, receiving grant credit.

The advance lands as GRANT balance: in-platform only, so it cannot be withdrawn, cannot be turned into cash and absconded with, and adds nothing to the float that could hit an order book. Restricted to wholesale channels — the limit was sized on the theory that the advance buys goods that get sold and generate the payout stream repaying it, and credit spent on a subscription does not create that stream. Refuses if the reserve has no unallocated tokens: credit is real value and cannot be advanced against tokens that do not exist. Idempotent on externalId.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateCbxCreditDrawResponse**](CreateCbxCreditDrawResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_cbx_credit_freeze

> crate::models::CreateCbxCreditFreezeResponse create_cbx_credit_freeze()
Stop new draws. Leaves the drawn balance on its terms.

The only lever over a line, and deliberately the only one. A seller who took inventory on Tuesday keeps Tuesday's terms whatever the token does on Wednesday — the only way a credit product sits next to a volatile asset without transmitting its volatility.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateCbxCreditFreezeResponse**](CreateCbxCreditFreezeResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_cbx_credit_refresh

> crate::models::CreateCbxCreditRefreshResponse create_cbx_credit_refresh()
Recompute a limit from trading history and stake.

The earned limit is a share of trailing SETTLED payout volume — money that actually reached the seller, not listed inventory or projected sales. The stake bonus is capped at a share of that, so a seller with no history gets nothing however much they stake. A frozen line stays frozen: freezing is a credit decision somebody made, and a recompute must not quietly undo it.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateCbxCreditRefreshResponse**](CreateCbxCreditRefreshResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_cbx_credit_repay

> crate::models::CreateCbxCreditRepayResponse create_cbx_credit_repay()
Apply a repayment to a line.

`treasury` scope, which looks backwards next to a draw needing only `spend` and is deliberate: a forged repayment writes off real money owed to us, while a forged draw hands out credit spendable only in our own marketplace. The scope follows the loss. Clamped to what is outstanding — a payout larger than the debt would otherwise push the balance negative and read as credit nobody underwrote.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateCbxCreditRepayResponse**](CreateCbxCreditRepayResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_cbx_disbursement_rule

> crate::models::CreateCbxDisbursementRuleResponse create_cbx_disbursement_rule()
Fire a distribution when the events pool crosses a threshold.

Applies to the EVENTS POOL only — never free reserve surplus. Reserve surplus is the recirculation buffer that funds the next accrual without touching the market, so distributing it would force us to buy the same tokens back at spread plus MEV. `checkCadenceHours` bounds how often the rule may fire even when the pool is over the line: a pure threshold fires at an unpredictable moment, and the pool jumps most after a lapse sweep — precisely when engagement was worst. `distributeBps` is capped under 100% because draining the pool removes the standing balance that makes the next event credible. The rule decides WHEN only: firing opens a campaign that still needs preview, approval and execution. Metrics: accruals_count, accrued_cents, spend_count, spend_base_units.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateCbxDisbursementRuleResponse**](CreateCbxDisbursementRuleResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_cbx_disbursement_rule_active

> crate::models::CreateCbxDisbursementRuleActiveResponse create_cbx_disbursement_rule_active(rule_id)
Enable or disable a rule.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule_id** | **String** |  | [required] |

### Return type

[**crate::models::CreateCbxDisbursementRuleActiveResponse**](CreateCbxDisbursementRuleActiveResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_cbx_disbursement_rule_check

> crate::models::CreateCbxDisbursementRuleCheckResponse create_cbx_disbursement_rule_check(rule_id)
Evaluate a rule now. Fires it if every gate passes.

Gates in order: cadence, threshold, coverage. `outcome` names the one that stopped it — every evaluation is recorded including the declines, because \"why didn't the event happen\" is the question people ask and the answer is always a check that ran and said no. A missing or stale treasury snapshot declines on `coverage`: unknown coverage is not healthy coverage. Send dryRun to evaluate without opening a campaign or writing anything.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule_id** | **String** |  | [required] |

### Return type

[**crate::models::CreateCbxDisbursementRuleCheckResponse**](CreateCbxDisbursementRuleCheckResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_cbx_earn_tier

> crate::models::CreateCbxEarnTierResponse create_cbx_earn_tier()
Define an earn term.

Setting `isDefault` moves the default off whatever held it — exactly one active default per merchant is enforced by a unique index, because two would make \"what rate did this user get\" depend on row order.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateCbxEarnTierResponse**](CreateCbxEarnTierResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_cbx_rate_quote

> crate::models::CreateCbxRateQuoteResponse create_cbx_rate_quote()
What would this order earn, and why.

Resolve the rate without accruing anything, so a checkout can show the real number and its reason. `source` tells you whether it came from your base rate, a tier, or a boost; `stakeBoostBps` is the part the user's own stake contributed. Quoting does NOT reserve boost budget — a quote and the subsequent accrual can differ if the budget runs out in between, which is why the accrual response repeats the rate it actually granted.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateCbxRateQuoteResponse**](CreateCbxRateQuoteResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_cbx_redemption

> crate::models::CreateCbxRedemptionResponse create_cbx_redemption()
Pay for a service in CBX.

Idempotent on (serviceKind, externalId) rather than externalId alone: a grading submission and a listing boost can legitimately share an id because they refer to the same item, and without the service in the key, boosting a listing you had already graded would return the grading receipt and never charge for the boost. Staked tokens cannot pay — the debit checks spendable balance, not total.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateCbxRedemptionResponse**](CreateCbxRedemptionResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_cbx_redemption_quote

> crate::models::CreateCbxRedemptionQuoteResponse create_cbx_redemption_quote()
What a service costs in tokens right now.

Refuses with 409 when there is no fresh price. A dollar-priced service has no honest token quantity without a spot, and there is no safe direction to guess in — a guess either overcharges the user or undercharges us.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateCbxRedemptionQuoteResponse**](CreateCbxRedemptionQuoteResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_cbx_revenue_sweep

> crate::models::CreateCbxRevenueSweepResponse create_cbx_revenue_sweep()
Move accrued revenue from the reserve to your revenue wallet.

Computes what is genuinely free — reserve minus outstanding balances, minus claims in flight, minus the pool, minus unswept revenue — and moves at most that. If the reserve is short it moves NOTHING, whatever the ledger says: an under-covered reserve is not a reason to stop paying users, it is a reason to stop paying yourself. Send dryRun to see the arithmetic without moving tokens.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateCbxRevenueSweepResponse**](CreateCbxRevenueSweepResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_cbx_spend

> crate::models::CreateCbxSpendResponse create_cbx_spend()
Redeem a user's CBX against an order.

Debits the user exactly what they spent — the skim is never added on top, because making CBX worth less when used than when sold would invert the whole reason to spend rather than liquidate. The skim comes out of YOUR fee on the order and is capped against it, so an order paid entirely in saved-up CBX does not have its whole margin eaten. Idempotent on externalId: a retried checkout returns the original spend rather than debiting twice. Spending is always free and has no minimum — that asymmetry against the withdrawal fee is what steers toward spending without ever telling anyone they cannot have their money.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateCbxSpendResponse**](CreateCbxSpendResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_cbx_spend_reverse

> crate::models::CreateCbxSpendReverseResponse create_cbx_spend_reverse(external_id)
Refund a spend — give the tokens back and claw the skim back.

Returns the user's tokens AND reverses the skim out of both the community pool and operator revenue. All three move together: returning the tokens while the pool and operator kept their shares would count the same tokens twice against one reserve. The spend also stops counting as activity for campaign weighting, so buy-then-refund cannot farm distributions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_id** | **String** |  | [required] |

### Return type

[**crate::models::CreateCbxSpendReverseResponse**](CreateCbxSpendReverseResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_cbx_stake_tier

> crate::models::CreateCbxStakeTierResponse create_cbx_stake_tier()
Define a staking tier.

A user gets the highest tier their amount clears. `cooldownDays` is what makes the discount real: without a wait, a user stakes for the discount and unstakes the moment they want to withdraw, so the commitment it was priced against never existed.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateCbxStakeTierResponse**](CreateCbxStakeTierResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_cbx_subject

> crate::models::CreateCbxSubjectResponse create_cbx_subject()
Map one of your user ids to a CBX subject.

Idempotent. Call it whenever you need a subject id; repeated calls with the same externalUserId return the same subject. Your user ids are opaque to us and unique only within your merchant, so two marketplaces can both have a user \"1\" without collision.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateCbxSubjectResponse**](CreateCbxSubjectResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_cbx_subject_grant

> crate::models::CreateCbxSubjectGrantResponse create_cbx_subject_grant(subject_id)
Issue grant credit — in-platform, non-withdrawable.

Requires a funding `batchId` for every kind except `grant_makegood`. Grant credit is spendable at merchants who receive real value, so the tokens have to exist — the same rule earned balance obeys. A makegood is exempt because compensating somebody for our failure must not be blocked on treasury state. There is no path that converts a grant to earned balance or pays it to an address, and the database enforces that rather than a comment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject_id** | **String** |  | [required] |

### Return type

[**crate::models::CreateCbxSubjectGrantResponse**](CreateCbxSubjectGrantResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_cbx_subject_spend_plan

> crate::models::CreateCbxSubjectSpendPlanResponse create_cbx_subject_spend_plan(subject_id)
Which balances would pay for a spend, and in what order.

The order is granted → earned → connected and you do not get to choose it. Granted first is a security property, not a preference: if earned spent first, a subject holding both would spend their withdrawable balance down while their non-withdrawable grant sat untouched — converting a grant into a withdrawable balance one purchase at a time. Returns a `shortfallBaseUnits` rather than failing, so a checkout can charge the remainder to a card. Send excludeConnected on a flow that cannot wait for an on-chain transfer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject_id** | **String** |  | [required] |

### Return type

[**crate::models::CreateCbxSubjectSpendPlanResponse**](CreateCbxSubjectSpendPlanResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_cbx_subject_stake

> crate::models::CreateCbxSubjectStakeResponse create_cbx_subject_stake(subject_id)
Lock a subject's tokens for a tier.

Refuses an amount that clears no tier — locking tokens for no benefit is never what somebody meant to do. One stake per subject: to change the amount, unstake and wait out the cooldown first.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject_id** | **String** |  | [required] |

### Return type

[**crate::models::CreateCbxSubjectStakeResponse**](CreateCbxSubjectStakeResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_cbx_subject_stake_unstake

> crate::models::CreateCbxSubjectStakeUnstakeResponse create_cbx_subject_stake_unstake(subject_id)
Start the cooldown. Tokens unlock when it elapses.

The earn boost stops immediately; the tokens stay locked until `unlocksAt`. The cooldown is read from the tier as configured NOW, which is the one place a later config change is allowed to matter — holding somebody to a longer wait the merchant has since abandoned would be the worse behaviour.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject_id** | **String** |  | [required] |

### Return type

[**crate::models::CreateCbxSubjectStakeUnstakeResponse**](CreateCbxSubjectStakeUnstakeResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_cbx_wallet_challenge

> crate::models::CreateCbxWalletChallengeResponse create_cbx_wallet_challenge()
Start wallet verification. Returns a message for the user to sign.

Present the returned `message` verbatim to the user's wallet for signing. It binds your merchant, their subject, the address and a single-use nonce, so the resulting signature is not transferable to another address or account. A signature is required because there is no custody here: a send cannot be undone, so a typo or a swapped address is permanent.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateCbxWalletChallengeResponse**](CreateCbxWalletChallengeResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_cbx_wallet_payment_confirm

> crate::models::CreateCbxWalletPaymentConfirmResponse create_cbx_wallet_payment_confirm()
Present the signature. Returns a ship / do-not-ship decision.

Everything is read from the CHAIN at finalized commitment — the amount, the payer, the destination. Nothing you assert about the payment is trusted, because a client that can state its own payment amount can state a larger one. The response `releaseDecision` is about the ORDER, not the payment: by the time we see a signature the tokens have moved and cannot be un-moved, so the only decision left is whether to hand over goods. `release` means ship. `review` means hold — a person needs to look, and that includes the case where no screening provider is configured. `refuse` means do not ship; the payment is still recorded, because we received the tokens and that fact does not go away. Idempotent twice over: on txSig globally, so one payment cannot pay two orders, and on (merchant, externalId), so one order is not paid twice. A 409 means the transaction has not finalized yet and you should retry; a 400 means it never will be claimable.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateCbxWalletPaymentConfirmResponse**](CreateCbxWalletPaymentConfirmResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_cbx_wallet_payment_quote

> crate::models::CreateCbxWalletPaymentQuoteResponse create_cbx_wallet_payment_quote()
Build a transfer for the buyer to sign themselves.

Returns an unsigned, base64 transaction. The buyer's wallet signs and submits it; we never hold a key or a delegation and never submit anything, so the platform has no authority over their tokens at any point. RESERVES NOTHING — no row, no hold, no balance change. The buyer may never sign it. The SIGNATURE is the event, so treat this as a convenience and not a commitment. `lastValidBlockHeight` is when it expires: a wallet prompt left open for a couple of minutes produces a transaction the chain will reject, and you should re-quote rather than retry. `payerCanCover` is a courtesy read of their balance so you can warn before a prompt rather than after a failure; null means we could not read it, which is not the same as \"no\". The buyer needs no prior wallet registration: a payment proves control of the tokens, which is what a connect-and-verify step would have been proving.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateCbxWalletPaymentQuoteResponse**](CreateCbxWalletPaymentQuoteResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_cbx_wallet_payment_resolve

> crate::models::CreateCbxWalletPaymentResolveResponse create_cbx_wallet_payment_resolve(payment_id)
A human decides on a held payment.

Only moves a payment OUT of `review`, never between the other two. A refusal that could later be flipped to a release is an approval control with no teeth, and a release re-decided as a refusal after the goods shipped is a record that no longer describes what happened. The reviewer and their note are stored on the row, because this is the decision somebody will be asked to justify.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payment_id** | **String** |  | [required] |

### Return type

[**crate::models::CreateCbxWalletPaymentResolveResponse**](CreateCbxWalletPaymentResolveResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_cbx_wallet_verify

> crate::models::CreateCbxWalletVerifyResponse create_cbx_wallet_verify()
Complete wallet verification with the user's signature.

The signature is checked against the message WE issued and stored, never one supplied here. The nonce is single-use, so the same signature cannot re-verify an address later.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateCbxWalletVerifyResponse**](CreateCbxWalletVerifyResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cbx_ad_credit

> crate::models::GetCbxAdCreditResponse get_cbx_ad_credit()
Unspent advertising credit, in cents.

Advertising is payable in CBX and nothing else. Your budget is denominated in dollars, priced at the spot when your payment finalized — deliberately NOT held as a token quantity, since a price move would otherwise silently change the budget you prepaid, making you a market participant because you bought ads. Credit is 1:1 with market value: with CBX the only way to pay there is nothing to discount against. Spendable on advertising only, and not refundable.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetCbxAdCreditResponse**](GetCbxAdCreditResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cbx_claim

> crate::models::GetCbxClaimResponse get_cbx_claim(claim_id)
A claim's current state.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**claim_id** | **String** |  | [required] |

### Return type

[**crate::models::GetCbxClaimResponse**](GetCbxClaimResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cbx_credit

> crate::models::GetCbxCreditResponse get_cbx_credit()
A seller's wholesale credit line.

Trade credit, not token-collateralized lending. The line is secured by receivables we already hold — the payout stream sits under a hold with an exposure ceiling — and secondarily by goods bought from our own wholesale channel. CBX is the alignment mechanism, not the collateral: a stake raises the limit and lowers the rate, bounded to a share of the earned limit so a price collapse can never remove the majority of a facility. The limit may FALL. A drawn balance is never accelerated or margin-called — there is no liquidation engine, no keeper and no oracle trigger anywhere in it.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetCbxCreditResponse**](GetCbxCreditResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cbx_me

> crate::models::GetCbxMeResponse get_cbx_me()
Identity check — which merchant this key belongs to, and its terms.

Returns the economics your merchant is configured with, so an integration can display accurate terms rather than hard-coding ours. Note `claimsEnabled`: off means balances are store credit that cannot leave, which is the default and the smaller regulatory posture.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetCbxMeResponse**](GetCbxMeResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cbx_pool

> crate::models::GetCbxPoolResponse get_cbx_pool()
Your events-pool balance.

Funded by the skim on in-marketplace CBX spending, the community share of withdrawal fees, and lapsed balances. Sponsor budgets are tracked separately and are not included here.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetCbxPoolResponse**](GetCbxPoolResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cbx_revenue

> crate::models::GetCbxRevenueResponse get_cbx_revenue()
Operator revenue accrued and not yet withdrawn.

Your share of the spend skim plus your half of withdrawal fees, denominated in CBX. It sits inside the reserve until swept, which is why it is tracked here rather than inferred: without a number saying how much of the reserve is yours, there is no safe amount to take out.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetCbxRevenueResponse**](GetCbxRevenueResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cbx_subject_balance

> crate::models::GetCbxSubjectBalanceResponse get_cbx_subject_balance(subject_id)
What a subject holds: pending cents and available CBX.

Two numbers because they are two different things. `pendingCents` is cashback earned but still inside its window — reversible, denominated in dollars, not yet tokens. `availableBaseUnits` is CBX they hold now. Amounts are strings because a token balance can exceed what a JSON number represents exactly.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject_id** | **String** |  | [required] |

### Return type

[**crate::models::GetCbxSubjectBalanceResponse**](GetCbxSubjectBalanceResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cbx_subject_balance_by_subject_id

> crate::models::GetCbxSubjectBalanceBySubjectIdResponse get_cbx_subject_balance_by_subject_id(subject_id)
All three balances a subject holds.

EARNED is cashback and affiliate accruals — withdrawable once matured and above the claim floor. GRANTED is ad credit, wholesale draws and promos — spendable in your marketplace only, never withdrawable, so it creates no sell pressure. CONNECTED is the subject's own self-custodied CBX, reachable through a bounded delegation; it was never our liability and does not enter the reserve invariant. `connectedAvailableBaseUnits` is delegation HEADROOM, not a wallet balance — the subject may hold less than they approved, or have revoked on chain without telling us, so treat it as a ceiling and let the spend re-read the chain.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject_id** | **String** |  | [required] |

### Return type

[**crate::models::GetCbxSubjectBalanceBySubjectIdResponse**](GetCbxSubjectBalanceBySubjectIdResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cbx_subject_spent

> crate::models::GetCbxSubjectSpentResponse get_cbx_subject_spent(subject_id)
Total CBX a subject has spent in your marketplace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject_id** | **String** |  | [required] |

### Return type

[**crate::models::GetCbxSubjectSpentResponse**](GetCbxSubjectSpentResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cbx_subject_stake

> crate::models::GetCbxSubjectStakeResponse get_cbx_subject_stake(subject_id)
A subject's staking state and spendable balance.

`availableBaseUnits` is the number a checkout must use — balance minus anything locked. `earnBoostBps` reads zero once an unstake has been requested, because the boost ends with the commitment; `feeDiscountBps` survives the cooldown, since withdrawing is exactly what somebody in cooldown is trying to do.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject_id** | **String** |  | [required] |

### Return type

[**crate::models::GetCbxSubjectStakeResponse**](GetCbxSubjectStakeResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cbx_subject_wallet

> crate::models::GetCbxSubjectWalletResponse get_cbx_subject_wallet(subject_id)
The verified payout address for a subject, if any.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject_id** | **String** |  | [required] |

### Return type

[**crate::models::GetCbxSubjectWalletResponse**](GetCbxSubjectWalletResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cbx_treasury

> crate::models::GetCbxTreasuryResponse get_cbx_treasury()
Your most recent reserve reconciliation.

Coverage is your reserve measured against what you owe your users, including claims already in flight. Below 100% your conversions stop — we will not credit balances that nothing backs. Claims and spends keep working, because those move value out and improve coverage.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetCbxTreasuryResponse**](GetCbxTreasuryResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_cbx_ad_credit_ledger

> crate::models::V1List list_cbx_ad_credit_ledger()
Ad-credit movements, newest first.

Append-only. `purchase_cbx` and `grant` add; `ad_spend` and `expire` subtract. There is no refund kind and there will not be one: advertising credit exists to buy advertising, and any exit at face value turns it into a currency exchange.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::V1List**](V1List.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_cbx_boosts

> crate::models::V1List list_cbx_boosts()
Your funded cashback boosts, newest first.

`spentCents` against `budgetCents` is the live burn. The budget is a hard ceiling enforced inside the accrual transaction, so a boost cannot overspend — when it runs out, matching orders quietly fall back to your base rate rather than failing or accruing zero.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::V1List**](V1List.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_cbx_campaign_payouts

> crate::models::V1List list_cbx_campaign_payouts(campaign_id)
What a campaign actually paid, with the weight behind each amount.

The weight is kept so a payout can be explained to the person who received it. \"Why did I get this much\" should have an answer that is not \"the algorithm\".

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | **String** |  | [required] |

### Return type

[**crate::models::V1List**](V1List.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_cbx_campaigns

> crate::models::V1List list_cbx_campaigns()
Your campaigns, newest first.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::V1List**](V1List.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_cbx_disbursement_progress

> crate::models::V1List list_cbx_disbursement_progress()
How close each rule is to firing — the public counter.

Safe to show users. A climbing counter toward a known number is the reason to prefer cadence-plus-threshold over a pure threshold: people can see the pool rising and know roughly when the next event is possible. A trigger nobody can anticipate generates suspicion rather than anticipation.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::V1List**](V1List.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_cbx_disbursement_rules

> crate::models::V1List list_cbx_disbursement_rules()
Threshold rules that fire community distributions.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::V1List**](V1List.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_cbx_earn_tiers

> crate::models::V1List list_cbx_earn_tiers()
Earn terms on offer — longer maturation, higher rate.

A term structure on a rebate, not a yield: the user chooses WHEN to be paid for a purchase they already made, and a longer wait earns more. Nothing accrues to a balance for being held. Show these at checkout so the choice is the user's — the rate is snapshot onto the accrual, so a tier edited later never reprices a promise already made.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::V1List**](V1List.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_cbx_redemption_services

> crate::models::V1List list_cbx_redemption_services()
Services payable in CBX, and the discount each carries.

Paying in CBX costs less than paying in dollars, which is what makes anybody choose it. These are real services with real cost behind them rather than a sink invented to soak up supply — and sinks matter: supply is fixed, so tokens that only ever accumulate starve the market the protocol has to buy from.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::V1List**](V1List.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_cbx_stake_tiers

> crate::models::V1List list_cbx_stake_tiers()
Staking tiers — what locking tokens buys.

Staking pays NOTHING. It confers a lower withdrawal fee and a higher earn rate on future purchases: a discount for commitment, not a return on a holding. Staked tokens never leave the user — they stay in their balance and stay backed by the reserve — they simply become unspendable until unstaked.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::V1List**](V1List.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_cbx_subject_grants

> crate::models::V1List list_cbx_subject_grants(subject_id)
Live grants, soonest-expiring first.

That ordering is the allocation order: a spend consumes the grant closest to lapsing, so value about to expire is used before value that will not. Grants with no expiry sort last.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject_id** | **String** |  | [required] |

### Return type

[**crate::models::V1List**](V1List.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_cbx_subject_ledger

> crate::models::V1List list_cbx_subject_ledger(subject_id, limit)
A subject's CBX ledger, newest first.

Append-only. The balance is the sum of these rows and there is no cached balance anywhere that could disagree with them.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject_id** | **String** |  | [required] |
**limit** | Option<**i32**> |  |  |[default to 50]

### Return type

[**crate::models::V1List**](V1List.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_cbx_wallet_payment_review

> crate::models::V1List list_cbx_wallet_payment_review()
Payments held for a human — the ops queue.

Every row here is money taken and goods not shipped, which is not a state to leave a buyer in without it appearing on a list. `riskLevel` and `riskExposures` are the verdict as recorded at the time, not re-derived — asking a provider again next month answers a different question than the one already decided.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::V1List**](V1List.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

