# html/semantics/scripting-1/the-script-element/module/evaluation-error-5.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/module/evaluation-error-5.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Handling of evaluation errors, 5</title>

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
    setup({allow_uncaught_exception: true});
    const test_load = async_test("Test for the inline module script error that reports wrong line\n");
    window.addEventListener("error", test_load.step_func_done(ev => {

        const expectedLine = 16;
        assert_equals(ev.lineno, expectedLine);

    }));
</script>
<script type="module">a</script>
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
  "source_name": "html/semantics/scripting-1/the-script-element/module/evaluation-error-5.html"
}
```
