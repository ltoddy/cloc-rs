# cloc-rs

Count, or compute differences of, lines of source code and comments.

## Overview

```
                                    --> calculate -\
walk directory --> channel<PathBuf> --> calculate --> sum
                                    --> calculate -/
```

### Usage

e.g.

```
ltoddy@linuxbox:~/.cargo/registry/src$ cloc github.com-1ecc6299db9ec823 --sort-by=code
      0.5971 secs
┌────────────────────────────────────────────────────────────────────────┐
| Language         files        Size       Blank     Comment        Code |
├────────────────────────────────────────────────────────────────────────┤
| Html                 3    622.00 B           1           0          19 |
| CSS                  3     1.56 KB          15           6          54 |
| Ruby                 1     1.41 KB           8           2          68 |
| Python              21   120.17 KB         556        2031        1049 |
| Cpp                  9    79.25 KB         190          42        1830 |
| Shell               96   289.45 KB        1169        1728        7523 |
| Json               222   410.30 KB           0           0       17563 |
| CHeader            242  1018.47 KB        3655        8764       20101 |
| Markdown           381     1.59 MB       11747           0       30570 |
| C                  257     2.46 MB       11613        8589       70725 |
| Rust              4861    59.06 MB      128216      245762     1476442 |
├────────────────────────────────────────────────────────────────────────┤
| Sum               6096    64.99 MB      157170      266924     1625944 |
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
- [ ] ...
