# 大肥鱼文档定制版

基于 Quiet.md 0.1.9 的独立本机编辑器。保留上游编辑、Markdown 外部文件保存保护、搜索、收藏、目录、回收站和版本能力。

## 身份与文件

- 安装路径：`/Applications/大肥鱼文档.app`
- Bundle ID：`com.followhongyun.dafeiyu.documents`
- 原版 Quiet.md 与其 `~/Documents/Quiet.md` 文档库保持独立。
- 编辑器索引：`~/Documents/大肥鱼文档/.editor`。
- DSH 导出的重要文档仍原地编辑，`.sync/catalog.json` 只包含名字和本机路径，不存凭据。
- `fish-folders.json` 保存初始化分类身份。用户移动或重命名后不会被启动过程重新归类。
- “重要文档”是大肥鱼的重要文档目录的视图，“最近编辑”和“收藏”复用同一条文档记录。
- 经历、创作和归档是可扩展的本机目录。新建本机文档不会自动升级为重要文档或上传云端。

## 同步

DSH 的既有文档桥继续同步 Markdown。双方同时修改时保留本机、远端和共同基线，停在冲突待选状态。编辑器自身也拒绝用旧缓冲区覆盖外部更新。DSH 退出时只保存本机，重开 DSH 后继续同步。冲突在 DSH 重要文档面板处理；编辑器“已保存”只代表保存到本机。

DSH 打开外部文档时同步目录中的全部重要文件并优先启动本定制版；未安装时回退原版 Quiet.md。不递归收录会话临时产物。

## 构建

`cargo build --release --locked`，然后 `scripts/package-fish.sh` 使用固定 Developer ID 签名。定制版阻止调用上游自动更新，避免被原版覆盖。原代码 MIT 归属保留；侧栏原商用图标已替换为已有 ISC/原创图标，应用图标为新绘制的鲸鱼。

仅本机安装，没有对外发布或公证声明。测试：`python3 scripts/check.py`；DSH：`node --test packages/dafeiyu/test/*.test.mjs`。
