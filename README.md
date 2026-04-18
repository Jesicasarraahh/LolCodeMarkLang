LOLCODE MARKDOWN LANGUAGE COMPILER PROJECT COSC455

Author: Jesica Sarah
___________________________________________________________________________________________________________________________________________________

PROJECT DESCRIPTION:

This project intends to use and apply the concepts of programming languages discussed in class (COSC455) to design and develop 
a relatively complex interpreter/compiler. It translates structured input into HTML output. The compiler is written in Rust programming 
language, and follows the standard compilation phases:

           -Lexical Analysis Phase
           -Syntax Analysis Phase
           -Semantic Analysis Phase
           
The goal of the project is to demonstrate an understanding of:

    -Language design and grammar (BNF/EBNF)
    -Lexical analysis using token recognition
    -Syntax validation and parsing
    -Semantic handling (correct variables or scopes)
    -Code generation
The compiler processes a .lol input file and produces a corresponding .html file that can be put in a web browser.

COMPILATION STRUCTURE/PHASES DESCRIPTION:
___________________________________________________________________________________________________________________________________________________
1. Lexical Analysis:
```   
   -Reads the input individually for each character
   -Groups input into meaningful tokens (keywords, identifiers, text)
   -Recognizes valid language annotations
   -Detects invalid or unknown tokens
```

3. Syntax Analysis(Parser):
```
   -Validates correct ordering and nesting of constructs
   -Ensures required program structure (#HAI...#KBYE)
   -Verifies block-based constructs (#MAEK..#MKAY,#GIMMEH..#OIC)
   -Detects syntax errors such as missing or misplaced keywords
```

4. Semantic Handling:
```
   -Tracks declared variables using a symbol-like structure
   -Ensures variables are defined before usage (#LEMMESEE)
   -Handles reassignment and scoping rules within nested blocks
```

5. Code Generation:
```
   -Converts validated structures into HTML
   -Maps LOLCODE constructs to HTML elements
   -Produces a clean and readable HTML output file
```
__________________________________________________________________________________________________________________________________________________
LANGUAGE FEATURES:

LOLCODE Markdown language will support the following commands (bold is used to
emphasize the syntax and differentiate it from the text):

1. #HAI … #KBYE:-

The #HAI…#KBYE annotations denote the beginning and ending of a valid source file in our
LOLCODE Markdown language. All valid source files must start with #HAI and end with #KBYE
(i.e., there cannot be any text before or after). Between these annotations, all other annotations (or
none at all) may occur except for a repetition of the #HAI or #KBYE annotations and except as
noted below. In HTML, these annotations correspond with the <html> and </html> tags,
respectively.

2. #OBTW … #TLDR:-
   
The #OBTW…#TLDR annotations denote the beginning and ending of a comment in our
LOLCODE Markdown language. The comment annotations are optional in any LOLCODE
Markdown source file may occur immediately after any legal annotation. Within the comment
annotation, only plain text is possible (i.e., no other annotations) and may span several lines. In
HTML, these annotations correspond with the <!-- and --> tags, respectively.

3. #MAEK HEAD … #MKAY:-
   
The head annotations are a container for any head elements. In our LOLCODE Markdown language,
only the #GIMMEH TITLE annotation is allowed. The head tag is not required in a LOLCODE
Markdown file, but if it is present, it must be immediately following the #HAI annotation unless
there are comments between the #HAI annotation and the #MAEK HEAD annotation. In HTML,
these annotations correspond with the <head> and </head> tags, respectively.

4. #GIMMEH TITLE … #OIC:-
   
The title annotations denote the title of the resulting html page that shows up in the browser’s
toolbar. Within these annotations, only plain text is possible (i.e., no other annotations). Title
annotations must occur within #MAEK HEAD annotations. In HTML, these annotations
correspond with the <title> and </title> tags, respectively.

5. #MAEK PARAGRAF … #MKAY:-
   
The paragraph annotations denote the beginning and ending of a paragraph within a LOLCODE
Markdown source file. Within these annotations, the bold, italics, list, item, sounds, and video
(described below) annotations are allowed but not required (note that you cannot have a #MAEK
PARAGRAF annotation within another #MAEK PARAGRAF annotation. 

6. #MAEK LIST … #MKAY:-
   
The list annotations denote the beginning and ending of a bulleted list within the LOLCODE
Markdown source file. This annotation must be immediately followed by the #GIMMEH ITEM
annotation. 

7. #GIMMEH ITEM … #OIC:-
    
The item annotations denote the beginning and ending of a list item within the LOLCODE
Markdown source file. All list item annotations must occur within a #MAEK LIST annotation block.
Within these annotations, the bold and italic annotations are allowed but not required (i.e., it can
just be plain text). 

8. #NEWLINE:-
    
The #NEWLINE annotation, within the LOLCODE Markdown source file, may appear anywhere
within a LOLCODE source document outside of the head and title annotations. 

9. #IHAZ variable name #ITIZ value #MKAY:-
    
This annotation structure denotes the beginning and ending of a variable definition within a
LOLCODE Markdown source file. The #IHAZ … #MKAY annotations contain a variable name
following the #IHAZ annotation.

10. #LEMMESEE variable name #OIC:-
    
This annotation denotes the beginning and ending of the use of a variable within the LOLCODE
Markdown source file. The #LEMMESEE … #OIC annotations must contain only text (denoted
by variable name above), noting the variable value to use. Again, the variable name must only
contain text and is a single word (i.e., no spaces). The #LEMMESEE annotation may occur within
any other annotation block.

____________________________________________________________________________________________________________________________
ERROR HANDLING AND VALIDATION:
Lexical Errors:
```
Unknown or unsupported annotations (e.g., invalid tags)
Invalid token formats
```
Syntax Errors:
```
Missing required keywords (#HAI, #KBYE)
Improper block structure (#MAEK without #MKAY, etc.)
Misordered constructs
Incomplete comment blocks (#OBTW without #TLDR)
```
 Semantic Errors:
 ```
Use of undefined variables
Invalid variable references
```
All errors are printed to the terminal with a clear explanation of the issue.
________________________________________________________________________________________________________________________________________________
HOW TO RUN THE COMPILER:

Step 1: ```Open terminal in the project root directory```
Step 2: ```Run the following command:```
        ```cargo run -- .\tests\TestX.lol```
        ```Replace TestX.lol with the desired test file.```

Step 3:``` Check terminal output:```
If successful → HTML file is generated
If error → descriptive error message is displayed
__________________________________________________________________________________________________________________________________________________
PLATFORM AND ENVIRONMENT:
This project was developed and tested on:

Operating System:```Windows```

Programming Language:```Rust (Cargo build system)```

Execution Environment:```Visual Studio Code (VS Code)```

Browser for Output:```Google Chrome```
