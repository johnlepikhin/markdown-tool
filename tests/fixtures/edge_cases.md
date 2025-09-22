# Edge Cases Document

## Special LaTeX Characters

Text with $ dollar signs, & ampersands, % percent signs, and # hash symbols.

## Empty Elements

### Empty Code Block

```

```

### Empty List

-

## Unicode and Special Characters

Text with émojis 🚀 and ünicöde çharacters.

## Nested Elements

- Item with **bold *and italic* text**
- Item with `code` and [link](http://example.com)

## Very Long Line

This is a very long line that should test the word wrapping functionality of the markdown renderer and ensure that it handles long lines correctly without breaking the formatting or losing any content during the conversion process between different formats.

## Edge Cases in Code

```javascript
// Code with special characters
const regex = /[.*+?^${}()|[\]\\]/g;
const template = `Template with ${variable} and \`backticks\``;
```

## Complex Table

| Header 1 | Header 2 | Header 3 |
|----------|:--------:|---------:|
| Left     | Center   | Right    |
| `code`   | **bold** | *italic* |
| [link](http://example.com) | $math$ | 100% |