# \WorldsAdsApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_world_experience_ad**](WorldsAdsApi.md#get_world_experience_ad) | **GET** /v1/worlds/experiences/{experienceId}/ads | The ads for a game's in-world ad surfaces (parts tagged CrosslyAdSurface), for the token's owner: Sponsored Listings labelled \"Ad\", served only to signed-in adults where the game's creator turned ads on, never to its team. A click is POST /api/public/shop/ads/click { token }.
[**get_world_experience_ad_report**](WorldsAdsApi.md#get_world_experience_ad_report) | **GET** /v1/worlds/experiences/{experienceId}/ads/report | In-game ads over the last N days (default 30, max 90): impressions, clicks, charged clicks, advertiser spend and your earned share, per day.
[**get_world_experience_ad_setting**](WorldsAdsApi.md#get_world_experience_ad_setting) | **GET** /v1/worlds/experiences/{experienceId}/ad-settings | Whether this game shows in-game ads (opt-in, default off) and the creator's share of Crossly's ad revenue in force.
[**update_world_experience_ad_setting**](WorldsAdsApi.md#update_world_experience_ad_setting) | **PUT** /v1/worlds/experiences/{experienceId}/ad-settings | Turn in-game ads on or off for this game. Ads fill the game's ad surfaces (parts tagged CrosslyAdSurface); nothing shows until it has one.



## get_world_experience_ad

> crate::models::GetWorldExperienceAdResponse get_world_experience_ad(experience_id)
The ads for a game's in-world ad surfaces (parts tagged CrosslyAdSurface), for the token's owner: Sponsored Listings labelled \"Ad\", served only to signed-in adults where the game's creator turned ads on, never to its team. A click is POST /api/public/shop/ads/click { token }.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::GetWorldExperienceAdResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_experience_ad_report

> crate::models::GetWorldExperienceAdReportResponse get_world_experience_ad_report(experience_id)
In-game ads over the last N days (default 30, max 90): impressions, clicks, charged clicks, advertiser spend and your earned share, per day.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::GetWorldExperienceAdReportResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_experience_ad_setting

> crate::models::GetWorldExperienceAdSettingResponse get_world_experience_ad_setting(experience_id)
Whether this game shows in-game ads (opt-in, default off) and the creator's share of Crossly's ad revenue in force.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::GetWorldExperienceAdSettingResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_experience_ad_setting

> crate::models::GetWorldExperienceAdSettingResponse update_world_experience_ad_setting(experience_id)
Turn in-game ads on or off for this game. Ads fill the game's ad surfaces (parts tagged CrosslyAdSurface); nothing shows until it has one.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::GetWorldExperienceAdSettingResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

