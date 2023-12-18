Day 17: Clumsy Crucible
=============================

## ダイクストラ法

1. 隣接するノードを取得 (part1は1~3つ先まで、part2は4~10先まで)
2. ノード毎に到達コスト(heat_lossの合計)を計算してキュー(BinaryHeap)にスタック。
3. コストの低いキュー(BinaryHeap)を取り出し、再度１番から繰り返す。
4. ゴールまで到達したらそこが一番低いコスト

## BinaryHeap

数字が高いものからPOPできるスタック (Reverseつけると、逆順で取得できる)
https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html#examples



