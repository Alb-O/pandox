---
title: "Text Formatting"
category: "Demo"
order: 2
---

# Text Formatting

Markdown supports various text formatting options.

## Bold Text

This is **bold text** and this is __also bold__.

## Italic Text

This is *italic text* and this is _also italic_.

## Combined Styles

This is ***bold and italic*** text.

Normal text with *emphasis* and **strong**.

Combined: ***strong emphasis***.

## Strikethrough

This is ~~strikethrough~~ text.

## Inline Code

You can also use `inline code` within text.

Use the `println!` macro in Rust.

## Callout Boxes

Callout boxes automatically display icons and titles based on their type.

::: {.callout .warning}
This section demonstrates a custom `callout` block. Be careful when using this feature in production.
:::

::: {.callout .info}
This is an informational callout for general notes and tips.
:::

::: {.callout .note}
The `.note` class is an alias for `.info` callouts.
:::

::: {.callout .danger}
This indicates something dangerous or a breaking change.
:::

::: {.callout .success}
This indicates a successful operation or best practice.
:::

::: {.callout .tip}
The `.tip` class is an alias for `.success` callouts.
:::
