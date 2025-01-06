# ApiV5TradeOrderAlgoPostRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**inst_id** | **String** | 必填<br>产品ID，如：`BTC-USDT` | 
**td_mode** | **String** | 必填<br>交易模式<br>保证金模式：`isolated`：逐仓 ；`cross`<br>全仓非保证金模式：`cash`：非保证金 | 
**side** | **String** | 必填<br>订单方向。买：`buy` 卖：`sell` | 
**ord_type** | **String** | 必填<br>订单类型。<br>`conditional`：单向止盈止损<br>`oco`：双向止盈止损<br>`trigger`：计划委托<br>`iceberg`：冰山委托<br>`twap`：时间加权委托 | 
**sz** | **String** | 必填<br>委托数量 | 
**tag** | Option<**String**> | 订单标签<br>字母（区分大小写）与数字的组合，可以是纯字母、纯数字，且长度在1-16位之间 | [optional]
**pos_side** | Option<**String**> | 可选<br>持仓方向<br>在双向持仓模式下必填，且仅可选择 `long` 或 `short` | [optional]
**ccy** | Option<**String**> | 非必填<br>保证金币种，如：USDT<br>仅适用于单币种保证金模式下的全仓杠杆订单 | [optional]
**reduce_only** | Option<**bool**> | 非必填<br>是否只减仓，`true` 或 `false`，默认`false`<br>仅适用于币币杠杆订单 | [optional]
**tgt_ccy** | Option<**String**> | 非必填<br>市价单委托数量的类型<br>交易货币：`base_ccy`<br>计价货币：`quote_ccy`<br>仅适用于币币订单 | [optional]
**tp_trigger_px** | Option<**String**> | 非必填<br>止盈触发价，如果填写此参数，必须填写止盈委托价<br>适用于`止盈止损委托` | [optional]
**tp_ord_px** | Option<**String**> | 非必填<br>止盈委托价，如果填写此参数，必须填写止盈触发价<br>委托价格为-1时，执行市价止盈<br>适用于`止盈止损委托` | [optional]
**sl_trigger_px** | Option<**String**> | 非必填<br>止损触发价，如果填写此参数，必须填写止损委托价<br>适用于`止盈止损委托` | [optional]
**sl_ord_px** | Option<**String**> | 非必填<br>止损委托价，如果填写此参数，必须填写止损触发价<br>委托价格为-1时，执行市价止损<br>适用于`止盈止损委托` | [optional]
**trigger_px** | Option<**String**> | 非必填<br>计划委托触发价格<br>适用于`计划委托` | [optional]
**order_px** | Option<**String**> | 非必填<br>委托价格<br>委托价格为-1时，执行市价委托<br>适用于`计划委托` | [optional]
**px_var** | Option<**String**> | 非必填<br>距离盘口的比例<br>pxVar和pxSpread只能传入一个<br>适用于`冰山委托`和`时间加权委托` | [optional]
**px_spread** | Option<**String**> | 非必填<br>距离盘口的比例价距<br>适用于`冰山委托`和`时间加权委托` | [optional]
**sz_limit** | Option<**String**> | 非必填<br>单笔数量<br>适用于`冰山委托`和`时间加权委托` | [optional]
**px_limit** | Option<**String**> | 非必填<br>挂单限制价<br>适用于`冰山委托`和`时间加权委托` | [optional]
**time_interval** | Option<**String**> | 非必填<br>挂单限制价<br>适用于`时间加权委托` | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


