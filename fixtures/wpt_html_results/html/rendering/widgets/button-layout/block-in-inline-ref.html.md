# html/rendering/widgets/button-layout/block-in-inline-ref.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/widgets/button-layout/block-in-inline-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<button style="width: 100px; height: 100px;">
  <div style="width: 100%; height: 100%; background: green;"></div>
</button>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.child.disallowed_in_phrasing_parent",
      "message": "Element “div” not allowed as child of “button” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 123,
        "byte_start": 64,
        "col": 3,
        "line": 3
      }
    },
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/rendering/widgets/button-layout/block-in-inline-ref.html"
}
```
