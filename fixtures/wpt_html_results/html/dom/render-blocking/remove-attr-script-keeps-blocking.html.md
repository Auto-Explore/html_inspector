# html/dom/render-blocking/remove-attr-script-keeps-blocking.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/render-blocking/remove-attr-script-keeps-blocking.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Synchronous script element still blocks rendering after removing `blocking=render`</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="support/test-render-blocking.js"></script>

<script>
// Test script must be added before the synchronous script because the
// synchronous script is parser-blocking.

promise_setup(async () => {
  let script = await nodeInserted(document.head, node => node.id === 'script');
  script.blocking = '';

  // Also inserts some contents for non-compliant UA to render
  document.body = document.createElement('body');
  document.body.appendChild(document.createTextNode('Some text'));
});

test_render_blocking(
  () => assert_equals(window.dummy, 1),
  'Render-blocking script is loaded and evaluated');
</script>

<script id="script" blocking="render" src="support/dummy-1.js?pipe=trickle(d1)"></script>
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
  "source_name": "html/dom/render-blocking/remove-attr-script-keeps-blocking.html"
}
```
