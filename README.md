# rust-turing

## 実行方法
`cargo run "initial tape string"`で実行できます。
引数のテープ初期値は末尾にBを付けます

## 実行例
0011を受理する例
```
$ cargo run 0011B
|q0|0011B
X|q1|011B
X0|q1|11B
X|q2|0Y1B
|q2|X0Y1B
X|q0|0Y1B
XX|q1|Y1B
XXY|q1|1B
XX|q2|YYB
X|q2|XYYB
XX|q0|YYB
XXY|q3|YB
XXYY|q3|B
XXYYB|q4|
```
001の受理に失敗する例
```
$ cargo run 011B 
|q0|011B
X|q1|11B
|q2|XY1B
X|q0|Y1B
XY|q3|1B
no rules for this state: (state, char)=(q3, 1)
```
