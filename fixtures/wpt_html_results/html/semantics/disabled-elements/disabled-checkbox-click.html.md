# html/semantics/disabled-elements/disabled-checkbox-click.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/disabled-elements/disabled-checkbox-click.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<link rel="author" title="Aditya Keerthi" href="https://github.com/pxlcoder">
<link rel="help" href="https://html.spec.whatwg.org/multipage/form-control-infrastructure.html#enabling-and-disabling-form-controls:-the-disabled-attribute">
<title>Test disabled checkbox does not change state when clicked</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>

<input type="checkbox" disabled>
<script>
const input = document.querySelector("input");

promise_test(async function() {
    assert_false(input.checked);

    await new test_driver.Actions()
        .pointerMove(0, 0, { origin: input })
        .pointerDown()
        .pointerUp()
        .send();

    assert_false(input.checked);
}, `Disabled checkbox does not change state when clicked`);
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
  "source_name": "html/semantics/disabled-elements/disabled-checkbox-click.html"
}
```
