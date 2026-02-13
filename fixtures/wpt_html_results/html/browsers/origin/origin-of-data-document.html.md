# html/browsers/origin/origin-of-data-document.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/origin/origin-of-data-document.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <meta charset=utf-8>
    <title>Origin of document produced from a 'data:' URL</title>
    <link rel="help" href="https://html.spec.whatwg.org/multipage/browsers.html#origin">
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
  </head>
  <body>
    <script>
      async_test(function (t) {
        var i = document.createElement('iframe');
        i.src = "data:text/html,<script>" +
                "  window.parent.postMessage('Hello!', '*');" +
                "</scr" + "ipt>";

        window.addEventListener("message", t.step_func_done(function (e) {
          assert_equals(e.origin, "null", "Messages sent from a 'data:' URL should have an opaque origin (which serializes to 'null').");
          assert_throws_dom("SecurityError", function () {
            var couldAccessCrossOriginProperty = e.source.location.href;
          }, "The 'data:' frame should be cross-origin: 'window.location.href'");
          assert_equals(i.contentDocument, null, "The 'data:' iframe should be unable to access its contentDocument.");
        }));

        document.body.appendChild(i);
      }, "The origin of a 'data:' document in a frame is opaque.");
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
  "source_name": "html/browsers/origin/origin-of-data-document.html"
}
```
