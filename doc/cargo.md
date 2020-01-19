## 生成文档
```
cargo doc -p download_rs
```
或
```
cargo doc -p download_rs --features="full"
```
生成在 `target/doc` 目录

## 发布到 cargo
* 配置cargo api
```
cargo login cargo_api

```
* 验证并不发布
```
cargo publish --dry-run
```
* 发布
```
cargo publish
```