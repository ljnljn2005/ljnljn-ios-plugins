# ljnljn-ios-native-artifacts

LovelyCarve **LPP v2** 插件：直接读取 iOS 完整文件系统提取物中的核心工件，
不运行 iLEAPP，也不依赖它的 TSV 报告。

- 插件 ID：`org.ljnljn.ios-native-artifacts`
- 记录：`mobile.artifact`、`mobile.artifact.file`
- 权限：`file_materialize`（只读物化数据库副本）

## 覆盖范围

| 分析器 | 数据源 | 核心字段 |
| --- | --- | --- |
| `ios_device_info` | `Manifest.plist`、`SystemVersion.plist` | 设备名、产品版本、备份时间 |
| `ios_messages` | `sms.db` | 收发方向、号码、正文、服务、时间、已读/送达时间 |
| `ios_calls` | `CallHistory.storedata`、`CallHistoryTemp.storedata`、`call_history.db` | 号码、方向、时长、服务、时间 |
| `ios_contacts` | `AddressBook.sqlitedb` | 姓名、组织、职位、电话、邮箱 |
| `ios_safari_history` | `Safari/History.db` | URL、标题、访问次数、访问时间 |
| `ios_whatsapp` | `ChatStorage.sqlite` | 聊天名、JID、方向、正文、媒体、时间 |
| `ios_photos` | `PhotoData/Photos.sqlite` | 文件名、类型、尺寸、经纬度、创建时间 |
| `ios_calendar` | `Calendar.sqlitedb` | 标题、开始/结束、地点、日历、备注 |

另有 `ios_selected_databases`（手动选数据库）和 `ios_selected_plist`
（手动选 plist）。

## 读取方式

- 每个数据库先复制到私有临时目录，连同存在的 `-wal` / `-shm` / `-journal`
  边车一起打开；检材本身不被写入。
- 只执行插件内固定的只读 SQL，不执行检材里的 SQL 文本，不联网。
- SQLite 文件头、表是否存在、列是否存在都会先校验；不同 iOS 版本通过
  固定查询变体回退，例如 `ZASSET` 与 `ZGENERICASSET`、新旧短信列。
- 时间按存储格式归一化：Cocoa/Core Data 时间、Safari 的 Cocoa/Unix 混合
  时间分别处理；原始值仍保留在 `data` 中。
- 单个数据库上限 1 GiB、单个工件最多 100,000 行、单个文本单元最多 64 KiB；
  超出会在文件记录中标记 `truncated`，不静默丢弃。

## 已知边界

- 当前只覆盖**完整文件系统提取**；iTunes/Finder 备份除 `Manifest.plist`
  的设备信息外，尚未做哈希文件映射，加密备份中的数据库也无法直接读取。
- `sms.db` 的 `attributedBody` 二进制正文暂不解码；正文列为空时会保留
  `<blob N bytes>` 标记，需回看原始库。
- 不解析附件文件内容、不恢复删除记录、不合并 iCloud 同步数据。
- WhatsApp 只读 `ChatStorage.sqlite`，不读 `ContactsV2.sqlite` 的通讯录名；
  媒体只记录引用路径，不导出文件。
- Photos 只读资产表元数据，不做人脸、地点反查或相册关系展开。
- 不处理加密数据库；遇到随机文件头会告警并跳过。
- 其它 iLEAPP 模块仍可用 `ljnljn-ileapp-aleapp-report` 导入报告，后续按批次
  迁移为原生解析。

## 构建

```bash
cargo test --locked
cargo build --release --locked
```

需要 LovelyCarve v2 宿主（protocol 2）。Release 中的 `.lcpkg` 可直接安装。

## 许可证

AGPL-3.0-only。解析规则参考 iLEAPP（MIT）的公开实现，未复制其 Python 代码。
