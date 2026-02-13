# html/browsers/history/the-location-interface/assign-replace-from-iframe.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-location-interface/assign-replace-from-iframe.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<html>
  <head>
    <title>Referer with location.replace and location.assign</title>
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
  </head>
  <body>
    <iframe src="/resources/blank.html" hidden></iframe>
    <script>
      async_test(function(t) {
        function on_message(e) {
        const referrer = e.data;
        assert_equals(referrer, window.location.href);
        t.done();
      }
      window.addEventListener('message', t.step_func(on_message), false);
      document.querySelector("iframe").contentWindow.location.replace("resources/iframe-contents.sub.html?replace");
      }, "Browser sends Referer header in iframe request when location.replace is called from an iframe");
      async_test(function(t) {
        function on_message(e) {
        const referrer = e.data;
        assert_equals(referrer, window.location.href);
        t.done();
      }
      window.addEventListener('message', t.step_func(on_message), false);
      document.querySelector("iframe").contentWindow.location.assign("resources/iframe-contents.sub.html?assign");
      }, "Browser sends Referer header in iframe request when location.assign is called from an iframe");
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
  "source_name": "html/browsers/history/the-location-interface/assign-replace-from-iframe.html"
}
```
