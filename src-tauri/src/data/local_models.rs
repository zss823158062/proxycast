//! 本地硬编码的国内模型数据
//!
//! 这些模型数据用于补充 models.dev API 未覆盖的国内模型

use crate::models::model_registry::{
    EnhancedModelMetadata, ModelCapabilities, ModelLimits, ModelPricing, ModelSource, ModelStatus,
    ModelTier,
};

/// 获取所有本地硬编码的国内模型
pub fn get_local_models() -> Vec<EnhancedModelMetadata> {
    let mut models = Vec::new();
    models.extend(get_dashscope_models());
    models.extend(get_zhipu_models());
    models.extend(get_baichuan_models());
    models.extend(get_moonshot_models());
    models.extend(get_deepseek_models());
    models.extend(get_doubao_models());
    models.extend(get_minimax_models());
    models.extend(get_yi_models());
    models.extend(get_stepfun_models());
    models
}

/// 通义千问系列模型 (阿里云百炼)
fn get_dashscope_models() -> Vec<EnhancedModelMetadata> {
    let now = chrono::Utc::now().timestamp();
    vec![
        EnhancedModelMetadata {
            id: "qwen3-coder-plus".to_string(),
            display_name: "通义千问 Coder Plus".to_string(),
            provider_id: "dashscope".to_string(),
            provider_name: "阿里云百炼".to_string(),
            family: Some("qwen-coder".to_string()),
            tier: ModelTier::Pro,
            capabilities: ModelCapabilities {
                vision: false, tools: true, streaming: true,
                json_mode: true, function_calling: true, reasoning: false,
            },
            pricing: Some(ModelPricing {
                input_per_million: Some(4.0), output_per_million: Some(16.0),
                cache_read_per_million: None, cache_write_per_million: None,
                currency: "CNY".to_string(),
            }),
            limits: ModelLimits {
                context_length: Some(131072), max_output_tokens: Some(8192),
                requests_per_minute: None, tokens_per_minute: None,
            },
            status: ModelStatus::Active,
            release_date: Some("2025-01-01".to_string()),
            is_latest: true,
            description: Some("阿里云通义千问代码模型".to_string()),
            source: ModelSource::Local,
            created_at: now, updated_at: now,
        },
        EnhancedModelMetadata {
            id: "qwen-max".to_string(),
            display_name: "通义千问 Max".to_string(),
            provider_id: "dashscope".to_string(),
            provider_name: "阿里云百炼".to_string(),
            family: Some("qwen".to_string()),
            tier: ModelTier::Max,
            capabilities: ModelCapabilities {
                vision: true, tools: true, streaming: true,
                json_mode: true, function_calling: true, reasoning: true,
            },
            pricing: Some(ModelPricing {
                input_per_million: Some(20.0), output_per_million: Some(60.0),
                cache_read_per_million: None, cache_write_per_million: None,
                currency: "CNY".to_string(),
            }),
            limits: ModelLimits {
                context_length: Some(32768), max_output_tokens: Some(8192),
                requests_per_minute: None, tokens_per_minute: None,
            },
            status: ModelStatus::Active,
            release_date: Some("2024-12-01".to_string()),
            is_latest: true,
            description: Some("通义千问旗舰模型".to_string()),
            source: ModelSource::Local,
            created_at: now, updated_at: now,
        },
        EnhancedModelMetadata {
            id: "qwen-plus".to_string(),
            display_name: "通义千问 Plus".to_string(),
            provider_id: "dashscope".to_string(),
            provider_name: "阿里云百炼".to_string(),
            family: Some("qwen".to_string()),
            tier: ModelTier::Pro,
            capabilities: ModelCapabilities {
                vision: true, tools: true, streaming: true,
                json_mode: true, function_calling: true, reasoning: false,
            },
            pricing: Some(ModelPricing {
                input_per_million: Some(4.0), output_per_million: Some(12.0),
                cache_read_per_million: None, cache_write_per_million: None,
                currency: "CNY".to_string(),
            }),
            limits: ModelLimits {
                context_length: Some(131072), max_output_tokens: Some(8192),
                requests_per_minute: None, tokens_per_minute: None,
            },
            status: ModelStatus::Active,
            release_date: Some("2024-12-01".to_string()),
            is_latest: true,
            description: Some("通义千问增强版".to_string()),
            source: ModelSource::Local,
            created_at: now, updated_at: now,
        },
        EnhancedModelMetadata {
            id: "qwen-turbo".to_string(),
            display_name: "通义千问 Turbo".to_string(),
            provider_id: "dashscope".to_string(),
            provider_name: "阿里云百炼".to_string(),
            family: Some("qwen".to_string()),
            tier: ModelTier::Mini,
            capabilities: ModelCapabilities {
                vision: false, tools: true, streaming: true,
                json_mode: true, function_calling: true, reasoning: false,
            },
            pricing: Some(ModelPricing {
                input_per_million: Some(0.3), output_per_million: Some(0.6),
                cache_read_per_million: None, cache_write_per_million: None,
                currency: "CNY".to_string(),
            }),
            limits: ModelLimits {
                context_length: Some(131072), max_output_tokens: Some(8192),
                requests_per_minute: None, tokens_per_minute: None,
            },
            status: ModelStatus::Active,
            release_date: Some("2024-12-01".to_string()),
            is_latest: true,
            description: Some("通义千问快速版".to_string()),
            source: ModelSource::Local,
            created_at: now, updated_at: now,
        },
    ]
}

