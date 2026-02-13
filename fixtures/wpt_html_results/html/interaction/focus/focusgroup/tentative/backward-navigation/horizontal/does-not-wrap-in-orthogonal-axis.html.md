# html/interaction/focus/focusgroup/tentative/backward-navigation/horizontal/does-not-wrap-in-orthogonal-axis.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/interaction/focus/focusgroup/tentative/backward-navigation/horizontal/does-not-wrap-in-orthogonal-axis.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTML Test: focusgroup - Focus does not wrap in the arrow key pressed orthogonal axis.</title>
<link rel="author" title="Microsoft" href="http://www.microsoft.com/">
<link rel="help" href="https://open-ui.org/components/focusgroup.explainer/">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="../../resources/focusgroup-utils.js"></script>

<div focusgroup="toolbar block wrap">
  <span id=item1 tabindex=0>item1</span>
  <span id=item2 tabindex=0>item2</span>
  <span id=item3 tabindex=0>item3</span>
</div>

<script>

  promise_test(async t => {
    var item1 = document.getElementById("item1");

    await focusAndKeyPress(item1, kArrowLeft);
    assert_equals(document.activeElement, item1);
  }, "When the focus is set on the first item of a focusgroup that wraps and supports only the orthogonal axis of the pressed arrow key, a backward arrow key press shouldn't move the focus.");

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
  "source_name": "html/interaction/focus/focusgroup/tentative/backward-navigation/horizontal/does-not-wrap-in-orthogonal-axis.html"
}
```
