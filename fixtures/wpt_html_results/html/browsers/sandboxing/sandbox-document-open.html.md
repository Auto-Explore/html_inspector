# html/browsers/sandboxing/sandbox-document-open.html

Counts:
- errors: 1
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/sandboxing/sandbox-document-open.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>
  Check sandbox-flags aren't lost after using document.open().
</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body>
<script>
promise_test(async test => {
  let message = new Promise(resolve =>
    window.addEventListener("message", event => resolve(event.data))
  );

  let iframe = document.createElement("iframe");
  iframe.setAttribute("sandbox", "allow-scripts allow-same-origin");
  iframe.setAttribute("src", "./resources/document-open.html")
  document.body.appendChild(iframe);

  assert_equals(await message, "document-domain-is-disallowed");
}, "document.open()");

promise_test(async test => {
  let iframe = document.createElement("iframe");
  iframe.setAttribute("sandbox", "allow-scripts allow-same-origin");
  iframe.setAttribute("src", "/common/blank.html");
  let loaded = new Promise(resolve => iframe.onload = resolve);
  document.body.appendChild(iframe);
  await loaded;

  let message = new Promise(resolve =>
    window.addEventListener("message", event => resolve(event.data))
  );

  iframe.contentDocument.write(`
    <script>
      try {
        document.domain = document.domain;
        parent.postMessage('document-domain-is-allowed', '*');
      } catch (error) {
        parent.postMessage('document-domain-is-disallowed', '*');
      }
    </sc`+`ript>
  `);

  assert_equals(await message, "document-domain-is-disallowed");
}, "other_document.open()");
</script>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “html”.",
      "severity": "Error",
      "span": {
        "byte_end": 1526,
        "byte_start": 1519,
        "col": 1,
        "line": 50
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/browsers/sandboxing/sandbox-document-open.html"
}
```
