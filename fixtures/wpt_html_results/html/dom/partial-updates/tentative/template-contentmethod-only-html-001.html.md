# html/dom/partial-updates/tentative/template-contentmethod-only-html-001.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/partial-updates/tentative/template-contentmethod-only-html-001.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<meta charset="utf-8" />
<title>HTML partial updates: template with contentmethod does not work in XHTML</title>
<link rel=help href="https://github.com/WICG/declarative-partial-updates">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body>
<script>
promise_test(async () => {
    const iframe = document.createElement("iframe");
    iframe.src = "resources/contentmethod.xhtml";
    document.body.append(iframe);
    await new Promise(resolve => iframe.addEventListener("load", resolve));
    const {contentDocument} = iframe;
    assert_not_equals(contentDocument.querySelector("#patch"), null);
    assert_equals(contentDocument.querySelector("#placeholder").textContent, "Unchanged");
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
  "source_name": "html/dom/partial-updates/tentative/template-contentmethod-only-html-001.html"
}
```
