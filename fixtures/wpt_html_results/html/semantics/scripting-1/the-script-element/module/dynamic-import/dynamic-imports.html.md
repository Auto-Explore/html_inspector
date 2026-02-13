# html/semantics/scripting-1/the-script-element/module/dynamic-import/dynamic-imports.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/module/dynamic-import/dynamic-imports.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Basic dynamic imports</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script type="module">
promise_test(t => {
  return import("./../imports-a.js").then(module => {
    assert_true(window.evaluated_imports_a);
    assert_equals(module.A["from"], "imports-a.js");
  });
}, "Dynamic imports should resolve module.");
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
  "source_name": "html/semantics/scripting-1/the-script-element/module/dynamic-import/dynamic-imports.html"
}
```
