Day 24: Never Tell Me The Odds
=============================

```
# python の z3 ライブラリを使う場合はインストールする。
pip3 install z3-solver
```

Using z3 crate for necessarily install  
Rustで、z3 crateを使う場合は、z3ライブラリをシステムにインストールしておく必要がある。

```
brew install z3
```

z3 crate をビルドする際に、z3のヘッダファイルが必要。
MacOSだと、homebrewでインストールしたz3ライブラリのヘッダファイルが見つからないので、
以下のように環境変数(CPATH and LIBRARY_PATH)を設定する必要あり。  
`brew --prefix` で、homebrewのインストール先はわかる。

`/opt/homebrew/include/z3.h`

```
# ~/.zprofile or ~/.bash_profile
# brew header and lib
export CPATH="/opt/homebrew/include:$CPATH"
export LIBRARY_PATH="/opt/homebrew/lib:$LIBRARY_PATH"
```

include配下が、ヘッダーファイル。
lib配下が、ライブラリファイル。
