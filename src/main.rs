use std::collections::HashMap;
use std::env;
use std::fs;
use std::process;

//this is the compiler trait like a bluprint 
//compilers we built MUST have these functions
pub trait Compiler {
    fn compile(&mut self, source: &str);
    fn next_token(&mut self) -> String;
    fn parse(&mut self);
    fn current_token(&self) -> String;
    fn set_current_token(&mut self, tok: String);
}

//this is the lexical analyzer
//reads raw text files and produces tokens based on whats inside
pub struct SimpleLexicalAnalyzer {
    //holds tokens that are prodiced from the file
    pub tokens: Vec<String>,
    //these are the tags that thhe lexer will recognise
    pub known_tags: Vec<String>,
}

impl SimpleLexicalAnalyzer {
    pub fn new(_source: &str) -> Self {
        Self {
            tokens: Vec::new(),
            known_tags: vec![
                //keywords and tags that the lexer will recognise
                "#GIMMEH ITALICS".to_string(),
                "#GIMMEH TITLE".to_string(),
                "#MAEK PARAGRAF".to_string(),
                "#GIMMEH BOLD".to_string(),
                "#GIMMEH ITEM".to_string(),
                "#GIMMEH LINX".to_string(),
                "#MAEK HEAD".to_string(),
                "#MAEK LIST".to_string(),
                "#LEMMESEE".to_string(),
                "#NEWLINE".to_string(),
                "#OBTW".to_string(),
                "#TLDR".to_string(),
                "#KBYE".to_string(),
                "#MKAY".to_string(),
                "#IHAZ".to_string(),
                "#ITIZ".to_string(),
                "#HAI".to_string(),
                "#OIC".to_string(),
            ],
        }
    }
   //NOT CASE SENSITIVE
   //checks if strings starts with a pattern
    fn starts_with_ignore_case(s: &str, pat: &str) -> bool {
        s.len() >= pat.len() && s[..pat.len()].eq_ignore_ascii_case(pat)
    }

    //finds the position of a pattern in a string
    //it ignore case
    fn find_ignore_case(s: &str, pat: &str) -> Option<usize> {
        let lower_s = s.to_ascii_lowercase();
        let lower_pat = pat.to_ascii_lowercase();
        lower_s.find(&lower_pat)
    }

