# CreateListingByIdResponseItems

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**effective_title** | **String** |  | 
**effective_price** | Option<**String**> |  | [optional]
**effective_description** | Option<**String**> |  | [optional]
**effective_images** | **Vec<String>** |  | 
**effective_brand** | Option<**String**> |  | [optional]
**effective_condition** | Option<**String**> |  | [optional]
**effective_size** | Option<**String**> |  | [optional]
**effective_sku** | Option<**String**> |  | [optional]
**effective_color** | **Vec<String>** |  | 
**effective_tags** | **Vec<String>** |  | 
**platform_listings** | [**Vec<serde_json::Value>**](serde_json::Value.md) |  | 
**inventory_item_id** | Option<**String**> |  | [optional]
**id** | **String** |  | 
**user_id** | **String** |  | 
**name** | Option<**String**> |  | [optional]
**title** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**description_html** | Option<**String**> |  | [optional]
**price** | Option<**String**> |  | [optional]
**images** | Option<**Vec<String>**> |  | [optional]
**video_url** | Option<**String**> | Optional single product video (R2/CDN URL). Shown on the Crossly buyer page. | [optional]
**status** | **String** |  | 
**condition** | Option<**String**> |  | [optional]
**grade_key** | Option<**String**> | Third-party grading, when the item is slabbed. Migration 0277.    Separate from `condition` on purpose and never derived from it: a grade  is a claim about what a GRADING COMPANY certified, and inferring \"PSA 10\"  from a coarse condition would be a false authenticity claim. It is also  never filled from our own AI estimate (`bulk_market_items.grade`), which  carries an explicit \"not a professional grade\" disclaimer.    `gradeKey` is the canonical form from `gradeKey()` in  shared/constants/graders.ts; `grading` holds the full GradingInfo  including the cert number and whether a cert lookup verified it. | [optional]
**grading** | Option<[**crate::models::ListListingsItemGrading**](ListListingsItem_grading.md)> |  | [optional]
**brand** | Option<**String**> |  | [optional]
**size** | Option<**String**> |  | [optional]
**material** | Option<**String**> | Migration 0179 — see the matching fields on inventory_items above. | [optional]
**style** | Option<**String**> |  | [optional]
**pattern** | Option<**String**> |  | [optional]
**department** | Option<**String**> |  | [optional]
**gender** | Option<**String**> |  | [optional]
**item_type** | Option<**String**> |  | [optional]
**size_system** | Option<**String**> |  | [optional]
**color** | **Vec<String>** |  | 
**tags** | **Vec<String>** |  | 
**sku** | Option<**String**> |  | [optional]
**quantity** | **f32** |  | 
**quantity_available** | **f32** |  | 
**weight_lb** | Option<**String**> |  | [optional]
**weight_oz** | Option<**String**> |  | [optional]
**dimension_lin** | Option<**String**> |  | [optional]
**dimension_win** | Option<**String**> |  | [optional]
**dimension_hin** | Option<**String**> |  | [optional]
**publish_at** | Option<**String**> | Scheduled go-live time. When set on a draft, the listing-scheduler  worker waits until this passes then dispatches the crosspost to  scheduledPlatforms and flips status from 'draft' to 'active'. | [optional]
**scheduled_platforms** | Option<**Vec<String>**> | Which platforms to publish to when publishAt fires. JSON array of  platform ids. Null/empty = scheduler skips (listing won't auto-  publish, even after publishAt — gives the seller an escape hatch). | [optional]
**parent_listing_id** | Option<**String**> | Parent listing when this row is a CHILD in a listing chain. Null =  standalone. What being a child means depends on the parent's  `groupKind` — see it. | [optional]
**is_bundle** | **bool** |  | 
**automation_assigned_rule_ids** | **Vec<String>** | Per-listing automation overrides. See migration 0098.     automationAssignedRuleIds  — force-include for these rules   automationBlockedRuleIds   — exempt from these rules   automationAssignedChainIds — force-include for these workflow chains   automationBlockedChainIds  — exempt from these workflow chains | 
**automation_blocked_rule_ids** | **Vec<String>** |  | 
**automation_assigned_chain_ids** | **Vec<String>** |  | 
**automation_blocked_chain_ids** | **Vec<String>** |  | 
**hs_code** | Option<**String**> | Harmonised System customs code — international shipping declarations. | [optional]
**country_of_origin** | Option<**String**> | Customs country of origin. Distinct from the seller's location. | [optional]
**price_floor_cents** | Option<**f32**> | Never let a repricing rule go below this. On the ITEM because it is a  fact about the thing owned, not about any one rule — \"this jacket never  goes below $45\" should apply to every rule, and before this it was  expressible only as one rule per jacket. Listings inherit when null. | [optional]
**floor_is_net** | **bool** | When true the floor is a TAKE-HOME target, converted to a per-platform  gross at reprice time. A gross floor is four different promises across  four platforms; this is the one number a seller actually cares about. | 
**delisted_at** | Option<**String**> |  | [optional]
**sold_at** | Option<**String**> |  | [optional]
**source** | **String** | Mirrors inventory_items.source. 'manual' for every seller-created  listing; external-stub.ts sets 'external_sale' on the synthetic  listing it fabricates for a sale detected on a platform id Crossly  never listed — those rows have no real photos/description of their  own (everything is lifted from the platform's sale payload) and are  otherwise indistinguishable from a real listing in the UI. | 
**duplicate_of_listing_id** | Option<**String**> | Set when import's bin-packing (see _import-one.ts) created THIS  listing to hold a same-platform straggler it couldn't fit onto an  existing candidate listing for the same physical item — points at  the primary/first candidate. Purely informational: this listing  is a real, independently listable/delistable row, not a shadow.  Null for every ordinarily-created listing. | [optional]
**client_draft_id** | Option<**String**> | UUID minted on a seller's machine for a draft written offline.    The idempotency key for desktop sync. The failure it guards is a POST  that succeeds server-side whose reply is lost — the client cannot tell  that from a failure, retries, and one item becomes two live listings  against one piece of stock. Unique per user (partial index, migration  0268); null for every listing that did not come from an offline draft. | [optional]
**created_at** | **String** |  | 
**updated_at** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


