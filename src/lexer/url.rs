use logos::{Lexer, Logos};
use std::{fmt::{Display, Formatter}};

/// Tuple struct for link URLs
#[derive(Debug, PartialEq)]
pub struct LinkUrl(String);

/// Implement Display for printing
impl Display for LinkUrl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// Tuple struct for link texts
#[derive(Debug, PartialEq)]
pub struct LinkText(String);

/// Implement Display for printing
impl Display for LinkText {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// Token enum for capturing of link URLs and Texts
#[derive(Logos, Debug, PartialEq)]
pub enum URLToken {
    // TODO: Capture link definitions
    

    #[regex("<a[^>]*href=[^>]+>[^<]*</a", extract_link_info)]

    Link((LinkUrl, LinkText)),

    // TODO: Ignore all characters that do not belong to a link definition
    
    #[regex("<[^>]+>", logos::skip)]
    #[regex(r"[ \t\n\f\r]+",logos::skip)]
    #[regex("[a-zA-Z]+", logos::skip)]
    #[regex("[0-9]+", logos::skip)]
    #[regex(">", logos::skip)]
    #[regex("<a[^>]*>[^<]*</a",logos::skip)]
    Ignored,

    // Catch any error
    #[error]
    Error,
}

// Extracts the URL and text from a string that matched a Link token DONE!!!!" "
fn extract_link_info(lex: &mut Lexer<URLToken>) -> (LinkUrl, LinkText) {
    // TODO: Implement extraction from link definition
    let slice = lex.slice();
    
    let length = slice.len();
    let mut v = Vec::with_capacity(length);
    let mut count = 0;
    let mut  url = String::new();
    let mut  text = String::new();
    

    //create a vec bc i need the indexes
    while count < length {                      
        v.push(slice.chars().nth(count));
        count = count +1;
    }

    count = 0;
    //find href="" within the string
        while v[count] != Some('>') {   
        count = count + 1;        
        if v[count-1] == Some('h'){
            
            if v[count] == Some('r'){
                count = count + 1;
                if v[count] == Some('e'){
                    count = count + 1;
                    if v[count] == Some('f'){
                        count = count + 1;
                        if v[count] == Some('='){
                            count = count + 1;

                            
                            count = count + 1;
                            
                            while v[count] != Some('"'){
                                url.push(v[count].unwrap());
                                count = count + 1;
                            } 

                            
                            count = count + 1;    



                        }
                    }
                }
            }
        }
        
        

    }
    
    count = count + 1;

    
    while  v[count] != Some('<'){

        text.push(v[count].unwrap());
        count = count + 1;
    }
    

    return (LinkUrl(url),LinkText(text))
}
