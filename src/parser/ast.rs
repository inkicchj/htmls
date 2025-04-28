use std::fmt;

/// Top-level node type.
#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    /// Selector node
    Selector(Box<SelectorNode>),

    /// Pipeline operation: left operand > right operand
    Pipeline(Box<Node>, Box<Node>),

    /// Set operation node
    SetOperation(Box<SetOperationNode>),

    /// Index selection: node with index selector
    IndexSelection(Box<Node>, Box<IndexNode>),

    /// Function call: node with function node
    FunctionCall(Box<Node>, FunctionNode),
}

/// Selector node type
#[derive(Debug, Clone, PartialEq)]
pub enum SelectorNode {
    /// Element query selector
    ElementSelector(ElementNode),

    /// Text query selector
    TextSelector(TextNode),
}

/// Element query node
#[derive(Debug, Clone, PartialEq)]
pub enum ElementNode {
    /// Class selector: parameter, whether it's a regex
    Class(String, bool),

    /// ID selector: parameter, whether it's a regex
    Id(String, bool),

    /// Tag selector: parameter, whether it's a regex
    Tag(String, bool),

    /// Attribute selector: attribute name, attribute value (optional), whether it's a regex
    Attr(String, Option<String>, bool),
}

/// Text query node
#[derive(Debug, Clone, PartialEq)]
pub enum TextNode {
    /// Element text content
    Text,

    /// href attribute value
    Href,

    /// src attribute value
    Src,

    /// attribute value
    AttrValue(String, bool),
}

/// Set operation node
#[derive(Debug, Clone, PartialEq)]
pub enum SetOperationNode {
    /// Union: left operand | right operand
    Union(Box<Node>, Box<Node>),

    /// Intersection: left operand & right operand
    Intersection(Box<Node>, Box<Node>),

    /// Difference: left operand ^ right operand
    Difference(Box<Node>, Box<Node>),
}

/// Index selection node
#[derive(Debug, Clone, PartialEq)]
pub enum IndexNode {
    /// Single index: index value
    Single(Literal),

    /// Multiple indices: list of index values
    Multiple(Vec<Literal>),

    /// Range index: start value, end value, step (optional)
    Range(Option<Literal>, Option<Literal>, Option<Literal>),
}

/// Function node
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionNode {
    /// Function name
    pub name: String,

    /// Function parameter list
    pub arguments: Vec<Literal>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(i64),
    Str(String),
    Float(f64),
    Bool(bool),
    List(Vec<Literal>),
    Nil,
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Int(n) => write!(f, "{}", n),
            Literal::Str(n) => write!(f, "{}", n),
            Literal::Float(n) => write!(f, "{}", n),
            Literal::Bool(n) => write!(f, "{}", n),
            Literal::List(list) => {
                for n in list {
                    write!(f, "{}", n)?;
                }
                Ok(())
            }
            Literal::Nil => write!(f, ""),
        }
    }
}

// Implementing the Display trait for debugging and error reporting
impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Selector(selector) => write!(f, "{}", selector),
            Node::Pipeline(left, right) => write!(f, "{} > {}", left, right),
            Node::SetOperation(op) => write!(f, "{}", op),
            Node::IndexSelection(node, idx) => write!(f, "{}:{}", node, idx),
            Node::FunctionCall(node, func) => write!(f, "{} @{}", node, func),
        }
    }
}

impl fmt::Display for SelectorNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SelectorNode::ElementSelector(elem) => write!(f, "{}", elem),
            SelectorNode::TextSelector(text) => write!(f, "{}", text),
        }
    }
}

impl fmt::Display for ElementNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ElementNode::Class(value, is_regex) => {
                if *is_regex {
                    write!(f, "class ~{}", value)
                } else {
                    write!(f, "class {}", value)
                }
            }
            ElementNode::Id(value, is_regex) => {
                if *is_regex {
                    write!(f, "id ~{}", value)
                } else {
                    write!(f, "id {}", value)
                }
            }
            ElementNode::Tag(value, is_regex) => {
                if *is_regex {
                    write!(f, "tag ~{}", value)
                } else {
                    write!(f, "tag {}", value)
                }
            }
            ElementNode::Attr(value, attr_value, is_regex) => {
                if let Some(attr_value) = attr_value {
                    if *is_regex {
                        write!(f, "attr ~{}:{}", value, attr_value)
                    } else {
                        write!(f, "attr {}:{}", value, attr_value)
                    }
                } else {
                    if *is_regex {
                        write!(f, "attr ~{}", value)
                    } else {
                        write!(f, "attr {}", value)
                    }
                }
            }
        }
    }
}

impl fmt::Display for TextNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TextNode::Text => write!(f, "text"),
            TextNode::Href => write!(f, "href"),
            TextNode::Src => write!(f, "src"),
            TextNode::AttrValue(name, is_regex) => {
                if *is_regex {
                    write!(f, "#~{}", name)
                } else {
                    write!(f, "#{}", name)
                }
            }
        }
    }
}

impl fmt::Display for SetOperationNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SetOperationNode::Union(left, right) => write!(f, "{} | {}", left, right),
            SetOperationNode::Intersection(left, right) => write!(f, "{} & {}", left, right),
            SetOperationNode::Difference(left, right) => write!(f, "{} ^ {}", left, right),
        }
    }
}

impl fmt::Display for IndexNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IndexNode::Single(idx) => write!(f, "{}", idx),
            IndexNode::Multiple(indices) => {
                let indices_str: Vec<String> = indices.iter().map(|i| i.to_string()).collect();
                write!(f, "{}", indices_str.join(","))
            }
            IndexNode::Range(start, end, step) => {
                if let Some(step_val) = step {
                    write!(
                        f,
                        "{}:{}:{}",
                        start.clone().unwrap_or(Literal::Nil),
                        end.clone().unwrap_or(Literal::Nil),
                        step_val
                    )
                } else {
                    write!(
                        f,
                        "{}:{}",
                        start.clone().unwrap_or(Literal::Nil),
                        end.clone().unwrap_or(Literal::Nil)
                    )
                }
            }
        }
    }
}

impl fmt::Display for FunctionNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.arguments.is_empty() {
            write!(f, "{}", self.name)
        } else {
            write!(f, "{}", self.name)?;
            for arg in self.arguments.iter() {
                write!(f, ",{}", arg)?;
            }
            Ok(())
        }
    }
}

/// Visitor pattern trait
pub trait Visitor<T> {
    fn visit_node(&mut self, node: &Node) -> T;
    fn visit_selector(&mut self, node: &SelectorNode) -> T;
    fn visit_element(&mut self, node: &ElementNode) -> T;
    fn visit_text(&mut self, node: &TextNode) -> T;
    fn visit_set_operation(&mut self, node: &SetOperationNode) -> T;
    fn visit_index(&mut self, node: &IndexNode) -> T;
    fn visit_function(&mut self, node: &FunctionNode) -> T;
    fn visit_pipeline(&mut self, left: &Node, right: &Node) -> T;
}

/// Visitable trait
pub trait Visitable {
    fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T;
}

impl Visitable for Node {
    fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
        visitor.visit_node(self)
    }
}
