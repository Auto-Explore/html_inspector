# html/semantics/popovers/popover-hidden-appearance.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/popovers/popover-hidden-appearance.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8" />
<link rel="author" href="mailto:masonf@chromium.org">
<link rel=help href="https://html.spec.whatwg.org/multipage/popover.html">
<link rel=help href="https://issues.chromium.org/issues/393947100">
<link rel="match" href="/compat/green-ref.html">

<div popover></div>
<div id=blocker></div>

<style>
  div {
    position: fixed;
    inset:auto;
    top:8px;
    left:8px;
    width: 100px;
    height: 100px;
    padding:0;
    border:0;
  }
  [popover] {
    display:block;
    background-color:red;
  }
  #blocker {
    background-color:green;
  }
</style>

<script>
  const popover = document.querySelector('[popover]');
  popover.showPopover();
  popover.hidePopover();
  // The popover should still be display:block, but not in the top layer.
</script>
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
        "byte_end": 339,
        "byte_start": 332,
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
  "source_name": "html/semantics/popovers/popover-hidden-appearance.html"
}
```
