# html/browsers/offline/introduction-4/event_obsolete-manual.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/offline/introduction-4/event_obsolete-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<html manifest="../resources/manifest/clock.manifest">
  <head>
    <title>Offline Application Cache - Event_obsolete</title>
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
  </head>
  <body>
    <ol>
      <li>Remove the manifest file (manifest/clock.manifest) from the server.</li>
      <li>Refresh the page.</li>
    </ol>

    <div id="log"></div>

    <script>
      var t = async_test("obsolete event test");
      var cache = window.applicationCache;

      cache.onobsolete = t.done();
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
  "source_name": "html/browsers/offline/introduction-4/event_obsolete-manual.html"
}
```
