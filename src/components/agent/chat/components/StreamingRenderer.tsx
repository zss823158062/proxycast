/**
 * 流式消息渲染组件
 *
 * 参考 Goose UI 设计，支持思考内容、工具调用和实时 Markdown 渲染
 * Requirements: 9.3, 9.4
 */

import React, { memo, useMemo, useState, useEffect, useRef } from "react";
import { cn } from "@/lib/utils";
import { ChevronDown, Lightbulb } from "lucide-react";
import { MarkdownRenderer } from "./MarkdownRenderer";
import { ToolCallList, ToolCallItem } from "./ToolCallDisplay";
import type { ToolCallState } from "@/lib/api/agent";
import type { ContentPart } from "../types";

// ============ 思考内容组件 ============

interface ThinkingBlockProps {
  content: string;
  defaultExpanded?: boolean;
}

const ThinkingBlock: React.FC<ThinkingBlockProps> = ({
  content,
  defaultExpanded = false,
}) => {
  const [expanded, setExpanded] = React.useState(defaultExpanded);

  if (!content) return null;

  return (
    <details
      className="bg-muted/50 border border-border rounded-lg overflow-hidden mb-3"
      open={expanded}
      onToggle={(e) => setExpanded((e.target as HTMLDetailsElement).open)}
    >
      <summary className="cursor-pointer px-3 py-2 text-sm text-muted-foreground select-none flex items-center gap-2 hover:bg-muted/70 transition-colors">
        <Lightbulb className="w-4 h-4 text-yellow-500" />
        <span className="flex-1">思考过程</span>
        <ChevronDown
          className={cn(
            "w-4 h-4 transition-transform duration-200",
            expanded && "rotate-180",
          )}
        />
      </summary>
      <div className="px-3 py-2 border-t border-border bg-background/50">
        <MarkdownRenderer content={content} />
      </div>
    </details>
  );
};

// ============ 流式光标 ============

const StreamingCursor: React.FC = () => (
  <span
    className="inline-block w-0.5 h-[1em] bg-primary ml-0.5 align-text-bottom animate-pulse"
    style={{ animationDuration: "1s" }}
  />
);

// ============ 流式文本组件（逐字符动画） ============

interface StreamingTextProps {
  /** 目标文本（完整内容） */
  text: string;
  /** 是否正在流式输出 */
  isStreaming: boolean;
  /** 是否显示光标 */
  showCursor?: boolean;
  /** 每个字符的渲染间隔（毫秒），默认 12ms */
  charInterval?: number;
}

/**
 * 流式文本组件
 *
 * 实现逐字符平滑显示效果，类似 ChatGPT/Claude 的打字机效果。
 * 当流式结束时，立即显示完整文本。
 */
const StreamingText: React.FC<StreamingTextProps> = memo(
  ({ text, isStreaming, showCursor = true, charInterval = 12 }) => {
    const [displayText, setDisplayText] = useState("");
    const displayIndexRef = useRef(0);
    const animationRef = useRef<number | null>(null);
    const prevTextRef = useRef("");

    useEffect(() => {
      // 如果不是流式输出，直接显示完整文本
      if (!isStreaming) {
        setDisplayText(text);
        displayIndexRef.current = text.length;
        prevTextRef.current = text;
        if (animationRef.current) {
          cancelAnimationFrame(animationRef.current);
          animationRef.current = null;
        }
        return;
      }

      // 检测文本是否有新增
      if (text.length <= prevTextRef.current.length) {
        prevTextRef.current = text;
        return;
      }

      prevTextRef.current = text;

      // 如果已经有动画在运行，让它继续
      if (animationRef.current !== null) {
        return;
      }

      let lastTime = 0;

      const animate = (currentTime: number) => {
        if (!lastTime) lastTime = currentTime;
        const elapsed = currentTime - lastTime;

        if (elapsed >= charInterval) {
          // 计算这一帧应该显示多少个字符
          const charsToAdd = Math.max(1, Math.floor(elapsed / charInterval));
          const newIndex = Math.min(
            displayIndexRef.current + charsToAdd,
            text.length,
          );

          if (newIndex > displayIndexRef.current) {
            displayIndexRef.current = newIndex;
            setDisplayText(text.slice(0, newIndex));
          }

          lastTime = currentTime;
        }

        // 继续动画直到追上目标
        if (displayIndexRef.current < text.length) {
          animationRef.current = requestAnimationFrame(animate);
        } else {
          animationRef.current = null;
        }
      };

      animationRef.current = requestAnimationFrame(animate);

      return () => {
        if (animationRef.current) {
          cancelAnimationFrame(animationRef.current);
          animationRef.current = null;
        }
      };
    }, [text, isStreaming, charInterval]);

    // 组件卸载时清理
    useEffect(() => {
      return () => {
        if (animationRef.current) {
          cancelAnimationFrame(animationRef.current);
        }
      };
    }, []);

    const shouldShowCursor =
      isStreaming && showCursor && displayIndexRef.current < text.length;

    return (
      <div className="relative">
        <MarkdownRenderer content={displayText} />
        {shouldShowCursor && <StreamingCursor />}
      </div>
    );
  },
);

StreamingText.displayName = "StreamingText";

// ============ 思考内容解析 ============

interface ParsedContent {
  visibleText: string;
  thinkingText: string | null;
}

