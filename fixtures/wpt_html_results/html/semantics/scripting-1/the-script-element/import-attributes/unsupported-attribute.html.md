# html/semantics/scripting-1/the-script-element/import-attributes/unsupported-attribute.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/import-attributes/unsupported-attribute.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Handling of unsupported attribute</title>

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
  setup({allow_uncaught_exception: true});

  window.log = [];

  window.addEventListener("error", ev => log.push(ev.error));

  const test_load = async_test(
      "Test that invalid module attribute leads to SyntaxError on window.");
  window.addEventListener("load", test_load.step_func_done(ev => {
    assert_equals(log.length, 1);
    assert_equals(log[0].constructor, SyntaxError);
  }));

  function unreachable() { log.push("unexpected"); }
</script>
<script type="module" src="./unsupported-attribute.js" onerror="unreachable()"></script>
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
  "source_name": "html/semantics/scripting-1/the-script-element/import-attributes/unsupported-attribute.html"
}
```
