# html/browsers/windows/consume-user-activation/window-open-multi-global.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/windows/consume-user-activation/window-open-multi-global.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>window.open() and consuming user activation with multiple globals in play</title>
<link rel="help" href="https://html.spec.whatwg.org/#window-open-steps">
<link rel="help" href="https://html.spec.whatwg.org/#the-rules-for-choosing-a-navigable">
<!--
2. Let sourceDocument be the entry global object's associated Document.
10. Let targetNavigable and windowType be the result of applying the rules for choosing a navigable
    given target, sourceDocument's node navigable, and noopener.
-->
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>

<!-- This is the entry global -->

<script>
'use strict';

function pageDone(expectedMessage) {
  return new Promise(resolve => {
    window.addEventListener('message', e => {
      if (e.data === expectedMessage) {
        resolve();
      }
    });
  });
}

promise_test(async function(t) {
  await test_driver.bless("open incumbent popup");
  const incumbentDone = pageDone("incumbent page");
  const incumbentWin = window.open("support/incumbent.html", "_blank");
  // incumbent.html opens two further popups and sets these properties (for this window):
  // window.currentWin
  // window.relevantWin
  await incumbentDone;
  await test_driver.bless("user activation in entry global");
  const testWin = incumbentWin.openTestPopup();
  t.add_cleanup(() => {
    testWin.close();
    relevantWin.close();
    currentWin.close();
    incumbentWin.close();
  });
  assert_false(navigator.userActivation.isActive, "User activation of the entry global should be consumed");
});
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
  "source_name": "html/browsers/windows/consume-user-activation/window-open-multi-global.html"
}
```
