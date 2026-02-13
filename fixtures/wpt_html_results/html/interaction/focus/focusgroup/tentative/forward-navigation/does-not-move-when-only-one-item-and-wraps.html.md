# html/interaction/focus/focusgroup/tentative/forward-navigation/does-not-move-when-only-one-item-and-wraps.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/interaction/focus/focusgroup/tentative/forward-navigation/does-not-move-when-only-one-item-and-wraps.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTML Test: focusgroup - Focus does not move when there is only one item, even though it wraps.</title>
<link rel="author" title="Microsoft" href="http://www.microsoft.com/">
<link rel="help" href="https://github.com/MicrosoftEdge/MSEdgeExplainers/blob/main/Focusgroup/explainer.md">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="../resources/focusgroup-utils.js"></script>

<div focusgroup="toolbar wrap">
  <span id=item1 tabindex=0>item1</span>
</div>

<script>

  promise_test(async t => {
    var item1 = document.getElementById("item1");

    await focusAndKeyPress(item1, kArrowDown);
    assert_equals(document.activeElement, item1);

    await focusAndKeyPress(item1, kArrowRight);
    assert_equals(document.activeElement, item1);
  }, "When the focus is set on the only focusgroup item and the focusgroup wraps in the axis of the arrow key pressed, the focus shouldn't move and we shouldn't get stuck in an infinite loop.");

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
  "source_name": "html/interaction/focus/focusgroup/tentative/forward-navigation/does-not-move-when-only-one-item-and-wraps.html"
}
```
