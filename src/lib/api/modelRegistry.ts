/**
 * 模型注册表 API
 *
 * 提供与后端 ModelRegistryService 交互的 API
 */

import { invoke } from "@tauri-apps/api/core";
import type {
  EnhancedModelMetadata,
  ModelSyncState,
  ModelTier,
  UserModelPreference,
} from "@/lib/types/modelRegistry";

/**
 * 获取所有模型
 */
export async function getModelRegistry(): Promise<EnhancedModelMetadata[]> {
  return invoke("get_model_registry");
}

/**
 * 刷新模型注册表（从 models.dev 获取最新数据）
 */
export async function refreshModelRegistry(): Promise<void> {
  return invoke("refresh_model_registry");
}

/**
 * 搜索模型
 * @param query 搜索关键词
 * @param limit 返回数量限制
 */
export async function searchModels(
  query: string,
  limit?: number,
): Promise<EnhancedModelMetadata[]> {
  return invoke("search_models", { query, limit });
}

/**
 * 获取用户模型偏好
 */
export async function getModelPreferences(): Promise<UserModelPreference[]> {
  return invoke("get_model_preferences");
}

/**
 * 切换模型收藏状态
 * @param modelId 模型 ID
 * @returns 新的收藏状态
 */
export async function toggleModelFavorite(modelId: string): Promise<boolean> {
  return invoke("toggle_model_favorite", { modelId });
}

/**
 * 隐藏模型
 * @param modelId 模型 ID
 */
export async function hideModel(modelId: string): Promise<void> {
  return invoke("hide_model", { modelId });
}

/**
 * 记录模型使用
 * @param modelId 模型 ID
 */
export async function recordModelUsage(modelId: string): Promise<void> {
  return invoke("record_model_usage", { modelId });
}

/**
 * 获取模型同步状态
 */
export async function getModelSyncState(): Promise<ModelSyncState> {
  return invoke("get_model_sync_state");
}

/**
 * 按 Provider 获取模型
 * @param providerId Provider ID
 */
export async function getModelsForProvider(
  providerId: string,
): Promise<EnhancedModelMetadata[]> {
  return invoke("get_models_for_provider", { providerId });
}

/**
 * 按服务等级获取模型
 * @param tier 服务等级
 */
export async function getModelsByTier(
  tier: ModelTier,
): Promise<EnhancedModelMetadata[]> {
  return invoke("get_models_by_tier", { tier });
}

/**
 * 模型注册表 API 对象
 */
export const modelRegistryApi = {
  getModelRegistry,
  refreshModelRegistry,
  searchModels,
  getModelPreferences,
  toggleModelFavorite,
  hideModel,
  recordModelUsage,
  getModelSyncState,
  getModelsForProvider,
  getModelsByTier,
};
