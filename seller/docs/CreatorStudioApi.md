# \CreatorStudioApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_creator_card_set**](CreatorStudioApi.md#create_creator_card_set) | **POST** /v1/creator/cards/sets | Start a card set: a game id (3-24 of a-z, 0-9, -), a set code (MG1) and a name. Answers a starter draft that already passes conformance. Paid seller plans only.
[**create_creator_card_set_art**](CreatorStudioApi.md#create_creator_card_set_art) | **POST** /v1/creator/cards/sets/{id}/art/{slug} | One card's art: a PNG (base64, at most 1 MB and 2048 px), scanned and stored until the version is approved.
[**create_creator_card_set_check**](CreatorStudioApi.md#create_creator_card_set_check) | **POST** /v1/creator/cards/sets/{id}/check | The full check publish runs: draft issues, tcgStaticConformance and bot-vs-bot games of the set.
[**create_creator_card_set_publish**](CreatorStudioApi.md#create_creator_card_set_publish) | **POST** /v1/creator/cards/sets/{id}/publish | Publish: the set must pass its check; it is packaged (game.json, cards.json, the designer rules, art) and submitted as a new immutable version for review, through the same intake as a hand-made card game.
[**create_creator_creation_submit**](CreatorStudioApi.md#create_creator_creation_submit) | **POST** /v1/creator/creations/{format}/{id}/submit | Send one creation to review through its format's own submit (wearable, model, decor, interactive, card: its whole set). Games are published from world-kit.
[**create_creator_import**](CreatorStudioApi.md#create_creator_import) | **POST** /v1/creator/imports | Validate a filled-in template (manifest text, CSV text, every path in the upload): issues with file, line and column, and a preview. Creates nothing.
[**create_creator_import_commit**](CreatorStudioApi.md#create_creator_import_commit) | **POST** /v1/creator/imports/{id}/commit | Create the drafts of a validated template upload (drafts only; nothing is submitted to review).
[**create_creator_item_model**](CreatorStudioApi.md#create_creator_item_model) | **POST** /v1/creator/items/models | Upload a .glb (base64, 4 MB): validated, measured and stored (quarantined until approved). An over-budget model is refused with every failing number.
[**create_creator_item_model_check**](CreatorStudioApi.md#create_creator_item_model_check) | **POST** /v1/creator/items/models/check | The budget report for a .glb (base64), storing nothing: every limit (triangles, draw calls, texture edge per phone/desktop; size, nodes, meshes, images, animations; bones for wearables) with pass/fail, problems and LOD suggestions.
[**create_creator_item_model_submit**](CreatorStudioApi.md#create_creator_item_model_submit) | **POST** /v1/creator/items/models/{id}/submit | Send an uploaded model to art review.
[**create_creator_item_preview**](CreatorStudioApi.md#create_creator_item_preview) | **POST** /v1/creator/items/previews | Upload a preview PNG (base64, at most 1 MB and 2048 px): parsed, scanned and stored on the Crossly CDN; answers its https URL.
[**create_creator_item_wearable**](CreatorStudioApi.md#create_creator_item_wearable) | **POST** /v1/creator/items/wearables | Start a wearable from a template: slug, name, colourways (panel -> #rrggbb), a COSM price and optional edition size. Text is filtered; answers the automated checks. Paid seller plans only.
[**create_creator_item_wearable_submit**](CreatorStudioApi.md#create_creator_item_wearable_submit) | **POST** /v1/creator/items/wearables/{id}/submit | Submit a wearable whose checks pass: each colourway becomes a COSM design (inactive) filed for review. Needs an https preview image (POST /v1/creator/items/previews).
[**create_creator_review_hide**](CreatorStudioApi.md#create_creator_review_hide) | **POST** /v1/creator/reviews/{reviewId}/hide | Hide a buyer review from your public page (it is kept, and its rating still counts).
[**create_creator_review_reply**](CreatorStudioApi.md#create_creator_review_reply) | **POST** /v1/creator/reviews/{reviewId}/reply | Reply to a buyer review of your creation (or change your reply).
[**create_creator_review_report**](CreatorStudioApi.md#create_creator_review_report) | **POST** /v1/creator/reviews/{reviewId}/report | Report a buyer review to Crossly (abuse, spam, not a real buyer).
[**create_creator_review_unhide**](CreatorStudioApi.md#create_creator_review_unhide) | **POST** /v1/creator/reviews/{reviewId}/unhide | Show a hidden buyer review on your page again.
[**delete_creator_card_set**](CreatorStudioApi.md#delete_creator_card_set) | **DELETE** /v1/creator/cards/sets/{id} | Delete a set draft that was never published.
[**delete_creator_item_model**](CreatorStudioApi.md#delete_creator_item_model) | **DELETE** /v1/creator/items/models/{id} | Delete a draft or rejected model.
[**delete_creator_item_wearable**](CreatorStudioApi.md#delete_creator_item_wearable) | **DELETE** /v1/creator/items/wearables/{id} | Delete a draft wearable.
[**get_creator_card_set**](CreatorStudioApi.md#get_creator_card_set) | **GET** /v1/creator/cards/sets/{id} | One set draft with its check: draft issues by field and tcgStaticConformance problems.
[**get_creator_card_set_progress**](CreatorStudioApi.md#get_creator_card_set_progress) | **GET** /v1/creator/cards/sets/{id}/progress | A set's three steps: live on COSM, the mint threshold toward NFTs, and the print-run cost (trading_cards estimate) against production funding banked.
[**get_creator_collection**](CreatorStudioApi.md#get_creator_collection) | **GET** /v1/creator/collections/{collectionId} | One collection (a card set, a launch or a claimed contract): its six-stage road to the chain, the launch (phase timeline, proceeds, recent mints), sales by marketplace, royalty estimate, holders, its cards or creations with their earnings, holder perks set at mint, and which actions apply.
[**get_creator_collection_perk**](CreatorStudioApi.md#get_creator_collection_perk) | **GET** /v1/creator/collections/{collectionId}/perks | A collection's holder perks (percent off at your shop, early access, a World item, a loyalty tier), whether they can be edited, and why not (a card set not started yet, a contract claimed from elsewhere).
[**get_creator_creation**](CreatorStudioApi.md#get_creator_creation) | **GET** /v1/creator/creations/{format}/{id} | One creation: its status timeline and review history, the files it uses, where it sells, its sales, its buyer reviews and its mint status.
[**get_creator_creation_status**](CreatorStudioApi.md#get_creator_creation_status) | **GET** /v1/creator/creations/{format}/{id}/status | Where one creation stands: its status, the reviewer's last note and its status timeline.
[**get_creator_earning_export**](CreatorStudioApi.md#get_creator_earning_export) | **GET** /v1/creator/earnings/export | Download your creator earnings: format=csv (every ledger row for the filter) or format=pdf (a statement with per-rail totals for the same filter: rail, source, group, gameId, state; period defaults to this month).
[**get_creator_earning_reconcile**](CreatorStudioApi.md#get_creator_earning_reconcile) | **GET** /v1/creator/earnings/reconcile | Reconcile your creator earnings: every source recomputed directly from its own ledger, side by side with the earnings view. ok is true when every source matches.
[**get_creator_import**](CreatorStudioApi.md#get_creator_import) | **GET** /v1/creator/imports/{id} | A stored template validation.
[**get_creator_item_model**](CreatorStudioApi.md#get_creator_item_model) | **GET** /v1/creator/items/models/{id} | One of your models with its budget report.
[**get_creator_item_template**](CreatorStudioApi.md#get_creator_item_template) | **GET** /v1/creator/items/templates | Wearable templates: each garment the Crossly avatar wears, its recolourable panels, default colours and measured cost (triangles, draw calls, bones, bytes, phone LOD), plus the per-wearable budget.
[**get_creator_item_wearable**](CreatorStudioApi.md#get_creator_item_wearable) | **GET** /v1/creator/items/wearables/{id} | One of your wearables with its checks, review state and note.
[**get_creator_minting**](CreatorStudioApi.md#get_creator_minting) | **GET** /v1/creator/minting | Everything you take on chain as collection rows with a six-stage status (designed, eligible, drafted, deploying, live, ended): gettingReady and launched. Game items (source game_item: tradeable game products, NFT declarations for your games) carry href and gameId.
[**get_creator_overview**](CreatorStudioApi.md#get_creator_overview) | **GET** /v1/creator/overview | Creator Studio overview: earnings this month and last by rail, live creations, review queue, publish lock and next steps.
[**get_creator_template**](CreatorStudioApi.md#get_creator_template) | **GET** /v1/creator/templates/{format} | The template files for a format (wearable, model, decor, interactive, card_set): crossly-creation.json, creations.csv where it applies, and a README.
[**get_creator_template_zip**](CreatorStudioApi.md#get_creator_template_zip) | **GET** /v1/creator/templates/{format}/zip | A format's template as a .zip download (the same files as GET /templates/:format, plus its empty folders).
[**list_creator_card_sets**](CreatorStudioApi.md#list_creator_card_sets) | **GET** /v1/creator/cards/sets | Your card set drafts, each with its road from COSM to print: live on COSM, the mint threshold (collectors, packs opened, resales) and the print-run cost against what NFT sales have banked.
[**list_creator_creation_reviews**](CreatorStudioApi.md#list_creator_creation_reviews) | **GET** /v1/creator/creations/{format}/{id}/reviews | Buyer reviews of one creation, newest first (hidden ones included).
[**list_creator_creations**](CreatorStudioApi.md#list_creator_creations) | **GET** /v1/creator/creations | Every creation of your shop in one list: wearables, 3D objects, decor, interactive objects, each card of your card sets, Classic games and card games, with status, collection, rating, earnings and mint status.
[**list_creator_earning_trend**](CreatorStudioApi.md#list_creator_earning_trend) | **GET** /v1/creator/earnings/trend | Your creator earnings over time: net (available + paid) per day, week or month (UTC) and rail, for the same filter as the ledger. meta.totals holds the per-rail and per-source totals of that filter.
[**list_creator_earnings**](CreatorStudioApi.md#list_creator_earnings) | **GET** /v1/creator/earnings | Your creator earnings ledger across every rail (cash, COSM, CBX), newest first. Each row names the source ledger row it is. meta.totals holds per-rail and per-source totals for the same filter; meta.nextCursor pages.
[**list_creator_item_models**](CreatorStudioApi.md#list_creator_item_models) | **GET** /v1/creator/items/models | Your uploaded 3D models with their budget reports and review state.
[**list_creator_item_wearables**](CreatorStudioApi.md#list_creator_item_wearables) | **GET** /v1/creator/items/wearables | Your wearables: drafts, in review, live or with changes requested, each with its automated checks and per-colourway designs.
[**list_creator_items**](CreatorStudioApi.md#list_creator_items) | **GET** /v1/creator/items | Every creation of your shop — wearables, decor, uploaded 3D models, interactive objects, Classic games, card games (incl. unpublished designer sets), physical runs — with status, price and earnings.
[**list_creator_reviews**](CreatorStudioApi.md#list_creator_reviews) | **GET** /v1/creator/reviews | Every creator submission (items, interactive objects, Classic games, card games) and its review decision with the reviewer note.
[**update_creator_card_set**](CreatorStudioApi.md#update_creator_card_set) | **PUT** /v1/creator/cards/sets/{id} | Save the whole draft: cards (name, type, rarity, rules text, flavour, power/draw/guard), COSM prices and supply, frame colours, print plan. Every word goes through the text filter. Art is merged: a card without `art` (or with art as read) keeps its stored art, `art: null` clears it; only the art upload sets art. Answers the check.
[**update_creator_collection_perk**](CreatorStudioApi.md#update_creator_collection_perk) | **PUT** /v1/creator/collections/{collectionId}/perks | Replace a collection's holder perks: `{ perks: [...] }`, at most one of each type: percent_off { percent 1-50 }, early_access { hours 1-720 }, world_item { creationRef, name }, loyalty_tier { tierId, tierName } (one of your loyalty tiers), each with an optional note. 409 when the collection has no launch yet.
[**update_creator_item_wearable**](CreatorStudioApi.md#update_creator_item_wearable) | **PATCH** /v1/creator/items/wearables/{id} | Edit a draft wearable (name, description, colourways, price, edition). Submitted wearables are frozen.



## create_creator_card_set

> crate::models::CreatorCardSetResponse create_creator_card_set()
Start a card set: a game id (3-24 of a-z, 0-9, -), a set code (MG1) and a name. Answers a starter draft that already passes conformance. Paid seller plans only.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreatorCardSetResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_creator_card_set_art

> crate::models::CreatorCardSetResponse create_creator_card_set_art(id, slug)
One card's art: a PNG (base64, at most 1 MB and 2048 px), scanned and stored until the version is approved.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**slug** | **String** |  | [required] |

### Return type

**crate::models::CreatorCardSetResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_creator_card_set_check

> crate::models::CreatorCardSetResponse create_creator_card_set_check(id)
The full check publish runs: draft issues, tcgStaticConformance and bot-vs-bot games of the set.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreatorCardSetResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_creator_card_set_publish

> crate::models::CreateCreatorCardSetPublishResponse create_creator_card_set_publish(id)
Publish: the set must pass its check; it is packaged (game.json, cards.json, the designer rules, art) and submitted as a new immutable version for review, through the same intake as a hand-made card game.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateCreatorCardSetPublishResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_creator_creation_submit

> crate::models::CreateCreatorCreationSubmitResponse create_creator_creation_submit(format, id)
Send one creation to review through its format's own submit (wearable, model, decor, interactive, card: its whole set). Games are published from world-kit.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateCreatorCreationSubmitResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_creator_import

> crate::models::GetCreatorImportResponse create_creator_import()
Validate a filled-in template (manifest text, CSV text, every path in the upload): issues with file, line and column, and a preview. Creates nothing.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetCreatorImportResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_creator_import_commit

> crate::models::CreateCreatorImportCommitResponse create_creator_import_commit(id)
Create the drafts of a validated template upload (drafts only; nothing is submitted to review).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateCreatorImportCommitResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_creator_item_model

> crate::models::CreatorItemModelResponse create_creator_item_model()
Upload a .glb (base64, 4 MB): validated, measured and stored (quarantined until approved). An over-budget model is refused with every failing number.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreatorItemModelResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_creator_item_model_check

> crate::models::CreateCreatorItemModelCheckResponse create_creator_item_model_check()
The budget report for a .glb (base64), storing nothing: every limit (triangles, draw calls, texture edge per phone/desktop; size, nodes, meshes, images, animations; bones for wearables) with pass/fail, problems and LOD suggestions.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateCreatorItemModelCheckResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_creator_item_model_submit

> crate::models::CreatorItemModelResponse create_creator_item_model_submit(id)
Send an uploaded model to art review.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreatorItemModelResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_creator_item_preview

> crate::models::CreateCreatorItemPreviewResponse create_creator_item_preview()
Upload a preview PNG (base64, at most 1 MB and 2048 px): parsed, scanned and stored on the Crossly CDN; answers its https URL.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateCreatorItemPreviewResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_creator_item_wearable

> crate::models::CreatorItemWearableResponse create_creator_item_wearable()
Start a wearable from a template: slug, name, colourways (panel -> #rrggbb), a COSM price and optional edition size. Text is filtered; answers the automated checks. Paid seller plans only.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreatorItemWearableResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_creator_item_wearable_submit

> crate::models::CreatorItemWearableResponse create_creator_item_wearable_submit(id)
Submit a wearable whose checks pass: each colourway becomes a COSM design (inactive) filed for review. Needs an https preview image (POST /v1/creator/items/previews).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreatorItemWearableResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_creator_review_hide

> crate::models::CreatorReviewResponse create_creator_review_hide(review_id)
Hide a buyer review from your public page (it is kept, and its rating still counts).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**review_id** | **String** |  | [required] |

### Return type

**crate::models::CreatorReviewResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_creator_review_reply

> crate::models::CreatorReviewResponse create_creator_review_reply(review_id)
Reply to a buyer review of your creation (or change your reply).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**review_id** | **String** |  | [required] |

### Return type

**crate::models::CreatorReviewResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_creator_review_report

> crate::models::CreatorReviewResponse create_creator_review_report(review_id)
Report a buyer review to Crossly (abuse, spam, not a real buyer).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**review_id** | **String** |  | [required] |

### Return type

**crate::models::CreatorReviewResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_creator_review_unhide

> crate::models::CreatorReviewResponse create_creator_review_unhide(review_id)
Show a hidden buyer review on your page again.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**review_id** | **String** |  | [required] |

### Return type

**crate::models::CreatorReviewResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_creator_card_set

> delete_creator_card_set(id)
Delete a set draft that was never published.

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


## delete_creator_item_model

> delete_creator_item_model(id)
Delete a draft or rejected model.

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


## delete_creator_item_wearable

> delete_creator_item_wearable(id)
Delete a draft wearable.

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


## get_creator_card_set

> crate::models::CreatorCardSetResponse get_creator_card_set(id)
One set draft with its check: draft issues by field and tcgStaticConformance problems.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreatorCardSetResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_creator_card_set_progress

> crate::models::GetCreatorCardSetProgressResponse get_creator_card_set_progress(id)
A set's three steps: live on COSM, the mint threshold toward NFTs, and the print-run cost (trading_cards estimate) against production funding banked.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetCreatorCardSetProgressResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_creator_collection

> crate::models::GetCreatorCollectionResponse get_creator_collection(collection_id)
One collection (a card set, a launch or a claimed contract): its six-stage road to the chain, the launch (phase timeline, proceeds, recent mints), sales by marketplace, royalty estimate, holders, its cards or creations with their earnings, holder perks set at mint, and which actions apply.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** |  | [required] |

### Return type

**crate::models::GetCreatorCollectionResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_creator_collection_perk

> crate::models::GetCreatorCollectionPerkResponse get_creator_collection_perk(collection_id)
A collection's holder perks (percent off at your shop, early access, a World item, a loyalty tier), whether they can be edited, and why not (a card set not started yet, a contract claimed from elsewhere).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** |  | [required] |

### Return type

**crate::models::GetCreatorCollectionPerkResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_creator_creation

> crate::models::GetCreatorCreationResponse get_creator_creation(format, id)
One creation: its status timeline and review history, the files it uses, where it sells, its sales, its buyer reviews and its mint status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

**crate::models::GetCreatorCreationResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_creator_creation_status

> crate::models::GetCreatorCreationStatusResponse get_creator_creation_status(format, id)
Where one creation stands: its status, the reviewer's last note and its status timeline.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

**crate::models::GetCreatorCreationStatusResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_creator_earning_export

> String get_creator_earning_export(format, rail, source, state, group, game_id, from, to)
Download your creator earnings: format=csv (every ledger row for the filter) or format=pdf (a statement with per-rail totals for the same filter: rail, source, group, gameId, state; period defaults to this month).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format** | Option<**String**> |  |  |
**rail** | Option<**String**> |  |  |
**source** | Option<**String**> |  |  |
**state** | Option<**String**> |  |  |
**group** | Option<**String**> |  |  |
**game_id** | Option<**String**> |  |  |
**from** | Option<**String**> |  |  |
**to** | Option<**String**> |  |  |

### Return type

**String**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/csv, application/pdf, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_creator_earning_reconcile

> crate::models::GetCreatorEarningReconcileResponse get_creator_earning_reconcile()
Reconcile your creator earnings: every source recomputed directly from its own ledger, side by side with the earnings view. ok is true when every source matches.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetCreatorEarningReconcileResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_creator_import

> crate::models::GetCreatorImportResponse get_creator_import(id)
A stored template validation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetCreatorImportResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_creator_item_model

> crate::models::CreatorItemModelResponse get_creator_item_model(id)
One of your models with its budget report.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreatorItemModelResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_creator_item_template

> crate::models::GetCreatorItemTemplateResponse get_creator_item_template()
Wearable templates: each garment the Crossly avatar wears, its recolourable panels, default colours and measured cost (triangles, draw calls, bones, bytes, phone LOD), plus the per-wearable budget.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetCreatorItemTemplateResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_creator_item_wearable

> crate::models::CreatorItemWearableResponse get_creator_item_wearable(id)
One of your wearables with its checks, review state and note.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreatorItemWearableResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_creator_minting

> crate::models::GetCreatorMintingResponse get_creator_minting()
Everything you take on chain as collection rows with a six-stage status (designed, eligible, drafted, deploying, live, ended): gettingReady and launched. Game items (source game_item: tradeable game products, NFT declarations for your games) carry href and gameId.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetCreatorMintingResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_creator_overview

> crate::models::GetCreatorOverviewResponse get_creator_overview()
Creator Studio overview: earnings this month and last by rail, live creations, review queue, publish lock and next steps.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetCreatorOverviewResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_creator_template

> crate::models::GetCreatorTemplateResponse get_creator_template(format)
The template files for a format (wearable, model, decor, interactive, card_set): crossly-creation.json, creations.csv where it applies, and a README.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format** | **String** |  | [required] |

### Return type

**crate::models::GetCreatorTemplateResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_creator_template_zip

> std::path::PathBuf get_creator_template_zip(format)
A format's template as a .zip download (the same files as GET /templates/:format, plus its empty folders).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format** | **String** |  | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/zip, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_creator_card_sets

> crate::models::ListCreatorCardSetsResponse list_creator_card_sets()
Your card set drafts, each with its road from COSM to print: live on COSM, the mint threshold (collectors, packs opened, resales) and the print-run cost against what NFT sales have banked.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListCreatorCardSetsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_creator_creation_reviews

> crate::models::ListCreatorCreationReviewsResponse list_creator_creation_reviews(format, id)
Buyer reviews of one creation, newest first (hidden ones included).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

**crate::models::ListCreatorCreationReviewsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_creator_creations

> crate::models::ListCreatorCreationsResponse list_creator_creations()
Every creation of your shop in one list: wearables, 3D objects, decor, interactive objects, each card of your card sets, Classic games and card games, with status, collection, rating, earnings and mint status.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListCreatorCreationsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_creator_earning_trend

> crate::models::ListCreatorEarningTrendResponse list_creator_earning_trend(bucket, rail, group, game_id, from, to)
Your creator earnings over time: net (available + paid) per day, week or month (UTC) and rail, for the same filter as the ledger. meta.totals holds the per-rail and per-source totals of that filter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bucket** | Option<**String**> |  |  |
**rail** | Option<**String**> |  |  |
**group** | Option<**String**> |  |  |
**game_id** | Option<**String**> |  |  |
**from** | Option<**String**> |  |  |
**to** | Option<**String**> |  |  |

### Return type

**crate::models::ListCreatorEarningTrendResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_creator_earnings

> crate::models::ListCreatorEarningsResponse list_creator_earnings(rail, source, group, game_id, state, from, to, cursor, limit)
Your creator earnings ledger across every rail (cash, COSM, CBX), newest first. Each row names the source ledger row it is. meta.totals holds per-rail and per-source totals for the same filter; meta.nextCursor pages.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rail** | Option<**String**> |  |  |
**source** | Option<**String**> |  |  |
**group** | Option<**String**> |  |  |
**game_id** | Option<**String**> |  |  |
**state** | Option<**String**> |  |  |
**from** | Option<**String**> |  |  |
**to** | Option<**String**> |  |  |
**cursor** | Option<**String**> |  |  |
**limit** | Option<**i32**> |  |  |

### Return type

**crate::models::ListCreatorEarningsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_creator_item_models

> crate::models::ListCreatorItemModelsResponse list_creator_item_models()
Your uploaded 3D models with their budget reports and review state.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListCreatorItemModelsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_creator_item_wearables

> crate::models::ListCreatorItemWearablesResponse list_creator_item_wearables()
Your wearables: drafts, in review, live or with changes requested, each with its automated checks and per-colourway designs.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListCreatorItemWearablesResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_creator_items

> crate::models::ListCreatorItemsResponse list_creator_items(kind, status, rail)
Every creation of your shop — wearables, decor, uploaded 3D models, interactive objects, Classic games, card games (incl. unpublished designer sets), physical runs — with status, price and earnings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**kind** | Option<**String**> |  |  |
**status** | Option<**String**> |  |  |
**rail** | Option<**String**> |  |  |

### Return type

**crate::models::ListCreatorItemsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_creator_reviews

> crate::models::ListCreatorReviewsResponse list_creator_reviews(kind, state, limit)
Every creator submission (items, interactive objects, Classic games, card games) and its review decision with the reviewer note.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**kind** | Option<**String**> |  |  |
**state** | Option<**String**> |  |  |
**limit** | Option<**i32**> |  |  |

### Return type

**crate::models::ListCreatorReviewsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_creator_card_set

> crate::models::CreatorCardSetResponse update_creator_card_set(id)
Save the whole draft: cards (name, type, rarity, rules text, flavour, power/draw/guard), COSM prices and supply, frame colours, print plan. Every word goes through the text filter. Art is merged: a card without `art` (or with art as read) keeps its stored art, `art: null` clears it; only the art upload sets art. Answers the check.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreatorCardSetResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_creator_collection_perk

> crate::models::GetCreatorCollectionPerkResponse update_creator_collection_perk(collection_id)
Replace a collection's holder perks: `{ perks: [...] }`, at most one of each type: percent_off { percent 1-50 }, early_access { hours 1-720 }, world_item { creationRef, name }, loyalty_tier { tierId, tierName } (one of your loyalty tiers), each with an optional note. 409 when the collection has no launch yet.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** |  | [required] |

### Return type

**crate::models::GetCreatorCollectionPerkResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_creator_item_wearable

> crate::models::CreatorItemWearableResponse update_creator_item_wearable(id)
Edit a draft wearable (name, description, colourways, price, edition). Submitted wearables are frozen.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreatorItemWearableResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

