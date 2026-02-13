# html/dom/render-blocking/script-inserted-style-element.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/render-blocking/script-inserted-style-element.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Script-inserted style elements with "blocking=render" are render-blocking</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="support/test-render-blocking.js"></script>

<script>
const style = document.createElement('style');
style.blocking = 'render';
style.textContent = "@import url('support/target-red.css?pipe=trickle(d1)');";
document.head.appendChild(style);
</script>

<div class="target">
  This should be red
</div>

<script>
test_render_blocking(
    style,
    () => {
      let color = getComputedStyle(document.querySelector('.target')).color;
      assert_equals(color, 'rgb(255, 0, 0)');
    },
    'Render-blocking stylesheet is applied');
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
  "source_name": "html/dom/render-blocking/script-inserted-style-element.html"
}
```
