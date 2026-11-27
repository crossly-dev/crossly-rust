# \WorldsBadgesApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_world_experience_badge**](WorldsBadgesApi.md#create_world_experience_badge) | **POST** /v1/worlds/experiences/{experienceId}/badges | Add a badge (at most 100 per game). name 1-50 characters, description up to 1000, iconSha256 = the sha256 a png upload answered (POST /v1/world/interactive/assets, kind png); a new icon is shown once a person approved it.
[**get_world_badge_mine**](WorldsBadgesApi.md#get_world_badge_mine) | **GET** /v1/worlds/badges/mine | Every game badge you hold, newest first, and whether your person page shows them.
[**get_world_experience_badge**](WorldsBadgesApi.md#get_world_experience_badge) | **GET** /v1/worlds/experiences/{experienceId}/badges | A game's badges, oldest first: name, description, icon and its review state, on/off, how many players hold each; plus the player count (the rarity denominator) and the limits. Scripts award them with BadgeService.AwardBadge(player, badgeId).
[**get_world_game_badge**](WorldsBadgesApi.md#get_world_game_badge) | **GET** /v1/worlds/games/{game}/badges | A Crossly Carnival game's badges as players see them: name, description, approved icon, how many hold it, rarity (% of the game's players) and, for you, when you earned it. :game is a Worlds experience id or key.
[**get_world_player_badge**](WorldsBadgesApi.md#get_world_player_badge) | **GET** /v1/worlds/players/{username}/badges | The game badges a person shows on their person page (only when they switched it on; Mature and non-public games never appear).
[**update_world_badge_profile**](WorldsBadgesApi.md#update_world_badge_profile) | **PUT** /v1/worlds/badges/profile | Show (or hide) the game badges you hold on your person page. Off by default.
[**update_world_experience_badge**](WorldsBadgesApi.md#update_world_experience_badge) | **PATCH** /v1/worlds/experiences/{experienceId}/badges/{badgeId} | Edit a badge: name, description, icon (null removes it), enabled. Badges are never deleted (players keep what they earned); turn one off to hide it and stop AwardBadge.



## create_world_experience_badge

> crate::models::CreateWorldExperienceBadgeResponse create_world_experience_badge(experience_id)
Add a badge (at most 100 per game). name 1-50 characters, description up to 1000, iconSha256 = the sha256 a png upload answered (POST /v1/world/interactive/assets, kind png); a new icon is shown once a person approved it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldExperienceBadgeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_badge_mine

> crate::models::GetWorldBadgeMineResponse get_world_badge_mine()
Every game badge you hold, newest first, and whether your person page shows them.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWorldBadgeMineResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_experience_badge

> crate::models::GetWorldExperienceBadgeResponse get_world_experience_badge(experience_id)
A game's badges, oldest first: name, description, icon and its review state, on/off, how many players hold each; plus the player count (the rarity denominator) and the limits. Scripts award them with BadgeService.AwardBadge(player, badgeId).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::GetWorldExperienceBadgeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_game_badge

> crate::models::GetWorldGameBadgeResponse get_world_game_badge(game)
A Crossly Carnival game's badges as players see them: name, description, approved icon, how many hold it, rarity (% of the game's players) and, for you, when you earned it. :game is a Worlds experience id or key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game** | **String** |  | [required] |

### Return type

**crate::models::GetWorldGameBadgeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_player_badge

> crate::models::GetWorldPlayerBadgeResponse get_world_player_badge(username)
The game badges a person shows on their person page (only when they switched it on; Mature and non-public games never appear).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** |  | [required] |

### Return type

**crate::models::GetWorldPlayerBadgeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_badge_profile

> crate::models::UpdateWorldBadgeProfileResponse update_world_badge_profile()
Show (or hide) the game badges you hold on your person page. Off by default.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::UpdateWorldBadgeProfileResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_experience_badge

> crate::models::CreateWorldExperienceBadgeResponse update_world_experience_badge(experience_id, badge_id)
Edit a badge: name, description, icon (null removes it), enabled. Badges are never deleted (players keep what they earned); turn one off to hide it and stop AwardBadge.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**badge_id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldExperienceBadgeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

