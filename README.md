# json_parse_utils

|name|status|
|:--:|:--:|
|Rust|[![Rust](https://github.com/motii8128/json_parse_utils/actions/workflows/rust.yml/badge.svg)](https://github.com/motii8128/json_parse_utils/actions/workflows/rust.yml)|

#　使用方法
Cargo.tomlに以下のように追記する
```toml
serde = {version = "1.0.148", features = ["derive"]}
json_parse_utils = {git = "https://github.com/motii8128/json_parse_utils.git"}
```

# 例
以下に利用例を示します。ここではMyStructという構造体をJson変換可能にしました。
```rs
// 本ライブラリと共にserdeもuseする
use json_parse_utils::{JsonParse, impl_json_parse};
use serde::{Serialize, Deserialize};

// 少しめんどくさいがこのderive(...)は絶対につけなければならない
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyStruct
{
    name : String,
    age : u8
}

// このマクロによって構造体にトレイトが実装される
impl_json_parse!(MyStruct);

fn main()
{
    // 構造体の初期化
    let s = MyStruct{
        name : "Hello".to_string(),
        age : 2
    };

    // 構造体->String
    let json_text = s.create_packet();

    // 変換して得た文字列を表示
    println!("Result to text -> {}", json_text);

    // 文字列->構造体
    let new_my_struct = MyStruct::from_packet(&json_text);

    // 構造体変数として表示
    println!("Result from text -> Name:{}, Age:{}", new_my_struct.name, new_my_struct.age);
}
```

実行結果は以下のようになります。
```
Result to text -> {"name":"Hello","age":2}
Result from text -> Name:Hello, Age:2
```
変換しても値がしっかり同じであることが確認できます。