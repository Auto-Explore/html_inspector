# html/webappapis/scripting/events/event-handler-onmove-02.tentative.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/scripting/events/event-handler-onmove-02.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Window Object 'onmove'</title>
<link rel="author" title="Javier Fernandez" href="mailto:jfernandez@igalia.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#handler-window-onmove">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>

<script>
  promise_test(async t => {
    const expectedRect = {'x': 100, 'y': 200};

    const moved = new Promise(resolve => window.addEventListener("move", resolve));
    await test_driver.set_window_rect(expectedRect);
    await moved;

    const rect = await test_driver.get_window_rect();

    assert_equals(rect.x, expectedRect.x, "The window rect X is correct.");
    assert_equals(rect.y, expectedRect.y, "The window rect Y is correct.");
  }, "Window move event");
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
  "source_name": "html/webappapis/scripting/events/event-handler-onmove-02.tentative.html"
}
```
