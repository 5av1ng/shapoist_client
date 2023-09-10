# Source Code

### 目录架构

```
SRC
├─error 				关于可能的错误的枚举
├─language				国际化方法
├─log					日志输出
├─play					关于游玩的区域
├─setting				设置
├─system				I/O 操作
└─ui					渲染部分
    ├─component				组件
    └─shape					图形
```

### 已知 BUG

* Android 排版会出现问题
* Android 无法读取外部文件
* Android 无法输入
* Android 从后台返回会使顶端栏显示
* 重载主界面导致 ID 冲突
* 判定系统一坨屎

### TODO(因个人血液原因未完全写入)

* 改善制谱器使用体验
* 加入逻辑块（处理包括 点击、陀螺仪、碰撞等）
* 加入 3D 支持
* 改进 error 显示（为其加入多语言支持等）
* 改进 log （格式化输出等）
* 构建一门解释性语言便于玩家拓展
* 性能优化
* 代码整理
* 制作页面自定义工具箱
