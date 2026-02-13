# html/semantics/scripting-1/the-script-element/async_010.htm

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/async_010.htm",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
    <head>
        <title>Removing an async script before execution</title>
        <meta name="timeout" content="long">
        <meta http-equiv="content-type" content="text/html; charset=UTF-8" />
        <meta description="This test ensures that an async script still executes if it is removed from a markup before the download is complete. The other two scripts that come after it in insertion order should execute as well." />
        <link rel="author" title="Microsoft" href="http://www.microsoft.com/" />
        <link rel="help" href="https://html.spec.whatwg.org/multipage/#execute-the-script-block"/>
        <script src="/resources/testharness.js"></script>
        <script src="/resources/testharnessreport.js"></script>
        <script type="text/javascript">
        var t = async_test("Removing an async script before execution");

        function timeout()
        {
            t.step(function(){ assert_equals(document.getElementById("testresult").innerHTML, "4123")});
            t.done();
        }

        var timer = setTimeout(timeout, 8000);

        function log(text)
        {
            var textNode = document.createTextNode(text);
            document.getElementById("testresult").appendChild(textNode);
        }
        </script>
    </head>
    <body>
        <div id=log></div>
        <span id="testresult"></span>
        <script type="text/javascript">
            var s1 = document.createElement("script");
            s1.src="log.py?sec=2&id=1";
            s1.async = false;
            document.body.appendChild(s1);

            var s2 = document.createElement("script");
            s2.src="log.py?sec=1&id=2";
            s2.async = false;
            document.body.appendChild(s2);

            var s3 = document.createElement("script");
            s3.id = "s3";
            s3.src="log.py?sec=0&id=3";
            s3.async = false;
            document.body.appendChild(s3);

            //Remove s1 (Should still execute)
            document.body.removeChild(s1);
        </script>
        <script type="text/javascript">log('4');</script>
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
        "byte_end": 796,
        "byte_start": 765,
        "col": 9,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 1417,
        "byte_start": 1386,
        "col": 9,
        "line": 33
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 2098,
        "byte_start": 2067,
        "col": 9,
        "line": 53
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
  "source_name": "html/semantics/scripting-1/the-script-element/async_010.htm"
}
```
