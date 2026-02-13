# html/browsers/windows/consume-user-activation/window-open.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/windows/consume-user-activation/window-open.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>window.open() and consuming user activation</title>
<link rel="help" href="https://html.spec.whatwg.org/#the-rules-for-choosing-a-navigable">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script>
'use strict';

promise_test(async function(t) {
  await test_driver.bless("user activation");
  const testWin = window.open("/resources/blank.html", "_blank");
  t.add_cleanup(() => {
    testWin.close();
  });
  assert_false(navigator.userActivation.isActive, "User activation should be consumed");
}, "Opening a new window should consume user activation");

promise_test(async function(t) {
  await test_driver.bless("user activation");
  const testWin = window.open("/resources/blank.html", "testWindow");
  assert_false(navigator.userActivation.isActive, "User activation should be consumed");
  t.add_cleanup(() => {
    testWin.close();
  });

  // Open the existing window again
  await test_driver.bless("user activation");
  const testWin2 = window.open("/resources/blank.html", "testWindow");
  assert_true(navigator.userActivation.isActive, "User activation should not be consumed");
}, "Opening an existing window should not consume user activation");
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
  "source_name": "html/browsers/windows/consume-user-activation/window-open.html"
}
```
