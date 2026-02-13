# html/semantics/embedded-content/the-iframe-element/sandbox_005.htm

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/sandbox_005.htm",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
    <title>HTML5 Sandbox: Block script execution inside iframe with sandbox attribute.</title>
    <meta name="timeout" content="long">
    <meta content="text/html; charset=UTF-8" http-equiv="Content-Type" />
    <link rel="author" title="Microsoft" href="http://www.microsoft.com/" />
    <link rel="help" href="http://dev.w3.org/html5/spec/Overview.html#sandboxed-scripts-browsing-context-flag" />
    <meta name="assert" content="Block script execution inside iframe with sandbox attribute." />
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
</head>
<body>
    <script type="text/javascript">

        var t = async_test("Block script execution inside iframe with sandbox attribute.");

        function callback(event)
        {
            t.step(function(){
                assert_true('sandbox' in document.createElement('iframe'));
                assert_true(!event);
            });
            t.done();
        }

        var timer = setTimeout(callback, 4000);
        window.addEventListener("message", callback, false);
    </script>
    <iframe src="support/iframe_sandbox_001.htm" sandbox style="display: none"></iframe>
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
        "byte_end": 693,
        "byte_start": 662,
        "col": 5,
        "line": 14
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/sandbox_005.htm"
}
```
