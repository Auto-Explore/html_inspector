# html/semantics/embedded-content/the-embed-element/embed-in-object-fallback-2.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-embed-element/embed-in-object-fallback-2.html",
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
      var nestingTest = async_test("Test <embed> nesting inside <object>");
      onload = nestingTest.step_func_done(function() {
        assert_equals(loadedCount, 12, "Should have loaded all should-load elements");
      });
    </script>
  </head>
  <body>
    <object data="../resources/should-load.html" style="width: 100px; height: 100px">
      <embed type="text/html" src="../resources/should-not-load.html"
             test-description="<embed> inside <object>">
    </object>
    <object style="width: 100px; height: 100px" data="data:application/x-does-not-exist,test">
      <embed type="text/html" src="../resources/should-load.html" />
    </object>
    <object style="width: 100px; height: 100px" data="data:application/x-does-not-exist,test">
      <div></div>
      <embed type="text/html" src="../resources/should-load.html" />
    </object>
    <object style="width: 100px; height: 100px" data="data:application/x-does-not-exist,test">
      <div>
        <embed type="text/html" src="../resources/should-load.html" />
      </div>
    </object>
    <object style="width: 100px; height: 100px" data="data:application/x-does-not-exist,test">
      <embed type="text/html" src="../resources/should-load.html" />
      <embed type="text/html" src="../resources/should-load.html" />
      <object data="../resources/should-load.html">
        <embed type="text/html" src="../resources/should-not-load.html"
               test-description="<embed> inside loaded <object> inside non-loaded <object>">
      </object>
      <object data="data:application/x-does-not-exist,test">
        <embed type="text/html" src="../resources/should-load.html" />
      </object>
    </object>
    <div>
      <object data="../resources/should-load.html" style="width: 100px; height: 100px"></object>
      <embed type="text/html" src="../resources/should-load.html" />
    </div>
    <div>
      <embed type="text/html" src="../resources/should-load.html" />
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
  "source_name": "html/semantics/embedded-content/the-embed-element/embed-in-object-fallback-2.html"
}
```
