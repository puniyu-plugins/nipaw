# Changelog

## [1.1.0](https://github.com/puniyu-plugins/nipaw/compare/node-v1.0.2...node-v1.1.0) (2025-10-02)


### ✨ 新功能

* **client:** 添加获取用户头像URL功能 ([e02321d](https://github.com/puniyu-plugins/nipaw/commit/e02321d7eee5e225fb4e235148643031496f1b11))
* **core:** 支持通过token控制获取仓库默认分支的方式 ([6c729de](https://github.com/puniyu-plugins/nipaw/commit/6c729dec53f0d6e29263e22344c67c88721b517d))
* **core:** 添加用户贡献数据和仓库列表功能 ([ebc8947](https://github.com/puniyu-plugins/nipaw/commit/ebc894715d67d6a14c3385ccbe6c786f48c080bd))
* **core:** 添加获取仓库提交信息功能 ([e54aca3](https://github.com/puniyu-plugins/nipaw/commit/e54aca38e6f5b68a34f0729e4f1052cc31d50f6e))
* **core:** 添加获取仓库默认分支功能 ([8dead32](https://github.com/puniyu-plugins/nipaw/commit/8dead321fe0aae917d08ea61fa64a3d64c2c56e3))
* **gitcode:** 添加获取用户头像URL功能 ([cae522d](https://github.com/puniyu-plugins/nipaw/commit/cae522d36232bd45f9bcd22a3a774c4383e760d5))
* **nipaw_node:** 初始化 Node.js 绑定模块 ([0917a1d](https://github.com/puniyu-plugins/nipaw/commit/0917a1d1623e6bca98f78da00546806f21a9d113))
* **platform:** 为多个平台客户端添加代理设置功能 ([ff14fdf](https://github.com/puniyu-plugins/nipaw/commit/ff14fdf6bc78549dcce956e4ee91744dc57a0b0e))


### 🐛 错误修复

* **node:** constructor错误 ([cb996b7](https://github.com/puniyu-plugins/nipaw/commit/cb996b70fb9f57c35e4d886ce39a4632d6def181))


### 🔧 其他更新

* **deps:** 升级 nipaw 系列 crate 至 0.3.0 版本 ([104274b](https://github.com/puniyu-plugins/nipaw/commit/104274b3b62b4e662e1376fcd35b5ec1fcd29e2d))
* **deps:** 更新 nipaw 相关包描述并调整发布流程 ([f064c21](https://github.com/puniyu-plugins/nipaw/commit/f064c211da35fb62f938725406ebe969c320e35b))
* **release:** 移除 nipaw_core 包的发布配置 ([fd0aa9e](https://github.com/puniyu-plugins/nipaw/commit/fd0aa9e595230b9011080736966a3864b53d8419))
* **release:** 配置 release-please 和发布工作流 ([5b2700c](https://github.com/puniyu-plugins/nipaw/commit/5b2700c2155645a6fd5625c9514e3bb89b484307))
* **workflows:** 调整构建和发布工作流配置 ([8d6b6d7](https://github.com/puniyu-plugins/nipaw/commit/8d6b6d7fc9994bbd832afd9ee010b88513c1e5e8))


### ♻️ 代码重构

* **core:** 统一数据转换逻辑并优化依赖管理 ([8765bf8](https://github.com/puniyu-plugins/nipaw/commit/8765bf8e6b483ee10ab723efb01e7476cccc1ff4))
* **platform:** 统一使用全局tokio运行时 ([0356528](https://github.com/puniyu-plugins/nipaw/commit/03565285d577c6210906691854fdf5be9a8ffd99))
* **platform:** 统一客户端获取方式 ([1926f74](https://github.com/puniyu-plugins/nipaw/commit/1926f747aadedaf960f7750306a736c0c48081df))
