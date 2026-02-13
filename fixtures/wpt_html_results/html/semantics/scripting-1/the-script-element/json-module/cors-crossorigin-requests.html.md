# html/semantics/scripting-1/the-script-element/json-module/cors-crossorigin-requests.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/json-module/cors-crossorigin-requests.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
<head>
    <title>json-module-crossorigin</title>
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
</head>
<body>
    <h1>json-module-crossorigin</h1>
    <script>

        var tests = [
                { "obj": async_test("Imported JSON module, cross-origin with CORS"), "id": "import-WithCORS", "expected": "imported JSON: 42", "url": "crossorigin-import-with-cors.sub.html" },
                { "obj": async_test("Imported JSON module, cross-origin, missing CORS ACAO header"), "id": "import-NoCORS", "expected": "error", "url": "crossorigin-import-without-cors.sub.html" },
                { "obj": async_test("Imported JSON module with parse error, cross-origin, with CORS"), "id": "import-parseerror-WithCors", "expected": "SyntaxError", "url": "crossorigin-import-parse-error-with-cors.sub.html" },
            ];

        async function loadTest(test) {
            return new Promise((resolve) => {
                const iframe = document.createElement('iframe');
                iframe.id = test.id;
                iframe.src = test.url;
                iframe.onload = () => resolve(iframe);
                document.body.appendChild(iframe);
            });
        }

        (async function () {
            for (const test of tests) {
                const target = await loadTest(test);
                test.obj.step(function () {
                    assert_equals(target.contentDocument._log, test.expected, "Unexpected _log value");
                });
                test.obj.done();
            }
        })();

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
  "source_name": "html/semantics/scripting-1/the-script-element/json-module/cors-crossorigin-requests.html"
}
```
