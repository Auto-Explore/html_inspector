# html/semantics/popovers/popover-anchor-display-none.tentative.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/popovers/popover-anchor-display-none.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Tests that a popover can be anchored to an unrendered element.</title>
<link rel=author href="mailto:xiaochengh@chromium.org">
<link rel=help href="https://open-ui.org/components/popover.research.explainer">
<link rel=help href="https://html.spec.whatwg.org/multipage/popover.html">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<div id=popover popover anchor=anchor></div>
<div id=anchor></div>

<style>
  #anchor {
    display: none;
  }
  [popover] {
    inset: auto;
    background: lime;
    padding: 0;
    border: 0;
    width: 100px;
    height: 100px;
    top: anchor(top, 100px);
    left: anchor(left, 100px);
  }
</style>

<script>
test(() => {
  popover.showPopover();
  assert_equals(popover.offsetLeft, 100);
  assert_equals(popover.offsetTop, 100);
});
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
        "byte_end": 511,
        "byte_start": 504,
        "col": 1,
        "line": 13
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
  "source_name": "html/semantics/popovers/popover-anchor-display-none.tentative.html"
}
```
