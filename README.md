includes 3 commands: grep, echo, ls


echo display text on the screen:


cargo run echo [TEXT TO ECHO]


EXAMPLE:

cargo run echo hello world!

hello world!

grep searches for words in the text:

EXAMPLE:

cargo run grep text.txt любовь:

любовь моя —

что душу цветущую любовью выжег,

cargo run grep [FILE PATH] [TEXT TO SEARCH]

and ls outputs the full directory:

cargo run ls [DIRECTORY]