    //looks at the words and checks if 
    //checks if they match any of the tags we provided
    //if it matches we get an output of the tag and the sentence
    //if it doesnt match we dont get anything
    fn match_known_tag<'a>(&self, s: &'a str) -> Option<(String, &'a str)> {
        for tag in &self.known_tags {
            if Self::starts_with_ignore_case(s, tag) {
                let rest = &s[tag.len()..];
                return Some((tag.clone(), rest));
            }
        }
        None
    }
    
    //if there is a text that is not empty we push it to the tokens vector
    fn push_text_if_any(&mut self, text: &str) {
        let trimmed = text.trim();
        if !trimmed.is_empty() {
            self.tokens.push(format!("TEXT:{}", trimmed));
        }
    }
  //breaking single line into tokens 
  //checking if they match any of the tafs we provided
    fn tokenize_regular_line(&mut self, line: &str) -> Result<(), String> {
        let mut rest = line;
      //we keep checking line till we reach the end of the line
      //if we find a tag we push it to the tokens vector 
      //after psuhing keep checking for more
        while !rest.is_empty() {
            if rest.starts_with('#') {
                if let Some((tag, new_rest)) = self.match_known_tag(rest) {
                    self.tokens.push(tag);
                    rest = new_rest.trim_start();
                //if we find a tag that is not in our known tags
                //return error
                } else {
                    return Err(format!(
                        "Lexical error: unknown annotation starting with '{}'",
                        rest
                    ));
                }
            //if we dont find a tag we look for the next tag 
            //and we push the text before the tag to the tokens vector as a text token
            //then we keep checkingh for more tags until we reach the end
            } else {
                let next_tag = rest.find('#').unwrap_or(rest.len());
                let text = &rest[..next_tag];
                self.push_text_if_any(text);
                rest = rest[next_tag..].trim_start();
            }
        }
        //if we reach the end of the line without error we return ok
        Ok(())
        //tokenise lines that are not comments and not empty
    }
 
    //this is the main function of the lexer
    //it reads the file and produces tokens based on the content
    pub fn tokenize(&mut self, source: &str) -> Result<(), String> {
        self.tokens.clear();
    //split the file into fines and process each line
        let lines: Vec<&str> = source.lines().collect();
        let mut i = 0;
    //looping through the lines of the file
        while i < lines.len() {
            //we trim the line to remove whitespace
            let line = lines[i].trim();

            if line.is_empty() {
                i += 1;
                continue;
            }
        //we check if the line contains these invalid chars
        //if it does we return an error
            if line.contains('<') || line.contains('>') || line.contains('&') {
                return Err(format!(
                    "Lexical error: invalid character found in line '{}'",
                    line
                ));
            }
        //if the line starts with #OBTW 
        //its a comment
        //we push it to the tokens vector
        //then we check if the comment contains #TLDR
        //if it does then we split the comment into two parts 
            if Self::starts_with_ignore_case(line, "#OBTW") {
                self.tokens.push("#OBTW".to_string());

                let rest = line[5..].trim();

                if let Some(pos) = Self::find_ignore_case(rest, "#TLDR") {
                    let comment_body = rest[..pos].trim();
                    let after_tldr = rest[pos + 5..].trim();
                 //if the comment body is not empty we push it to the tokens vector as a text token
                    if !comment_body.is_empty() {
                        self.tokens.push(format!("TEXT:{}", comment_body));
                    }
                 //then we push the #TLDR tag to the tokens vector 
                    self.tokens.push("#TLDR".to_string());
                //if there is any text after the #TLDR tag we push it to the tokens vector as a text token
                   if !after_tldr.is_empty() {
                        self.tokenize_regular_line(after_tldr)?;
                    }
                //we find #OBTW but we dont find TLDR we need to keep looking
                } else {

                    let mut comment_parts: Vec<String> = Vec::new();

                    if !rest.is_empty() {
                        comment_parts.push(rest.to_string());
                    }

                    let mut found_tldr = false;
                    i += 1;
                 
                 //keep lookinh for TLDR
                    while i < lines.len() {
                        let next_line = lines[i].trim();
                        if let Some(pos) = Self::find_ignore_case(next_line, "#TLDR") {
                            let before_tldr = next_line[..pos].trim();
                            let after_tldr = next_line[pos + 5..].trim();

                            if !before_tldr.is_empty() {
                                comment_parts.push(before_tldr.to_string());
                            }

                            let comment_body = comment_parts.join(" ");
                            if !comment_body.trim().is_empty() {
                                self.tokens.push(format!("TEXT:{}", comment_body.trim()));
                            }

                            self.tokens.push("#TLDR".to_string());

                            if !after_tldr.is_empty() {
                                self.tokenize_regular_line(after_tldr)?;
                            }

                            found_tldr = true;
                            break;
                        } else if !next_line.is_empty() {
                            comment_parts.push(next_line.to_string());
                        }

                        i += 1;
                    }

                    if !found_tldr {
                        return Err(
                            "Syntax error: comment started with #OBTW but missing #TLDR."
                                .to_string(),
                        );
                    }
                }
            } else {
                self.tokenize_regular_line(line)?;
            }

            i += 1;
        }
    //after processing all lines we reverse the tokens vector to prepare for parsing
        self.tokens.reverse();
        Ok(())
    }
    //checking is a string is a known tag or a text token
    fn lookup(&self, s: &str) -> bool {
        self.known_tags.iter().any(|tag| tag == s) || s.starts_with("TEXT:")
    }
    //print token for debugging purposes
}

//main compiler struct that implements the compiler trait
pub struct LolcodeCompiler {
    lexer: SimpleLexicalAnalyzer,
    //curernt token that we have during parsing
    current_tok: String,
    //stack of scopes for var definitions
    scopes: Vec<HashMap<String, String>>,
    //string that holds the generated HTML output
    pub output: String,
}

//implementation of the compiler functions and the parsing logic
impl LolcodeCompiler {
    //constructor for the compiler 
    pub fn new() -> Self {
        //initialising the compiler with an empty lexer and empty output
        Self {
            lexer: SimpleLexicalAnalyzer::new(""),
            current_tok: String::new(),
            scopes: vec![HashMap::new()],
            output: String::new(),
        }
    }

    //start parsing by getting the first token from the lexer
    //if the file is empty we return an error
    fn start(&mut self) {
        //getting the first token from the lexer
        let candidate = self.next_token();
        if candidate.is_empty() {
            eprintln!("User error: the provided file is empty.");
            //if the file is empty we exit with an error
            process::exit(1);
        }
        //setting the current token to the first token we got from the lexer to start parsing
        self.current_tok = candidate;
    }
    
