# html/semantics/popovers/popover-anchor-change-display-ref.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/popovers/popover-anchor-change-display-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">

<p>There should be a green box attached to the right side of each orange box.</p>
<div class=ex><div class=anchor></div><div class=popover></div></div>
<div class=ex><div class=anchor></div><div class=popover></div></div>

<style>
  .ex {
    margin: 25px;
    font-size: 0;
  }
  .ex div {
    display:inline-block;
    width: 100px;
    height: 100px;
  }
  .anchor {
    background: orange;
  }
  .popover {
    background: lime;
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
        "byte_end": 270,
        "byte_start": 263,
        "col": 1,
        "line": 8
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
  "source_name": "html/semantics/popovers/popover-anchor-change-display-ref.html"
}
```
