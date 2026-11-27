# \TCGApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_tcg_challenge**](TCGApi.md#create_tcg_challenge) | **POST** /v1/tcg/{game}/challenges | Open a 1v1 challenge with one of your decks; another player joins it.
[**create_tcg_deck**](TCGApi.md#create_tcg_deck) | **POST** /v1/tcg/{game}/decks | Create (no id) or update a deck: its lead (Crew Boss, Mastermind or Keeper) and 40 cards you own, checked against the deck rules of its game.
[**create_tcg_match_act**](TCGApi.md#create_tcg_match_act) | **POST** /v1/tcg/matches/{id}/act | Make one move: { action } as listed in the match's legal moves. The engine checks it.
[**create_tcg_match_join**](TCGApi.md#create_tcg_match_join) | **POST** /v1/tcg/matches/{id}/join | Join an open 1v1 challenge with one of your decks.
[**create_tcg_pack_open**](TCGApi.md#create_tcg_pack_open) | **POST** /v1/tcg/packs/{id}/open | Open one of your earned packs: its eight cards join your collection.
[**create_tcg_practice**](TCGApi.md#create_tcg_practice) | **POST** /v1/tcg/{game}/practice | Start a practice match against the house bot with one of your decks.
[**create_tcg_queue**](TCGApi.md#create_tcg_queue) | **POST** /v1/tcg/{game}/queue | Join the ranked queue with an owned, legal deck (free). Returns the match at once when an opponent fits.
[**create_tcg_starter**](TCGApi.md#create_tcg_starter) | **POST** /v1/tcg/{game}/starter | Claim the free starter deck for a game, choosing its faction (once per account; account-bound, untradeable).
[**create_tcg_table**](TCGApi.md#create_tcg_table) | **POST** /v1/tcg/{game}/table | Sit at a table with your free starter deck (claimed now if you never did: starter names it, else one is picked for you): joins the oldest open 1v1 challenge, or opens one; already waiting, your own seat back.
[**create_tcg_tournament**](TCGApi.md#create_tcg_tournament) | **POST** /v1/tcg/{game}/tournaments | Host a Swiss tournament: name, optional top cut (4 or 8), round length, player cap and start time.
[**create_tcg_tournament_drop**](TCGApi.md#create_tcg_tournament_drop) | **POST** /v1/tcg/tournaments/{id}/drop | Leave a tournament before it starts, or drop from it (your results stand).
[**create_tcg_tournament_register**](TCGApi.md#create_tcg_tournament_register) | **POST** /v1/tcg/tournaments/{id}/register | Enter a tournament with an owned, legal deck before it starts.
[**create_tcg_tournament_start**](TCGApi.md#create_tcg_tournament_start) | **POST** /v1/tcg/tournaments/{id}/start | As the host, start the tournament: Swiss rounds are sized to the field and round 1 is dealt.
[**delete_tcg_deck**](TCGApi.md#delete_tcg_deck) | **DELETE** /v1/tcg/decks/{id} | Delete one of your decks.
[**delete_tcg_queue**](TCGApi.md#delete_tcg_queue) | **DELETE** /v1/tcg/{game}/queue | Leave the ranked queue.
[**get_tcg_creator_game_print**](TCGApi.md#get_tcg_creator_game_print) | **GET** /v1/tcg/creator/games/{id}/print | Print file links for your own creator card game (owner only): the set's print file, home sheets and a proof, for its newest version.
[**get_tcg_creator_game_print_by_kind**](TCGApi.md#get_tcg_creator_game_print_by_kind) | **GET** /v1/tcg/creator/games/{id}/print/{kind} | A PDF of your own creator card game (owner only): kind set | sheet | proof; ?version=, ?paper=letter|a4, ?card= for a proof. Binary (application/pdf).
[**get_tcg_ladder**](TCGApi.md#get_tcg_ladder) | **GET** /v1/tcg/{game}/ladder | The season's ranked leaderboard for a game: place, rating, tier and record (named only where the player chose to be).
[**get_tcg_ladder_me**](TCGApi.md#get_tcg_ladder_me) | **GET** /v1/tcg/{game}/ladder/me | Your ranked rating, tier, place and record this season, and whether you are queued.
[**get_tcg_match**](TCGApi.md#get_tcg_match) | **GET** /v1/tcg/{game}/matches | Your recent matches, and open challenges you could join.
[**get_tcg_match_by_id**](TCGApi.md#get_tcg_match_by_id) | **GET** /v1/tcg/matches/{id} | Your view of a match (no hidden information) and, on your turn, every legal move.
[**get_tcg_me**](TCGApi.md#get_tcg_me) | **GET** /v1/tcg/{game}/me | Your starter faction, collection (card id -> copies), decks and earned packs.
[**get_tcg_print**](TCGApi.md#get_tcg_print) | **GET** /v1/tcg/{game}/print | Print files for one of Crossly's own card games: a card proof (template), 3 x 3 Letter/A4 home sheets with crop marks, and the set's print file (63 x 88 mm, 3 mm bleed).
[**get_tcg_tournament**](TCGApi.md#get_tcg_tournament) | **GET** /v1/tcg/tournaments/{id} | A tournament's standings (points, opponents' and game win %), this round's pairings and your seat.
[**list_tcg_cards**](TCGApi.md#list_tcg_cards) | **GET** /v1/tcg/{game}/cards | The game's card pool: every card with its rules text, stats, rarity and collector number.
[**list_tcg_games**](TCGApi.md#list_tcg_games) | **GET** /v1/tcg/games | Crossly's own card games: id, name, seats and card count.
[**list_tcg_tournaments**](TCGApi.md#list_tcg_tournaments) | **GET** /v1/tcg/{game}/tournaments | A game's recent and upcoming Swiss tournaments (free to enter; prizes are earned packs).



## create_tcg_challenge

> crate::models::TcgResponse create_tcg_challenge(game)
Open a 1v1 challenge with one of your decks; another player joins it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game** | **String** |  | [required] |

### Return type

**crate::models::TcgResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_tcg_deck

> crate::models::CreateTcgDeckResponse create_tcg_deck(game)
Create (no id) or update a deck: its lead (Crew Boss, Mastermind or Keeper) and 40 cards you own, checked against the deck rules of its game.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game** | **String** |  | [required] |

### Return type

**crate::models::CreateTcgDeckResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_tcg_match_act

> crate::models::TcgResponse create_tcg_match_act(id)
Make one move: { action } as listed in the match's legal moves. The engine checks it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::TcgResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_tcg_match_join

> crate::models::TcgResponse create_tcg_match_join(id)
Join an open 1v1 challenge with one of your decks.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::TcgResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_tcg_pack_open

> crate::models::CreateTcgPackOpenResponse create_tcg_pack_open(id)
Open one of your earned packs: its eight cards join your collection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateTcgPackOpenResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_tcg_practice

> crate::models::TcgResponse create_tcg_practice(game)
Start a practice match against the house bot with one of your decks.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game** | **String** |  | [required] |

### Return type

**crate::models::TcgResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_tcg_queue

> crate::models::CreateTcgQueueResponse create_tcg_queue(game)
Join the ranked queue with an owned, legal deck (free). Returns the match at once when an opponent fits.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game** | **String** |  | [required] |

### Return type

**crate::models::CreateTcgQueueResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_tcg_starter

> crate::models::CreateTcgStarterResponse create_tcg_starter(game)
Claim the free starter deck for a game, choosing its faction (once per account; account-bound, untradeable).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game** | **String** |  | [required] |

### Return type

**crate::models::CreateTcgStarterResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_tcg_table

> crate::models::TcgResponse create_tcg_table(game)
Sit at a table with your free starter deck (claimed now if you never did: starter names it, else one is picked for you): joins the oldest open 1v1 challenge, or opens one; already waiting, your own seat back.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game** | **String** |  | [required] |

### Return type

**crate::models::TcgResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_tcg_tournament

> crate::models::CreateTcgTournamentResponse create_tcg_tournament(game)
Host a Swiss tournament: name, optional top cut (4 or 8), round length, player cap and start time.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game** | **String** |  | [required] |

### Return type

**crate::models::CreateTcgTournamentResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_tcg_tournament_drop

> crate::models::TcgTournamentResponse create_tcg_tournament_drop(id)
Leave a tournament before it starts, or drop from it (your results stand).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::TcgTournamentResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_tcg_tournament_register

> crate::models::TcgTournamentResponse create_tcg_tournament_register(id)
Enter a tournament with an owned, legal deck before it starts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::TcgTournamentResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_tcg_tournament_start

> crate::models::TcgTournamentResponse create_tcg_tournament_start(id)
As the host, start the tournament: Swiss rounds are sized to the field and round 1 is dealt.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::TcgTournamentResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_tcg_deck

> delete_tcg_deck(id)
Delete one of your decks.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_tcg_queue

> delete_tcg_queue(game)
Leave the ranked queue.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_tcg_creator_game_print

> crate::models::GetTcgCreatorGamePrintResponse get_tcg_creator_game_print(id)
Print file links for your own creator card game (owner only): the set's print file, home sheets and a proof, for its newest version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetTcgCreatorGamePrintResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_tcg_creator_game_print_by_kind

> std::path::PathBuf get_tcg_creator_game_print_by_kind(id, kind)
A PDF of your own creator card game (owner only): kind set | sheet | proof; ?version=, ?paper=letter|a4, ?card= for a proof. Binary (application/pdf).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**kind** | **String** |  | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/pdf, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_tcg_ladder

> crate::models::GetTcgLadderResponse get_tcg_ladder(game)
The season's ranked leaderboard for a game: place, rating, tier and record (named only where the player chose to be).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game** | **String** |  | [required] |

### Return type

**crate::models::GetTcgLadderResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_tcg_ladder_me

> crate::models::GetTcgLadderMeResponse get_tcg_ladder_me(game)
Your ranked rating, tier, place and record this season, and whether you are queued.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game** | **String** |  | [required] |

### Return type

**crate::models::GetTcgLadderMeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_tcg_match

> crate::models::GetTcgMatchResponse get_tcg_match(game)
Your recent matches, and open challenges you could join.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game** | **String** |  | [required] |

### Return type

**crate::models::GetTcgMatchResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_tcg_match_by_id

> crate::models::TcgResponse get_tcg_match_by_id(id)
Your view of a match (no hidden information) and, on your turn, every legal move.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::TcgResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_tcg_me

> crate::models::GetTcgMeResponse get_tcg_me(game)
Your starter faction, collection (card id -> copies), decks and earned packs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game** | **String** |  | [required] |

### Return type

**crate::models::GetTcgMeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_tcg_print

> crate::models::GetTcgPrintResponse get_tcg_print(game)
Print files for one of Crossly's own card games: a card proof (template), 3 x 3 Letter/A4 home sheets with crop marks, and the set's print file (63 x 88 mm, 3 mm bleed).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game** | **String** |  | [required] |

### Return type

**crate::models::GetTcgPrintResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_tcg_tournament

> crate::models::TcgTournamentResponse get_tcg_tournament(id)
A tournament's standings (points, opponents' and game win %), this round's pairings and your seat.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::TcgTournamentResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_tcg_cards

> crate::models::ListTcgCardsResponse list_tcg_cards(game)
The game's card pool: every card with its rules text, stats, rarity and collector number.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game** | **String** |  | [required] |

### Return type

**crate::models::ListTcgCardsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_tcg_games

> crate::models::ListTcgGamesResponse list_tcg_games()
Crossly's own card games: id, name, seats and card count.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListTcgGamesResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_tcg_tournaments

> crate::models::ListTcgTournamentsResponse list_tcg_tournaments(game)
A game's recent and upcoming Swiss tournaments (free to enter; prizes are earned packs).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game** | **String** |  | [required] |

### Return type

**crate::models::ListTcgTournamentsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

