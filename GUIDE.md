# pg2sqlx 技术指南

## 目录

- [pg2sqlx 技术指南](#pg2sqlx-技术指南)
  - [目录](#目录)
  - [简介](#简介)
  - [安装和设置](#安装和设置)
  - [工具使用](#工具使用)
    - [基本用法](#基本用法)
    - [指定输出文件](#指定输出文件)
    - [命令行选项](#命令行选项)
  - [工作流程](#工作流程)
  - [配置文件](#配置文件)
    - [goctl.yaml 结构](#goctlyaml-结构)
    - [自定义类型映射](#自定义类型映射)
  - [最佳实践](#最佳实践)
  - [故障排除](#故障排除)

## 简介

pg2sqlx 是一个专门为 Go 开发者设计的工具，用于解决在使用 goctl 工具生成 PostgreSQL 数据库模型时遇到的自定义类型映射问题。该工具能够自动识别 PostgreSQL schema 文件中的 ENUM 和 VECTOR 类型，并生成相应的类型映射配置，使 goctl 能够正确生成包含这些自定义类型的模型代码。

## 安装和设置

1. 确保系统已安装 Rust 和 Cargo
2. 克隆项目代码库
3. 在项目根目录运行以下命令构建工具：

```bash
cargo install --path .
```

编译后的二进制文件位于 `~/.cargo/bin/pg2sqlx`。

## 工具使用

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

## 配置文件

### goctl.yaml 结构

goctl.yaml 文件包含模型生成的配置信息，其中最重要的部分是 `types_map`，它定义了数据库类型到 Go 类型的映射关系。

### 自定义类型映射

pg2sqlx 工具会自动为以下类型添加映射：

- ENUM 类型：映射为 Go `string` 类型，空值类型为 `sql.NullString`
- VECTOR 类型：映射为 Go `string` 类型，空值类型为 `sql.NullString`，并包含 pgvector 包

## 最佳实践

1. 在运行 pg2sqlx 之前，确保 PostgreSQL schema 文件是最新的
2. 在运行模型生成脚本之前，先运行 pg2sqlx 更新配置文件
3. 定期检查生成的模型代码，确保类型映射正确
4. 在团队中共享更新后的 goctl.yaml 配置文件，确保一致性
5. goctl.yaml 要放到运行`goctl model pg ...`命令目录下

## 故障排除

1. 如果遇到 "unsupported database type" 错误，请确保：
   - pg2sqlx 已正确运行并更新了配置文件
   - goctl.yaml 文件包含所有必要的类型映射
   - goctl 版本大于等于 1.6.5
   - 实验性功能已开启 (GOCTL_EXPERIMENTAL=on)
