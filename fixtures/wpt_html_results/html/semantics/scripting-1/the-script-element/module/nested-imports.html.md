# html/semantics/scripting-1/the-script-element/module/nested-imports.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/module/nested-imports.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Test imports under more than 3 levels in different modules</title>

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
  setup({ allow_uncaught_exception: true });

  window.log = [];

  const test_load = async_test("should load all modules successfully");
  window.addEventListener(
    "load",
    test_load.step_func_done((ev) => {
      assert_equals(log.length, 2);

      assert_equals(log[0], 1);
      assert_equals(log[1], 2);
    })
  );

  function unreachable() {
    log.push("unexpected");
  }
</script>
<script type="module" src="./nested-imports-a.js" onerror="unreachable()"
    onload="log.push(1)"></script>
<script type="module" src="./nested-imports-e.js" onerror="unreachable()"
  onload="log.push(2)"></script>
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
  "source_name": "html/semantics/scripting-1/the-script-element/module/nested-imports.html"
}
```
