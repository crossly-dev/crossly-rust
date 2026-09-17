# CreateListingCheckDuplicateResponseMatches

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**suggested** | **String** | What we'd offer to do about this match. A suggestion for which button to  feature — never a decision. All three actions stay available. | 
**variation_group_id** | Option<**String**> | The variation group to ADD to, when the match already belongs to one.  Null means there is no group yet and choosing `variation` creates one from  the match plus the new listing. Without this the UI has to guess, and  guessing wrong means either a second group beside the first or a silent  no-op. | [optional]
**listing_id** | Option<**String**> | The seller's existing listing this scan probably duplicates (null if the  match landed only on an inventory item with no listing row). | [optional]
**inventory_item_id** | Option<**String**> | The inventory item behind that listing, when linked. Drives the  \"View inventory\" button. | [optional]
**title** | **String** |  | 
**image_url** | Option<**String**> |  | [optional]
**match_type** | **String** |  | 
**score** | **f32** | 0–1 confidence. Image matches report 1; title matches the similarity. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


