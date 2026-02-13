# html/semantics/popovers/popover-stacking-context-ref.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/popovers/popover-stacking-context-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<link rel="author" href="mailto:masonf@chromium.org">
<link rel="stylesheet" href="resources/popover-styles.css">

<div class="fake-popover">
  Inside popover
  <div class=z style="z-index: 2; background:lightgreen">z-index 2
    <div class=z style="z-index: 3; background:lightblue; left: 20px;">z-index 3</div>
    <div class=z style="z-index: 1; background:pink; top:-20px; left: 10px;">z-index 1</div>
  </div>
  <div class=z style="background:green; top:-100px; left: 250px; width: 100px;">Outside</div>
  Bottom of popover
</div>

<style>
  .fake-popover {
    width: 200px;
    height: 230px;
    border: 1px solid red;
    top:50px;
    left:50px;
  }
  .z {
    position: relative;
    border: 1px solid black;
    padding: 1em;
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
        "byte_end": 583,
        "byte_start": 576,
        "col": 1,
        "line": 16
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
  "source_name": "html/semantics/popovers/popover-stacking-context-ref.html"
}
```
