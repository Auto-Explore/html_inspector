# html/semantics/document-metadata/the-style-element/style_load_event.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-style-element/style_load_event.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>HTML Test: The style load event should fire when textContent is changed</title>
<link rel="author" href="mailto:masonf@chromium.org">
<link rel="help" href="https://html.spec.whatwg.org/multipage/semantics.html#update-a-style-block">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<script>
  var loadCount = 0;
  function load() { loadCount++; }
</script>

<style id=target onload="load()">
  .box { color:red; }
</style>
<div class='box'>Box</div>

<script>
window.onload = () => {
  const target = document.getElementById('target');
  promise_test(async t => {
    assert_equals(loadCount,1,"Style element should have loaded once by now");
    target.textContent = `.box { color: green; }`;
    await new Promise(resolve => target.addEventListener('load', resolve));
    assert_equals(loadCount,2,"Style element should fire the load event when textContent changes");
  },"style load event should fire when textContent changed");
};
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
  "source_name": "html/semantics/document-metadata/the-style-element/style_load_event.html"
}
```
