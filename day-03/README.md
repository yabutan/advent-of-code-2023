Day 3: Gear Ratios
=============================

Tips:

1. nom で位置取得もしたい場合は、input を `nom_locale::LocatedSpan` でラップした物を利用する。
   https://docs.rs/nom_locate/latest/nom_locate/

2. `pos_x.saturating_sub(1)` で、 u32でマイナス値にならずにゼロにしてくれる。
    

