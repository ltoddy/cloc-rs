# cloc-rs

Count, or compute differences of, lines of source code and comments fastly.

## Overview

![](https://img.vim-cn.com/1b/5f012cb8f2e45001ae20d18310ec0da474fff3.jpg)

### Usage

e.g.

```
macbox :: github/kubernetes » time cloc .
     72.5313 secs
┌───────────────────────────────────────────────────────────────────────────────────────┐
│ Language                        files        size       blank     comment        code │
├───────────────────────────────────────────────────────────────────────────────────────┤
│ Autoconf                          533   515.91 KB         100         992       21399 │
│ Bash                              148   420.09 KB        1183         592       10153 │
│ C                                 149   149.31 KB         970        1671        3065 │
│ C Header                           25     6.94 MB        9975      109175       20975 │
│ GNU Style Assembly               2946     8.96 MB       48895       43894      285473 │
│ Go                             463514     4.44 GB    13873486    22200757   106047029 │
│ Html                               50    950.00 B           0           0          50 │
│ Ini                                24     5.70 KB          48           0         240 │
│ JSON                            21759  1011.18 MB         100           0    23771190 │
│ Lua                                25   419.92 KB         750         100       11875 │
│ Markdown                        18270   162.09 MB      530001           0     1794019 │
│ Plain Text                       1161     5.50 MB        7186           0      176276 │
│ PowerShell                        174     2.92 MB        8947       74817        1149 │
│ Protocol Buffer                  4489    51.42 MB      220374      684092      425720 │
│ Python                            175   581.23 KB        2975        5500        8750 │
│ SVG                               101     1.22 MB         101         101        9532 │
│ Shell                            9470    56.70 MB      189547      361195      998808 │
│ Toml                              299   342.58 KB        3667        4325        7223 │
│ Yaml                            38405   140.98 MB       31308       31043     5317632 │
├───────────────────────────────────────────────────────────────────────────────────────┤
│ Sum                            561717     5.86 GB    14929613    23518254   138910558 │
└───────────────────────────────────────────────────────────────────────────────────────┘
cloc .  23.21s user 153.47s system 239% cpu 1:13.72 total
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

- 在`src/calculator.rs`文件中, 有一个在`lazy_static`中的全局变量: `MANAGER`,内部有一个宏

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
- [ ] 压榨性能
- [ ] ...
