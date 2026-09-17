# CreateMagicScanResponseTopHits

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**platform** | **String** |  | 
**title** | **String** |  | 
**price_cents** | **f32** |  | 
**image_url** | Option<**String**> |  | [optional]
**listing_url** | Option<**String**> |  | [optional]
**origin** | **String** | Origin marker for the UI badge. | 
**visual_sim** | Option<**f32**> | CLIP cosine [0,1]; populated after the visual-rank pass. | [optional]
**state** | **String** | 'active' = currently for sale; 'sold' = historical comp. UI  renders distinct badges so the seller can see both at a glance. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


