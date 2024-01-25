<p align="center">
<a href="./assets/icon.raw.png">
<img src="./assets/icon.raw.png" width="100" height="100" alt="logo">
</a>
<h3 align="center">Aria 下载管理器</h3>
<p align="center">一款专为 macOS 设计的，基于 aria2 下载管理器</p>
</p>
<p align="center">
<a href="./README_zh.md">中文</a> | <a href="./README.md">English</a>
</p>

## 展示
![show_img](./assets/show.png)

## 依赖
* rust `>= 1.74.0`
* aria2

## 构建
1. 克隆此仓库.
```shell
git clone https://github.com/iewnfod/aria-download-manager.git
```
2. 运行打包脚本.
```shell
scripts/build.sh
```
3. target 文件夹中将会生成后缀名为 `.app` 和 `.dmg` 的应用文件

## 开发
### Adm Tray
1. 使用 `cargo build` 为下载器创建一个 debug 版的可执行文件
2. 将此文件移动到 adm-tray 的 target 目录中
3. 进入 adm-tray 的目录并使用 `cargo run` 来运行你的代码
### Aria Download Manager
1. 打开此应用或者 adm-tray 的可执行文件，确保 aria2 的服务已经正常开启
2. 使用 `cargo run` 来运行你的代码
