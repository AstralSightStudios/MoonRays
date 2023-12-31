# MoonRays
A Next-Gen game engine powered by vulkan
## 编译与开发
需要`nightly`版本的rust toolchain
安装：
```bash
rustup toolchain install nightly
```
## 常见编译报错修复
1. 报错`Unable to find libclang: "couldn't find any valid shared libraries matching: ['clang.dll', 'libclang.dll'], set the "LIBCLANG_PATH" environment variable to a path where one of these files can be found (invalid: [])"`

 - 解决：没安装LLVM导致的。前往`https://releases.llvm.org/`下载它，并在安装时勾选`Add LLVM to the system PATH for all users`，然后重启开发环境再试