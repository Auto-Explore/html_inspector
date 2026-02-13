# html/browsers/windows/restore-window-name-manual.https.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/windows/restore-window-name-manual.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
<head>
  <title>Restore window.name when navigating back from a cross-origin</title>
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
  <script src="/common/utils.js"></script>
</head>
<body>
  <script>
    function runTest() {
      const name = token();
      window.open(`resources/restore-window-name.sub.html?name=${name}`, "start");
    }

  </script>
  <p>Please click the following link and follow the instructions in the page for testing</p>
  <a target="start" href="javascript:void(0)" onclick="runTest()">run test</a>
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
  "source_name": "html/browsers/windows/restore-window-name-manual.https.html"
}
```
