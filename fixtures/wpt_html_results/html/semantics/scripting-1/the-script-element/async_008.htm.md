# html/semantics/scripting-1/the-script-element/async_008.htm

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/async_008.htm",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
    <head>
        <title>Async script element execution delays the window's load event</title>
        <meta http-equiv="content-type" content="text/html; charset=UTF-8" />
        <meta description="This test ensures an async script element's execution delays the window's load event." />
        <link rel="author" title="Microsoft" href="http://www.microsoft.com/" />
        <link rel="help" href="https://html.spec.whatwg.org/multipage/#delay-the-load-event"/>
        <script src="/resources/testharness.js"></script>
        <script src="/resources/testharnessreport.js"></script>
    </head>
    <body>
        <div id=log></div>
        <script type="text/javascript">
        var t = async_test("Async script element execution delays the window's load event");

        function timeout()
        {
            t.step(function(){ assert_equals(document.getElementById("testresult").innerHTML, "213")});
            t.done();
        }

        var timer = setTimeout(timeout, 8000);

        function log(text)
        {
            var textNode = document.createTextNode(text);
            document.getElementById("testresult").appendChild(textNode);
        }
        </script>

        <span id="testresult"></span>
        <script type="text/javascript">
            window.addEventListener("load", function() {
                log("3");
                timeout();
            }, false);

            var s1 = document.createElement("script");
            s1.src = "log.py?sec=2&id=1";
            document.head.appendChild(s1);
        </script>
        <script type="text/javascript">
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
        "byte_end": 701,
        "byte_start": 670,
        "col": 9,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 1292,
        "byte_start": 1261,
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
        "byte_end": 1624,
        "byte_start": 1593,
        "col": 9,
        "line": 43
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
  "source_name": "html/semantics/scripting-1/the-script-element/async_008.htm"
}
```
