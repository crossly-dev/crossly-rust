# \WorldsRewardsApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_world_experience_reward_close**](WorldsRewardsApi.md#create_world_experience_reward_close) | **POST** /v1/worlds/experiences/{experienceId}/rewards/close | Close the pool: the unspent balance returns to your Crossly Carnival earnings. Free rewards keep working.
[**create_world_experience_reward_definition**](WorldsRewardsApi.md#create_world_experience_reward_definition) | **POST** /v1/worlds/experiences/{experienceId}/rewards/definitions | Add a reward scripts can grant: pin, emote, decor, item (your own COSM design), cosm or cbx (with an amount).
[**create_world_experience_reward_deposit**](WorldsRewardsApi.md#create_world_experience_reward_deposit) | **POST** /v1/worlds/experiences/{experienceId}/rewards/deposits | Move available Crossly Carnival earnings into the pool ($1-$10,000). Owner only; idempotent on idempotencyKey.
[**create_world_experience_reward_topup**](WorldsRewardsApi.md#create_world_experience_reward_topup) | **POST** /v1/worlds/experiences/{experienceId}/rewards/topups | Open a card top-up ($5-$10,000 into the pool; the 2.9% + 30¢ card fee is added on top). Returns checkoutUrl, where the owner pays with Crossly checkout; the pool is credited once when the payment lands. Owner only.
[**create_world_reward_topup_cancel**](WorldsRewardsApi.md#create_world_reward_topup_cancel) | **POST** /v1/worlds/rewards/topups/{topupId}/cancel | Cancel an unpaid top-up sheet.
[**get_world_experience_reward**](WorldsRewardsApi.md#get_world_experience_reward) | **GET** /v1/worlds/experiences/{experienceId}/rewards | The funded reward pool: balance, reserved (owed) value, caps, today's grants, the reward catalogue and the published rates.
[**get_world_reward_topup**](WorldsRewardsApi.md#get_world_reward_topup) | **GET** /v1/worlds/rewards/topups/{topupId} | One card top-up: its status (open, paying, paid, cancelled, expired, refused), amounts and the pool it funds.
[**list_world_experience_reward_grants**](WorldsRewardsApi.md#list_world_experience_reward_grants) | **GET** /v1/worlds/experiences/{experienceId}/rewards/grants | Every grant, newest first: reward, player sub, amount, cost, status. Cursor-paged; ?sub= for one player.
[**list_world_experience_reward_ledger**](WorldsRewardsApi.md#list_world_experience_reward_ledger) | **GET** /v1/worlds/experiences/{experienceId}/rewards/ledger | The pool's own ledger: deposits, grant draw-downs, refunds. Cursor-paged.
[**list_world_experience_reward_topups**](WorldsRewardsApi.md#list_world_experience_reward_topups) | **GET** /v1/worlds/experiences/{experienceId}/rewards/topups | Card top-ups of the reward pool, newest first: amount into the pool, card fee, status, what went back to the card.
[**update_world_experience_reward_definition**](WorldsRewardsApi.md#update_world_experience_reward_definition) | **PATCH** /v1/worlds/experiences/{experienceId}/rewards/definitions/{rewardId} | Edit a reward: name, description, image, amount, per-player daily cap, on/off. Key and kind never change.
[**update_world_experience_reward_pool**](WorldsRewardsApi.md#update_world_experience_reward_pool) | **PATCH** /v1/worlds/experiences/{experienceId}/rewards/pool | Set the pool's daily caps: total value per day, value and grant count per player per day.



## create_world_experience_reward_close

> crate::models::CreateWorldExperienceRewardCloseResponse create_world_experience_reward_close(experience_id)
Close the pool: the unspent balance returns to your Crossly Carnival earnings. Free rewards keep working.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldExperienceRewardCloseResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_reward_definition

> crate::models::CreateWorldExperienceRewardDefinitionResponse create_world_experience_reward_definition(experience_id)
Add a reward scripts can grant: pin, emote, decor, item (your own COSM design), cosm or cbx (with an amount).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldExperienceRewardDefinitionResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_reward_deposit

> crate::models::CreateWorldExperienceRewardDepositResponse create_world_experience_reward_deposit(experience_id)
Move available Crossly Carnival earnings into the pool ($1-$10,000). Owner only; idempotent on idempotencyKey.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldExperienceRewardDepositResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_reward_topup

> crate::models::CreateWorldExperienceRewardTopupResponse create_world_experience_reward_topup(experience_id)
Open a card top-up ($5-$10,000 into the pool; the 2.9% + 30¢ card fee is added on top). Returns checkoutUrl, where the owner pays with Crossly checkout; the pool is credited once when the payment lands. Owner only.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldExperienceRewardTopupResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_reward_topup_cancel

> crate::models::CreateWorldSpaceJoinResponse create_world_reward_topup_cancel(topup_id)
Cancel an unpaid top-up sheet.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**topup_id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldSpaceJoinResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_experience_reward

> crate::models::GetWorldExperienceRewardResponse get_world_experience_reward(experience_id)
The funded reward pool: balance, reserved (owed) value, caps, today's grants, the reward catalogue and the published rates.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::GetWorldExperienceRewardResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_reward_topup

> crate::models::GetWorldRewardTopupResponse get_world_reward_topup(topup_id)
One card top-up: its status (open, paying, paid, cancelled, expired, refused), amounts and the pool it funds.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**topup_id** | **String** |  | [required] |

### Return type

**crate::models::GetWorldRewardTopupResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_experience_reward_grants

> crate::models::ListWorldExperienceRewardGrantsResponse list_world_experience_reward_grants(experience_id)
Every grant, newest first: reward, player sub, amount, cost, status. Cursor-paged; ?sub= for one player.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::ListWorldExperienceRewardGrantsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_experience_reward_ledger

> crate::models::ListWorldExperienceRewardLedgerResponse list_world_experience_reward_ledger(experience_id)
The pool's own ledger: deposits, grant draw-downs, refunds. Cursor-paged.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::ListWorldExperienceRewardLedgerResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_experience_reward_topups

> crate::models::ListWorldExperienceRewardTopupsResponse list_world_experience_reward_topups(experience_id)
Card top-ups of the reward pool, newest first: amount into the pool, card fee, status, what went back to the card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::ListWorldExperienceRewardTopupsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_experience_reward_definition

> crate::models::CreateWorldExperienceRewardDefinitionResponse update_world_experience_reward_definition(experience_id, reward_id)
Edit a reward: name, description, image, amount, per-player daily cap, on/off. Key and kind never change.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**reward_id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldExperienceRewardDefinitionResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_experience_reward_pool

> crate::models::UpdateWorldExperienceRewardPoolResponse update_world_experience_reward_pool(experience_id)
Set the pool's daily caps: total value per day, value and grant count per player per day.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::UpdateWorldExperienceRewardPoolResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

