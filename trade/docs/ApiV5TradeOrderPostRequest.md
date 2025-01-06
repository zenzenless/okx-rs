# ApiV5TradeOrderPostRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**inst_id** | **String** | 必填<br>产品ID，如：`BTC-USDT` | 
**td_mode** | **String** | 必填<br>交易模式<br>保证金模式：`isolated`：逐仓 ；`cross`<br>全仓非保证金模式：`cash`：非保证金 | 
**side** | **String** | 必填<br>订单方向。买：`buy` 卖：`sell` | 
**ord_type** | **String** | 必填<br>订单类型。<br>市价单：`market`<br>限价单：`limit`<br>只做maker单：`post_only`<br>全部成交或立即取消：`fok`<br>立即成交并取消剩余：`ioc`<br>市价委托立即成交并取消剩余：`optimal_limit_ioc`（仅适用交割、永续） | 
**sz** | **String** | 必填<br>委托数量 | 
**px** | Option<**String**> | 可选<br>委托价格<br>仅适用于`limit`、`post_only`、`fok`、`ioc`类型的订单 | [optional]
**pos_side** | Option<**String**> | 可选<br>持仓方向<br>在双向持仓模式下必填，且仅可选择 `long` 或 `short` | [optional]
**ccy** | Option<**String**> | 非必填<br>保证金币种，如：USDT<br>仅适用于单币种保证金模式下的全仓杠杆订单 | [optional]
**cl_ord_id** | Option<**String**> | 非必填<br>客户自定义订单ID<br>字母（区分大小写）与数字的组合，可以是纯字母、纯数字且长度要在1-32位之间。 | [optional]
**tag** | Option<**String**> | 非必填<br>订单标签<br>字母（区分大小写）与数字的组合，可以是纯字母、纯数字，且长度在1-8位之间。 | [optional]
**reduce_only** | Option<**bool**> | 非必填<br>是否只减仓，`true` 或 `false`，默认`false`<br>仅适用于币币杠杆订单 | [optional]
**tgt_ccy** | Option<**String**> | 非必填<br>市价单委托数量的类型<br>交易货币：`base_ccy`<br>计价货币：`quote_ccy`<br>仅适用于币币订单 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


