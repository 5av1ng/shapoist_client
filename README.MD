# Shpoist Client

[English version](https://github.com/5av1ng/shapoist_client/blob/main/ReadMeEnUs.MD)

## 关于 Shapoist 本身

Shapoist 是一款社区向的音游（尽管服务器功能暂未实现）

### 关于玩法

Shapoist 提供了一种较为抽象的玩法——将判定范围与 Note 分离。Note 与判定范围建立映射关系，在合适时间点击 Note 对应的判定范围即可判定，同时提供动画制作功能。结合这两者便可以较为轻松的实现绝大多数音游玩法（注：拥有滑块控件的音游暂时无法还原，因为 Shapoist 并未提供对应的抽象）。

### 关于判定与计分

Shapoist 判定方式分为两种，判定等级分为五种。

1. 无暇模式

    | 判定等级   | 准度计算方式（Acc）                                          | 判定区间 |
    | ---------- | ------------------------------------------------------------ | -------- |
    | Immaculate | $80\% + (1 - \frac{(点击时间 - 判定时间)}{50ms})\cdot 20\%$ | ±50 ms   |
    | Extra      | $70\%$                                                      | ±70 ms   |
    | Normal     | $50\%$                                                       | ±120 ms  |
    | Fade       | $0\%$                                                        | ±150 ms  |
    | Miss       | $0\%$                                                        | N/A      |

    计分方式：$(\frac{Acc\cdot当前已经判定的 Note}{总 Note 数}\times0.95+\frac{最大连击数}{总 Note 数}\times0.05)\times2000000$

2. 非无暇模式

    | 判定等级 | 准度计算方式（Acc） | 判定区间 |
    | -------- | ------------------- | -------- |
    | S-Extra  | $100\%$             | ±50 ms   |
    | Extra    | $ 70\%$             | ±70 ms   |
    | Normal   | $50\%$              | ±120 ms  |
    | Fade     | $0\%$               | ±150 ms  |
    | Miss     | $0\%$               | N/A      |

    计分方式：$(\frac{Acc \cdot 当前已经判定的 Note}{总 Note 数}\times0.95+\frac{最大连击数}{总 Note 数}\times 0.05 )\times 1000000$

### 支持的平台

理论上来说 Windows MacOS Linux （[Android 见此](https://github.com/5av1ng/adgk-shapoist)） 都可以运行，但是后两者需要自行编译。

## 关于开发

项目使用 Rust 编写，各个部分的开发进度见 Src 内 ReadMe 注解。

**警告**：Shapoist 的代码未经系统化整理且实现某些功能的方式比较基础（低情商：屎）你将看到包括但不限于：多层嵌套，复制粘贴式复用，无注释……

## 开源许可证

代码部分：MIT

默认资源：

* 图标部分：[xicons](xicons.org/) MIT/Apache 2
* 音频部分：CC-BY-SA
