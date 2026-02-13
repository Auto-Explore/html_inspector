# html/semantics/embedded-content/the-object-element/object-in-display-none-load-event.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-object-element/object-in-display-none-load-event.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
<meta charset=utf-8>
<title>Test that an object in a display:none subtree does not block the load event</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
  let objLoaded = false;
  function objOnload() {
    objLoaded = true;
  }
  async_test(t => {
    addEventListener('load', t.step_func_done(() => {
      assert_true(!!document.querySelector('object'));
      assert_false(objLoaded);
    }));
  }, 'Load event triggered on window without the object element load');
</script>
<div style="display: none;">
  <object data="data:text/html," onload="objOnload()"></object>
</div>
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
  "source_name": "html/semantics/embedded-content/the-object-element/object-in-display-none-load-event.html"
}
```
