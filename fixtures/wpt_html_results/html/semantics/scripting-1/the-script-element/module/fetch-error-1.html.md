# html/semantics/scripting-1/the-script-element/module/fetch-error-1.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/module/fetch-error-1.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Handling of fetch errors, 1</title>

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
    window.log = [];

    const test_load = async_test(
        "Test that failure to fetch root leads to error event on script.");
    window.addEventListener("load", test_load.step_func_done(ev => {
      assert_array_equals(log, ["script"]);
    }));
</script>
<script type="module" src="./no-such-file.js" onerror="log.push('script')"></script>
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
  "source_name": "html/semantics/scripting-1/the-script-element/module/fetch-error-1.html"
}
```
