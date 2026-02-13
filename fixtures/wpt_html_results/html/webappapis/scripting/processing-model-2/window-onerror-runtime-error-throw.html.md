# html/webappapis/scripting/processing-model-2/window-onerror-runtime-error-throw.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/scripting/processing-model-2/window-onerror-runtime-error-throw.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
 <head>
  <title>window.onerror: runtime scripterrors</title>
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
  <!--

    https://html.spec.whatwg.org/multipage/#runtime-script-errors
    says what to do for uncaught runtime script errors, and just below
    describes what to do when onerror is a Function.

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
      test(function() {assert_equals(lineno, 36)},
           "correct line number passed to window.onerror");
    };
  </script>
  <script>
  try {
    // This error is caught, so it should NOT trigger onerror.
    throw "foo";
  } catch (ex) {
  }
  // This error is NOT caught, so it should trigger onerror.
  throw "bar";
  </script>
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
  "source_name": "html/webappapis/scripting/processing-model-2/window-onerror-runtime-error-throw.html"
}
```
