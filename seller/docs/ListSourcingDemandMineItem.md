# ListSourcingDemandMineItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**identifier_value** | **String** |  | 
**lookers** | **f32** | How many distinct shoppers looked, in the window. | 
**misses** | **f32** | How many of those looks Crossly could not answer at all. | 
**median_retail_cents** | Option<**f32**> | What the retailers were charging, median of what Scout saw. | [optional]
**relation** | **String** | 'in_stock' — it is in their inventory. 'sold_before' — they have sold one. | 
**inventory_item_id** | Option<**String**> | Their own row, for the link. | [optional]
**title** | Option<**String**> |  | [optional]
**last_sold_cents** | Option<**f32**> | What they got for it last time, when they have sold one. | [optional]
**last_sold_at** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


