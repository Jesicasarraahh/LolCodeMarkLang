use std::collections::HashMap;
use std::env;
use std::fs;
use std::process;

//
// ==================== Compiler Trait ====================
//
pub trait Compiler {
    fn compile(&mut self, source: &str);
    fn next_token(&mut self) -> String;
    fn parse(&mut self);
    fn current_token(&self) -> String;
    fn set_current_token(&mut self, tok: String);
}

//
// ================= Lexical Analyzer Trait =================
//
pub trait LexicalAnalyzer {
    fn get_char(&mut self) -> char;
    fn add_char(&mut self, c: char);
    fn lookup(&self, s: &str) -> bool;
}

//
// ============ Concrete Lexical Analyzer ===================
//
pub struct SimpleLexicalAnalyzer {
    input: Vec<char>,
    position: usize,
    current_build: String,
    pub tokens: Vec<String>,
    pub known_tags: Vec<String>,
}

impl SimpleLexicalAnalyzer {
    pub fn new(source: &str) -> Self {
        Self {
            input: source.chars().collect(),
            position: 0,
            current_build: String::new(),
            tokens: Vec::new(),
            // longest tags first helps matching
            known_tags: vec![
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

    fn peek_char(&self) -> char {
        if self.position < self.input.len() {
            self.input[self.position]
        } else {
            '\0'
        }
    }

    fn matches_tag_ignore_case(&self, tag: &str) -> bool {
        let tag_chars: Vec<char> = tag.chars().collect();

        if self.position + tag_chars.len() > self.input.len() {
            return false;
        }

        for (i, tag_ch) in tag_chars.iter().enumerate() {
            let source_ch = self.input[self.position + i];
            if !source_ch.eq_ignore_ascii_case(tag_ch) {
                return false;
            }
        }

        true
    }

    fn flush_text(&mut self) {
        let text = self.current_build.trim().to_string();
        if !text.is_empty() {
            self.tokens.push(format!("TEXT:{}", text));
        }
        self.current_build.clear();
    }

    fn read_tag(&mut self) -> Result<(), String> {
        for tag in &self.known_tags {
            if self.matches_tag_ignore_case(tag) {
                self.position += tag.chars().count();
                self.tokens.push(tag.clone());
                return Ok(());
            }
        }

        Err(format!(
            "Lexical error at character {}: unknown annotation starting with '#'",
            self.position + 1
        ))
    }

    pub fn tokenize(&mut self) -> Result<(), String> {
        while self.peek_char() != '\0' {
            let c = self.peek_char();

            if c == '#' {
                self.flush_text();
                self.read_tag()?;
            } else {
                let ch = self.get_char();

                // professor said these can be assumed not to appear
                if ch == '<' || ch == '>' || ch == '&' {
                    return Err(format!(
                        "Lexical error at character {}: invalid character '{}'",
                        self.position,
                        ch
                    ));
                }

                self.add_char(ch);
            }
        }

        self.flush_text();
        self.tokens.reverse(); // so pop() reads in original order
        Ok(())
    }
}

impl LexicalAnalyzer for SimpleLexicalAnalyzer {
    fn get_char(&mut self) -> char {
        if self.position < self.input.len() {
            let c = self.input[self.position];
            self.position += 1;
            c
        } else {
            '\0'
        }
    }

    fn add_char(&mut self, c: char) {
        self.current_build.push(c);
    }

    fn lookup(&self, s: &str) -> bool {
        self.known_tags.iter().any(|tag| tag == s) || s.starts_with("TEXT:")
    }
}

//
// ==================== LOLCODE Compiler ====================
//
pub struct LolcodeCompiler {
    lexer: SimpleLexicalAnalyzer,
    current_tok: String,
    variables: HashMap<String, String>,
    pub output: String,
}

impl LolcodeCompiler {
    pub fn new() -> Self {
        Self {
            lexer: SimpleLexicalAnalyzer::new(""),
            current_tok: String::new(),
            variables: HashMap::new(),
            output: String::new(),
        }
    }

    fn start(&mut self) {
        let candidate = self.next_token();

        if !candidate.is_empty() {
            self.current_tok = candidate;
        } else {
            eprintln!("User error: the provided file is empty.");
            process::exit(1);
        }
    }

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

    fn is_text_token(&self) -> bool {
        self.current_tok.starts_with("TEXT:")
    }

    fn text_value(&self) -> String {
        if self.current_tok.starts_with("TEXT:") {
            self.current_tok[5..].trim().to_string()
        } else {
            String::new()
        }
    }

    fn emit_text(&mut self, s: &str) {
        self.output.push_str(s);
    }

    fn emit_text_with_space(&mut self, s: &str) {
        self.output.push_str(s);
        self.output.push(' ');
    }

    fn text(&mut self) {
        if self.is_text_token() {
            let val = self.text_value();
            self.emit_text_with_space(&val);
            self.next_token();
        } else {
            eprintln!("Syntax error: expected TEXT, but found '{}'.", self.current_tok);
            process::exit(1);
        }
    }

    fn program(&mut self) {
        if !self.current_tok.eq_ignore_ascii_case("#HAI") {
            eprintln!("Syntax error: program must start with #HAI.");
            process::exit(1);
        }

        self.emit_text("<html>\n");
        self.expect("#HAI");
        self.body();

        if !self.current_tok.eq_ignore_ascii_case("#KBYE") {
            eprintln!("Syntax error: program must end with #KBYE.");
            process::exit(1);
        }

        self.expect("#KBYE");

        if !self.current_tok.is_empty() {
            eprintln!(
                "Syntax error: extra tokens found after #KBYE: '{}'.",
                self.current_tok
            );
            process::exit(1);
        }

        self.emit_text("</html>\n");
    }

    fn body(&mut self) {
        while !self.current_tok.is_empty() && !self.current_tok.eq_ignore_ascii_case("#KBYE") {
            self.body_item();
        }
    }

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
            self.list();
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
            eprintln!("Syntax error: unexpected token '{}'.", self.current_tok);
            process::exit(1);
        }
    }

    fn comment(&mut self) {
        self.expect("#OBTW");

        if !self.is_text_token() {
            eprintln!("Syntax error: expected comment text, found '{}'.", self.current_tok);
            process::exit(1);
        }

        let comment_text = self.text_value();
        self.emit_text("<!-- ");
        self.emit_text(&comment_text);
        self.emit_text(" -->\n");

        self.next_token();
        self.expect("#TLDR");
    }

    fn head(&mut self) {
        self.expect("#MAEK HEAD");
        self.emit_text("<head>\n");
        self.title();
        self.emit_text("</head>\n");
        self.expect("#MKAY");
    }

    fn title(&mut self) {
        self.expect("#GIMMEH TITLE");

        if !self.is_text_token() {
            eprintln!("Syntax error: expected title text, found '{}'.", self.current_tok);
            process::exit(1);
        }

        let title_text = self.text_value();
        self.emit_text("<title>");
        self.emit_text(&title_text);
        self.emit_text("</title>\n");

        self.next_token();
        self.expect("#OIC");
    }

    fn paragraph(&mut self) {
        self.expect("#MAEK PARAGRAF");
        self.emit_text("<p>");
        self.paragraph_body();
        self.emit_text("</p>\n");
        self.expect("#MKAY");
    }

    fn paragraph_body(&mut self) {
        if self.current_tok.eq_ignore_ascii_case("#IHAZ") {
            self.variable_definition();
        }

        self.paragraph_content();
    }

    fn paragraph_content(&mut self) {
        while !self.current_tok.is_empty() && !self.current_tok.eq_ignore_ascii_case("#MKAY") {
            self.paragraph_item();
        }
    }

    fn paragraph_item(&mut self) {
        if self.is_text_token() {
            self.text();
        } else if self.current_tok.eq_ignore_ascii_case("#GIMMEH BOLD") {
            self.bold();
        } else if self.current_tok.eq_ignore_ascii_case("#GIMMEH ITALICS") {
            self.italics();
        } else if self.current_tok.eq_ignore_ascii_case("#MAEK LIST") {
            self.list();
        } else if self.current_tok.eq_ignore_ascii_case("#NEWLINE") {
            self.newline_tag();
        } else if self.current_tok.eq_ignore_ascii_case("#GIMMEH LINX") {
            self.link();
        } else if self.current_tok.eq_ignore_ascii_case("#LEMMESEE") {
            self.variable_usage();
        } else {
            eprintln!("Syntax error: invalid paragraph item '{}'.", self.current_tok);
            process::exit(1);
        }
    }

    fn bold(&mut self) {
        self.expect("#GIMMEH BOLD");

        if !self.is_text_token() {
            eprintln!("Syntax error: expected bold text, found '{}'.", self.current_tok);
            process::exit(1);
        }

        let bold_text = self.text_value();
        self.emit_text("<b>");
        self.emit_text(&bold_text);
        self.emit_text("</b>");

        self.next_token();
        self.expect("#OIC");
    }

    fn italics(&mut self) {
        self.expect("#GIMMEH ITALICS");

        if !self.is_text_token() {
            eprintln!("Syntax error: expected italics text, found '{}'.", self.current_tok);
            process::exit(1);
        }

        let italics_text = self.text_value();
        self.emit_text("<i>");
        self.emit_text(&italics_text);
        self.emit_text("</i>");

        self.next_token();
        self.expect("#OIC");
    }

    fn list(&mut self) {
        self.expect("#MAEK LIST");
        self.emit_text("<ul>\n");
        self.item_list();
        self.emit_text("</ul>\n");
        self.expect("#MKAY");
    }

    fn item_list(&mut self) {
        if !self.current_tok.eq_ignore_ascii_case("#GIMMEH ITEM") {
            eprintln!("Syntax error: list must contain at least one #GIMMEH ITEM.");
            process::exit(1);
        }

        while self.current_tok.eq_ignore_ascii_case("#GIMMEH ITEM") {
            self.item();
        }
    }

    fn item(&mut self) {
        self.expect("#GIMMEH ITEM");
        self.emit_text("<li>");
        self.item_content();
        self.emit_text("</li>\n");
        self.expect("#OIC");
    }

    fn item_content(&mut self) {
        if self.current_tok.eq_ignore_ascii_case("#OIC") {
            eprintln!("Syntax error: list item cannot be empty.");
            process::exit(1);
        }

        while !self.current_tok.is_empty() && !self.current_tok.eq_ignore_ascii_case("#OIC") {
            self.item_piece();
        }
    }

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
            eprintln!("Syntax error: invalid list item content '{}'.", self.current_tok);
            process::exit(1);
        }
    }

    fn newline_tag(&mut self) {
        self.expect("#NEWLINE");
        self.emit_text("<br>\n");
    }

    fn link(&mut self) {
        self.expect("#GIMMEH LINX");

        if !self.is_text_token() {
            eprintln!(
                "Syntax error: expected address text after #GIMMEH LINX, found '{}'.",
                self.current_tok
            );
            process::exit(1);
        }

        let address = self.text_value();
        self.emit_text(&format!("<a href=\"{0}\">{0}</a>", address));

        self.next_token();
        self.expect("#OIC");
    }

    fn variable_definition(&mut self) {
        self.expect("#IHAZ");

        if !self.is_text_token() {
            eprintln!(
                "Semantic/Syntax error: expected variable name after #IHAZ, found '{}'.",
                self.current_tok
            );
            process::exit(1);
        }

        let var_name = self.text_value();
        self.next_token();

        self.expect("#ITIZ");

        if !self.is_text_token() {
            eprintln!(
                "Semantic/Syntax error: expected variable value after #ITIZ, found '{}'.",
                self.current_tok
            );
            process::exit(1);
        }

        let var_value = self.text_value();
        self.next_token();

        self.variables.insert(var_name, var_value);

        self.expect("#MKAY");
    }

    fn variable_usage(&mut self) {
        self.expect("#LEMMESEE");

        if !self.is_text_token() {
            eprintln!(
                "Semantic/Syntax error: expected variable name after #LEMMESEE, found '{}'.",
                self.current_tok
            );
            process::exit(1);
        }

        let var_name = self.text_value();

        if !self.variables.contains_key(&var_name) {
            eprintln!(
                "Static semantic error: variable '{}' was used before it was defined.",
                var_name
            );
            process::exit(1);
        }

        let value = self.variables.get(&var_name).unwrap().clone();
        self.emit_text_with_space(&value);

        self.next_token();
        self.expect("#OIC");
    }
}

