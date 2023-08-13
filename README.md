# August (working title)

August is an Emmet-like language that produces JSON, TOML, and YAML (and potentially Nix). It allows you to quickly write selectors to create and edit documents. Other targets supported by `serde` can potentially be supported.

The current code is a small POC to play with the syntax.

Please feel free to open issues with syntax suggestions / general suggestions if you want to become involved!

## Examples (syntax not final)

### Prettier-esque

```
.semi:true,trailingComma:all,singleQuote:true,printwidth:120,tabwidth:2,ignored:.hello:world
````

JSON Output

```json
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
```

TOML Output

```toml
tabwidth = "2"
trailingComma = "all"
singleQuote = "true"
semi = "true"
printwidth = "120"

[ignored]
hello = "world"
```

YAML Output

```yaml
ignored:
  hello: world
tabwidth: '2'
trailingComma: all
singleQuote: 'true'
semi: 'true'
printwidth: '120'
```

### Deeply Nested

Input

```
.this:.is:.deeply:.nested:.indeed:.how:odd
```

JSON Output

```json
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
```

TOML Output

```toml
[this.is.deeply.nested.indeed]
how = "odd"
```

YAML Output

```yaml
this:
  is:
    deeply:
      nested:
        indeed:
          how: odd
```

