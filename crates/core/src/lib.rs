//! Code Optimizer Core Engine
//! Advanced pattern matching and configuration support!

use std::collections::HashMap;

/// Programming languages we support
#[derive(Debug, Clone, PartialEq)]
pub enum Language {
    JavaScript,
    Python,
    Rust,
}

/// Pattern matching types
#[derive(Debug, Clone)]
pub enum PatternType {
    Contains(String),   // Simple substring matching
    Regex(String),      // Regex pattern (we'll simulate for now)
    StartsWith(String), // Line starts with pattern
    EndsWith(String),   // Line ends with pattern
}

/// Configuration for the optimizer
#[derive(Debug, Clone)]
pub struct OptimizerConfig {
    pub enabled_rules: HashMap<String, bool>,
    pub custom_rules: Vec<OptimizationRule>,
    pub severity_filter: Vec<Severity>,
}

/// The main brain of our code optimizer
pub struct CodeOptimizer {
    name: String,
    rules: Vec<OptimizationRule>,
    config: OptimizerConfig,
}

/// Represents a single optimization suggestion
#[derive(Debug, Clone)]
pub struct Optimization {
    pub rule_name: String,
    pub language: Language,
    pub line_number: usize,
    pub original_code: String,
    pub suggested_code: String,
    pub explanation: String,
    pub severity: Severity,
    pub confidence: f32, // 0.0 to 1.0
}

/// How important is this optimization?
#[derive(Debug, Clone, PartialEq)]
pub enum Severity {
    Info,           // Nice to have
    Warning,        // Should fix
    Error,          // Must fix
    Custom(String), // User-defined severity
}

/// A rule that can find and fix code issues
#[derive(Debug, Clone)]
pub struct OptimizationRule {
    pub name: String,
    pub language: Language,
    pub pattern_type: PatternType,
    pub replacement: String,
    pub explanation: String,
    pub severity: Severity,
    pub confidence: f32,
    pub enabled: bool,
}

impl OptimizerConfig {
    /// Create default configuration
    pub fn new() -> Self {
        OptimizerConfig {
            enabled_rules: HashMap::new(),
            custom_rules: Vec::new(),
            severity_filter: vec![Severity::Info, Severity::Warning, Severity::Error],
        }
    }

    /// Load configuration from a simulated config file
    pub fn from_config_string(config_str: &str) -> Self {
        let mut config = OptimizerConfig::new();

        // Simple config parser (in real app, use TOML/JSON)
        for line in config_str.lines() {
            let line = line.trim();
            if line.starts_with("disable_rule:") {
                let rule_name = line.replace("disable_rule:", "").trim().to_string();
                config.enabled_rules.insert(rule_name, false);
            } else if line.starts_with("enable_rule:") {
                let rule_name = line.replace("enable_rule:", "").trim().to_string();
                config.enabled_rules.insert(rule_name, true);
            }
        }

        config
    }

    /// Add a custom rule
    pub fn add_custom_rule(&mut self, rule: OptimizationRule) {
        self.custom_rules.push(rule);
    }
}

impl CodeOptimizer {
    /// Create a new optimizer with default config
    pub fn new() -> Self {
        let mut optimizer = CodeOptimizer {
            name: "Advanced Code Optimizer".to_string(),
            rules: Vec::new(),
            config: OptimizerConfig::new(),
        };

        optimizer.add_built_in_rules();
        optimizer
    }

    /// Create optimizer with custom config
    pub fn with_config(config: OptimizerConfig) -> Self {
        let mut optimizer = CodeOptimizer {
            name: "Advanced Code Optimizer".to_string(),
            rules: Vec::new(),
            config,
        };

        optimizer.add_built_in_rules();
        optimizer
    }

    /// Show capabilities
    pub fn hello(&self) -> String {
        let total_rules = self.rules.len() + self.config.custom_rules.len();
        let enabled_rules = self.get_enabled_rules().len();

        format!(
            "Hello from {}!\n  📊 Total rules: {}\n  ✅ Enabled rules: {}\n  🎯 Custom rules: {}",
            self.name,
            total_rules,
            enabled_rules,
            self.config.custom_rules.len()
        )
    }

