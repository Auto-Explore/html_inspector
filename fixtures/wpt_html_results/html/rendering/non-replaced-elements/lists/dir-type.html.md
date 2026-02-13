# html/rendering/non-replaced-elements/lists/dir-type.html

Counts:
- errors: 0
- warnings: 5
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/lists/dir-type.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>&lt;dir&gt; doesn't map the type and start attributes to CSS</title>
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1831863">
<link rel="author" href="mailto:emilio@crisal.io" title="Emilio Cobos Álvarez">
<link rel="author" href="https://mozilla.com" title="Mozilla">
<link rel="match" href="dir-type-ref.html">
<dir type="square">
  <li>A
</dir>
<dir type="decimal" start="5">
  <li>B
</dir>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 400,
        "byte_start": 381,
        "col": 1,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “dir” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 407,
        "byte_start": 403,
        "col": 3,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 446,
        "byte_start": 416,
        "col": 1,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “dir” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 453,
        "byte_start": 449,
        "col": 3,
        "line": 12
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
  "source_name": "html/rendering/non-replaced-elements/lists/dir-type.html"
}
```