/// 智谱 GLM 系列模型
fn get_zhipu_models() -> Vec<EnhancedModelMetadata> {
    let now = chrono::Utc::now().timestamp();
    vec![
        EnhancedModelMetadata {
            id: "glm-4-plus".to_string(),
            display_name: "GLM-4 Plus".to_string(),
            provider_id: "zhipu".to_string(),
            provider_name: "智谱 AI".to_string(),
            family: Some("glm-4".to_string()),
            tier: ModelTier::Max,
            capabilities: ModelCapabilities {
                vision: true, tools: true, streaming: true,
                json_mode: true, function_calling: true, reasoning: true,
            },
            pricing: Some(ModelPricing {
                input_per_million: Some(50.0), output_per_million: Some(50.0),
                cache_read_per_million: None, cache_write_per_million: None,
                currency: "CNY".to_string(),
            }),
            limits: ModelLimits {
                context_length: Some(128000), max_output_tokens: Some(4096),
                requests_per_minute: None, tokens_per_minute: None,
            },
            status: ModelStatus::Active,
            release_date: Some("2024-08-01".to_string()),
            is_latest: true,
            description: Some("智谱旗舰模型".to_string()),
            source: ModelSource::Local,
            created_at: now, updated_at: now,
        },
        EnhancedModelMetadata {
            id: "glm-4-air".to_string(),
            display_name: "GLM-4 Air".to_string(),
            provider_id: "zhipu".to_string(),
            provider_name: "智谱 AI".to_string(),
            family: Some("glm-4".to_string()),
            tier: ModelTier::Pro,
            capabilities: ModelCapabilities {
                vision: false, tools: true, streaming: true,
                json_mode: true, function_calling: true, reasoning: false,
            },
            pricing: Some(ModelPricing {
                input_per_million: Some(1.0), output_per_million: Some(1.0),
                cache_read_per_million: None, cache_write_per_million: None,
                currency: "CNY".to_string(),
            }),
            limits: ModelLimits {
                context_length: Some(128000), max_output_tokens: Some(4096),
                requests_per_minute: None, tokens_per_minute: None,
            },
            status: ModelStatus::Active,
            release_date: Some("2024-06-01".to_string()),
            is_latest: true,
            description: Some("智谱高性价比模型".to_string()),
            source: ModelSource::Local,
            created_at: now, updated_at: now,
        },
        EnhancedModelMetadata {
            id: "glm-4-flash".to_string(),
            display_name: "GLM-4 Flash".to_string(),
            provider_id: "zhipu".to_string(),
            provider_name: "智谱 AI".to_string(),
            family: Some("glm-4".to_string()),
            tier: ModelTier::Mini,
            capabilities: ModelCapabilities {
                vision: false, tools: true, streaming: true,
                json_mode: true, function_calling: true, reasoning: false,
            },
            pricing: Some(ModelPricing {
                input_per_million: Some(0.1), output_per_million: Some(0.1),
                cache_read_per_million: None, cache_write_per_million: None,
                currency: "CNY".to_string(),
            }),
            limits: ModelLimits {
                context_length: Some(128000), max_output_tokens: Some(4096),
                requests_per_minute: None, tokens_per_minute: None,
            },
            status: ModelStatus::Active,
            release_date: Some("2024-06-01".to_string()),
            is_latest: true,
            description: Some("智谱快速模型".to_string()),
            source: ModelSource::Local,
            created_at: now, updated_at: now,
        },
    ]
}

