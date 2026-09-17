# CreateMagicScanResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**run_id** | **String** |  | 
**ebay_match** | Option<[**crate::models::CreateMagicScanResponseEbayMatch**](CreateMagicScanResponse_ebayMatch.md)> |  | [optional]
**top_hits** | [**Vec<crate::models::CreateMagicScanResponseTopHits>**](CreateMagicScanResponse_topHits.md) | Unified top-10-globally list, ranked by CLIP visual similarity to  the seller's source photo. Each hit carries its origin platform. | 
**ebay_hits** | [**Vec<crate::models::CreateMagicScanResponseEbayHits>**](CreateMagicScanResponse_ebayHits.md) | Legacy compat — UI's existing render. ebayHits now == visually-  validated eBay subset; otherMatches is re-grouped from topHits. | 
**other_matches** | [**serde_json::Value**](.md) |  | 
**image_urls** | **Vec<String>** | Every photo the seller uploaded for this scan, primary first. | 
**vision_aspects** | [**serde_json::Value**](.md) | Vision-LLM aspects extracted across all photos. Populated only  when the seller has magic-list-vision-aspects enabled + a vision  provider configured. Empty otherwise. | 
**possible_duplicates** | [**Vec<crate::models::CreateMagicScanResponsePossibleDuplicates>**](CreateMagicScanResponse_possibleDuplicates.md) | The seller's OWN listings/inventory that this scan probably duplicates  (image + fuzzy-title self-dedup). Empty when nothing matched. Drives the  \"you may already have this\" prompt. | 
**cached** | **bool** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


