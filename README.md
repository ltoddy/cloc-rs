# cloc-rs

Count, or compute differences of, lines of source code and comments.

## Overview

![](https://img.vim-cn.com/1b/5f012cb8f2e45001ae20d18310ec0da474fff3.jpg)

### Usage

e.g.

```
macbox :: src/github.com-1ecc6299db9ec823 » cloc .
      0.8915 secs
┌────────────────────────────────────────────────────────────────────────┐
| Language         files        size       blank     comment        code |
├────────────────────────────────────────────────────────────────────────┤
| Bat                 25    69.00 KB         321           0        2127 |
| C                 2022    31.11 MB      171915      146101      806618 |
| CHeader           1684    12.19 MB       42874      131952      156036 |
| CSS                 24   169.76 KB         271          92        1414 |
| Cpp                 89     1.19 MB        4571        4242       28867 |
| CppHeader           73   345.98 KB        1673        1707        7456 |
| Go                   5   137.84 KB         605         447        3911 |
| Haskell              1     74.00 B           1           0           3 |
| Html               252     1.55 MB         987           0       63786 |
| JavaScript         294     1.41 MB        4375        2208       19874 |
| Json              1090     1.29 MB           3           0       33999 |
| Markdown          2930    10.59 MB       78502           0      208939 |
| Protobuf             1     1.10 KB           2           0          28 |
| Python              95   865.36 KB        4095       10216       15230 |
| Ruby               175   594.49 KB        3501        3603       18335 |
| Rust             34286   306.20 MB      749237     1305784     7270348 |
| Shell              270     1.11 MB        5060        6647       28384 |
| Toml              1640     2.18 MB        8957       14968       54014 |
| Xml                236     1.23 MB        2852        1681       22814 |
| Yaml              1256     1.36 MB        4238        2892       43028 |
├────────────────────────────────────────────────────────────────────────┤
| Sum              46448   373.54 MB     1084040     1632540     8785211 |
└────────────────────────────────────────────────────────────────────────┘
```

### Install

```
$ cargo install cloc
```

or

```
$ git clone https://github.com/ltoddy/cloc-rs.git
$ cargo install --path .
```

### How to contribute(support for more language)

- 在`src/config.rs`文件中, `Config`结构体的`default`方法中, 使用`language!`宏来定义规则.

> language!($name, $ext, $single, $multi)

参数解释:

- 第一个参数: 语言的名字
- 第二个参数: 这个语言文件的文件后缀
- 第三个参数: 这个语言的单行注释(没有就不填)
- 第四个参数: 这个语言的多行注释(没有就不填)

### TODO

- [ ] 统计结果可以生成Markdown, Html文件(由参数--output=(markdown)|(html) 指定)
- [ ] 去除代码中使用的`unwrap()`
- [ ] 去除代码中的expect, 更细粒度的错误处理
- [ ] 美化输出
- [ ] 完善测试
- [ ] 完善文档
- [ ] CI加上`cargo clippy --release -- -D clippy::all`限制
- [ ] 压榨性能
- [ ] ...