/// 百川系列模型
fn get_baichuan_models() -> Vec<EnhancedModelMetadata> {
    let now = chrono::Utc::now().timestamp();
    vec![
        EnhancedModelMetadata {
            id: "Baichuan4".to_string(),
            display_name: "百川 4".to_string(),
            provider_id: "baichuan".to_string(),
            provider_name: "百川智能".to_string(),
            family: Some("baichuan".to_string()),
            tier: ModelTier::Max,
            capabilities: ModelCapabilities {
                vision: false, tools: true, streaming: true,
                json_mode: true, function_calling: true, reasoning: false,
            },
            pricing: Some(ModelPricing {
                input_per_million: Some(100.0), output_per_million: Some(100.0),
                cache_read_per_million: None, cache_write_per_million: None,
                currency: "CNY".to_string(),
            }),
            limits: ModelLimits {
                context_length: Some(32768), max_output_tokens: Some(4096),
                requests_per_minute: None, tokens_per_minute: None,
            },
            status: ModelStatus::Active,
            release_date: Some("2024-10-01".to_string()),
            is_latest: true,
            description: Some("百川旗舰模型".to_string()),
            source: ModelSource::Local,
            created_at: now, updated_at: now,
        },
        EnhancedModelMetadata {
            id: "Baichuan3-Turbo".to_string(),
            display_name: "百川 3 Turbo".to_string(),
            provider_id: "baichuan".to_string(),
            provider_name: "百川智能".to_string(),
            family: Some("baichuan".to_string()),
            tier: ModelTier::Pro,
            capabilities: ModelCapabilities {
                vision: false, tools: true, streaming: true,
                json_mode: true, function_calling: true, reasoning: false,
            },
            pricing: Some(ModelPricing {
                input_per_million: Some(12.0), output_per_million: Some(12.0),
                cache_read_per_million: None, cache_write_per_million: None,
                currency: "CNY".to_string(),
            }),
            limits: ModelLimits {
                context_length: Some(32768), max_output_tokens: Some(4096),
                requests_per_minute: None, tokens_per_minute: None,
            },
            status: ModelStatus::Active,
            release_date: Some("2024-06-01".to_string()),
            is_latest: true,
            description: Some("百川高性价比模型".to_string()),
            source: ModelSource::Local,
            created_at: now, updated_at: now,
        },
    ]
}

