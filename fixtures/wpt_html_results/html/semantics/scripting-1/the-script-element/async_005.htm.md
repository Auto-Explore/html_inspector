# html/semantics/scripting-1/the-script-element/async_005.htm

Counts:
- errors: 0
- warnings: 6
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/async_005.htm",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
    <head>
        <title>A script element with both async and defer set should execute asynchronously</title>
        <meta name="timeout" content="long">
        <meta http-equiv="content-type" content="text/html; charset=UTF-8" />
        <meta description="This test ensures a script element with both async and defer set should execute asynchronously." />
        <link rel="author" title="Microsoft" href="http://www.microsoft.com/" />
        <link rel="help" href="https://html.spec.whatwg.org/multipage/#attr-script-async"/>
        <script src="/resources/testharness.js"></script>
        <script src="/resources/testharnessreport.js"></script>
    </head>
    <body>
 <div id=log></div>
        <script type="text/javascript">
        var t = async_test("A script element with both async and defer set should execute asynchronously");

        function timeout()
        {
            t.step(function(){
                var actual = document.getElementById("testresult").innerHTML;
                assert_in_array(actual, ["2134", "2341"]);
            });
            t.done();
        }

        var timer = setTimeout(timeout, 5000);

        function log(text)
        {
            var textNode = document.createTextNode(text);
            document.getElementById("testresult").appendChild(textNode);
        }
        </script>

        <span id="testresult"></span>

        <script type="text/javascript" src="log.py?sec=1&id=1" defer async></script>
        <script type="text/javascript">log('2');</script>
        <script type="text/javascript" src="log.py?sec=3&id=3"></script>
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
        "byte_end": 761,
        "byte_start": 730,
        "col": 9,
        "line": 15
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 1484,
        "byte_start": 1417,
        "col": 9,
        "line": 38
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 1533,
        "byte_start": 1502,
        "col": 9,
        "line": 39
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 1615,
        "byte_start": 1560,
        "col": 9,
        "line": 40
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 1664,
        "byte_start": 1633,
        "col": 9,
        "line": 41
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
  "source_name": "html/semantics/scripting-1/the-script-element/async_005.htm"
}
```
