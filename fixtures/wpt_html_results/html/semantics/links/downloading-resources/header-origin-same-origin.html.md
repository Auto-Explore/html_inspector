# html/semantics/links/downloading-resources/header-origin-same-origin.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/links/downloading-resources/header-origin-same-origin.html",
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
    <title>Ping attribute Origin Header Same Origin Policy</title>
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
    <meta name='referrer' content='same-origin'>
  </head>
  <body>
    <a id="a" href="#">
    <script src="/common/utils.js"></script>
    <script src="/common/get-host-info.sub.js"></script>
    <script src="/resources/chromium/enable-hyperlink-auditing.js"></script>
    <script src="header-origin.js"></script>
    <script>
      testOriginHeader(self.location.origin);
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
  "source_name": "html/semantics/links/downloading-resources/header-origin-same-origin.html"
}
```
