# html/semantics/document-metadata/the-style-element/style_load_async.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-style-element/style_load_async.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
<meta charset="utf-8">
<title>Style load event should be async</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
    var t = async_test("style load should be async");
    var sync = true;
    function check() {
        assert_false(sync);
        t.done();
    }
</script>
<style onload="t.step(check)">
</style>
<script>
  sync = false
</script>

<body>
  <div id="log"></div>
  <div id="test"></div>
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
  "source_name": "html/semantics/document-metadata/the-style-element/style_load_async.html"
}
```
