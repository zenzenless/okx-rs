# ApiV5RfqExecuteQuotePostRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**rfq_id** | **String** | 询价单ID.  | 
**quote_id** | **String** | 报价单ID. | 
**legs** | Option<[**models::ArrayOfObjects**](array of objects.md)> | 用于部分执行的腿的数量。腿的数量比例必须与原RFQ相同。注意：每条腿的tgtCcy和side和原RFQ一致，px和对应Quote一致。 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


