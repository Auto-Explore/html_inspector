# html/browsers/offline/introduction-4/event_downloading-manual.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/offline/introduction-4/event_downloading-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<html manifest="../resources/manifest/clock.manifest">
  <head>
    <title>Offline Application Cache - Event_downloading</title>
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
  </head>
  <body>
    <ol>
      <li>Modify the commented part of the manifest file (manifest/clock.manifest) on the server.</li>
      <li>Refresh the page.</li>
    </ol>

    <div id="log"></div>

    <script>
      var t = async_test("downloading event test");
      var cache = window.applicationCache;

      cache.ondownloading = t.done();
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
  "source_name": "html/browsers/offline/introduction-4/event_downloading-manual.html"
}
```
