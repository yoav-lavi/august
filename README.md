<h1 align="center" style="color:red"><code>.august:true</code></h1>

August (working name) is an Emmet-like language that produces JSON, TOML, or YAML. It allows you to quickly write selectors to create and edit documents. Other targets supported by `serde` can potentially be supported.

August is currently in very early stages.

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

