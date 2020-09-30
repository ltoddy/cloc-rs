# cloc-rs

Count, or compute differences of, lines of source code and comments.

## Overview

![](https://img.vim-cn.com/1b/5f012cb8f2e45001ae20d18310ec0da474fff3.jpg)

### Usage

e.g.

```
macbox :: src/github.com-1ecc6299db9ec823 » cloc . --sort-by code
      0.9228 secs
┌───────────────────────────────────────────────────────────────────────────────────────┐
| Language                        files        size       blank     comment        code |
├───────────────────────────────────────────────────────────────────────────────────────┤
| Vue                                 1     31.00 B           0           0           2 |
| Haskell                             1     74.00 B           1           0           3 |
| Vim script                          3    168.00 B           0           0           3 |
| Sass                                2    189.00 B           2           0          11 |
| Protocol Buffer                     1     1.10 KB           2           0          28 |
| Prolog                              1    975.00 B          10           0          33 |
| Ini                                 5    577.00 B           5           1          35 |
| R                                   2     1.95 KB           6           0          40 |
| Emacs Lisp                          2     3.17 KB           6          36          40 |
| Zsh                                 2     5.21 KB          13          28          72 |
| PowerShell                          4     4.89 KB          35          36          78 |
| VBScript                            1    16.23 KB          30          60         341 |
| C#                                  4    19.89 KB          94         108         462 |
| Pest                                8    25.08 KB         122          84         542 |
| Bash                               14    29.84 KB         135         210         663 |
| Objective-C                         3    36.60 KB         290         150         776 |
| Rakefile                           56    49.90 KB         173          68        1247 |
| CSS                                22   169.58 KB         269          92        1403 |
| Pascal                              8    82.84 KB         438         416        2032 |
| Batch                              25    69.00 KB         321           2        2125 |
| GNU Style Assembly                 40    84.70 KB         497         312        2710 |
| D                                 232   128.49 KB         375           0        3138 |
| Ada                                20   173.98 KB        1198        1120        3362 |
| Go                                  5   137.84 KB         605         447        3911 |
| Automake                           76   214.18 KB        1030        1407        4550 |
| CMake                             135   302.33 KB        1300        2346        5614 |
| Visual Studio Solution             48   660.20 KB           0           0        6971 |
| ReStructuredText                   57   510.73 KB        4655           0       11246 |
| Python                             96   872.35 KB        4125       10268       15339 |
| C++ Header                         76   998.84 KB        1745        1793       17218 |
| Ruby                              175   594.49 KB        3501        3700       18238 |
| SVG                               212     2.59 MB         267        1597       19680 |
| JavaScript                        294     1.41 MB        4375        2208       19874 |
| XML                               236     1.23 MB        2852        1681       22814 |
| Autoconf                          141     1.03 MB        3643        4739       22940 |
| Shell                             281     1.13 MB        5171        6683       28811 |
| Assembly                           36   661.32 KB        5535        1056       30654 |
| JSON                             1116     1.29 MB           3           0       34131 |
| Yaml                             1277     1.41 MB        4377        3017       44398 |
| Perl                               98     1.69 MB        7408        8714       49675 |
| Toml                             1667     2.22 MB        9107       15243       54844 |
| Html                              252     1.55 MB         987          21       63765 |
| Visual Studio Project              72     2.80 MB           0           0       68806 |
| C++                               194     2.82 MB       13367        8699       72971 |
| C Header                         1962    13.68 MB       48885      154409      174226 |
| Markdown                         2981    10.93 MB       80716           0      215636 |
| C                                2274    35.55 MB      201292      163029      928637 |
| Plain Text                        591    36.28 MB       22670           0     2057371 |
| Rust                            34720   310.92 MB      765390     1340059     7375701 |
├───────────────────────────────────────────────────────────────────────────────────────┤
| Sum                             49529   434.24 MB     1197028     1733839    11387167 |
└───────────────────────────────────────────────────────────────────────────────────────┘
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
