# html/semantics/embedded-content/the-embed-element/embed-hidden-attribute.html

Counts:
- errors: 6
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-embed-element/embed-hidden-attribute.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>The hidden global presentation attribute within the embed element</title>
<link rel="author" href="mailto:masonf@chromium.org">
<link rel=match  href="embed-hidden-attribute-ref.html">

This embed should be visible (green box):
<embed src='../../../../images/green-256x256.png' type='image/png'></embed>

These should not be visible (no red):
<embed hidden          src='../../../../images/fail.gif' type='image/png'></embed>
<embed hidden=""       src='../../../../images/fail.gif' type='image/png'></embed>
<embed hidden="hidden" src='../../../../images/fail.gif' type='image/png'></embed>
<embed hidden="true"   src='../../../../images/fail.gif' type='image/png'></embed>
<embed hidden="yes"    src='../../../../images/fail.gif' type='image/png'></embed>

<style>
embed {
  display:block;
}
</style>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “embed”.",
      "severity": "Error",
      "span": {
        "byte_end": 326,
        "byte_start": 318,
        "col": 68,
        "line": 7
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “embed”.",
      "severity": "Error",
      "span": {
        "byte_end": 448,
        "byte_start": 440,
        "col": 75,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “embed”.",
      "severity": "Error",
      "span": {
        "byte_end": 531,
        "byte_start": 523,
        "col": 75,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “embed”.",
      "severity": "Error",
      "span": {
        "byte_end": 614,
        "byte_start": 606,
        "col": 75,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “embed”.",
      "severity": "Error",
      "span": {
        "byte_end": 697,
        "byte_start": 689,
        "col": 75,
        "line": 13
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “embed”.",
      "severity": "Error",
      "span": {
        "byte_end": 780,
        "byte_start": 772,
        "col": 75,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.style.not_allowed_here",
      "message": "Element “style” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 789,
        "byte_start": 782,
        "col": 1,
        "line": 16
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/embedded-content/the-embed-element/embed-hidden-attribute.html"
}
```
