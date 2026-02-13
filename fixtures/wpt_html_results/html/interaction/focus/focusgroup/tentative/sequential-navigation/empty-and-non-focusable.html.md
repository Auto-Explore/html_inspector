# html/interaction/focus/focusgroup/tentative/sequential-navigation/empty-and-non-focusable.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/interaction/focus/focusgroup/tentative/sequential-navigation/empty-and-non-focusable.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTML Test: focusgroup - Empty and non-focusable focusgroups</title>
<link rel="author" title="Microsoft" href="http://www.microsoft.com/">
<link rel="help" href="https://open-ui.org/components/scoped-focusgroup.explainer/">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/shadow-dom/focus-navigation/resources/focus-utils.js"></script>
<script src="../resources/focusgroup-utils.js"></script>

<!-- Empty focusgroup -->
<div id=before1 tabindex=0>Before empty</div>
<div id=empty focusgroup="toolbar"></div>
<div id=after1 tabindex=0>After empty</div>

<!-- Focusgroup with only non-focusable elements -->
<div id=before2 tabindex=0>Before non-focusable</div>
<div id=nonfocus focusgroup="toolbar">
  <span id=span1>Non-focusable 1</span>
  <span id=span2>Non-focusable 2</span>
</div>
<div id=after2 tabindex=0>After non-focusable</div>

<!-- Focusgroup with only disabled elements -->
<div id=before3 tabindex=0>Before disabled</div>
<div id=disabled focusgroup="toolbar">
  <button id=btn-disabled disabled>Disabled</button>
</div>
<div id=after3 tabindex=0>After disabled</div>

<!-- Empty focusgroup container but container itself is focusable -->
<div id=before4 tabindex=0>Before focusable container</div>
<div id=container tabindex=0 focusgroup="toolbar"></div>
<div id=after4 tabindex=0>After focusable container</div>

<script>
  promise_test(async t => {
    document.getElementById("before1").focus();
    await navigateFocusForward();
    assert_equals(document.activeElement, document.getElementById("after1"),
                  "Tab skips empty focusgroup");
  }, "Empty focusgroup is skipped during Tab navigation");

  promise_test(async t => {
    document.getElementById("before2").focus();
    await navigateFocusForward();
    assert_equals(document.activeElement, document.getElementById("after2"),
                  "Tab skips focusgroup with only non-focusable elements");
  }, "Focusgroup with only non-focusable elements is skipped");

  promise_test(async t => {
    document.getElementById("before3").focus();
    await navigateFocusForward();
    assert_equals(document.activeElement, document.getElementById("after3"),
                  "Tab skips focusgroup with disabled elements");
  }, "Focusgroup with only disabled elements is skipped");

  promise_test(async t => {
    document.getElementById("before4").focus();
    await navigateFocusForward();
    assert_equals(document.activeElement, document.getElementById("container"),
                  "Tab focuses the focusgroup container itself when empty but focusable");
  }, "Focusable empty focusgroup container receives focus");
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
  "source_name": "html/interaction/focus/focusgroup/tentative/sequential-navigation/empty-and-non-focusable.html"
}
```
