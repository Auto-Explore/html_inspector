# html/semantics/embedded-content/the-object-element/historical.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-object-element/historical.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Historical object element features should not be supported</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id=log></div>
<object id=object></object>
<script>
test(function() {
  var elm = document.getElementById('object');
  assert_equals(typeof elm, 'object', 'typeof');
  assert_throws_js(TypeError, function() {
    elm();
  });
}, 'object legacycaller should not be supported');

test(() => {
  const obj = document.createElement("object");
  assert_false("typeMustMatch" in obj);
}, "object's typeMustMatch IDL attribute should not be supported");

async_test(t => {
  const obj = document.createElement("object");
  t.add_cleanup(() => obj.remove());
  obj.setAttribute("data", "/common/blank.html");
  obj.setAttribute("type", "text/plain");
  obj.setAttribute("typemustmatch", "");
  obj.onload = t.step_func_done(() => {
    assert_not_equals(obj.contentDocument, null, "/common/blank.html should be loaded");
  });
  obj.onerror = t.unreached_func();
  document.body.appendChild(obj);
}, "object's typemustmatch content attribute should not be supported");

async_test(t => {
  const obj = document.createElement("object");
  t.add_cleanup(() => obj.remove());
  obj.setAttribute("data", "/common/blank.html");
  obj.setAttribute("codebase", "https://test.invalid/");
  obj.onload = t.step_func_done(() => {
    assert_not_equals(obj.contentDocument, null, "/common/blank.html should be loaded");
    assert_equals(obj.contentDocument.location.origin, location.origin, "document should be loaded with current origin as base");
  });
  obj.onerror = t.unreached_func();
  document.body.appendChild(obj);
}, "object's codebase content attribute should not be supported");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.object.data.missing",
      "message": "Element “object” is missing required attribute “data”.",
      "severity": "Warning",
      "span": {
        "byte_end": 233,
        "byte_start": 215,
        "col": 1,
        "line": 6
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
  "source_name": "html/semantics/embedded-content/the-object-element/historical.html"
}
```
