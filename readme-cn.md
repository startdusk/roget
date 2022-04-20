## 查看性能

https://rustc-dev-guide.rust-lang.org/profiling/with_perf.html
https://perf.wiki.kernel.org/index.php/Main_Page

### WSL2 安装`perf`(https://gist.github.com/abel0b/b1881e41b9e1c4b16d84e5e083c38a13)

WSL2 还是直接下载源码下来编译，然后放到/usr/bin

首先编译运行(要 release 运行，debug 模式会附带很多调试的东西)

```bash
cargo r --release
```

查看进程, 获取 pid

```bash
pgrep -af roget
```

然后收集程序运行数据写入到 perf.data 用于分析(注意，会写入很快，文件会很快变大，所以需要手动 Ctrl+C 停止写入)

```bash
perf record --call-graph dwarf -p 上面获取的pid
```
