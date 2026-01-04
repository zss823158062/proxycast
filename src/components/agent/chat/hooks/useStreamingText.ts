import { useState, useEffect, useRef, useCallback } from "react";

interface UseStreamingTextOptions {
  /** 每个字符的渲染间隔（毫秒），默认 15ms */
  charInterval?: number;
  /** 是否启用动画，默认 true */
  animated?: boolean;
  /** 当文本追赶上目标时的回调 */
  onCatchUp?: () => void;
}

interface UseStreamingTextReturn {
  /** 当前显示的文本 */
  displayText: string;
  /** 目标文本（完整内容） */
  targetText: string;
  /** 是否正在动画中 */
  isAnimating: boolean;
  /** 设置目标文本 */
  setTargetText: (text: string) => void;
  /** 追加文本到目标 */
  appendText: (text: string) => void;
  /** 重置状态 */
  reset: () => void;
  /** 立即显示完整文本（跳过动画） */
  skipToEnd: () => void;
}

/**
 * 流式文本渲染 Hook
 *
 * 实现逐字符平滑显示效果，类似 ChatGPT/Claude 的打字机效果。
 *
 * @example
 * ```tsx
 * const { displayText, appendText, reset } = useStreamingText();
 *
 * // 当收到流式数据时
 * appendText(newChunk);
 *
 * // 渲染
 * <div>{displayText}</div>
 * ```
 */
export function useStreamingText(
  options: UseStreamingTextOptions = {},
): UseStreamingTextReturn {
  const { charInterval = 15, animated = true, onCatchUp } = options;

  const [displayText, setDisplayText] = useState("");
  const [targetText, setTargetText] = useState("");
  const [isAnimating, setIsAnimating] = useState(false);

  const animationRef = useRef<number | null>(null);
  const displayIndexRef = useRef(0);

  // 清理动画
  const clearAnimation = useCallback(() => {
    if (animationRef.current !== null) {
      cancelAnimationFrame(animationRef.current);
      animationRef.current = null;
    }
  }, []);

  // 动画循环
  useEffect(() => {
    if (!animated) {
      // 禁用动画时直接显示完整文本
      setDisplayText(targetText);
      displayIndexRef.current = targetText.length;
      setIsAnimating(false);
      return;
    }

    // 如果显示文本已经追上目标文本，停止动画
    if (displayIndexRef.current >= targetText.length) {
      setIsAnimating(false);
      onCatchUp?.();
      return;
    }

    setIsAnimating(true);

    let lastTime = 0;

    const animate = (currentTime: number) => {
      if (!lastTime) lastTime = currentTime;
      const elapsed = currentTime - lastTime;

      if (elapsed >= charInterval) {
        // 计算这一帧应该显示多少个字符
        const charsToAdd = Math.max(1, Math.floor(elapsed / charInterval));
        const newIndex = Math.min(
          displayIndexRef.current + charsToAdd,
          targetText.length,
        );

        if (newIndex > displayIndexRef.current) {
          displayIndexRef.current = newIndex;
          setDisplayText(targetText.slice(0, newIndex));
        }

        lastTime = currentTime;
      }

      // 继续动画直到追上目标
      if (displayIndexRef.current < targetText.length) {
        animationRef.current = requestAnimationFrame(animate);
      } else {
        setIsAnimating(false);
        onCatchUp?.();
      }
    };

    animationRef.current = requestAnimationFrame(animate);

    return clearAnimation;
  }, [targetText, animated, charInterval, clearAnimation, onCatchUp]);

  // 追加文本
  const appendText = useCallback((text: string) => {
    setTargetText((prev) => prev + text);
  }, []);

  // 重置
  const reset = useCallback(() => {
    clearAnimation();
    setDisplayText("");
    setTargetText("");
    displayIndexRef.current = 0;
    setIsAnimating(false);
  }, [clearAnimation]);

  // 跳过动画，立即显示完整文本
  const skipToEnd = useCallback(() => {
    clearAnimation();
    setDisplayText(targetText);
    displayIndexRef.current = targetText.length;
    setIsAnimating(false);
  }, [clearAnimation, targetText]);

  // 组件卸载时清理
  useEffect(() => {
    return clearAnimation;
  }, [clearAnimation]);

  return {
    displayText,
    targetText,
    isAnimating,
    setTargetText,
    appendText,
    reset,
    skipToEnd,
  };
}
