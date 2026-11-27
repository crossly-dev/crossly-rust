# \WorldsMoneyApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_world_earning_payout_account_onboard**](WorldsMoneyApi.md#create_world_earning_payout_account_onboard) | **POST** /v1/worlds/earnings/payout-account/onboard | Start or resume Stripe onboarding for your creator payout account. Returns a one-time Stripe URL. Body: { country? } (fixed once the account exists).
[**create_world_experience_drop_table**](WorldsMoneyApi.md#create_world_experience_drop_table) | **POST** /v1/worlds/experiences/{experienceId}/drop-tables | Declare a drop table for a paid random item: cosmetic or in-experience outcomes only. Crossly pre-simulates and hashes it.
[**create_world_experience_product**](WorldsMoneyApi.md#create_world_experience_product) | **POST** /v1/worlds/experiences/{experienceId}/products | Create a product: price 50-100,000 cents (a private server may be free), optional per-country prices. Paid random items need an approved drop table and are never tradeable.
[**create_world_experience_product_archive**](WorldsMoneyApi.md#create_world_experience_product_archive) | **POST** /v1/worlds/experiences/{experienceId}/products/{productId}/archive | Take a product off sale for good. Owned passes and running subscriptions keep working.
[**create_world_experience_purchase**](WorldsMoneyApi.md#create_world_experience_purchase) | **POST** /v1/worlds/experiences/{experienceId}/purchases | Open a purchase sheet for a product (paid access, a pass…). Returns checkoutPath: the player pays only in the Crossly checkout.
[**create_world_experience_receipt_ack**](WorldsMoneyApi.md#create_world_experience_receipt_ack) | **POST** /v1/worlds/experiences/{experienceId}/receipts/{orderId}/ack | Acknowledge a grant your back-end made off-server. Idempotent: a second ack reports duplicate.
[**create_world_me_receipt_refund**](WorldsMoneyApi.md#create_world_me_receipt_refund) | **POST** /v1/worlds/me/receipts/{receiptId}/refund | Refund a developer product bought by mistake: within the self-refund window and before the experience granted it.
[**create_world_me_subscription_cancel**](WorldsMoneyApi.md#create_world_me_subscription_cancel) | **POST** /v1/worlds/me/subscriptions/{id}/cancel | Cancel: no more charges; the benefit lasts to the end of the paid month.
[**create_world_me_subscription_private_server_code**](WorldsMoneyApi.md#create_world_me_subscription_private_server_code) | **POST** /v1/worlds/me/subscriptions/{id}/private-server-code | A fresh access code for your private server (the previous one stops working). Join with reservedCode.
[**create_world_purchase_cancel**](WorldsMoneyApi.md#create_world_purchase_cancel) | **POST** /v1/worlds/purchases/{intentId}/cancel | Cancel an unpaid purchase sheet.
[**get_world_earning**](WorldsMoneyApi.md#get_world_earning) | **GET** /v1/worlds/earnings | Your Crossly Carnival earnings: pending, available, held, paid; next payout; what cashing out still needs.
[**get_world_earning_payout_account**](WorldsMoneyApi.md#get_world_earning_payout_account) | **GET** /v1/worlds/earnings/payout-account | Your creator payout account (your own, separate from any store payout account): not_started, pending, restricted, active or payouts_unavailable.
[**get_world_economy**](WorldsMoneyApi.md#get_world_economy) | **GET** /v1/worlds/economy | The published payout formula: splits, thresholds, scheduled changes (90-day notice) and guarantees.
[**get_world_experience_revenue**](WorldsMoneyApi.md#get_world_experience_revenue) | **GET** /v1/worlds/experiences/{experienceId}/revenue | Gross, tax, refunds, chargebacks, net and your share per product, plus pending / available / paid.
[**get_world_experience_store**](WorldsMoneyApi.md#get_world_experience_store) | **GET** /v1/worlds/experiences/{experienceId}/store | An experience's store as you see it: your regional price, odds for paid random items, what you own.
[**get_world_purchase**](WorldsMoneyApi.md#get_world_purchase) | **GET** /v1/worlds/purchases/{intentId} | A purchase sheet: item, price, tax, total, odds, status.
[**list_world_earning_affiliate**](WorldsMoneyApi.md#list_world_earning_affiliate) | **GET** /v1/worlds/earnings/affiliate | Real-goods orders attributed to your games: the fee, your 20% share, and whether it is posted.
[**list_world_earning_ledger**](WorldsMoneyApi.md#list_world_earning_ledger) | **GET** /v1/worlds/earnings/ledger | Your creator ledger, newest first: every earning and reversal. Cursor-paged.
[**list_world_earning_payouts**](WorldsMoneyApi.md#list_world_earning_payouts) | **GET** /v1/worlds/earnings/payouts | Your monthly Crossly Carnival payouts.
[**list_world_experience_drop_tables**](WorldsMoneyApi.md#list_world_experience_drop_tables) | **GET** /v1/worlds/experiences/{experienceId}/drop-tables | Declared drop tables with their odds, simulation (10^6 draws) and hash.
[**list_world_experience_products**](WorldsMoneyApi.md#list_world_experience_products) | **GET** /v1/worlds/experiences/{experienceId}/products | What an experience sells: passes, developer products, subscriptions, private servers, paid access (?includeArchived=true).
[**list_world_experience_receipts**](WorldsMoneyApi.md#list_world_experience_receipts) | **GET** /v1/worlds/experiences/{experienceId}/receipts | Every purchase: orderId, sub, product, price, net, state (paid/delivered/granted/refunded/revoked). Cursor-paged; ?since=.
[**list_world_me_subscriptions**](WorldsMoneyApi.md#list_world_me_subscriptions) | **GET** /v1/worlds/me/subscriptions | Your experience subscriptions and private servers.
[**update_world_experience_product**](WorldsMoneyApi.md#update_world_experience_product) | **PATCH** /v1/worlds/experiences/{experienceId}/products/{productId} | Edit a product: name, description, icon, price, regional prices, hourly limit, on/off sale. Kind and place never change.



## create_world_earning_payout_account_onboard

> crate::models::CreateWorldEarningPayoutAccountOnboardResponse create_world_earning_payout_account_onboard()
Start or resume Stripe onboarding for your creator payout account. Returns a one-time Stripe URL. Body: { country? } (fixed once the account exists).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateWorldEarningPayoutAccountOnboardResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_drop_table

> crate::models::CreateWorldExperienceDropTableResponse create_world_experience_drop_table(experience_id)
Declare a drop table for a paid random item: cosmetic or in-experience outcomes only. Crossly pre-simulates and hashes it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldExperienceDropTableResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_product

> crate::models::WorldExperienceProductResponse create_world_experience_product(experience_id)
Create a product: price 50-100,000 cents (a private server may be free), optional per-country prices. Paid random items need an approved drop table and are never tradeable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::WorldExperienceProductResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_product_archive

> crate::models::WorldExperienceProductResponse create_world_experience_product_archive(experience_id, product_id)
Take a product off sale for good. Owned passes and running subscriptions keep working.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**product_id** | **String** |  | [required] |

### Return type

**crate::models::WorldExperienceProductResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_purchase

> crate::models::CreateWorldExperiencePurchaseResponse create_world_experience_purchase(experience_id)
Open a purchase sheet for a product (paid access, a pass…). Returns checkoutPath: the player pays only in the Crossly checkout.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldExperiencePurchaseResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_receipt_ack

> crate::models::CreateWorldExperienceReceiptAckResponse create_world_experience_receipt_ack(experience_id, order_id)
Acknowledge a grant your back-end made off-server. Idempotent: a second ack reports duplicate.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**order_id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldExperienceReceiptAckResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_me_receipt_refund

> crate::models::CreateWorldMeReceiptRefundResponse create_world_me_receipt_refund(receipt_id)
Refund a developer product bought by mistake: within the self-refund window and before the experience granted it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**receipt_id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldMeReceiptRefundResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_me_subscription_cancel

> crate::models::CreateWorldMeSubscriptionCancelResponse create_world_me_subscription_cancel(id)
Cancel: no more charges; the benefit lasts to the end of the paid month.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldMeSubscriptionCancelResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_me_subscription_private_server_code

> crate::models::CreateWorldMeSubscriptionPrivateServerCodeResponse create_world_me_subscription_private_server_code(id)
A fresh access code for your private server (the previous one stops working). Join with reservedCode.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldMeSubscriptionPrivateServerCodeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_purchase_cancel

> crate::models::CreateWorldSpaceJoinResponse create_world_purchase_cancel(intent_id)
Cancel an unpaid purchase sheet.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**intent_id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldSpaceJoinResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_earning

> crate::models::GetWorldEarningResponse get_world_earning()
Your Crossly Carnival earnings: pending, available, held, paid; next payout; what cashing out still needs.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWorldEarningResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_earning_payout_account

> crate::models::GetWorldEarningPayoutAccountResponse get_world_earning_payout_account()
Your creator payout account (your own, separate from any store payout account): not_started, pending, restricted, active or payouts_unavailable.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWorldEarningPayoutAccountResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_economy

> crate::models::GetWorldEconomyResponse get_world_economy()
The published payout formula: splits, thresholds, scheduled changes (90-day notice) and guarantees.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWorldEconomyResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_experience_revenue

> crate::models::GetWorldExperienceRevenueResponse get_world_experience_revenue(experience_id)
Gross, tax, refunds, chargebacks, net and your share per product, plus pending / available / paid.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::GetWorldExperienceRevenueResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_experience_store

> crate::models::GetWorldExperienceStoreResponse get_world_experience_store(experience_id)
An experience's store as you see it: your regional price, odds for paid random items, what you own.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::GetWorldExperienceStoreResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_purchase

> crate::models::GetWorldPurchaseResponse get_world_purchase(intent_id)
A purchase sheet: item, price, tax, total, odds, status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**intent_id** | **String** |  | [required] |

### Return type

**crate::models::GetWorldPurchaseResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_earning_affiliate

> crate::models::ListWorldEarningAffiliateResponse list_world_earning_affiliate()
Real-goods orders attributed to your games: the fee, your 20% share, and whether it is posted.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWorldEarningAffiliateResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_earning_ledger

> crate::models::ListWorldEarningLedgerResponse list_world_earning_ledger()
Your creator ledger, newest first: every earning and reversal. Cursor-paged.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWorldEarningLedgerResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_earning_payouts

> crate::models::ListWorldEarningPayoutsResponse list_world_earning_payouts()
Your monthly Crossly Carnival payouts.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWorldEarningPayoutsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_experience_drop_tables

> crate::models::ListWorldExperienceDropTablesResponse list_world_experience_drop_tables(experience_id)
Declared drop tables with their odds, simulation (10^6 draws) and hash.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::ListWorldExperienceDropTablesResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_experience_products

> crate::models::ListWorldExperienceProductsResponse list_world_experience_products(experience_id)
What an experience sells: passes, developer products, subscriptions, private servers, paid access (?includeArchived=true).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::ListWorldExperienceProductsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_experience_receipts

> crate::models::ListWorldExperienceReceiptsResponse list_world_experience_receipts(experience_id)
Every purchase: orderId, sub, product, price, net, state (paid/delivered/granted/refunded/revoked). Cursor-paged; ?since=.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::ListWorldExperienceReceiptsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_me_subscriptions

> crate::models::ListWorldMeSubscriptionsResponse list_world_me_subscriptions()
Your experience subscriptions and private servers.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWorldMeSubscriptionsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_experience_product

> crate::models::WorldExperienceProductResponse update_world_experience_product(experience_id, product_id)
Edit a product: name, description, icon, price, regional prices, hourly limit, on/off sale. Kind and place never change.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**product_id** | **String** |  | [required] |

### Return type

**crate::models::WorldExperienceProductResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

