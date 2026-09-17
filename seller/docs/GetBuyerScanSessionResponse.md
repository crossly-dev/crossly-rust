# GetBuyerScanSessionResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**device** | **String** |  | 
**label** | Option<**String**> |  | [optional]
**started_at** | **String** |  | 
**ended_at** | Option<**String**> |  | [optional]
**live** | **bool** |  | 
**capture_count** | **f32** |  | 
**saved_cents** | **f32** | Sum of measured savings. Unmeasured captures contribute 0, not null. | 
**captures** | [**Vec<crate::models::GetBuyerScanSessionResponseCaptures>**](GetBuyerScanSessionResponse_captures.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


