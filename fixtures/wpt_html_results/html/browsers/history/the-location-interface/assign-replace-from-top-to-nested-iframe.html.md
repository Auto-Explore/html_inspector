# html/browsers/history/the-location-interface/assign-replace-from-top-to-nested-iframe.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-location-interface/assign-replace-from-top-to-nested-iframe.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<html>
  <head>
    <title>Referer with location.replace and location.assign with nested iframes</title>
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
  </head>
  <body>
    <iframe src="resources/iframe-with-iframe.html" hidden></iframe>
    <script>
      const iframe = document.querySelector("iframe");
      async_test(function(t) {
        function on_message(e) {
          const referrer = e.data;
          assert_equals(referrer, iframe.contentWindow.location.href);
          t.done();
        }
        window.addEventListener('message', t.step_func(on_message), false);
        window.addEventListener('load', function () {
            iframe.contentDocument.querySelector("iframe").contentWindow.location.replace("/resources/blank.html");
        }, false);
      }, "Browser sends Referer header in nested iframe request when location.replace is called on an iframe");
      async_test(function(t) {
        function on_message(e) {
          const referrer = e.data;
          assert_equals(referrer, iframe.contentWindow.location.href);
          t.done();
        }
        window.addEventListener('message', t.step_func(on_message), false);
        window.addEventListener('load', function () {
            iframe.contentDocument.querySelector("iframe").contentWindow.location.replace("/resources/blank.html");
        }, false);
      }, "Browser sends Referer header in nested iframe request when location.assign is called on an iframe");
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
  "source_name": "html/browsers/history/the-location-interface/assign-replace-from-top-to-nested-iframe.html"
}
```
