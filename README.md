# AI Coding Plan 与 Token Plan 对比

本仓库收集整理了中国各大厂商提供的 AI 编程套餐（Coding Plan）、Token 套餐（Token Plan）、会员套餐和智能体套餐信息，方便开发者对比选择。

> 数据更新日期：2026-07-21
>
> 当前收录：Coding Plan 51 个，Token Plan 51 个，Membership Plan 4 个，Agent Plan 4 个
>
> 完整对比网站：<https://www.coding-plan.xyz>

## 最新动态

**6 月 2 日** — [阿里正式发布多模态智能体模型 Qwen3.7-Plus，通过 API 开放商业调用](https://bailian.aliyun.com)

**6 月 1 日** — [MiniMax 发布 M3 大模型](https://www.minimaxi.com/models/text/m3)

**5 月 28 日** — [Anthropic 发布 Claude Opus 4.8](https://www.anthropic.com/news/claude-opus-4-8)

**5 月 27 日** — [永久降价最高99%并统一上下文计费，小米MiMo大模型全面升级计费与套餐体系](https://mimo.xiaomi.com)

**5 月 22 日** — [DeepSeek 发布 V4 Pro 永久 2.5 折优惠活动](https://deepseek.com)

**5 月 22 日** — [阿里云百炼 Token Plan 新增 Qwen3.7 Max 模型](https://bailian.aliyun.com)

---

## 核心选购建议

### 1. Token Plan 现状

Update 2026.5.27: 今天小米 MiMo API 降价了，Token Plan 也加上缓存的计费规则，如果觉得 MiMo 的模型能力满足需求，可以冲。
在 DeepSeek V4 Pro 目前优惠力度下，所有的 Token Plan 都不值得购买（MiniMax 除外，因为它是按次计费的）。使用量不多的话，直接用 DeepSeek API 按量计费更划算。

### 2. Coding 开发推荐

从模型能力维度，最建议的组合是：

- 一个官方 Claude/Codex 订阅
- 一个智谱（GLM）套餐

日常用 Claude/Codex 开发，按 Token 计量；GLM 用来做 Spec 执行，按 Prompt 计次，很划算。
GLM 的模型能力在国产模型里是遥遥领先的存在，不接受反驳。
之前本来是搭配 Github Copilot 更佳，但是 Copilot 在 6 月 1 号就要改计费方式了，所以不推荐了。

缺点：

- Claude/Codex 在国内使用有一定门槛
- 智谱的套餐不太好抢

### 3. 日常用途推荐

龙虾、聊天、文档撰写等日常用途，建议直接上 MiniMax：

- 不用抢购
- MiniMax 3 推出后，新的 Token Plan 改成按量计费，我个人觉得量给得还是可以，49 元的档位中度使用完全没有问题
- 模型能力, MiniMax 3 的模型能力较 2.7 有明显提升，能满足绝大多数日常需求，也能满足一些日常代码需求
- 包含图像/语音/视频生成

## 站长简评

原则上，像上面说的，按量的 Token Plan 都不建议购买了, Coding Plan 必须选有 GLM 5 以上模型的，尽量选算力充沛的大厂商。

| 厂商 - 套餐类型 | 评价 |
| --- | --- |
| 讯飞星火 - Coding Plan | 可以买，高峰期会限流，最高档位有 GLM-5.1 |
| 联通云 - Coding Plan | 不建议，限流严重 |
| 联通云 - Token Plan | 不要买 |
| 京东云 - Coding Plan | 可以买，限流较少, 有 GLM-5 |
| 腾讯云 - Token Plan | 不要买 |
| 百度千帆 - Coding Plan | 不建议，限流较严重 |
| 国家超算中心 - Coding Plan | 不要买，不如直接买 MiniMax 的 |
| 阿里云百炼 - Token Plan | 不要买 |
| 小米 MiMo - Token Plan | 推荐，降价后的 Toke Plan 可以冲 |
| 火山方舟 - Coding Plan | 不要买，倍率消耗太快 |
| 火山方舟 - Agent Plan | 不要买 |
| 智谱 GLM - Coding Plan | ★推荐，但是难抢 |
| Kimi - Membership | 可以买 |
| MiniMax - Token Plan | ★推荐，如果模型能力满足需求 |
| 移动云 - Coding Plan | 不要买, 不如直接买 MiniMax 的 |
| OpenCode Go - Coding Plan | ★推荐，可以使用站长的 [Freemodel Auto Router 开源桌面应用](https://github.com/will-17173/freemodel-auto-router) 来实现在 Claude Code 上使用 |
| 无问芯穹 - Coding Plan | 这个不懂了，买不到 |
| 天翼云 - Coding Plan | 不太懂，好像也买不到 |
| 阶跃星辰 - Coding Plan | 不要买，模型能力不太行 |
| ZenMux - Coding Plan | 太贵 |
| 摩尔线程 - Coding Plan | 不要买，没模型 |
| 优云智算 - Coding Plan | 次数少，抵扣率高，价格贵，不值 |
| 商汤科技 - Token Plan | 反正不要钱，玩玩 DeepSeek V4 Flash 可以 |
| GitCode - Coding Plan | 没抢到免费的现在也买不到了。站长运气好拿了个Pro，试了半个下午的GLM5.1，几乎一半时间都不可用 |
| 九章智算云 - Coding Plan | 看额度和模型，Lite 和 Pro 都没有买的价值，但是 Max 售价高达 699，还是去买 199 的讯飞吧 |

---

## 目录

- [Coding Plan（编程套餐）](#coding-plan编程套餐)
- [Token Plan（Token 套餐）](#token-plantoken-套餐)
- [Membership Plan（会员套餐）](#membership-plan会员套餐)
- [Agent Plan（智能体套餐）](#agent-plan智能体套餐)
- [国际 IDE Plan 对比](ide-plans.md)

---

## Coding Plan（编程套餐）

Coding Plan 通常以请求次数、prompts 或 Coding Credits 为计量单位，适合高频编程与 Agent 调用场景。

### 讯飞星火（Coding Plan）

| 套餐 | 价格 | 额度 | 支持模型 | 备注 |
| --- | --- | --- | --- | --- |
| 无忧版 | ¥19.00/月 | 请求次数不限 | Spark-X2-Flash, Qwen3.6-35B-A3B, Qwen3.5-35B-A3B, Qwen3-Coder-Next-FP8, GLM-4.7-Flash | 状态：停售；礼品卡 |
| 专业版 | ¥39.00/月 | 1,200次/5小时，9,000次/周，18,000次/月 | Spark X2, GLM-5, KIMI-K2.6, GLM-5.1, MiniMax-M2.5, KIMI-K2.5, DeepSeek-V3.2, Spark-X2-Flash, Qwen3.6-35B-A3B, GLM-4.7-Flash, Qwen3.5-35B-A3B, Qwen3-Coder-Next-FP8, Qwen3.5-397B-A17B | 状态：限售；礼品卡 |
| 高效版 | ¥199.00/月 | 6,000次/5小时，45,000次/周，90,000次/月 | Spark X2, GLM-5, GLM-5.2, DeepSeek-V4-Pro, DeepSeek-V4-Flash, KIMI-K2.6, GLM-5.1, MiniMax-M2.5, KIMI-K2.5, DeepSeek-V3.2, Spark-X2-Flash, Qwen3.6-35B-A3B, GLM-4.7-Flash, Qwen3.5-35B-A3B, Qwen3-Coder-Next-FP8, Qwen3.5-397B-A17B | 状态：限售；礼品卡 |

🔗 [官方链接](https://maas.xfyun.cn/packageSubscription?inviteCode=MAAS-A95A9B6C)

---

### 联通云（Coding Plan）

| 套餐 | 价格 | 额度 | 支持模型 | 备注 |
| --- | --- | --- | --- | --- |
| Coding Plan-Lite | ¥40/月 | 1,200次/5小时，9,000次/周，18,000次/月 | DeepSeek-V4-Flash, GLM-5.1, GLM-5, Qwen3.6-27B, Kimi-K2.6, Kimi-K2.5, Qwen3.5-397B-A17B, Qwen3-235B-A22B, MiniMax-M2.5 | DeepSeek-V4-Flash 当前服务资源有限，仅供尝鲜体验；上下文窗口目前仅支持 200K |
| Coding Plan-Pro | ¥200/月 | 6,000次/5小时，45,000次/周，90,000次/月 | DeepSeek-V4-Flash, GLM-5.1, GLM-5, Qwen3.6-27B, Kimi-K2.6, Kimi-K2.5, Qwen3.5-397B-A17B, Qwen3-235B-A22B, MiniMax-M2.5 | DeepSeek-V4-Flash 当前服务资源有限，仅供尝鲜体验；上下文窗口目前仅支持 200K |

🔗 [官方链接](https://support.cucloud.cn/document/127/591/2357.html?id=2357&arcid=7015&lang=zh)

---

### 京东云（Coding Plan）

| 套餐 | 价格 | 额度 | 支持模型 | 备注 |
| --- | --- | --- | --- | --- |
| Lite | ¥40/月 | 1,200次/5小时，9,000次/周，18,000次/月 | DeepSeek-V3.2, GLM-5, GLM-4.7, MiniMax-M2.5, Kimi-K2.5, Kimi-K2-Turbo, Qwen3-Coder | 状态：售罄；已售罄，点击预约 |
| Pro | ¥200/月 | 6,000次/5小时，45,000次/周，90,000次/月 | DeepSeek-V3.2, GLM-5, GLM-4.7, MiniMax-M2.5, Kimi-K2.5, Kimi-K2-Turbo, Qwen3-Coder | 状态：售罄；已售罄，点击预约 |

🔗 [官方链接](https://www.jdcloud.com/cn/pages/codingplan)

---

### 优云智算（Coding Plan） ✨ NEW

| 套餐 | 价格 | 额度 | 支持模型 | 备注 |
| --- | --- | --- | --- | --- |
| Mini 迷你版 | ¥49/月 | ≈200次/5小时，≈650次/周，≈1,900次/月 | MiniMax-M2.7, Kimi-K2.6, GLM-5.2, GLM-5.1, DeepSeek-V3.2, DeepSeek-V4-Flash | 送¥5 |
| Lite 入门版 | ¥99/月 | ≈400次/5小时，≈1,300次/周，≈3,800次/月 | MiniMax-M2.7, Kimi-K2.6, GLM-5.2, GLM-5.1, DeepSeek-V3.2, DeepSeek-V4-Flash | 送¥5 |
| Basic 基础版 | ¥199/月 | ≈800次/5小时，≈2,600次/周，≈7,600次/月 | MiniMax-M2.7, Kimi-K2.6, GLM-5.2, GLM-5.1, DeepSeek-V3.2, DeepSeek-V4-Flash | 送¥5 |
| Pro 增强版 | ¥499/月 | ≈2,000次/5小时，≈6,500次/周，≈19,000次/月 | MiniMax-M2.7, Kimi-K2.6, GLM-5.2, GLM-5.1, DeepSeek-V3.2, DeepSeek-V4-Flash | 送¥5 |
| Max 高级版 | ¥799/月 | ≈3,000次/5小时，≈10,500次/周，≈31,000次/月 | MiniMax-M2.7, Kimi-K2.6, GLM-5.2, GLM-5.1, DeepSeek-V3.2, DeepSeek-V4-Flash | 送¥5 |
| Ultra 畅享版 | ¥999/月 | ≈4,000次/5小时，≈13,500次/周，≈39,000次/月 | MiniMax-M2.7, Kimi-K2.6, GLM-5.2, GLM-5.1, DeepSeek-V3.2, DeepSeek-V4-Flash | 送¥5 |

🔗 [官方链接](https://passport.compshare.cn/register?referral_code=HBwlVOWwBRSFsCWYz6WEmj)

---

### 百度千帆（Coding Plan）

| 套餐 | 价格 | 额度 | 支持模型 | 备注 |
| --- | --- | --- | --- | --- |
| Lite | ¥40/月 | 1,200次/5小时，9,000次/周，18,000次/月 | Kimi-K2.5, DeepSeek-V3.2, DeepSeek-V4-Pro, DeepSeek-V4-Flash, GLM-5, GLM-5.1, MiniMax-M2.5, ERNIE-4.5-Turbo-20260402 | GLM-5 高峰期抵扣系数 3，低峰期 2；GLM-5.1 高峰期 4，低峰期 3（高峰期：每日 10:30-12:00、14:00-18:00，非固定值，随流量变化）；MiniMax-M2.5 即将下线。 |
| Pro | ¥200/月 | 6,000次/5小时，45,000次/周，90,000次/月 | Kimi-K2.5, DeepSeek-V3.2, DeepSeek-V4-Pro, DeepSeek-V4-Flash, GLM-5, GLM-5.1, MiniMax-M2.5, ERNIE-4.5-Turbo-20260402 | GLM-5 高峰期抵扣系数 3，低峰期 2；GLM-5.1 高峰期 4，低峰期 3（高峰期：每日 10:30-12:00、14:00-18:00，非固定值，随流量变化）；MiniMax-M2.5 即将下线。 |

🔗 [官方链接](https://cloud.baidu.com/product/codingplan.html)

---

### 国家超算中心（Coding Plan）

| 套餐 | 价格 | 额度 | 支持模型 | 备注 |
| --- | --- | --- | --- | --- |
| Lite | ¥20/月 | 1,200次/5小时，9,000次/周，18,000次/月 | MiniMax-M2.5, Qwen3-235B-A22B | - |
| Pro | ¥100/月 | 6,000次/5小时，45,000次/周，90,000次/月 | MiniMax-M2.5, Qwen3-235B-A22B | - |

🔗 [官方链接](https://www.scnet.cn/ac/openapi/doc/2.0/moduleapi/codingplan/quickstart.html)

---

### 阿里云百炼（Coding Plan）

| 套餐 | 价格 | 额度 | 支持模型 | 备注 |
| --- | --- | --- | --- | --- |
| Lite | ¥40/月 | 停售 | Qwen3.6-Plus, Kimi-K2.5, GLM-5, MiniMax-M2.5 | 状态：停售 |
| Pro | ¥200/月 | 6,000次/5小时，45,000次/周，90,000次/月 | Qwen3.6-Plus, Kimi-K2.5, GLM-5, MiniMax-M2.5 | 状态：限售；限量抢购 |

🔗 [官方链接](https://common-buy.aliyun.com/token-plan)

---

### 火山方舟（Coding Plan）

| 套餐 | 价格 | 额度 | 支持模型 | 备注 |
| --- | --- | --- | --- | --- |
| Lite | ¥40/月（首月优惠¥9.9/月） | 1,200次/5小时，9,000次/周，18,000次/月 | Auto, Doubao-Seed-2.0-Code, Doubao-Seed-2.0-Pro, Doubao-Seed-2.0-Lite, Doubao-Seed-Code, MiniMax-M2.7, MiniMax-M3, Kimi-K2.6, Kimi-K2.7-Code, GLM-5.2, DeepSeek-V4-Flash, DeepSeek-V4-Pro | 状态：限售；有倍率消耗机制，实际消耗量极快 |
| Pro | ¥200/月（首月优惠¥49.9/月） | 6,000 次 / 5小时，45,000 次 / 周，90,000 次 / 月 | Auto, Doubao-Seed-2.0-Code, Doubao-Seed-2.0-Pro, Doubao-Seed-2.0-Lite, Doubao-Seed-Code, MiniMax-M2.7, MiniMax-M3, Kimi-K2.6, Kimi-K2.7-Code, GLM-5.2, DeepSeek-V4-Flash, DeepSeek-V4-Pro | 有倍率消耗机制，实际消耗量极快 |

🔗 [官方链接](https://www.volcengine.com/activity/codingplan-feishu)

---

### 智谱（Coding Plan）

| 套餐 | 价格 | 额度 | 支持模型 | 备注 |
| --- | --- | --- | --- | --- |
| Lite | ¥49/月（¥39/月包年） | 约 80 prompts / 5小时，约 400 prompts / 周 | GLM-5.1, GLM-5.2, GLM-5-Turbo, GLM-4.7, GLM-4.5-Air | 状态：限售；GLM-5.1、GLM-5-Turbo 在高峰期（每日 14:00～18:00 UTC+8）消耗系数为 3 倍，非高峰期 2 倍（限时福利：非高峰期仅 1 倍，至 6 月底）；减 5% |
| Pro | ¥149/月（¥119/月包年） | 约 400 prompts / 5小时，约 2,000 prompts / 周 | GLM-5.1, GLM-5.2, GLM-5-Turbo, GLM-4.7, GLM-4.5-Air | 状态：限售；GLM-5.1、GLM-5-Turbo 在高峰期（每日 14:00～18:00 UTC+8）消耗系数为 3 倍，非高峰期 2 倍（限时福利：非高峰期仅 1 倍，至 6 月底）；减 5% |
| Max | ¥469/月（¥375/月包年） | 约 1,600 prompts / 5小时，约 8,000 prompts / 周 | GLM-5.1, GLM-5.2, GLM-5-Turbo, GLM-4.7, GLM-4.5-Air | 状态：限售；GLM-5.1、GLM-5-Turbo 在高峰期（每日 14:00～18:00 UTC+8）消耗系数为 3 倍，非高峰期 2 倍（限时福利：非高峰期仅 1 倍，至 6 月底）；减 5% |

🔗 [官方链接](https://www.bigmodel.cn/glm-coding?ic=TZCACBJWPP)

---

### GitCode（Coding Plan） ✨ NEW

| 套餐 | 价格 | 额度 | 支持模型 | 备注 |
| --- | --- | --- | --- | --- |
| AtomCode Lite | 免费 | 额度不明 | DeepSeek-V4-Flash, Qwen/Qwen3.6-35B-A3B, Qwen/Qwen3-VL-8B-Instruct | 状态：限售 |
| AtomCode Pro | 免费 | 额度不明 | GLM-5.1, DeepSeek-V4-Flash, Qwen/Qwen3.6-35B-A3B, Qwen/Qwen3-VL-8B-Instruct | 状态：限售 |
| AtomCode Max | 免费 | 额度不明 | GLM-5.1, DeepSeek-V4-Flash, Qwen/Qwen3.6-35B-A3B, Qwen/Qwen3-VL-8B-Instruct | 状态：限售 |

🔗 [官方链接](https://ai.gitcode.com/serverless-api)

---

### 移动云（Coding Plan）

| 套餐 | 价格 | 额度 | 支持模型 | 备注 |
| --- | --- | --- | --- | --- |
| Lite | ¥40/月（首月¥7.9） | 1,200次/5小时，9,000次/周，18,000次/月 | MiniMax-M2.5 | 注意：无购买价值 |
| Pro | ¥200/月（首月¥39.9） | 6,000次/5小时，45,000次/周，90,000次/月 | MiniMax-M2.5 | 注意：无购买价值 |

🔗 [官方链接](https://ecloud.10086.cn/portal/act/codingplan)

---

### OpenCode（Coding Plan）

| 套餐 | 价格 | 额度 | 支持模型 | 备注 |
| --- | --- | --- | --- | --- |
| Go | $10/月（首月 $5） | $12/5小时, $30/周, $60/月 | GLM-5.1, GLM-5.2, Kimi-K2.6, Kimi-K2.7-Code, MiMo-V2.5, MiMo-V2.5-Pro, MiniMax-M3, MiniMax-M2.7, Qwen3.6 Plus, Qwen3.7 Max, Qwen3.7 Plus, DeepSeek V4 Pro, DeepSeek V4 Flash | 送$5 |

🔗 [官方链接](https://opencode.ai/go?ref=GWM08015AT)

---

### 无问芯穹（Coding Plan）

| 套餐 | 价格 | 额度 | 支持模型 | 备注 |
| --- | --- | --- | --- | --- |
| Lite | ¥40/月 | 停售 | DeepSeek-V3.2, DeepSeek-V3.2-Thinking, Kimi-K2.5, MiniMax-M2.1, MiniMax-M2.5, MiniMax-M2.7, GLM-4.7, GLM-5, GLM-5.1 | 状态：停售 |
| Pro | ¥200/月 | 停售 | DeepSeek-V3.2, DeepSeek-V3.2-Thinking, Kimi-K2.5, MiniMax-M2.1, MiniMax-M2.5, MiniMax-M2.7, GLM-4.7, GLM-5, GLM-5.1 | 状态：停售 |

🔗 [官方链接](https://cloud.infini-ai.com/platform/ai)

---

### 天翼云（Coding Plan）

| 套餐 | 价格 | 额度 | 支持模型 | 备注 |
| --- | --- | --- | --- | --- |
| GLM Lite | ¥49/月 | 每 5 小时 80 次 prompts，每周 400 次 prompts，每月 1,600 次 prompts | GLM-5-Turbo, GLM-5.1, GLM-4.5, GLM-4.5-Air, GLM-4.6, GLM-4.7 | 状态：限售 |
| GLM Pro | ¥149/月 | 每 5 小时 400 次 prompts，每周 2,000 次 prompts，每月 8,000 次 prompts | GLM-5-Turbo, GLM-5.1, GLM-4.5, GLM-4.5-Air, GLM-4.6, GLM-4.7 | 状态：限售 |
| GLM Max | ¥469/月 | 每 5 小时 1,600 次 prompts，每周 8,000 次 prompts，每月 32,000 次 prompts | GLM-5-Turbo, GLM-5.1, GLM-4.5, GLM-4.5-Air, GLM-4.6, GLM-4.7 | 状态：限售 |

🔗 [官方链接](https://ctxirang.ctyun.cn/maas/codingPlan)

---

### 阶跃星辰（Coding Plan）

| 套餐 | 价格 | 额度 | 支持模型 | 备注 |
| --- | --- | --- | --- | --- |
| Flash Mini（入门版） | ¥49/月 | 每 5 小时 100 次 prompts，每周 400 次 prompts | Step 3.5 Flash 2603, Step 3.5 Flash, StepAudio 2.5 TTS, StepAudio 2.5 ASR, Step Router V1, Step Image Edit 2 | - |
| Flash Plus（进阶版） | ¥99/月 | 每 5 小时约 400 次 prompts，每周约 1,600 次 prompts | Step 3.5 Flash 2603, Step 3.5 Flash, StepAudio 2.5 TTS, StepAudio 2.5 ASR, Step Router V1, Step Image Edit 2 | - |
| Flash Pro（专业版） | ¥199/月 | 每 5 小时约 1,500 次 prompts，每周约 6,000 次 prompts | Step 3.5 Flash 2603, Step 3.5 Flash, StepAudio 2.5 TTS, StepAudio 2.5 ASR, Step Router V1, Step Image Edit 2 | - |
| Flash Max（旗舰版） | ¥699/月 | 每 5 小时约 5,000 次 prompts，每周约 2 万次 prompts | Step 3.5 Flash 2603, Step 3.5 Flash, StepAudio 2.5 TTS, StepAudio 2.5 ASR, Step Router V1, Step Image Edit 2 | - |

🔗 [官方链接](https://platform.stepfun.com/step-plan)

---

### ZenMux（Coding Plan）

| 套餐 | 价格 | 额度 | 支持模型 | 备注 |
| --- | --- | --- | --- | --- |
| Pro | $20/月（约 ¥144） | 每 5 小时 50 Flows（约 $1.64），每周 213.37 Flows（约 $7.01），每月 914.44 Flows（约 $30.03） | 100+ 模型 | 减 5%；[模型列表](https://zenmux.ai/models) |
| Max | $100/月（约 ¥720） | 每 5 小时 300 Flows（约 $9.85），每周 1,280.22 Flows（约 $42.04），每月 5,486.66 Flows（约 $180.15） | 100+ 模型 | 减 5%；[模型列表](https://zenmux.ai/models) |
| Ultra | $200/月（约 ¥1440） | 每 5 小时 800 Flows（约 $26.27），每周 3,413.92 Flows（约 $112.09），每月 14,631.09 Flows（约 $480.40） | 100+ 模型 | 减 5%；[模型列表](https://zenmux.ai/models) |

🔗 [官方链接](https://zenmux.ai/invite/6VGCHU) | [模型详情](https://zenmux.ai/models)

---

### 摩尔线程（Coding Plan）

| 套餐 | 价格 | 额度 | 支持模型 | 备注 |
| --- | --- | --- | --- | --- |
| Lite Plan | ¥120/季度 | 每 5 小时最多约 120 次 prompts | GLM-4.7 | Claude Pro 套餐的 3 倍用量 |
| Pro Plan | ¥600/季度 | 每 5 小时最多约 600 次 prompts | GLM-4.7 | Lite Plan 的 5 倍用量，生成速度高于 Lite Plan |
| Max Plan | ¥1200/季度 | 每 5 小时最多约 2400 次 prompts | GLM-4.7 | Pro Plan 的 4 倍用量，优先保障用量高峰，抢先体验新功能 |

🔗 [官方链接](https://code.mthreads.com/)

---

### 九章智算云（Coding Plan）

| 套餐 | 价格 | 额度 | 支持模型 | 备注 |
| --- | --- | --- | --- | --- |
| Lite | ¥39/月 | 停售 | MiniMax-M2.5, MiniMax-M2.1 | 状态：停售；具体用量不明 |
| Plus | ¥199/月 | 停售 | MiniMax-M2.5, MiniMax-M2.1, GLM-5 | 状态：停售 |
| Max | ¥699/月 | 停售 | MiniMax-M2.5, MiniMax-M2.1, GLM-5, GLM-5.1, DeepSeek-V4-Pro | 状态：停售；独享 DeepSeek-V4-Pro 1M 上下文 |

🔗 [官方链接](https://codingplan.alayanew.com/)

---

### Agnes（Coding Plan） ✨ NEW

| 套餐 | 价格 | 额度 | 支持模型 | 备注 |
| --- | --- | --- | --- | --- |
| Starter | $2/月（原价 $4，50% OFF） | 1,500次/5小时 | Agnes-2.0-Flash | 更大的 RPM 限制；由 Agnes-2.0-Flash 提供支持，通常约 100 TPS，非高峰时段 150 TPS。；兼容主流 Coding 工具；可使用图像理解；可使用图片模型；可使用视频模型 |
| Plus | $5/月（原价 $10，50% OFF） | 7,500次/5小时 | Agnes-2.0-Flash | 更大的 RPM 限制；由 Agnes-2.0-Flash 提供支持，通常约 100 TPS，非高峰时段 150 TPS。；兼容主流 Coding 工具；可使用图像理解；可使用图片模型；可使用视频模型 |
| Pro | $25/月（原价 $50，50% OFF） | 30,000次/5小时 | Agnes-2.0-Flash | 更大的 RPM 限制；由 Agnes-2.0-Flash 提供支持，通常约 100 TPS，非高峰时段 150 TPS。；兼容主流 Coding 工具；可使用图像理解；可使用图片模型；可使用视频模型 |

🔗 [官方链接](https://agnes-ai.com/)

---

## Token Plan（Token 套餐）

Token Plan 以 Tokens 或 Credits 为计量单位，适合需要精细控制消耗和成本的 API 使用场景。

### 讯飞星火（Token Plan）

| 套餐 | 价格 | 额度 | 支持模型 | 备注 |
| --- | --- | --- | --- | --- |
| Token Plan 团队版 标准成员 | ¥160/席/月（限时8折，原价 ¥200） | 20,000 Credits / 月，200万 TPM | Spark X2, Spark-X2-Flash, GLM-5.2, GLM-5.1, GLM-5, DeepSeek-V4-Pro, DeepSeek-V4-Flash, DeepSeek-V3.2, Kimi-K2.6, Kimi-K2.5, MiniMax-M2.5, Qwen3.5-397B-A17B, Qwen3.6-35B-A3B, Qwen3.5-35B-A3B, Qwen3-Coder-Next-FP8, GLM-4.7-Flash | - |
| Token Plan 团队版 高级成员 | ¥420/席/月（限时7折，原价 ¥600） | 60,000 Credits / 月，300万 TPM | Spark X2, Spark-X2-Flash, GLM-5.2, GLM-5.1, GLM-5, DeepSeek-V4-Pro, DeepSeek-V4-Flash, DeepSeek-V3.2, Kimi-K2.6, Kimi-K2.5, MiniMax-M2.5, Qwen3.5-397B-A17B, Qwen3.6-35B-A3B, Qwen3.5-35B-A3B, Qwen3-Coder-Next-FP8, GLM-4.7-Flash | - |
| Token Plan 团队版 尊享成员 | ¥1,200/席/月（限时6折，原价 ¥2,000） | 200,000 Credits / 月，500万 TPM | Spark X2, Spark-X2-Flash, GLM-5.2, GLM-5.1, GLM-5, DeepSeek-V4-Pro, DeepSeek-V4-Flash, DeepSeek-V3.2, Kimi-K2.6, Kimi-K2.5, MiniMax-M2.5, Qwen3.5-397B-A17B, Qwen3.6-35B-A3B, Qwen3.5-35B-A3B, Qwen3-Coder-Next-FP8, GLM-4.7-Flash | - |

🔗 [官方链接](https://maas.xfyun.cn/tokenPlan/subscription)

---

### 联通云（Token Plan）

| 套餐 | 价格 | 额度 | 支持模型 | 备注 |
| --- | --- | --- | --- | --- |
| Token Plan-Lite（个人版） | ¥15/月 | 600万 Tokens | DeepSeek-V4-Flash, MiniMax-M2.5 | - |
| Token Plan-Pro（个人版） | ¥30/月 | 1,200万 Tokens | DeepSeek-V4-Flash, MiniMax-M2.5 | - |
| Token Plan-Max（个人版） | ¥45/月 | 1,800万 Tokens | DeepSeek-V4-Flash, MiniMax-M2.5 | - |
| Token Plan-Lite（团队版） | ¥198/月 | 25,000 Credits | DeepSeek-V4-Pro, DeepSeek-V4-Flash, MiniMax-M2.5 | - |
| Token Plan-Pro（团队版） | ¥698/月 | 100,000 Credits | DeepSeek-V4-Pro, DeepSeek-V4-Flash, MiniMax-M2.5 | - |
| Token Plan-Max（团队版） | ¥1,398/月 | 250,000 Credits | DeepSeek-V4-Pro, DeepSeek-V4-Flash, MiniMax-M2.5 | - |

🔗 [官方链接](https://support.cucloud.cn/document/127/591/2357.html?id=2357&arcid=7080)

---

### 京东云（Token Plan）

| 套餐 | 价格 | 额度 | 支持模型 | 备注 |
| --- | --- | --- | --- | --- |
| 个人版 Lite | ¥69/月 | 17,250 Credits / 月 | GLM-5.1, GLM-5.2, Kimi-K2.6, MiniMax-M3, DeepSeek-V4-Flash, DeepSeek-V4-Pro | - |
| 个人版 Standard | ¥199/月 | 49,750 Credits / 月 | GLM-5.1, GLM-5.2, Kimi-K2.6, MiniMax-M3, DeepSeek-V4-Flash, DeepSeek-V4-Pro | - |
| 个人版 Pro | ¥599/月 | 149,750 Credits / 月 | GLM-5.1, GLM-5.2, Kimi-K2.6, MiniMax-M3, DeepSeek-V4-Flash, DeepSeek-V4-Pro | - |
| 个人版 Max | ¥999/月 | 249,750 Credits / 月 | GLM-5.1, GLM-5.2, Kimi-K2.6, MiniMax-M3, DeepSeek-V4-Flash, DeepSeek-V4-Pro | - |
| 企业版 Lite | ¥207/席/月 | 34,500 Credits / 席 / 月 | GLM-5.1, GLM-5.2, Kimi-K2.6, MiniMax-M3, DeepSeek-V4-Flash, DeepSeek-V4-Pro | 白名单邀请制，单个企业最多 50 个席位 |
| 企业版 Standard | ¥597/席/月 | 99,500 Credits / 席 / 月 | GLM-5.1, GLM-5.2, Kimi-K2.6, MiniMax-M3, DeepSeek-V4-Flash, DeepSeek-V4-Pro | 白名单邀请制，单个企业最多 50 个席位 |
| 企业版 Pro | ¥1,797/席/月 | 299,500 Credits / 席 / 月 | GLM-5.1, GLM-5.2, Kimi-K2.6, MiniMax-M3, DeepSeek-V4-Flash, DeepSeek-V4-Pro | 白名单邀请制，单个企业最多 50 个席位 |
| 企业版 Max | ¥2,997/席/月 | 499,500 Credits / 席 / 月 | GLM-5.1, GLM-5.2, Kimi-K2.6, MiniMax-M3, DeepSeek-V4-Flash, DeepSeek-V4-Pro | 白名单邀请制，单个企业最多 50 个席位 |

🔗 [官方链接](https://docs.jdcloud.com/cn/jdaip/pricing)

---

### 腾讯云（Token Plan）

| 套餐 | 价格 | 额度 | 支持模型 | 备注 |
| --- | --- | --- | --- | --- |
| Lite | ¥39/月 | 3,500万 Tokens/月 | Auto, GLM-5, GLM-5.1, Kimi-K2.5, MiniMax-M2.5, MiniMax-M2.7, DeepSeek-V4-Flash, DeepSeek-V4-Pro | Kimi-K2.5 将于 2026-07-31 下线。；MiniMax-M2.5 将于 2026-08-06 下线。 |
| Standard | ¥99/月 | 1亿 Tokens/月 | Auto, GLM-5, GLM-5.1, Kimi-K2.5, MiniMax-M2.5, MiniMax-M2.7, DeepSeek-V4-Flash, DeepSeek-V4-Pro | Kimi-K2.5 将于 2026-07-31 下线。；MiniMax-M2.5 将于 2026-08-06 下线。 |
| Pro | ¥299/月 | 3.2亿 Tokens/月 | Auto, GLM-5, GLM-5.1, Kimi-K2.5, MiniMax-M2.5, MiniMax-M2.7, DeepSeek-V4-Flash, DeepSeek-V4-Pro | Kimi-K2.5 将于 2026-07-31 下线。；MiniMax-M2.5 将于 2026-08-06 下线。 |
| Max | ¥599/月 | 6.5亿 Tokens/月 | Auto, GLM-5, GLM-5.1, Kimi-K2.5, MiniMax-M2.5, MiniMax-M2.7, DeepSeek-V4-Flash, DeepSeek-V4-Pro | Kimi-K2.5 将于 2026-07-31 下线。；MiniMax-M2.5 将于 2026-08-06 下线。 |
| HY Lite | ¥28/月 | 3500万 Tokens/月 | Hy3, Hy3 preview | - |
| HY Standard | ¥78/月 | 1亿 Tokens/月 | Hy3, Hy3 preview | - |
| HY Pro | ¥238/月 | 3.2亿 Tokens/月 | Hy3, Hy3 preview | - |
| HY Max | ¥468/月 | 6.5亿 Tokens/月 | Hy3, Hy3 preview | - |
| 企业版专业套餐 | ¥1,000/月起 | 100,000 积分/月起 | Auto, GLM-5.2, GLM-5, GLM-5.1, GLM-5-Turbo, Kimi-K2.5, Kimi-K2.6, Kimi-K2.7-Code, Kimi-K2.7-Code-HighSpeed, MiniMax-M2.5, MiniMax-M2.7, MiniMax-M3, DeepSeek-V4-Flash, DeepSeek-V4-Pro, DeepSeek-V4-Flash 原厂直供, DeepSeek-V4-Pro 原厂直供 | 自定义规格，100 积分 ¥1；单次最低购买 100,000 积分 |
| 企业版轻享套餐 | ¥100/月起 | 5,000万 Tokens/月起 | - | 自定义规格，¥2/百万 Tokens；单次最低购买 5,000 万 Tokens |

🔗 [官方链接](https://cloud.tencent.com/document/product/1823/130060)

---

### 百度千帆（Token Plan）

| 套餐 | 价格 | 额度 | 支持模型 | 备注 |
| --- | --- | --- | --- | --- |
| 轻享版（企业版） | ¥149/月 | 20,000 积分/月 | DeepSeek-V4-Pro, DeepSeek-V4-Flash, DeepSeek-V3.2, GLM-5.2, GLM-5.1, GLM-5, Kimi-K2.6 | 企业/团队席位；仅限兼容 AI 编程和智能体工具中的交互式使用 |
| 标准版（企业版） | ¥419/月 | 60,000 积分/月 | DeepSeek-V4-Pro, DeepSeek-V4-Flash, DeepSeek-V3.2, GLM-5.2, GLM-5.1, GLM-5, Kimi-K2.6 | 企业/团队席位；仅限兼容 AI 编程和智能体工具中的交互式使用 |
| 高级版（企业版） | ¥989/月 | 150,000 积分/月 | DeepSeek-V4-Pro, DeepSeek-V4-Flash, DeepSeek-V3.2, GLM-5.2, GLM-5.1, GLM-5, Kimi-K2.6 | 企业/团队席位；仅限兼容 AI 编程和智能体工具中的交互式使用 |

🔗 [官方链接](https://cloud.baidu.com/doc/qianfan/s/ymq8wwch2)

---

### 阿里云百炼（Token Plan）

| 套餐 | 价格 | 额度 | 支持模型 | 备注 |
| --- | --- | --- | --- | --- |
| 个人版 Lite | ¥99/月 | 12,500 Credits / 月 | qwen3.8-max-preview, qwen3.7-max, qwen3.7-plus, qwen3.6-plus, qwen3.6-flash, qwen-image-2.0, qwen-image-2.0-pro, wan2.7-image, wan2.7-image-pro, happyhorse-1.1, deepseek-v4-pro, deepseek-v4-flash, deepseek-v3.2, kimi-k2.7-code, kimi-k2.6, kimi-k2.5, glm-5.2, glm-5.1, glm-5, MiniMax-M2.5 | - |
| 个人版 Standard | ¥199/月 | 25,000 Credits / 月 | qwen3.8-max-preview, qwen3.7-max, qwen3.7-plus, qwen3.6-plus, qwen3.6-flash, qwen-image-2.0, qwen-image-2.0-pro, wan2.7-image, wan2.7-image-pro, happyhorse-1.1, deepseek-v4-pro, deepseek-v4-flash, deepseek-v3.2, kimi-k2.7-code, kimi-k2.6, kimi-k2.5, glm-5.2, glm-5.1, glm-5, MiniMax-M2.5 | - |
| 个人版 Pro | ¥599/月 | 100,000 Credits / 月 | qwen3.8-max-preview, qwen3.7-max, qwen3.7-plus, qwen3.6-plus, qwen3.6-flash, qwen-image-2.0, qwen-image-2.0-pro, wan2.7-image, wan2.7-image-pro, happyhorse-1.1, deepseek-v4-pro, deepseek-v4-flash, deepseek-v3.2, kimi-k2.7-code, kimi-k2.6, kimi-k2.5, glm-5.2, glm-5.1, glm-5, MiniMax-M2.5 | - |
| 团队版 标准坐席 | ¥150/坐席/月（限时价，原价 ¥198） | 25,000 Credits / 月 | qwen3.8-max-preview, qwen3.7-max, qwen3.7-plus, qwen3.6-plus, qwen3.6-flash, qwen-image-2.0, qwen-image-2.0-pro, wan2.7-image, wan2.7-image-pro, happyhorse-1.1, deepseek-v4-pro, deepseek-v4-flash, deepseek-v3.2, kimi-k2.7-code, kimi-k2.6, kimi-k2.5, glm-5.2, glm-5.1, glm-5, MiniMax-M2.5 | - |
| 团队版 高级坐席 | ¥550/坐席/月（限时价，原价 ¥698） | 100,000 Credits / 月 | qwen3.8-max-preview, qwen3.7-max, qwen3.7-plus, qwen3.6-plus, qwen3.6-flash, qwen-image-2.0, qwen-image-2.0-pro, wan2.7-image, wan2.7-image-pro, happyhorse-1.1, deepseek-v4-pro, deepseek-v4-flash, deepseek-v3.2, kimi-k2.7-code, kimi-k2.6, kimi-k2.5, glm-5.2, glm-5.1, glm-5, MiniMax-M2.5 | - |
| 团队版 尊享坐席 | ¥1,398/坐席/月 | 250,000 Credits / 月 | qwen3.8-max-preview, qwen3.7-max, qwen3.7-plus, qwen3.6-plus, qwen3.6-flash, qwen-image-2.0, qwen-image-2.0-pro, wan2.7-image, wan2.7-image-pro, happyhorse-1.1, deepseek-v4-pro, deepseek-v4-flash, deepseek-v3.2, kimi-k2.7-code, kimi-k2.6, kimi-k2.5, glm-5.2, glm-5.1, glm-5, MiniMax-M2.5 | - |
| 团队版 共享包 | ¥5,000 | 625,000 Credits（有效期1月） | qwen3.8-max-preview, qwen3.7-max, qwen3.7-plus, qwen3.6-plus, qwen3.6-flash, qwen-image-2.0, qwen-image-2.0-pro, wan2.7-image, wan2.7-image-pro, happyhorse-1.1, deepseek-v4-pro, deepseek-v4-flash, deepseek-v3.2, kimi-k2.7-code, kimi-k2.6, kimi-k2.5, glm-5.2, glm-5.1, glm-5, MiniMax-M2.5 | - |

🔗 [官方链接](https://common-buy.aliyun.com/token-plan)

---

### 商汤科技（Token Plan）

| 套餐 | 价格 | 额度 | 支持模型 | 备注 |
| --- | --- | --- | --- | --- |
| Free · 公测 | ¥0/月 | SenseNova 6.7 Flash-Lite 与 SenseNova U1 Fast：每 5 小时 1,500 次调用；DeepSeek V4 Flash：每 5 小时 150 次调用 | SenseNova 6.7 Flash-Lite, SenseNova U1 Fast, DeepSeek V4 Flash | 状态：限售 |

🔗 [官方链接](https://www.sensenova.cn/token-plan)

---

### 小米 MiMo（Token Plan）

| 套餐 | 价格 | 额度 | 支持模型 | 备注 |
| --- | --- | --- | --- | --- |
| Lite | ¥39/月 | 41亿 Credits / 月 | MiMo-V2.5-Pro, MiMo-V2.5, MiMo-V2.5-TTS-VoiceClone, MiMo-V2.5-TTS-VoiceDesign, MiMo-V2.5-TTS, V2 系列模型 | 非高峰期（16:00-24:00 UTC） 0.8x 系数消耗；TTS 系列模型限时免费，不消耗套餐 Token；送¥10 |
| Standard | ¥99.00/月 | 110亿 Credits / 月 | MiMo-V2.5-Pro, MiMo-V2.5, MiMo-V2.5-TTS-VoiceClone, MiMo-V2.5-TTS-VoiceDesign, MiMo-V2.5-TTS, V2 系列模型 | 非高峰期（16:00-24:00 UTC） 0.8x 系数消耗；TTS 系列模型限时免费，不消耗套餐 Token；送¥10 |
| Pro | ¥329.00/月 | 380亿 Credits / 月 | MiMo-V2.5-Pro, MiMo-V2.5, MiMo-V2.5-TTS-VoiceClone, MiMo-V2.5-TTS-VoiceDesign, MiMo-V2.5-TTS, V2 系列模型 | 非高峰期（16:00-24:00 UTC） 0.8x 系数消耗；TTS 系列模型限时免费，不消耗套餐 Token；送¥10 |
| Max | ¥659.00/月 | 820亿 Credits / 月 | MiMo-V2.5-Pro, MiMo-V2.5, MiMo-V2.5-TTS-VoiceClone, MiMo-V2.5-TTS-VoiceDesign, MiMo-V2.5-TTS, V2 系列模型 | 非高峰期（16:00-24:00 UTC） 0.8x 系数消耗；TTS 系列模型限时免费，不消耗套餐 Token；送¥10 |

🔗 [官方链接](https://platform.xiaomimimo.com?ref=Q3UZRP)

---

### MiniMax（Token Plan）

| 套餐 | 价格 | 额度 | 支持模型 | 备注 |
| --- | --- | --- | --- | --- |
| Starter | ¥29/月 | 月度约 2 亿+ token M3 用量 | MiniMax-M3, MiniMax-M2.7, MiniMax-M2.5, Image-01 | 9折 |
| Plus | ¥49/月（¥490包年） | 月度约 6 亿+ token M3 用量 | MiniMax-M3, MiniMax-M2.7, MiniMax-M2.5, Image-01, Speech-2.8-HD | 9折 |
| Max | ¥119/月（¥1,190/包年） | 月度约 18 亿+ token M3 用量 | MiniMax-M3, MiniMax-M2.7, MiniMax-M2.5, Image-01, Speech-2.8-HD, Hailuo-2.3, Music-2.6 | 9折 |
| Plus 极速 | ¥98/月（¥82/月包年） | 停售 | MiniMax-M3-Highspeed, MiniMax-M2.7-Highspeed, MiniMax-M2.5-Highspeed | 状态：停售；9折 |
| Max 极速 | ¥199/月（¥166/月包年） | 停售 | MiniMax-M3-Highspeed, MiniMax-M2.7-Highspeed, MiniMax-M2.5-Highspeed | 状态：停售；9折 |
| Ultra | ¥469/月（¥4,690包年） | 月度约 55 亿+ token M3 用量 | MiniMax-M3, MiniMax-M2.7, MiniMax-M2.5, Image-01, Speech-2.8-HD, Hailuo-2.3, Music-2.6 | 9折 |

🔗 [官方链接](https://platform.minimaxi.com/subscribe/token-plan?code=Kl2lCY9oN3&source=link)

---

### 九章智算云（Token Plan）

| 套餐 | 价格 | 额度 | 支持模型 | 备注 |
| --- | --- | --- | --- | --- |
| TokenPlan 入门版 | ¥199/月 | 约 3,270万 Tokens/月（GLM-5.1 估算）；GLM-5.2 约 2,460万，DeepSeek-V4-Flash 约 1.98亿 | GLM-5.1, GLM-5.2, DeepSeek-V4-Flash | 按模型实际单价实时扣减，估算基于输入:输出约 200:1；命中上下文缓存时实际可用量会更高；当月套餐额度消耗完毕后自动停止推理服务 |
| TokenPlan 进阶版 | ¥399/月 | 约 6,550万 Tokens/月（GLM-5.1 估算）；GLM-5.2 约 4,930万，DeepSeek-V4-Flash 约 3.97亿 | GLM-5.1, GLM-5.2, DeepSeek-V4-Flash | 按模型实际单价实时扣减，估算基于输入:输出约 200:1；命中上下文缓存时实际可用量会更高；当月套餐额度消耗完毕后自动停止推理服务 |
| TokenPlan 旗舰版 | ¥699/月 | 约 1.15亿 Tokens/月（GLM-5.1 估算）；GLM-5.2 约 8,630万，DeepSeek-V4-Flash 约 6.96亿 | GLM-5.1, GLM-5.2, DeepSeek-V4-Flash | 按模型实际单价实时扣减，估算基于输入:输出约 200:1；命中上下文缓存时实际可用量会更高；当月套餐额度消耗完毕后自动停止推理服务 |

🔗 [官方链接](https://codingplan.alayanew.com/docs/billing)

---

## Membership Plan（会员套餐）

Membership Plan 通常绑定厂商自有客户端、网页会员或代码助手权益。

### 月之暗面（Membership Plan）

| 套餐 | 价格 | 额度 | 支持模型 | 备注 |
| --- | --- | --- | --- | --- |
| Andante | ¥49/月（¥39/月年付） | 1倍 Kimi Code 额度，约 30 个 Agent / 月 | Kimi-K2.6, Kimi-K2.5, Kimi-K2 | - |
| Moderato | ¥99/月（¥79/月年付） | 4倍 Kimi Code 额度，约 60 个 Agent / 月 | Kimi-K2.6, Kimi-K2.5, Kimi-K2 | - |
| Allegretto | ¥199/月（¥159/月年付） | 20倍 Kimi Code 额度，约 150 个 Agent / 月 | Kimi-K2.6, Kimi-K2.5, Kimi-K2 | - |
| Allegro | ¥699/月（¥559/月年付） | 60倍 Kimi Code 额度，约 360 个 Agent / 月 | Kimi-K2.6, Kimi-K2.5, Kimi-K2 | - |

🔗 [官方链接](https://www.kimi.com/code)

---

## Agent Plan（智能体套餐）

Agent Plan 面向更长链路的智能体任务，额度常包含工具调用、视觉处理或 AFP 等复合计量。

### 火山方舟（Agent Plan）

| 套餐 | 价格 | 额度 | 支持模型 | 备注 |
| --- | --- | --- | --- | --- |
| Small | ¥40/月 | 20,000 AFP/月，7,000 AFP/周，2,000 AFP/5小时，10,000 AFP/视觉日额度 | Doubao-Seed-2.0-Mini, Doubao-Seed-2.0-Lite, Doubao-Seed-2.0-Code, Doubao-Seed-2.0-Pro, DeepSeek-V4-Flash, MiniMax-M3, GLM-5.2, Kimi-K2.6, Kimi-K2.7-Code, DeepSeek-V4-Pro, Doubao-Embedding-Vision, Doubao-Seedream-5.0-Lite, Doubao-Seed-TTS-2.0, Doubao-Seed-ASR-2.0 | [细则](https://www.volcengine.com/docs/82379/2366394?lang=zh) |
| Medium | ¥200/月 | 100,000 AFP/月，35,000 AFP/周，10,000 AFP/5小时，50,000 AFP/视觉日额度 | Doubao-Seed-2.0-Mini, Doubao-Seed-2.0-Lite, Doubao-Seed-2.0-Code, Doubao-Seed-2.0-Pro, DeepSeek-V4-Flash, MiniMax-M2.7, MiniMax-M3, GLM-5.2, Kimi-K2.6, Kimi-K2.7-Code, DeepSeek-V4-Pro, Doubao-Embedding-Vision, Doubao-Seedream-5.0-Lite, Doubao-Seedance-1.5-Pro, Doubao-Seed-TTS-2.0, Doubao-Seed-ASR-2.0 | [细则](https://www.volcengine.com/docs/82379/2366394?lang=zh) |
| Large | ¥500/月 | 250,000 AFP/月，87,500 AFP/周，25,000 AFP/5小时，125,000 AFP/视觉日额度 | Doubao-Seed-2.0-Mini, Doubao-Seed-2.0-Lite, Doubao-Seed-2.0-Code, Doubao-Seed-2.0-Pro, DeepSeek-V4-Flash, MiniMax-M2.7, MiniMax-M3, GLM-5.2, Kimi-K2.6, Kimi-K2.7-Code, DeepSeek-V4-Pro, Doubao-Embedding-Vision, Doubao-Seedream-5.0-Lite, Doubao-Seedance-1.5-Pro, Doubao-Seedance-2.0, Doubao-Seedance-2.0-Fast, Doubao-Seedance-2.0-Mini, Doubao-Seed-TTS-2.0, Doubao-Seed-ASR-2.0 | [细则](https://www.volcengine.com/docs/82379/2366394?lang=zh) |
| Max | ¥1000/月 | 500,000 AFP/月，175,000 AFP/周，50,000 AFP/5小时，250,000 AFP/视觉日额度 | Doubao-Seed-2.0-Mini, Doubao-Seed-2.0-Lite, Doubao-Seed-2.0-Code, Doubao-Seed-2.0-Pro, DeepSeek-V4-Flash, MiniMax-M2.7, MiniMax-M3, GLM-5.2, Kimi-K2.6, Kimi-K2.7-Code, DeepSeek-V4-Pro, Doubao-Embedding-Vision, Doubao-Seedream-5.0-Lite, Doubao-Seedance-1.5-Pro, Doubao-Seedance-2.0, Doubao-Seedance-2.0-Fast, Doubao-Seedance-2.0-Mini, Doubao-Seed-TTS-2.0, Doubao-Seed-ASR-2.0 | [细则](https://www.volcengine.com/docs/82379/2366394?lang=zh) |

🔗 [官方链接](https://ai.volcengine.com/activity/agentplan)

---

## 数据来源

所有套餐数据均从网站主数据源 `src/data/plans` 生成，来源链接见各厂商表格后的官方链接。具体价格、模型、额度和购买状态可能随厂商调整而变化，请以官方页面为准。

如有数据错误或遗漏，欢迎提交 Issue 或 PR 修正。

---

## 相关链接

- [国际 IDE Plan 对比（Cursor、GitHub Copilot、Claude 等）](ide-plans.md)
- [完整对比网站](https://www.coding-plan.xyz)
