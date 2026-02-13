# html/semantics/scripting-1/the-script-element/async_002.htm

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/async_002.htm",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
    <head>
        <title>Changes to the 'async' attribute are reflected in the async property</title>
        <meta http-equiv="content-type" content="text/html; charset=UTF-8" />
        <meta description="This test ensures changes to the 'async' attribute are reflected in the async property." />
        <link rel="author" title="Microsoft" href="http://www.microsoft.com/" />
        <link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-script-async"/>
        <script src="/resources/testharness.js"></script>
        <script src="/resources/testharnessreport.js"></script>
    </head>
    <body>
        <div id=log></div>
        <script type="text/javascript">
          test(function() {
            var s = document.createElement("script");
            s.async = false;
            s.setAttribute('async', ''); /*Should change s.async to true*/
            assert_true(s.async)
          }, "Test 'async' attribute are reflected in the async property with setAttribute");

          test(function() {
            var s = document.createElement("script");
            s.async = false;
            s.setAttribute('async', ''); /*Should change s.async to true*/
            s.removeAttribute('async'); /*Should change s.async to false*/
            assert_false(s.async)
          }, "Test 'async' attribute are reflected in the async property with removeAttribute");
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
        "byte_end": 706,
        "byte_start": 675,
        "col": 9,
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
  "source_name": "html/semantics/scripting-1/the-script-element/async_002.htm"
}
```
