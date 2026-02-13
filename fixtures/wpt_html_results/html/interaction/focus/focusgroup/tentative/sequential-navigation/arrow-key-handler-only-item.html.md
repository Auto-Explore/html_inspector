# html/interaction/focus/focusgroup/tentative/sequential-navigation/arrow-key-handler-only-item.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/interaction/focus/focusgroup/tentative/sequential-navigation/arrow-key-handler-only-item.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTML Test: focusgroup - Native arrow key handler as only item in segment</title>
<link rel="author" title="Microsoft" href="http://www.microsoft.com/">
<link rel="help" href="https://open-ui.org/components/scoped-focusgroup.explainer/">
<meta name="assert" content="When a native arrow key handler is the only item in a segment, the segment still has an entry point when the element is not focused.">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="../resources/focusgroup-utils.js"></script>

<button id="before">Before</button>
<div id="toolbar" focusgroup="toolbar">
  <input id="search" type="search" placeholder="Search">
</div>
<button id="after">After</button>

<script>

  promise_test(async t => {
    const search = document.getElementById("search");

    search.focus();

    // Arrow keys should not move focus (native arrow key handling).
    await assert_arrow_keys_do_not_move_focus(search);
  }, "Arrow keys do not work from native arrow key handler even when it's the only item");

  promise_test(async t => {
    const before = document.getElementById("before");
    const search = document.getElementById("search");
    const after = document.getElementById("after");

    // Tab navigation: enter at search, exit to after.
    await assert_focusgroup_tab_navigation([before, search, after]);

    // Shift+Tab from after returns to focused search.
    await sendTabBackward();
    assert_equals(document.activeElement, search);
  }, "Segment with only native arrow key handler navigates correctly based on focus state");

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
  "source_name": "html/interaction/focus/focusgroup/tentative/sequential-navigation/arrow-key-handler-only-item.html"
}
```
