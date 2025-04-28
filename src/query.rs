use std::collections::HashMap;

use super::{Interpreter, InterpreterError, InterpreterResult, SelectionResult};
/// Provides fluent API interfaces and result caching to simplify queries and operations.
#[derive(Clone)]
pub struct Query {
    /// interpreter
    interpreter: Interpreter,

    /// current query result
    current_result: Option<Result<SelectionResult, InterpreterError>>,

    /// query cache
    cache: HashMap<String, Result<SelectionResult, InterpreterError>>,
}

impl Query {
    pub fn new(html: &str) -> Self {
        match Interpreter::new(html) {
            Ok(it) => Query {
                interpreter: it,
                current_result: None,
                cache: HashMap::new(),
            },
            Err(e) => panic!("{}", e.to_string()),
        }
    }


    /// Query nodes using a selector.
    pub fn query(mut self, selector: &str) -> Self {
        let cache_key = selector.to_string();
        if !self.cache.contains_key(&cache_key) {
            let result = self.interpreter.select(selector);
            self.cache.insert(cache_key.clone(), result);
        }

        self.current_result = Some(self.cache.get(&cache_key).unwrap().clone());
        self
    }

    /// Query nodes from the specified context.
    pub fn from(mut self, context: SelectionResult, selector: &str) -> Self {
        let cache_key = format!("ctx:{}:{}", context_hash(&context), selector);

        if !self.cache.contains_key(&cache_key) {
            let result = self.interpreter.select_from(&context, selector);
            self.cache.insert(cache_key.clone(), result);
        }

        self.current_result = Some(self.cache.get(&cache_key).unwrap().clone());
        self
    }

    /// Execute a function for each selection result.
    pub fn for_each<F>(self, mut f: F)
    where
        F: FnMut(Query, SelectionResult),
    {
        if let Some(Ok(result)) = self.current_result.clone() {
            for item in result.iter() {
                f(self.clone(), item)
            }
        }
    }

    /// Get the result of the first text.
    pub fn text(self) -> Option<String> {
        match self.current_result {
            Some(Ok(result)) => {
                if result.is_texts() {
                    result.first_text().ok().map(|s| s.to_string())
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Get results of all texts.
    pub fn texts(self) -> Vec<String> {
        match self.current_result {
            Some(Ok(result)) => {
                if result.is_texts() {
                    result.texts().map(|t| t.clone()).unwrap_or_default()
                } else {
                    Vec::new()
                }
            }
            _ => Vec::new(),
        }
    }

    /// Get the result of the first node.
    pub fn node(self) -> Option<super::NodeHandle> {
        match self.current_result {
            Some(Ok(result)) => {
                if result.is_nodes() {
                    result.first_node().ok().cloned()
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Get results of all nodes
    pub fn nodes(self) -> Vec<super::NodeHandle> {
        match self.current_result {
            Some(Ok(result)) => {
                if result.is_nodes() {
                    result.nodes().map(|n| n.clone()).unwrap_or_default()
                } else {
                    Vec::new()
                }
            }
            _ => Vec::new(),
        }
    }

    /// Obtain the final query results.
    pub fn result(self) -> InterpreterResult<SelectionResult> {
        match self.current_result {
            Some(result) => result,
            None => Err(InterpreterError::execution_error("No queries were executed.")),
        }
    }

    /// Use the query results as the context for another query.
    pub fn then(self, selector: &str) -> Self {
        match self.current_result {
            Some(Ok(ref result)) => {
                let result_clone = result.clone();
                self.from(result_clone, selector)
            }
            Some(Err(ref e)) => {
                let mut builder = self.clone();
                builder.current_result = Some(Err(e.clone()));
                builder
            }
            None => {
                let mut builder = self.clone();
                builder.current_result =
                    Some(Err(InterpreterError::execution_error("No queries were executed.")));
                builder
            }
        }
    }

    /// Clear cache.
    pub fn clear_cache(mut self) -> Self {
        self.cache.clear();
        self
    }

    /// Get result count.
    pub fn count(self) -> usize {
        match self.current_result {
            Some(Ok(result)) => result.count(),
            _ => 0,
        }
    }

    /// Check if the result is empty.
    pub fn is_empty(self) -> bool {
        match self.current_result {
            Some(Ok(result)) => result.is_empty(),
            _ => true,
        }
    }
}

/// Generate the hash value of the context for use as a cache key.
fn context_hash(context: &SelectionResult) -> String {
    match context {
        SelectionResult::Nodes(nodes) => {
            if nodes.is_empty() {
                "empty_nodes".to_string()
            } else if nodes.len() <= 3 {
                let ids: Vec<String> = nodes.iter().map(|n| n.id().to_string()).collect();
                ids.join(",")
            } else {
                format!("{}+{}", nodes[0].id(), nodes.len())
            }
        }
        SelectionResult::Texts(texts) => {
            if texts.is_empty() {
                "empty_texts".to_string()
            } else if texts.len() <= 3 {
                let truncated: Vec<String> = texts
                    .iter()
                    .map(|t| {
                        if t.len() > 20 {
                            format!("{}...", &t[0..20])
                        } else {
                            t.clone()
                        }
                    })
                    .collect();
                truncated.join(",")
            } else {
                let first = if texts[0].len() > 20 {
                    format!("{}...", &texts[0][0..20])
                } else {
                    texts[0].clone()
                };
                format!("{}+{}", first, texts.len())
            }
        }
    }
}