/// 月之暗面 Moonshot 系列模型
fn get_moonshot_models() -> Vec<EnhancedModelMetadata> {
    let now = chrono::Utc::now().timestamp();
    vec![
        EnhancedModelMetadata {
            id: "moonshot-v1-128k".to_string(),
            display_name: "Moonshot V1 128K".to_string(),
            provider_id: "moonshot".to_string(),
            provider_name: "月之暗面".to_string(),
            family: Some("moonshot".to_string()),
            tier: ModelTier::Pro,
            capabilities: ModelCapabilities {
                vision: false, tools: true, streaming: true,
                json_mode: true, function_calling: true, reasoning: false,
            },
            pricing: Some(ModelPricing {
                input_per_million: Some(60.0), output_per_million: Some(60.0),
                cache_read_per_million: None, cache_write_per_million: None,
                currency: "CNY".to_string(),
            }),
            limits: ModelLimits {
                context_length: Some(128000), max_output_tokens: Some(4096),
                requests_per_minute: None, tokens_per_minute: None,
            },
            status: ModelStatus::Active,
            release_date: Some("2024-03-01".to_string()),
            is_latest: true,
            description: Some("月之暗面长上下文模型".to_string()),
            source: ModelSource::Local,
            created_at: now, updated_at: now,
        },
        EnhancedModelMetadata {
            id: "moonshot-v1-32k".to_string(),
            display_name: "Moonshot V1 32K".to_string(),
            provider_id: "moonshot".to_string(),
            provider_name: "月之暗面".to_string(),
            family: Some("moonshot".to_string()),
            tier: ModelTier::Pro,
            capabilities: ModelCapabilities {
                vision: false, tools: true, streaming: true,
                json_mode: true, function_calling: true, reasoning: false,
            },
            pricing: Some(ModelPricing {
                input_per_million: Some(24.0), output_per_million: Some(24.0),
                cache_read_per_million: None, cache_write_per_million: None,
                currency: "CNY".to_string(),
            }),
            limits: ModelLimits {
                context_length: Some(32000), max_output_tokens: Some(4096),
                requests_per_minute: None, tokens_per_minute: None,
            },
            status: ModelStatus::Active,
            release_date: Some("2024-03-01".to_string()),
            is_latest: true,
            description: Some("月之暗面标准模型".to_string()),
            source: ModelSource::Local,
            created_at: now, updated_at: now,
        },
        EnhancedModelMetadata {
            id: "moonshot-v1-8k".to_string(),
            display_name: "Moonshot V1 8K".to_string(),
            provider_id: "moonshot".to_string(),
            provider_name: "月之暗面".to_string(),
            family: Some("moonshot".to_string()),
            tier: ModelTier::Mini,
            capabilities: ModelCapabilities {
                vision: false, tools: true, streaming: true,
                json_mode: true, function_calling: true, reasoning: false,
            },
            pricing: Some(ModelPricing {
                input_per_million: Some(12.0), output_per_million: Some(12.0),
                cache_read_per_million: None, cache_write_per_million: None,
                currency: "CNY".to_string(),
            }),
            limits: ModelLimits {
                context_length: Some(8000), max_output_tokens: Some(4096),
                requests_per_minute: None, tokens_per_minute: None,
            },
            status: ModelStatus::Active,
            release_date: Some("2024-03-01".to_string()),
            is_latest: true,
            description: Some("月之暗面快速模型".to_string()),
            source: ModelSource::Local,
            created_at: now, updated_at: now,
        },
    ]
}

/// DeepSeek 系列模型
fn get_deepseek_models() -> Vec<EnhancedModelMetadata> {
    let now = chrono::Utc::now().timestamp();
    vec![
        EnhancedModelMetadata {
            id: "deepseek-chat".to_string(),
            display_name: "DeepSeek Chat".to_string(),
            provider_id: "deepseek".to_string(),
            provider_name: "DeepSeek".to_string(),
            family: Some("deepseek".to_string()),
            tier: ModelTier::Pro,
            capabilities: ModelCapabilities {
                vision: false, tools: true, streaming: true,
                json_mode: true, function_calling: true, reasoning: false,
            },
            pricing: Some(ModelPricing {
                input_per_million: Some(1.0), output_per_million: Some(2.0),
                cache_read_per_million: Some(0.1), cache_write_per_million: Some(1.0),
                currency: "CNY".to_string(),
            }),
            limits: ModelLimits {
                context_length: Some(64000), max_output_tokens: Some(8192),
                requests_per_minute: None, tokens_per_minute: None,
            },
            status: ModelStatus::Active,
            release_date: Some("2024-12-01".to_string()),
            is_latest: true,
            description: Some("DeepSeek V3 对话模型".to_string()),
            source: ModelSource::Local,
            created_at: now, updated_at: now,
        },
        EnhancedModelMetadata {
            id: "deepseek-reasoner".to_string(),
            display_name: "DeepSeek Reasoner".to_string(),
            provider_id: "deepseek".to_string(),
            provider_name: "DeepSeek".to_string(),
            family: Some("deepseek".to_string()),
            tier: ModelTier::Max,
            capabilities: ModelCapabilities {
                vision: false, tools: false, streaming: true,
                json_mode: false, function_calling: false, reasoning: true,
            },
            pricing: Some(ModelPricing {
                input_per_million: Some(4.0), output_per_million: Some(16.0),
                cache_read_per_million: Some(0.4), cache_write_per_million: Some(4.0),
                currency: "CNY".to_string(),
            }),
            limits: ModelLimits {
                context_length: Some(64000), max_output_tokens: Some(8192),
                requests_per_minute: None, tokens_per_minute: None,
            },
            status: ModelStatus::Active,
            release_date: Some("2025-01-01".to_string()),
            is_latest: true,
            description: Some("DeepSeek R1 推理模型".to_string()),
            source: ModelSource::Local,
            created_at: now, updated_at: now,
        },
        EnhancedModelMetadata {
            id: "deepseek-coder".to_string(),
            display_name: "DeepSeek Coder".to_string(),
            provider_id: "deepseek".to_string(),
            provider_name: "DeepSeek".to_string(),
            family: Some("deepseek-coder".to_string()),
            tier: ModelTier::Pro,
            capabilities: ModelCapabilities {
                vision: false, tools: true, streaming: true,
                json_mode: true, function_calling: true, reasoning: false,
            },
            pricing: Some(ModelPricing {
                input_per_million: Some(1.0), output_per_million: Some(2.0),
                cache_read_per_million: Some(0.1), cache_write_per_million: Some(1.0),
                currency: "CNY".to_string(),
            }),
            limits: ModelLimits {
                context_length: Some(64000), max_output_tokens: Some(8192),
                requests_per_minute: None, tokens_per_minute: None,
            },
            status: ModelStatus::Active,
            release_date: Some("2024-06-01".to_string()),
            is_latest: true,
            description: Some("DeepSeek 代码模型".to_string()),
            source: ModelSource::Local,
            created_at: now, updated_at: now,
        },
    ]
}

