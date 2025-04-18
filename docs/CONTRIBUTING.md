Prefer newtypes to type aliases.
```rs
// Newtype example
struct Newtype(i32);

// Type alias
type TypeAlias = i32;
```

==Newtypes provide clearer linting and prevent errors caused by functions accepting the incorrect type because it shares an alias.==