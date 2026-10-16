# PetPhrase Orb — 幻彩圆球版

Windows 桌面常用语与剪贴板短语管理器，以小巧的幻彩渐变圆球作为桌面入口。点击圆球打开面板，点击短语复制到剪贴板。

**这是基于 [chengbuilds/PetPhrase](https://github.com/chengbuilds/PetPhrase) 的独立二次开发版本。** 原项目及核心功能由上游作者 chengbuilds 提供，本仓库主要调整桌面入口外观、边缘渲染和任务栏行为，由 [diseno1210](https://github.com/diseno1210) 维护。

## 来源与许可

- 上游项目：[chengbuilds/PetPhrase](https://github.com/chengbuilds/PetPhrase)。
- 开发基线：上游 v0.7.0，提交 [`c259810`](https://github.com/chengbuilds/PetPhrase/commit/c259810)，保留上游 Git 历史。
- 原项目说明保存在 [README.upstream.md](README.upstream.md)，其中截图、下载链接及版本介绍对应上游项目。
- 圆球外观参考 Uiverse 上 dexter-st 的 [Neon liquid button](https://uiverse.io/dexter-st/breezy-zebra-94)，改为圆形并使用原生生成的动画素材。
- 保留上游 MIT 版权与许可全文，详见 [LICENSE](LICENSE)；外观参考及相关许可见 [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md)。

本仓库为独立维护的二开版本，问题反馈和下载请使用本仓库的 Issues 与 Releases。

## 本版改动

- 默认桌宠替换为蓝紫、青色流动渐变的玻璃圆球。
- 去除 Start 文字与白色外描边，默认入口尺寸缩小。
- 按窗口实际物理像素预缩放动画帧，结合预乘透明度过滤，改善小尺寸和不同显示缩放比例下的边缘锯齿。
- 桌面入口、短语面板和设置窗口隐藏任务栏按钮，保留系统托盘入口。
- 保留点击打开面板、拖拽、位置记忆、分组、搜索、复制、导入导出、主题及自定义桌宠功能。
- 更新地址指向本仓库，避免更新回上游外观。

![幻彩圆球的三个动画帧](screenshots/liquid-orb-preview.png)

上图为放大的素材预览，实际桌面大小可在设置中选择。

## 下载与使用

支持 Windows 10/11 x64。

1. 打开 [Releases](https://github.com/diseno1210/PetPhrase-Orb/releases/latest)，下载 `PetPhrase-Orb_0.7.1_windows-x64.zip`。
2. 完整解压压缩包，双击 `PetPhrase.exe`。保留同目录下的 `pets` 文件夹。
3. 点击圆球打开常用语面板；通过面板设置或系统托盘管理程序。

便携包无需安装，程序未做商业代码签名。用户数据保存在 `%APPDATA%\PetPhrase`，沿用上游的单实例标识和数据目录；与上游版本切换运行时会共用现有设置和短语。

本次发布提供便携包。源码保留安装包构建能力，自动更新功能需要后续发行版提供与版本对应的 NSIS 安装包及 SHA256 文件。

## 从源码构建

安装 Rust stable（MSVC）、Visual Studio C++ Build Tools 和 Windows SDK。

```powershell
git clone https://github.com/diseno1210/PetPhrase-Orb.git
cd PetPhrase-Orb/app-slint
cargo test --locked
cargo build --release --locked
Copy-Item -LiteralPath pets -Destination target/release -Recurse -Force
.\target\release\PetPhrase.exe
```

重新生成圆球素材需要 Python、NumPy 和 Pillow；普通构建直接使用仓库中已生成的 PNG，无需 Python。在仓库根目录运行：

```powershell
python -m pip install numpy pillow
python scripts/gen_liquid_orb.py
```

可选安装包：在 `app-slint` 目录运行 `makensis installer.nsi`（需要 NSIS）。版本号需同步修改 `Cargo.toml`、`Cargo.lock` 与 `installer.nsi`。

## 代码入口

| 内容 | 文件 |
| --- | --- |
| 圆球动画素材生成 | `scripts/gen_liquid_orb.py` |
| 物理像素缩图与透明边缘处理 | `app-slint/src/liquid_orb.rs` |
| 桌面入口与交互 | `app-slint/ui/pet.slint` |
| Windows 任务栏隐藏 | `app-slint/src/tray_window.rs` |
| 程序入口、托盘、面板与事件 | `app-slint/src/main.rs` |
| 分组、短语及设置持久化 | `app-slint/src/storage.rs` |

## 致谢

感谢 [chengbuilds](https://github.com/chengbuilds) 开源 PetPhrase，感谢 [dexter-st](https://uiverse.io/dexter-st) 提供外观灵感；项目继续使用 [Slint](https://slint.dev/)、[petdex](https://petdex.dev/) 格式及 [Lucide](https://lucide.dev/) 图标。
