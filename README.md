<h1 align="center"><code>.august:true</code></h1>

August (working name) is an [Emmet](https://github.com/emmetio/emmet)-like language that produces JSON, TOML, or YAML from a single-line concise selector-like syntax. 

August is currently in very early stages. The repository currently contains a CLI under `crates/cli`, which will soon be published in various forms. Editor extensions and a playground will be supported later on.

Please feel free to open issues with syntax suggestions / general suggestions if you want to become involved!

## Features

- Values that are alphanumeric (may be expanded) do not need to be quoted
- Familiar JSON-like operators (`.`, `,`, `:`)
- Easy to type (close to the home row) operators (`^` being the current exception, may change)
- Context aware: `.true:true` becomes `{ "true": true }`
- Supports exiting a context (object or list) via `^` to the parent scope
- More document types can be added in the future
- Other features TBD!

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

## Syntax (current)

- `.` - opens a new object (similar to `{` in JSON`)
- `:` - sets a value for a key
- `>` - opens a new list (similar to `[` in JSON)
- `^` - climb up to the parent scope (object or list)
- `identifier` - an unquoted string, can be used for keys and values
- `"this is my identifier"` - a quoted string, can be used for keys and values
- `true`, `false`, `null` - similar to JSON counterparts
- `\` - escapes a character within a quoted string (e.g. `\"`)

## CLI Usage (CLI command TBD)

```sh
ag -o / --output [OUTPUT] (json|toml|yaml) [INPUT]
```
