# GetSpatialSceneResponseItems

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**unit_id** | Option<**String**> |  | [optional]
**title** | **String** |  | 
**image_url** | Option<**String**> |  | [optional]
**thumb_url** | Option<**String**> | A small copy of `imageUrl`, when the catalog has one.    The room binds this for everything except the few items you are standing  in front of. A 600x600 original costs 1.83 MiB of VRAM; a 150x150 thumb  costs 0.11 MiB, and at more than a couple of metres they are the same  handful of pixels on screen. Null when the catalog never made one, which  the renderer treats as \"use the original\" rather than as \"draw nothing\". | [optional]
**size** | Option<[**crate::models::GetSpatialSceneResponseSize**](GetSpatialSceneResponse_size.md)> |  | [optional]
**cost_cents** | Option<**f32**> | Cents the seller paid. Drives the `value` overlay and capital density. | [optional]
**age_days** | Option<**f32**> | Days since first listed anywhere. Drives the `aging` overlay. | [optional]
**status** | **String** |  | 
**location** | Option<**String**> | Free-text or structured location, when the unit has one. | [optional]
**market_tagged** | **bool** | True when this row resolved to a catalog product. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


