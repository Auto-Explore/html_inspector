# html/dom/partial-updates/tentative/stream-append-with-template-contentmethod.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/partial-updates/tentative/stream-append-with-template-contentmethod.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<meta charset="utf-8" />
<title>streamHTML methods can apply template with contentmethod</title>
<link rel=help href="https://github.com/WICG/declarative-partial-updates">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body>
<div contentname=p id="placeholder">Old content</div>
<script>
promise_test(async t => {
    const writable = document.body.streamAppendHTMLUnsafe({runScripts: true});
    const writer = writable.getWriter();
    await writer.write('<template contentmethod="replace-children">');
    await writer.write("<div contentname=p>");
    await writer.write("New content");
    await writer.write("</div>");
    await writer.close();
    assert_equals(document.querySelector("#placeholder").textContent, "New content")
});
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
  "source_name": "html/dom/partial-updates/tentative/stream-append-with-template-contentmethod.html"
}
```
