# html/interaction/focus/focusgroup/tentative/grid-navigation/relayout-before-navigation.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/interaction/focus/focusgroup/tentative/grid-navigation/relayout-before-navigation.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTML Test: focusgroup - Relayout before navigating in a grid</title>
<link rel="author" title="Microsoft" href="http://www.microsoft.com/">
<link rel="help" href="https://github.com/MicrosoftEdge/MSEdgeExplainers/blob/main/Focusgroup/explainer.md">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="../resources/focusgroup-utils.js"></script>
<script>
  function removeMiddleRow() {
    document.getElementById("middle-row").remove();
  }
</script>
<table focusgroup="grid">
  <tr>
    <td id="item1" tabindex="0" onkeydown="removeMiddleRow()">item1</td>
  </tr>
  <tr id="middle-row">
    <td id="item2" tabindex="0">item2</td>
  </tr>
  <tr>
    <td id="item3" tabindex="0">item3</td>
  </tr>
</table>

<script>

  promise_test(async t => {
    var item1 = document.getElementById("item1");
    var item3 = document.getElementById("item3");

    await focusAndKeyPress(item1, kArrowDown);
    assert_equals(document.activeElement, item3);
  }, "Since |item1| removes the middle row on key press, the grid focusgroup should check for a relayout before navigating to the next row.");

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
  "source_name": "html/interaction/focus/focusgroup/tentative/grid-navigation/relayout-before-navigation.html"
}
```