    /// Add built-in optimization rules with advanced patterns
    fn add_built_in_rules(&mut self) {
        // JavaScript rules
        self.rules.push(OptimizationRule {
            name: "use-const".to_string(),
            language: Language::JavaScript,
            pattern_type: PatternType::Contains("let ".to_string()),
            replacement: "const ".to_string(),
            explanation: "Use 'const' for variables that never change".to_string(),
            severity: Severity::Info,
            confidence: 0.8,
            enabled: true,
        });

        self.rules.push(OptimizationRule {
            name: "arrow-function".to_string(),
            language: Language::JavaScript,
            pattern_type: PatternType::Contains("function(".to_string()),
            replacement: "(".to_string(),
            explanation: "Consider using arrow functions for shorter syntax".to_string(),
            severity: Severity::Info,
            confidence: 0.6,
            enabled: true,
        });

        self.rules.push(OptimizationRule {
            name: "use-strict-equality".to_string(),
            language: Language::JavaScript,
            pattern_type: PatternType::Contains(" == ".to_string()),
            replacement: " === ".to_string(),
            explanation: "Use strict equality '===' to avoid type coercion bugs".to_string(),
            severity: Severity::Warning,
            confidence: 0.9,
            enabled: true,
        });

        // Python rules with advanced patterns
        self.rules.push(OptimizationRule {
            name: "list-comprehension".to_string(),
            language: Language::Python,
            pattern_type: PatternType::Contains("for ".to_string()),
            replacement: "[".to_string(),
            explanation: "Consider using list comprehension for better performance".to_string(),
            severity: Severity::Info,
            confidence: 0.7,
            enabled: true,
        });

        self.rules.push(OptimizationRule {
            name: "pathlib-usage".to_string(),
            language: Language::Python,
            pattern_type: PatternType::Contains("os.path.".to_string()),
            replacement: "pathlib.".to_string(),
            explanation: "Use pathlib instead of os.path for modern path handling".to_string(),
            severity: Severity::Warning,
            confidence: 0.9,
            enabled: true,
        });

        // Rust rules
        self.rules.push(OptimizationRule {
            name: "clippy-style".to_string(),
            language: Language::Rust,
            pattern_type: PatternType::Contains(".clone()".to_string()),
            replacement: "".to_string(),
            explanation: "Unnecessary clone() - consider borrowing instead".to_string(),
            severity: Severity::Warning,
            confidence: 0.8,
            enabled: true,
        });
    }

    /// Get rules that are currently enabled
    fn get_enabled_rules(&self) -> Vec<&OptimizationRule> {
        let mut enabled_rules = Vec::new();

        // Check built-in rules
        for rule in &self.rules {
            let is_enabled = self
                .config
                .enabled_rules
                .get(&rule.name)
                .unwrap_or(&rule.enabled);

            if *is_enabled {
                enabled_rules.push(rule);
            }
        }

        // Add custom rules
        for rule in &self.config.custom_rules {
            if rule.enabled {
                enabled_rules.push(rule);
            }
        }

        enabled_rules
    }

    /// Advanced pattern matching
    fn matches_pattern(&self, line: &str, pattern: &PatternType) -> bool {
        match pattern {
            PatternType::Contains(text) => line.contains(text),
            PatternType::StartsWith(text) => line.trim_start().starts_with(text),
            PatternType::EndsWith(text) => line.trim_end().ends_with(text),
            PatternType::Regex(pattern) => {
                // Simplified regex - in real app use regex crate
                if pattern == r"let\s+\w+\s*=" {
                    line.contains("let ") && line.contains("=")
                } else {
                    line.contains(&pattern.replace(r"\s+", " "))
                }
            }
        }
    }

