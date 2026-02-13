# html/browsers/windows/targeting-cross-origin-nested-browsing-contexts.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/windows/targeting-cross-origin-nested-browsing-contexts.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
<head>
  <meta charset="utf-8">
  <title>Targeting nested browsing contexts</title>
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
</head>
<body>
  <script src="/common/get-host-info.sub.js"></script>
  <script>
    async_test(function (t) {
      var windowsToClose = [];
      window.onmessage = t.step_func(function (e) {
        if (e.data.name == "openee") {
          var a = document.body.appendChild(document.createElement('a'));
          a.target = "nested1";
          a.href = "resources/post-to-opener.html";
          a.click();
          windowsToClose.push(e.source);
        } else {
          assert_equals(e.data.name, "nested1");
          assert_equals(e.data.isTop, true);
          windowsToClose.push(e.source);
          windowsToClose.forEach(function (w) {
            w.close();
          });
          t.done();
        }
      });

      var a = document.body.appendChild(document.createElement('a'));
      a.target = "openee";
      a.href = get_host_info().HTTP_REMOTE_ORIGIN + "/html/browsers/windows/resources/nested-post-to-opener.html";
      a.click();
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
  "source_name": "html/browsers/windows/targeting-cross-origin-nested-browsing-contexts.html"
}
```
