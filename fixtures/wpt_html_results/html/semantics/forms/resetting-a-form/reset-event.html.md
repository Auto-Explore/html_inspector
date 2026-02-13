# html/semantics/forms/resetting-a-form/reset-event.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/resetting-a-form/reset-event.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Test aspects of the reset event</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
  async_test((t) => {
    var form = document.createElement("form")
    form.onreset = t.step_func_done((e) => {
      assert_true(e.bubbles)
      assert_true(e.cancelable)
      assert_true(e.isTrusted)
      assert_equals(e.target, form)
    })
    form.reset()
    assert_unreached()
  })
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
  "source_name": "html/semantics/forms/resetting-a-form/reset-event.html"
}
```
