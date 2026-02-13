# html/browsers/windows/nested-browsing-contexts/resources/frameElement-sibling-accessed.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/windows/nested-browsing-contexts/resources/frameElement-sibling-accessed.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>This page will set its document.domain on request so that its sibling can access it</title>

<h1>I did get loaded</h1>

<script>
"use strict";

window.onmessage = e => {
  const { newDocumentDomain } = e.data;
  document.domain = newDocumentDomain;

  parent.postMessage("done", "*");
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
  "source_name": "html/browsers/windows/nested-browsing-contexts/resources/frameElement-sibling-accessed.html"
}
```
