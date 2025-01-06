# ApiV5TradeCancelOrderPostRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**inst_id** | **String** | 必填<br>产品ID，如：`BTC-USDT` | 
**ord_id** | Option<**String**> | 可选<br>订单ID， ordId和clOrdId必须传一个，若传两个，以ordId为主。 | [optional]
**cl_ord_id** | Option<**String**> | 可选<br>客户自定义订单ID<br>字母（区分大小写）与数字的组合，可以是纯字母、纯数字且长度要在1-32位之间。 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


