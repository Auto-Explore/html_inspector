# html/semantics/scripting-1/the-script-element/execution-timing/non-external-no-import.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/execution-timing/non-external-no-import.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
  <title>Module scripts with no imports always execute asynchronously</title>
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
  <link rel="help" href="https://github.com/whatwg/html/issues/3746">
</head>
<body>
<script>
async_test(t => {
  window.results = [];
  window.logExecution = msg => window.results.push(msg);

  const script = document.createElement('script');
  script.type = 'module';
  script.textContent = "window.logExecution('module')";
  document.body.append(script);
  window.logExecution('classic');

  window.onload = t.step_func_done(e => {
    assert_array_equals(window.results, ['classic', 'module']);
  });
});
</script>
</body>
</html>
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
  "source_name": "html/semantics/scripting-1/the-script-element/execution-timing/non-external-no-import.html"
}
```
