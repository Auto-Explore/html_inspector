# html/interaction/focus/focusgroup/tentative/sequential-navigation/arrow-key-handler-multiple-in-segment.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/interaction/focus/focusgroup/tentative/sequential-navigation/arrow-key-handler-multiple-in-segment.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTML Test: focusgroup - Multiple native arrow key handlers in same segment</title>
<link rel="author" title="Microsoft" href="http://www.microsoft.com/">
<link rel="help" href="https://open-ui.org/components/scoped-focusgroup.explainer/">
<meta name="assert" content="Multiple native arrow key handlers can exist in a segment, each blocks arrow navigation independently.">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="../resources/focusgroup-utils.js"></script>

<div id="toolbar" focusgroup="toolbar">
  <button id="btn1">Button 1</button>
  <input id="input1" type="text" placeholder="First">
  <button id="btn2">Button 2</button>
  <select id="select1">
    <option>Option 1</option>
    <option>Option 2</option>
  </select>
  <button id="btn3">Button 3</button>
</div>

<script>

  promise_test(async t => {
    const btn1 = document.getElementById("btn1");
    const input1 = document.getElementById("input1");
    const btn2 = document.getElementById("btn2");
    const select1 = document.getElementById("select1");

    // Arrow navigation TO native arrow key handlers works.
    await focusAndKeyPress(btn1, kArrowRight);
    assert_equals(document.activeElement, input1);
    await focusAndKeyPress(btn2, kArrowRight);
    assert_equals(document.activeElement, select1);

    // Each native arrow key handler blocks arrow navigation when focused.
    await assert_arrow_keys_do_not_move_focus(input1);
    await assert_arrow_keys_do_not_move_focus(select1);
  }, "Multiple native arrow key handlers each block arrow navigation independently");

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
  "source_name": "html/interaction/focus/focusgroup/tentative/sequential-navigation/arrow-key-handler-multiple-in-segment.html"
}
```
