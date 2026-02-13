# html/semantics/forms/the-fieldset-element/disabled-002.xhtml

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-fieldset-element/disabled-002.xhtml",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE html>
<html xmlns="http://www.w3.org/1999/xhtml">
<head>
  <title>File input descendants of disabled fieldsets</title>
  <link rel="author" title="Chris Rebert" href="http://chrisrebert.com" />
  <link rel="help" href="https://html.spec.whatwg.org/multipage/#attr-fieldset-disabled" />
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
</head>
<body>
  <div id="log"></div>
  <form>
    <fieldset id="fs" disabled="disabled">
      <input id="myfile" type="file" />
    </fieldset>
  </form>
  <script>
    test(function () {
      assert_true(document.getElementById('fs').disabled, "disabled fieldset should be disabled");
      assert_false(document.getElementById('myfile').willValidate, "form control descendant of disabled fieldset that is not also a descendant of a legend should be disabled");
    }, "A file input without a disabled attribute that is a descendant of a disabled fieldset should be disabled (modulo legend-related complications that don't apply here)");
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
  "source_name": "html/semantics/forms/the-fieldset-element/disabled-002.xhtml"
}
```
