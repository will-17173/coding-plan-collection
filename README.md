# AI Coding Plan 与 Token Plan 对比

本仓库收集整理了中国各大厂商提供的 AI 编程套餐（Coding Plan）和 Token 套餐（Token Plan）信息，方便开发者对比选择。

> 数据更新日期：2026年5月8日
>
> 完整对比网站：https://www.coding-plan.xyz

## 目录

- [Coding Plan（编程套餐）](#coding-plan编程套餐)
- [Token Plan（Token 套餐）](#token-plantoken-套餐)
- [Membership Plan（会员套餐）](#membership-plan会员套餐)
- [Agent Plan（智能体套餐）](#agent-plan智能体套餐)
- [国际 IDE Plan 对比](ide-plans.md)

---

## Coding Plan（编程套餐）

Coding Plan 通常以"请求次数"为计量单位，适合高频调用场景。

### 讯飞星火

| 套餐 | 价格 | 额度 | 支持模型 | 备注 |
|------|------|------|----------|------|
| 无忧版 | ¥3.90/月 | 请求次数不限 | Qwen3.5-35B-A3B, Qwen3-Coder-Next-FP8, GLM-4.7-Flash | |
| 专业版 | ¥39/月 | 1,200次/5小时，9,000次/周，18,000次/月 | Spark X2, GLM-5, MiniMax-M2.5, KIMI-K2.5, DeepSeek-V3.2, GLM-4.7-Flash, Qwen3.5-35B-A3B, Qwen3-Coder-Next-FP8, Qwen3.5-397B-A17B | |
| 高效版 | ¥199/月 | 6,000次/5小时，45,000次/周，90,000次/月 | Spark X2, GLM-5, GLM-5.1, MiniMax-M2.5, KIMI-K2.5, DeepSeek-V3.2, GLM-4.7-Flash, Qwen3.5-35B-A3B, Qwen3-Coder-Next-FP8, Qwen3.5-397B-A17B | |

🔗 [官方链接](https://maas.xfyun.cn/packageSubscription)

---

### 联通云

| 套餐 | 价格 | 额度 | 支持模型 | 备注 |
|------|------|------|----------|------|
| Coding Plan-Lite | ¥40/月 | 1,200次/5小时，9,000次/周，18,000次/月 | DeepSeek-V4-Flash, GLM-5.1, GLM-5, MiniMax-M2.5, Qwen3.5-397B-A17B, Qwen3-235B-A22B, Kimi-K2.5 | GLM-5.1 当前服务资源有限 |
| Coding Plan-Pro | ¥200/月 | 6,000次/5小时，45,000次/周，90,000次/月 | DeepSeek-V4-Flash, GLM-5.1, GLM-5, MiniMax-M2.5, Qwen3.5-397B-A17B, Qwen3-235B-A22B, Kimi-K2.5 | GLM-5.1 当前服务资源有限 |

🔗 [官方链接](https://www.cucloud.cn/Product/CodingPlan.html)

---

### 京东云

| 套餐 | 价格 | 额度 | 支持模型 | 备注 |
|------|------|------|----------|------|
| Lite | ¥40/月 | 1,200次/5小时，9,000次/周，18,000次/月 | DeepSeek-V3.2, GLM-5, GLM-4.7, MiniMax-M2.5, Kimi-K2.5, Kimi-K2-Turbo, Qwen3-Coder | 每天早上 10:30 抢购 |
| Pro | ¥200/月 | 6,000次/5小时，45,000次/周，90,000次/月 | DeepSeek-V3.2, GLM-5, GLM-4.7, MiniMax-M2.5, Kimi-K2.5, Kimi-K2-Turbo, Qwen3-Coder | 每天早上 10:30 抢购 |

🔗 [官方链接](https://www.jdcloud.com/cn/pages/codingplan)

---

### 百度千帆

| 套餐 | 价格 | 额度 | 支持模型 | 备注 |
|------|------|------|----------|------|
| Lite | ¥40/月 | 1,200次/5小时，9,000次/周，18,000次/月 | GLM-5, Kimi-K2.5, MiniMax-M2.5, DeepSeek-V3.2, ERNIE-4.5-Turbo | GLM-5 高峰期抵扣系数 3，低峰期 2 |
| Pro | ¥200/月 | 6,000次/5小时，9,000次/周，90,000次/月 | GLM-5, Kimi-K2.5, MiniMax-M2.5, DeepSeek-V3.2, ERNIE-4.5-Turbo | GLM-5 高峰期抵扣系数 3，低峰期 2 |

🔗 [官方链接](https://cloud.baidu.com/product/codingplan.html)

---

### 国家超算中心

| 套餐 | 价格 | 额度 | 支持模型 | 备注 |
|------|------|------|----------|------|
| Lite | ¥20/月 | 1,200次/5小时，9,000次/周，18,000次/月 | MiniMax-M2.5, Qwen3-235B-A22B | 模型太少 |
| Pro | ¥100/月 | 6,000次/5小时，45,000次/周，90,000次/月 | MiniMax-M2.5, Qwen3-235B-A22B | 模型太少 |

🔗 [官方链接](https://www.scnet.cn/ac/openapi/doc/2.0/moduleapi/codingplan/quickstart.html)

---

### 火山方舟

| 套餐 | 价格 | 额度 | 支持模型 | 备注 |
|------|------|------|----------|------|
| Lite | ¥40/月 | 1,200次/5小时，9,000次/周，18,000次/月 | Doubao-Seed-2.0-Code, Doubao-Seed-2.0-Pro, Doubao-Seed-2.0-Lite, Doubao-Seed-Code, MiniMax-M2.7, GLM-5.1, GLM-4.7, DeepSeek-V3.2, Kimi-K2.6, Kimi-K2.5 | 有倍率消耗机制，实际消耗量极快 |
| Pro | ¥200/月 | 6,000次/5小时，45,000次/周，90,000次/月 | Doubao-Seed-2.0-Code, Doubao-Seed-2.0-Pro, Doubao-Seed-2.0-Lite, Doubao-Seed-Code, MiniMax-M2.7, GLM-5.1, GLM-4.7, DeepSeek-V3.2, Kimi-K2.6, Kimi-K2.5 | 有倍率消耗机制，实际消耗量极快 |

🔗 [官方链接](https://www.volcengine.com/activity/codingplan-feishu)

---

### 智谱 GLM

| 套餐 | 价格 | 额度 | 支持模型 | 备注 |
|------|------|------|----------|------|
| Lite | ¥49/月（¥39/月包年） | 约 80 prompts/5小时，约 400 prompts/周 | GLM-5.1, GLM-5-Turbo, GLM-4.7, GLM-4.5-Air | GLM-5.1、GLM-5-Turbo 高峰期消耗系数 3 倍，非高峰期 2 倍 |
| Pro | ¥149/月（¥119/月包年） | 约 400 prompts/5小时，约 2,000 prompts/周 | GLM-5.1, GLM-5-Turbo, GLM-4.7, GLM-4.5-Air | 同上 |
| Max | ¥469/月（¥375/月包年） | 约 1,600 prompts/5小时，约 8,000 prompts/周 | GLM-5.1, GLM-5-Turbo, GLM-4.7, GLM-4.5-Air | 同上 |

🔗 [官方链接](https://www.bigmodel.cn/glm-coding)

---

### 无问芯穹

| 套餐 | 价格 | 额度 | 支持模型 | 备注 |
|------|------|------|----------|------|
| Lite | ¥40/月 | 1,000次/5小时，6,000次/周，12,000次/月 | DeepSeek-V3.2, DeepSeek-V3.2-Thinking, Kimi-K2.5, MiniMax-M2.1, MiniMax-M2.5, MiniMax-M2.7, GLM-4.7, GLM-5, GLM-5.1 | |
| Pro | ¥200/月 | 5,000次/5小时，30,000次/周，60,000次/月 | DeepSeek-V3.2, DeepSeek-V3.2-Thinking, Kimi-K2.5, MiniMax-M2.1, MiniMax-M2.5, MiniMax-M2.7, GLM-4.7, GLM-5, GLM-5.1 | |

🔗 [官方链接](https://cloud.infini-ai.com/platform/ai)

---

### 天翼云

| 套餐 | 价格 | 额度 | 支持模型 | 备注 |
|------|------|------|----------|------|
| GLM Lite | ¥49/月 | 80 prompts/5小时，400 prompts/周，1,600 prompts/月 | GLM-5-Turbo, GLM-5.1, GLM-4.5, GLM-4.5-Air, GLM-4.6, GLM-4.7 | |
| GLM Pro | ¥149/月 | 400 prompts/5小时，2,000 prompts/周，8,000 prompts/月 | 同上 | |
| GLM Max | ¥469/月 | 1,600 prompts/5小时，8,000 prompts/周，32,000 prompts/月 | 同上 | |

🔗 [官方链接](https://ctxirang.ctyun.cn/maas/codingPlan)

---

### 阶跃星辰

| 套餐 | 价格 | 额度 | 支持模型 | 备注 |
|------|------|------|----------|------|
| Flash Mini（入门版） | ¥49/月 | 100 prompts/5小时，400 prompts/周 | Step 3.5 Flash 2603, Step 3.5 Flash, StepAudio 2.5 TTS, StepAudio 2.5 ASR, Step Router V1, Step Image Edit 2 | |
| Flash Plus（进阶版） | ¥99/月 | 约 400 prompts/5小时，约 1,600 prompts/周 | 同上 | |
| Flash Pro（专业版） | ¥199/月 | 约 1,500 prompts/5小时，约 6,000 prompts/周 | 同上 | |
| Flash Max（旗舰版） | ¥699/月 | 约 5,000 prompts/5小时，约 2万 prompts/周 | 同上 | |

🔗 [官方链接](https://platform.stepfun.com/step-plan)

---

### 移动云

| 套餐 | 价格 | 额度 | 支持模型 | 备注 |
|------|------|------|----------|------|
| Lite | ¥40/月（首月¥7.9） | 1,200次/5小时，9,000次/周，18,000次/月 | MiniMax-M2.5 | 注意：无购买价值 |
| Pro | ¥200/月（首月¥39.9） | 6,000次/5小时，45,000次/周，90,000次/月 | MiniMax-M2.5 | 注意：无购买价值 |

🔗 [官方链接](https://ecloud.10086.cn/portal/act/codingplan)

---

### 摩尔线程

| 套餐 | 价格 | 额度 | 支持模型 | 备注 |
|------|------|------|----------|------|
| Lite Plan | ¥120/季度（约¥40/月） | 约 120 prompts/5小时 | GLM-4.7 | Claude Pro 套餐的 3 倍用量 |
| Pro Plan | ¥600/季度（约¥200/月） | 约 600 prompts/5小时 | GLM-4.7 | Lite Plan 的 5 倍用量 |
| Max Plan | ¥1200/季度（约¥400/月） | 约 2,400 prompts/5小时 | GLM-4.7 | Pro Plan 的 4 倍用量 |

🔗 [官方链接](https://code.mthreads.com/)

---

### ZenMux

| 套餐 | 价格 | 额度 | 支持模型 | 备注 |
|------|------|------|----------|------|
| Pro | $20/月（约¥144） | 50 Flows/5小时，213 Flows/周，914 Flows/月 | 100+ 模型 | |
| Max | $100/月（约¥720） | 300 Flows/5小时，1,280 Flows/周，5,486 Flows/月 | 100+ 模型 | |
| Ultra | $200/月（约¥1440） | 800 Flows/5小时，3,413 Flows/周，14,631 Flows/月 | 100+ 模型 | |

🔗 [官方链接](https://zenmux.ai/) | [模型列表](https://zenmux.ai/models)

---

### OpenCode

| 套餐 | 价格 | 额度 | 支持模型 | 备注 |
|------|------|------|----------|------|
| Go | $10/月（首月$5） | $12/5小时, $30/周, $60/月 | GLM-5, GLM-5.1, Kimi-K2.5, Kimi-K2.6, MiMo-V2-Pro, MiMo-V2-Omni, MiMo-V2.5-Pro, MiMo-V2.5, MiniMax-M2.5, MiniMax-M2.7, Qwen3.5 Plus, Qwen3.6 Plus, DeepSeek V4 Pro, DeepSeek V4 Flash | |

🔗 [官方链接](https://opencode.ai/zh/go)

---

## Token Plan（Token 套餐）

Token Plan 以 Token 数量为计量单位，适合需要精细控制使用量的场景。

### 联通云

| 套餐 | 价格 | 额度 | 支持模型 |
|------|------|------|----------|
| Token Plan-Lite（个人版） | ¥15/月 | 600万 Tokens | DeepSeek-V4-Flash, MiniMax-M2.5 |
| Token Plan-Pro（个人版） | ¥30/月 | 1,200万 Tokens | DeepSeek-V4-Flash, MiniMax-M2.5 |
| Token Plan-Max（个人版） | ¥45/月 | 1,800万 Tokens | DeepSeek-V4-Flash, MiniMax-M2.5 |
| Token Plan-Lite（团队版） | ¥198/月 | 25,000 Credits | DeepSeek-V4-Pro, DeepSeek-V4-Flash, MiniMax-M2.5 |
| Token Plan-Pro（团队版） | ¥698/月 | 100,000 Credits | DeepSeek-V4-Pro, DeepSeek-V4-Flash, MiniMax-M2.5 |
| Token Plan-Max（团队版） | ¥1,398/月 | 250,000 Credits | DeepSeek-V4-Pro, DeepSeek-V4-Flash, MiniMax-M2.5 |

🔗 [官方链接](https://www.cucloud.cn/product/tokenplan.html)

---

### 腾讯云

| 套餐 | 价格 | 额度 | 支持模型 |
|------|------|------|----------|
| Lite | ¥39/月 | 3,500万 Tokens | MiniMax-M2.5, MiniMax-M2.7, GLM-5, GLM-5.1, Kimi-K2.5, Hunyuan 2.0 Instruct, Hunyuan 2.0 Think, Hunyuan-T1, Hunyuan-TurboS |
| Standard | ¥99/月 | 1亿 Tokens | 同上 |
| Pro | ¥299/月 | 3.2亿 Tokens | 同上 |
| Max | ¥599/月 | 6.5亿 Tokens | 同上 |

🔗 [官方链接](https://cloud.tencent.com/act/pro/tokenplan)

---

### 阿里云百炼

| 套餐 | 价格 | 额度 | 支持模型 |
|------|------|------|----------|
| 团队版 标准坐席 | ¥198/坐席/月 | 25,000 Credits | qwen3.6-plus, glm-5, MiniMax-M2.5, deepseek-v3.2, qwen-image-2.0, qwen-image-2.0-pro, wan2.7-image, wan2.7-image-pro |
| 团队版 高级坐席 | ¥698/坐席/月 | 100,000 Credits | 同上 |
| 团队版 尊享坐席 | ¥1,398/坐席/月 | 250,000 Credits | 同上 |
| 团队版 共享包 | ¥5,000 | 625,000 Credits（有效期1月） | 同上 |

🔗 [官方链接](https://common-buy.aliyun.com/token-plan)

---

### 小米 MiMo

| 套餐 | 价格 | 额度 | 支持模型 | 备注 |
|------|------|------|----------|------|
| Lite | ¥39/月 | 6,000万 Credits | MiMo-V2.5-Pro, MiMo-V2.5, MiMo-V2.5-TTS-VoiceClone, MiMo-V2.5-TTS-VoiceDesign, MiMo-V2.5-TTS, V2 系列模型 | 非高峰期 0.8x 系数消耗，TTS 系列模型限时免费 |
| Standard | ¥99/月 | 2亿 Credits | 同上 | 同上 |
| Pro | ¥329/月 | 7亿 Credits | 同上 | 同上 |
| Max | ¥659/月 | 16亿 Credits | 同上 | 同上 |

🔗 [官方链接](https://platform.xiaomimimo.com)

---

### MiniMax

| 套餐 | 价格 | 额度 | 支持模型 | 备注 |
|------|------|------|----------|------|
| Starter | ¥29/月 | 600次/5小时 | MiniMax-M2.7, MiniMax-M2.5, Image-01 | 每周额度为 5 小时额度的 10 倍 |
| Plus | ¥49/月（¥41/月包年） | 1,500次/5小时 | MiniMax-M2.7, MiniMax-M2.5, Image-01, Speech-2.8-HD | 同上 |
| Max | ¥119/月（¥99/月包年） | 4,500次/5小时 | MiniMax-M2.7, MiniMax-M2.5, Image-01, Speech-2.8-HD, Hailuo-2.3, Music-2.6 | 同上 |
| Plus 极速 | ¥98/月（¥82/月包年） | 1,500次/5小时 | MiniMax-M2.7-Highspeed, MiniMax-M2.5-Highspeed | 同上 |
| Max 极速 | ¥199/月（¥166/月包年） | 4,500次/5小时 | MiniMax-M2.7-Highspeed, MiniMax-M2.5-Highspeed | 同上 |
| Ultra 极速 | ¥899/月（¥749/月包年） | 30,000次/5小时 | MiniMax-M2.7-Highspeed, MiniMax-M2.5-Highspeed | 已售罄 |

🔗 [官方链接](https://platform.minimaxi.com/subscribe/token-plan)

---

### 商汤科技

| 套餐 | 价格 | 额度 | 支持模型 | 备注 |
|------|------|------|----------|------|
| Free · 公测 | ¥0/月 | SenseNova 6.7 Flash-Lite 与 SenseNova U1 Fast：1,500次/5小时；DeepSeek V4 Flash：150次/5小时 | SenseNova 6.7 Flash-Lite, SenseNova U1 Fast, DeepSeek V4 Flash | 公测阶段 |

🔗 [官方链接](https://www.sensenova.cn/token-plan)

---

## Membership Plan（会员套餐）

### Kimi

| 套餐 | 价格 | 额度 | 支持模型 |
|------|------|------|----------|
| Andante | ¥49/月（¥39/月年付） | 1倍 Kimi Code 额度，约 30 个 Agent/月 | Kimi-K2.6, Kimi-K2.5, Kimi-K2 |
| Moderato | ¥99/月（¥79/月年付） | 4倍 Kimi Code 额度，约 60 个 Agent/月 | 同上 |
| Allegretto | ¥199/月（¥159/月年付） | 20倍 Kimi Code 额度，约 150 个 Agent/月 | 同上 |
| Allegro | ¥699/月（¥559/月年付） | 60倍 Kimi Code 额度，约 360 个 Agent/月 | 同上 |

🔗 [官方链接](https://www.kimi.com/code)

---

## Agent Plan（智能体套餐）

### 火山方舟

| 套餐 | 价格 | 额度 | 支持模型 |
|------|------|------|----------|
| Small | ¥40/月 | 20,000 AFP/月，7,000 AFP/周，2,000 AFP/5小时 | Doubao-Seed-2.0-Mini, Doubao-Seed-2.0-Lite, Doubao-Seed-2.0-Code, Doubao-Seed-2.0-Pro, DeepSeek-V3.2, MiniMax-M2.7, GLM-5.1, Kimi-K2.6, Doubao-Embedding-Vision, Doubao-Seedream-5.0-Lite |
| Medium | ¥200/月 | 100,000 AFP/月，35,000 AFP/周，10,000 AFP/5小时 | 同上 + Doubao-Seedance-1.5-Pro, Doubao-Seedance-2.0, Doubao-Seedance-2.0-Fast |
| Large | ¥500/月 | 250,000 AFP/月，87,500 AFP/周，25,000 AFP/5小时 | 同上 |
| Max | ¥1,000/月 | 500,000 AFP/月，175,000 AFP/周，50,000 AFP/5小时 | 同上 |

🔗 [官方链接](https://ai.volcengine.com/activity/agentplan)

---

## 数据来源

所有数据均来自各厂商官方页面，具体链接见各套餐表格。

如有数据错误或遗漏，欢迎提交 Issue 或 PR 修正。

---

## 相关链接

- [国际 IDE Plan 对比（Cursor、GitHub Copilot、Claude 等）](ide-plans.md)
- [完整对比网站](https://www.coding-plan.xyz)