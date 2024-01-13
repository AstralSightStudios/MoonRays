# 导入os模块，用于操作文件系统
import os

# 定义一个函数，接受一个目录作为参数，返回该目录下所有.rs文件的行数总和
def count_rs_lines(dir):
    # 初始化行数为0
    lines = 0
    # 遍历目录下的所有文件和子文件夹
    for entry in os.scandir(dir):
        # 如果是文件，且文件名以.rs结尾
        if entry.is_file() and entry.name.endswith(".rs"):
            # 打开文件
            with open(entry.path, encoding="utf-8") as f:
                # 用sum函数和生成器表达式计算文件的行数
                file_lines = sum(1 for line in f)
                # 把文件的行数加到总行数上
                lines += file_lines
                # 打印文件名和文件的行数
                print(f"文件 {entry.name}: {file_lines} 行")
        # 如果是文件夹，递归调用函数
        elif (entry.is_dir() and entry.name != "target"):
            lines += count_rs_lines(entry.path)
    # 返回行数
    return lines

# 调用函数，传入当前目录
total_lines = count_rs_lines(".")
# 打印结果
print(f"项目的总Rust代码行数为{total_lines}行")