/// 字节豆包系列模型
fn get_doubao_models() -> Vec<EnhancedModelMetadata> {
    let now = chrono::Utc::now().timestamp();
    vec![
        EnhancedModelMetadata {
            id: "doubao-pro-256k".to_string(),
            display_name: "豆包 Pro 256K".to_string(),
            provider_id: "doubao".to_string(),
            provider_name: "字节跳动".to_string(),
            family: Some("doubao".to_string()),
            tier: ModelTier::Max,
            capabilities: ModelCapabilities {
                vision: true, tools: true, streaming: true,
                json_mode: true, function_calling: true, reasoning: false,
            },
            pricing: Some(ModelPricing {
                input_per_million: Some(5.0), output_per_million: Some(9.0),
                cache_read_per_million: None, cache_write_per_million: None,
                currency: "CNY".to_string(),
            }),
            limits: ModelLimits {
                context_length: Some(256000), max_output_tokens: Some(4096),
                requests_per_minute: None, tokens_per_minute: None,
            },
            status: ModelStatus::Active,
            release_date: Some("2024-10-01".to_string()),
            is_latest: true,
            description: Some("豆包旗舰长上下文模型".to_string()),
            source: ModelSource::Local,
            created_at: now, updated_at: now,
        },
        EnhancedModelMetadata {
            id: "doubao-pro-32k".to_string(),
            display_name: "豆包 Pro 32K".to_string(),
            provider_id: "doubao".to_string(),
            provider_name: "字节跳动".to_string(),
            family: Some("doubao".to_string()),
            tier: ModelTier::Pro,
            capabilities: ModelCapabilities {
                vision: true, tools: true, streaming: true,
                json_mode: true, function_calling: true, reasoning: false,
            },
            pricing: Some(ModelPricing {
                input_per_million: Some(0.8), output_per_million: Some(2.0),
                cache_read_per_million: None, cache_write_per_million: None,
                currency: "CNY".to_string(),
            }),
            limits: ModelLimits {
                context_length: Some(32000), max_output_tokens: Some(4096),
                requests_per_minute: None, tokens_per_minute: None,
            },
            status: ModelStatus::Active,
            release_date: Some("2024-06-01".to_string()),
            is_latest: true,
            description: Some("豆包标准模型".to_string()),
            source: ModelSource::Local,
            created_at: now, updated_at: now,
        },
        EnhancedModelMetadata {
            id: "doubao-lite-32k".to_string(),
            display_name: "豆包 Lite 32K".to_string(),
            provider_id: "doubao".to_string(),
            provider_name: "字节跳动".to_string(),
            family: Some("doubao".to_string()),
            tier: ModelTier::Mini,
            capabilities: ModelCapabilities {
                vision: false, tools: true, streaming: true,
                json_mode: true, function_calling: true, reasoning: false,
            },
            pricing: Some(ModelPricing {
                input_per_million: Some(0.3), output_per_million: Some(0.6),
                cache_read_per_million: None, cache_write_per_million: None,
                currency: "CNY".to_string(),
            }),
            limits: ModelLimits {
                context_length: Some(32000), max_output_tokens: Some(4096),
                requests_per_minute: None, tokens_per_minute: None,
            },
            status: ModelStatus::Active,
            release_date: Some("2024-06-01".to_string()),
            is_latest: true,
            description: Some("豆包轻量模型".to_string()),
            source: ModelSource::Local,
            created_at: now, updated_at: now,
        },
    ]
}

