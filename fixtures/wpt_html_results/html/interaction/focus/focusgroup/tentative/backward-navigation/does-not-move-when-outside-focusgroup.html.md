# html/interaction/focus/focusgroup/tentative/backward-navigation/does-not-move-when-outside-focusgroup.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/interaction/focus/focusgroup/tentative/backward-navigation/does-not-move-when-outside-focusgroup.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTML Test: focusgroup - Focus does not move when initially set on an element not included in the focusgroup.</title>
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
<span id=out tabindex=0>out</span>

<script>

  promise_test(async t => {
    var out = document.getElementById("out");

    await focusAndKeyPress(out, kArrowUp);
    assert_equals(document.activeElement, out);

    await focusAndKeyPress(out, kArrowLeft);
    assert_equals(document.activeElement, out);
  }, "When the focus is set on an element outside of the focusgroup, an arrow keypress shouldn't move the focus at all.");

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
  "source_name": "html/interaction/focus/focusgroup/tentative/backward-navigation/does-not-move-when-outside-focusgroup.html"
}
```
