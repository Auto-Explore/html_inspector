# html/dom/render-blocking/script-inserted-script.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/render-blocking/script-inserted-script.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Script-inserted script elements with "blocking=render" are render-blocking</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="support/test-render-blocking.js"></script>

<script>
const script = document.createElement('script');
script.src = 'support/dummy-1.js?pipe=trickle(d1)';
script.blocking = 'render';
document.head.appendChild(script);
</script>

<div>Some text</div>

<script>
test_render_blocking(
    script,
    () => assert_equals(window.dummy, 1),
    'Script-inserted render-blocking script is evaluated');
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
  "source_name": "html/dom/render-blocking/script-inserted-script.html"
}
```
