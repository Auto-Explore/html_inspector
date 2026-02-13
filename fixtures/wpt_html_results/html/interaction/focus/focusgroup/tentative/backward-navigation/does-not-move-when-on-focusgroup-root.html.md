# html/interaction/focus/focusgroup/tentative/backward-navigation/does-not-move-when-on-focusgroup-root.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/interaction/focus/focusgroup/tentative/backward-navigation/does-not-move-when-on-focusgroup-root.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTML Test: focusgroup - Focus does not move when initially set on the focusgroup root.</title>
<link rel="author" title="Microsoft" href="http://www.microsoft.com/">
<link rel="help" href="https://github.com/MicrosoftEdge/MSEdgeExplainers/blob/main/Focusgroup/explainer.md">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="../resources/focusgroup-utils.js"></script>

<div id=root tabindex=0 focusgroup="toolbar">
  <span id=item1 tabindex=0>item1</span>
  <span id=item2 tabindex=0>item2</span>
</div>

<script>

  promise_test(async t => {
    var root = document.getElementById("root");

    await focusAndKeyPress(root, kArrowUp);
    assert_equals(document.activeElement, root);

    await focusAndKeyPress(root, kArrowLeft);
    assert_equals(document.activeElement, root);
  }, "When the focus is set on the root of a focusgroup element, an arrow key press shouldn't move the focus at all.");

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
  "source_name": "html/interaction/focus/focusgroup/tentative/backward-navigation/does-not-move-when-on-focusgroup-root.html"
}
```
