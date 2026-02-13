# html/semantics/scripting-1/the-script-element/import-attributes/empty-attributes-clause.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/import-attributes/empty-attributes-clause.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Handling of empty import attributes clause</title>

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
    window.log = [];

    window.addEventListener("error", ev => log.push(ev.error));

    const test_load = async_test(
        "Test that no error occurs when an empty import attributes clause is provided.");
    window.addEventListener("load", test_load.step_func_done(ev => {
      assert_array_equals(window.log, ["hello", "empty-attributes-clause"]);
    }));

    function unreachable() { log.push("unexpected"); }
</script>
<script type="module" src="./empty-attributes-clause.js" onerror="unreachable()"></script>
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
  "source_name": "html/semantics/scripting-1/the-script-element/import-attributes/empty-attributes-clause.html"
}
```
