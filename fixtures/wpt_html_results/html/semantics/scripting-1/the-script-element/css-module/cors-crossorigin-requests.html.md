# html/semantics/scripting-1/the-script-element/css-module/cors-crossorigin-requests.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/css-module/cors-crossorigin-requests.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
<head>
    <title>css-module-crossorigin</title>
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
</head>
<body>
    <h1>css-module-crossorigin</h1>
    <iframe id="import-WithCORS" src="resources/crossorigin-import-with-cors.sub.html"></iframe>
    <iframe id="import-NoCORS" src="resources/crossorigin-import-without-cors.sub.html"></iframe>
    <iframe id="import-parseerror-WithCors" src="resources/crossorigin-import-parse-error-with-cors.sub.html"></iframe>
    <script>

        var tests = [
                { "obj": async_test("Imported CSS module, cross-origin with CORS"), "id": "import-WithCORS", "expected": "imported CSS: #test { background-color: rgb(255, 0, 0); }" },
                { "obj": async_test("Imported CSS module, cross-origin, missing CORS ACAO header"), "id": "import-NoCORS", "expected": "error" },
                { "obj": async_test("Imported CSS module with parse error, cross-origin, with CORS"), "id": "import-parseerror-WithCors", "expected": "imported CSS rules count: 0" },
            ];

        window.addEventListener("load", function () {
            tests.forEach(function (test) {
                var target = document.getElementById(test.id);
                test.obj.step(function () {
                    assert_equals(target.contentDocument._log, test.expected, "Unexpected _log value");
                });
                test.obj.done();
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
  "source_name": "html/semantics/scripting-1/the-script-element/css-module/cors-crossorigin-requests.html"
}
```
