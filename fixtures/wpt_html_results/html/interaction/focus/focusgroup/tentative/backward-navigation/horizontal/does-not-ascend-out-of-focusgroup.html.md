# html/interaction/focus/focusgroup/tentative/backward-navigation/horizontal/does-not-ascend-out-of-focusgroup.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/interaction/focus/focusgroup/tentative/backward-navigation/horizontal/does-not-ascend-out-of-focusgroup.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTML Test: focusgroup - Focus does not ascend out of current focusgroup if it does not extend the parent focusgroup.</title>
<link rel="author" title="Microsoft" href="http://www.microsoft.com/">
<link rel="help" href="https://open-ui.org/components/focusgroup.explainer/">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="../../resources/focusgroup-utils.js"></script>

<ul focusgroup="toolbar inline">
  <li id=item1 tabindex="0">
    <ul focusgroup="toolbar block">
      <li id=item2 tabindex="0">item2</li>
    </ul>
  </li>
</ul>

<script>

  promise_test(async t => {
    var item2 = document.getElementById("item2");

    await focusAndKeyPress(item2, kArrowLeft);
    assert_equals(document.activeElement, item2);
  }, "When the focus is set on an element in a nested focusgroup that doesn't support the navigation direction, focus should remain on that element and not ascend to the parent focusgroup.");

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
  "source_name": "html/interaction/focus/focusgroup/tentative/backward-navigation/horizontal/does-not-ascend-out-of-focusgroup.html"
}
```
