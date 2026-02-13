# html/browsers/history/the-location-interface/replace-with-nested-iframe.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-location-interface/replace-with-nested-iframe.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<html>
  <head>
    <title>Referer with location.replace and nested frames</title>
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
  </head>
  <body>
    <iframe src="resources/replace-or-assign-call-on-iframe.html?replace" hidden></iframe>
    <script>
      async_test(function(t) {
        function on_message(e) {
        const nestedIframeReferrer = e.data;
        assert_equals(nestedIframeReferrer, document.querySelector("iframe").contentWindow.location.href);
        t.done();
      }
      window.addEventListener('message', t.step_func(on_message), false);
      }, "Browser sends Referer header when location.replace is called in iframe document on another nested iframe element");
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
  "source_name": "html/browsers/history/the-location-interface/replace-with-nested-iframe.html"
}
```
