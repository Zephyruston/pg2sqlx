# pg2sqlx - PostgreSQL to Go Type Mapper

## 目录

- [pg2sqlx - PostgreSQL to Go Type Mapper](#pg2sqlx---postgresql-to-go-type-mapper)
  - [目录](#目录)
  - [简介](#简介)
  - [功能特性](#功能特性)
  - [安装](#安装)
  - [快速开始](#快速开始)
  - [使用方法](#使用方法)
    - [基本用法](#基本用法)
    - [指定输出文件](#指定输出文件)
    - [命令行选项](#命令行选项)
  - [工作流程](#工作流程)
  - [类型映射](#类型映射)
    - [ENUM 类型](#enum-类型)
    - [VECTOR 类型](#vector-类型)
  - [性能](#性能)
  - [兼容性](#兼容性)
  - [故障排除](#故障排除)
  - [贡献](#贡献)
  - [许可证](#许可证)

## 简介

pg2sqlx 是一个专门为 Go 开发者设计的工具，用于解决在使用 goctl 工具生成 PostgreSQL 数据库模型时遇到的自定义类型映射问题。该工具能够自动识别 PostgreSQL schema 文件中的 ENUM 和 VECTOR 类型，并生成相应的类型映射配置，使 goctl 能够正确生成包含这些自定义类型的模型代码。

## 功能特性

- 解析 PostgreSQL schema 文件以识别自定义 ENUM 类型
- 处理 pgvector 的 VECTOR 类型
- 自动生成 goctl.yaml 配置文件中的类型映射
- 保留现有类型映射，只添加新的映射
- 生成配置文件备份以防止配置损坏
- 快速执行（通常在 5 秒内完成）
- 支持单行和多行 ENUM 定义

## 安装

要安装 pg2sqlx，您需要在系统上安装 Rust 和 Cargo，然后运行：

```bash
cargo install --path .
```

编译后的二进制文件将位于 `~/.cargo/bin/pg2sqlx`。

## 快速开始

1. 构建项目：

   ```bash
   cargo build --release
   ```

2. 运行工具处理 schema 文件：

   ```bash
   ./target/release/pg2sqlx --schema-file=001_init_schema.sql --config=goctl.yaml
   ```

3. 生成模型：
   ```bash
   cd script && ./genAll.sh
   ```

## 使用方法

### 基本用法

```bash
pg2sqlx --schema-file=001_init_schema.sql --config=goctl.yaml
```

此命令将解析 schema 文件并就地更新 goctl.yaml 配置。

### 指定输出文件

```bash
pg2sqlx --schema-file=001_init_schema.sql --config=goctl.yaml --output=goctl_updated.yaml
```

此命令将解析 schema 文件并将更新后的配置保存到新文件中，保留原始文件不变。

### 命令行选项

- `--schema-file` 或 `-s`：PostgreSQL schema 文件路径（必需）
- `--config` 或 `-c`：goctl.yaml 配置文件路径（必需）
- `--output` 或 `-o`：更新后配置文件的输出路径（可选）
- `--verbose` 或 `-v`：启用详细输出以进行调试（可选）

## 工作流程

pg2sqlx 的工作流程包括以下几个步骤：

1. 解析 PostgreSQL schema 文件，识别自定义 ENUM 和 VECTOR 类型
2. 读取现有的 goctl.yaml 配置文件
3. 将识别到的自定义类型添加到配置文件的类型映射中
4. 保存更新后的配置文件
5. 使用更新后的配置文件生成模型代码

## 类型映射

### ENUM 类型

ENUM 类型被映射为 Go 的 `string` 类型，空值类型为 `sql.NullString`：

```yaml
model:
  types_map:
    emotion_type:
      null_type: sql.NullString
      type: string
```

### VECTOR 类型

VECTOR 类型被映射为 Go 的 `string` 类型，空值类型为 `sql.NullString`，并包含 pgvector 包：

```yaml
model:
  types_map:
    vector:
      null_type: sql.NullString
      type: string
      pkg: github.com/pgvector/pgvector-go
```

## 性能

该工具设计为快速轻量：

- Schema 解析通常在 1ms 内完成
- 总执行时间通常在 5ms 以下
- 内存使用量极少

## 兼容性

- 适用于 goctl 1.6.5 及以上版本
- 兼容 PostgreSQL 10+
- 可在 Linux、macOS 和 Windows 上运行

## 故障排除

1. 如果遇到 "unsupported database type" 错误，请确保：

   - pg2sqlx 已正确运行并更新了配置文件
   - goctl.yaml 文件包含所有必要的类型映射
   - goctl 版本大于等于 1.6.5
   - 实验性功能已开启 (GOCTL_EXPERIMENTAL=on)

2. 如果工具运行时间过长，请使用 timeout 命令限制执行时间：
   ```bash
   timeout 10s pg2sqlx --schema-file=001_init_schema.sql --config=goctl.yaml
   ```

## 贡献

欢迎贡献代码、报告问题或提出新功能建议。请提交 Pull Request 或创建 Issue。

## 许可证

本项目采用 MIT 许可证。详情请见 [LICENSE](LICENSE) 文件。