/// MiniMax 系列模型
fn get_minimax_models() -> Vec<EnhancedModelMetadata> {
    let now = chrono::Utc::now().timestamp();
    vec![
        EnhancedModelMetadata {
            id: "abab6.5s-chat".to_string(),
            display_name: "MiniMax abab6.5s".to_string(),
            provider_id: "minimax".to_string(),
            provider_name: "MiniMax".to_string(),
            family: Some("abab".to_string()),
            tier: ModelTier::Pro,
            capabilities: ModelCapabilities {
                vision: false, tools: true, streaming: true,
                json_mode: true, function_calling: true, reasoning: false,
            },
            pricing: Some(ModelPricing {
                input_per_million: Some(1.0), output_per_million: Some(1.0),
                cache_read_per_million: None, cache_write_per_million: None,
                currency: "CNY".to_string(),
            }),
            limits: ModelLimits {
                context_length: Some(245760), max_output_tokens: Some(8192),
                requests_per_minute: None, tokens_per_minute: None,
            },
            status: ModelStatus::Active,
            release_date: Some("2024-06-01".to_string()),
            is_latest: true,
            description: Some("MiniMax 长上下文模型".to_string()),
            source: ModelSource::Local,
            created_at: now, updated_at: now,
        },
    ]
}

/// 零一万物 Yi 系列模型
fn get_yi_models() -> Vec<EnhancedModelMetadata> {
    let now = chrono::Utc::now().timestamp();
    vec![
        EnhancedModelMetadata {
            id: "yi-large".to_string(),
            display_name: "Yi Large".to_string(),
            provider_id: "yi".to_string(),
            provider_name: "零一万物".to_string(),
            family: Some("yi".to_string()),
            tier: ModelTier::Max,
            capabilities: ModelCapabilities {
                vision: false, tools: true, streaming: true,
                json_mode: true, function_calling: true, reasoning: false,
            },
            pricing: Some(ModelPricing {
                input_per_million: Some(20.0), output_per_million: Some(20.0),
                cache_read_per_million: None, cache_write_per_million: None,
                currency: "CNY".to_string(),
            }),
            limits: ModelLimits {
                context_length: Some(32768), max_output_tokens: Some(4096),
                requests_per_minute: None, tokens_per_minute: None,
            },
            status: ModelStatus::Active,
            release_date: Some("2024-05-01".to_string()),
            is_latest: true,
            description: Some("零一万物旗舰模型".to_string()),
            source: ModelSource::Local,
            created_at: now, updated_at: now,
        },
        EnhancedModelMetadata {
            id: "yi-medium".to_string(),
            display_name: "Yi Medium".to_string(),
            provider_id: "yi".to_string(),
            provider_name: "零一万物".to_string(),
            family: Some("yi".to_string()),
            tier: ModelTier::Pro,
            capabilities: ModelCapabilities {
                vision: false, tools: true, streaming: true,
                json_mode: true, function_calling: true, reasoning: false,
            },
            pricing: Some(ModelPricing {
                input_per_million: Some(2.5), output_per_million: Some(2.5),
                cache_read_per_million: None, cache_write_per_million: None,
                currency: "CNY".to_string(),
            }),
            limits: ModelLimits {
                context_length: Some(16384), max_output_tokens: Some(4096),
                requests_per_minute: None, tokens_per_minute: None,
            },
            status: ModelStatus::Active,
            release_date: Some("2024-05-01".to_string()),
            is_latest: true,
            description: Some("零一万物标准模型".to_string()),
            source: ModelSource::Local,
            created_at: now, updated_at: now,
        },
        EnhancedModelMetadata {
            id: "yi-spark".to_string(),
            display_name: "Yi Spark".to_string(),
            provider_id: "yi".to_string(),
            provider_name: "零一万物".to_string(),
            family: Some("yi".to_string()),
            tier: ModelTier::Mini,
            capabilities: ModelCapabilities {
                vision: false, tools: true, streaming: true,
                json_mode: true, function_calling: true, reasoning: false,
            },
            pricing: Some(ModelPricing {
                input_per_million: Some(1.0), output_per_million: Some(1.0),
                cache_read_per_million: None, cache_write_per_million: None,
                currency: "CNY".to_string(),
            }),
            limits: ModelLimits {
                context_length: Some(16384), max_output_tokens: Some(4096),
                requests_per_minute: None, tokens_per_minute: None,
            },
            status: ModelStatus::Active,
            release_date: Some("2024-05-01".to_string()),
            is_latest: true,
            description: Some("零一万物快速模型".to_string()),
            source: ModelSource::Local,
            created_at: now, updated_at: now,
        },
    ]
}

