# html/interaction/focus/focusgroup/tentative/descendant-navigation/simple-descendant-test.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/interaction/focus/focusgroup/tentative/descendant-navigation/simple-descendant-test.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTML Test: focusgroup - Simple descendant navigation test</title>
<link rel="author" title="Microsoft" href="http://www.microsoft.com/">
<link rel="help" href="https://open-ui.org/components/scoped-focusgroup.explainer/">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="../resources/focusgroup-utils.js"></script>

<div id=root focusgroup="toolbar">
  <button id=item1 tabindex=0>Item 1</button>
  <div>
    <button id=item2 tabindex=0>Item 2 (nested)</button>
  </div>
  <button id=item3 tabindex=0>Item 3</button>
</div>

<script>

  promise_test(async t => {
    var item1 = document.getElementById("item1");
    var item2 = document.getElementById("item2");
    var item3 = document.getElementById("item3");

    await focusAndKeyPress(item1, kArrowRight);
    assert_equals(document.activeElement, item2, "Should navigate to nested item2");

    await focusAndKeyPress(item2, kArrowRight);
    assert_equals(document.activeElement, item3, "Should navigate to item3");
  }, "Simple descendant navigation should work");

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
  "source_name": "html/interaction/focus/focusgroup/tentative/descendant-navigation/simple-descendant-test.html"
}
```
