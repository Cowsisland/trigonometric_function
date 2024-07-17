# traitを勉強する

## 三角関数を実装
それぞれの三角関数のtraitを実装し、f64型にtraitを実装していく

## 次回実装方針(20240718)
### num crate
numクレートなるものがあるらしい
下記のように書けるとのこと
```toml
[dependencies]
num = "0.4"
```

```rust
extern crate num;
use num::Num;

fn sample<T: Num + std::fmt::Display>(num: T)
```

この例では、Numトレイトは整数型や浮動小数点型に対して共通の振る舞いを提供する。
また、Numトレイトを使用することで、数値型の一貫した処理が可能になる。


### 標準ライブラリの数値トレイト
std::opsモジュール:

Add: 加算
Sub: 減算
Mul: 乗算
Div: 除算
Rem: 剰余
これらのトレイトは、数値型に対して算術演算を定義するために使用される。

std::cmpモジュール:

PartialOrd: 部分順序付け
Ord: 全順序付け
PartialEq: 部分等価
Eq: 全等価
これらのトレイトは、数値型に対して比較操作を定義するために使用される。