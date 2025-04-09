[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](https://github.com/Antidote1911/deadpool/blob/master/LICENSE-MIT)
[![made-with-rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg)](https://www.rust-lang.org/)
# 🔑 Deadpool and Shuffle

Deadpool is a crate to generate passwords and Shuffle a command-line application to demonstrate how to use it.
Deadpool is verry simple.

```
use deadpool::*;

let mut pool = Pool::new();
pool.extend_from_uppercase();
pool.extend_from_digits();
pool.extend_from_dashes();
pool.extend_from_string("@é=");
pool.exclude_chars("0Oo1iIlL5S"); // exclude some ambigous
...
...
let password = pool.generate(25)
```


## Usage for shuffle cli application

The generated passwords always contains at least one character from the selected groups and from --include option.
Without argument, the generated password is 10 characters long and uses lowercase letters and numbers.
```
# similar to ./shuffle -ld -L 10
./shuffle
uabhbunf0q
```

Generate 3 passwords with 30 chars with lower,digits,maths and include @ é è à % M
```
./shuffle --count 3 -L 30 -ldm --include "@éèà%M"
0c3mi<l1=Ma6xfujp>ddc3%%*n76èp
3>j+%=?5k*ubyd@p+=wior4a@qhiàu
tz6z99iwà1h!s+Mg4iv5t%@%5kenq8
```

Generate a password with 30 chars with only digits and exclude 0,1,2,3,4 and 5
```
./shuffle -d -L 30 --exclude 012345 
879866968679799766976867796776
```

Note:
The exclude option takes precedence over the include option. A character included with --include will always be removed by exclude.

Display full help with -h flag:

```
./shuffle -h

🔑 Random password generator

Usage: shuffle [OPTIONS]

Options:
  -u, --uppercase          Use UPPERCASE letters [A-Z]
  -l, --lowercase          Use lowercase letters [a-z]
  -d, --digits             Use digits [0-9]
  -b, --braces             Use special symbols [*&^%$#@!~]
  -p, --punctuation        
  -q, --quotes             
      --dashes             
  -m, --math               
      --logograms          
  -C, --count <NUMBER>     Number of passwords to generate [default: 1]
  -L, --length <NUMBER>    Sets the required password length [default: 10]
      --output <OUTPUT>    Output in a txt file
      --exclude <EXCLUDE>  Exclude char
      --include <INCLUDE>  include char
  -h, --help               Print help
  -V, --version            Print version
```
## Build Shuffle
Clone this repo, go in deadpool folder and build it with cargo :
```
git clone https://github.com/Antidote1911/deadpool
cd deadpool
cargo build --release

```
