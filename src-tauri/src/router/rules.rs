//! 路由器
//!
//! 简化的路由器，直接使用用户配置的默认 Provider

use crate::ProviderType;

/// 路由结果
#[derive(Debug, Clone)]
pub struct RouteResult {
    /// 目标 Provider
    pub provider: ProviderType,
    /// 是否使用默认 Provider
    pub is_default: bool,
}

/// 路由器 - 根据默认 Provider 路由请求
#[derive(Debug, Clone)]
pub struct Router {
    /// 默认 Provider
    default_provider: ProviderType,
}

impl Router {
    /// 创建新的路由器
    pub fn new(default_provider: ProviderType) -> Self {
        Self { default_provider }
    }

    /// 设置默认 Provider
    pub fn set_default_provider(&mut self, provider: ProviderType) {
        self.default_provider = provider;
    }

    /// 获取默认 Provider
    pub fn default_provider(&self) -> ProviderType {
        self.default_provider
    }

    /// 路由请求到 Provider
    ///
    /// 直接返回默认 Provider
    pub fn route(&self, _model: &str) -> RouteResult {
        RouteResult {
            provider: self.default_provider,
            is_default: true,
        }
    }
}

impl Default for Router {
    fn default() -> Self {
        Self::new(ProviderType::Kiro)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_router() {
        let router = Router::new(ProviderType::Kiro);
        assert_eq!(router.default_provider(), ProviderType::Kiro);
    }

    #[test]
    fn test_route_returns_default() {
        let router = Router::new(ProviderType::Antigravity);
        let result = router.route("any-model");
        assert_eq!(result.provider, ProviderType::Antigravity);
        assert!(result.is_default);
    }

    #[test]
    fn test_set_default_provider() {
        let mut router = Router::new(ProviderType::Kiro);
        router.set_default_provider(ProviderType::Gemini);
        assert_eq!(router.default_provider(), ProviderType::Gemini);
    }
}
