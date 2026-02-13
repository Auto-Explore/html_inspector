# html/dom/render-blocking/remove-attr-style-keeps-blocking.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/render-blocking/remove-attr-style-keeps-blocking.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Parser-inserted style element still blocks rendering after removing `blocking=render`</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="support/test-render-blocking.js"></script>

<script>
// Test script must be added before the style element because the style
// element is script-blocking.

promise_setup(async () => {
  let sheet = await nodeInserted(document.head, node => node.id === 'sheet');
  sheet.blocking = '';
});

test_render_blocking(
  () => {
    let color = getComputedStyle(document.querySelector('.target')).color;
    assert_equals(color, 'rgb(255, 0, 0)');
  },
  'Render-blocking stylesheet is applied');
</script>

<style id="sheet" blocking="render">
@import url("support/target-red.css?pipe=trickle(d1)");
</style>

<div class="target">Some text</div>
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
  "source_name": "html/dom/render-blocking/remove-attr-style-keeps-blocking.html"
}
```
