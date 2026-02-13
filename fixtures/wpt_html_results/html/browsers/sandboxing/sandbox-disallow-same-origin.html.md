# html/browsers/sandboxing/sandbox-disallow-same-origin.html

Counts:
- errors: 2
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/sandboxing/sandbox-disallow-same-origin.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
  <head>
    <title>Access to sandbox iframe</title>
    <link rel="author" title="Kinuko Yasuda" href="mailto:kinuko@chromium.org">
    <link rel="help" href="https://html.spec.whatwg.org/multipage/#sandboxed-origin-browsing-context-flag">
    <link rel="help" href="https://html.spec.whatwg.org/multipage/#integration-with-idl">
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
  </head>

  <body>
    <h1>Access to sandbox iframe</h1>
    <script type="text/javascript">
      var t = async_test("Access to sandbox iframe is disallowed")
      var called = 0;
      function calledFromIframe() {
        called++;
      }
      function loaded() {
        t.step(() => {
          assert_throws_dom("SecurityError", () => {
            document.getElementById('sandboxedframe').contentWindow.document;
          });
          assert_equals(called, 0);
          t.done();
        });
      }
    </script>

    <iframe src="/html/browsers/sandboxing/inner-iframe.html" style="visibility:hidden;display:none" sandbox id="sandboxedframe" onload="loaded();"></iframe>
  </body>

    <div id="log"></div>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 561,
        "byte_start": 530,
        "col": 5,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_start_tag",
      "message": "Stray start tag “div”.",
      "severity": "Error",
      "span": {
        "byte_end": 1185,
        "byte_start": 1171,
        "col": 5,
        "line": 34
      }
    },
    {
      "category": "Html",
      "code": "html.parser.end_tag_after_body_closed",
      "message": "Saw an end tag after “body” had been closed.",
      "severity": "Error",
      "span": {
        "byte_end": 1191,
        "byte_start": 1185,
        "col": 19,
        "line": 34
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/browsers/sandboxing/sandbox-disallow-same-origin.html"
}
```
