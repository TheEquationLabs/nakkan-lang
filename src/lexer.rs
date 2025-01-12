##[derive(Debug)]
pub enum Token{
    Number(f64),
    Identifier(String),
    Keyword(String),
    Operator(String),
    Openparameter,
    Closeparameter,
}

pub fn tokenize(input:&str)->Result<Vec<Token>,String>{
    let mut tokens=Vec::new();
    let words=input.split_whitespaces();
    
    for word in words{
        match word{
            "print"=>tokens.push(Token::Keyword(word.to_string())),
            "+"|"-"|"*"|"/"=>tokens.push(Token::Opearator(word.to_String())),
            "(" => tokens.push(Token::Openparameter(word.to_string())),
            ")"=>tokens.push(Token::Closeparameter(wors.to_string())),
            "_"=>if let Ok(num)=word.parse::<f64>(){
                tokens.push(Token::Number(num));
            }else{
                token.push(Token::Identifier(word.to_string()));
            }
        }
    }
    Ok(tokens)
}