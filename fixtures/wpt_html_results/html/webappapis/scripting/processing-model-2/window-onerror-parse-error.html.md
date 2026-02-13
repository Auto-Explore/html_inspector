# html/webappapis/scripting/processing-model-2/window-onerror-parse-error.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/scripting/processing-model-2/window-onerror-parse-error.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
 <head>
  <title>window.onerror: parse errors</title>
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
  <!--

    In https://html.spec.whatwg.org/multipage/#creating-scripts ,
    step 3 describes parsing the script, and step 5 says:
      # Otherwise, report the error using the onerror event handler of
      # the script's global object. If the error is still not handled
      # after this, then the error may be reported to the user.
    which links to
    https://html.spec.whatwg.org/multipage/#report-the-error ,
    which describes what to do when onerror is a Function.

    -->
 </head>
 <body>

  <div id="log"></div>
  <script>
    setup({allow_uncaught_exception:true});
    var error_count = 0;
    window.onerror = function(msg, url, lineno) {
      ++error_count;
      test(function() {assert_equals(url, window.location.href)},
           "correct url passed to window.onerror");
      test(function() {assert_equals(lineno, 34)},
           "correct line number passed to window.onerror");
    };
  </script>
  <script>This script does not parse correctly.</script>
  <script>
  test(function() {assert_equals(error_count, 1)},
       "correct number of calls to window.onerror");
  </script>
 </body>
</html>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/webappapis/scripting/processing-model-2/window-onerror-parse-error.html"
}
```
