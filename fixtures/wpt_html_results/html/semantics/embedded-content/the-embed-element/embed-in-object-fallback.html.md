# html/semantics/embedded-content/the-embed-element/embed-in-object-fallback.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-embed-element/embed-in-object-fallback.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>Ensure that embed elements inside object elements load when the objects
  fall back but not otherwise</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<script>
  var child1Loaded = false;
  var child2Loaded = false;
  var child3Loaded = false;
  var parent3Loaded = false;
</script>
<object>
  <embed src="embed-in-object-fallback-subdocument.html?child1Loaded">
</object>
<object>
  <embed id="two" src="embed-in-object-fallback-subdocument.html?child2Loaded">
  <!-- Something that forces the embed to be in the tree before the <object>
       is done parsing -->
  <script>
    test(function() {
      assert_equals(document.getElementById("two").localName,
                    "embed");
    }, "We have the right embed element");
  </script>
</object>
<object data="embed-in-object-fallback-subdocument.html?parent3Loaded">
  <embed src="embed-in-object-fallback-subdocument.html?child3Loaded">
</object>
<script>
  var t = async_test("Check that the right things loaded");
  onload = t.step_func_done(function() {
    assert_true(child1Loaded, "child 1 should load");
    assert_true(child2Loaded, "child 2 should load");
    assert_false(child3Loaded, "child 3 should not load");
    assert_true(parent3Loaded, "parent 3 should load");
  });
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
        "byte_end": 396,
        "byte_start": 388,
        "col": 1,
        "line": 13
      }
    },
    {
      "category": "Html",
      "code": "html.object.data.missing",
      "message": "Element “object” is missing required attribute “data”.",
      "severity": "Warning",
      "span": {
        "byte_end": 486,
        "byte_start": 478,
        "col": 1,
        "line": 16
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
  "source_name": "html/semantics/embedded-content/the-embed-element/embed-in-object-fallback.html"
}
```