/// 阶跃星辰 Step 系列模型
fn get_stepfun_models() -> Vec<EnhancedModelMetadata> {
    let now = chrono::Utc::now().timestamp();
    vec![
        EnhancedModelMetadata {
            id: "step-2-16k".to_string(),
            display_name: "Step 2 16K".to_string(),
            provider_id: "stepfun".to_string(),
            provider_name: "阶跃星辰".to_string(),
            family: Some("step".to_string()),
            tier: ModelTier::Max,
            capabilities: ModelCapabilities {
                vision: true, tools: true, streaming: true,
                json_mode: true, function_calling: true, reasoning: false,
            },
            pricing: Some(ModelPricing {
                input_per_million: Some(38.0), output_per_million: Some(120.0),
                cache_read_per_million: None, cache_write_per_million: None,
                currency: "CNY".to_string(),
            }),
            limits: ModelLimits {
                context_length: Some(16384), max_output_tokens: Some(4096),
                requests_per_minute: None, tokens_per_minute: None,
            },
            status: ModelStatus::Active,
            release_date: Some("2024-09-01".to_string()),
            is_latest: true,
            description: Some("阶跃星辰旗舰模型".to_string()),
            source: ModelSource::Local,
            created_at: now, updated_at: now,
        },
        EnhancedModelMetadata {
            id: "step-1-128k".to_string(),
            display_name: "Step 1 128K".to_string(),
            provider_id: "stepfun".to_string(),
            provider_name: "阶跃星辰".to_string(),
            family: Some("step".to_string()),
            tier: ModelTier::Pro,
            capabilities: ModelCapabilities {
                vision: false, tools: true, streaming: true,
                json_mode: true, function_calling: true, reasoning: false,
            },
            pricing: Some(ModelPricing {
                input_per_million: Some(40.0), output_per_million: Some(100.0),
                cache_read_per_million: None, cache_write_per_million: None,
                currency: "CNY".to_string(),
            }),
            limits: ModelLimits {
                context_length: Some(128000), max_output_tokens: Some(4096),
                requests_per_minute: None, tokens_per_minute: None,
            },
            status: ModelStatus::Active,
            release_date: Some("2024-06-01".to_string()),
            is_latest: true,
            description: Some("阶跃星辰长上下文模型".to_string()),
            source: ModelSource::Local,
            created_at: now, updated_at: now,
        },
        EnhancedModelMetadata {
            id: "step-1-flash".to_string(),
            display_name: "Step 1 Flash".to_string(),
            provider_id: "stepfun".to_string(),
            provider_name: "阶跃星辰".to_string(),
            family: Some("step".to_string()),
            tier: ModelTier::Mini,
            capabilities: ModelCapabilities {
                vision: false, tools: true, streaming: true,
                json_mode: true, function_calling: true, reasoning: false,
            },
            pricing: Some(ModelPricing {
                input_per_million: Some(1.0), output_per_million: Some(4.0),
                cache_read_per_million: None, cache_write_per_million: None,
                currency: "CNY".to_string(),
            }),
            limits: ModelLimits {
                context_length: Some(8192), max_output_tokens: Some(4096),
                requests_per_minute: None, tokens_per_minute: None,
            },
            status: ModelStatus::Active,
            release_date: Some("2024-06-01".to_string()),
            is_latest: true,
            description: Some("阶跃星辰快速模型".to_string()),
            source: ModelSource::Local,
            created_at: now, updated_at: now,
        },
    ]
}
