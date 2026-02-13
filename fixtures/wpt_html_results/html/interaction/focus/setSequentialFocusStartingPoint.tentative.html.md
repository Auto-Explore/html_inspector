# html/interaction/focus/setSequentialFocusStartingPoint.tentative.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/interaction/focus/setSequentialFocusStartingPoint.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://github.com/whatwg/html/issues/5326">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>

<button id=b1>b1</button>
<div id=d1>d1</div>
<button id=b2>b2</button>

<script>
const tabKey = '\uE004';
promise_test(async () => {
  assert_equals(document.activeElement, document.body,
    'Focus should initially be set on the body element.');

  document.setSequentialFocusStartingPoint(d1);
  assert_equals(document.activeElement, document.body,
    'Calling setSequentialFocusStartingPoint should not change the focused element.');

  await test_driver.send_keys(document.activeElement, tabKey);
  assert_equals(document.activeElement, b2,
    'Pressing tab should focus the next button after the sequential focus starting point.');
}, 'document.setSequentialFocusStartingPoint should set the sequential focus starting point on any element.');
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/interaction/focus/setSequentialFocusStartingPoint.tentative.html"
}
```
