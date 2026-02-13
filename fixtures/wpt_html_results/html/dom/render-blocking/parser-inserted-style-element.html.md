# html/dom/render-blocking/parser-inserted-style-element.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/render-blocking/parser-inserted-style-element.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Parser-inserted style elements are implicitly render-blocking</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="support/test-render-blocking.js"></script>
<script>
// Test case must be set up before the stylesheet, because the stylesheet is
// script-blocking, which means we can't set it up while the stylesheet is
// loading.
test_render_blocking(() => {
  let color = getComputedStyle(document.querySelector('.target')).color;
  assert_equals(color, 'rgb(255, 0, 0)');
}, 'Render-blocking stylesheet is applied');
</script>
<style>
@import url('support/target-red.css?pipe=trickle(d1)');
</style>
<div class="target">
  This should be red
</div>
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
  "source_name": "html/dom/render-blocking/parser-inserted-style-element.html"
}
```