    //this function is used to check if the current token matches an expected value
    //if it matches we move to the next token
    //if it doesnt match we return an error and leave
    fn expect(&mut self, expected: &str) {
        if self.current_tok.eq_ignore_ascii_case(expected) {
            self.next_token();
        } else {
            eprintln!(
                "Syntax error: expected '{}', but found '{}'.",
                expected, self.current_tok
            );
            process::exit(1);
        }
    }
    
    //this function checks if the current token is a text token
    fn is_text_token(&self) -> bool {
        self.current_tok.starts_with("TEXT:")
    }
    //this extracts the text value froma  text token 
    fn text_value(&self) -> String {
        if self.is_text_token() {
            self.current_tok[5..].trim().to_string()
        } else {
            String::new()
        }
    }
    //add raw strinngs to the output without space
    //used for tags and html structure
    //raw strings are the strings we add without any modifications and soace
    fn emit_raw(&mut self, s: &str) {
        self.output.push_str(s);
    }
    
    //this adds pieces of texts to the output with a space after
    //when we want to seperate texts with spaces
    fn emit_piece(&mut self, s: &str) {
        if !s.is_empty() {
            self.output.push_str(s);
            self.output.push(' ');
        }
    }
    //used to manage variable scopes
    //we enter a scope and push a new empty hashmap to the scopes stock 
    fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }
    //when we exit a scope we pop the last hashman from the scope
    //we only pop if we have more than one scope to avoid poppingthe global scope
    //for var definitions
    fn exit_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }
    
    //this is used to drfine a variable in the current scope
    //we add the var name and value to the last hashmap in the scopes stock which we consider the current one
    fn define_variable(&mut self, name: String, value: String) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, value); 
        }
    }
 
    //looking up a var value by its name
    //
    fn lookup_variable(&self, name: &str) -> Option<String> {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Some(value.clone());
            }
        }
        //if we dont find the var in any scope we return none
        None
    }

    //used to process text tokens and add their value to the output
    fn text(&mut self) {
        //checking if the current token is a text token
        if self.is_text_token() {
            //then we extract the text valie and add it to the output
            let value = self.text_value();
            self.emit_piece(&value);
            //then after processing that we move to the next token to continye parsing
            self.next_token();
        
        //if we find a token that is not a text token when we expect text we return an error and exit
        } else {
            eprintln!("Syntax error: expected text, but found '{}'.", self.current_tok);
            process::exit(1);
        }
    }
 
    //main function that does program structure
    //body of the program is processed here
    //checking for strating and ending tags
    //chcek for extra tokens
    fn program(&mut self) {
        //program MUST start with #HAI
        if !self.current_tok.eq_ignore_ascii_case("#HAI") {
            //if it doesnt we return an error and exit
            eprintln!("Syntax error: program must start with #HAI.");
            process::exit(1);
        }
      //if itstarts with #HAI add the opening html tag to the output
      //then we process the body of the program
        self.emit_raw("<html>\n");
        self.expect("#HAI");
        self.body();
     //MUST end with #KBYE
     //if it doesnt we return and error and exit
        if !self.current_tok.eq_ignore_ascii_case("#KBYE") {
            eprintln!("Syntax error: program must end with #KBYE.");
            process::exit(1);
        }
    //if it ends with #KBYE add the closing html tag to the output
    //then check for extra token after #KBYE
    //if we find any extra tokens we return error and exit
        self.expect("#KBYE");

        if !self.current_tok.is_empty() {
            eprintln!(
                "Syntax error: extra tokens found after #KBYE: '{}'.",
                self.current_tok
            );
            process::exit(1);
        }
    //adding closing ht,l tag to the output
        self.emit_raw("</html>\n");
    }

    //processes the body of the program which contains the main content and strictire of the HTML
    //keep processing till we reach the end
    fn body(&mut self) {
        while !self.current_tok.is_empty() && !self.current_tok.eq_ignore_ascii_case("#KBYE") {
            self.body_item();
        }
    }
    //processes things that can be in the body of the program
    //comments, paragraphs, headers, lists
    //we check for the tags and then then call the correspomdinh function to process each one
    fn body_item(&mut self) {
        if self.current_tok.eq_ignore_ascii_case("#OBTW") {
            self.comment();
        } else if self.current_tok.eq_ignore_ascii_case("#MAEK HEAD") {
            self.head();
        } else if self.current_tok.eq_ignore_ascii_case("#MAEK PARAGRAF") {
            self.paragraph();
        } else if self.current_tok.eq_ignore_ascii_case("#GIMMEH BOLD") {
            self.bold();
        } else if self.current_tok.eq_ignore_ascii_case("#GIMMEH ITALICS") {
            self.italics();
        } else if self.current_tok.eq_ignore_ascii_case("#MAEK LIST") {
            self.list_block();
        } else if self.current_tok.eq_ignore_ascii_case("#NEWLINE") {
            self.newline_tag();
        } else if self.current_tok.eq_ignore_ascii_case("#GIMMEH LINX") {
            self.link();
        } else if self.current_tok.eq_ignore_ascii_case("#IHAZ") {
            self.variable_definition();
        } else if self.current_tok.eq_ignore_ascii_case("#LEMMESEE") {
            self.variable_usage();
        } else if self.is_text_token() {
            self.text();
        } else {
            //if we find a token that is not valid in the body we return an error and exit
            eprintln!("Syntax error: unexpected token '{}'.", self.current_tok);
            process::exit(1);
        }
    }

    //processes comments in the program
    //&mut self allows us to modify the compiler state while processing the comment
    fn comment(&mut self) {
        self.expect("#OBTW");

        //checking for texts after #OBTW and before #TLDR
        //if text found, store it in var comment_text
        let mut comment_text = String::new();
        if self.is_text_token() {
            comment_text = self.text_value();
            self.next_token();
        }
     //then we check for #TLDR tag
     //if found we output the comment as an HTML comment in the output
        self.expect("#TLDR");

        self.emit_raw("<!-- ");
        self.emit_raw(&comment_text);
        self.emit_raw(" -->\n");
    }
    //head section of the HTML doc
    //only contain title of the oage
    fn head(&mut self) {
        self.expect("#MAEK HEAD");
        self.emit_raw("<head>\n");
        self.title();
        self.emit_raw("</head>\n");
        //after that we exoect MKAY as the closing tag for the head section
        self.expect("#MKAY");
    }
    //processing the title of the page
    fn title(&mut self) {
        self.expect("#GIMMEH TITLE");

        if !self.is_text_token() {
            eprintln!("Syntax error: expected title text, but found '{}'.", self.current_tok);
            process::exit(1);
        }
    //extract the title text from the token and add it to the output as
    //the current title tah in the HTML head section
        let title_text = self.text_value();
        self.emit_raw("<title>");
        self.emit_raw(&title_text);
        self.emit_raw("</title>\n");
    //after processing the title we expect the closing tag for the title which is #OIC
        self.next_token();
        self.expect("#OIC");
    }
    //processing paragraphs in the program
    //paragraphs are the main content of the page
    fn paragraph(&mut self) {
        self.expect("#MAEK PARAGRAF");
        self.enter_scope();
        self.emit_raw("<p>");

        if self.current_tok.eq_ignore_ascii_case("#MKAY") {
            eprintln!("Syntax error: empty paragraph is not allowed.");
            process::exit(1);
        }
    //we keep processing the content of the paragraph until we get #KBYE
        while !self.current_tok.is_empty() && !self.current_tok.eq_ignore_ascii_case("#MKAY") {
            //process each item in the patagraph 
            self.paragraph_item();
        }
        self.expect("#MKAY");
        self.emit_raw("</p>\n");
        self.exit_scope();
    }
    //processing items that are in a paragraph
    fn paragraph_item(&mut self) {
        if self.is_text_token() {
            self.text();
        } else if self.current_tok.eq_ignore_ascii_case("#GIMMEH BOLD") {
            self.bold();
        } else if self.current_tok.eq_ignore_ascii_case("#GIMMEH ITALICS") {
            self.italics();
        } else if self.current_tok.eq_ignore_ascii_case("#MAEK LIST") {
            self.list_block();
        } else if self.current_tok.eq_ignore_ascii_case("#NEWLINE") {
            self.newline_tag();
        } else if self.current_tok.eq_ignore_ascii_case("#GIMMEH LINX") {
            self.link();
        } else if self.current_tok.eq_ignore_ascii_case("#IHAZ") {
            self.variable_definition();
        } else if self.current_tok.eq_ignore_ascii_case("#LEMMESEE") {
            self.variable_usage();
        } else {
            eprintln!("Syntax error: invalid paragraph item '{}'.", self.current_tok);
            process::exit(1);
        }
    }
    //processing bold text in the program
    fn bold(&mut self) {
        self.expect("#GIMMEH BOLD");

        if !self.is_text_token() {
            eprintln!("Syntax error: expected bold text, but found '{}'.", self.current_tok);
            process::exit(1);
        }
    //extract the bold text from the token and add it to the output
        let text = self.text_value();
        self.emit_piece(&format!("<b>{}</b>", text));

        self.next_token();
        self.expect("#OIC");
    }
     //processing italics text in the program
    fn italics(&mut self) {
        self.expect("#GIMMEH ITALICS");

        if !self.is_text_token() {
            eprintln!("Syntax error: expected italics text, but found '{}'.", self.current_tok);
            process::exit(1);
        }
     //extract the italics text from the token and add it to the outpit
     //then we make HTML italics
        let text = self.text_value();
        self.emit_piece(&format!("<i>{}</i>", text));

        self.next_token();
        self.expect("#OIC");
    }
    //processing lists in the program
    fn list_block(&mut self) {
        self.expect("#MAEK LIST");
        self.emit_raw("<ul>\n");

        if !self.current_tok.eq_ignore_ascii_case("#GIMMEH ITEM") {
            eprintln!("Syntax error: list must contain at least one #GIMMEH ITEM.");
            process::exit(1);
        }
        while self.current_tok.eq_ignore_ascii_case("#GIMMEH ITEM") {
            self.item();
        }

        self.expect("#MKAY");
        self.emit_raw("</ul>\n");
    }
    fn item(&mut self) {
        self.expect("#GIMMEH ITEM");
        self.emit_raw("<li>");

        if self.current_tok.eq_ignore_ascii_case("#OIC") {
            eprintln!("Syntax error: empty list item is not allowed.");
            process::exit(1);
        }

        while !self.current_tok.is_empty() && !self.current_tok.eq_ignore_ascii_case("#OIC") {
            self.item_piece();
        }

        self.expect("#OIC");
        self.emit_raw("</li>\n");
    }
    //processing the content of a list item
    //they cannot contain paragraphs or nested lists
    fn item_piece(&mut self) {
        if self.is_text_token() {
            self.text();
        } else if self.current_tok.eq_ignore_ascii_case("#GIMMEH BOLD") {
            self.bold();
        } else if self.current_tok.eq_ignore_ascii_case("#GIMMEH ITALICS") {
            self.italics();
        } else if self.current_tok.eq_ignore_ascii_case("#LEMMESEE") {
            self.variable_usage();
        } else {
            //if we find a token that is not valid in a list item we return an error and exit
            eprintln!("Syntax error: invalid list item content '{}'.", self.current_tok);
            process::exit(1);
        }
    }

    //processing newlines in the program
    fn newline_tag(&mut self) {
        self.expect("#NEWLINE");
        self.emit_raw("<br>\n");
    }

    fn link(&mut self) {
        self.expect("#GIMMEH LINX");

        if !self.is_text_token() {
            eprintln!(
                "Syntax error: expected address text after #GIMMEH LINX, but found '{}'.",
                self.current_tok
            );
            process::exit(1);
        }
        let address = self.text_value();
        self.emit_piece(&format!("<a href=\"{0}\">{0}</a>", address));

        self.next_token();
        self.expect("#OIC");
    }
   
   //processing variable definitions in the program
   //variables are defined with #IHAZ and #ITIZ tags
   //we check for the var name and value and store them to the current svope
    fn variable_definition(&mut self) {
        self.expect("#IHAZ");
    
    //after #IHAZ we expect a var name which is a text token
        if !self.is_text_token() {
            //we dont find a text token after #IHAZ we return an error and exit
            eprintln!(
                "Syntax error: expected variable name after #IHAZ, but found '{}'.",
                self.current_tok
            );
            process::exit(1);
        }
    //extracting the var name from the token and storing it in var_name
    //then move to the next token to continue parsing
    //after the var name we expect #ITIZ tag to indicate the start of the var value
    //if we dont fund #ITIZ we return an error and exit
        let var_name = self.text_value();
        self.next_token();
        self.expect("#ITIZ");
        if !self.is_text_token() {
            eprintln!(
                "Syntax error: expected variable value after #ITIZ, but found '{}'.",
                self.current_tok
            );
            process::exit(1);
        }
 
    //extracting the var value from the token and storing it in var_value
    //then definning the var value in the current scope using the define_variable function
        let var_value = self.text_value();
        self.next_token();

        self.define_variable(var_name, var_value);
        self.expect("#MKAY");
    }

    //processing variable usage in the program
    //when we want to use a var we use #LEMMESEE tag followed by the var name
    //checking if its defined in any scope and if it is we add its value to the output
    //we dont find the var we return an error and exit
    fn variable_usage(&mut self) {
        self.expect("#LEMMESEE");

        if !self.is_text_token() {
            eprintln!(
                "Syntax error: expected variable name after #LEMMESEE, but found '{}'.",
                self.current_tok
            );
            process::exit(1);
        }
      
    //extracting the var name from the token and storing it in var_name
    //looking up for the var value using the lookup_variable function
        let var_name = self.text_value();

        if let Some(value) = self.lookup_variable(&var_name) {
            self.emit_piece(&value);
        } else {
            eprintln!(
                "Static semantic error: variable '{}' was used before it was defined.",
                var_name
            );
            process::exit(1);
        }
    //after processing the var usage we expect the closing tag #MKAY
        self.next_token();
        self.expect("#OIC");
    }

    //clean up the output by removing extra spaces before puncs and tags
    //makes HTML more organised
    //done at the end of parsing after generatung the HTML output
    fn cleanup_output(&mut self) {
        let cleaned = self
            .output
            .replace(" .", ".")
            .replace(" ,", ",")
            .replace(" !", "!")
            .replace(" ?", "?")
            .replace(" :", ":")
            .replace(" ;", ";")
            .replace(" </p>", "</p>")
            .replace(" </li>", "</li>")
            .replace("> </", "></")
            .replace("<p> ", "<p>")
            .replace("<li> ", "<li>");

        self.output = cleaned;
    }
}

