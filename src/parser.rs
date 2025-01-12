use crate::ast{ASTNode,Expression};

pub fn parse(tokens:Vec<Token>)->Result<ASTNode,String>{
    let mut iter = tokens.into_iter();

    if let Some(Token::Keyword(Keyword))=iter.next(){
        if Keyword=="print"{
            let expr = parse_expression(&mut iter)?;
            return Ok(ASTNode::Print(expr));
        }
    }
    Err("error while parsing".to_string());
}