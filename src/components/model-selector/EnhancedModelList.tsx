/**
 * 增强版模型列表组件
 *
 * 使用 model_registry 数据，支持搜索、收藏、分组等功能
 */

import { useState, useMemo } from "react";
import {
  Check,
  AlertCircle,
  Loader2,
  Star,
  Search,
  ChevronDown,
  ChevronRight,
  Eye,
  Wrench,
  Brain,
  DollarSign,
} from "lucide-react";
import { cn } from "@/lib/utils";
import type { EnhancedModelMetadata } from "@/lib/types/modelRegistry";

interface EnhancedModelListProps {
  /** 模型列表 */
  models: EnhancedModelMetadata[];
  /** 选中的模型 ID */
  selectedModelId?: string;
  /** 选择模型回调 */
  onSelectModel?: (model: EnhancedModelMetadata) => void;
  /** 收藏模型回调 */
  onToggleFavorite?: (modelId: string) => void;
  /** 收藏的模型 ID 集合 */
  favorites?: Set<string>;
  /** 是否加载中 */
  loading?: boolean;
  /** 错误信息 */
  error?: string | null;
  /** 是否按 Provider 分组 */
  groupByProvider?: boolean;
  /** 是否显示搜索框 */
  showSearch?: boolean;
  /** 是否显示定价信息 */
  showPricing?: boolean;
  /** 自定义类名 */
  className?: string;
}

export function EnhancedModelList({
  models,
  selectedModelId,
  onSelectModel,
  onToggleFavorite,
  favorites = new Set(),
  loading = false,
  error = null,
  groupByProvider = true,
  showSearch = true,
  showPricing = false,
  className,
}: EnhancedModelListProps) {
  const [searchQuery, setSearchQuery] = useState("");
  const [expandedGroups, setExpandedGroups] = useState<Set<string>>(
    new Set(["favorites"]),
  );

  // 过滤模型
  const filteredModels = useMemo(() => {
    if (!searchQuery.trim()) return models;

    const query = searchQuery.toLowerCase();
    return models.filter(
      (m) =>
        m.id.toLowerCase().includes(query) ||
        m.display_name.toLowerCase().includes(query) ||
        m.provider_name.toLowerCase().includes(query) ||
        m.family?.toLowerCase().includes(query),
    );
  }, [models, searchQuery]);

  // 按 Provider 分组
  const groupedModels = useMemo(() => {
    if (!groupByProvider) {
      return { all: filteredModels };
    }

    const groups: Record<string, EnhancedModelMetadata[]> = {};

    // 先添加收藏组
    const favoriteModels = filteredModels.filter((m) => favorites.has(m.id));
    if (favoriteModels.length > 0) {
      groups["favorites"] = favoriteModels;
    }

    // 按 Provider 分组
    for (const model of filteredModels) {
      if (!groups[model.provider_id]) {
        groups[model.provider_id] = [];
      }
      groups[model.provider_id].push(model);
    }

    return groups;
  }, [filteredModels, groupByProvider, favorites]);

  // 切换分组展开状态
  const toggleGroup = (groupId: string) => {
    setExpandedGroups((prev) => {
      const next = new Set(prev);
      if (next.has(groupId)) {
        next.delete(groupId);
      } else {
        next.add(groupId);
      }
      return next;
    });
  };

  if (loading) {
    return (
      <div className={cn("flex items-center justify-center py-8", className)}>
        <Loader2 className="h-6 w-6 animate-spin text-muted-foreground" />
        <span className="ml-2 text-muted-foreground">加载模型列表...</span>
      </div>
    );
  }

  if (error) {
    return (
      <div
        className={cn(
          "flex items-center justify-center py-8 text-destructive",
          className,
        )}
      >
        <AlertCircle className="h-5 w-5 mr-2" />
        <span>{error}</span>
      </div>
    );
  }

  if (models.length === 0) {
    return (
      <div className={cn("text-center py-8 text-muted-foreground", className)}>
        <p>暂无可用模型</p>
        <p className="text-sm mt-1">请等待模型数据加载</p>
      </div>
    );
  }

  return (
    <div className={cn("space-y-3", className)}>
      {/* 搜索框 */}
      {showSearch && (
        <div className="relative">
          <Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
          <input
            type="text"
            placeholder="搜索模型..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="w-full rounded-lg border bg-background pl-10 pr-4 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-primary/20"
          />
        </div>
      )}

      {/* 模型列表 */}
      <div className="space-y-2">
        {Object.entries(groupedModels).map(([groupId, groupModels]) => {
          const isExpanded = expandedGroups.has(groupId);
          const groupName = getGroupName(groupId, groupModels[0]);

          return (
            <div key={groupId} className="border rounded-lg overflow-hidden">
              {/* 分组头部 */}
              {groupByProvider && (
                <button
                  type="button"
                  onClick={() => toggleGroup(groupId)}
                  className="w-full flex items-center justify-between px-3 py-2 bg-muted/50 hover:bg-muted transition-colors"
                >
                  <div className="flex items-center gap-2">
                    {isExpanded ? (
                      <ChevronDown className="h-4 w-4" />
                    ) : (
                      <ChevronRight className="h-4 w-4" />
                    )}
                    {groupId === "favorites" && (
                      <Star className="h-4 w-4 text-yellow-500 fill-yellow-500" />
                    )}
                    <span className="font-medium text-sm">{groupName}</span>
                  </div>
                  <span className="text-xs text-muted-foreground">
                    {groupModels.length} 个模型
                  </span>
                </button>
              )}

              {/* 模型列表 */}
              {(!groupByProvider || isExpanded) && (
                <div className="divide-y">
                  {groupModels.map((model) => (
                    <ModelItem
                      key={model.id}
                      model={model}
                      isSelected={model.id === selectedModelId}
                      isFavorite={favorites.has(model.id)}
                      onSelect={() => onSelectModel?.(model)}
                      onToggleFavorite={() => onToggleFavorite?.(model.id)}
                      showPricing={showPricing}
                    />
                  ))}
                </div>
              )}
            </div>
          );
        })}
      </div>

      {/* 无搜索结果 */}
      {filteredModels.length === 0 && searchQuery && (
        <div className="text-center py-8 text-muted-foreground">
          <p>未找到匹配的模型</p>
          <p className="text-sm mt-1">尝试其他搜索词</p>
        </div>
      )}
    </div>
  );
}

