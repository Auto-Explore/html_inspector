# html/webappapis/structured-clone/structured-clone-cross-realm-method.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/structured-clone/structured-clone-cross-realm-method.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>self.structuredClone() uses this's relevant Realm for deserialization</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/structured-data.html#structured-cloning">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<body>
<script>
const iframe = document.createElement("iframe");
iframe.onload = () => {
  const otherWindow = iframe.contentWindow;
  for (const key of ["Object", "Array", "Date", "RegExp"]) {
    test(() => {
      const cloned = otherWindow.structuredClone.call(window, new otherWindow[key]);
      assert_true(cloned instanceof window[key]);
    }, `${key} instance`);
  }
};
document.body.append(iframe);
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
  "source_name": "html/webappapis/structured-clone/structured-clone-cross-realm-method.html"
}
```
