# Danki ORM

基于 Sea-ORM 实现的数据访问层，为论坛系统提供数据持久化支持。

## 特性

- 使用 Sea-ORM 作为底层 ORM 框架
- 所有 ID 使用 UUID v7 格式
- 支持 SQLite 和 PostgreSQL 数据库
- 提供完整的实体模型和存储库模式
- 包含用户、帖子和评论的数据模型

## 实体模型

- **User**: 用户实体，包含用户基本信息
- **Post**: 帖子实体，用户发布的内容
- **Comment**: 评论实体，支持嵌套回复

## 使用示例

```sh
wrangler d1 migrations create TTDB 'schema'
wrangler d1 export "TTDB" --output ".wrangler/online.sql" --remote
```

## 测试

运行测试：

```bash
cargo test
```

## 许可证

MPL-2.0
