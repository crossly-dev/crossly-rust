# CreateListingImportByUrlResponseListing

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**platform_listing_id** | **String** |  | 
**title** | **String** |  | 
**description** | Option<**String**> |  | [optional]
**price** | Option<**f32**> |  | [optional]
**images** | Option<**Vec<String>**> |  | [optional]
**brand** | Option<**String**> |  | [optional]
**condition** | Option<**String**> |  | [optional]
**size** | Option<**String**> |  | [optional]
**color** | Option<**Vec<String>**> | Color(s) the platform's own response exposes (e.g. Poshmark's  `colors` array). Only set when actually present in the scrape. | [optional]
**sku** | Option<**String**> |  | [optional]
**url** | Option<**String**> |  | [optional]
**category** | Option<**Vec<String>**> | The platform's OWN category, as a top-down path (e.g. Poshmark's  [\"Kids\", \"Toys\", \"Dolls & Accessories\"] from department → category →  feature). Only set when the platform's list/scrape response actually  carries this — never guessed. Mapped onto Crossly's category.main/  sub/sub2 (listings) or categoryMain/categorySub (inventory_items) at  create/enrich time. | [optional]
**category_id** | Option<**String**> | The platform's raw numeric category id, same space as  overrides.<platform>.categoryId — distinct from `category`'s  human-readable path. Only set by platforms whose id space is directly  comparable to what we publish (currently eBay's drift-check second  call — see diff-fields/second-call.ts's ebayRemoteFields). eBay's own  CategoryName wording/depth is a different vocabulary from Crossly's  master taxonomy breadcrumb and will essentially never string-match  it, so drift-detection compares ids instead of names. | [optional]
**tags** | Option<**Vec<String>**> | Tag-like strings the platform's own response exposes (e.g. Poshmark's  marketing \"experience\" tags). Distinct from a full search-tag  generator — just whatever real tag data the scrape already carries. | [optional]
**material** | Option<**String**> | Structured item aspects — the same shape the listing form's Item  Details section captures (and templates already persist), now sourced  from the platform's own data instead of only manual entry. Only set  when the platform's response genuinely carries a semantically-matching  field (eBay item specifics, Facebook attributes, Poshmark catalog,  Depop's detail-call ride-along) — never derived or guessed. | [optional]
**style** | Option<**String**> |  | [optional]
**pattern** | Option<**String**> |  | [optional]
**department** | Option<**String**> |  | [optional]
**gender** | Option<**String**> |  | [optional]
**item_type** | Option<**String**> |  | [optional]
**size_system** | Option<**String**> |  | [optional]
**item_specifics** | Option<[**serde_json::Value**](.md)> | Category-specific facets from a per-item DETAIL call (e.g. Facebook's  Age Range/Character/Age Group), keyed the same way eBay item-specifics  are: aspect name -> value array. Only set by platforms with a genuine  per-item attribute source — costs one extra call per listing, so  populated by a dedicated enrichment pass, not the main list mapper.  Maps onto listings.itemSpecifics; inventory_items has no equivalent  column. | [optional]
**listed_at** | Option<**String**> | When the listing was first published on the platform. Used by  the Advanced filter's listedAfter / listedBefore knobs. Optional  because not every platform returns it on the listing endpoint. | [optional]
**quantity** | Option<**f32**> | Units this platform reports. Only meaningful for platforms  `PLATFORM_QUANTITY_SYNC` marks 'native' — relist platforms show one item  and say 1 forever, so they leave this undefined rather than voting with  a number they cannot actually express. See `_quantity.ts`. | [optional]
**account_slot** | Option<**f32**> | Which of the user's connected accounts on this platform this listing  was scraped from — stamped by fetchCookieListings as it loops each  connected account (see listActiveForPlatform in  user-platform-accounts/read.ts). Undefined for API-track platforms  (single connection, no multi-account concept) and for any cookie path  that hasn't been threaded yet; importOne treats undefined as slot 1,  matching the historical single-account default. | [optional]
**weight_oz** | Option<**f32**> | Total item weight in ounces (already summed, not split lb+oz —  the write path converts to the DB's weightLb+weightOz split). | [optional]
**dimension_lin** | Option<**f32**> |  | [optional]
**dimension_win** | Option<**f32**> |  | [optional]
**dimension_hin** | Option<**f32**> |  | [optional]
**handling_time_days** | Option<**f32**> | Max days the platform's own listing commits to ship within (eBay's  DispatchTimeMax, Etsy's processing_max). | [optional]
**best_offer_auto_accept_cents** | Option<**f32**> |  | [optional]
**best_offer_auto_decline_cents** | Option<**f32**> |  | [optional]
**item_location** | Option<**String**> | Free text as the platform itself expresses it (e.g. eBay's `Location`  is a single seller-typed string like \"Austin, TX\", not a structured  address) — never parsed into city/state. | [optional]
**item_location_zip** | Option<**String**> |  | [optional]
**item_location_country** | Option<**String**> |  | [optional]
**declared_shipping_cost** | Option<**f32**> | What the platform's OWN listing declares shipping costs — reference  only, distinct from a realized post-sale shipping cost. | [optional]
**return_policy_text** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


