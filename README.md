# ljnljn-ios-plugins

iOS 取证插件，全部使用 LovelyCarve **LPP v2**（protocol 2）与类型化记录。
统一 git tag 与 GitHub topic：`lovelycarve-plugins`。

| 插件 | LPP | 能力 |
| --- | --- | --- |
| `ljnljn-ios-native-artifacts` | v2 | 原生读取设备/备份信息、短信/iMessage、通话、联系人、Safari 历史、WhatsApp、照片元数据与日历 |

`ljnljn-ios-native-artifacts` 直接打开完整文件系统提取物中的数据库和
plist，不运行 iLEAPP；数据库先物化到私有临时目录，连同 `-wal`/`-shm`
边车只读打开，检材不会被写入。共享解析层在仓库根目录的 `mobile-core/`。

尚未原生覆盖的 iLEAPP 模块仍可用 `ljnljn-ileapp-aleapp-report` 导入它的
`_TSV Exports` 报告，后续按批次迁移为原生解析。

每个插件是独立 Cargo 工程，仓库根目录提供共享 SDK v2：

```bash
cd plugins/<插件目录>
cargo test --locked
cargo build --release --locked
```

Release 提供各插件 `.lcpkg`、完整平台 zip、`INDEX.json` 和
`SHA256SUMS.txt`。插件只读检材，不执行检材内容，不访问网络；结果需要
取证员复核。源码采用 AGPL-3.0-only。
