pub mod lexer;
pub mod parser;
pub mod interpreter;
pub mod query;

use lexer::*;
use interpreter::*;
use parser::*;
pub use query::Query;



#[cfg(test)]
mod tests {


    use super::Query;

    #[test]
    fn main_test() {
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
        let result = q.query(r#"tag p > text @contains,"1""#).texts();
        println!("{:?}", result); // ["text2", "text3", "text4"]
    }


}