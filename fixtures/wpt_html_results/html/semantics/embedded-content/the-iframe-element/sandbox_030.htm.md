# html/semantics/embedded-content/the-iframe-element/sandbox_030.htm

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/sandbox_030.htm",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
    <title>HTML5 Sandbox: Block parent content to access sandbox child iframe content when sandbox attribute exists</title>
    <meta content="text/html; charset=UTF-8" http-equiv="Content-Type" />
    <link rel="author" title="Microsoft" href="http://www.microsoft.com/" />
    <link rel="help" href="http://dev.w3.org/html5/spec/Overview.html#sandboxed-origin-browsing-context-flag" />
    <meta name="assert" content="Block parent content to access sandbox child iframe content when sandbox attribute exists" />
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
</head>
<body>
    <script type="text/javascript">

     var t = async_test("Block parent content to access sandbox child iframe content when sandbox attribute exists");

    function callback(event)
    {
        t.step(function(){
            assert_true('sandbox' in document.createElement('iframe'));
            try { document.getElementById('sandboxIframe').contentDocument.title; assert_true(false);}
            catch(e) {assert_true(true);}
        });
        t.done();
    }
    </script>
    <div id=log></div>

    <iframe id='sandboxIframe' src="support/standalone-iframe-content.htm" sandbox onload="callback()" style="display : none"></iframe>
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
        "byte_end": 709,
        "byte_start": 678,
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/sandbox_030.htm"
}
```
