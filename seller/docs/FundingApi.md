# \FundingApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_funding_apply**](FundingApi.md#create_funding_apply) | **POST** /v1/funding/apply | Apply for Crossly Funding. Decided at once on the checklist rules: approved opens your line (limit = 25% of 90 days of settled Crossly payouts), declined lists why, needs_review goes to a person.
[**create_funding_draw**](FundingApi.md#create_funding_draw) | **POST** /v1/funding/draws | Draw on your line for a supplier order. Send the supplier, the invoice (an uploaded invoiceFileId from POST /v1/funding/invoice-files, a reference, or a URL) and the APR from the quote you accepted (acceptedAprBps); refused if your rate changed. The advance is held as in-platform Crossly balance, never cash. Idempotent on idempotencyKey.
[**create_funding_draw_approve**](FundingApi.md#create_funding_draw_approve) | **POST** /v1/funding/draws/{id}/approve | Approve a held draw. The two-person rule needs two distinct approvers; the requester never approves their own.
[**create_funding_draw_quote**](FundingApi.md#create_funding_draw_quote) | **POST** /v1/funding/draws/quote | The disclosure for a draw of amountCents: APR, the simple-interest fee for the full term, the total and the due date, plus your available headroom. Nothing is drawn.
[**create_funding_draw_reject**](FundingApi.md#create_funding_draw_reject) | **POST** /v1/funding/draws/{id}/reject | Reject a held draw. Nothing was drawn, so nothing is owed.
[**create_funding_draw_use**](FundingApi.md#create_funding_draw_use) | **POST** /v1/funding/draws/use | Request a draw for an approved use. Refused by the spend controls, held for approval (stage requested — no debt until disbursed), or disbursed at once as restricted funding CBX. A DEBT with interest: show the disclosure first.
[**create_funding_invoice_file**](FundingApi.md#create_funding_invoice_file) | **POST** /v1/funding/invoice-files | Sign an upload for a supplier invoice: a PDF or image (application/pdf, image/jpeg, image/png, image/webp, image/heic) of at most 15 MB. PUT the bytes to uploadUrl with the same Content-Type and exactly contentLength bytes, then draw with invoiceFileId = file.id. Stored privately; one draw per file.
[**create_funding_pay_crossly_fee**](FundingApi.md#create_funding_pay_crossly_fee) | **POST** /v1/funding/pay/crossly-fees | Pay an open invoice of your Crossly plan with restricted funding CBX (no card is charged; the invoice is marked paid). Idempotent per invoice.
[**create_funding_pay_shipping_label**](FundingApi.md#create_funding_pay_shipping_label) | **POST** /v1/funding/pay/shipping-labels | Top up your shipping wallet with restricted funding CBX; labels you buy are then debited from it. Idempotent on idempotencyKey. Refused when the spend controls forbid shipping labels or it is over your approval threshold with no approved draw.
[**create_funding_pay_vault_fee**](FundingApi.md#create_funding_pay_vault_fee) | **POST** /v1/funding/pay/vault-fees | Ship a stored Vault unit home and pay its fee (shipping, storage owed, early withdrawal) with restricted funding CBX instead of your card.
[**create_funding_pool_stake**](FundingApi.md#create_funding_pool_stake) | **POST** /v1/funding/pool/stake | Stake earned CBX into the funding pool. Refused, with the reason, until the pool is live and staking is open.
[**create_funding_pool_unstake**](FundingApi.md#create_funding_pool_unstake) | **POST** /v1/funding/pool/unstake | Queue an unstake. Paid first-in first-out once your lockup has passed.
[**create_funding_pool_unstake_cancel**](FundingApi.md#create_funding_pool_unstake_cancel) | **POST** /v1/funding/pool/unstake/{id}/cancel | Cancel a queued unstake that has not been paid yet.
[**create_funding_pool_waitlist**](FundingApi.md#create_funding_pool_waitlist) | **POST** /v1/funding/pool/waitlist | Join the staker waitlist: told the moment staking into the funding pool opens.
[**create_funding_repay**](FundingApi.md#create_funding_repay) | **POST** /v1/funding/repay | Repay. source \"balance\" (default) takes it from your available Stripe payout balance and applies it at once. source \"card\" returns a PaymentIntent client secret to confirm with Stripe; source \"checkout\" returns checkoutUrl, a hosted Stripe Checkout page the seller pays on (no Stripe SDK needed). source \"cbx\" pays early from your EARNED CBX at the obligation rate (granted and funding CBX never repay a line) and applies at once. Card and checkout apply when the payment succeeds. Never more than you owe; idempotent on idempotencyKey.
[**delete_funding_member**](FundingApi.md#delete_funding_member) | **DELETE** /v1/funding/members/{memberUserId} | Remove a team member’s funding limit: they can no longer draw.
[**get_funding_control**](FundingApi.md#get_funding_control) | **GET** /v1/funding/controls | Your funding spend controls: approved uses, supplier allow/block lists, the approval threshold, the two-person rule and each team member’s funding limit.
[**get_funding_draw**](FundingApi.md#get_funding_draw) | **GET** /v1/funding/draws/{id} | One draw: amount, use, supplier, stage and its timeline (requested → approved → disbursed → spent → repaid).
[**get_funding_draw_invoice**](FundingApi.md#get_funding_draw_invoice) | **GET** /v1/funding/draws/{id}/invoice | A 15-minute signed link to a draw's uploaded supplier invoice, with the file's name, type and size. 404 when the draw has no uploaded file.
[**get_funding_line**](FundingApi.md#get_funding_line) | **GET** /v1/funding/line | Your line: limit, available, outstanding (principal + interest), APR, interest accrued since the last posting, the payoff figure, next due date, anything overdue, the per-draw schedule and your auto-repay share.
[**get_funding_offer**](FundingApi.md#get_funding_offer) | **GET** /v1/funding/offer | Your pre-qualified Crossly Funding limit and the eligibility checklist: each rule, whether you pass it, and what to do if not. Works with or without a line.
[**get_funding_pool**](FundingApi.md#get_funding_pool) | **GET** /v1/funding/pool | The Crossly Funding pool: live or not and why (no token yet, no pool, no price, no capacity), capacity, utilisation, the reserve, and the uses and suppliers it allows.
[**get_funding_pool_stake**](FundingApi.md#get_funding_pool_stake) | **GET** /v1/funding/pool/stake | Your position in the funding pool, the trailing APY reading (never a promise), lockup, unstake queue, history and the risk disclosure.
[**get_funding_spendable**](FundingApi.md#get_funding_spendable) | **GET** /v1/funding/spendable | What your restricted funding CBX can pay right now, in cents, for each Crossly use (shipping labels, Crossly fees, ads, Vault fees). 0 = none, or your spend controls refuse that use.
[**get_funding_statement**](FundingApi.md#get_funding_statement) | **GET** /v1/funding/statements | A monthly statement (?month=YYYY-MM, default the latest): opening balance, every draw, interest posting and payment, and the closing balance; plus the months available.
[**list_funding_approvals**](FundingApi.md#list_funding_approvals) | **GET** /v1/funding/approvals | Draws your spend controls hold for approval, with the approvals so far and how many are needed.
[**list_funding_draws**](FundingApi.md#list_funding_draws) | **GET** /v1/funding/draws | Every draw on your line: what it paid for (a supplier order, or a Crossly checkout), its APR, estimated fee and due date.
[**list_funding_receipts**](FundingApi.md#list_funding_receipts) | **GET** /v1/funding/receipts | Your funding payments to Crossly, newest first: the use, what it paid for, cents, status (pending, paid, failed) and the draw it was stamped against.
[**list_funding_repayments**](FundingApi.md#list_funding_repayments) | **GET** /v1/funding/repayments | Manual repayments (card / bank, or from your payout balance), newest first.
[**update_funding_autorepay**](FundingApi.md#update_funding_autorepay) | **PUT** /v1/funding/autorepay | Set the share of each Crossly payout applied to what you owe: a whole percent from 10 to 100 (default 20). Never more than you owe is taken.
[**update_funding_control**](FundingApi.md#update_funding_control) | **PUT** /v1/funding/controls | Change your funding spend controls. Enforced at every spend: wholesale checkout, labels, ads, Crossly and Vault fees.
[**update_funding_member**](FundingApi.md#update_funding_member) | **PUT** /v1/funding/members/{memberUserId} | Set a team member’s funding limit (per draw or per month).



## create_funding_apply

> crate::models::CreateFundingApplyResponse create_funding_apply()
Apply for Crossly Funding. Decided at once on the checklist rules: approved opens your line (limit = 25% of 90 days of settled Crossly payouts), declined lists why, needs_review goes to a person.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateFundingApplyResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_funding_draw

> crate::models::CreateFundingDrawResponse create_funding_draw()
Draw on your line for a supplier order. Send the supplier, the invoice (an uploaded invoiceFileId from POST /v1/funding/invoice-files, a reference, or a URL) and the APR from the quote you accepted (acceptedAprBps); refused if your rate changed. The advance is held as in-platform Crossly balance, never cash. Idempotent on idempotencyKey.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateFundingDrawResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_funding_draw_approve

> crate::models::CreateFundingDrawRejectResponse create_funding_draw_approve(id)
Approve a held draw. The two-person rule needs two distinct approvers; the requester never approves their own.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateFundingDrawRejectResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_funding_draw_quote

> crate::models::CreateFundingDrawQuoteResponse create_funding_draw_quote()
The disclosure for a draw of amountCents: APR, the simple-interest fee for the full term, the total and the due date, plus your available headroom. Nothing is drawn.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateFundingDrawQuoteResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_funding_draw_reject

> crate::models::CreateFundingDrawRejectResponse create_funding_draw_reject(id)
Reject a held draw. Nothing was drawn, so nothing is owed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateFundingDrawRejectResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_funding_draw_use

> crate::models::CreateFundingDrawUseResponse create_funding_draw_use()
Request a draw for an approved use. Refused by the spend controls, held for approval (stage requested — no debt until disbursed), or disbursed at once as restricted funding CBX. A DEBT with interest: show the disclosure first.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateFundingDrawUseResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_funding_invoice_file

> crate::models::CreateFundingInvoiceFileResponse create_funding_invoice_file()
Sign an upload for a supplier invoice: a PDF or image (application/pdf, image/jpeg, image/png, image/webp, image/heic) of at most 15 MB. PUT the bytes to uploadUrl with the same Content-Type and exactly contentLength bytes, then draw with invoiceFileId = file.id. Stored privately; one draw per file.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateFundingInvoiceFileResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_funding_pay_crossly_fee

> crate::models::CreateFundingPayCrosslyFeeResponse create_funding_pay_crossly_fee()
Pay an open invoice of your Crossly plan with restricted funding CBX (no card is charged; the invoice is marked paid). Idempotent per invoice.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateFundingPayCrosslyFeeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_funding_pay_shipping_label

> crate::models::CreateFundingPayShippingLabelResponse create_funding_pay_shipping_label()
Top up your shipping wallet with restricted funding CBX; labels you buy are then debited from it. Idempotent on idempotencyKey. Refused when the spend controls forbid shipping labels or it is over your approval threshold with no approved draw.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateFundingPayShippingLabelResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_funding_pay_vault_fee

> crate::models::CreateFundingPayVaultFeeResponse create_funding_pay_vault_fee()
Ship a stored Vault unit home and pay its fee (shipping, storage owed, early withdrawal) with restricted funding CBX instead of your card.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateFundingPayVaultFeeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_funding_pool_stake

> crate::models::CreateFundingPoolStakeResponse create_funding_pool_stake()
Stake earned CBX into the funding pool. Refused, with the reason, until the pool is live and staking is open.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateFundingPoolStakeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_funding_pool_unstake

> crate::models::FundingPoolResponse create_funding_pool_unstake()
Queue an unstake. Paid first-in first-out once your lockup has passed.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::FundingPoolResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_funding_pool_unstake_cancel

> crate::models::FundingPoolResponse create_funding_pool_unstake_cancel(id)
Cancel a queued unstake that has not been paid yet.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::FundingPoolResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_funding_pool_waitlist

> crate::models::FundingPoolResponse create_funding_pool_waitlist()
Join the staker waitlist: told the moment staking into the funding pool opens.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::FundingPoolResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_funding_repay

> crate::models::CreateFundingRepayResponse create_funding_repay()
Repay. source \"balance\" (default) takes it from your available Stripe payout balance and applies it at once. source \"card\" returns a PaymentIntent client secret to confirm with Stripe; source \"checkout\" returns checkoutUrl, a hosted Stripe Checkout page the seller pays on (no Stripe SDK needed). source \"cbx\" pays early from your EARNED CBX at the obligation rate (granted and funding CBX never repay a line) and applies at once. Card and checkout apply when the payment succeeds. Never more than you owe; idempotent on idempotencyKey.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateFundingRepayResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_funding_member

> crate::models::DeleteFundingMemberResponse delete_funding_member(member_user_id)
Remove a team member’s funding limit: they can no longer draw.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**member_user_id** | **String** |  | [required] |

### Return type

**crate::models::DeleteFundingMemberResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_funding_control

> crate::models::GetFundingControlResponse get_funding_control()
Your funding spend controls: approved uses, supplier allow/block lists, the approval threshold, the two-person rule and each team member’s funding limit.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetFundingControlResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_funding_draw

> crate::models::GetFundingDrawResponse get_funding_draw(id)
One draw: amount, use, supplier, stage and its timeline (requested → approved → disbursed → spent → repaid).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetFundingDrawResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_funding_draw_invoice

> crate::models::GetFundingDrawInvoiceResponse get_funding_draw_invoice(id)
A 15-minute signed link to a draw's uploaded supplier invoice, with the file's name, type and size. 404 when the draw has no uploaded file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetFundingDrawInvoiceResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_funding_line

> crate::models::GetFundingLineResponse get_funding_line()
Your line: limit, available, outstanding (principal + interest), APR, interest accrued since the last posting, the payoff figure, next due date, anything overdue, the per-draw schedule and your auto-repay share.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetFundingLineResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_funding_offer

> crate::models::GetFundingOfferResponse get_funding_offer()
Your pre-qualified Crossly Funding limit and the eligibility checklist: each rule, whether you pass it, and what to do if not. Works with or without a line.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetFundingOfferResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_funding_pool

> crate::models::GetFundingPoolResponse get_funding_pool()
The Crossly Funding pool: live or not and why (no token yet, no pool, no price, no capacity), capacity, utilisation, the reserve, and the uses and suppliers it allows.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetFundingPoolResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_funding_pool_stake

> crate::models::FundingPoolResponse get_funding_pool_stake()
Your position in the funding pool, the trailing APY reading (never a promise), lockup, unstake queue, history and the risk disclosure.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::FundingPoolResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_funding_spendable

> crate::models::GetFundingSpendableResponse get_funding_spendable()
What your restricted funding CBX can pay right now, in cents, for each Crossly use (shipping labels, Crossly fees, ads, Vault fees). 0 = none, or your spend controls refuse that use.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetFundingSpendableResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_funding_statement

> crate::models::GetFundingStatementResponse get_funding_statement()
A monthly statement (?month=YYYY-MM, default the latest): opening balance, every draw, interest posting and payment, and the closing balance; plus the months available.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetFundingStatementResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_funding_approvals

> crate::models::ListFundingApprovalsResponse list_funding_approvals()
Draws your spend controls hold for approval, with the approvals so far and how many are needed.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListFundingApprovalsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_funding_draws

> crate::models::ListFundingDrawsResponse list_funding_draws()
Every draw on your line: what it paid for (a supplier order, or a Crossly checkout), its APR, estimated fee and due date.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListFundingDrawsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_funding_receipts

> crate::models::ListFundingReceiptsResponse list_funding_receipts()
Your funding payments to Crossly, newest first: the use, what it paid for, cents, status (pending, paid, failed) and the draw it was stamped against.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListFundingReceiptsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_funding_repayments

> crate::models::ListFundingRepaymentsResponse list_funding_repayments()
Manual repayments (card / bank, or from your payout balance), newest first.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListFundingRepaymentsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_funding_autorepay

> crate::models::UpdateFundingAutorepayResponse update_funding_autorepay()
Set the share of each Crossly payout applied to what you owe: a whole percent from 10 to 100 (default 20). Never more than you owe is taken.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::UpdateFundingAutorepayResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_funding_control

> crate::models::UpdateFundingControlResponse update_funding_control()
Change your funding spend controls. Enforced at every spend: wholesale checkout, labels, ads, Crossly and Vault fees.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::UpdateFundingControlResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_funding_member

> crate::models::UpdateFundingMemberResponse update_funding_member(member_user_id)
Set a team member’s funding limit (per draw or per month).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**member_user_id** | **String** |  | [required] |

### Return type

**crate::models::UpdateFundingMemberResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

