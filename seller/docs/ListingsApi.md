# \ListingsApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_ai_detection_dispute**](ListingsApi.md#create_ai_detection_dispute) | **POST** /v1/ai-detections/{id}/dispute | Dispute an AI-image detection on your photo (sends it to Crossly review).
[**create_event**](ListingsApi.md#create_event) | **POST** /v1/events | Create an event. A draft unless publish:true. Linked listings, the live show, the drop and the discount code must all be yours.
[**create_event_publish**](ListingsApi.md#create_event_publish) | **POST** /v1/events/{id}/publish | Publish an event: its page goes live, and with notifyFollowers your followers are told once.
[**create_listing**](ListingsApi.md#create_listing) | **POST** /v1/listings | Create a listing and fan out crosspost jobs across platforms.
[**create_listing_bulk**](ListingsApi.md#create_listing_bulk) | **POST** /v1/listings/bulk | Create up to 50 listings in one call, with a result per item.
[**create_listing_bulk_check_status**](ListingsApi.md#create_listing_bulk_check_status) | **POST** /v1/listings/bulk-check-status | Check listing status on platforms
[**create_listing_bulk_crosspost**](ListingsApi.md#create_listing_bulk_crosspost) | **POST** /v1/listings/bulk-crosspost | Bulk crosspost (no delist phase)
[**create_listing_bulk_delete**](ListingsApi.md#create_listing_bulk_delete) | **POST** /v1/listings/bulk-delete | Bulk archive + delist
[**create_listing_bulk_delist**](ListingsApi.md#create_listing_bulk_delist) | **POST** /v1/listings/bulk-delist | Bulk delist from platforms
[**create_listing_bulk_delist_preview**](ListingsApi.md#create_listing_bulk_delist_preview) | **POST** /v1/listings/bulk-delist-preview | Preview which marketplaces a delist would touch
[**create_listing_bulk_hard_delete**](ListingsApi.md#create_listing_bulk_hard_delete) | **POST** /v1/listings/bulk-hard-delete | Permanently delete listings (live ones need allowLive)
[**create_listing_bulk_relist**](ListingsApi.md#create_listing_bulk_relist) | **POST** /v1/listings/bulk-relist | Bulk relist across platforms
[**create_listing_bulk_update**](ListingsApi.md#create_listing_bulk_update) | **POST** /v1/listings/bulk-update | Bulk update listing fields
[**create_listing_by_id**](ListingsApi.md#create_listing_by_id) | **POST** /v1/listings/by-ids | Fetch hydrated listings by ID
[**create_listing_check_duplicate**](ListingsApi.md#create_listing_check_duplicate) | **POST** /v1/listings/check-duplicates | Check whether the seller already owns something matching this title/photo, and what to do about it.
[**create_listing_combine**](ListingsApi.md#create_listing_combine) | **POST** /v1/listings/combine | Combine duplicate listings into one: sums their stock, delists and archives the rest.
[**create_listing_discrepancy_resolve**](ListingsApi.md#create_listing_discrepancy_resolve) | **POST** /v1/listings/{id}/discrepancies/{discrepancyId}/resolve | Resolve a detected marketplace-drift discrepancy: accept the platform value, push ours back, relist to apply it, or dismiss.
[**create_listing_import_by_url**](ListingsApi.md#create_listing_import_by_url) | **POST** /v1/listings/{id}/import-by-url | Attach a real platform listing to this listing by pasting its live URL.
[**create_listing_magic_fill**](ListingsApi.md#create_listing_magic_fill) | **POST** /v1/listings/{id}/magic-fill | Auto-fill empty fields on one platform tab from the master listing + AI/deterministic taxonomy resolution.
[**create_listing_merge**](ListingsApi.md#create_listing_merge) | **POST** /v1/listings/merge | Merge the same item listed on different marketplaces into one listing; all stay live.
[**create_listing_tag_bulk**](ListingsApi.md#create_listing_tag_bulk) | **POST** /v1/listings/tags/bulk | Add and/or remove tags on many listings (yours only; max 30 tags a listing, 40 chars a tag). `dryRun` reports what would change.
[**create_listing_tag_rename**](ListingsApi.md#create_listing_tag_rename) | **POST** /v1/listings/tags/rename | Rename a tag on every listing; merges into `to` when it already exists.
[**create_live_stream**](ListingsApi.md#create_live_stream) | **POST** /v1/live-streams | Schedule a live show: title, start time, optional cover, category and the listings to feature (becomes the lineup, in order).
[**create_live_stream_connection**](ListingsApi.md#create_live_stream_connection) | **POST** /v1/live-streams/{id}/connection | The show’s video connection, minted on first call and identical after: RTMP(S) server + stream key, SRT URL, WHIP URL for browser publishing (null where the ingest has none) and the HLS playback URL. configured=false (with a reason) when the deployment has no streaming provider.
[**create_live_stream_connection_rotate**](ListingsApi.md#create_live_stream_connection_rotate) | **POST** /v1/live-streams/{id}/connection/rotate | Replace the show’s stream key (e.g. after it was shown on screen). Refused while the show is live.
[**create_live_stream_end**](ListingsApi.md#create_live_stream_end) | **POST** /v1/live-streams/{id}/end | End the show. Idempotent.
[**create_live_stream_pin_launch**](ListingsApi.md#create_live_stream_pin_launch) | **POST** /v1/live-streams/{id}/pin-launch | Pin one of your live NFT launches on your live show; viewers mint it from the stream. Returns `{ ok }`.
[**create_live_stream_start**](ListingsApi.md#create_live_stream_start) | **POST** /v1/live-streams/{id}/start | Go live: the show’s status becomes live and followers are notified once. Idempotent while live; refused once ended.
[**create_passport**](ListingsApi.md#create_passport) | **POST** /v1/passports | Register a product passport for a Crossly listing or an inventory item you sell, optionally bound to an NFC tag UID (hex). Active at once. Returns the URL to write to the NFC tag or print as a QR; ownership then follows the item’s sales. For several items with photos and labels, use the drafts flow (POST /passports/drafts).
[**create_passport_draft**](ListingsApi.md#create_passport_draft) | **POST** /v1/passports/drafts | Start a passport draft for up to 100 inventory items (or Crossly listings): one draft passport per item with its code reserved. Items whose every unit already has a passport come back in skipped.
[**create_passport_draft_activate**](ListingsApi.md#create_passport_draft_activate) | **POST** /v1/passports/drafts/{id}/activate | Activate every passport of a draft: their pages open for anyone who scans the tag or QR. Closes the draft’s capture link.
[**create_passport_draft_capture_link**](ListingsApi.md#create_passport_draft_capture_link) | **POST** /v1/passports/drafts/{id}/capture-link | A link (and its QR as SVG) any phone opens with no sign-in to add photos, serials and NFC tags to this draft. Open 30 minutes after its last use, 4 hours at most; minting a new one closes the old.
[**create_passport_print**](ListingsApi.md#create_passport_print) | **POST** /v1/passports/print | Print passport labels: sent to your printer agent (the Crossly desktop app includes one) when it is online and prints the template, else a signed PDF link with the reason. Templates: sticker-2in, sticker-1in, label-4x6, hang-tag, sheet-letter, sheet-a4, transfer-card.
[**create_passport_revoke**](ListingsApi.md#create_passport_revoke) | **POST** /v1/passports/{id}/revoke | Revoke a passport (a lost or misprinted tag). Refused once the item has sold — the passport is then part of its history.
[**create_passport_tag**](ListingsApi.md#create_passport_tag) | **POST** /v1/passports/{id}/tag | Bind (or re-bind) a passport to the NFC tag you wrote its nfcUrl to: the tag UID (hex), and whether you locked the tag read-only. Refused once the item has sold, for a locked tag, or a tag already on another item.
[**create_rental_bulk_rentable**](ListingsApi.md#create_rental_bulk_rentable) | **POST** /v1/rentals/bulk-rentable | Make up to 200 Crossly listings rentable with the same options (a day rate, or a share of each price).
[**create_shop_limit_request**](ListingsApi.md#create_shop_limit_request) | **POST** /v1/shop-limits/requests | Ask for a higher Crossly Shop selling limit.
[**create_story**](ListingsApi.md#create_story) | **POST** /v1/stories | Share a story: 1–10 slides played in order. Each slide is a photo or video plus optional overlays (positions are fractions of the 9:16 frame). Listing stickers must be your own Crossly listings and also tag the slide; @mentions must be real shops. audience: everyone | followers; expiresInHours: 6, 12, 24 or 48.
[**create_variation_group**](ListingsApi.md#create_variation_group) | **POST** /v1/variation-groups | Group listings you already have: a variation (one product, an axis such as Size) or a purchase-rule group (\"buy any 3\").
[**create_variation_group_generate**](ListingsApi.md#create_variation_group_generate) | **POST** /v1/variation-groups/generate | Turn one listing into a variation group: each new value becomes its own listing (a copy, with its own price, quantity and SKU) and all of them one group.
[**create_variation_group_generate_matrix**](ListingsApi.md#create_variation_group_generate_matrix) | **POST** /v1/variation-groups/generate-matrix | Turn one listing into a variation group over up to three axes (Size × Color): each other cell is a listing you already have (listingId) or a new copy of this one with its own price, quantity and SKU.
[**create_variation_group_publish**](ListingsApi.md#create_variation_group_publish) | **POST** /v1/variation-groups/{id}/publish | Publish a variation group per platform: one listing with a variation picker where the platform supports it (eBay, Shopify, Etsy), one per option elsewhere. Each platform reports its own outcome.
[**create_video_studio_finalize**](ListingsApi.md#create_video_studio_finalize) | **POST** /v1/video-studio/finalize | Check an uploaded listing video (exists, MP4, size) and, with listingId, set it as that listing's video.
[**create_video_studio_upload_url**](ListingsApi.md#create_video_studio_upload_url) | **POST** /v1/video-studio/upload-url | Presign an R2 PUT (video/mp4, 15 min) for a listing video; finalize it after the upload.
[**delete_event**](ListingsApi.md#delete_event) | **DELETE** /v1/events/{id} | Delete a draft, or cancel a published event (its page then says it was called off).
[**delete_listing**](ListingsApi.md#delete_listing) | **DELETE** /v1/listings/{id} | Delist a listing (optionally narrowed to specific platforms via ?platforms=).
[**delete_listing_tag**](ListingsApi.md#delete_listing_tag) | **DELETE** /v1/listings/tags/{tag} | Remove a tag (any case) from every listing.
[**delete_live_stream_pin_launch**](ListingsApi.md#delete_live_stream_pin_launch) | **DELETE** /v1/live-streams/{id}/pin-launch | Unpin the launch from your live show. Returns `{ ok }`.
[**delete_rental_offer**](ListingsApi.md#delete_rental_offer) | **DELETE** /v1/rentals/offers/{platformListingId} | Stop offering a listing for rent (existing bookings stand).
[**delete_story**](ListingsApi.md#delete_story) | **DELETE** /v1/stories/{id} | Take one story slide down before it expires.
[**delete_variation_group**](ListingsApi.md#delete_variation_group) | **DELETE** /v1/variation-groups/{id} | Dissolve a group. Its options stay as ordinary listings.
[**get_event**](ListingsApi.md#get_event) | **GET** /v1/events/{id} | One of your events.
[**get_event_analytic**](ListingsApi.md#get_event_analytic) | **GET** /v1/events/{id}/analytics | Views, RSVPs, and what the linked listings sold on Crossly during the event (orders, units, revenue in cents).
[**get_listing**](ListingsApi.md#get_listing) | **GET** /v1/listings/{id} | Get one listing with its platform rows.
[**get_listing_facet**](ListingsApi.md#get_listing_facet) | **GET** /v1/listings/facets | Distinct brands + categories across listings + inventory.
[**get_listing_sku_exist**](ListingsApi.md#get_listing_sku_exist) | **GET** /v1/listings/sku-exists | Check whether a SKU is already used by one of this user's items.
[**get_live_stream_recap**](ListingsApi.md#get_live_stream_recap) | **GET** /v1/live-streams/{id}/recap | The show’s numbers: duration, peak viewers, items sold and gross, tips, chat messages, auctions run, lineup size and the replay URL.
[**get_passport**](ListingsApi.md#get_passport) | **GET** /v1/passports/{id} | One passport: its item and every listing of it, the event history (created, printed, tag bound, transferred, claimed…), scans per day for 30 days, custody, grading.
[**get_passport_draft**](ListingsApi.md#get_passport_draft) | **GET** /v1/passports/drafts/{id} | A passport draft and its passports (photos, serial, tag and label state of each).
[**get_passport_print**](ListingsApi.md#get_passport_print) | **GET** /v1/passports/print/{jobId} | Where a passport print job sent to your agent is: queued, sent, done, failed, expired or duplicate.
[**get_pricing_setting**](ListingsApi.md#get_pricing_setting) | **GET** /v1/pricing/settings | Selling options: own-price match (Crossly sells your items at your own lower price for the same item on another marketplace; dollar-priced copies only) and layaway (buyers may put items on layaway). Both on by default.
[**get_pricing_sold_comp**](ListingsApi.md#get_pricing_sold_comp) | **GET** /v1/pricing/sold-comps | What one catalog product (market_sku_id, in one condition) actually sold for across Crossly in the last 180 days: median, p25/p75, p10/p90 (null under 10 sales), sale count, last sold, and days from listing to sale (median, p25, p75). Pass marketSkuId (+ optional condition) or one of your listingIds. comps is null until there are 3+ sales from 2+ sellers; holdCode says why there is nothing (no_market_sku, mixed_inventory, below_privacy_floor) — the same reasons a sold_median reprice rule holds.
[**get_rental_setting**](ListingsApi.md#get_rental_setting) | **GET** /v1/rentals/settings | The store’s rental defaults (every item inherits them) and its payout timing.
[**get_shop_limit**](ListingsApi.md#get_shop_limit) | **GET** /v1/shop-limits | Your Crossly Shop selling limit: live listings and listed value, used of limit, per store, and the next step.
[**list_event_attendees**](ListingsApi.md#list_event_attendees) | **GET** /v1/events/{id}/attendees | Who RSVPd: display name, avatar, going or interested, and when. No contact details.
[**list_events**](ListingsApi.md#list_events) | **GET** /v1/events | Your events (launches, sales, live shows, drops, meetups, pop-ups, workshops, parties): schedule, status, phase, and going/interested counts.
[**list_listing_ai_detections**](ListingsApi.md#list_listing_ai_detections) | **GET** /v1/listings/{id}/ai-detections | Crossly's AI-image detections for a listing's photos.
[**list_listing_discrepancies**](ListingsApi.md#list_listing_discrepancies) | **GET** /v1/listings/{id}/discrepancies | List detected marketplace-drift discrepancies for a listing.
[**list_listing_ids**](ListingsApi.md#list_listing_ids) | **GET** /v1/listings/ids | Filter listings → return matching id list (no pagination).
[**list_listing_tag_suggest**](ListingsApi.md#list_listing_tag_suggest) | **GET** /v1/listings/tags/suggest | Deterministic tag suggestions for one listing (brand, category, size, condition, colors, style tags, era words in the title); `known` = you already use the tag.
[**list_listing_tags**](ListingsApi.md#list_listing_tags) | **GET** /v1/listings/tags | Your listing tags with how many listings carry each, most used first (case-insensitive; `q` filters).
[**list_listings**](ListingsApi.md#list_listings) | **GET** /v1/listings | List active platform listings.
[**list_live_streams**](ListingsApi.md#list_live_streams) | **GET** /v1/live-streams | Your live shows, newest scheduled first: status, cover, category, connection method and peak viewers.
[**list_passport_events**](ListingsApi.md#list_passport_events) | **GET** /v1/passports/{id}/events | A passport’s event history, newest first (never a buyer or a price).
[**list_passport_print_targets**](ListingsApi.md#list_passport_print_targets) | **GET** /v1/passports/print-targets | Your printer agents (online, printers, default) and the passport label templates each can print.
[**list_passports**](ListingsApi.md#list_passports) | **GET** /v1/passports | Product passports you registered (optionally for one Crossly listing): the code and URL each tag carries.
[**list_rental_offers**](ListingsApi.md#list_rental_offers) | **GET** /v1/rentals/offers | What you offer for rent.
[**list_stories**](ListingsApi.md#list_stories) | **GET** /v1/stories | Your active story slides (newest run first, each run in play order), with their overlays: text, listing stickers, @mentions, links, emoji and drawing.
[**update_event**](ListingsApi.md#update_event) | **PATCH** /v1/events/{id} | Edit an event. Omitted fields are unchanged; null clears.
[**update_hotspot**](ListingsApi.md#update_hotspot) | **PUT** /v1/hotspots | Tag your listings at points on a story or lookbook photo (shop the look). Replaces that photo’s tags; at most 12.
[**update_listing**](ListingsApi.md#update_listing) | **PATCH** /v1/listings/{id} | Edit a listing and fan out update jobs to existing platform listings.
[**update_listing_custom**](ListingsApi.md#update_listing_custom) | **PUT** /v1/listings/{id}/customs | Set a listing’s customs: HS code (6–10 digits), country of origin, and countries this item must not ship to on Crossly. Omitted fields are left alone; null (or [] for restrictedCountries) clears.
[**update_listing_platform_listing**](ListingsApi.md#update_listing_platform_listing) | **PATCH** /v1/listings/{id}/platform-listings/{plId} | Set one marketplace copy's price (e.g. the ebay.com copy of a listing also live on ebay.co.uk), in that copy's own currency.
[**update_live_stream**](ListingsApi.md#update_live_stream) | **PATCH** /v1/live-streams/{id} | Edit a show that has not ended: title, description, cover, category, start time (not once live) or connection method (browser | phone | encoder).
[**update_live_stream_lineup**](ListingsApi.md#update_live_stream_lineup) | **PUT** /v1/live-streams/{id}/lineup | Set a live show’s lineup: your Crossly listings, in the order you will sell them. Buyers see it before they join; a sale of a lineup item while the show is live is recorded at its replay moment.
[**update_passport**](ListingsApi.md#update_passport) | **PATCH** /v1/passports/{id} | Change what a passport says (title, condition, maker, year, materials, notes) or add photo URLs (photos append: a passport keeps the ones it was registered with).
[**update_passport_draft**](ListingsApi.md#update_passport_draft) | **PATCH** /v1/passports/drafts/{id} | Move a draft to a step, pick its label template, add items or remove draft passports.
[**update_pricing_setting**](ListingsApi.md#update_pricing_setting) | **PUT** /v1/pricing/settings | Turn own-price match and/or layaway on or off for every one of your stores. One store: PUT /v1/storefronts/{id}/settings.
[**update_rental_offer**](ListingsApi.md#update_rental_offer) | **PUT** /v1/rentals/offers/{platformListingId} | Offer a Crossly listing for rent: day rate, optional week rate, deposit, min/max days, turnaround buffer, rent-only (off sale while offered), seller approval, unavailable date ranges (blackoutRanges, inclusive YYYY-MM-DD) and a return policy. Keeps the listing’s own rental terms in step. Needs a ship-from address.
[**update_rental_setting**](ListingsApi.md#update_rental_setting) | **PUT** /v1/rentals/settings | Set the store’s rental defaults and payout timing (on_start needs 3 rentals that came back).
[**update_variation_group**](ListingsApi.md#update_variation_group) | **PATCH** /v1/variation-groups/{id} | Retitle a group, rename its axis, or change a group's purchase rule (written into every member's description).



## create_ai_detection_dispute

> crate::models::CreateAiDetectionDisputeResponse create_ai_detection_dispute(id)
Dispute an AI-image detection on your photo (sends it to Crossly review).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateAiDetectionDisputeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_event

> crate::models::EventResponse create_event()
Create an event. A draft unless publish:true. Linked listings, the live show, the drop and the discount code must all be yours.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::EventResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_event_publish

> crate::models::EventResponse create_event_publish(id)
Publish an event: its page goes live, and with notifyFollowers your followers are told once.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::EventResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_listing

> crate::models::CreateListingResponse create_listing()
Create a listing and fan out crosspost jobs across platforms.

Listing fields go in `data`. Channel apps can also send `categoryId` (GET /v1/categories/search), `itemSpecifics` (validated against GET /v1/categories/:id/aspects; `warnings` lists soft problems), `condition`, `brand`, per-marketplace `overrides` and `policies` (preset ids from GET /v1/policies), folded in the way the web form does: an override replaces the master value on that marketplace, anything derived only fills blanks.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateListingResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_listing_bulk

> crate::models::CreateListingBulkResponse create_listing_bulk()
Create up to 50 listings in one call, with a result per item.

Each item is a POST /v1/listings body plus a required `clientReferenceId` (your id for it). Items run in order and independently: `data[i].status` is `created`, `existing` (that reference was already used — the listing is returned, nothing is created) or `error` with the code the single route would give. `error` means no listing exists for the item. A listing that was saved but could not be sent to a platform (e.g. no weight) is `created`, with that platform in `skipped` (reason, code, details). `meta` counts each. Retrying a batch only creates what is missing.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateListingBulkResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_listing_bulk_check_status

> crate::models::CreateListingBulkDeleteResponse create_listing_bulk_check_status()
Check listing status on platforms

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateListingBulkDeleteResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_listing_bulk_crosspost

> crate::models::CreateListingBulkDeleteResponse create_listing_bulk_crosspost()
Bulk crosspost (no delist phase)

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateListingBulkDeleteResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_listing_bulk_delete

> crate::models::CreateListingBulkDeleteResponse create_listing_bulk_delete()
Bulk archive + delist

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateListingBulkDeleteResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_listing_bulk_delist

> crate::models::CreateListingBulkDeleteResponse create_listing_bulk_delist()
Bulk delist from platforms

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateListingBulkDeleteResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_listing_bulk_delist_preview

> crate::models::CreateListingBulkDelistPreviewResponse create_listing_bulk_delist_preview()
Preview which marketplaces a delist would touch

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateListingBulkDelistPreviewResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_listing_bulk_hard_delete

> crate::models::CreateListingBulkHardDeleteResponse create_listing_bulk_hard_delete()
Permanently delete listings (live ones need allowLive)

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateListingBulkHardDeleteResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_listing_bulk_relist

> crate::models::CreateListingBulkDeleteResponse create_listing_bulk_relist()
Bulk relist across platforms

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateListingBulkDeleteResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_listing_bulk_update

> crate::models::CreateListingBulkUpdateResponse create_listing_bulk_update()
Bulk update listing fields

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


## create_listing_by_id

> crate::models::CreateListingByIdResponse create_listing_by_id()
Fetch hydrated listings by ID

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateListingByIdResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_listing_check_duplicate

> crate::models::CreateListingCheckDuplicateResponse create_listing_check_duplicate()
Check whether the seller already owns something matching this title/photo, and what to do about it.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateListingCheckDuplicateResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_listing_combine

> crate::models::CreateListingCombineResponse create_listing_combine()
Combine duplicate listings into one: sums their stock, delists and archives the rest.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateListingCombineResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_listing_discrepancy_resolve

> crate::models::CreateListingDiscrepancyResolveResponse create_listing_discrepancy_resolve(id, discrepancy_id)
Resolve a detected marketplace-drift discrepancy: accept the platform value, push ours back, relist to apply it, or dismiss.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**discrepancy_id** | **String** |  | [required] |

### Return type

**crate::models::CreateListingDiscrepancyResolveResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_listing_import_by_url

> crate::models::CreateListingImportByUrlResponse create_listing_import_by_url(id)
Attach a real platform listing to this listing by pasting its live URL.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateListingImportByUrlResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_listing_magic_fill

> crate::models::CreateListingMagicFillResponse create_listing_magic_fill(id)
Auto-fill empty fields on one platform tab from the master listing + AI/deterministic taxonomy resolution.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateListingMagicFillResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_listing_merge

> crate::models::CreateListingMergeResponse create_listing_merge()
Merge the same item listed on different marketplaces into one listing; all stay live.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateListingMergeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_listing_tag_bulk

> crate::models::CreateListingTagBulkResponse create_listing_tag_bulk()
Add and/or remove tags on many listings (yours only; max 30 tags a listing, 40 chars a tag). `dryRun` reports what would change.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateListingTagBulkResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_listing_tag_rename

> crate::models::CreateListingTagRenameResponse create_listing_tag_rename()
Rename a tag on every listing; merges into `to` when it already exists.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateListingTagRenameResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_live_stream

> crate::models::LiveStreamResponse create_live_stream()
Schedule a live show: title, start time, optional cover, category and the listings to feature (becomes the lineup, in order).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::LiveStreamResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_live_stream_connection

> crate::models::CreateLiveStreamConnectionResponse create_live_stream_connection(id)
The show’s video connection, minted on first call and identical after: RTMP(S) server + stream key, SRT URL, WHIP URL for browser publishing (null where the ingest has none) and the HLS playback URL. configured=false (with a reason) when the deployment has no streaming provider.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateLiveStreamConnectionResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_live_stream_connection_rotate

> crate::models::CreateLiveStreamConnectionResponse create_live_stream_connection_rotate(id)
Replace the show’s stream key (e.g. after it was shown on screen). Refused while the show is live.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateLiveStreamConnectionResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_live_stream_end

> crate::models::LiveStreamResponse create_live_stream_end(id)
End the show. Idempotent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::LiveStreamResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_live_stream_pin_launch

> crate::models::OkResponse create_live_stream_pin_launch(id)
Pin one of your live NFT launches on your live show; viewers mint it from the stream. Returns `{ ok }`.

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


## create_live_stream_start

> crate::models::LiveStreamResponse create_live_stream_start(id)
Go live: the show’s status becomes live and followers are notified once. Idempotent while live; refused once ended.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::LiveStreamResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_passport

> crate::models::PassportResponse create_passport()
Register a product passport for a Crossly listing or an inventory item you sell, optionally bound to an NFC tag UID (hex). Active at once. Returns the URL to write to the NFC tag or print as a QR; ownership then follows the item’s sales. For several items with photos and labels, use the drafts flow (POST /passports/drafts).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::PassportResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_passport_draft

> crate::models::PassportDraftResponse create_passport_draft()
Start a passport draft for up to 100 inventory items (or Crossly listings): one draft passport per item with its code reserved. Items whose every unit already has a passport come back in skipped.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::PassportDraftResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_passport_draft_activate

> crate::models::PassportDraftResponse create_passport_draft_activate(id)
Activate every passport of a draft: their pages open for anyone who scans the tag or QR. Closes the draft’s capture link.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::PassportDraftResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_passport_draft_capture_link

> crate::models::CreatePassportDraftCaptureLinkResponse create_passport_draft_capture_link(id)
A link (and its QR as SVG) any phone opens with no sign-in to add photos, serials and NFC tags to this draft. Open 30 minutes after its last use, 4 hours at most; minting a new one closes the old.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreatePassportDraftCaptureLinkResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_passport_print

> crate::models::CreatePassportPrintResponse create_passport_print()
Print passport labels: sent to your printer agent (the Crossly desktop app includes one) when it is online and prints the template, else a signed PDF link with the reason. Templates: sticker-2in, sticker-1in, label-4x6, hang-tag, sheet-letter, sheet-a4, transfer-card.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreatePassportPrintResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_passport_revoke

> crate::models::PassportResponse create_passport_revoke(id)
Revoke a passport (a lost or misprinted tag). Refused once the item has sold — the passport is then part of its history.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::PassportResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_passport_tag

> crate::models::PassportResponse create_passport_tag(id)
Bind (or re-bind) a passport to the NFC tag you wrote its nfcUrl to: the tag UID (hex), and whether you locked the tag read-only. Refused once the item has sold, for a locked tag, or a tag already on another item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::PassportResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_rental_bulk_rentable

> crate::models::CreateRentalBulkRentableResponse create_rental_bulk_rentable()
Make up to 200 Crossly listings rentable with the same options (a day rate, or a share of each price).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateRentalBulkRentableResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_shop_limit_request

> crate::models::CreateShopLimitRequestResponse create_shop_limit_request(create_shop_limit_request_request)
Ask for a higher Crossly Shop selling limit.

One pending request at a time (409 otherwise). Approved on the spot when your last 90 days are clean (no auto-cancelled orders, disputes lost or chargebacks lost) and every number asked is at most twice your current limit; otherwise Crossly staff review it. `valueCents` is US cents.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_shop_limit_request_request** | **CreateShopLimitRequestRequest** |  | [required] |

### Return type

**crate::models::CreateShopLimitRequestResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_story

> crate::models::CreateStoryResponse create_story()
Share a story: 1–10 slides played in order. Each slide is a photo or video plus optional overlays (positions are fractions of the 9:16 frame). Listing stickers must be your own Crossly listings and also tag the slide; @mentions must be real shops. audience: everyone | followers; expiresInHours: 6, 12, 24 or 48.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateStoryResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_variation_group

> crate::models::CreateVariationGroupResponse create_variation_group()
Group listings you already have: a variation (one product, an axis such as Size) or a purchase-rule group (\"buy any 3\").

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateVariationGroupResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_variation_group_generate

> crate::models::CreateVariationGroupGenerateResponse create_variation_group_generate()
Turn one listing into a variation group: each new value becomes its own listing (a copy, with its own price, quantity and SKU) and all of them one group.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateVariationGroupGenerateResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_variation_group_generate_matrix

> crate::models::CreateVariationGroupGenerateMatrixResponse create_variation_group_generate_matrix()
Turn one listing into a variation group over up to three axes (Size × Color): each other cell is a listing you already have (listingId) or a new copy of this one with its own price, quantity and SKU.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateVariationGroupGenerateMatrixResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_variation_group_publish

> crate::models::CreateVariationGroupPublishResponse create_variation_group_publish(id)
Publish a variation group per platform: one listing with a variation picker where the platform supports it (eBay, Shopify, Etsy), one per option elsewhere. Each platform reports its own outcome.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateVariationGroupPublishResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_video_studio_finalize

> crate::models::CreateVideoStudioFinalizeResponse create_video_studio_finalize()
Check an uploaded listing video (exists, MP4, size) and, with listingId, set it as that listing's video.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateVideoStudioFinalizeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_video_studio_upload_url

> crate::models::CreateVideoStudioUploadUrlResponse create_video_studio_upload_url()
Presign an R2 PUT (video/mp4, 15 min) for a listing video; finalize it after the upload.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateVideoStudioUploadUrlResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_event

> crate::models::DeleteEventResponse delete_event(id)
Delete a draft, or cancel a published event (its page then says it was called off).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::DeleteEventResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_listing

> crate::models::DeleteListingResponse delete_listing(id, platforms, platform_listing_ids)
Delist a listing (optionally narrowed to specific platforms via ?platforms=).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**platforms** | Option<**String**> | Comma-separated platform slugs to limit the delist fan-out. |  |
**platform_listing_ids** | Option<**String**> | Comma-separated platform listing ids — end only those copies (e.g. one eBay site's copy). |  |

### Return type

**crate::models::DeleteListingResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_listing_tag

> crate::models::UpdatedResponse delete_listing_tag(tag)
Remove a tag (any case) from every listing.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag** | **String** |  | [required] |

### Return type

**crate::models::UpdatedResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_live_stream_pin_launch

> crate::models::OkResponse delete_live_stream_pin_launch(id)
Unpin the launch from your live show. Returns `{ ok }`.

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


## delete_rental_offer

> crate::models::UpdateGroupBuyResponse delete_rental_offer(platform_listing_id)
Stop offering a listing for rent (existing bookings stand).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform_listing_id** | **String** |  | [required] |

### Return type

**crate::models::UpdateGroupBuyResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_story

> crate::models::DeleteStoryResponse delete_story(id)
Take one story slide down before it expires.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::DeleteStoryResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_variation_group

> crate::models::UpdateGroupBuyResponse delete_variation_group(id)
Dissolve a group. Its options stay as ordinary listings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::UpdateGroupBuyResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_event

> crate::models::EventResponse get_event(id)
One of your events.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::EventResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_event_analytic

> crate::models::GetEventAnalyticResponse get_event_analytic(id)
Views, RSVPs, and what the linked listings sold on Crossly during the event (orders, units, revenue in cents).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetEventAnalyticResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_listing

> crate::models::GetListingResponse get_listing(id)
Get one listing with its platform rows.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetListingResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_listing_facet

> crate::models::GetListingFacetResponse get_listing_facet()
Distinct brands + categories across listings + inventory.

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


## get_listing_sku_exist

> crate::models::GetListingSkuExistResponse get_listing_sku_exist(sku)
Check whether a SKU is already used by one of this user's items.

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


## get_live_stream_recap

> crate::models::GetLiveStreamRecapResponse get_live_stream_recap(id)
The show’s numbers: duration, peak viewers, items sold and gross, tips, chat messages, auctions run, lineup size and the replay URL.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetLiveStreamRecapResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_passport

> crate::models::GetPassportResponse get_passport(id)
One passport: its item and every listing of it, the event history (created, printed, tag bound, transferred, claimed…), scans per day for 30 days, custody, grading.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetPassportResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_passport_draft

> crate::models::PassportDraftResponse get_passport_draft(id)
A passport draft and its passports (photos, serial, tag and label state of each).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::PassportDraftResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_passport_print

> crate::models::GetPassportPrintResponse get_passport_print(job_id)
Where a passport print job sent to your agent is: queued, sent, done, failed, expired or duplicate.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** |  | [required] |

### Return type

**crate::models::GetPassportPrintResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_pricing_setting

> crate::models::GetPricingSettingResponse get_pricing_setting()
Selling options: own-price match (Crossly sells your items at your own lower price for the same item on another marketplace; dollar-priced copies only) and layaway (buyers may put items on layaway). Both on by default.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetPricingSettingResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_pricing_sold_comp

> crate::models::GetPricingSoldCompResponse get_pricing_sold_comp()
What one catalog product (market_sku_id, in one condition) actually sold for across Crossly in the last 180 days: median, p25/p75, p10/p90 (null under 10 sales), sale count, last sold, and days from listing to sale (median, p25, p75). Pass marketSkuId (+ optional condition) or one of your listingIds. comps is null until there are 3+ sales from 2+ sellers; holdCode says why there is nothing (no_market_sku, mixed_inventory, below_privacy_floor) — the same reasons a sold_median reprice rule holds.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetPricingSoldCompResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_rental_setting

> crate::models::GetRentalSettingResponse get_rental_setting()
The store’s rental defaults (every item inherits them) and its payout timing.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetRentalSettingResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_shop_limit

> crate::models::GetShopLimitResponse get_shop_limit()
Your Crossly Shop selling limit: live listings and listed value, used of limit, per store, and the next step.

Per seller account, shared by all your stores. Counts live + scheduled Crossly Shop listings once each; value is the US-dollar shop price x quantity (an auction at max(start, reserve, buy-now)). Drafts, sold/ended listings and Trade Block posts are not counted, and selling frees space at once. `unlimited` is true for sandbox sellers.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetShopLimitResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_event_attendees

> crate::models::ListEventAttendeesResponse list_event_attendees(id)
Who RSVPd: display name, avatar, going or interested, and when. No contact details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListEventAttendeesResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_events

> crate::models::ListEventsResponse list_events()
Your events (launches, sales, live shows, drops, meetups, pop-ups, workshops, parties): schedule, status, phase, and going/interested counts.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListEventsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_listing_ai_detections

> crate::models::ListListingAiDetectionsResponse list_listing_ai_detections(id)
Crossly's AI-image detections for a listing's photos.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListListingAiDetectionsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_listing_discrepancies

> crate::models::ListListingDiscrepanciesResponse list_listing_discrepancies(id)
List detected marketplace-drift discrepancies for a listing.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListListingDiscrepanciesResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_listing_ids

> crate::models::ListListingIdsResponse list_listing_ids()
Filter listings → return matching id list (no pagination).

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


## list_listing_tag_suggest

> crate::models::ListListingTagSuggestResponse list_listing_tag_suggest(listing_id)
Deterministic tag suggestions for one listing (brand, category, size, condition, colors, style tags, era words in the title); `known` = you already use the tag.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**listing_id** | **String** |  | [required] |

### Return type

**crate::models::ListListingTagSuggestResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_listing_tags

> crate::models::ListListingTagsResponse list_listing_tags(q, limit)
Your listing tags with how many listings carry each, most used first (case-insensitive; `q` filters).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | Option<**String**> |  |  |
**limit** | Option<**i32**> |  |  |

### Return type

**crate::models::ListListingTagsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_listings

> crate::models::ListListingsResponse list_listings(page, limit, platform, status, sku, updated_since, kind, tag, still_live)
List active platform listings.

Each row carries `pricing`: `avgAsking` (average price across its active marketplace rows, per-platform overrides and price rules applied) and `avgSale` (average price per unit sold, refunds netted), each `{ amount, currency, count }` or null. Never averaged across currencies.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |[default to 1]
**limit** | Option<**i32**> |  |  |[default to 25]
**platform** | Option<**String**> |  |  |
**status** | Option<**String**> |  |  |
**sku** | Option<**String**> | Exact SKU match: look up the row a supplier product maps to. |  |
**updated_since** | Option<**String**> | Only rows created or changed at or after this time (ISO 8601), oldest change first. Poll with the updatedAt of the last row you saw. |  |
**kind** | Option<**String**> | Physical goods or Web3 assets (NFT listings). Omit for both. |  |
**tag** | Option<[**crate::models::AnyOfstringarray**](.md)> | Only listings carrying ANY of these tags (case-insensitive): repeat `tag` or comma-separate. |  |
**still_live** | Option<**bool**> | Only listings still active on at least one marketplace. With status=sold: sold items still buyable somewhere (a delist failed, is pending or never ran). |  |

### Return type

**crate::models::ListListingsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_live_streams

> crate::models::ListLiveStreamsResponse list_live_streams()
Your live shows, newest scheduled first: status, cover, category, connection method and peak viewers.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListLiveStreamsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_passport_events

> crate::models::ListPassportEventsResponse list_passport_events(id)
A passport’s event history, newest first (never a buyer or a price).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListPassportEventsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_passport_print_targets

> crate::models::ListPassportPrintTargetsResponse list_passport_print_targets()
Your printer agents (online, printers, default) and the passport label templates each can print.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListPassportPrintTargetsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_passports

> crate::models::ListPassportsResponse list_passports()
Product passports you registered (optionally for one Crossly listing): the code and URL each tag carries.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListPassportsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_rental_offers

> crate::models::ListRentalOffersResponse list_rental_offers()
What you offer for rent.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListRentalOffersResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_stories

> crate::models::ListStoriesResponse list_stories()
Your active story slides (newest run first, each run in play order), with their overlays: text, listing stickers, @mentions, links, emoji and drawing.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListStoriesResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_event

> crate::models::EventResponse update_event(id)
Edit an event. Omitted fields are unchanged; null clears.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::EventResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_hotspot

> crate::models::UpdateHotspotResponse update_hotspot()
Tag your listings at points on a story or lookbook photo (shop the look). Replaces that photo’s tags; at most 12.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::UpdateHotspotResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_listing

> crate::models::UpdateListingResponse update_listing(id)
Edit a listing and fan out update jobs to existing platform listings.

The fields to change go in `changes` (a sparse object, e.g. `{ \"changes\": { \"price\": 24.99 } }`). `data` is accepted as an alias of `changes` (the name create and bulk create use); sending both is a 400. Omit `platforms` to push the edit everywhere the listing is live.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::UpdateListingResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_listing_custom

> crate::models::UpdateListingCustomResponse update_listing_custom(id)
Set a listing’s customs: HS code (6–10 digits), country of origin, and countries this item must not ship to on Crossly. Omitted fields are left alone; null (or [] for restrictedCountries) clears.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::UpdateListingCustomResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_listing_platform_listing

> crate::models::UpdateListingPlatformListingResponse update_listing_platform_listing(id, pl_id)
Set one marketplace copy's price (e.g. the ebay.com copy of a listing also live on ebay.co.uk), in that copy's own currency.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**pl_id** | **String** |  | [required] |

### Return type

**crate::models::UpdateListingPlatformListingResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_live_stream

> crate::models::LiveStreamResponse update_live_stream(id)
Edit a show that has not ended: title, description, cover, category, start time (not once live) or connection method (browser | phone | encoder).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::LiveStreamResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_live_stream_lineup

> crate::models::UpdateLiveStreamLineupResponse update_live_stream_lineup(id)
Set a live show’s lineup: your Crossly listings, in the order you will sell them. Buyers see it before they join; a sale of a lineup item while the show is live is recorded at its replay moment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::UpdateLiveStreamLineupResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_passport

> crate::models::UpdatePassportResponse update_passport(id)
Change what a passport says (title, condition, maker, year, materials, notes) or add photo URLs (photos append: a passport keeps the ones it was registered with).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::UpdatePassportResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_passport_draft

> crate::models::PassportDraftResponse update_passport_draft(id)
Move a draft to a step, pick its label template, add items or remove draft passports.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::PassportDraftResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_pricing_setting

> crate::models::GetPricingSettingResponse update_pricing_setting()
Turn own-price match and/or layaway on or off for every one of your stores. One store: PUT /v1/storefronts/{id}/settings.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetPricingSettingResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_rental_offer

> crate::models::UpdateRentalOfferResponse update_rental_offer(platform_listing_id)
Offer a Crossly listing for rent: day rate, optional week rate, deposit, min/max days, turnaround buffer, rent-only (off sale while offered), seller approval, unavailable date ranges (blackoutRanges, inclusive YYYY-MM-DD) and a return policy. Keeps the listing’s own rental terms in step. Needs a ship-from address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform_listing_id** | **String** |  | [required] |

### Return type

**crate::models::UpdateRentalOfferResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_rental_setting

> crate::models::GetRentalSettingResponse update_rental_setting()
Set the store’s rental defaults and payout timing (on_start needs 3 rentals that came back).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetRentalSettingResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_variation_group

> crate::models::CreateVariationGroupResponse update_variation_group(id)
Retitle a group, rename its axis, or change a group's purchase rule (written into every member's description).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateVariationGroupResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

