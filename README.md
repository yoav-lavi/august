# August (working title)

August is an Emmet-like language that produces JSON, TOML, and YAML (and potentially Nix). It allows you to quickly write selectors to create and edit documents. Other targets supported by `serde` can potentially be supported.

The current code is a small POC to play with the syntax, there is no functional CLI / editor extension (yet).

Please feel free to open issues with syntax suggestions / general suggestions if you want to become involved!

## Examples (syntax not final)

```
Input

.semi:true+trailingComma:all+singleQuote:true+printwidth:120+tabwidth:2+ignored:.hello:world

Output

---JSON---

{
  "ignored": {
    "hello": "world"
  },
  "tabwidth": "2",
  "trailingComma": "all",
  "singleQuote": "true",
  "semi": "true",
  "printwidth": "120"
}

---TOML---

tabwidth = "2"
trailingComma = "all"
singleQuote = "true"
semi = "true"
printwidth = "120"

[ignored]
hello = "world"


---YAML---

ignored:
  hello: world
tabwidth: '2'
trailingComma: all
singleQuote: 'true'
semi: 'true'
printwidth: '120'
```

```
Input

.this:.is:.deeply:.nested:.indeed:.how:odd

Output

---JSON---

{
  "this": {
    "is": {
      "deeply": {
        "nested": {
          "indeed": {
            "how": "odd"
          }
        }
      }
    }
  }
}

---TOML---

[this.is.deeply.nested.indeed]
how = "odd"


---YAML---

this:
  is:
    deeply:
      nested:
        indeed:
          how: odd
```
