# Test Markdown Document

This is a test document showcasing various markdown features for parser testing.

## Table of Contents

- [Headings](#headings)
- [Text Formatting](#text-formatting)
- [Lists](#lists)
- [Links and Images](#links-and-images)
- [Code](#code)
- [Tables](#tables)
- [Blockquotes](#blockquotes)
- [Horizontal Rules](#horizontal-rules)

# Heading 1
## Heading 2
### Heading 3
#### Heading 4
##### Heading 5
###### Heading 6

## Text Formatting

This is **bold text** and this is __also bold__.

This is *italic text* and this is _also italic_.

This is ***bold and italic*** text.

This is ~~strikethrough~~ text.

You can also use `inline code` within text.

::: {.callout .warning}
**Heads up:** this section demonstrates a custom `callout` block rendered with a bespoke component.
:::

## Lists

### Unordered Lists

- Item 1
- Item 2
  - Nested item 2.1
  - Nested item 2.2
    - Deeply nested item 2.2.1
- Item 3

* Alternative bullet style
+ Another alternative

### Ordered Lists

1. First item
2. Second item
   1. Nested item 2.1
   2. Nested item 2.2
3. Third item

### Task Lists

- [x] Completed task
- [ ] Uncompleted task
- [ ] Another task to do

## Links and Images

[This is a link to Anthropic](https://www.anthropic.com)

[This is a reference-style link][reference]

[reference]: https://www.rust-lang.org

![Alt text for an online image](https://picsum.photos/id/40/4106/2806)

![Local Image](assets/local_image.jpg)

![Local Video](assets/sample_video.mp4)

## Code

### Inline Code

Use the `println!` macro in Rust.

### Code Blocks

```rust
fn main() {
    println!("Hello, world!");

    let numbers = vec![1, 2, 3, 4, 5];
    for num in numbers.iter() {
        println!("{}", num);
    }
}
```

```javascript
function greet(name) {
    console.log(`Hello, ${name}!`);
}

greet("World");
```

```python
def fibonacci(n):
    if n <= 1:
        return n
    return fibonacci(n-1) + fibonacci(n-2)

print(fibonacci(10))
```

## Tables

| Header 1     | Header 2     | Header 3     |
| ------------ | ------------ | ------------ |
| Row 1, Col 1 | Row 1, Col 2 | Row 1, Col 3 |
| Row 2, Col 1 | Row 2, Col 2 | Row 2, Col 3 |
| Row 3, Col 1 | Row 3, Col 2 | Row 3, Col 3 |

### Table with Alignment

| Left Aligned | Center Aligned | Right Aligned |
| :----------- | :------------: | ------------: |
| Left         |     Center     |         Right |
| Text         |      Text      |          Text |

## Blockquotes

> This is a blockquote.
>
> It can span multiple lines.

> Blockquotes can be nested
>> Like this
>>> And even deeper

> You can also use **formatting** in blockquotes.
> - And lists
> - Like this

## Horizontal Rules

---

***

___

## Special Characters and Escaping

You can escape special characters like \* and \_ with backslashes.

## Math (if supported)

Inline math: $E = mc^2$

Block math:

$$
\int_{0}^{\infty} e^{-x^2} dx = \frac{\sqrt{\pi}}{2}
$$

## Footnotes

Here's a sentence with a footnote[^1].

[^1]: This is the footnote content.

## Definition Lists

Term 1
: Definition 1

Term 2
: Definition 2a
: Definition 2b

## Abbreviations

The HTML specification is maintained by the W3C.

*[HTML]: Hyper Text Markup Language
*[W3C]: World Wide Web Consortium

## Emphasis and Strong

Normal text with *emphasis* and **strong**.

Combined: ***strong emphasis***.

## Line Breaks

This is the first line.
This is the second line (two spaces at the end of previous line).

This is a new paragraph.

## HTML

<div style="color: blue;">
This is HTML content in markdown.
</div>

<details>
<summary>Click to expand</summary>

Hidden content goes here.

</details>

## Emojis

:smile: :rocket: :heart:

## Conclusion

This document covers most common markdown features for comprehensive parser testing.
