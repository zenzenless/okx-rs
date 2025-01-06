# ApiV5TradeAmendOrderPostRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**inst_id** | **String** | 必填<br>产品ID，如：`BTC-USDT` | 
**ord_id** | Option<**String**> | 可选<br>订单ID， ordId和clOrdId必须传一个，若传两个，以ordId为主。 | [optional]
**cl_ord_id** | Option<**String**> | 可选<br>客户自定义订单ID<br>字母（区分大小写）与数字的组合，可以是纯字母、纯数字且长度要在1-32位之间。 | [optional]
**new_sz** | Option<**String**> | 可选<br>修改的新数量，newSz和newPx不可同时为空。对于部分成交订单，该数量应包含已成交数量。 | [optional]
**new_px** | Option<**String**> | 可选<br>修改的新价格 | [optional]
**cxl_on_fail** | Option<**bool**> | 非必填<br>`false`：不自动撤单 `true`：自动撤单 当订单修改失败时，该订单是否需要自动撤销。默认为`false` | [optional]
**req_id** | Option<**String**> | 非必填<br>用户自定义修改事件ID，字母（区分大小写）与数字的组合，可以是纯字母、纯数字且长度要在1-32位之间。 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


