# html/browsers/sandboxing/sandbox-navigation-timing-iframe.tentative.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/sandboxing/sandbox-navigation-timing-iframe.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<script>
  let result;
  try {
    parent.document.getElementsByClassName('script');
    result = 'iframe not sandboxed'
  } catch (e) {
    result = 'iframe sandboxed(' + e.message + ')';
  }
  window.onmessage = m => {
    window.parent.postMessage({
      result: result
    }, '*');
  };
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/browsers/sandboxing/sandbox-navigation-timing-iframe.tentative.html"
}
```
