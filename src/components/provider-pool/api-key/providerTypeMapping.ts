/**
 * @file Provider 类型映射工具
 * @description Provider 类型到 model_registry provider_id 的映射
 * @module components/provider-pool/api-key/providerTypeMapping
 */

/**
 * Provider 类型到 model_registry provider_id 的映射
 */
const PROVIDER_TYPE_TO_REGISTRY_ID: Record<string, string> = {
  anthropic: "anthropic",
  openai: "openai",
  "openai-response": "openai",
  gemini: "google",
  "azure-openai": "openai",
  vertexai: "google",
  "aws-bedrock": "anthropic",
  ollama: "ollama",
  "new-api": "custom",
  gateway: "custom",
};

/**
 * 将 Provider 类型转换为 model_registry 的 provider_id
 */
export function mapProviderTypeToRegistryId(providerType: string): string {
  return PROVIDER_TYPE_TO_REGISTRY_ID[providerType] || providerType;
}