/** 单个模型项 */
function ModelItem({
  model,
  isSelected,
  isFavorite,
  onSelect,
  onToggleFavorite,
  showPricing,
}: {
  model: EnhancedModelMetadata;
  isSelected: boolean;
  isFavorite: boolean;
  onSelect: () => void;
  onToggleFavorite: () => void;
  showPricing: boolean;
}) {
  return (
    <div
      className={cn(
        "flex items-center justify-between px-3 py-2.5 hover:bg-muted/30 transition-colors cursor-pointer",
        isSelected && "bg-primary/5",
      )}
      onClick={onSelect}
    >
      <div className="flex items-center gap-3 flex-1 min-w-0">
        {/* 选中指示器 */}
        <div
          className={cn(
            "w-4 h-4 rounded-full border-2 flex-shrink-0 flex items-center justify-center",
            isSelected
              ? "border-primary bg-primary"
              : "border-muted-foreground",
          )}
        >
          {isSelected && <Check className="h-3 w-3 text-primary-foreground" />}
        </div>

        {/* 模型信息 */}
        <div className="flex-1 min-w-0">
          <div className="flex items-center gap-2">
            <span className="font-medium text-sm truncate">
              {model.display_name}
            </span>
            {model.is_latest && (
              <span className="text-xs px-1.5 py-0.5 rounded bg-primary/10 text-primary">
                最新
              </span>
            )}
            <TierBadge tier={model.tier} />
          </div>
          <div className="flex items-center gap-2 text-xs text-muted-foreground">
            <span>{model.id}</span>
            {model.limits.context_length && (
              <>
                <span>·</span>
                <span>{formatContextLength(model.limits.context_length)}</span>
              </>
            )}
          </div>
        </div>
      </div>

      {/* 能力标签和操作 */}
      <div className="flex items-center gap-2 flex-shrink-0">
        {/* 能力图标 */}
        <div className="flex items-center gap-1">
          {model.capabilities.vision && (
            <span title="支持视觉">
              <Eye className="h-3.5 w-3.5 text-blue-500" />
            </span>
          )}
          {model.capabilities.tools && (
            <span title="支持工具">
              <Wrench className="h-3.5 w-3.5 text-green-500" />
            </span>
          )}
          {model.capabilities.reasoning && (
            <span title="支持推理">
              <Brain className="h-3.5 w-3.5 text-purple-500" />
            </span>
          )}
        </div>

        {/* 定价 */}
        {showPricing && model.pricing && (
          <div className="flex items-center gap-1 text-xs text-muted-foreground">
            <DollarSign className="h-3 w-3" />
            <span>{model.pricing.input_per_million?.toFixed(2) || "?"}</span>
          </div>
        )}

        {/* 收藏按钮 */}
        <button
          type="button"
          onClick={(e) => {
            e.stopPropagation();
            onToggleFavorite();
          }}
          className="p-1 rounded hover:bg-muted transition-colors"
          title={isFavorite ? "取消收藏" : "收藏"}
        >
          <Star
            className={cn(
              "h-4 w-4",
              isFavorite
                ? "text-yellow-500 fill-yellow-500"
                : "text-muted-foreground",
            )}
          />
        </button>
      </div>
    </div>
  );
}

/** 服务等级徽章 */
function TierBadge({ tier }: { tier: string }) {
  const config = {
    mini: {
      label: "Mini",
      color:
        "bg-green-100 text-green-700 dark:bg-green-900 dark:text-green-300",
    },
    pro: {
      label: "Pro",
      color: "bg-blue-100 text-blue-700 dark:bg-blue-900 dark:text-blue-300",
    },
    max: {
      label: "Max",
      color:
        "bg-purple-100 text-purple-700 dark:bg-purple-900 dark:text-purple-300",
    },
  }[tier] || { label: tier, color: "bg-gray-100 text-gray-700" };

  return (
    <span className={cn("text-xs px-1.5 py-0.5 rounded", config.color)}>
      {config.label}
    </span>
  );
}

/** 获取分组名称 */
function getGroupName(
  groupId: string,
  firstModel: EnhancedModelMetadata,
): string {
  if (groupId === "favorites") return "收藏";
  if (groupId === "all") return "全部模型";
  return firstModel?.provider_name || groupId;
}

/** 格式化上下文长度 */
function formatContextLength(length: number): string {
  if (length >= 1000000) {
    return `${(length / 1000000).toFixed(1)}M`;
  }
  if (length >= 1000) {
    return `${(length / 1000).toFixed(0)}K`;
  }
  return String(length);
}
