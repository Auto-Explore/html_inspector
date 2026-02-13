# html/rendering/the-details-element/details-focus-steps.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/the-details-element/details-focus-steps.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<meta charset="utf-8" />
<title>details content, when opened, should be included in the focus flow</title>
<link rel="author" title="Keith Cirkel" href="mailto:wpt@keithcirkel.co.uk">
<link rel="help" href="https://html.spec.whatwg.org/multipage/rendering.html#the-details-and-summary-elements">
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1981724">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="/resources/testdriver-actions.js"></script>

<details id=details open>
  <summary id=first>First</summary>
  <p>Then <a id=second href="#">Second</a></p>
</details>
Finally <button id=third>Third</button>

<script>
  const tabKey = "\ue004";
  const shiftKey = "\uE008";

  promise_test(async (t) => {
    details.open = true;
    first.focus();
    await test_driver.send_keys(first, tabKey);
    assert_equals(document.activeElement, second);
    await test_driver.send_keys(second, tabKey);
    assert_equals(document.activeElement, third);
    await new test_driver.Actions().keyDown(shiftKey).keyDown(tabKey).keyUp(tabKey).keyUp(shiftKey).send();
    assert_equals(document.activeElement, second);
    await new test_driver.Actions().keyDown(shiftKey).keyDown(tabKey).keyUp(tabKey).keyUp(shiftKey).send();
    assert_equals(document.activeElement, first);
  }, "Focus flows into second when details is open");

  promise_test(async (t) => {
    details.open = false;
    first.focus();
    await test_driver.send_keys(first, tabKey);
    assert_equals(document.activeElement, third);
    await new test_driver.Actions().keyDown(shiftKey).keyDown(tabKey).keyUp(tabKey).keyUp(shiftKey).send();
    assert_equals(document.activeElement, first);
  }, "Focus flows into third when details is closed");
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
  "source_name": "html/rendering/the-details-element/details-focus-steps.html"
}
```
