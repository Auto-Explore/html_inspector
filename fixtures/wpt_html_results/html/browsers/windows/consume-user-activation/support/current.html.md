# html/browsers/windows/consume-user-activation/support/current.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/windows/consume-user-activation/support/current.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Current page used as a test helper</title>
<button id="focus-opener-button" onclick="opener.focus()">Focus opener</button>
<script>
'use strict';

onload = async () => {
  await opener.opener.test_driver.click(document.getElementById("focus-opener-button"));
  opener.postMessage("current page", "*");
};
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
  "source_name": "html/browsers/windows/consume-user-activation/support/current.html"
}
```
