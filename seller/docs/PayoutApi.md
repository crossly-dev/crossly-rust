# \PayoutApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_payout_connect_onboard**](PayoutApi.md#create_payout_connect_onboard) | **POST** /v1/payout/connect/onboard | Start or resume payout-account onboarding; returns a Stripe-hosted URL.
[**create_payout_identity_link**](PayoutApi.md#create_payout_identity_link) | **POST** /v1/payouts/identity/link | Continue with Stripe: a one-time Stripe-hosted link that collects what Stripe needs.
[**create_payout_payout**](PayoutApi.md#create_payout_payout) | **POST** /v1/payout/payouts | Pay out now: instant (fee, minutes, to an eligible card/bank) or standard (free, manual schedule only).
[**get_payout_connect**](PayoutApi.md#get_payout_connect) | **GET** /v1/payout/connect | Your Stripe payout account: status, country, agreement and payout currency.
[**get_payout_estimate**](PayoutApi.md#get_payout_estimate) | **GET** /v1/payout/estimate | What one platform nets at a given price, after fees and shipping.
[**get_payout_gross_for_net**](PayoutApi.md#get_payout_gross_for_net) | **GET** /v1/payout/gross-for-net | The gross price needed to clear a target net on one platform.
[**get_payout_identity**](PayoutApi.md#get_payout_identity) | **GET** /v1/payouts/identity | Stripe's verification of your payout account: status and what Stripe still needs, in plain words.
[**get_payout_wallet**](PayoutApi.md#get_payout_wallet) | **GET** /v1/payout/wallet | Your payout wallet: balances, schedule, next payout, instant-payout quote, methods, payouts, transactions.
[**list_payout_compare**](PayoutApi.md#list_payout_compare) | **GET** /v1/payout/compare | Rank platforms by what they net at a given price. Defaults to connected ones.
[**update_payout_schedule**](PayoutApi.md#update_payout_schedule) | **PUT** /v1/payout/schedule | Change the automatic payout schedule.



## create_payout_connect_onboard

> crate::models::CreatePayoutConnectOnboardResponse create_payout_connect_onboard(create_payout_connect_onboard_request)
Start or resume payout-account onboarding; returns a Stripe-hosted URL.

country (ISO 3166-1 alpha-2) defaults to the storefront ship-from country, then US. Fixed once the account exists. Nothing is set up until a person completes the link.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_payout_connect_onboard_request** | **CreatePayoutConnectOnboardRequest** |  | [required] |

### Return type

**crate::models::CreatePayoutConnectOnboardResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_payout_identity_link

> crate::models::CreatePayoutIdentityLinkResponse create_payout_identity_link(create_payout_connect_onboard_request)
Continue with Stripe: a one-time Stripe-hosted link that collects what Stripe needs.

Creates the payout account first when there is none (country: ISO alpha-2, default the ship-from country). type is account_onboarding (Stripe shows only what is outstanding) or account_update. The link expires in minutes; ask for a new one rather than storing it. Nothing changes until a person completes it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_payout_connect_onboard_request** | **CreatePayoutConnectOnboardRequest** |  | [required] |

### Return type

**crate::models::CreatePayoutIdentityLinkResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_payout_payout

> crate::models::CreatePayoutPayoutResponse create_payout_payout(create_payout_payout_request)
Pay out now: instant (fee, minutes, to an eligible card/bank) or standard (free, manual schedule only).

amountCents defaults to everything eligible. Pass idempotencyKey so a retry is the same payout. Returns { payout, feeCents }. 409 when the account cannot be paid or instant is unavailable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_payout_payout_request** | **CreatePayoutPayoutRequest** |  | [required] |

### Return type

**crate::models::CreatePayoutPayoutResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_payout_connect

> crate::models::GetPayoutConnectResponse get_payout_connect()
Your Stripe payout account: status, country, agreement and payout currency.

status is not_started (with defaultCountry), pending, active, restricted or payouts_unavailable. Only active can be paid; payouts_unavailable means Stripe offers no payout path to the account's country yet.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetPayoutConnectResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_payout_estimate

> crate::models::GetPayoutEstimateResponse get_payout_estimate()
What one platform nets at a given price, after fees and shipping.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetPayoutEstimateResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_payout_gross_for_net

> crate::models::GetPayoutGrossForNetResponse get_payout_gross_for_net()
The gross price needed to clear a target net on one platform.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetPayoutGrossForNetResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_payout_identity

> crate::models::GetPayoutIdentityResponse get_payout_identity(fresh)
Stripe's verification of your payout account: status and what Stripe still needs, in plain words.

status is not_started, pending (Stripe is checking), action_required (dueNow lists what to add), verified (payouts on) or restricted (disabledReasonText says why). dueNow / dueLater / inReview are Stripe's requirement keys as plain-words lines ({ keys, label }); errors carry Stripe's reason. Cached up to 60 s; fresh=1 re-reads Stripe. Payouts are gated on payoutsEnabled only.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fresh** | Option<**String**> |  |  |

### Return type

**crate::models::GetPayoutIdentityResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_payout_wallet

> crate::models::GetPayoutWalletResponse get_payout_wallet()
Your payout wallet: balances, schedule, next payout, instant-payout quote, methods, payouts, transactions.

connected:false (with defaultCountry) before onboarding. balance has availableCents, pendingCents and instantAvailableCents in the account currency. instant.available says whether an instant payout can be made now, with feeCents/netCents for the full instant-available amount and reason when not.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetPayoutWalletResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_payout_compare

> crate::models::ListPayoutCompareResponse list_payout_compare()
Rank platforms by what they net at a given price. Defaults to connected ones.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListPayoutCompareResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_payout_schedule

> crate::models::UpdatePayoutScheduleResponse update_payout_schedule(update_payout_schedule_request)
Change the automatic payout schedule.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_payout_schedule_request** | **UpdatePayoutScheduleRequest** |  | [required] |

### Return type

**crate::models::UpdatePayoutScheduleResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

