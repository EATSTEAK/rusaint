use super::{client::WebDynproClient, error::WebDynproError};

/// WebDynpro 클라이언트를 조작하는 명령
pub trait WebDynproCommand {
    /// 해당 명령이 반환할 결과
    type Result;

    /// 해당 명령을 주어진 클라이언트에 대해 실행합니다.
    async fn dispatch(&self, client: &mut WebDynproClient) -> Result<Self::Result, WebDynproError>;
}

/// 엘리먼트 관련 명령
pub mod element;
