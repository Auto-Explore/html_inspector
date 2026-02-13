# html/infrastructure/safe-passing-of-structured-data/shared-array-buffers/resources/iframe-messagechannel-complex.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/infrastructure/safe-passing-of-structured-data/shared-array-buffers/resources/iframe-messagechannel-complex.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<script>
  const channel = new MessageChannel();
  window.parent.postMessage({ state: "port1", data: channel.port1 }, '*', [channel.port1]);
  window.onmessage = () => window.parent.postMessage({ state: "port2", data: channel.port2 }, '*', [channel.port2]);
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
  "source_name": "html/infrastructure/safe-passing-of-structured-data/shared-array-buffers/resources/iframe-messagechannel-complex.html"
}
```
