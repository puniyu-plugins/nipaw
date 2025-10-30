# 变更日志

## [0.4.0](https://github.com/puniyu-plugins/nipaw/compare/core-v0.3.4...core-v0.4.0) (2025-10-30)


### ✨ 新功能

* **client:** 添加获取用户头像URL功能 ([e02321d](https://github.com/puniyu-plugins/nipaw/commit/e02321d7eee5e225fb4e235148643031496f1b11))
* **core:** 支持通过token控制获取仓库默认分支的方式 ([6c729de](https://github.com/puniyu-plugins/nipaw/commit/6c729dec53f0d6e29263e22344c67c88721b517d))
* **core:** 添加仓库信息获取功能 ([c0f1114](https://github.com/puniyu-plugins/nipaw/commit/c0f1114af7764e6a7e1362edbafe04721119639b))
* **core:** 添加仓库协作者功能支持 ([ce87b1c](https://github.com/puniyu-plugins/nipaw/commit/ce87b1cebeb6319096718353082759ca1f0d897b))
* **core:** 添加用户贡献数据和仓库列表功能 ([ebc8947](https://github.com/puniyu-plugins/nipaw/commit/ebc894715d67d6a14c3385ccbe6c786f48c080bd))
* **core:** 添加组织信息相关功能支持 ([47730ab](https://github.com/puniyu-plugins/nipaw/commit/47730ab307762f4a63bd3dd6b4007684891df351))
* **core:** 添加获取仓库提交信息功能 ([e54aca3](https://github.com/puniyu-plugins/nipaw/commit/e54aca38e6f5b68a34f0729e4f1052cc31d50f6e))
* **core:** 添加获取仓库提交列表功能 ([0bc8a0a](https://github.com/puniyu-plugins/nipaw/commit/0bc8a0a8ae385cf7d53a2e40c8990f5c89262aac))
* **core:** 添加获取仓库默认分支功能 ([8dead32](https://github.com/puniyu-plugins/nipaw/commit/8dead321fe0aae917d08ea61fa64a3d64c2c56e3))
* **gitcode:** 添加获取用户头像URL功能 ([cae522d](https://github.com/puniyu-plugins/nipaw/commit/cae522d36232bd45f9bcd22a3a774c4383e760d5))
* **nipaw_node:** 初始化 Node.js 绑定模块 ([0917a1d](https://github.com/puniyu-plugins/nipaw/commit/0917a1d1623e6bca98f78da00546806f21a9d113))
* **repo:** 重构仓库信息结构并优化默认分支获取逻辑 ([2850050](https://github.com/puniyu-plugins/nipaw/commit/28500500c653ec15103b1442270941a59e243af8))


### 🔧 其他更新

* release main ([01f9d1d](https://github.com/puniyu-plugins/nipaw/commit/01f9d1dc7cc91edd7eec22d4989dcb2d84bcebf2))
* release main ([889df11](https://github.com/puniyu-plugins/nipaw/commit/889df11713dce70094ab9715f772866516ed1277))
* release main ([cf4843c](https://github.com/puniyu-plugins/nipaw/commit/cf4843cef525bd11cbf52967b0a15741d8a9e726))
* **release:** 配置 release-please 和发布工作流 ([5b2700c](https://github.com/puniyu-plugins/nipaw/commit/5b2700c2155645a6fd5625c9514e3bb89b484307))
* **release:** 配置 release-please 支持 Rust 项目 ([69873f0](https://github.com/puniyu-plugins/nipaw/commit/69873f0ddc696958d6b4905611fcf155c0feeea8))
* **user:** 将用户昵称字段改为可选 ([8c9fac1](https://github.com/puniyu-plugins/nipaw/commit/8c9fac1aa0f47e825b8665ed4f0bb69c84a2b201))
* **workflows:** 调整构建和发布工作流配置 ([8d6b6d7](https://github.com/puniyu-plugins/nipaw/commit/8d6b6d7fc9994bbd832afd9ee010b88513c1e5e8))


### ♻️ 代码重构

* **core:** 简化Result类型使用并优化模块导出 ([aad14e1](https://github.com/puniyu-plugins/nipaw/commit/aad14e1f9a0c21e413bc2d457f4c55f507ec1b68))
* **core:** 统一数据转换逻辑并优化依赖管理 ([8765bf8](https://github.com/puniyu-plugins/nipaw/commit/8765bf8e6b483ee10ab723efb01e7476cccc1ff4))
* **core:** 重命名 CoreError为 Error 并更新相关引用 ([0d2f8c4](https://github.com/puniyu-plugins/nipaw/commit/0d2f8c44e654f0f2640929d20b98dbb85c8b7b60))

## [0.3.0](https://github.com/puniyu-plugins/nipaw/compare/v0.2.1...v0.3.0) (2025-10-01)


### ✨ 新功能

* **client:** 添加获取用户头像URL功能 ([e02321d](https://github.com/puniyu-plugins/nipaw/commit/e02321d7eee5e225fb4e235148643031496f1b11))
* **core:** 支持通过token控制获取仓库默认分支的方式 ([6c729de](https://github.com/puniyu-plugins/nipaw/commit/6c729dec53f0d6e29263e22344c67c88721b517d))
* **core:** 添加仓库信息获取功能 ([c0f1114](https://github.com/puniyu-plugins/nipaw/commit/c0f1114af7764e6a7e1362edbafe04721119639b))
* **core:** 添加用户贡献数据和仓库列表功能 ([ebc8947](https://github.com/puniyu-plugins/nipaw/commit/ebc894715d67d6a14c3385ccbe6c786f48c080bd))
* **core:** 添加获取仓库提交信息功能 ([e54aca3](https://github.com/puniyu-plugins/nipaw/commit/e54aca38e6f5b68a34f0729e4f1052cc31d50f6e))
* **core:** 添加获取仓库默认分支功能 ([8dead32](https://github.com/puniyu-plugins/nipaw/commit/8dead321fe0aae917d08ea61fa64a3d64c2c56e3))
* **gitcode:** 添加获取用户头像URL功能 ([cae522d](https://github.com/puniyu-plugins/nipaw/commit/cae522d36232bd45f9bcd22a3a774c4383e760d5))
* **nipaw_node:** 初始化 Node.js 绑定模块 ([0917a1d](https://github.com/puniyu-plugins/nipaw/commit/0917a1d1623e6bca98f78da00546806f21a9d113))


### 🔧 其他更新

* release main ([889df11](https://github.com/puniyu-plugins/nipaw/commit/889df11713dce70094ab9715f772866516ed1277))
* release main ([cf4843c](https://github.com/puniyu-plugins/nipaw/commit/cf4843cef525bd11cbf52967b0a15741d8a9e726))
* **release:** 配置 release-please 和发布工作流 ([5b2700c](https://github.com/puniyu-plugins/nipaw/commit/5b2700c2155645a6fd5625c9514e3bb89b484307))
* **workflows:** 调整构建和发布工作流配置 ([8d6b6d7](https://github.com/puniyu-plugins/nipaw/commit/8d6b6d7fc9994bbd832afd9ee010b88513c1e5e8))
* 初始化仓库 ([ba51774](https://github.com/puniyu-plugins/nipaw/commit/ba517747af1ca817786475db2bf15ad753d91000))


### ♻️ 代码重构

* **core:** 统一数据转换逻辑并优化依赖管理 ([8765bf8](https://github.com/puniyu-plugins/nipaw/commit/8765bf8e6b483ee10ab723efb01e7476cccc1ff4))

## [0.2.1](https://github.com/puniyu-plugins/nipaw/compare/v0.2.0...v0.2.1) (2025-10-01)


### 🔧 其他更新

* **workflows:** 调整构建和发布工作流配置 ([8d6b6d7](https://github.com/puniyu-plugins/nipaw/commit/8d6b6d7fc9994bbd832afd9ee010b88513c1e5e8))

## [0.2.0](https://github.com/puniyu-plugins/nipaw/compare/v0.1.0...v0.2.0) (2025-10-01)


### ✨ 新功能

* **client:** 添加获取用户头像URL功能 ([e02321d](https://github.com/puniyu-plugins/nipaw/commit/e02321d7eee5e225fb4e235148643031496f1b11))
* **core:** 支持通过token控制获取仓库默认分支的方式 ([6c729de](https://github.com/puniyu-plugins/nipaw/commit/6c729dec53f0d6e29263e22344c67c88721b517d))
* **core:** 添加仓库信息获取功能 ([c0f1114](https://github.com/puniyu-plugins/nipaw/commit/c0f1114af7764e6a7e1362edbafe04721119639b))
* **core:** 添加用户贡献数据和仓库列表功能 ([ebc8947](https://github.com/puniyu-plugins/nipaw/commit/ebc894715d67d6a14c3385ccbe6c786f48c080bd))
* **core:** 添加获取仓库提交信息功能 ([e54aca3](https://github.com/puniyu-plugins/nipaw/commit/e54aca38e6f5b68a34f0729e4f1052cc31d50f6e))
* **core:** 添加获取仓库默认分支功能 ([8dead32](https://github.com/puniyu-plugins/nipaw/commit/8dead321fe0aae917d08ea61fa64a3d64c2c56e3))
* **gitcode:** 添加获取用户头像URL功能 ([cae522d](https://github.com/puniyu-plugins/nipaw/commit/cae522d36232bd45f9bcd22a3a774c4383e760d5))
* **nipaw_node:** 初始化 Node.js 绑定模块 ([0917a1d](https://github.com/puniyu-plugins/nipaw/commit/0917a1d1623e6bca98f78da00546806f21a9d113))


### 🔧 其他更新

* **release:** 配置 release-please 和发布工作流 ([5b2700c](https://github.com/puniyu-plugins/nipaw/commit/5b2700c2155645a6fd5625c9514e3bb89b484307))
* 初始化仓库 ([ba51774](https://github.com/puniyu-plugins/nipaw/commit/ba517747af1ca817786475db2bf15ad753d91000))


### ♻️ 代码重构

* **core:** 统一数据转换逻辑并优化依赖管理 ([8765bf8](https://github.com/puniyu-plugins/nipaw/commit/8765bf8e6b483ee10ab723efb01e7476cccc1ff4))
