# html/interaction/focus/focusgroup/tentative/forward-navigation/horizontal/moves-when-only-current-axis-supported.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/interaction/focus/focusgroup/tentative/forward-navigation/horizontal/moves-when-only-current-axis-supported.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTML Test: focusgroup - Focus moves to next focusgroup item when only the axis of the arrow key pressed is supported.</title>
<link rel="author" title="Microsoft" href="http://www.microsoft.com/">
<link rel="help" href="https://open-ui.org/components/focusgroup.explainer/">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="../../resources/focusgroup-utils.js"></script>

<div id=root focusgroup="toolbar inline">
  <span id=item1 tabindex=0>item1</span>
  <span id=item2 tabindex=0>item2</span>
</div>

<script>

  promise_test(async t => {
    var item1 = document.getElementById("item1");
    var item2 = document.getElementById("item2");

    await focusAndKeyPress(item1, kArrowRight);
    assert_equals(document.activeElement, item2);
  }, "When the focus is set on an item of a focusgroup that only supports the axis of the arrow key pressed the focus should move.");

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
  "source_name": "html/interaction/focus/focusgroup/tentative/forward-navigation/horizontal/moves-when-only-current-axis-supported.html"
}
```