impl Compiler for LolcodeCompiler {
    fn compile(&mut self, source: &str) {
        self.lexer = SimpleLexicalAnalyzer::new(source);

        if let Err(err) = self.lexer.tokenize() {
            eprintln!("{}", err);
            process::exit(1);
        }

        self.start();
    }

    fn next_token(&mut self) -> String {
        let candidate = self.lexer.tokens.pop().unwrap_or_default();

        if self.lexer.lookup(&candidate) {
            self.current_tok = candidate.clone();
            candidate
        } else if self.lexer.tokens.is_empty() && candidate.is_empty() {
            self.current_tok.clear();
            String::new()
        } else {
            eprintln!("Lexical error: '{}' is not a recognized token.", candidate);
            process::exit(1);
        }
    }

    fn parse(&mut self) {
        self.program();
        println!("Syntax analysis completed successfully.");

        let cleaned = self.output.replace(" </p>", "</p>");
        self.output = cleaned;
    }

    fn current_token(&self) -> String {
        self.current_tok.clone()
    }

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

    let output_filename = filename.replace(".lol", ".html");

    fs::write(&output_filename, &compiler.output).unwrap_or_else(|err| {
        eprintln!("Error writing HTML file '{}': {}", output_filename, err);
        process::exit(1);
    });

    println!("HTML file generated successfully: {}", output_filename);
}