const parseThinkingContent = (text: string): ParsedContent => {
  // 支持 <think>...</think> 和 <thinking>...</thinking> 标签
  const thinkRegex = /<think(?:ing)?>([\s\S]*?)<\/think(?:ing)?>/gi;
  let thinkingText: string | null = null;
  let visibleText = text;

  const matches = text.matchAll(thinkRegex);
  const thinkingParts: string[] = [];

  for (const match of matches) {
    thinkingParts.push(match[1].trim());
    visibleText = visibleText.replace(match[0], "");
  }

  if (thinkingParts.length > 0) {
    thinkingText = thinkingParts.join("\n\n");
  }

  return {
    visibleText: visibleText.trim(),
    thinkingText,
  };
};

// ============ 主组件 ============

interface StreamingRendererProps {
  /** 文本内容（向后兼容） */
  content: string;
  /** 是否正在流式输出 */
  isStreaming?: boolean;
  /** 工具调用列表（向后兼容） */
  toolCalls?: ToolCallState[];
  /** 是否显示光标 */
  showCursor?: boolean;
  /** 思考内容（可选，如果不提供则从 content 中解析） */
  thinkingContent?: string;
  /**
   * 交错内容列表（按事件到达顺序排列）
   * 如果存在且非空，按顺序渲染
   * 否则回退到 content + toolCalls 渲染方式
   */
  contentParts?: ContentPart[];
}

/**
 * 流式消息渲染组件
 *
 * 支持：
 * - 思考内容折叠显示（<think> 或 <thinking> 标签）
 * - 工具调用状态和结果显示
 * - 实时 Markdown 渲染
 * - 流式光标
 * - **交错内容显示**（文本和工具调用按事件顺序交错）
 */
export const StreamingRenderer: React.FC<StreamingRendererProps> = memo(
  ({
    content,
    isStreaming = false,
    toolCalls,
    showCursor = true,
    thinkingContent: externalThinking,
    contentParts,
  }) => {
    // 判断是否使用交错显示模式
    const useInterleavedMode = contentParts && contentParts.length > 0;

    // 解析思考内容（仅在非交错模式下使用）
    const { visibleText, thinkingText } = useMemo(
      () => parseThinkingContent(content),
      [content],
    );

    // 使用外部提供的思考内容或解析出的内容
    const finalThinking = externalThinking || thinkingText;

    // 判断是否有正在执行的工具
    const hasRunningTools = useMemo(() => {
      if (useInterleavedMode) {
        return contentParts.some(
          (part) =>
            part.type === "tool_use" && part.toolCall.status === "running",
        );
      }
      return toolCalls?.some((tc) => tc.status === "running") ?? false;
    }, [contentParts, toolCalls, useInterleavedMode]);

    // 判断是否显示光标
    const shouldShowCursor = isStreaming && showCursor && !hasRunningTools;

    // 判断是否有可见内容
    const hasVisibleContent = useInterleavedMode
      ? contentParts.some(
          (part) => part.type === "text" && part.text.length > 0,
        )
      : visibleText.length > 0;

    // 交错显示模式：按顺序渲染 contentParts
    if (useInterleavedMode) {
      return (
        <div className="flex flex-col gap-2">
          {/* 思考内容 - 显示在最前面 */}
          {finalThinking && (
            <ThinkingBlock
              content={finalThinking}
              defaultExpanded={isStreaming}
            />
          )}

          {/* 交错内容 */}
          {contentParts.map((part, index) => {
            if (part.type === "text") {
              // 解析并渲染文本（可能包含 thinking 标签）
              const { visibleText: partVisible } = parseThinkingContent(
                part.text,
              );
              if (!partVisible) return null;

              const isLastPart = index === contentParts.length - 1;
              // 使用 StreamingText 组件实现逐字符动画
              return (
                <StreamingText
                  key={`text-${index}`}
                  text={partVisible}
                  isStreaming={isStreaming && isLastPart}
                  showCursor={shouldShowCursor && isLastPart}
                />
              );
            } else if (part.type === "tool_use") {
              // 渲染单个工具调用
              return (
                <ToolCallItem key={part.toolCall.id} toolCall={part.toolCall} />
              );
            }
            return null;
          })}

          {/* 如果没有内容但正在流式输出，显示光标 */}
          {!hasVisibleContent &&
            isStreaming &&
            showCursor &&
            !hasRunningTools && (
              <div>
                <StreamingCursor />
              </div>
            )}
        </div>
      );
    }

    // 回退模式：传统的 content + toolCalls 分开渲染
    const hasToolCalls = toolCalls && toolCalls.length > 0;

    return (
      <div className="flex flex-col gap-2">
        {/* 思考内容 - 显示在最前面 */}
        {finalThinking && (
          <ThinkingBlock
            content={finalThinking}
            defaultExpanded={isStreaming}
          />
        )}

        {/* 工具调用区域 */}
        {hasToolCalls && <ToolCallList toolCalls={toolCalls} />}

        {/* 文本内容区域 - 使用 StreamingText 组件实现逐字符动画 */}
        {visibleText.length > 0 && (
          <StreamingText
            text={visibleText}
            isStreaming={isStreaming}
            showCursor={shouldShowCursor}
          />
        )}

        {/* 如果没有内容但正在流式输出，显示光标 */}
        {!hasVisibleContent &&
          isStreaming &&
          showCursor &&
          !hasRunningTools && (
            <div>
              <StreamingCursor />
            </div>
          )}
      </div>
    );
  },
);

StreamingRenderer.displayName = "StreamingRenderer";

export default StreamingRenderer;
