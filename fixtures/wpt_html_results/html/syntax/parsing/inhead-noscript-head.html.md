# html/syntax/parsing/inhead-noscript-head.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/parsing/inhead-noscript-head.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>Test that when the scripting flag is disabled, a head start tag in "in head noscript" mode is ignored</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<body>
<script>
promise_test(async function(t) {
    let iframe = document.createElement("iframe");
    iframe.srcdoc = "<!DOCTYPE html><head><noscript><head><style></style>";
    iframe.sandbox = "allow-same-origin";
    let loaded = new Promise(resolve => iframe.onload = resolve);
    document.body.append(iframe);
    await loaded;
    assert_equals(String(iframe.contentDocument.querySelector("noscript").firstChild), "[object HTMLStyleElement]");
}, "When the scripting flag is disabled, a head start tag in \"in head noscript\" mode should be ignored");
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
  "source_name": "html/syntax/parsing/inhead-noscript-head.html"
}
```
