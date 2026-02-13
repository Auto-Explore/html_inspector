# html/dom/elements/global-attributes/the-anchor-attribute-003-crash.tentative.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/elements/global-attributes/the-anchor-attribute-003-crash.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Tests that using the `anchor` attribute with inline containing block does not crash</title>
<link rel="help" href="https://github.com/whatwg/html/pull/9144">
<link rel="help" href="https://crbug.com/1486148">
<link rel="author" href="mailto:xiaochengh@chromium.org">

<style>
.target {
  position: absolute;
  top: anchor(top);
  left: anchor(right);
}
</style>

<div id="anchor">foo</div>

<span style="position: relative">
  <div anchor="anchor2" class="target">bar</div>
</span>

<span style="position: sticky">
  <div anchor="anchor2" class="target">bar</div>
</span>

<span style="backdrop-filter: blur(1px)">
  <div anchor="anchor2" class="target">bar</div>
</span>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.child.disallowed_in_phrasing_parent",
      "message": "Element “div” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 487,
        "byte_start": 450,
        "col": 3,
        "line": 18
      }
    },
    {
      "category": "Html",
      "code": "html.child.disallowed_in_phrasing_parent",
      "message": "Element “div” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 577,
        "byte_start": 540,
        "col": 3,
        "line": 22
      }
    },
    {
      "category": "Html",
      "code": "html.child.disallowed_in_phrasing_parent",
      "message": "Element “div” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 677,
        "byte_start": 640,
        "col": 3,
        "line": 26
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
  "source_name": "html/dom/elements/global-attributes/the-anchor-attribute-003-crash.tentative.html"
}
```
