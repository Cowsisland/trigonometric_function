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
```
Add: 加算
Sub: 減算
Mul: 乗算
Div: 除算
Rem: 剰余
```
これらのトレイトは、数値型に対して算術演算を定義するために使用される。

std::cmpモジュール:
```
PartialOrd: 部分順序付け
Ord: 全順序付け
PartialEq: 部分等価
Eq: 全等価
```
これらのトレイトは、数値型に対して比較操作を定義するために使用される。

## 次回方針(20240719)
Number型の実装について詳細に理解しなおす。

下記手順で構成中。
1. 三角関数を行うための`Number`型をenumで作成
2. Number型に対して、三角関数を計算する`TrigFunctions`トレイトを実装
3. 各変数をNumber型に変更してビルドを通す

次にやること
1. `TrigFunctions`の残りの三角関数を実装する
2. `Number`の残りのプリンシプル型への変更を実装する
3. `Number`についてはもっと型への対応を追加してもよいかも（現在、i32,i64,f32,f64のみ）

## 次回方針(20240720)
Number型をジェネリックから関連型に変更。理由は、三角関数のtraitの実装時、Number型一つに定まるため。

次にやること
1. `TrigFunctions`の残りの三角関数を実装する
2. エラー処理の実装
3. `Number`の残りのプリンシプル型への変更を実装する
4. `Number`についてはもっと型への対応を追加してもよいかも（現在、i32,i64,f32,f64のみ）

## 次回方針(20240724)
Numberをtraitで再実装。これにより、enum型を通していない。数値のすべてにtoほにゃららを実装し、さらにf64にはNumberを継承させたtrig_functionsトレイトを実装

次にやること
1. `TrigFunctions`の残りの三角関数を実装する
2. テストの実装
3. エラー処理の実装

## 次回方針(20240802)
三角関数を実装完了。テストの実装も完了。残りはエラー処理を実装する。（this_errorかな？）

次にやること
1. エラー処理の実装

## 次回方針(20240802)
thisErrorを使用する枠組みは作成済。overflowの時とtanの時用の実装をする。

次にやること
1. エラー処理の実装（tanの時に違うエラーを実装する）

## 次回方針(20240802)
overflowの時とtanの時用の実装済。

次にやること
1. 正弦定理、余弦定理あたりの実装