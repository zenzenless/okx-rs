/*
 * REST API
 *
 * # 使用说明   <b>该功能接口用户需先登陆，接口只会请求模拟环境</b><br><br>*Parameters* 面板中点击`Try it out`按钮，编辑请求参数，点击`Execute`按钮发送请求。*Responses* 面板中查看请求结果。<br>
 *
 * The version of the OpenAPI document: v5
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiV5AccountSimulatedMarginPostRequest {
    /// 产品类型<br>`SWAP`：永续合约，`FUTURES`：交割合约，`OPTION`：期权
    #[serde(rename = "instType", skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
    /// 是否代入已有仓位<br>`true`：调整被代入的已有仓位信息<br>`false`：不代入已有仓位，仅使用simPos里新增的模拟仓位进行计算,默认为True
    #[serde(rename = "inclRealPos", skip_serializing_if = "Option::is_none")]
    pub incl_real_pos: Option<String>,
    /// 调整持仓列表
    #[serde(rename = "simPos", skip_serializing_if = "Option::is_none")]
    pub sim_pos: Option<String>,
    /// 交易产品ID
    #[serde(rename = "> instId", skip_serializing_if = "Option::is_none")]
    pub greater_than__inst_id: Option<String>,
    /// 持仓量
    #[serde(rename = "> pos", skip_serializing_if = "Option::is_none")]
    pub greater_than__pos: Option<String>,
}

impl ApiV5AccountSimulatedMarginPostRequest {
    pub fn new() -> ApiV5AccountSimulatedMarginPostRequest {
        ApiV5AccountSimulatedMarginPostRequest {
            inst_type: None,
            incl_real_pos: None,
            sim_pos: None,
            greater_than__inst_id: None,
            greater_than__pos: None,
        }
    }
}

