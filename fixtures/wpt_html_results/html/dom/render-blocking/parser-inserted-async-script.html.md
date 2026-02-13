# html/dom/render-blocking/parser-inserted-async-script.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/render-blocking/parser-inserted-async-script.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Parser-inserted async script elements with "blocking=render" are render-blocking</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="support/test-render-blocking.js"></script>

<script id="async-script" async blocking="render"
        src="support/dummy-1.js?pipe=trickle(d1)">
</script>

<div>Some text</div>

<script>
const asyncScript = document.getElementById('async-script');
test_render_blocking(
    asyncScript,
    () => assert_equals(window.dummy, 1),
    'Parser-inserted render-blocking async script is evaluated');
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
  "source_name": "html/dom/render-blocking/parser-inserted-async-script.html"
}
```
