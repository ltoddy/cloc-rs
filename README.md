# cloc-rs

Count, or compute differences of, lines of source code and comments.

## 设计

```
                                    --> Calculate -\
walk directory --> channel<PathBuf> --> Calculate -> Sum
                                    --> Calculate -/
```

主线程与线程池, 主线程去递归的读取目录下的文件路径, 然后将路径发送到管道里面,
线程池中的每个线程去接收管道的发送来的路径, 然后去读取路径文件中的内容, 按照
计算规则去计算, 最后将计算结果进行聚合.

### 使用

e.g.

```
$ cargo run -- src/
      0.0066 secs
┌────────────────────────────────────────────────────────────────────────┐
| Language         files        Size       Blank     Comment        Code |
├────────────────────────────────────────────────────────────────────────┤
| Rust                10    19.91 KB          91          17         594 |
├────────────────────────────────────────────────────────────────────────┤
| Sum                 10    19.91 KB          91          17         594 |
└────────────────────────────────────────────────────────────────────────┘
```

### 安装

```
$ cargo install cloc
```

或者本地安装

```
$ git clone https://github.com/ltoddy/cloc-rs.git
$ cargo install --path cloc-rs
```

### 如何贡献(更多语言支持)

- 在`src/config.rs`文件中, `Config`结构体的`default`方法中, 使用`language!`宏来定义规则.

> language!($name, $ext, $single, $multi)

参数解释:

- 第一个参数: 语言的名字
- 第二个参数: 这个语言文件的文件后缀
- 第三个参数: 这个语言的单行注释(没有就不填)
- 第四个参数: 这个语言的多行注释(没有就不填)

### TODO

- [x] 添加更多语言的统计规则
- [x] 每种语言分类统计的文件总大小和文件数量
- [x] 添加统计详情的总时间
- [x] 统计结果可以按照顺序排列(sort_by: language name, code, comment ..., 由参数--sort-by指定
- [ ] 统计结果可以生成Markdown, Html文件(由参数--output=(Markdown)|(Html) 指定)
- [ ] 优化代码最终统计详情的聚合方式
- [ ] 去除代码中使用的`unwrap()`
- [ ] 添加日志(由--trace参数指定是否打开)
- [ ] 优化代码实现
- [ ] 去除代码中的expect, 更细粒度的错误处理
- [ ] 完善文档
- [ ] ...
