# Simmer 🍚

While trying to learn a bit of NLP. I decided to ddo a small crate which implements the [porter stemmer algorithm](https://tartarus.org/martin/PorterStemmer/def.txt?fbclid=IwAR3nCMZAT0Ggg-TGKgb0GBtR_f4ZFtWnbt0FKVmD11Kdf8TCQlpX9GMy3ak)

## Usage 

You can run the example with the command `cargo run --example example`. Otherwise you may refer to the snippet below

```rust
use simmer;

fn main() {
    let stem = simmer::stem("excellent").unwrap();
    assert_eq!(stem, "excel");

    let sentence = simmer::stem_sentence("Alex was an excellent dancer.").unwrap().join(" ");
    assert_eq!(sentence, "alex wa an excel dancer");
}
```

## Resources

Big thanks to the author of these articles which allows me to understand the porter stemmer algorithm

- [Building a porter stemmer (python)](https://medium.com/analytics-vidhya/building-a-stemmer-492e9a128e84)
- [Description of the porter stemmer with schemas](https://vijinimallawaarachchi.com/2017/05/09/porter-stemming-algorithm/?fbclid=IwAR2x4FQ1jM3H2t3P8_H2oiPCyIE7MMDVqsZ-9on8SELAjze1yYssGJwOTE0)
- [NLTK porter stemmer implementation](https://github.com/nltk/nltk/blob/develop/nltk/stem/porter.py)
