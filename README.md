# 端到端加密,安全文件共享程序 (End-to-End Encryption Secure File Sharing App)

## 使用技术

- aes & block-modes: 一个高级加密标准的加密库.
- anyhow: 一个基于 std::error:Error 构建的灵活具体的错误类型库.
- argon2: 一个安全的密码哈希库.
- axum: 一个轻量级且符合人体工程学的 Web 框架，用于在 Rust 中构建 API.
- axum-extra: Axum 的附加实用程序，包括 cookie 支持.
- base64: Base64 编码和解码库，常用于文件和加密操作.
- dotenv: 用于管理开发中的环境变量.
- jsonwebtoken: 用于对 JSON Web Tokens (JWT) 进行编码和解码以进行身份 ​​ 验证的库.
- rand: 用于生成随机值的库，用于加密和令牌生成.
- rsa: RSA 加密和解密的库.
- serde & serde_json: 用于高效序列化和反序列化 Rust 数据结构的框架，与 JSON 数据一起使用.
- sqlx: 一个异步的、编译时验证的 SQL 包，支持多个数据库.
- tokio: Rust 的异步运行时，为非阻塞操作提供支持.
- tokio-cron-scheduler: 一个基于类似 cron 的表达式定期运行任务的调度程序库.
- tower & tower-http: 用于构建强大 HTTP 服务的中间件和实用程序，包括 CORS 和跟踪支持.
- tracing & tracing-subscriber: Rust 应用程序的日志库，提供结构化日志记录.
- uuid: 用于生成和解析通用唯一标识符（UUID）的库.
- validator: Rust 中用于输入验证的验证框架.

## 环境设置

### 安装 Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 安装 VSCode 插件

- crates: Rust 包管理
- Even Better TOML: TOML 文件支持
- Better Comments: 优化注释显示
- Error Lens: 错误提示优化
- GitLens: Git 增强
- Github Copilot: 代码提示
- indent-rainbow: 缩进显示优化
- Prettier - Code formatter: 代码格式化
- REST client: REST API 调试
- rust-analyzer: Rust 语言支持
- Rust Test lens: Rust 测试支持
- Rust Test Explorer: Rust 测试概览
- TODO Highlight: TODO 高亮
- vscode-icons: 图标优化
- YAML: YAML 文件支持

### 安装 cargo generate

cargo generate 是一个用于生成项目模板的工具。它可以使用已有的 github repo 作为模版生成新的项目。

```bash
cargo install cargo-generate
```

```bash
cargo generate TeamMeng/template
```

### 安装 pre-commit

pre-commit 是一个代码检查工具，可以在提交代码前进行代码检查。

```bash
pipx install pre-commit
```

安装成功后运行 `pre-commit install` 即可。

### 安装 Cargo deny

Cargo deny 是一个 Cargo 插件，可以用于检查依赖的安全性。

```bash
cargo install --locked cargo-deny
```

### 安装 typos

typos 是一个拼写检查工具。

```bash
cargo install typos-cli
```

### 安装 git cliff

git cliff 是一个生成 changelog 的工具。

```bash
cargo install git-cliff
```

### 安装 cargo nextest

cargo nextest 是一个 Rust 增强测试工具。

```bash
cargo install cargo-nextest --locked
```
