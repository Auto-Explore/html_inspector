# html/interaction/focus/focusgroup/tentative/reading-flow-navigation/simple-mixed.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/interaction/focus/focusgroup/tentative/reading-flow-navigation/simple-mixed.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTML Test: focusgroup - Simple mixed reading flow test</title>
<link rel="author" title="Microsoft" href="http://www.microsoft.com/">
<link rel="help" href="https://open-ui.org/components/scoped-focusgroup.explainer/">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="../resources/focusgroup-utils.js"></script>

<style>
.container {
  display: flex;
  flex-direction: row-reverse;
  reading-flow: flex-visual;
}
</style>

<div focusgroup="toolbar">
  <div class="container">
    <button id="btn3" tabindex="0">Button 3</button>
    <button id="btn2" tabindex="0">Button 2</button>
    <button id="btn1" tabindex="0">Button 1</button>
  </div>
  <button id="btn4" tabindex="0">Button 4</button>
</div>

<script>
  promise_test(async t => {
    // Visual order due to flex-direction: row-reverse and reading-flow: flex-visual
    // Visual order: btn1, btn2, btn3, btn4
    const elementsInVisualOrder = [
      document.getElementById("btn1"),
      document.getElementById("btn2"),
      document.getElementById("btn3"),
      document.getElementById("btn4")
    ];

    await assert_arrow_navigation_bidirectional(elementsInVisualOrder);
  }, "Simple mixed reading flow test.");
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
  "source_name": "html/interaction/focus/focusgroup/tentative/reading-flow-navigation/simple-mixed.html"
}
```
