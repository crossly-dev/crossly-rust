# \InventoryApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_catalog_match_confirm**](InventoryApi.md#create_catalog_match_confirm) | **POST** /v1/catalog-matches/{id}/confirm | Confirm a catalog-match suggestion: links the item to the catalog product (sets its market_sku_id), so sold comps and market pricing apply to it.
[**create_catalog_match_dismiss**](InventoryApi.md#create_catalog_match_dismiss) | **POST** /v1/catalog-matches/{id}/dismiss | Dismiss a catalog-match suggestion. That product is never suggested for this item again.
[**create_inventory**](InventoryApi.md#create_inventory) | **POST** /v1/inventory | Create a new inventory item.
[**create_inventory_bulk_archive**](InventoryApi.md#create_inventory_bulk_archive) | **POST** /v1/inventory/bulk-archive | Bulk archive inventory items (soft).
[**create_inventory_bulk_delete**](InventoryApi.md#create_inventory_bulk_delete) | **POST** /v1/inventory/bulk-delete | Bulk delete inventory items (delinks listings).
[**create_inventory_bulk_label**](InventoryApi.md#create_inventory_bulk_label) | **POST** /v1/inventory/bulk-labels | Bulk add/remove labels on inventory items.
[**create_inventory_bulk_quantity**](InventoryApi.md#create_inventory_bulk_quantity) | **POST** /v1/inventory/bulk-quantity | Set / add / subtract stock across many items, syncing live listings.
[**create_inventory_catalog_match_refresh**](InventoryApi.md#create_inventory_catalog_match_refresh) | **POST** /v1/inventory/{id}/catalog-matches/refresh | Run catalog matching for one item now. An identifier (UPC, style code, catalogue number) that resolves to exactly one product links it; otherwise likely products are returned as suggestions to confirm.
[**create_inventory_csv_export**](InventoryApi.md#create_inventory_csv_export) | **POST** /v1/inventory/csv/export | Export inventory as CSV. Round-trips back through csv/import.
[**create_inventory_csv_import**](InventoryApi.md#create_inventory_csv_import) | **POST** /v1/inventory/csv/import | Import a CSV. Rows whose sku matches an existing item update it; others are added. Pass dryRun to preview.
[**create_inventory_label_rename**](InventoryApi.md#create_inventory_label_rename) | **POST** /v1/inventory/labels/rename | Rename a label across every inventory item.
[**create_inventory_merge**](InventoryApi.md#create_inventory_merge) | **POST** /v1/inventory/merge | Merge inventory items that are the same physical item into one (stock is not summed).
[**create_inventory_unit_identifier**](InventoryApi.md#create_inventory_unit_identifier) | **POST** /v1/inventory/{id}/units/identifiers | Record a serial, IMEI, or licence key against an inventory item.
[**create_inventory_unit_lookup**](InventoryApi.md#create_inventory_unit_lookup) | **POST** /v1/inventory/units/lookup | Find a unit by identifier.
[**create_spatial_property_deed_lease**](InventoryApi.md#create_spatial_property_deed_lease) | **POST** /v1/spatial/property/deeds/{id}/lease | Offer your plot for lease.
[**create_spatial_property_deed_list**](InventoryApi.md#create_spatial_property_deed_list) | **POST** /v1/spatial/property/deeds/{id}/list | List your plot for resale.
[**create_spatial_property_deed_maintenance**](InventoryApi.md#create_spatial_property_deed_maintenance) | **POST** /v1/spatial/property/deeds/{id}/maintenance | Pay a plot's maintenance forward.
[**create_spatial_property_lease_accept**](InventoryApi.md#create_spatial_property_lease_accept) | **POST** /v1/spatial/property/leases/{id}/accept | Take a plot on lease.
[**create_spatial_property_lease_end**](InventoryApi.md#create_spatial_property_lease_end) | **POST** /v1/spatial/property/leases/{id}/end | End a lease (owner or tenant).
[**create_spatial_property_lease_rent**](InventoryApi.md#create_spatial_property_lease_rent) | **POST** /v1/spatial/property/leases/{id}/rent | Pay rent forward.
[**create_spatial_property_listing_buy**](InventoryApi.md#create_spatial_property_listing_buy) | **POST** /v1/spatial/property/listings/{id}/buy | Buy a plot on the resale market, in CBX.
[**create_spatial_property_listing_cancel**](InventoryApi.md#create_spatial_property_listing_cancel) | **POST** /v1/spatial/property/listings/{id}/cancel | Take your plot off the market.
[**create_spatial_shop_staff_pay**](InventoryApi.md#create_spatial_shop_staff_pay) | **POST** /v1/spatial/shop/staff/{grantId}/pay | Pay a staff member's wage forward.
[**create_spatial_street_claim**](InventoryApi.md#create_spatial_street_claim) | **POST** /v1/spatial/streets/{slug}/claim | Put your shop on a street.
[**create_spatial_street_owned_upkeep**](InventoryApi.md#create_spatial_street_owned_upkeep) | **POST** /v1/spatial/streets/owned/{id}/upkeep | Pay a street's upkeep forward.
[**create_spatial_street_plot_buy**](InventoryApi.md#create_spatial_street_plot_buy) | **POST** /v1/spatial/streets/{slug}/plots/{number}/buy | Buy a vacant plot from Crossly, in CBX.
[**create_spatial_street_request**](InventoryApi.md#create_spatial_street_request) | **POST** /v1/spatial/streets/requests | Ask to found a street.
[**create_spatial_street_request_found**](InventoryApi.md#create_spatial_street_request_found) | **POST** /v1/spatial/streets/requests/{id}/found | Found an approved street, in CBX.
[**delete_inventory**](InventoryApi.md#delete_inventory) | **DELETE** /v1/inventory/{id} | Soft-archive an inventory item.
[**delete_spatial_street_plot**](InventoryApi.md#delete_spatial_street_plot) | **DELETE** /v1/spatial/streets/{slug}/plot | Take your shop off a street.
[**get_game_leaderboard**](InventoryApi.md#get_game_leaderboard) | **GET** /v1/games/leaderboards | Game leaderboards: for each of Price Guess (wins, then points), scavenger hunts (finished) and speed trades (won), the top ten this week (Monday 00:00 UTC on) under week and of all time under all -- rank, name, named, value and, for Price Guess, score. A name appears only for somebody who chose to be seen by name in Crossly World; everyone else is \"A player\" in their place.
[**get_home**](InventoryApi.md#get_home) | **GET** /v1/homes/{slug} | A home by its address, as you may see it: open homes to anybody, friends-only homes to mutual follows, locked homes to their owner. 404 when you may not enter.
[**get_home_mine**](InventoryApi.md#get_home_mine) | **GET** /v1/homes/mine | Your Crossly World home: its address, privacy (open / friends / locked), theme, the room, the layout (each piece with what it shows -- things you no longer own are left out at read time) and the guestbook. Made on first read.
[**get_inventory**](InventoryApi.md#get_inventory) | **GET** /v1/inventory/{id} | Get one inventory item with platform listings.
[**get_inventory_aging**](InventoryApi.md#get_inventory_aging) | **GET** /v1/inventory/aging | Unsold stock by age (0-7, 7-30, 30-90, 90+ days, unlisted), each bucket with a recommendation.
[**get_inventory_catalog_match**](InventoryApi.md#get_inventory_catalog_match) | **GET** /v1/inventory/{id}/catalog-matches | One inventory item's catalog binding (if confirmed) and its pending match suggestions.
[**get_inventory_facet**](InventoryApi.md#get_inventory_facet) | **GET** /v1/inventory/facets | Distinct brands + categories across this user's inventory.
[**get_inventory_label**](InventoryApi.md#get_inventory_label) | **GET** /v1/inventory/labels | List every distinct label across this user's inventory.
[**get_inventory_label_stat**](InventoryApi.md#get_inventory_label_stat) | **GET** /v1/inventory/labels/stats | List distinct labels with usage counts + colors.
[**get_inventory_sell_through**](InventoryApi.md#get_inventory_sell_through) | **GET** /v1/inventory/sell-through | Cohort sell-through for a window (default: the 30 days ending 30 days ago) against the previous window of the same length.
[**get_inventory_sku_exist**](InventoryApi.md#get_inventory_sku_exist) | **GET** /v1/inventory/sku-exists | Check whether a SKU is already in use on this user's inventory.
[**get_inventory_value_history**](InventoryApi.md#get_inventory_value_history) | **GET** /v1/inventory/value-history | Portfolio value of unsold stock over time, oldest first (one point per nightly snapshot).
[**get_main_stage**](InventoryApi.md#get_main_stage) | **GET** /v1/main-stage | The Crossly World Main Stage right now: the live show (title, kind, host lineup), the stream on its big screen while a lineup host is on air, how many people are there (it holds 64 per copy and opens copies as it fills) and what is on next.
[**get_pet_mine**](InventoryApi.md#get_pet_mine) | **GET** /v1/pets/mine | Your Crossly World pets: every kind (cat, dog, slab bot) with whether you own it, when it was earned and the ways to earn it (a quest or a reputation level -- pets are never sold), plus the pet walking beside you and the one greeting visitors at your shop door.
[**get_reputation**](InventoryApi.md#get_reputation) | **GET** /v1/reputation/{profile} | Somebody's public reputation levels by their storefront handle: the level on each track and their nameplate badge. 404 for an unknown handle.
[**get_reputation_mine**](InventoryApi.md#get_reputation_mine) | **GET** /v1/reputation/mine | Your Crossly World reputation: four tracks (trader, collector, seller, host) with points, level, what each level has unlocked and the next unlock, and how the points were earned (recency-weighted; a lost dispute or a failed authentication costs points). Earned from real activity only; nothing buys a level.
[**get_spatial_address**](InventoryApi.md#get_spatial_address) | **GET** /v1/spatial/address/{street}/{number} | The shop at an address.
[**get_spatial_auction_public**](InventoryApi.md#get_spatial_auction_public) | **GET** /v1/spatial/auction/public/{slug} | A shared saleroom, opened by its public share handle.
[**get_spatial_auction_room**](InventoryApi.md#get_spatial_auction_room) | **GET** /v1/spatial/auction/rooms/{sceneId} | What the rostrum says in this room, right now.
[**get_spatial_emote**](InventoryApi.md#get_spatial_emote) | **GET** /v1/spatial/emotes | Your Crossly World emotes: every emote pack (unlocked or not, with the quest that unlocks it), your 8-slot emote wheel as stored and as it shows (labels, locks, ready dots), and the moments earned and not yet played.
[**get_spatial_employee_config**](InventoryApi.md#get_spatial_employee_config) | **GET** /v1/spatial/employee/config | How the shop employee in one of your rooms is configured.
[**get_spatial_party**](InventoryApi.md#get_spatial_party) | **GET** /v1/spatial/party | Your Crossly World party: party is null when you are in none, else its id, leader and up to 8 members (each with the place and copy they are in right now, or null), plus the invites it has out. invites lists the invites waiting for you.
[**get_spatial_property**](InventoryApi.md#get_spatial_property) | **GET** /v1/spatial/property | Your plots and tenancies.
[**get_spatial_public**](InventoryApi.md#get_spatial_public) | **GET** /v1/spatial/public/{slug} | A shared room, as a visitor sees it.
[**get_spatial_scene**](InventoryApi.md#get_spatial_scene) | **GET** /v1/spatial/scenes/{id} | A solved room: every item, where it sits, and why.
[**get_spatial_scene_warehouse**](InventoryApi.md#get_spatial_scene_warehouse) | **GET** /v1/spatial/scenes/{id}/warehouse | A warehouse room: your racks, where they stand, and what is on each.
[**get_spatial_shop_staff**](InventoryApi.md#get_spatial_shop_staff) | **GET** /v1/spatial/shop/staff | Your shop's staff and their wages.
[**get_spatial_street**](InventoryApi.md#get_spatial_street) | **GET** /v1/spatial/streets/{slug} | A street: every plot and the shop open on each.
[**get_spatial_street_founding**](InventoryApi.md#get_spatial_street_founding) | **GET** /v1/spatial/streets/founding | What founding a street costs.
[**get_spatial_street_plot_quote**](InventoryApi.md#get_spatial_street_plot_quote) | **GET** /v1/spatial/streets/{slug}/plots/{number}/quote | What a plot costs.
[**get_world_room_funnel**](InventoryApi.md#get_world_room_funnel) | **GET** /v1/world/rooms/{sceneId}/funnel | Your room's funnel over the last `days` (1-35, default 30): visitors (people, once a day), looks, items opened, checkouts started from the room, paid sales and revenueCents. Sales count only lines the room itself put in a cart.
[**get_world_shelf**](InventoryApi.md#get_world_shelf) | **GET** /v1/world/shelf/{listingSlug} | Where a Crossly listing sits on a shelf in a public room right now (roomSlug, roomName, itemId), or data:null when it is in no room.
[**get_world_skus_cheapest**](InventoryApi.md#get_world_skus_cheapest) | **GET** /v1/world/skus/{skuId}/cheapest | The cheapest copy of a catalogue piece for sale on Crossly now, matched by the catalogue id (listingSlug, priceCents, and the room and item when it stands on a shelf), or data:null.
[**list_catalog_matches**](InventoryApi.md#list_catalog_matches) | **GET** /v1/catalog-matches | Pending catalog-match suggestions: inventory items that look like a catalog product, awaiting your confirm.
[**list_festivals**](InventoryApi.md#list_festivals) | **GET** /v1/festivals | Live and upcoming Crossly World festivals, soonest first: a weekend when every street wears one decor and the festival quest line (limited pins) counts. meta.live is the festival on now, if any.
[**list_game_break_upcoming**](InventoryApi.md#list_game_break_upcoming) | **GET** /v1/games/breaks/upcoming | Pack breaks staged on shop stages in Crossly World, live first then soonest: the break, its shop, start time, spots (taken or not, buyer masked), hits revealed so far, and buyUrl -- the break's page on crossly.net, the only place a spot is bought (null once every spot has sold).
[**list_game_hunts**](InventoryApi.md#list_game_hunts) | **GET** /v1/games/hunts | Scavenger hunts running now in Crossly World shops and pop-up stalls, ending soonest first: title, shop (name and slug -- for a hunt at a stall, the street slug, with stall.bookingId set), how many tokens, what finishing unlocks (\"10% off\" -- never the code), when it ends and how many people have finished. Optional place filters to one shop or street slug.
[**list_game_result_mine**](InventoryApi.md#list_game_result_mine) | **GET** /v1/games/results/mine | Your market-game results, newest first: game (price_guess, price_guess_solo, hunt, speed_trade, break_host), the round/hunt/trade/break id, score, whether you won, Bucks prize in cents (0 while prizes are pins only) and detail. meta.unlocks lists the hunt coupons you earned (with their codes); meta.hits the break hits pulled for spots you bought, with the order each ships with.
[**list_home_mine_builders**](InventoryApi.md#list_home_mine_builders) | **GET** /v1/homes/mine/builders | Your co-builders: friends you let rearrange your home in the world (move, turn, take out your things, add free furniture; never your bought pieces). Each: userId, name, friends (false once the friendship ended: listed but powerless).
[**list_home_of_the_week**](InventoryApi.md#list_home_of_the_week) | **GET** /v1/homes/of-the-week | Home of the week: open homes by how many different people signed their guestbook in the last 7 days, most first.
[**list_inventory**](InventoryApi.md#list_inventory) | **GET** /v1/inventory | List inventory items.
[**list_inventory_activity**](InventoryApi.md#list_inventory_activity) | **GET** /v1/inventory/{id}/activity | Activity log for an inventory item (created/sold/edited/etc.).
[**list_inventory_ids**](InventoryApi.md#list_inventory_ids) | **GET** /v1/inventory/ids | Filter inventory → return matching id list.
[**list_inventory_units**](InventoryApi.md#list_inventory_units) | **GET** /v1/inventory/{id}/units | List the individually identified units of an inventory item.
[**list_job_appraisals**](InventoryApi.md#list_job_appraisals) | **GET** /v1/jobs/appraisals | Open price checks appraisers can answer (Collector level 3+ holding 3+ pieces in the category), newest first, optionally one market category. Each has its bounty; meta.payOn says whether bounties are paid yet.
[**list_job_shifts**](InventoryApi.md#list_job_shifts) | **GET** /v1/jobs/shifts | Open shifts on the Crossly World Jobs board, soonest first: shop staff shifts in boutiques and grading-desk helper shifts, each with its window, place and pay. meta.payOn says whether Crossly is paying jobs yet.
[**list_main_stage_events**](InventoryApi.md#list_main_stage_events) | **GET** /v1/main-stage/events | Live and upcoming Main Stage events, soonest first: brand launches, creator meetups, talks, live sales and festival shows, each with its window and host lineup.
[**list_spatial_auction_room_lots**](InventoryApi.md#list_spatial_auction_room_lots) | **GET** /v1/spatial/auction/rooms/{sceneId}/lots | The catalogue: every lot in the session this room is hosting.
[**list_spatial_auction_rooms**](InventoryApi.md#list_spatial_auction_rooms) | **GET** /v1/spatial/auction/rooms | Rooms of yours that are hosting an auction.
[**list_spatial_deed_listings**](InventoryApi.md#list_spatial_deed_listings) | **GET** /v1/spatial/deeds/listings | Plots for resale.
[**list_spatial_employee_sessions**](InventoryApi.md#list_spatial_employee_sessions) | **GET** /v1/spatial/employee/sessions | The itemised bill: one row per customer-conversation.
[**list_spatial_employee_usage**](InventoryApi.md#list_spatial_employee_usage) | **GET** /v1/spatial/employee/usage | What your shop employees cost over a period, per room.
[**list_spatial_moments**](InventoryApi.md#list_spatial_moments) | **GET** /v1/spatial/moments | Your moment emotes, newest first: unbox (a Crossly purchase delivered), slab_flip (a card graded through Crossly Grading), handshake (a trade-table shake settled). Each plays once in a room; playedAt is null until then. ready=true lists only the unplayed ones.
[**list_spatial_party_wishlist**](InventoryApi.md#list_spatial_party_wishlist) | **GET** /v1/spatial/party/wishlist | Your party's shared wishlist, newest first: each listing (title, price in cents, image, shop slug while it is live), who added it, and every member's thumb up or down with the totals. Empty when you are in no party.
[**list_spatial_public**](InventoryApi.md#list_spatial_public) | **GET** /v1/spatial/public | Public rooms anyone can walk into.
[**list_spatial_public_offers**](InventoryApi.md#list_spatial_public_offers) | **GET** /v1/spatial/public/{slug}/offers | What is for sale in a shared room.
[**list_spatial_scene_movements**](InventoryApi.md#list_spatial_scene_movements) | **GET** /v1/spatial/scenes/{id}/movements | Stock movements in a room over a time window.
[**list_spatial_scenes**](InventoryApi.md#list_spatial_scenes) | **GET** /v1/spatial/scenes | The rooms this account has.
[**list_spatial_shop_addresses**](InventoryApi.md#list_spatial_shop_addresses) | **GET** /v1/spatial/shop/addresses | Your shop's addresses.
[**list_spatial_staffing**](InventoryApi.md#list_spatial_staffing) | **GET** /v1/spatial/staffing | Shops you work in.
[**list_spatial_street_mine**](InventoryApi.md#list_spatial_street_mine) | **GET** /v1/spatial/streets/mine | Your streets.
[**list_spatial_streets**](InventoryApi.md#list_spatial_streets) | **GET** /v1/spatial/streets | Every street in the world.
[**list_world_blocks**](InventoryApi.md#list_world_blocks) | **GET** /v1/world/blocks | The people you blocked in Crossly World, as you saw them (label, never the account behind a pseudonym), oldest first. Blocks are made in a room, on the person.
[**list_world_item_search**](InventoryApi.md#list_world_item_search) | **GET** /v1/world/items/search | Find an item on a shelf anywhere in Crossly World: what public rooms have for sale now, by title or catalogue name (q, 2+ characters). Each hit names the room (roomSlug) and the item (itemId); /c/<roomSlug>?item=<itemId> opens it there.
[**list_world_room_wants**](InventoryApi.md#list_world_room_wants) | **GET** /v1/world/rooms/{sceneId}/wants | Items your room shows but does not sell that visitors said they would buy (\"I'd buy this\"), most wanted first: itemId and how many people.
[**update_inventory**](InventoryApi.md#update_inventory) | **PATCH** /v1/inventory/{id} | Update an inventory item (partial).
[**update_spatial_employee_config**](InventoryApi.md#update_spatial_employee_config) | **PUT** /v1/spatial/employee/config | Create or replace the shop employee in one of your rooms.
[**update_spatial_shop_staff_wage**](InventoryApi.md#update_spatial_shop_staff_wage) | **PUT** /v1/spatial/shop/staff/{grantId}/wage | Set a staff member's weekly wage.



## create_catalog_match_confirm

> crate::models::CreateCatalogMatchConfirmResponse create_catalog_match_confirm(id)
Confirm a catalog-match suggestion: links the item to the catalog product (sets its market_sku_id), so sold comps and market pricing apply to it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateCatalogMatchConfirmResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_catalog_match_dismiss

> crate::models::CreateCatalogMatchConfirmResponse create_catalog_match_dismiss(id)
Dismiss a catalog-match suggestion. That product is never suggested for this item again.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateCatalogMatchConfirmResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_inventory

> crate::models::CreateInventoryResponse create_inventory()
Create a new inventory item.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateInventoryResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_inventory_bulk_archive

> crate::models::CreateInventoryBulkArchiveResponse create_inventory_bulk_archive()
Bulk archive inventory items (soft).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateInventoryBulkArchiveResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_inventory_bulk_delete

> crate::models::CreateInventoryBulkDeleteResponse create_inventory_bulk_delete()
Bulk delete inventory items (delinks listings).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateInventoryBulkDeleteResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_inventory_bulk_label

> crate::models::CreateListingBulkUpdateResponse create_inventory_bulk_label()
Bulk add/remove labels on inventory items.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateListingBulkUpdateResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_inventory_bulk_quantity

> crate::models::CreateInventoryBulkQuantityResponse create_inventory_bulk_quantity()
Set / add / subtract stock across many items, syncing live listings.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateInventoryBulkQuantityResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_inventory_catalog_match_refresh

> crate::models::CreateInventoryCatalogMatchRefreshResponse create_inventory_catalog_match_refresh(id)
Run catalog matching for one item now. An identifier (UPC, style code, catalogue number) that resolves to exactly one product links it; otherwise likely products are returned as suggestions to confirm.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateInventoryCatalogMatchRefreshResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


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

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_inventory_csv_import

> crate::models::CreateInventoryCsvImportResponse create_inventory_csv_import()
Import a CSV. Rows whose sku matches an existing item update it; others are added. Pass dryRun to preview.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateInventoryCsvImportResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_inventory_label_rename

> crate::models::CreateInventoryLabelRenameResponse create_inventory_label_rename()
Rename a label across every inventory item.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateInventoryLabelRenameResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_inventory_merge

> crate::models::CreateInventoryMergeResponse create_inventory_merge()
Merge inventory items that are the same physical item into one (stock is not summed).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateInventoryMergeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_inventory_unit_identifier

> crate::models::CreateInventoryUnitIdentifierResponse create_inventory_unit_identifier(id)
Record a serial, IMEI, or licence key against an inventory item.

Creates the unit lazily if no `unitId` is given. Recording BEFORE the item sells is what makes the identifier usable as evidence on a return — one first recorded after a dispute opens is graded `weak` and says so.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateInventoryUnitIdentifierResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_inventory_unit_lookup

> crate::models::CreateInventoryUnitLookupResponse create_inventory_unit_lookup()
Find a unit by identifier.

\"Have I ever seen this serial?\" — for when something arrives back and nobody knows which order it belongs to. Scoped to the caller, so it can never be used to probe another seller's stock.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateInventoryUnitLookupResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_spatial_property_deed_lease

> crate::models::CreateSpatialPropertyDeedLeaseResponse create_spatial_property_deed_lease(id)
Offer your plot for lease.

Body `{ rentWeeklyCents }` ($1-$5,000). The plot must be vacant and its maintenance current. Terms freeze when a tenant accepts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateSpatialPropertyDeedLeaseResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_spatial_property_deed_list

> crate::models::CreateSpatialPropertyDeedListResponse create_spatial_property_deed_list(id)
List your plot for resale.

Body `{ priceCents }` ($1-$100,000). The deed's maintenance must be current. A lease on it carries on with the buyer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateSpatialPropertyDeedListResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_spatial_property_deed_maintenance

> crate::models::CreateSpatialPropertyDeedMaintenanceResponse create_spatial_property_deed_maintenance(id)
Pay a plot's maintenance forward.

Body `{ weeks }` (1-12). Extends `paidThrough` from the later of now and the current date.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateSpatialPropertyDeedMaintenanceResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_spatial_property_lease_accept

> crate::models::CreateSpatialPropertyLeaseAcceptResponse create_spatial_property_lease_accept(id)
Take a plot on lease.

Pays the first week to the owner from your EARNED CBX and moves your (public) shop onto the plot.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateSpatialPropertyLeaseAcceptResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_spatial_property_lease_end

> crate::models::CreateSpatialPropertyLeaseEndResponse create_spatial_property_lease_end(id)
End a lease (owner or tenant).

The tenant's shop gets a free Main Street address again.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateSpatialPropertyLeaseEndResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_spatial_property_lease_rent

> crate::models::CreateSpatialPropertyLeaseRentResponse create_spatial_property_lease_rent(id)
Pay rent forward.

Body `{ weeks }` (1-12). While the owner's deed is delinquent rent is WAIVED (`waived: true`), never collected.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateSpatialPropertyLeaseRentResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_spatial_property_listing_buy

> crate::models::CreateSpatialPropertyListingBuyResponse create_spatial_property_listing_buy(id)
Buy a plot on the resale market, in CBX.

Pays the asking price from your EARNED CBX to the seller (Crossly takes the 2% skim). The deed, its maintenance clock and any lease pass to you; your shop moves onto the plot if it is vacant. 409 if it has just sold.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateSpatialPropertyListingBuyResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_spatial_property_listing_cancel

> crate::models::DeleteWorldJobShiftResponse create_spatial_property_listing_cancel(id)
Take your plot off the market.

Only a listing nobody is buying right now.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::DeleteWorldJobShiftResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_spatial_shop_staff_pay

> crate::models::CreateSpatialShopStaffPayResponse create_spatial_shop_staff_pay(grant_id)
Pay a staff member's wage forward.

Body `{ weeks }` (1-4). Paid from your EARNED CBX to theirs. 403 while paying staff is not open.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**grant_id** | **String** |  | [required] |

### Return type

**crate::models::CreateSpatialShopStaffPayResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_spatial_street_claim

> crate::models::CreateSpatialStreetClaimResponse create_spatial_street_claim(slug)
Put your shop on a street.

Seats your shop on the lowest vacant plot of an OPEN street and returns its `index` and `address`; calling it again returns the plot you already have. Your shop must be public (400 otherwise). A street that takes shops by application or invitation answers 400; a full street you do not own answers 409. Main Street grows instead of filling.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::CreateSpatialStreetClaimResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_spatial_street_owned_upkeep

> crate::models::CreateSpatialStreetOwnedUpkeepResponse create_spatial_street_owned_upkeep(id)
Pay a street's upkeep forward.

Body `{ weeks }` (1-12). Extends `paidThrough` from the later of now and the current date.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateSpatialStreetOwnedUpkeepResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_spatial_street_plot_buy

> crate::models::CreateSpatialStreetPlotBuyResponse create_spatial_street_plot_buy(slug, number)
Buy a vacant plot from Crossly, in CBX.

Pays the quoted price from your EARNED CBX at spot and moves your shop onto the plot. Includes the first weeks of maintenance. 409 if the plot is taken or was just bought; 403 while property is not open.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |
**number** | **String** |  | [required] |

### Return type

**crate::models::CreateSpatialStreetPlotBuyResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_spatial_street_request

> crate::models::SpatialStreetResponse create_spatial_street_request()
Ask to found a street.

Body `{ name, slug?, admission? }`. Free: the name (3-32 letters, numbers, spaces and ' & . -) and its web address (derived from the name when omitted) are held for Crossly's review. Crossly's own streets and room addresses are never available. You need a public shop, and hold one street at most. 409 when the address is taken or you already hold one.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::SpatialStreetResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_spatial_street_request_found

> crate::models::SpatialStreetResponse create_spatial_street_request_found(id)
Found an approved street, in CBX.

Pays the founding premium from your EARNED CBX at spot and opens the street: its plots exist, the first weeks of upkeep are included, and your shop stands on it. 403 \"CBX opens at mint\" until the token is live; 409 before approval.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::SpatialStreetResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_inventory

> crate::models::OkResponse delete_inventory(id)
Soft-archive an inventory item.

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


## delete_spatial_street_plot

> crate::models::CreateWorldCrewLeaveResponse delete_spatial_street_plot(slug)
Take your shop off a street.

The plot goes vacant and its address may be taken by another shop. `left: false` when your shop was not on it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldCrewLeaveResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_game_leaderboard

> crate::models::GetGameLeaderboardResponse get_game_leaderboard()
Game leaderboards: for each of Price Guess (wins, then points), scavenger hunts (finished) and speed trades (won), the top ten this week (Monday 00:00 UTC on) under week and of all time under all -- rank, name, named, value and, for Price Guess, score. A name appears only for somebody who chose to be seen by name in Crossly World; everyone else is \"A player\" in their place.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetGameLeaderboardResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_home

> crate::models::GetHomeResponse get_home(slug)
A home by its address, as you may see it: open homes to anybody, friends-only homes to mutual follows, locked homes to their owner. 404 when you may not enter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::GetHomeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_home_mine

> crate::models::GetHomeResponse get_home_mine()
Your Crossly World home: its address, privacy (open / friends / locked), theme, the room, the layout (each piece with what it shows -- things you no longer own are left out at read time) and the guestbook. Made on first read.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetHomeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_inventory

> crate::models::GetInventoryResponse get_inventory(id)
Get one inventory item with platform listings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetInventoryResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_inventory_aging

> crate::models::GetInventoryAgingResponse get_inventory_aging()
Unsold stock by age (0-7, 7-30, 30-90, 90+ days, unlisted), each bucket with a recommendation.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetInventoryAgingResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_inventory_catalog_match

> crate::models::GetInventoryCatalogMatchResponse get_inventory_catalog_match(id)
One inventory item's catalog binding (if confirmed) and its pending match suggestions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetInventoryCatalogMatchResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_inventory_facet

> crate::models::GetListingFacetResponse get_inventory_facet()
Distinct brands + categories across this user's inventory.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetListingFacetResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_inventory_label

> crate::models::GetInventoryLabelResponse get_inventory_label()
List every distinct label across this user's inventory.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetInventoryLabelResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_inventory_label_stat

> crate::models::GetInventoryLabelStatResponse get_inventory_label_stat()
List distinct labels with usage counts + colors.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetInventoryLabelStatResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_inventory_sell_through

> crate::models::GetInventorySellThroughResponse get_inventory_sell_through()
Cohort sell-through for a window (default: the 30 days ending 30 days ago) against the previous window of the same length.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetInventorySellThroughResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_inventory_sku_exist

> crate::models::GetListingSkuExistResponse get_inventory_sku_exist(sku)
Check whether a SKU is already in use on this user's inventory.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sku** | **String** |  | [required] |

### Return type

**crate::models::GetListingSkuExistResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_inventory_value_history

> crate::models::GetInventoryValueHistoryResponse get_inventory_value_history()
Portfolio value of unsold stock over time, oldest first (one point per nightly snapshot).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetInventoryValueHistoryResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_main_stage

> crate::models::GetMainStageResponse get_main_stage()
The Crossly World Main Stage right now: the live show (title, kind, host lineup), the stream on its big screen while a lineup host is on air, how many people are there (it holds 64 per copy and opens copies as it fills) and what is on next.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetMainStageResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_pet_mine

> crate::models::GetPetMineResponse get_pet_mine()
Your Crossly World pets: every kind (cat, dog, slab bot) with whether you own it, when it was earned and the ways to earn it (a quest or a reputation level -- pets are never sold), plus the pet walking beside you and the one greeting visitors at your shop door.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetPetMineResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_reputation

> crate::models::GetReputationResponse get_reputation(profile)
Somebody's public reputation levels by their storefront handle: the level on each track and their nameplate badge. 404 for an unknown handle.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile** | **String** |  | [required] |

### Return type

**crate::models::GetReputationResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_reputation_mine

> crate::models::GetReputationMineResponse get_reputation_mine()
Your Crossly World reputation: four tracks (trader, collector, seller, host) with points, level, what each level has unlocked and the next unlock, and how the points were earned (recency-weighted; a lost dispute or a failed authentication costs points). Earned from real activity only; nothing buys a level.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetReputationMineResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_spatial_address

> crate::models::GetSpatialAddressResponse get_spatial_address(street, number)
The shop at an address.

Resolves `<street>/<number>` to the shop's `publicSlug` and `name`. 404 when the plot is vacant or its shop is not public.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**street** | **String** |  | [required] |
**number** | **String** |  | [required] |

### Return type

**crate::models::GetSpatialAddressResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_spatial_auction_public

> crate::models::GetSpatialAuctionPublicResponse get_spatial_auction_public(slug)
A shared saleroom, opened by its public share handle.

The bridge between the public room directory and this auction surface: /api/v1/spatial/public hands out SLUGS, while everything else here is keyed by scene id, and the two are deliberately not interchangeable. Resolves `public` rooms ONLY — an `unlisted` room is a bearer link, and crowd-shaped features stay off in those — so a slug that works on /api/v1/spatial/public/{slug} may still answer 404 here. The reply carries the `sceneId` you then pass to the room state, lots and bid endpoints. Same reading rules as the room state: amounts in CENTS, `minimumNextBidCents` rather than your own arithmetic, no reserve amount.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::GetSpatialAuctionPublicResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_spatial_auction_room

> crate::models::GetSpatialAuctionRoomResponse get_spatial_auction_room(scene_id)
What the rostrum says in this room, right now.

Serves whoever may ENTER the room, not only its owner — a saleroom restricted to the host would leave every visitor polling a 404 while standing in it. `hosting: false` with a null `auction` is a described empty answer, not an error. READ `currentLot.minimumNextBidCents`, never `currentBidCents + bidIncrementCents`: the first bid on a lot is floored at the START price rather than start + step, so deriving it locally raises the entry price on any lot with a real increment. All amounts are CENTS. `reserveMet` says whether the seller's floor has been cleared; the floor ITSELF is never published, because a crowd that can read it bids to it and stops. `serverTime` is this server's clock, handed over so you can age the session without trusting your own — there is no per-lot deadline to report, because a lot closes when a human drops the hammer and not when a timer expires. A room you may not enter answers 404, the same as one that does not exist.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scene_id** | **String** |  | [required] |

### Return type

**crate::models::GetSpatialAuctionRoomResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_spatial_emote

> crate::models::GetSpatialEmoteResponse get_spatial_emote()
Your Crossly World emotes: every emote pack (unlocked or not, with the quest that unlocks it), your 8-slot emote wheel as stored and as it shows (labels, locks, ready dots), and the moments earned and not yet played.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetSpatialEmoteResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_spatial_employee_config

> crate::models::GetSpatialEmployeeConfigResponse get_spatial_employee_config(scene_id)
How the shop employee in one of your rooms is configured.

Owner-only, and 404 for a room this account does not own — the same answer a room that does not exist gives, so this cannot be used to discover scene ids. `config: null` means the room has never had an employee configured, which is different from one that exists and is switched off (`enabled: false`). THE SIGNING SECRET IS NEVER RETURNED, here or anywhere else. It is encrypted at rest and handed back in plaintext exactly once, in the response to the PUT that minted it; `hasAgentSecret` is the only thing any read path will say about it, and re-reading it is impossible by construction rather than by omission. `suspendedAt` non-null means the employee is off the floor because a charge was refused — `suspendedReason` says which — and saving the config again clears it. `brainKind` is idle (stands there), rules (the list in `rules`, evaluated in order, first match wins) or agent (your own endpoint decides).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scene_id** | **String** |  | [required] |

### Return type

**crate::models::GetSpatialEmployeeConfigResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_spatial_party

> crate::models::GetSpatialPartyResponse get_spatial_party()
Your Crossly World party: party is null when you are in none, else its id, leader and up to 8 members (each with the place and copy they are in right now, or null), plus the invites it has out. invites lists the invites waiting for you.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetSpatialPartyResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_spatial_property

> crate::models::GetSpatialPropertyResponse get_spatial_property()
Your plots and tenancies.

Each deed with its address, price, weekly maintenance, `paidThrough`, `status` (active, grace, delinquent — a delinquent owner cannot collect rent; past three weeks late a deed is foreclosed) and any lease; plus the leases you rent as a tenant.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetSpatialPropertyResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_spatial_public

> crate::models::GetSpatialPublicResponse get_spatial_public(slug)
A shared room, as a visitor sees it.

The room behind a share link: container geometry in METRES matching the real physical object, the solved arrangement, and one row per object on the shelves. REDACTED relative to the owner's view — no cost, no storage location, no listing status — so do not expect the fields /v1/spatial/scenes/{id} returns. Each placement carries `pinned`: true means a HUMAN put it there and it will not move; false means a layout SOLVER chose, and it may choose differently once the stock changes, so an unpinned placement is never a statement about where something physically is. `solved.overflow` lists what did not fit — a non-empty array means the room is INCOMPLETE and `stats.itemCount` exceeds what is on screen. Resolves rooms shared as `unlisted` as well as `public`: holding the link is the permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::GetSpatialPublicResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_spatial_scene

> crate::models::GetSpatialSceneResponse get_spatial_scene(id)
A solved room: every item, where it sits, and why.

Returns the space profile (container geometry in METRES, matching the real physical object), the solved placements, and the items. Placements carry a `pinned` flag: true means a human put it there and the layout solver will not move it; false means the solver chose, and it may choose differently once the stock changes. `overflow` lists anything that did not fit — it is reported, never silently dropped.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetSpatialSceneResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_spatial_scene_warehouse

> crate::models::GetSpatialSceneWarehouseResponse get_spatial_scene_warehouse(id)
A warehouse room: your racks, where they stand, and what is on each.

For a room of kind `warehouse`. Racks are your own storage units (pallet racks, shelving, garment rails, floor totes) with their location `code` (the one written to each item's storage location when stock is placed), a floor position in METRES, and `capacity` / `stocked` / `fill`. `positioned: false` means you never stood the rack anywhere and it was laid out for you. Each rack lists the items on it with the full code of the level or bin they sit in. A non-warehouse room is a 400.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetSpatialSceneWarehouseResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_spatial_shop_staff

> crate::models::GetSpatialShopStaffResponse get_spatial_shop_staff()
Your shop's staff and their wages.

Staff are people holding a `staff` grant on your shop (invite them through the room-sharing flow). They may arrange the shop and move its stock, nothing else. `wageWeeklyCents` is paid by you in CBX; `pending` is an invite not yet accepted.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetSpatialShopStaffResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_spatial_street

> crate::models::GetSpatialStreetResponse get_spatial_street(slug)
A street: every plot and the shop open on each.

Plots in order from the entrance. Each has its `index` (0-based), house `number` (1-based), `address` (`<street>/<number>`, stable for as long as the shop stands there) and `label` (\"12 Main Street\"). `shop` is the open shop's directory entry, or null for a vacant plot; `shuttered: true` is a plot whose shop is not public right now — it keeps its address. `reserved` plots are newcomer plots: occupiable, never sold or leased. Each open shop's plot carries `life`: `soldToday` (the seller's sales today, America/New_York) and `window` (up to three live listings: listingId, slug, title, priceCents, imageUrl). The street carries `life.billboards` (promoted listings, paid per sale, then Crossly house ads) and `life.buskers` (shop owners on it who are live now).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::GetSpatialStreetResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_spatial_street_founding

> crate::models::GetSpatialStreetFoundingResponse get_spatial_street_founding()
What founding a street costs.

The founding premium `priceCents` (paid in CBX at spot, from EARNED balance), weekly upkeep `maintenanceWeeklyCents`, `includedWeeks` of upkeep included, `plots` on a founded street, `maxPerOwner`, the `admissions` an owner may pick (open | apply | invite), `cbx` {open, reason} -- closed until mint -- and, for the token owner, `refusal`: why they cannot ask yet (no public shop, already holding a street), or null.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetSpatialStreetFoundingResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_spatial_street_plot_quote

> crate::models::GetSpatialStreetPlotQuoteResponse get_spatial_street_plot_quote(slug, number)
What a plot costs.

Price in CENTS (paid in CBX at spot), weekly maintenance in cents, weeks included with a purchase, and whether it can be bought now. Newcomer plots (1-2) are never sold.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |
**number** | **String** |  | [required] |

### Return type

**crate::models::GetSpatialStreetPlotQuoteResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_room_funnel

> crate::models::GetWorldRoomFunnelResponse get_world_room_funnel(scene_id, days)
Your room's funnel over the last `days` (1-35, default 30): visitors (people, once a day), looks, items opened, checkouts started from the room, paid sales and revenueCents. Sales count only lines the room itself put in a cart.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scene_id** | **String** |  | [required] |
**days** | Option<**i32**> |  |  |

### Return type

**crate::models::GetWorldRoomFunnelResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_shelf

> crate::models::GetWorldShelfResponse get_world_shelf(listing_slug)
Where a Crossly listing sits on a shelf in a public room right now (roomSlug, roomName, itemId), or data:null when it is in no room.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**listing_slug** | **String** |  | [required] |

### Return type

**crate::models::GetWorldShelfResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_skus_cheapest

> crate::models::GetWorldSkusCheapestResponse get_world_skus_cheapest(sku_id)
The cheapest copy of a catalogue piece for sale on Crossly now, matched by the catalogue id (listingSlug, priceCents, and the room and item when it stands on a shelf), or data:null.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sku_id** | **String** |  | [required] |

### Return type

**crate::models::GetWorldSkusCheapestResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_catalog_matches

> crate::models::ListCatalogMatchesResponse list_catalog_matches(limit, before)
Pending catalog-match suggestions: inventory items that look like a catalog product, awaiting your confirm.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> |  |  |
**before** | Option<**String**> | Keyset cursor: createdAt of the last row seen. |  |

### Return type

**crate::models::ListCatalogMatchesResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_festivals

> crate::models::ListFestivalsResponse list_festivals()
Live and upcoming Crossly World festivals, soonest first: a weekend when every street wears one decor and the festival quest line (limited pins) counts. meta.live is the festival on now, if any.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListFestivalsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_game_break_upcoming

> crate::models::ListGameBreakUpcomingResponse list_game_break_upcoming()
Pack breaks staged on shop stages in Crossly World, live first then soonest: the break, its shop, start time, spots (taken or not, buyer masked), hits revealed so far, and buyUrl -- the break's page on crossly.net, the only place a spot is bought (null once every spot has sold).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListGameBreakUpcomingResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_game_hunts

> crate::models::ListGameHuntsResponse list_game_hunts(place)
Scavenger hunts running now in Crossly World shops and pop-up stalls, ending soonest first: title, shop (name and slug -- for a hunt at a stall, the street slug, with stall.bookingId set), how many tokens, what finishing unlocks (\"10% off\" -- never the code), when it ends and how many people have finished. Optional place filters to one shop or street slug.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**place** | Option<**String**> |  |  |

### Return type

**crate::models::ListGameHuntsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_game_result_mine

> crate::models::ListGameResultMineResponse list_game_result_mine()
Your market-game results, newest first: game (price_guess, price_guess_solo, hunt, speed_trade, break_host), the round/hunt/trade/break id, score, whether you won, Bucks prize in cents (0 while prizes are pins only) and detail. meta.unlocks lists the hunt coupons you earned (with their codes); meta.hits the break hits pulled for spots you bought, with the order each ships with.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListGameResultMineResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_home_mine_builders

> crate::models::ListHomeMineBuildersResponse list_home_mine_builders()
Your co-builders: friends you let rearrange your home in the world (move, turn, take out your things, add free furniture; never your bought pieces). Each: userId, name, friends (false once the friendship ended: listed but powerless).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListHomeMineBuildersResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_home_of_the_week

> crate::models::ListHomeOfTheWeekResponse list_home_of_the_week()
Home of the week: open homes by how many different people signed their guestbook in the last 7 days, most first.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListHomeOfTheWeekResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_inventory

> crate::models::ListInventoryResponse list_inventory(page, limit, search, status, kind, platforms, platform_mode)
List inventory items.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |[default to 1]
**limit** | Option<**i32**> |  |  |[default to 25]
**search** | Option<**String**> |  |  |
**status** | Option<**String**> |  |  |
**kind** | Option<**String**> | Physical goods or Web3 assets (NFTs). Omit for both. |  |
**platforms** | Option<**String**> | Comma-separated marketplaces, e.g. \"ebay,vinted\". Items live (an active listing) on any of them. |  |
**platform_mode** | Option<**String**> | \"not_listed\" inverts `platforms`: items live on none of them. Default \"listed\". |  |

### Return type

**crate::models::ListInventoryResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_inventory_activity

> crate::models::ListInventoryActivityResponse list_inventory_activity(id, limit)
Activity log for an inventory item (created/sold/edited/etc.).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**limit** | Option<**i32**> |  |  |[default to 50]

### Return type

**crate::models::ListInventoryActivityResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_inventory_ids

> crate::models::ListListingIdsResponse list_inventory_ids()
Filter inventory → return matching id list.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListListingIdsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_inventory_units

> crate::models::ListInventoryUnitsResponse list_inventory_units(id)
List the individually identified units of an inventory item.

Each identifier carries a `strength` describing what it proves: `strong` was recorded before the item sold, `good` at packing, `weak` only after it shipped. The grade is derived from when it was recorded, never from the value itself.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListInventoryUnitsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_job_appraisals

> crate::models::ListJobAppraisalsResponse list_job_appraisals(category, limit)
Open price checks appraisers can answer (Collector level 3+ holding 3+ pieces in the category), newest first, optionally one market category. Each has its bounty; meta.payOn says whether bounties are paid yet.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**category** | Option<**String**> |  |  |
**limit** | Option<**i32**> |  |  |

### Return type

**crate::models::ListJobAppraisalsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_job_shifts

> crate::models::ListJobShiftsResponse list_job_shifts(limit)
Open shifts on the Crossly World Jobs board, soonest first: shop staff shifts in boutiques and grading-desk helper shifts, each with its window, place and pay. meta.payOn says whether Crossly is paying jobs yet.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> |  |  |

### Return type

**crate::models::ListJobShiftsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_main_stage_events

> crate::models::ListMainStageEventsResponse list_main_stage_events()
Live and upcoming Main Stage events, soonest first: brand launches, creator meetups, talks, live sales and festival shows, each with its window and host lineup.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListMainStageEventsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_spatial_auction_room_lots

> crate::models::ListSpatialAuctionRoomLotsResponse list_spatial_auction_room_lots(scene_id)
The catalogue: every lot in the session this room is hosting.

In lot-number order, which is the order the auctioneer reaches them. Each row carries the same fields as the current lot — `minimumNextBidCents` is the figure the bid transaction evaluates against, and every amount is CENTS. A lot with a `hammerAt` has been sold and one with `skipped: true` failed its reserve; neither takes further bids. The reserve AMOUNT is absent by design and `reserveMet` is the only thing said about it. Which lot is UP is not derivable from this list — the auctioneer is the authority and may skip ahead — so take that from the room state endpoint rather than from the lowest open lot number. A room hosting nothing answers an empty list, not a 404: 404 here means the room does not exist or you may not enter it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scene_id** | **String** |  | [required] |

### Return type

**crate::models::ListSpatialAuctionRoomLotsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_spatial_auction_rooms

> crate::models::ListSpatialAuctionRoomsResponse list_spatial_auction_rooms()
Rooms of yours that are hosting an auction.

One row per room you OWN that currently has a live-auction session bound into it, with that session's title, status and schedule joined in so you do not have to fetch each one to find out whether anything is running. Empty is the ordinary answer: hosting nothing is the normal state of every room on the platform. This is deliberately not \"rooms I could bid in\" — that is the public directory at /api/v1/spatial/public, and enumerating every stranger's saleroom here would be a different endpoint wearing this one's name. `topics.auction` is the WebSocket topic the price moves on; `topics.room` is null until the room has been shared.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListSpatialAuctionRoomsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_spatial_deed_listings

> crate::models::ListSpatialDeedListingsResponse list_spatial_deed_listings()
Plots for resale.

Live resale listings, cheapest first: address, `priceCents` (paid in CBX at spot), Crossly's `feeCents` (the 2% CBX skim), weekly maintenance, and the deed's `status` and `paidThrough` — a buyer inherits the maintenance clock. `?street=` narrows to one street.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListSpatialDeedListingsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_spatial_employee_sessions

> crate::models::ListSpatialEmployeeSessionsResponse list_spatial_employee_sessions(since, until, scene_id, unpaid_only, page, limit)
The itemised bill: one row per customer-conversation.

Newest first, filtered by `since`/`until` (ISO 8601, default 30 days, max 366) and optionally by `sceneId`. `unpaidOnly` narrows to the conversations that settled without payment. This is what makes a usage total checkable by hand: each row carries the interval it was priced from. `engagedMs` is SERVER-measured, from two timestamps this server wrote — no client reports its own duration anywhere in this feature — and it is null, not zero, while the conversation is still running. `settledAt` null with an `endedAt` set means billing has not yet run; `unpaidReason` set means it ran and moved nothing. Base units are STRINGS (64-bit integers). THE CUSTOMER IS NOT NAMED. Billing is per customer, but no row here carries a visitor identity — a bill is an account of what was spent, not a list of who walked in, and publishing the user ids of strangers through a seller API key is a different feature with a different consent question.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**since** | Option<**String**> |  |  |
**until** | Option<**String**> |  |  |
**scene_id** | Option<**String**> |  |  |
**unpaid_only** | Option<**bool**> |  |  |[default to false]
**page** | Option<**i32**> |  |  |[default to 1]
**limit** | Option<**i32**> |  |  |[default to 25]

### Return type

**crate::models::ListSpatialEmployeeSessionsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_spatial_employee_usage

> crate::models::ListSpatialEmployeeUsageResponse list_spatial_employee_usage(since, until, scene_id)
What your shop employees cost over a period, per room.

One row per room that had at least one conversation in the window; rooms with none are ABSENT rather than reported as zero. `since`/`until` are ISO 8601 and default to the last 30 days, and a window may span at most 366 days — longer is a 400, not a silently shortened period. Money: `billedCents` is COSM cents that were actually charged, and it is the figure to report. `burnBaseUnits` and `poolBaseUnits` are COSM base units as STRINGS because they are 64-bit integers — half the fee is destroyed and half moves to the employee-tax pool. `openConversations` are running right now: measured, not yet billed, and contributing 0 to every money field until they settle, so a total read mid-conversation is a floor rather than a final figure. `unpaidConversations` were settled WITHOUT payment — the seller was short of COSM or COSM is not live on this deployment — and each of those also suspended the employee in that room; they are not a debt and will never be retried. `meta.totals` is the same window summed in the database, not the sum of the rows. `meta.rate` restates the price so a figure can be checked by hand: billing is per STARTED minute, per CUSTOMER, only while somebody is engaged. An employee standing in an empty shop costs nothing.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**since** | Option<**String**> |  |  |
**until** | Option<**String**> |  |  |
**scene_id** | Option<**String**> |  |  |

### Return type

**crate::models::ListSpatialEmployeeUsageResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_spatial_moments

> crate::models::ListSpatialMomentsResponse list_spatial_moments(limit, ready)
Your moment emotes, newest first: unbox (a Crossly purchase delivered), slab_flip (a card graded through Crossly Grading), handshake (a trade-table shake settled). Each plays once in a room; playedAt is null until then. ready=true lists only the unplayed ones.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> |  |  |
**ready** | Option<**String**> |  |  |

### Return type

**crate::models::ListSpatialMomentsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_spatial_party_wishlist

> crate::models::CreateWorldPartyWishlistResponse list_spatial_party_wishlist()
Your party's shared wishlist, newest first: each listing (title, price in cents, image, shop slug while it is live), who added it, and every member's thumb up or down with the totals. Empty when you are in no party.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateWorldPartyWishlistResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_spatial_public

> crate::models::ListSpatialPublicResponse list_spatial_public()
Public rooms anyone can walk into.

The directory behind world-hopping. A room becomes public when its owner shares it; this lists those, newest first, with enough to draw a doorway AND enough to choose one — name, slug, category, itemCount, up to four previewImages, forSaleCount, a priceFromCents/priceToCents band and updatedAt. The band is the cheapest and dearest thing for sale in the room, never a quote for one object: /api/public/spatial/{slug}/offers is the authority on that. Fetch the room itself from /api/public/spatial/{slug}. Rooms with no items are omitted: an empty room is not a destination.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListSpatialPublicResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_spatial_public_offers

> crate::models::ListSpatialPublicOffersResponse list_spatial_public_offers(slug)
What is for sale in a shared room.

Price, stock, condition and grade for everything in the room its owner is actually selling. Correlate to the room by `itemId`, which is the SAME id the scene payload publishes per item — never by title. An item in the room with no row here is not for sale; an empty array means the owner is showing the collection rather than selling it, which is a different answer from a 404 (no such shared room). SEPARATE CALL ON PURPOSE: the room's geometry is stable for minutes, a price is not — it changes whenever the seller edits a listing. Re-read this before quoting, and do not cache a price alongside a cached room. `priceCents` is CENTS. `available` is remaining stock, or null when the listing declares none; null is unknown, not zero. Where an item sits inside more than one active listing, the offer quoted is the one for that item alone rather than a bundle it belongs to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::ListSpatialPublicOffersResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_spatial_scene_movements

> crate::models::ListSpatialSceneMovementsResponse list_spatial_scene_movements(id)
Stock movements in a room over a time window.

One row per physical transition: which item, from which node, to which node, when, and of what kind (placed/moved/picked/shipped/received/removed). Nodes are referenced by id; the `fromCode`/`toCode` strings are display snapshots of the location code AT THE TIME and are not stable identifiers — correlate on the node ids. `since`/`until` are ISO timestamps, defaulting to the last seven days and clamped to 90. Movements are NOT attributed to individual team members on this surface: a token has no team role, so there is no honest way to decide whether its holder may see who did the work.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListSpatialSceneMovementsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_spatial_scenes

> crate::models::ListSpatialScenesResponse list_spatial_scenes()
The rooms this account has.

The seller's ONE shop (kind `shop`, every display in one boutique) and their warehouse. Both are created on first read rather than requiring setup, so this call is safe to treat as the entry point. Older per-category rooms may also be listed as kind `collection`; they are no longer created or shown to visitors.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListSpatialScenesResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_spatial_shop_addresses

> crate::models::ListSpatialShopAddressesResponse list_spatial_shop_addresses()
Your shop's addresses.

One per street your shop stands on, Main Street first.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListSpatialShopAddressesResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_spatial_staffing

> crate::models::ListSpatialStaffingResponse list_spatial_staffing()
Shops you work in.

Each shop you hold a live staff grant on, with your wage.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListSpatialStaffingResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_spatial_street_mine

> crate::models::ListSpatialStreetMineResponse list_spatial_street_mine()
Your streets.

Streets you hold or asked to found, newest first: `status` (pending review, approved, founding, active, dormant, rejected, released), `upkeep` (active, grace, delinquent -- a delinquent owner collects no stall rent; three weeks late the street is foreclosed and closed), `paidThrough`, `admission`, `occupied` of `plotCount`, `pendingApplications`, and Crossly's `reviewNote`.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListSpatialStreetMineResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_spatial_streets

> crate::models::ListSpatialStreetsResponse list_spatial_streets()
Every street in the world.

Active streets with `plotCount` and how many plots are `occupied`. `platform: true` is Crossly's own street (Main Street), where every public shop has a plot.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListSpatialStreetsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_blocks

> crate::models::ListWorldBlocksResponse list_world_blocks()
The people you blocked in Crossly World, as you saw them (label, never the account behind a pseudonym), oldest first. Blocks are made in a room, on the person.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWorldBlocksResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_item_search

> crate::models::ListWorldItemSearchResponse list_world_item_search(q)
Find an item on a shelf anywhere in Crossly World: what public rooms have for sale now, by title or catalogue name (q, 2+ characters). Each hit names the room (roomSlug) and the item (itemId); /c/<roomSlug>?item=<itemId> opens it there.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | Option<**String**> |  |  |

### Return type

**crate::models::ListWorldItemSearchResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_room_wants

> crate::models::ListWorldRoomWantsResponse list_world_room_wants(scene_id)
Items your room shows but does not sell that visitors said they would buy (\"I'd buy this\"), most wanted first: itemId and how many people.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scene_id** | **String** |  | [required] |

### Return type

**crate::models::ListWorldRoomWantsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_inventory

> crate::models::CreateInventoryResponse update_inventory(id)
Update an inventory item (partial).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateInventoryResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_spatial_employee_config

> crate::models::UpdateSpatialEmployeeConfigResponse update_spatial_employee_config()
Create or replace the shop employee in one of your rooms.

A full replace, not a patch: every field is written from this body, so read GET /api/v1/spatial/employee/config first and send it back changed. Owner-only — a room this account does not own answers 404, the same as one that does not exist. `brainKind` is idle (stands there), rules (`rules` evaluated in order, first match wins, max 12) or agent (your endpoint decides, and an endpoint URL is then required). A rule is `{ when, afterSeconds?, do: { act, line?, offerId?, itemId? } }`; `when` and `do.act` must come from the published vocabularies and anything else is a 400. NOTHING IN A RULE CARRIES A PRICE — `do.offerId` is a key into your own offers and the server prices it, so an employee can propose a discount it cannot grant. `displayName` and `do.line` must be printable ASCII: they are drawn in the room from a fixed glyph atlas and anything else renders as a floating box, so they are refused rather than silently rewritten. `agentEndpointUrl` is resolved and validated before it is stored — a public-looking hostname that answers with a private address is refused here rather than accepted and then never called. THE SIGNING SECRET IS RETURNED AT MOST ONCE, in the response that MINTS it: when you pass `rotateSecret: true`, or when you set an agent endpoint on a room that had no secret. Store it then — there is no read path that will ever return it again, and `hasAgentSecret` is all any GET will say. Send an Idempotency-Key if you might retry a rotation: without one, a retried rotate mints a SECOND secret and invalidates the one you just received. Saving anything also CLEARS a suspension — if the employee was paused for want of COSM it is back on the floor, and will be paused again by the next refused charge. Note what this endpoint cannot do: it cannot start a conversation. Engagements are emitted by the room as it is drawn, and they are what costs money.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::UpdateSpatialEmployeeConfigResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_spatial_shop_staff_wage

> crate::models::UpdateSpatialShopStaffWageResponse update_spatial_shop_staff_wage(grant_id)
Set a staff member's weekly wage.

Body `{ wageWeeklyCents }` ($1-$2,000), or null to clear it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**grant_id** | **String** |  | [required] |

### Return type

**crate::models::UpdateSpatialShopStaffWageResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

