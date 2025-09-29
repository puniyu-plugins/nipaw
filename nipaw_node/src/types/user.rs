use napi_derive::napi;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[napi(object)]
pub struct UserInfo {
    /// 用户Id
    pub id: String,
    /// 登录用户名
    pub login: String,
    /// 用户昵称
    pub name: String,
    /// 用户邮箱
    pub email: Option<String>,
    /// 用户头像URL
    pub avatar_url: String,
    /// 关注者数量
    pub followers: u32,
    /// 关注的用户数量
    pub following: u32,
}
