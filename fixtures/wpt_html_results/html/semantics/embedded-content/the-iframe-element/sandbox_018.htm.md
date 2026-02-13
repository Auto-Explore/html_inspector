# html/semantics/embedded-content/the-iframe-element/sandbox_018.htm

Counts:
- errors: 0
- warnings: 6
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/sandbox_018.htm",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
    <title>HTML5 Sandbox: value of sandbox attribute must be an unordered set of unique space-separated tokens.</title>
    <meta content="text/html; charset=UTF-8" http-equiv="Content-Type" />
    <link rel="author" title="Microsoft" href="http://www.microsoft.com/" />
    <link rel="help" href="http://dev.w3.org/html5/spec/Overview.html#attr-iframe-sandbox" />
    <meta name="assert" content="value of sandbox attribute must be an unordered set of unique space-separated tokens." />
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
</head>
<body>
    <script type="text/javascript">

        var t = async_test("value of sandbox attribute must be an unordered set of unique space-separated tokens.");

        function callback(event)
        {
            t.step(function(){
                assert_true('sandbox' in document.createElement('iframe'));
                assert_equals(event.data, "cookies are R/W");
            });
            t.done();
        }

        var timer = setTimeout(callback, 4000);
        window.addEventListener("message", callback, false);
    </script>
    <div id=log></div>

    <iframe style="display:none" src="support/iframe_sandbox_012.htm" sandbox="&#10ALLOW-SCRIPTS&#10allow-same-origin&#10"></iframe>

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
        "byte_end": 682,
        "byte_start": 651,
        "col": 5,
        "line": 13
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_no_semicolon",
      "message": "Character reference was not terminated by a semicolon.",
      "severity": "Warning",
      "span": {
        "byte_end": 1215,
        "byte_start": 1214,
        "col": 5,
        "line": 31
      }
    },
    {
      "category": "Html",
      "code": "html.iframe.sandbox.scripts_and_same_origin",
      "message": "Bad value “\nALLOW-SCRIPTS\nallow-same-origin\n” for attribute “sandbox” on element “iframe”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1333,
        "byte_start": 1214,
        "col": 5,
        "line": 31
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_no_semicolon",
      "message": "Character reference was not terminated by a semicolon.",
      "severity": "Warning",
      "span": {
        "byte_end": 1232,
        "byte_start": 1231,
        "col": 5,
        "line": 31
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_no_semicolon",
      "message": "Character reference was not terminated by a semicolon.",
      "severity": "Warning",
      "span": {
        "byte_end": 1253,
        "byte_start": 1252,
        "col": 5,
        "line": 31
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/sandbox_018.htm"
}
```
