# html/semantics/document-metadata/the-style-element/style-load-after-mutate.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-style-element/style-load-after-mutate.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>The 'load' event on the style element should still fire after mutation</title>
<link rel="help" href="https://crbug.com/1323319">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
promise_test(async () => {
  const style = document.createElement('style');
  document.head.appendChild(style);
  style.appendChild(document.createTextNode('@import url(/support/css-red.txt);'));
  style.appendChild(document.createTextNode('body {color: green; }'));

  // The 'load' event should fire.
  await new Promise(resolve => style.onload = resolve);
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
  "source_name": "html/semantics/document-metadata/the-style-element/style-load-after-mutate.html"
}
```
