# html/dom/documents/resource-metadata-management/document-readyState.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/documents/resource-metadata-management/document-readyState.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>document.readyState</title>
<link rel="author" title="Denis Ah-Kang" href="mailto:denis@w3.org">
<link rel=help href="https://html.spec.whatwg.org/multipage/#resource-metadata-management">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
  var t1 = async_test("readyState equals 'complete' when the document has loaded"),
      t2 = async_test("readyState equals 'interactive' when the document is finished parsing"),
      t3 = async_test("readystatechange event is fired each time document.readyState changes");

  window.onload = t1.step_func_done(function(){
    assert_equals(document.readyState, "complete");
  });

  document.addEventListener("DOMContentLoaded", function(event) {
    t2.step(function() {
      assert_equals(document.readyState, "interactive")
    });
    t2.done();
  });

  var states = [document.readyState];
  document.onreadystatechange = t3.step_func(function(){
    states.push(document.readyState);
    if (document.readyState === "complete") {
      assert_array_equals(states, ["loading", "interactive", "complete"]);
      t3.done();
    }
  })
</script>
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
  "source_name": "html/dom/documents/resource-metadata-management/document-readyState.html"
}
```
