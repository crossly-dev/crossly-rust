# CreateInventoryCsvImportResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**problems** | [**Vec<crate::models::CreateInventoryCsvImportResponseProblems>**](CreateInventoryCsvImportResponse_problems.md) |  | 
**problem_count** | **f32** |  | 
**max_rows** | **f32** |  | 
**created** | **f32** |  | 
**updated** | **f32** |  | 
**usable** | **f32** | Rows that mapped cleanly. `created + updated` when not a dry run. | 
**total_rows** | **f32** |  | 
**listings_created** | **f32** |  | 
**dry_run** | **bool** | True when nothing was written — a preview pass. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


