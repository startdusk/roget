## 查看性能

https://rustc-dev-guide.rust-lang.org/profiling/with_perf.html
https://perf.wiki.kernel.org/index.php/Main_Page

### WSL2 安装`perf`(https://gist.github.com/abel0b/b1881e41b9e1c4b16d84e5e083c38a13)

WSL2 还是直接下载源码下来编译，然后放到/usr/bin

首先编译运行(要 release 运行，debug 模式会附带很多调试的东西)
先设置 Cargo.toml 如下

```toml
[profile.release]
debug = true
```

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

然后执行导出到 svg 图片

```bash
perf script | inferno-collapse-perf | inferno-flamegraph > perf.svg
```

如果导出碰到下面的问题

```
[ERROR inferno::flamegraph] No stack counts found
Error: Io(Custom { kind: InvalidData, error: "No stack counts found" })
```

[执行下面](https://github.com/jonhoo/inferno/issues/226)

```
echo 0 | sudo tee /proc/sys/kernel/perf_event_paranoid
```

然后再导出

上面的查看火焰图的方式，好像 wsl2 不支持，只能看到 roget，不能看到里面的程序的运行细节



### hyperfine 测试
```

```