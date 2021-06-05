# WORD COUNTER

Quick draft of a simple terminal tool for counting words within a line range on a file, ignoring comments, control flow, symbols, basic types, numbers, etc.

Meant to help analysing code functions, as described in [The Legacy Code Programmer's Toolbox](https://www.amazon.co.uk/Legacy-Code-Programmers-Toolbox-Developers-ebook/dp/B07Y6T2VN1/ref=sr_1_1?crid=27CS53OB5IMD0&dchild=1&keywords=the+legacy+code+programmer%27s+toolbox&qid=1622917732&sprefix=legacy+code+p%2Caps%2C161&sr=8-1)

## Usage

`word_counter <file_name> <start_line> <end_line>`

Will return a list of the words in that line range, order by frecuency, also displaying the number of times the word is used and the percentage it represents.

**NOTE:** Written with C/C++ code in mind

## Missing features
 - Inline comments
 - Block comments