# Language detector written in rust

## Supports
- English
- French
- *Turkish (partial)*
- *German (partial)*

## Usage
```bash
langdetect <file>
```

## How it works
*Check out my other repo to see how i collected the data <https://github.com/fkgmdev/markov-chains>*
So, first of all i got like 15 books per language from [Project Gutenberg]<gutenberg.org>.  
Then, i analyzed them with another program written in rust.  
I checked the probabilities of a consonant following a vowel or vice versa or double consonants/vowels and the total consonant/vowel ratio.  
These numbers were unique to their languages.  
This program analyzes the inputted text the same way and compares it to language profiles.  
Closest profile wins.

## To-do
- Get more data on Turkish and German
- Add Spanish and Italian
- Maybe? replace this algorithm with a ML algorithm