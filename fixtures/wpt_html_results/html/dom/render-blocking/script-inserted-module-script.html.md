# html/dom/render-blocking/script-inserted-module-script.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/render-blocking/script-inserted-module-script.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Script-inserted module script elements with "blocking=render" are render-blocking</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="support/test-render-blocking.js"></script>

<script>
const moduleScript = document.createElement('script');
moduleScript.type = 'module';
moduleScript.blocking = 'render';
moduleScript.src = 'support/dummy-1.mjs?pipe=trickle(d1)';
document.head.appendChild(moduleScript);
</script>

<div id="dummy">some text</div>

<script>
test_render_blocking(
    moduleScript,
    () => assert_equals(document.getElementById('dummy').textContent, '1'),
    'Script-inserted render-blocking module script is evaluated');
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
  "source_name": "html/dom/render-blocking/script-inserted-module-script.html"
}
```
