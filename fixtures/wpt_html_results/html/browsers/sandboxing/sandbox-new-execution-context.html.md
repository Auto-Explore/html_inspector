# html/browsers/sandboxing/sandbox-new-execution-context.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/sandboxing/sandbox-new-execution-context.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <title>Reuse of iframe about:blank document execution context</title>
    <link rel="author" title="Dan Clark" href="mailto:daniec@microsoft.com">
    <link rel="help" href="http://www.w3.org/html/wg/drafts/html/master/browsers.html#sandboxing">
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
  </head>

  <body>
    <h1>Reuse of iframe about:blank document execution context in sandbox="allow-scripts" iframe</h1>
    <script type="text/javascript">
      async_test(t => {
        let iframe = document.createElement("iframe");
        document.body.appendChild(iframe);

        let iframeAboutBlankDocument = iframe.contentDocument;
        assert_equals(iframeAboutBlankDocument.URL, "about:blank");

        iframe.sandbox = "allow-scripts";
        iframe.src = './sandbox-new-execution-context-iframe.html';

        iframe.onload = t.step_func_done(() => {
          assert_equals(iframe.contentDocument, null,
            "New document in sandboxed iframe should have opaque origin");

          assert_equals(Object.getPrototypeOf(iframeAboutBlankDocument).changeFromSandboxedIframe, undefined,
            "Sandboxed iframe contents should not have been able to mess with type system of about:blank document");

          let iframeAboutBlankContents = iframeAboutBlankDocument.querySelectorAll('body');
          assert_equals(iframeAboutBlankContents[0].tagName, "BODY",
            "about:blank document's contents should still be accessible");
        });
      },"iframe with sandbox should load with new execution context");
    </script>
    <div id="log"></div>
  </body>
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
        "byte_end": 553,
        "byte_start": 522,
        "col": 5,
        "line": 13
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
  "source_name": "html/browsers/sandboxing/sandbox-new-execution-context.html"
}
```
