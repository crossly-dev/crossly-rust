# CreateInventoryBulkQuantityResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**affected** | **f32** | Rows whose available stock actually changed. | 
**skipped** | **f32** | Ids that did not move. Either they were already at that number, or they  aren't this seller's. The two are deliberately not distinguished: telling  a caller \"that id isn't yours\" confirms the id exists. | 
**bulk_job_id** | Option<**String**> | Watchable job for the marketplace fan-out, when one was started. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


