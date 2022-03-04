# nimi

Simple something linguistic that can only evaluate arithmetic and function call

```sh
"+ + 100 10 20"               -> 130
"* 10 10"                     -> 100
"fn[* . .] fn(10)"            -> 100
"fn[* . .] fn(fn(fn(fn(2))))" -> 65536
```
