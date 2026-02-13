# html/semantics/scripting-1/the-script-element/json-module/http-accept-header.sub.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/json-module/http-accept-header.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<title>HTTP Accept header with JSON module requests</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
</head>
<body>
<script type="module">
  import json from "./http-accept-header-checker.py" with { type: "json"};
  test(t => {
    assert_equals(
        json.accept, "application/json,*/*;q=0.5",
        "The HTTP Accept header should be set to 'application/json' when statically importing a JSON module.");
  }, "Static import of a JSON module should send a valid HTTP Accept header.");
</script>

<script type="module">
promise_test(async () => {
  const module = await import("./http-accept-header-checker.py", { with: { type: "json"} });
  assert_equals(
      module.default.accept, "application/json,*/*;q=0.5",
      "The HTTP Accept header should be set to 'application/json' when dynamically importing a JSON module.");
  }, "Dynamic import of a JSON module should send a valid HTTP Accept header.");
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
  "source_name": "html/semantics/scripting-1/the-script-element/json-module/http-accept-header.sub.html"
}
```
