# html/interaction/focus/focusgroup/tentative/backward-navigation/skips-root-focusgroup.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/interaction/focus/focusgroup/tentative/backward-navigation/skips-root-focusgroup.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTML Test: focusgroup - Focus moves to previous item, skipping over a subtree that is a root focusgroup (unrelated to the one we're in).</title>
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
  <div>
    <div focusgroup="toolbar">
      <span id=item2 tabindex=0>item2</span>
      <span id=item3 tabindex=0>item3</span>
    </div>
  </div>
  <span id=item4 tabindex=0>item4</span>
</div>

<script>

  promise_test(async t => {
    var item1 = document.getElementById("item1");
    var item4 = document.getElementById("item4");

    await focusAndKeyPress(item4, kArrowUp);
    assert_equals(document.activeElement, item1);

    await focusAndKeyPress(item4, kArrowLeft);
    assert_equals(document.activeElement, item1);
  }, "When the focus is set on the last item of a focusgroup and the previous item is located past an other focusgroup subtree, a backward arrow key press should move the focus to that previous item without getting stuck in the other focusgroup.");

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
  "source_name": "html/interaction/focus/focusgroup/tentative/backward-navigation/skips-root-focusgroup.html"
}
```
