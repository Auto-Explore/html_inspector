# html/semantics/embedded-content/the-iframe-element/sandbox_023.htm

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/sandbox_023.htm",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<html>
<head>
    <title>HTML5 Sandbox: Allow sandbox iframe to access other content from the same origin if sandbox="allow-same-origin".</title>
    <meta content="text/html; charset=UTF-8" http-equiv="Content-Type" />
    <link rel="author" title="Microsoft" href="http://www.microsoft.com/" />
    <link rel="help" href="http://dev.w3.org/html5/spec/Overview.html#sandboxed-origin-browsing-context-flag" />
    <meta name="assert" content=" Allow sandbox iframe to access other content from the same origin if sandbox='allow-same-origin'." />
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
</head>
<body>
    <script type="text/javascript">

        var t = async_test("Allow sandbox iframe to access other content from the same origin if sandbox='allow-same-origin'");

        function callback(event)
        {
            t.step(function(){
                assert_true('sandbox' in document.createElement('iframe'));
                assert_equals(event.data, "window.parent.document");
            });
            t.done();
        }

        var timer = setTimeout(callback, 4000);
        window.addEventListener("message", callback, false);
    </script>
    <iframe src="support/iframe_sandbox_023.htm" sandbox="allow-scripts allow-same-origin" style="display:none"></iframe>
    <div id=log></div>

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
        "byte_end": 726,
        "byte_start": 695,
        "col": 5,
        "line": 13
      }
    },
    {
      "category": "Html",
      "code": "html.iframe.sandbox.scripts_and_same_origin",
      "message": "Bad value “allow-scripts allow-same-origin” for attribute “sandbox” on element “iframe”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1360,
        "byte_start": 1252,
        "col": 5,
        "line": 29
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/sandbox_023.htm"
}
```
