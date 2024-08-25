lexer:
```rs
struct SrcFile {
    source: Spur,
    file: Spur,
    hash: u64,
}
```
parser:
```rs
struct SrcFile {
    source: Spur,
    file: Spur,
    hash: u64,
    tree: Program,
}
```
analysis:
```rs
struct SrcFile {
    source: Spur,
    file: Spur,
    hash: u64,
    tree: Program,
}
```
