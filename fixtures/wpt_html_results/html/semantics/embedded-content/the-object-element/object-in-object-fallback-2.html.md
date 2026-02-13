# html/semantics/embedded-content/the-object-element/object-in-object-fallback-2.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-object-element/object-in-object-fallback-2.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <meta charset=utf-8>
    <title></title>
    <script src=/resources/testharness.js></script>
    <script src=/resources/testharnessreport.js></script>
    <script>
      var loadedCount = 0;
      var nestingTest = async_test("Test <object> nesting inside <object>");
      onload = nestingTest.step_func_done(function() {
        assert_equals(loadedCount, 12, "Should have loaded all should-load elements");
      });
    </script>
  </head>
  <body>
    <object data="../resources/should-load.html" style="width: 100px; height: 100px">
      <object type="text/html" data="../resources/should-not-load.html"
              test-description="<object> inside <object>"></object>
    </object>
    <object style="width: 100px; height: 100px" data="data:application/x-does-not-exist,test">
      <object type="text/html" data="../resources/should-load.html"></object>
    </object>
    <object style="width: 100px; height: 100px" data="data:application/x-does-not-exist,test">
      <div></div>
      <object type="text/html" data="../resources/should-load.html"></object>
    </object>
    <object style="width: 100px; height: 100px" data="data:application/x-does-not-exist,test">
      <div>
        <object type="text/html" data="../resources/should-load.html"></object>
      </div>
    </object>
    <object style="width: 100px; height: 100px" data="data:application/x-does-not-exist,test">
      <object type="text/html" data="../resources/should-load.html"></object>
      <object type="text/html" data="../resources/should-load.html"></object>
      <object data="../resources/should-load.html">
        <object type="text/html" data="../resources/should-not-load.html"
                test-description="<object> inside loaded <object> inside non-loaded <object>"></object>
      </object>
      <object data="data:application/x-does-not-exist,test">
        <object type="text/html" data="../resources/should-load.html"></object>
      </object>
    </object>
    <div>
      <object data="../resources/should-load.html" style="width: 100px; height: 100px"></object>
      <object type="text/html" data="../resources/should-load.html"></object>
    </div>
    <div>
      <object type="text/html" data="../resources/should-load.html"></object>
      <object data="../resources/should-load.html" style="width: 100px; height: 100px"></object>
    </div>
  </body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.title.empty",
      "message": "Element “title” must not be empty.",
      "severity": "Warning",
      "span": {
        "byte_end": 68,
        "byte_start": 61,
        "col": 5,
        "line": 5
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
  "source_name": "html/semantics/embedded-content/the-object-element/object-in-object-fallback-2.html"
}
```
