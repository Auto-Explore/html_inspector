# html/semantics/scripting-1/the-script-element/async_003.htm

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/async_003.htm",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
    <head>
        <title>An async script does not block the parser while downloading</title>
        <meta name="timeout" content="long">
        <meta http-equiv="content-type" content="text/html; charset=UTF-8" />
        <meta description="This test ensures an async script does not block the parser while downloading." />
        <link rel="author" title="Microsoft" href="http://www.microsoft.com/" />
        <link rel="help" href="https://html.spec.whatwg.org/multipage/#attr-script-async"/>
        <script src="/resources/testharness.js"></script>
        <script src="/resources/testharnessreport.js"></script>
    </head>
    <body>
        <div id=log></div>
        <script type="text/javascript">
        var t = async_test("An async script does not block the parser while downloading");

        function timeout()
        {
            t.step(function(){ assert_equals(document.getElementById("testresult").innerHTML, "21")});
            t.done();
        }

        var timer = setTimeout(timeout, 4000);

        function log(text)
        {
            var textNode = document.createTextNode(text);
            document.getElementById("testresult").appendChild(textNode);
        }
        </script>

        <span id="testresult"></span>

        <script src="log.py?sec=3&id=1" async></script>
        <script>
            log('2');
        </script>
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
        "byte_end": 734,
        "byte_start": 703,
        "col": 9,
        "line": 15
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
  "source_name": "html/semantics/scripting-1/the-script-element/async_003.htm"
}
```
