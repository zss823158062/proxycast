/**
 * @file ModelRegistryTab 组件
 * @description 模型库 Tab，显示所有可用模型
 * @module components/provider-pool/ModelRegistryTab
 */

import { EnhancedModelsTab } from "@/components/api-server/EnhancedModelsTab";

/**
 * 模型库 Tab 组件
 *
 * 复用 API Server 的 EnhancedModelsTab 组件
 */
export function ModelRegistryTab() {
  return (
    <div className="min-h-[400px]" data-testid="model-registry-section">
      <EnhancedModelsTab />
    </div>
  );
}

export default ModelRegistryTab;
