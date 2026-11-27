# \TCGCreatorsApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_tcg_creator_game**](TCGCreatorsApi.md#create_tcg_creator_game) | **POST** /v1/tcg/creator/games | Submit a card-game package version for review: game.json, cards.json, rules/game.js, thumb.png, art/_*.png as { path, base64 }. Checked, conformance-run, art quarantined, then reviewed by a person.
[**create_tcg_creator_game_check**](TCGCreatorsApi.md#create_tcg_creator_game_check) | **POST** /v1/tcg/creator/games/check | Check a card-game package exactly as a submission is checked -- the static report and bot games played against your rules in the sandbox -- storing nothing. Files as { path, base64 }.
[**create_tcg_creator_game_pack_open**](TCGCreatorsApi.md#create_tcg_creator_game_pack_open) | **POST** /v1/tcg/creator/games/{id}/packs/{packId}/open | Open a booster you hold: the game's rules pick the pulls, the pack is used up, and one COSM copy per card joins your collection (logged with its seed).
[**create_tcg_creator_game_starter**](TCGCreatorsApi.md#create_tcg_creator_game_starter) | **POST** /v1/tcg/creator/games/{id}/starter | Claim a creator game's free starter deck by its key (once per account; account-bound, never tradeable).
[**create_tcg_creator_game_version_withdraw**](TCGCreatorsApi.md#create_tcg_creator_game_version_withdraw) | **POST** /v1/tcg/creator/games/{id}/versions/{v}/withdraw | Withdraw one of your versions that is still waiting for review.
[**get_tcg_creator_game**](TCGCreatorsApi.md#get_tcg_creator_game) | **GET** /v1/tcg/creator/games/{id} | One live creator card game: its set, every card (rules text, rarity, art URL, game data) and its COSM prices.
[**get_tcg_creator_game_collection**](TCGCreatorsApi.md#get_tcg_creator_game_collection) | **GET** /v1/tcg/creator/games/{id}/collection | Your playable copies of a creator game: card id -> copies (free starter plus every COSM copy you hold).
[**get_tcg_creator_game_popularity**](TCGCreatorsApi.md#get_tcg_creator_game_popularity) | **GET** /v1/tcg/creator/games/{id}/popularity | The set's progress toward going on chain: distinct collectors, packs opened and resales against the bar; when it passes, every holder may mint their cards as NFTs.
[**get_tcg_creator_game_shelf**](TCGCreatorsApi.md#get_tcg_creator_game_shelf) | **GET** /v1/tcg/creator/games/{id}/shelf | The game's COSM shelf: the booster and every single, each with its COSM design id (buy via /api/v1/cosm/cosmetics/:id/buy), price, edition cap and copies issued.
[**list_tcg_creator_game_packs**](TCGCreatorsApi.md#list_tcg_creator_game_packs) | **GET** /v1/tcg/creator/games/{id}/packs | Your unopened boosters of a creator game.
[**list_tcg_creator_games**](TCGCreatorsApi.md#list_tcg_creator_games) | **GET** /v1/tcg/creator/games | Live creator card games: name, set, description, typical match length, frame colours, thumbnail.
[**list_tcg_creator_mine**](TCGCreatorsApi.md#list_tcg_creator_mine) | **GET** /v1/tcg/creator/mine | Your creator card games and every version with its review state and note.



## create_tcg_creator_game

> crate::models::CreateTcgCreatorGameResponse create_tcg_creator_game()
Submit a card-game package version for review: game.json, cards.json, rules/game.js, thumb.png, art/_*.png as { path, base64 }. Checked, conformance-run, art quarantined, then reviewed by a person.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateTcgCreatorGameResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_tcg_creator_game_check

> crate::models::CreateTcgCreatorGameCheckResponse create_tcg_creator_game_check()
Check a card-game package exactly as a submission is checked -- the static report and bot games played against your rules in the sandbox -- storing nothing. Files as { path, base64 }.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateTcgCreatorGameCheckResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_tcg_creator_game_pack_open

> crate::models::CreateTcgCreatorGamePackOpenResponse create_tcg_creator_game_pack_open(id, pack_id)
Open a booster you hold: the game's rules pick the pulls, the pack is used up, and one COSM copy per card joins your collection (logged with its seed).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**pack_id** | **String** |  | [required] |

### Return type

**crate::models::CreateTcgCreatorGamePackOpenResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_tcg_creator_game_starter

> crate::models::CreateTcgStarterResponse create_tcg_creator_game_starter(id)
Claim a creator game's free starter deck by its key (once per account; account-bound, never tradeable).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateTcgStarterResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_tcg_creator_game_version_withdraw

> create_tcg_creator_game_version_withdraw(id, v)
Withdraw one of your versions that is still waiting for review.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**v** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_tcg_creator_game

> crate::models::GetTcgCreatorGameResponse get_tcg_creator_game(id)
One live creator card game: its set, every card (rules text, rarity, art URL, game data) and its COSM prices.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetTcgCreatorGameResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_tcg_creator_game_collection

> crate::models::GetTcgCreatorGameCollectionResponse get_tcg_creator_game_collection(id)
Your playable copies of a creator game: card id -> copies (free starter plus every COSM copy you hold).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetTcgCreatorGameCollectionResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_tcg_creator_game_popularity

> crate::models::GetTcgCreatorGamePopularityResponse get_tcg_creator_game_popularity(id)
The set's progress toward going on chain: distinct collectors, packs opened and resales against the bar; when it passes, every holder may mint their cards as NFTs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetTcgCreatorGamePopularityResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_tcg_creator_game_shelf

> crate::models::GetTcgCreatorGameShelfResponse get_tcg_creator_game_shelf(id)
The game's COSM shelf: the booster and every single, each with its COSM design id (buy via /api/v1/cosm/cosmetics/:id/buy), price, edition cap and copies issued.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetTcgCreatorGameShelfResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_tcg_creator_game_packs

> crate::models::ListTcgCreatorGamePacksResponse list_tcg_creator_game_packs(id)
Your unopened boosters of a creator game.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListTcgCreatorGamePacksResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_tcg_creator_games

> crate::models::ListTcgCreatorGamesResponse list_tcg_creator_games()
Live creator card games: name, set, description, typical match length, frame colours, thumbnail.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListTcgCreatorGamesResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_tcg_creator_mine

> crate::models::ListTcgCreatorMineResponse list_tcg_creator_mine()
Your creator card games and every version with its review state and note.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListTcgCreatorMineResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

