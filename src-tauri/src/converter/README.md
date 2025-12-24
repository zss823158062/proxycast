# converter

<!-- 一旦我所属的文件夹有所变化，请更新我 -->

## 架构说明

协议转换模块，实现不同 LLM API 格式之间的转换。
支持 OpenAI、Claude、CodeWhisperer、Antigravity 等格式。

## 文件索引

- `mod.rs` - 模块入口
- `protocol_selector.rs` - 协议选择器
- `openai_to_cw.rs` - OpenAI → CodeWhisperer 转换
- `cw_to_openai.rs` - CodeWhisperer → OpenAI 转换
- `anthropic_to_openai.rs` - Anthropic → OpenAI 转换
- `openai_to_antigravity.rs` - OpenAI → Antigravity 转换

## 更新提醒

任何文件变更后，请更新此文档和相关的上级文档。
