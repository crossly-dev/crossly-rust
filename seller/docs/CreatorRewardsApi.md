# \CreatorRewardsApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_creator_reward_formula**](CreatorRewardsApi.md#get_creator_reward_formula) | **GET** /v1/creator-rewards/formula | The Creator Rewards formula: the pool's share of Crossly's Crossly Carnival take in force, the counting rules, and any scheduled change (announced at least 90 days ahead).
[**get_creator_reward_me**](CreatorRewardsApi.md#get_creator_reward_me) | **GET** /v1/creator-rewards/me/{month} | One month of your Creator Rewards (YYYY-MM).
[**get_public_creator_reward_reconciliation**](CreatorRewardsApi.md#get_public_creator_reward_reconciliation) | **GET** /v1/public/creator-rewards/reconciliation | Public monthly reconciliation of Creator Rewards (no auth): per month the pool funded from Crossly's take, carried in, distributed, paid (from the ledger), carried over, the formula version and weights, eligible experiences and every check; plus the formula in force and scheduled changes (90-day notice).
[**list_creator_reward_me**](CreatorRewardsApi.md#list_creator_reward_me) | **GET** /v1/creator-rewards/me | Your Creator Rewards: each month, each experience you own or are paid for (counted minutes and players, exclusions, its amount, your part).
[**list_creator_reward_months**](CreatorRewardsApi.md#list_creator_reward_months) | **GET** /v1/creator-rewards/months | Every calculated month: Crossly's take, the funded share, the pool, what was distributed and carried, counted minutes and exclusions by reason.



## get_creator_reward_formula

> crate::models::GetCreatorRewardFormulaResponse get_creator_reward_formula()
The Creator Rewards formula: the pool's share of Crossly's Crossly Carnival take in force, the counting rules, and any scheduled change (announced at least 90 days ahead).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetCreatorRewardFormulaResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_creator_reward_me

> crate::models::GetCreatorRewardMeResponse get_creator_reward_me(month)
One month of your Creator Rewards (YYYY-MM).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**month** | **String** |  | [required] |

### Return type

**crate::models::GetCreatorRewardMeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_public_creator_reward_reconciliation

> crate::models::GetPublicCreatorRewardReconciliationResponse get_public_creator_reward_reconciliation(limit)
Public monthly reconciliation of Creator Rewards (no auth): per month the pool funded from Crossly's take, carried in, distributed, paid (from the ledger), carried over, the formula version and weights, eligible experiences and every check; plus the formula in force and scheduled changes (90-day notice).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | Months to return, newest first (default 24). |  |

### Return type

**crate::models::GetPublicCreatorRewardReconciliationResponse**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_creator_reward_me

> crate::models::ListCreatorRewardMeResponse list_creator_reward_me()
Your Creator Rewards: each month, each experience you own or are paid for (counted minutes and players, exclusions, its amount, your part).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListCreatorRewardMeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_creator_reward_months

> crate::models::ListCreatorRewardMonthsResponse list_creator_reward_months()
Every calculated month: Crossly's take, the funded share, the pool, what was distributed and carried, counted minutes and exclusions by reason.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListCreatorRewardMonthsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

