# \GradingApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_grading_run_reserve**](GradingApi.md#create_grading_run_reserve) | **POST** /v1/grading/runs/{id}/reserve | Hold a slot in a Grading Run. Free now; charged on your saved card only if the run locks. Full-access token and Idempotency-Key required.
[**create_grading_submission_cancel**](GradingApi.md#create_grading_submission_cancel) | **POST** /v1/grading/submissions/{id}/cancel | Pull a reserved card out of its run. Only while it is reserved; nothing was charged.
[**create_grading_submission_shipped**](GradingApi.md#create_grading_submission_shipped) | **POST** /v1/grading/submissions/{id}/shipped | Say you mailed a charged card to the hub, with optional tracking.
[**create_grading_verify_cert**](GradingApi.md#create_grading_verify_cert) | **POST** /v1/grading/verify-cert | Look a cert number up with its grader. found=false when the grader has no record.
[**create_vault_market_grading_order**](GradingApi.md#create_vault_market_grading_order) | **POST** /v1/vault-markets/grading/orders | Start a grading submission: cards (vault units or inventory items) to one grader at one service. Priced from the price book; vault units are held. Nothing is charged until you pay.
[**create_vault_market_grading_order_cancel**](GradingApi.md#create_vault_market_grading_order_cancel) | **POST** /v1/vault-markets/grading/orders/{id}/cancel | Cancel a draft submission; its vault units go back on their shelf. A paid submission cannot be cancelled here.
[**create_vault_market_grading_order_pay**](GradingApi.md#create_vault_market_grading_order_pay) | **POST** /v1/vault-markets/grading/orders/{id}/pay | Pay for a draft submission. savedCard: true charges your saved card (paid, or requires_action with a client secret for 3-D Secure). Full-access token and Idempotency-Key required.
[**create_vault_market_grading_order_pay_confirm**](GradingApi.md#create_vault_market_grading_order_pay_confirm) | **POST** /v1/vault-markets/grading/orders/{id}/pay/confirm | After a 3-D Secure step or a card form: confirm the PaymentIntent with Stripe and mark the submission paid.
[**get_grading_price_book**](GradingApi.md#get_grading_price_book) | **GET** /v1/grading/price-book | The grading price book: every grader, its services (per-card price, minimum cards, max declared value, turnaround) and add-ons, plus Crossly handling. The same data the dashboard prices from.
[**get_grading_worth**](GradingApi.md#get_grading_worth) | **GET** /v1/grading/worth/{inventoryItemId} | Worth grading? Raw value vs what each grade is worth at each grader, against the cost. Never predicts the grade.
[**get_vault_market_grading_order**](GradingApi.md#get_vault_market_grading_order) | **GET** /v1/vault-markets/grading/orders/{id} | One grading submission: its timeline, every card with its grade, cert number and photos, the costs and the tracking.
[**list_grading_rate_card**](GradingApi.md#list_grading_rate_card) | **GET** /v1/grading/rate-card | Every grader and tier Crossly can send to, with the per-card price in cents (null = paused at the grader).
[**list_grading_runs**](GradingApi.md#list_grading_runs) | **GET** /v1/grading/runs | Grading Runs taking reservations now, soonest cutoff first. meta.open is false while runs are switched off.
[**list_grading_submissions**](GradingApi.md#list_grading_submissions) | **GET** /v1/grading/submissions | Your cards out for grading (runs and Direct), newest first. active=1 keeps only the ones still in flight.
[**list_vault_market_grading_candidates**](GradingApi.md#list_vault_market_grading_candidates) | **GET** /v1/vault-markets/grading/candidates | What you can send to grading: raw units already in your vault and inventory items, each with a declared value to start from and the reason when one cannot go.
[**list_vault_market_grading_orders**](GradingApi.md#list_vault_market_grading_orders) | **GET** /v1/vault-markets/grading/orders | Your grading submissions, newest first. Filter by status, active, grader, tier and dates; q searches the number, card names and cert numbers.



## create_grading_run_reserve

> crate::models::IdResponse create_grading_run_reserve(id)
Hold a slot in a Grading Run. Free now; charged on your saved card only if the run locks. Full-access token and Idempotency-Key required.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::IdResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_grading_submission_cancel

> crate::models::OkResponse create_grading_submission_cancel(id)
Pull a reserved card out of its run. Only while it is reserved; nothing was charged.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::OkResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_grading_submission_shipped

> crate::models::OkResponse create_grading_submission_shipped(id)
Say you mailed a charged card to the hub, with optional tracking.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::OkResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_grading_verify_cert

> crate::models::CreateGradingVerifyCertResponse create_grading_verify_cert()
Look a cert number up with its grader. found=false when the grader has no record.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateGradingVerifyCertResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_grading_order

> crate::models::CreateVaultMarketGradingOrderResponse create_vault_market_grading_order()
Start a grading submission: cards (vault units or inventory items) to one grader at one service. Priced from the price book; vault units are held. Nothing is charged until you pay.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateVaultMarketGradingOrderResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_grading_order_cancel

> crate::models::StatusResponse create_vault_market_grading_order_cancel(id)
Cancel a draft submission; its vault units go back on their shelf. A paid submission cannot be cancelled here.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::StatusResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_grading_order_pay

> crate::models::CreateVaultMarketGradingOrderPayResponse create_vault_market_grading_order_pay(id)
Pay for a draft submission. savedCard: true charges your saved card (paid, or requires_action with a client secret for 3-D Secure). Full-access token and Idempotency-Key required.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateVaultMarketGradingOrderPayResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_vault_market_grading_order_pay_confirm

> crate::models::StatusResponse create_vault_market_grading_order_pay_confirm(id)
After a 3-D Secure step or a card form: confirm the PaymentIntent with Stripe and mark the submission paid.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::StatusResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_grading_price_book

> crate::models::GetGradingPriceBookResponse get_grading_price_book()
The grading price book: every grader, its services (per-card price, minimum cards, max declared value, turnaround) and add-ons, plus Crossly handling. The same data the dashboard prices from.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetGradingPriceBookResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_grading_worth

> crate::models::GetGradingWorthResponse get_grading_worth(inventory_item_id)
Worth grading? Raw value vs what each grade is worth at each grader, against the cost. Never predicts the grade.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inventory_item_id** | **String** |  | [required] |

### Return type

**crate::models::GetGradingWorthResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_vault_market_grading_order

> crate::models::GetVaultMarketGradingOrderResponse get_vault_market_grading_order(id)
One grading submission: its timeline, every card with its grade, cert number and photos, the costs and the tracking.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetVaultMarketGradingOrderResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_grading_rate_card

> crate::models::ListGradingRateCardResponse list_grading_rate_card()
Every grader and tier Crossly can send to, with the per-card price in cents (null = paused at the grader).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListGradingRateCardResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_grading_runs

> crate::models::ListGradingRunsResponse list_grading_runs()
Grading Runs taking reservations now, soonest cutoff first. meta.open is false while runs are switched off.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListGradingRunsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_grading_submissions

> crate::models::ListGradingSubmissionsResponse list_grading_submissions(active)
Your cards out for grading (runs and Direct), newest first. active=1 keeps only the ones still in flight.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**active** | Option<**String**> | Only cards that have not finished. |  |

### Return type

**crate::models::ListGradingSubmissionsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_grading_candidates

> crate::models::ListVaultMarketGradingCandidatesResponse list_vault_market_grading_candidates()
What you can send to grading: raw units already in your vault and inventory items, each with a declared value to start from and the reason when one cannot go.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketGradingCandidatesResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_vault_market_grading_orders

> crate::models::ListVaultMarketGradingOrdersResponse list_vault_market_grading_orders()
Your grading submissions, newest first. Filter by status, active, grader, tier and dates; q searches the number, card names and cert numbers.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVaultMarketGradingOrdersResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

