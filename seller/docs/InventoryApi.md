# \InventoryApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_inventory**](InventoryApi.md#create_inventory) | **POST** /v1/inventory | Create a new inventory item.
[**create_inventory_bulk_archive**](InventoryApi.md#create_inventory_bulk_archive) | **POST** /v1/inventory/bulk-archive | Bulk archive inventory items (soft).
[**create_inventory_bulk_delete**](InventoryApi.md#create_inventory_bulk_delete) | **POST** /v1/inventory/bulk-delete | Bulk delete inventory items (delinks listings).
[**create_inventory_bulk_label**](InventoryApi.md#create_inventory_bulk_label) | **POST** /v1/inventory/bulk-labels | Bulk add/remove labels on inventory items.
[**create_inventory_bulk_quantity**](InventoryApi.md#create_inventory_bulk_quantity) | **POST** /v1/inventory/bulk-quantity | Set / add / subtract stock across many items, syncing live listings.
[**create_inventory_csv_export**](InventoryApi.md#create_inventory_csv_export) | **POST** /v1/inventory/csv/export | Export inventory as CSV. Round-trips back through csv/import.
[**create_inventory_csv_import**](InventoryApi.md#create_inventory_csv_import) | **POST** /v1/inventory/csv/import | Import a CSV. Rows whose sku matches an existing item update it; others are added. Pass dryRun to preview.
[**create_inventory_label_rename**](InventoryApi.md#create_inventory_label_rename) | **POST** /v1/inventory/labels/rename | Rename a label across every inventory item.
[**create_inventory_unit_identifier**](InventoryApi.md#create_inventory_unit_identifier) | **POST** /v1/inventory/{id}/units/identifiers | Record a serial, IMEI, or licence key against an inventory item.
[**create_inventory_unit_lookup**](InventoryApi.md#create_inventory_unit_lookup) | **POST** /v1/inventory/units/lookup | Find a unit by identifier.
[**delete_inventory**](InventoryApi.md#delete_inventory) | **DELETE** /v1/inventory/{id} | Soft-archive an inventory item.
[**get_inventory**](InventoryApi.md#get_inventory) | **GET** /v1/inventory/{id} | Get one inventory item with platform listings.
[**get_inventory_facet**](InventoryApi.md#get_inventory_facet) | **GET** /v1/inventory/facets | Distinct brands + categories across this user's inventory.
[**get_inventory_label**](InventoryApi.md#get_inventory_label) | **GET** /v1/inventory/labels | List every distinct label across this user's inventory.
[**get_inventory_label_stat**](InventoryApi.md#get_inventory_label_stat) | **GET** /v1/inventory/labels/stats | List distinct labels with usage counts + colors.
[**get_inventory_sku_exist**](InventoryApi.md#get_inventory_sku_exist) | **GET** /v1/inventory/sku-exists | Check whether a SKU is already in use on this user's inventory.
[**get_spatial_public**](InventoryApi.md#get_spatial_public) | **GET** /v1/spatial/public/{slug} | A shared room, as a visitor sees it.
[**get_spatial_scene**](InventoryApi.md#get_spatial_scene) | **GET** /v1/spatial/scenes/{id} | A solved room: every item, where it sits, and why.
[**list_inventory**](InventoryApi.md#list_inventory) | **GET** /v1/inventory | List inventory items.
[**list_inventory_activity**](InventoryApi.md#list_inventory_activity) | **GET** /v1/inventory/{id}/activity | Activity log for an inventory item (created/sold/edited/etc.).
[**list_inventory_ids**](InventoryApi.md#list_inventory_ids) | **GET** /v1/inventory/ids | Filter inventory → return matching id list.
[**list_inventory_units**](InventoryApi.md#list_inventory_units) | **GET** /v1/inventory/{id}/units | List the individually identified units of an inventory item.
[**list_spatial_public**](InventoryApi.md#list_spatial_public) | **GET** /v1/spatial/public | Public rooms anyone can walk into.
[**list_spatial_public_offers**](InventoryApi.md#list_spatial_public_offers) | **GET** /v1/spatial/public/{slug}/offers | What is for sale in a shared room.
[**list_spatial_scene_movements**](InventoryApi.md#list_spatial_scene_movements) | **GET** /v1/spatial/scenes/{id}/movements | Stock movements in a room over a time window.
[**list_spatial_scenes**](InventoryApi.md#list_spatial_scenes) | **GET** /v1/spatial/scenes | The rooms this account has.
[**update_inventory**](InventoryApi.md#update_inventory) | **PATCH** /v1/inventory/{id} | Update an inventory item (partial).



## create_inventory

> crate::models::CreateInventoryResponse create_inventory()
Create a new inventory item.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateInventoryResponse**](CreateInventoryResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_inventory_bulk_archive

> crate::models::CreateInventoryBulkArchiveResponse create_inventory_bulk_archive()
Bulk archive inventory items (soft).

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateInventoryBulkArchiveResponse**](CreateInventoryBulkArchiveResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_inventory_bulk_delete

> crate::models::CreateInventoryBulkDeleteResponse create_inventory_bulk_delete()
Bulk delete inventory items (delinks listings).

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateInventoryBulkDeleteResponse**](CreateInventoryBulkDeleteResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_inventory_bulk_label

> crate::models::CreateInventoryBulkLabelResponse create_inventory_bulk_label()
Bulk add/remove labels on inventory items.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateInventoryBulkLabelResponse**](CreateInventoryBulkLabelResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_inventory_bulk_quantity

> crate::models::CreateInventoryBulkQuantityResponse create_inventory_bulk_quantity()
Set / add / subtract stock across many items, syncing live listings.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateInventoryBulkQuantityResponse**](CreateInventoryBulkQuantityResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_inventory_csv_export

> String create_inventory_csv_export()
Export inventory as CSV. Round-trips back through csv/import.

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_inventory_csv_import

> crate::models::CreateInventoryCsvImportResponse create_inventory_csv_import()
Import a CSV. Rows whose sku matches an existing item update it; others are added. Pass dryRun to preview.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateInventoryCsvImportResponse**](CreateInventoryCsvImportResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_inventory_label_rename

> crate::models::CreateInventoryLabelRenameResponse create_inventory_label_rename()
Rename a label across every inventory item.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateInventoryLabelRenameResponse**](CreateInventoryLabelRenameResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_inventory_unit_identifier

> crate::models::CreateInventoryUnitIdentifierResponse create_inventory_unit_identifier(id)
Record a serial, IMEI, or licence key against an inventory item.

Creates the unit lazily if no `unitId` is given. Recording BEFORE the item sells is what makes the identifier usable as evidence on a return — one first recorded after a dispute opens is graded `weak` and says so.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::CreateInventoryUnitIdentifierResponse**](CreateInventoryUnitIdentifierResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_inventory_unit_lookup

> crate::models::CreateInventoryUnitLookupResponse create_inventory_unit_lookup()
Find a unit by identifier.

\"Have I ever seen this serial?\" — for when something arrives back and nobody knows which order it belongs to. Scoped to the caller, so it can never be used to probe another seller's stock.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateInventoryUnitLookupResponse**](CreateInventoryUnitLookupResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_inventory

> crate::models::DeleteInventoryResponse delete_inventory(id)
Soft-archive an inventory item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::DeleteInventoryResponse**](DeleteInventoryResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_inventory

> crate::models::GetInventoryResponse get_inventory(id)
Get one inventory item with platform listings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::GetInventoryResponse**](GetInventoryResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_inventory_facet

> crate::models::GetInventoryFacetResponse get_inventory_facet()
Distinct brands + categories across this user's inventory.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetInventoryFacetResponse**](GetInventoryFacetResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_inventory_label

> crate::models::GetInventoryLabelResponse get_inventory_label()
List every distinct label across this user's inventory.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetInventoryLabelResponse**](GetInventoryLabelResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_inventory_label_stat

> crate::models::GetInventoryLabelStatResponse get_inventory_label_stat()
List distinct labels with usage counts + colors.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetInventoryLabelStatResponse**](GetInventoryLabelStatResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_inventory_sku_exist

> crate::models::GetInventorySkuExistResponse get_inventory_sku_exist(sku)
Check whether a SKU is already in use on this user's inventory.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sku** | **String** |  | [required] |

### Return type

[**crate::models::GetInventorySkuExistResponse**](GetInventorySkuExistResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_spatial_public

> crate::models::GetSpatialPublicResponse get_spatial_public(slug)
A shared room, as a visitor sees it.

The room behind a share link: container geometry in METRES matching the real physical object, the solved arrangement, and one row per object on the shelves. REDACTED relative to the owner's view — no cost, no storage location, no listing status — so do not expect the fields /v1/spatial/scenes/{id} returns. Each placement carries `pinned`: true means a HUMAN put it there and it will not move; false means a layout SOLVER chose, and it may choose differently once the stock changes, so an unpinned placement is never a statement about where something physically is. `solved.overflow` lists what did not fit — a non-empty array means the room is INCOMPLETE and `stats.itemCount` exceeds what is on screen. Resolves rooms shared as `unlisted` as well as `public`: holding the link is the permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

[**crate::models::GetSpatialPublicResponse**](GetSpatialPublicResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_spatial_scene

> crate::models::GetSpatialSceneResponse get_spatial_scene(id)
A solved room: every item, where it sits, and why.

Returns the space profile (container geometry in METRES, matching the real physical object), the solved placements, and the items. Placements carry a `pinned` flag: true means a human put it there and the layout solver will not move it; false means the solver chose, and it may choose differently once the stock changes. `overflow` lists anything that did not fit — it is reported, never silently dropped.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::GetSpatialSceneResponse**](GetSpatialSceneResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_inventory

> crate::models::V1List list_inventory(page, limit, search, status)
List inventory items.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |[default to 1]
**limit** | Option<**i32**> |  |  |[default to 25]
**search** | Option<**String**> |  |  |
**status** | Option<**String**> |  |  |

### Return type

[**crate::models::V1List**](V1List.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_inventory_activity

> crate::models::V1List list_inventory_activity(id, limit)
Activity log for an inventory item (created/sold/edited/etc.).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**limit** | Option<**i32**> |  |  |[default to 50]

### Return type

[**crate::models::V1List**](V1List.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_inventory_ids

> crate::models::V1List list_inventory_ids()
Filter inventory → return matching id list.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::V1List**](V1List.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_inventory_units

> crate::models::V1List list_inventory_units(id)
List the individually identified units of an inventory item.

Each identifier carries a `strength` describing what it proves: `strong` was recorded before the item sold, `good` at packing, `weak` only after it shipped. The grade is derived from when it was recorded, never from the value itself.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::V1List**](V1List.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_spatial_public

> crate::models::V1List list_spatial_public()
Public rooms anyone can walk into.

The directory behind world-hopping. A room becomes public when its owner shares it; this lists those, newest first, with enough to draw a doorway AND enough to choose one — name, slug, category, itemCount, up to four previewImages, forSaleCount, a priceFromCents/priceToCents band and updatedAt. The band is the cheapest and dearest thing for sale in the room, never a quote for one object: /api/public/spatial/{slug}/offers is the authority on that. Fetch the room itself from /api/public/spatial/{slug}. Rooms with no items are omitted: an empty room is not a destination.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::V1List**](V1List.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_spatial_public_offers

> crate::models::V1List list_spatial_public_offers(slug)
What is for sale in a shared room.

Price, stock, condition and grade for everything in the room its owner is actually selling. Correlate to the room by `itemId`, which is the SAME id the scene payload publishes per item — never by title. An item in the room with no row here is not for sale; an empty array means the owner is showing the collection rather than selling it, which is a different answer from a 404 (no such shared room). SEPARATE CALL ON PURPOSE: the room's geometry is stable for minutes, a price is not — it changes whenever the seller edits a listing. Re-read this before quoting, and do not cache a price alongside a cached room. `priceCents` is CENTS. `available` is remaining stock, or null when the listing declares none; null is unknown, not zero. Where an item sits inside more than one active listing, the offer quoted is the one for that item alone rather than a bundle it belongs to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

[**crate::models::V1List**](V1List.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_spatial_scene_movements

> crate::models::V1List list_spatial_scene_movements(id)
Stock movements in a room over a time window.

One row per physical transition: which item, from which node, to which node, when, and of what kind (placed/moved/picked/shipped/received/removed). Nodes are referenced by id; the `fromCode`/`toCode` strings are display snapshots of the location code AT THE TIME and are not stable identifiers — correlate on the node ids. `since`/`until` are ISO timestamps, defaulting to the last seven days and clamped to 90. Movements are NOT attributed to individual team members on this surface: a token has no team role, so there is no honest way to decide whether its holder may see who did the work.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::V1List**](V1List.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_spatial_scenes

> crate::models::V1List list_spatial_scenes()
The rooms this account has.

One per market category the seller holds catalog-resolved stock in, plus a warehouse. Rooms are created on first read rather than requiring setup, so this call is safe to treat as the entry point.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::V1List**](V1List.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_inventory

> crate::models::UpdateInventoryResponse update_inventory(id)
Update an inventory item (partial).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::UpdateInventoryResponse**](UpdateInventoryResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

