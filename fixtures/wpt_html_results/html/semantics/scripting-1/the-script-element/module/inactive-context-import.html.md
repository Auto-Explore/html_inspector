# html/semantics/scripting-1/the-script-element/module/inactive-context-import.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/module/inactive-context-import.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Dynamic import triggered from inactive context should not crash</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<div id="container">
<iframe></iframe>
</div>

<script>
test(() => {
  const iframe = document.querySelector('iframe');
  const otherWindow = iframe.contentWindow;
  iframe.remove();

  // Below should not crash
  otherWindow.eval(`import('foobar');`);
}, 'dynamic import from inactive context should not crash');
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
  "source_name": "html/semantics/scripting-1/the-script-element/module/inactive-context-import.html"
}
```
