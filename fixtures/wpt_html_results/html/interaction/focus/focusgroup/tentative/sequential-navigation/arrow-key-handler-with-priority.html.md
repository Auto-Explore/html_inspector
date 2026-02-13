# html/interaction/focus/focusgroup/tentative/sequential-navigation/arrow-key-handler-with-priority.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/interaction/focus/focusgroup/tentative/sequential-navigation/arrow-key-handler-with-priority.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTML Test: focusgroup - focusgroupstart on native arrow key handler</title>
<link rel="author" title="Microsoft" href="http://www.microsoft.com/">
<link rel="help" href="https://open-ui.org/components/scoped-focusgroup.explainer/">
<meta name="assert" content="focusgroupstart on a native arrow key handler makes it the Tab entry point. Tab then traverses remaining items before exiting.">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="../resources/focusgroup-utils.js"></script>

<button id="before">Before</button>
<div id="toolbar" focusgroup="toolbar">
  <button id="btn1">Button 1</button>
  <input id="search" type="text" focusgroupstart placeholder="Search">
  <button id="btn2">Button 2</button>
</div>
<button id="after">After</button>

<script>
  promise_test(async t => {
    const before = document.getElementById("before");
    const search = document.getElementById("search");
    const btn2 = document.getElementById("btn2");
    const after = document.getElementById("after");

    // Focus moves to search (focusgroupstart) on Tab, then moves to btn2 to
    // ensure that content is not skipped while arrow keys are conflicting
    // with normal focusgroup navigation.
    await assert_focusgroup_tab_navigation([before, search, btn2, after]);
    // Going backwards, memory of last focused item (btn2) is respected,
    // and search is not considered to be in sequential tab ordering.
    await assert_focusgroup_shift_tab_navigation([after, btn2, before]);
  }, "Tab entry respects focusgroupstart on native arrow key handler");

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
  "source_name": "html/interaction/focus/focusgroup/tentative/sequential-navigation/arrow-key-handler-with-priority.html"
}
```
