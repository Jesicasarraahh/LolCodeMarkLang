use std::collections::HashMap;
use std::env;
use std::fs;
use std::process;

pub trait Compiler {
    fn compile(&mut self, source: &str);
    fn next_token(&mut self) -> String;
    fn parse(&mut self);
    fn current_token(&self) -> String;
    fn set_current_token(&mut self, tok: String);
}

pub struct SimpleLexicalAnalyzer {
    pub tokens: Vec<String>,
    pub known_tags: Vec<String>,
}

impl SimpleLexicalAnalyzer {
    pub fn new(_source: &str) -> Self {
        Self {
            tokens: Vec::new(),
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

    fn starts_with_ignore_case(s: &str, pat: &str) -> bool {
        s.len() >= pat.len() && s[..pat.len()].eq_ignore_ascii_case(pat)
    }

    fn find_ignore_case(s: &str, pat: &str) -> Option<usize> {
        let lower_s = s.to_ascii_lowercase();
        let lower_pat = pat.to_ascii_lowercase();
        lower_s.find(&lower_pat)
    }

    fn match_known_tag<'a>(&self, s: &'a str) -> Option<(String, &'a str)> {
        for tag in &self.known_tags {
            if Self::starts_with_ignore_case(s, tag) {
                let rest = &s[tag.len()..];
                return Some((tag.clone(), rest));
            }
        }
        None
    }

    fn push_text_if_any(&mut self, text: &str) {
        let trimmed = text.trim();
        if !trimmed.is_empty() {
            self.tokens.push(format!("TEXT:{}", trimmed));
        }
    }

    fn tokenize_regular_line(&mut self, line: &str) -> Result<(), String> {
        let mut rest = line;

        while !rest.is_empty() {
            if rest.starts_with('#') {
                if let Some((tag, new_rest)) = self.match_known_tag(rest) {
                    self.tokens.push(tag);
                    rest = new_rest.trim_start();
                } else {
                    return Err(format!(
                        "Lexical error: unknown annotation starting with '{}'",
                        rest
                    ));
                }
            } else {
                let next_tag = rest.find('#').unwrap_or(rest.len());
                let text = &rest[..next_tag];
                self.push_text_if_any(text);
                rest = rest[next_tag..].trim_start();
            }
        }

        Ok(())
    }

    pub fn tokenize(&mut self, source: &str) -> Result<(), String> {
        self.tokens.clear();

        let lines: Vec<&str> = source.lines().collect();
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i].trim();

            if line.is_empty() {
                i += 1;
                continue;
            }

            if line.contains('<') || line.contains('>') || line.contains('&') {
                return Err(format!(
                    "Lexical error: invalid character found in line '{}'",
                    line
                ));
            }

            if Self::starts_with_ignore_case(line, "#OBTW") {
                self.tokens.push("#OBTW".to_string());

                let rest = line[5..].trim();

                if let Some(pos) = Self::find_ignore_case(rest, "#TLDR") {
                    let comment_body = rest[..pos].trim();
                    let after_tldr = rest[pos + 5..].trim();

                    if !comment_body.is_empty() {
                        self.tokens.push(format!("TEXT:{}", comment_body));
                    }

                    self.tokens.push("#TLDR".to_string());

                    if !after_tldr.is_empty() {
                        self.tokenize_regular_line(after_tldr)?;
                    }
                } else {
                    let mut comment_parts: Vec<String> = Vec::new();

                    if !rest.is_empty() {
                        comment_parts.push(rest.to_string());
                    }

                    let mut found_tldr = false;
                    i += 1;

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

        self.tokens.reverse();
        Ok(())
    }

    fn lookup(&self, s: &str) -> bool {
        self.known_tags.iter().any(|tag| tag == s) || s.starts_with("TEXT:")
    }
}

pub struct LolcodeCompiler {
    lexer: SimpleLexicalAnalyzer,
    current_tok: String,
    scopes: Vec<HashMap<String, String>>,
    pub output: String,
}

impl LolcodeCompiler {
    pub fn new() -> Self {
        Self {
            lexer: SimpleLexicalAnalyzer::new(""),
            current_tok: String::new(),
            scopes: vec![HashMap::new()],
            output: String::new(),
        }
    }

    fn start(&mut self) {
        let candidate = self.next_token();
        if candidate.is_empty() {
            eprintln!("User error: the provided file is empty.");
            process::exit(1);
        }
        self.current_tok = candidate;
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
        if self.is_text_token() {
            self.current_tok[5..].trim().to_string()
        } else {
            String::new()
        }
    }

    fn emit_raw(&mut self, s: &str) {
        self.output.push_str(s);
    }

    fn emit_piece(&mut self, s: &str) {
        if !s.is_empty() {
            self.output.push_str(s);
            self.output.push(' ');
        }
    }

    fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn exit_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    fn define_variable(&mut self, name: String, value: String) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, value);
        }
    }

    fn lookup_variable(&self, name: &str) -> Option<String> {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Some(value.clone());
            }
        }
        None
    }

    fn text(&mut self) {
        if self.is_text_token() {
            let value = self.text_value();
            self.emit_piece(&value);
            self.next_token();
        } else {
            eprintln!("Syntax error: expected text, but found '{}'.", self.current_tok);
            process::exit(1);
        }
    }

    fn program(&mut self) {
        if !self.current_tok.eq_ignore_ascii_case("#HAI") {
            eprintln!("Syntax error: program must start with #HAI.");
            process::exit(1);
        }

        self.emit_raw("<html>\n");
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

        self.emit_raw("</html>\n");
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
            eprintln!("Syntax error: unexpected token '{}'.", self.current_tok);
            process::exit(1);
        }
    }

    fn comment(&mut self) {
        self.expect("#OBTW");

        let mut comment_text = String::new();
        if self.is_text_token() {
            comment_text = self.text_value();
            self.next_token();
        }

        self.expect("#TLDR");

        self.emit_raw("<!-- ");
        self.emit_raw(&comment_text);
        self.emit_raw(" -->\n");
    }

    fn head(&mut self) {
        self.expect("#MAEK HEAD");
        self.emit_raw("<head>\n");
        self.title();
        self.emit_raw("</head>\n");
        self.expect("#MKAY");
    }

    fn title(&mut self) {
        self.expect("#GIMMEH TITLE");

        if !self.is_text_token() {
            eprintln!("Syntax error: expected title text, but found '{}'.", self.current_tok);
            process::exit(1);
        }

        let title_text = self.text_value();
        self.emit_raw("<title>");
        self.emit_raw(&title_text);
        self.emit_raw("</title>\n");

        self.next_token();
        self.expect("#OIC");
    }

    fn paragraph(&mut self) {
        self.expect("#MAEK PARAGRAF");
        self.enter_scope();
        self.emit_raw("<p>");

        if self.current_tok.eq_ignore_ascii_case("#MKAY") {
            eprintln!("Syntax error: empty paragraph is not allowed.");
            process::exit(1);
        }

        while !self.current_tok.is_empty() && !self.current_tok.eq_ignore_ascii_case("#MKAY") {
            self.paragraph_item();
        }

        self.expect("#MKAY");
        self.emit_raw("</p>\n");
        self.exit_scope();
    }

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

    fn bold(&mut self) {
        self.expect("#GIMMEH BOLD");

        if !self.is_text_token() {
            eprintln!("Syntax error: expected bold text, but found '{}'.", self.current_tok);
            process::exit(1);
        }

        let text = self.text_value();
        self.emit_piece(&format!("<b>{}</b>", text));

        self.next_token();
        self.expect("#OIC");
    }

    fn italics(&mut self) {
        self.expect("#GIMMEH ITALICS");

        if !self.is_text_token() {
            eprintln!("Syntax error: expected italics text, but found '{}'.", self.current_tok);
            process::exit(1);
        }

        let text = self.text_value();
        self.emit_piece(&format!("<i>{}</i>", text));

        self.next_token();
        self.expect("#OIC");
    }

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

    fn variable_definition(&mut self) {
        self.expect("#IHAZ");

        if !self.is_text_token() {
            eprintln!(
                "Syntax error: expected variable name after #IHAZ, but found '{}'.",
                self.current_tok
            );
            process::exit(1);
        }

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

        let var_value = self.text_value();
        self.next_token();

        self.define_variable(var_name, var_value);
        self.expect("#MKAY");
    }

    fn variable_usage(&mut self) {
        self.expect("#LEMMESEE");

        if !self.is_text_token() {
            eprintln!(
                "Syntax error: expected variable name after #LEMMESEE, but found '{}'.",
                self.current_tok
            );
            process::exit(1);
        }

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

        self.next_token();
        self.expect("#OIC");
    }

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

impl Compiler for LolcodeCompiler {
    fn compile(&mut self, source: &str) {
        self.lexer = SimpleLexicalAnalyzer::new(source);

        if let Err(err) = self.lexer.tokenize(source) {
            eprintln!("{}", err);
            process::exit(1);
        }

        self.start();
    }

    fn next_token(&mut self) -> String {
        let candidate = self.lexer.tokens.pop().unwrap_or_default();

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

    fn parse(&mut self) {
        self.program();
        self.cleanup_output();
        println!("Syntax analysis completed successfully.");
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

    let base = &filename[..filename.len() - 4];
    let output_filename = format!("{}.html", base);

    fs::write(&output_filename, &compiler.output).unwrap_or_else(|err| {
        eprintln!("Error writing HTML file '{}': {}", output_filename, err);
        process::exit(1);
    });

    println!("HTML file generated successfully: {}", output_filename);
}