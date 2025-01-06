# ApiV5AccountPositionMarginBalancePostRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**inst_id** | **String** | 必填<br>产品ID，如：`BTC-USDT-SWAP` | 
**pos_side** | **String** | 必填<br>持仓方向，默认值是`net`<br>`long`：双向持仓多头<br>`short`：双向持仓空头<br>`net`：单向持仓 | 
**r#type** | **String** | 必填<br>增加/减少保证金<br>`add`：增加 `reduce`：减少 | 
**ccy** | Option<**String**> | 增加或减少的保证金的币种<br>仅适用于逐仓自主划转保证金模式下的币币杠杆 | [optional]
**auto** | Option<**String**> | 是否自动借币 true 或 false<br>仅适用于逐仓自主划转保证金模式下的币币杠杆 | [optional]
**loan_trans** | Option<**bool**> | 是否支持`跨币种保证金模式`下的借币转出/转入默认为`false`。 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


