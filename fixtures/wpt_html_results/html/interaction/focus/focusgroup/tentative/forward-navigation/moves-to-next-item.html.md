# html/interaction/focus/focusgroup/tentative/forward-navigation/moves-to-next-item.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/interaction/focus/focusgroup/tentative/forward-navigation/moves-to-next-item.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTML Test: focusgroup - Focus moves to next focusgroup item.</title>
<link rel="author" title="Microsoft" href="http://www.microsoft.com/">
<link rel="help" href="https://github.com/MicrosoftEdge/MSEdgeExplainers/blob/main/Focusgroup/explainer.md">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="../resources/focusgroup-utils.js"></script>

<div focusgroup="toolbar">
  <span id=item1 tabindex=0>item1</span>
  <span id=item2 tabindex=0>item2</span>
</div>

<script>

  promise_test(async t => {
    var item1 = document.getElementById("item1");
    var item2 = document.getElementById("item2");

    await focusAndKeyPress(item1, kArrowDown);
    assert_equals(document.activeElement, item2);

    await focusAndKeyPress(item1, kArrowRight);
    assert_equals(document.activeElement, item2);
  }, "When the focus is set on a focusgroup item, an arrow key press should move the focus to the next item.");

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
  "source_name": "html/interaction/focus/focusgroup/tentative/forward-navigation/moves-to-next-item.html"
}
```
