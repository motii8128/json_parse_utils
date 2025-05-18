use serde::{Deserialize, Serialize};
use serde_json;

/// Json形式文字列への変換、もしくは文字列から構造体を取得する関数を提供するトレイト
pub trait JsonParse
where
    Self: Serialize + for<'de> Deserialize<'de> + Sized + Clone,
{
    /// Json形式の文字列に変換する
    fn create_packet(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    /// Json形式の文字列を構造体に変換する
    fn from_packet(packet: &str) -> Self {
        serde_json::from_str(packet).unwrap()
    }
}

#[macro_export]
macro_rules! impl_json_parse {
    ($t:ty) => {
        impl JsonParse for $t {}
    };
}