# ApiV5AccountSetLeveragePostRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**lever** | **String** | 必填<br>杠杆倍数 | 
**mgn_mode** | **String** | 保证金模式<br>`isolated`：逐仓，`cross`：全仓<br>如果`ccy`有效传值，该参数值只能为`cross`。 | 
**inst_id** | Option<**String**> | 可选<br>产品ID：币对、合约<br>`instId`和`ccy`至少要传一个；如果两个都传，默认使用instId | [optional]
**ccy** | Option<**String**> | 可选<br>保证金币种，仅适用于跨币种保证金模式的全仓`币币杠杆`。 | [optional]
**pos_side** | Option<**String**> | 可选<br>持仓方向<br>`long`：双向持仓多头<br>`short`：双向持仓空头<br>`net`：单向持仓<br>在双向持仓且保证金模式为逐仓条件下必填，且仅可选择`long`或`short`，其他情况下非必填，默认`net`；仅适用于`交割/永续` | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


