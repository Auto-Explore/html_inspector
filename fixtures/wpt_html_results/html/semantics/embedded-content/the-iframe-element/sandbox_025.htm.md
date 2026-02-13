# html/semantics/embedded-content/the-iframe-element/sandbox_025.htm

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/sandbox_025.htm",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
    <title>HTML5 Sandbox: Allow parent content to access sandbox child iframe content when sandbox='allow-same-origin</title>
    <meta content="text/html; charset=UTF-8" http-equiv="Content-Type" />
    <link rel="author" title="Microsoft" href="http://www.microsoft.com/" />
    <link rel="help" href="http://dev.w3.org/html5/spec/Overview.html#sandboxed-origin-browsing-context-flag" />
    <meta name="assert" content="Allow parent content to access sandbox child iframe content when sandbox='allow-same-origin'" />
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
</head>
<body>
    <script type="text/javascript">

     var t = async_test("Allow parent content to access sandbox child iframe content when sandbox='allow-same-origin'");

    function callback(event)
    {
        t.step(function(){
            assert_true('sandbox' in document.createElement('iframe'));
            assert_equals(document.getElementById('sandboxIframe').contentDocument.title, "Page with a message");
        });
        t.done();
    }
    </script>
    <div id=log></div>

    <iframe id='sandboxIframe' src="support/standalone-iframe-content.htm" sandbox="allow-same-origin" onload="callback()" style="display : none"></iframe>
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
        "byte_end": 714,
        "byte_start": 683,
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/sandbox_025.htm"
}
```
