Day 7: Camel Cards
=============================

## PartialEq と Eq の違い

### PartialEq

eq関数を持つ

* 対称関係: a == b, b == a
* 推移関係: a == b, b == c, a == c

が成り立つことを表現する。

### Eq

PartialEqのサブセットで、  
反射関係(reflexive relation): a == a が成り立つことを定義する。  
(新たに関数実装が必要になるわけではない)

## PartialOrd と Ord の違い

### PartialOrd

partial_cmp関数を持つ  
比較できないものは、Noneを返せる。  
(例えば、f64::NAN は、どの値とも比較できないのでNoneを返す)

PartialEqのサブセットなので、PartialEq::eqを実装しておく必要がある。

### Ord

全ての要素で比較ができるもの。  
min/max などの関数を使えるようになる。

Ord まで実装する際には、cmp関数に実装をして、
partial_cmp関数でラップするようにしましょう。  
[non_canonical_partial_ord_impl](https://rust-lang.github.io/rust-clippy/master/index.html#/non_canonical_partial_ord_impl)

## ファントムマーカー

* Hand::<NormalRule>
* Hand::<JokerRule>

構造体を区別して使うため、
PartialEq, Eq をderiveマクロではなく明示的に実装する必要がある。

