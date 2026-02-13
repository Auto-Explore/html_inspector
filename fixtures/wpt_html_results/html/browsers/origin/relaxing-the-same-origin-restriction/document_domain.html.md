# html/browsers/origin/relaxing-the-same-origin-restriction/document_domain.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/origin/relaxing-the-same-origin-restriction/document_domain.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
    <head>
        <title>document.domain's getter</title>
        <script src="/resources/testharness.js"></script>
        <script src="/resources/testharnessreport.js"></script>
        <script>
            test(function() {
                assert_equals(typeof document.domain, "string", "document.domain is a string");
                assert_not_equals(document.domain, "", "document.domain is not empty");
            }, "basics");

            test(function() {
                assert_equals(document.domain, window.location.hostname, "equals location.hostname");
            }, "current document");

            test(function() {
                var doc = new Document();
                assert_equals(doc.domain, window.location.hostname, "equals location.hostname");
            }, "new Document()");

            async_test(t => {
                const client = new XMLHttpRequest();
                client.open("GET", "/common/blank.html");
                client.responseType = "document"
                client.send();
                client.onload = t.step_func_done(() => {
                    assert_equals(client.response.domain, window.location.hostname);
                });
            }, "XMLHttpRequest's response document");
        </script>
    </head>
    <body>
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
  "source_name": "html/browsers/origin/relaxing-the-same-origin-restriction/document_domain.html"
}
```