//implementation of the compiler trait for the struct
//connecting lexer and parsing logic
impl Compiler for LolcodeCompiler {
    //starting the compilation
    fn compile(&mut self, source: &str) {
        self.lexer = SimpleLexicalAnalyzer::new(source);
    //tokenising the source code using the lexer
        if let Err(err) = self.lexer.tokenize(source) {
            eprintln!("{}", err);
            process::exit(1);
        }

        //after this we start the parsing process
        self.start();
    }
    //getting next token from lexer and checking its validity
    //if valid = current token
    //if invalid = error and exit
    fn next_token(&mut self) -> String {
        let candidate = self.lexer.tokens.pop().unwrap_or_default();
    
    //once end is reached we get an empty string from the lexer tokens vector
    //when we get an empty string we clear the current token to inidicate we have reached the end
        if candidate.is_empty() {
            self.current_tok.clear();
            String::new()
        } else if self.lexer.lookup(&candidate) {
            self.current_tok = candidate.clone();
            candidate
        } else {
            eprintln!("Lexical error: '{}' is not a recognized token.", candidate);
            process::exit(1);
        }
    }

    //parsing 
    //processing program structure snd body
    //then we clean up and output a sucessful message
    fn parse(&mut self) {
        self.program();
        self.cleanup_output();
        //final HTML output stored
        println!("Syntax analysis completed successfully.");
    }
    
    //getting current token during parsing
    fn current_token(&self) -> String {
        //returning a clone of the current token 
        self.current_tok.clone()
    }

    //setting the current token during parsing
    fn set_current_token(&mut self, tok: String) {
        self.current_tok = tok;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        process::exit(1);
    }

    let filename = &args[1];

    if !filename.to_ascii_lowercase().ends_with(".lol") {
        eprintln!("Error: compiler only accepts files with a .lol extension.");
        process::exit(1);
    }

    let source = fs::read_to_string(filename).unwrap_or_else(|err| {
        eprintln!("Error reading file '{}': {}", filename, err);
        process::exit(1);
    });

    let mut compiler = LolcodeCompiler::new();
    compiler.compile(&source);
    compiler.parse();

    let base = &filename[..filename.len() - 4];
    let output_filename = format!("{}.html", base);

    fs::write(&output_filename, &compiler.output).unwrap_or_else(|err| {
        eprintln!("Error writing HTML file '{}': {}", output_filename, err);
        process::exit(1);
    });

    println!("HTML file generated successfully: {}", output_filename);
}