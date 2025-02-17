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
pub struct ApiV5AccountSetPositionModePostRequest {
    /// 持仓方式<br>`long_short_mode`：双向持仓<br>`net_mode`：单向持仓
    #[serde(rename = "posMode")]
    pub pos_mode: String,
}

impl ApiV5AccountSetPositionModePostRequest {
    pub fn new(pos_mode: String) -> ApiV5AccountSetPositionModePostRequest {
        ApiV5AccountSetPositionModePostRequest {
            pos_mode,
        }
    }
}

