# Getting started
`diorite new <project name>` or `diorite init` to initialize current directory

Example: `diorite new test_project`

*Note: Projects use snake_case*

# Project structure
Projects go like this
```
src/
    literally_anything.drt
    ending_with.drt
    even_nested/
        stuff.drt
target/
    out/
        raw/
            pe_Join.json
            fn_explode.json
        commmand/
            pe_Join.mcfunction
            fn_explode.mcfunction
    ...
diorite.toml
```

# Transpile your project
`diorite build [compilation target]` - Writes to the specified target

|name        |short|
|------------|-----|
|player event|pe   |
|entity event|ee   |
|function    |fn   |
|process     |pc   |

Available Compilation Targets:
- `<none>` - defaults to `raw`
- `raw` - Raw JSON (`.json`)
- `compressed` - Compressed JSON (`.bin`)
- `b64` - Raw, but just encoded in base 64 (`.b64`)
- `command` - Just like `b64` but as a runnable commands (`.mcfunction`)

# Or just send it

`diorite send <target>` - Sends the changed templates to the specified target

Available receivers:
- `recode`
- `codeclient`

Optional arguments:
`--all` - Sends all of the templates