    /// Advanced code analysis with configuration
    pub fn analyze_code(&self, code: &str, language: Language) -> Vec<Optimization> {
        let mut optimizations = Vec::new();
        let lines: Vec<&str> = code.lines().collect();

        let enabled_rules = self.get_enabled_rules();
        let relevant_rules: Vec<_> = enabled_rules
            .iter()
            .filter(|rule| rule.language == language)
            .collect();

        for (line_number, line) in lines.iter().enumerate() {
            for rule in &relevant_rules {
                if self.matches_pattern(line, &rule.pattern_type) {
                    // Check severity filter
                    if self.config.severity_filter.contains(&rule.severity) {
                        optimizations.push(Optimization {
                            rule_name: rule.name.clone(),
                            language: language.clone(),
                            line_number: line_number + 1,
                            original_code: line.to_string(),
                            suggested_code: self.apply_replacement(line, rule),
                            explanation: rule.explanation.clone(),
                            severity: rule.severity.clone(),
                            confidence: rule.confidence,
                        });
                    }
                }
            }
        }

        // Sort by confidence (highest first)
        optimizations.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());
        optimizations
    }

    /// Apply rule replacement
    fn apply_replacement(&self, line: &str, rule: &OptimizationRule) -> String {
        match &rule.pattern_type {
            PatternType::Contains(pattern) => line.replace(pattern, &rule.replacement),
            PatternType::StartsWith(pattern) => {
                if line.trim_start().starts_with(pattern) {
                    line.replacen(pattern, &rule.replacement, 1)
                } else {
                    line.to_string()
                }
            }
            _ => line.replace("pattern", &rule.replacement), // Simplified
        }
    }

    /// Add configuration at runtime
    pub fn update_config(&mut self, config: OptimizerConfig) {
        self.config = config;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_optimizer() {
        let optimizer = CodeOptimizer::new();
        let message = optimizer.hello();
        println!("🎉 {}", message);
        assert!(message.contains("Advanced Code Optimizer"));
    }

    #[test]
    fn test_custom_config() {
        let mut config = OptimizerConfig::new();
        config.enabled_rules.insert("use-const".to_string(), false);

        let optimizer = CodeOptimizer::with_config(config);
        let code = "let userName = 'John';";
        let optimizations = optimizer.analyze_code(code, Language::JavaScript);

        println!(
            "🔧 Config test: {} optimizations found",
            optimizations.len()
        );
        // Should find fewer optimizations because we disabled use-const
    }

    #[test]
    fn test_advanced_patterns() {
        let optimizer = CodeOptimizer::new();
        let python_code = r#"
import os.path
result = []
for item in items:
    result.append(item * 2)
"#;

        let optimizations = optimizer.analyze_code(python_code, Language::Python);
        println!(
            "🔍 Advanced pattern matching found {} optimizations:",
            optimizations.len()
        );

        for opt in &optimizations {
            println!(
                "  🎯 {:.0}% confidence: {}",
                opt.confidence * 100.0,
                opt.explanation
            );
        }

        assert!(!optimizations.is_empty());
    }

    #[test]
    fn test_config_from_string() {
        let config_str = r#"
            disable_rule: use-const
            enable_rule: arrow-function
        "#;

        let config = OptimizerConfig::from_config_string(config_str);
        let optimizer = CodeOptimizer::with_config(config);

        println!("📄 Configuration loaded successfully!");
        let message = optimizer.hello();
        println!("{}", message);
    }

    #[test]
    fn test_custom_rule() {
        let mut config = OptimizerConfig::new();

        // Add a custom rule for JavaScript
        let custom_rule = OptimizationRule {
            name: "no-var".to_string(),
            language: Language::JavaScript,
            pattern_type: PatternType::Contains("var ".to_string()),
            replacement: "let ".to_string(),
            explanation: "Use 'let' instead of 'var' for block scoping".to_string(),
            severity: Severity::Custom("Style".to_string()),
            confidence: 0.95,
            enabled: true,
        };

        config.add_custom_rule(custom_rule);
        let optimizer = CodeOptimizer::with_config(config);

        let code = "var oldStyle = 'bad';";
        let optimizations = optimizer.analyze_code(code, Language::JavaScript);

        println!(
            "🎨 Custom rule test: {} optimizations found",
            optimizations.len()
        );
        for opt in &optimizations {
            println!("  ✨ Custom: {}", opt.explanation);
        }
    }
}
