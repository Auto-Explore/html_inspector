# html/semantics/scripting-1/the-script-element/async_009.htm

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/async_009.htm",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
    <head>
        <title>Document.write() silently fails from an Async script</title>
        <meta http-equiv="content-type" content="text/html; charset=UTF-8" />
        <meta description="This test ensures Document.write() silently fails from an Async script." />
        <link rel="author" title="Microsoft" href="http://www.microsoft.com/" />
        <link rel="help" href="https://html.spec.whatwg.org/multipage/#execute-the-script-block"/>
        <script src="/resources/testharness.js"></script>
        <script src="/resources/testharnessreport.js"></script>
        <script type="text/javascript">
        var t = async_test("Document.write() silently fails from an Async script");

        var log = t.step_func(function() {
          document.write("<span id='writtenText'></span>");
          assert_equals(null, document.getElementById('writtenText'));
          t.done();
        });
        </script>
    </head>
    <body>
        <div id=log></div>
        <script type="text/javascript" src="log.py?sec=1&id=1" async></script>
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
        "byte_end": 632,
        "byte_start": 601,
        "col": 9,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 1061,
        "byte_start": 1000,
        "col": 9,
        "line": 23
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
  "source_name": "html/semantics/scripting-1/the-script-element/async_009.htm"
}
```
