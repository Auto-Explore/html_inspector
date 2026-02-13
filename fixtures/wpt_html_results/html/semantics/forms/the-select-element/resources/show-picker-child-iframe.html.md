# html/semantics/forms/the-select-element/resources/show-picker-child-iframe.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/resources/show-picker-child-iframe.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Test showPicker() in an iframe</title>
<script type=module>
    const urlParams = new URLSearchParams(location.search);
    const documentDomain = urlParams.get('documentDomain');
    if (documentDomain) {
        document.domain = documentDomain;
    }

    let securityErrors = [];
    const select = document.createElement("select");
    try {
        select.showPicker();
    } catch (error) {
        if (error instanceof DOMException && error.name == 'SecurityError') {
            securityErrors.push("select");
        }
    }
    parent.postMessage(securityErrors.join(','), "*");
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
  "source_name": "html/semantics/forms/the-select-element/resources/show-picker-child-iframe.html"
}
```
