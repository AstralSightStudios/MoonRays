# MoonRays
A Next-Gen game engine powered by vulkan
# ⚠️ In-Development Project WARNING ⚠️
## The project is still in the development stage, which means it cannot be compiled and used normally, and it is still far from the target effect. We are currently focusing on 2D rendering and our goal is to develop a full 2D game using this engine, so stay tuned!
## 编译与开发
需要`nightly`版本的rust toolchain
安装：
```bash
rustup toolchain install nightly
```
## 写死的规范
1. 着色器必须为`GLSL`格式且入口必须叫做`main`否则无法编译
## 对于Apple平台
目前这个任务还是交给社区吧
## 常见编译报错修复
1. 报错`Unable to find libclang: "couldn't find any valid shared libraries matching: ['clang.dll', 'libclang.dll'], set the "LIBCLANG_PATH" environment variable to a path where one of these files can be found (invalid: [])"`

 - 解决：没安装LLVM导致的。前往`https://releases.llvm.org/`下载它，并在安装时勾选`Add LLVM to the system PATH for all users`，然后重启开发环境再试