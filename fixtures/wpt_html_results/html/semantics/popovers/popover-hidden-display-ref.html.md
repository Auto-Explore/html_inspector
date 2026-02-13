# html/semantics/popovers/popover-hidden-display-ref.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/popovers/popover-hidden-display-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<link rel=author href="mailto:masonf@chromium.org">
<link rel="stylesheet" href="resources/popover-styles.css">


<div class=fake-popover>This content should be visible and green</div>
<div class=fake-popover style="top:100px;">This content should be visible and green</div>
<div class=fake-popover style="top:200px;">This content should be visible and green</div>

<style>
  .fake-popover {
    top: 0;
    margin:10px;
    width: 300px;
    height: 50px;
    background: green;
  }
</style>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.style.not_allowed_here",
      "message": "Element “style” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 412,
        "byte_start": 405,
        "col": 1,
        "line": 11
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
  "source_name": "html/semantics/popovers/popover-hidden-display-ref.html"
}
```
