# html/semantics/forms/the-input-element/resources/show-picker-child-iframe.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/resources/show-picker-child-iframe.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Test showPicker() in an iframe</title>
<script type=module>
import inputTypes from "./../input-types.js";

const urlParams = new URLSearchParams(location.search);
const documentDomain = urlParams.get('documentDomain');
if (documentDomain) {
  document.domain = documentDomain;
}

let securityErrors = [];
for (const inputType of inputTypes) {
  const input = document.createElement("input");
  input.setAttribute("type", inputType);

  try {
    input.showPicker();
  } catch (error) {
    if (error instanceof DOMException && error.name == 'SecurityError') {
      securityErrors.push(inputType);
    }
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
  "source_name": "html/semantics/forms/the-input-element/resources/show-picker-child-iframe.html"
}
```
