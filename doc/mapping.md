# pg2sqlx 映射规则

## ENUM 类型映射规则

- `null_type`: `sql.NullString`
- `type`: `string`
- `pkg`: 无（Go 标准库类型）

## VECTOR 类型映射规则

- `null_type`: `pgvector.Vector`
- `type`: `pgvector.Vector`
- `pkg`: `github.com/pgvector/pgvector-go`

## 配置文件格式

goctl.yaml 配置文件包含一个 `types_map` 部分，用于定义自定义类型到 Go 类型的映射：

```yaml
model:
  types_map:
    # ENUM 类型示例
    status:
      null_type: sql.NullString
      type: string

    # VECTOR 类型
    vector:
      null_type: pgvector.Vector
      type: pgvector.Vector
      pkg: github.com/pgvector/pgvector-go
```
