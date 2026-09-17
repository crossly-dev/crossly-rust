# CreateMagicScanSynthesizeResponsePayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**title** | **String** |  | 
**description** | **String** |  | 
**brand** | Option<**String**> |  | [optional]
**grading** | Option<[**crate::models::CreateMagicScanSynthesizeResponsePayloadGrading**](CreateMagicScanSynthesizeResponse_payload_grading.md)> |  | [optional]
**color** | Option<**String**> |  | [optional]
**material** | Option<**String**> |  | [optional]
**size** | Option<**String**> |  | [optional]
**size_system** | Option<**String**> | Poshmark/Vestiaire size system — \"US\", \"EU\", \"UK\", etc. | [optional]
**weight_oz** | Option<**f32**> |  | [optional]
**dimensions** | Option<[**crate::models::CreateMagicScanSynthesizeResponsePayloadDimensions**](CreateMagicScanSynthesizeResponse_payload_dimensions.md)> |  | [optional]
**price_cents** | Option<**f32**> | Echoed user-selected price (cents). Set on every synthesize call  so the master form's defaultPrice input gets populated regardless  of which path (AI or heuristic) generated the rest. | [optional]
**seller_note** | Option<**String**> | The seller's own words about this item — the pre-scan hint, or the  post-scan note that supersedes it (see synthesize/seller-note.ts).  Carried on the payload so the TAXONOMY resolvers see it too: category  and facet picking happen in their own LLM calls, which never saw the  note even after the main synthesis prompt did. A seller correcting the  variant was still getting the category of the wrong one. | [optional]
**price_low_cents** | Option<**f32**> | Comp price band (cents), outlier-trimmed — low/median/high across the  matched comps. The UI surfaces the range so the seller prices  strategically instead of trusting one number; priceCents defaults to  the median when the seller hasn't picked. | [optional]
**price_median_cents** | Option<**f32**> |  | [optional]
**price_high_cents** | Option<**f32**> |  | [optional]
**quantity** | Option<**f32**> | Echoed seller-supplied listable-unit count from Magic List. This is  the listing's ADVERTISED stock (how many of this listing to sell).  Hydrates the form's `quantity` field. Undefined = leave form default (1). | [optional]
**inventory_quantity** | Option<**f32**> | Physical units the seller actually has in stock — becomes the auto-  created inventory item's quantity/quantityAvailable. Distinct from  `quantity` (advertised stock) and `unitsPerListing` (composition).  Undefined = fall back to `quantity`. | [optional]
**units_per_listing** | Option<**f32**> | How many physical units are bundled inside ONE listing (composition;  e.g. a pack of 4). Becomes listing_inventory_items.quantity so the  multi-channel fulfillable = floor(inventoryQuantity / unitsPerListing).  Undefined = 1. | [optional]
**cost_basis_cents** | Option<**f32**> | What the seller paid, per physical unit, in cents — the auto-created  inventory item's `costBasisCents`. Nothing wrote that column from any  create path, so every Magic List item had a NULL cost basis and every  sale of one reported its full sale price as profit. Undefined = unknown,  which stays NULL (0 would be a claim that the item was free). | [optional]
**chosen_dimensions** | Option<[**crate::models::CreateMagicScanSynthesizeResponsePayloadChosenDimensions**](CreateMagicScanSynthesizeResponse_payload_chosenDimensions.md)> |  | [optional]
**image_urls** | Option<**Vec<String>**> | Every photo the seller uploaded for the source scan, primary  first. The NewListingPage hydrator drops these straight into the  form's images state so the seller doesn't have to re-upload. | [optional]
**category** | Option<[**crate::models::CreateMagicScanSynthesizeResponsePayloadCategory**](CreateMagicScanSynthesizeResponse_payload_category.md)> |  | [optional]
**department** | Option<**String**> | eBay item-specifics: Department/Gender/Style/Pattern/Type. These  drive the eBay-required aspects on the form's Item Details  section. Mostly only eBay matches have them. | [optional]
**gender** | Option<**String**> |  | [optional]
**style** | Option<**String**> |  | [optional]
**pattern** | Option<**String**> |  | [optional]
**item_type** | Option<**String**> |  | [optional]
**upc** | Option<**String**> | Universal product code, when eBay's enriched detail surfaced it.  Hydrates the master form's UPC/GTIN field. | [optional]
**handling_time_days** | Option<**String**> |  | [optional]
**return_window_days** | Option<**String**> |  | [optional]
**tags** | Option<**Vec<String>**> |  | [optional]
**item_specifics** | Option<**String**> | Master-level item specifics (JSON Record<string,string[]>). Item  specifics are now master-owned + fanned to every aspect platform, so the  form's shared Item-specifics editor prefills from this. Mirrored from the  richest resolved per-platform specifics blob (eBay after full-aspect  enrichment). Undefined = no specifics. | [optional]
**per_platform_overrides** | [**serde_json::Value**](.md) | Per-platform required-fields map — slotted directly into  platformOverrides when the form hydrates. | 
**confidence** | **f32** | AI's confidence in the synthesis (0-1). Surfaced to the seller  so they can decide whether to skim or trust + click. | 
**notes** | Option<**String**> | AI's free-text rationale for the merge — what it pulled from  where. Helps the seller spot a bad merge before listing. | [optional]
**section_applicability** | Option<[**crate::models::CreateMagicScanSynthesizeResponsePayloadSectionApplicability**](CreateMagicScanSynthesizeResponse_payload_sectionApplicability.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


