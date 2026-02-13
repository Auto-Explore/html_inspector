# html/interaction/focus/focusgroup/tentative/grid-navigation/non-table.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/interaction/focus/focusgroup/tentative/grid-navigation/non-table.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTML Test: focusgroup - Validate that Focusgroup doesn't work when not set on a table element</title>
<link rel="author" title="Microsoft" href="http://www.microsoft.com/">
<link rel="help" href="https://github.com/MicrosoftEdge/MSEdgeExplainers/blob/main/Focusgroup/explainer.md">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="../resources/focusgroup-utils.js"></script>

<div focusgroup=grid>
  <span id=nonitem1 tabindex=0>nonitem1</span>
  <span id=nonitem2 tabindex=0>nonitem2</span>
</div>

<script>
  promise_test(async t => {
    var nonitem1 = document.getElementById("nonitem1");

    await focusAndKeyPress(nonitem1, kArrowRight);
    assert_equals(document.activeElement, nonitem1);
  }, "Validates that focusgroup=grid set on a non table layout doesn't become a grid focusgroup nor a linear one.");

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
  "source_name": "html/interaction/focus/focusgroup/tentative/grid-navigation/non-table.html"
}
```
