# GetTaxonomyCategoryAspectResponseAspects

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**required** | **bool** |  | 
**data_type** | **String** | 'STRING' | 'NUMBER' | 'DATE' — eBay's dataType per aspect. | 
**has_enum_values** | **bool** | True when the aspect is selection-only (no free-text). | 
**enum_values** | Option<**Vec<String>**> |  | [optional]
**max_length** | Option<**f32**> | Helpful for client validation — max length when supplied. | [optional]
**cardinality** | **String** | Cardinality — SINGLE_VALUE / MULTIPLE_VALUES. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


