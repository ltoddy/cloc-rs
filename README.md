# cloc-rs
Count, or compute differences of, lines of source code and comments.


### 设计

```
                                             --> Calculate -\
explorer(walk directory) -- channel<PathBuf> --> Calculate -> Sum
                                             --> Calculate -/
```

由一个线程去递归读取某目录下的路径, 然后通过管道, 将路径发送到管道中,
管道的接收方(多个线程)收到路径,读取文件中的内容, 计算出相关的数据, 最后将
产生的数据进行聚合.

### 使用

e.g.

```
$ cargo run -- src/


           9 text files.
           0 files ignored.

┌────────────────────────────────────────────────────┐
| Language              Code     Comment       Blank |
├────────────────────────────────────────────────────┤
| Rust                   477          15          92 |
├────────────────────────────────────────────────────┤
| Sum                    477          15          92 |
└────────────────────────────────────────────────────┘
```

### 安装

```
cargo install --path .
```

### 如何贡献(更多语言支持)

- 在`src/config.rs`文件中, `Config`结构体的`default`方法中, 使用`language!`宏来定义规则.

> language!($name, $ext, $single, $multi)

- 在`src/lib.rs`文件中,为枚举`Language`添加对应的成员.


参数解释:

- 第一个参数: 来源于`src/lib.rs`的`Language`枚举
- 第二个参数: 这个语言文件的文件后缀
- 第三个参数: 这个语言的单行注释
- 第四个参数: 这个语言的多行注释(没有的话就用`vec![]`)

### TODO

- [ ] 统计结果可以生成Markdown, Html文件(由参数--output=(Markdown)|(Html) 指定)
- [ ] 统计结果可以按照顺序排列(sort_by: language name, code, comment ..., 由参数--sort-by指定)
- [ ] 优化代码最终统计详情的聚合方式
- [ ] 去除代码中使用的`unwrap()`
- [ ] 添加更多语言的统计规则
- [ ] 每种语言分类统计的文件总大小和文件数量
- [x] 添加统计详情的总时间
- [ ] 添加日志(由--trace参数指定是否打开)
- [ ] 优化代码实现
- [ ] ...
