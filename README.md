# HTML Selector

[DOCS](https://docs.rs/htmls/0.1.4/htmls/)

An HTML content extraction tool similar to CSS selectors that can precisely extract and process the required content from HTML documents through a simple and intuitive query language.

### Example

```rust
use query::Query;

fn main() {
    let html = r#"
    <div class="a">
        <p>text 1</p>
        <p>text 2</p>
        <p>text 3</p>
    </div>
    <div class="b">
        <p>text 4</p>
        <p>text 5</p>
        <p>text 6</p>
    </div>
    "#;
    let q = Query::new(html);
    let result = q.query(r#"(class a > tag p:1:2 | class b > tag p:0) > text @replace," ","""#).texts();
    println!("{:?}", result); // ["text2", "text3", "text4"]
}

```

### Basic Selectors

| Selector   | Syntax                | Description                                |
|------------|------------------------|--------------------------------------------|
| Class Selector | `class className`   | Select elements with the specified class name |
| ID Selector    | `id IDValue`        | Select elements with the specified ID      |
| Tag Selector   | `tag tagName`       | Select elements with the specified HTML tag |
| Attribute Selector | `attr "attributeName"` | Select elements with the specified attribute |
| Attribute Value Selector | `attr "attributeName" "value"` | Select elements with the attribute matching the specified value |

### Text Extraction

| Operation  | Syntax                | Description                                |
|------------|------------------------|--------------------------------------------|
| Text Content | `text`               | Extract the text content of elements        |
| Link Address | `href`               | Extract the href attribute value of elements |
| Image Address | `src`               | Extract the src attribute value of elements  |
| Attribute text value | `#"attributeName"` | Extract the value of a specific attribute |

### Pipeline Operations

The pipeline operator `>` is used to connect multiple selectors for layer-by-layer querying:

```
class main > tag p > text
```

This query first selects elements with the class "main", then finds p tags within them, and finally extracts the text content of these p tags.

### Regular Expression Matching

Using the tilde `~` allows for regular expression matching:

```
class ~".*ain" > tag p > text
```

This query selects elements with class names matching the regular expression `.*ain`, for example, it can match "main", "again", etc.

### Index Selection

You can add an index after the selector to select elements at specific positions:

| Index Syntax | Example              | Description                     |
|--------------|----------------------|---------------------------------|
| Single Index | `class a:1`          | Select the 2nd element (index starts from 0) |
| Range Index  | `class a:1:3`        | Select elements with indices from 1 to 3 |
| Range with Step | `class a:1:10:2`  | Select elements with indices from 1 to 10 with a step of 2 |
| Multiple Indices | `class a:1,3,5`  | Select elements with indices 1, 3, and 5 |

### Text Processing Functions

Using the `@` symbol can invoke built-in text processing functions:

| Function   | Syntax                     | Description                     |
|------------|----------------------------|---------------------------------|
| trim       | `text @trim`               | Remove whitespace from both ends of the text |
| replace    | `text @replace,A,B`        | Replace A with B in the text     |
| format     | `text @format,"{}value"`   | Format the text with the specified template |
| join       | `text @join,","`           | Join multiple texts with the specified separator |
| lowercase  | `text @lowercase`          | Convert text to lowercase       |
| uppercase  | `text @uppercase`          | Convert text to uppercase       |
| contains   | `text @contains,A`         | Get the string containing a certain substring  |
| starts_with | `text @starts_with,A`     | Get the string whose beginning contains a certain substring |
| starts_with | `text @ends_with,A`       | Get the string whose ending contains a certain substring |
| in | `text @in,[A,B,C ]` | Get the string in the list |

**Function Parameter Types**

| type | literal |
|------|---------|
| int  | `1 \| -1`  |
| float | `1.1 \| -1.1` |
| bool | `true \| false` |
| str  | `hello \| "hello"` |
| list | `[1,2,3 ] \| [a, "b"] \| [1.1, 1.2 ]`

Multiple functions can be chained:

```
class main > text @trim @replace,A,a @lowercase
```

### Set Operations

| Operator | Example                       | Description                     |
|----------|-------------------------------|---------------------------------|
| \|       | `expr1 \| expr2`              | Union, merge two selection results |
| &        | `expr1 & expr2`              | Intersection, get common elements from two results |
| ^        | `expr1 ^ expr2`              | Difference, exclude elements of expr2 from expr1 |

Complex set operations can be grouped with parentheses:

```
(((class a ^ class c) | class b) > tag a | class main > tag a) > text @trim
```

### Update History

**0.1.5 (2025.04.27)**

1. Added the `#` operator for selecting attribute text values.

    `class main > tag a > #target (or #~".*rget")`

2. Added parsing of function parameters for `int`, `float`, `bool`, `str` and `list` types.
    
    > Note: The current function only accepts parameters of type `str` and `list<str>`.

    > Note: There must be a space between the last value in the list and the right square bracket; otherwise, a parsing error will occur. For example: `[1, 2 ]`.

3. Add a new function `in`.

    `@in,["text1", text2 ]`

    