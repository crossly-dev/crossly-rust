# CreateMagicScanResponsePossibleDuplicates

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**listing_id** | Option<**String**> | The seller's existing listing this scan probably duplicates (null if the  match landed only on an inventory item with no listing row). | [optional]
**inventory_item_id** | Option<**String**> | The inventory item behind that listing, when linked. Drives the  \"View inventory\" button. | [optional]
**title** | **String** |  | 
**image_url** | Option<**String**> |  | [optional]
**match_type** | **String** |  | 
**score** | **f32** | 0–1 confidence. Image matches report 1; title matches the similarity. | 
**variation_group_id** | Option<**String**> | The variation group the matched listing already belongs to, if any.    This is what turns \"you already have this\" into something useful for a  seller scanning a size run. Scan the Medium, scan the Large, and the  second scan lands here — the honest answer is usually neither \"it's the  same one\" nor \"it's different\", it's \"it's another size of that\". Which  of the two offers to make depends entirely on this field:      null      → offer to CREATE a group from the match and the new listing    set       → offer to ADD the new listing to the group that exists    Without it the UI would have to guess, and guessing wrong means either a  second group beside the first or a silent no-op. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


