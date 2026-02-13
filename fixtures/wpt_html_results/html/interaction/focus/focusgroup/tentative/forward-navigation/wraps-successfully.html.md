# html/interaction/focus/focusgroup/tentative/forward-navigation/wraps-successfully.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/interaction/focus/focusgroup/tentative/forward-navigation/wraps-successfully.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTML Test: focusgroup - Focus wraps successfully from the last item to the first one.</title>
<link rel="author" title="Microsoft" href="http://www.microsoft.com/">
<link rel="help" href="https://github.com/MicrosoftEdge/MSEdgeExplainers/blob/main/Focusgroup/explainer.md">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="../resources/focusgroup-utils.js"></script>

<div id=root focusgroup="toolbar wrap">
  <span id=item1 tabindex=0>item1</span>
  <span id=item2 tabindex=0>item2</span>
</div>

<script>

  promise_test(async t => {
    var item1 = document.getElementById("item1");
    var item2 = document.getElementById("item2");

    await focusAndKeyPress(item2, kArrowDown);
    assert_equals(document.activeElement, item1);

    await focusAndKeyPress(item2, kArrowRight);
    assert_equals(document.activeElement, item1);
  }, "When the focus is set on the last item of a focusgroup that supports wrapping in the axis of the arrow key pressed, the focus should move back to the first item.");

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
  "source_name": "html/interaction/focus/focusgroup/tentative/forward-navigation/wraps-successfully.html"
}
```
