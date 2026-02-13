# html/interaction/focus/focusgroup/tentative/reading-flow-navigation/flex-visual-order-basic.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/interaction/focus/focusgroup/tentative/reading-flow-navigation/flex-visual-order-basic.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTML Test: focusgroup - Reading flow navigation respects flex visual order</title>
<link rel="author" title="Microsoft" href="http://www.microsoft.com/">
<link rel="help" href="https://open-ui.org/components/scoped-focusgroup.explainer/">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="../resources/focusgroup-utils.js"></script>

<style>
.flex-container {
  display: flex;
  flex-direction: row-reverse;
  reading-flow: flex-visual;
}
</style>

<div class="flex-container" focusgroup="toolbar">
  <span id=item1 tabindex=0>item1</span>
  <span id=item2 tabindex=0>item2</span>
  <span id=item3 tabindex=0>item3</span>
</div>

<script>

  promise_test(async t => {
    // Expected visual order due to flex-direction: row-reverse and reading-flow: flex-visual
    // Visual order: item3, item2, item1
    const elementsInVisualOrder = [
      document.getElementById("item3"),
      document.getElementById("item2"),
      document.getElementById("item1")
    ];

    await assert_arrow_navigation_bidirectional(elementsInVisualOrder);
  }, "Focusgroup navigation should respect reading-flow: flex-visual order instead of DOM order.");

</script>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/interaction/focus/focusgroup/tentative/reading-flow-navigation/flex-visual-order-basic.html"
}
```
