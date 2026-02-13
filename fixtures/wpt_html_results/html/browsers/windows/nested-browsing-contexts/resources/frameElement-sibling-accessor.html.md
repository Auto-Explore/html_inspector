# html/browsers/windows/nested-browsing-contexts/resources/frameElement-sibling-accessor.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/windows/nested-browsing-contexts/resources/frameElement-sibling-accessor.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>This page will attempt to access the frameElement of its sibling and report the results on request</title>

<h1>I did get loaded</h1>

<script>
"use strict";

window.onmessage = e => {
  const { newDocumentDomain } = e.data;
  if (newDocumentDomain) {
    document.domain = newDocumentDomain;
  }

  const siblingWindow = parent.frames[0];

  try {
    const { frameElement } = siblingWindow;

    let result = "something wierd happened";
    if (frameElement === null) {
      result = "null";
    } else if (frameElement.constructor) {
      result = frameElement.constructor.name;
    }

    parent.postMessage(result, "*");
  } catch (e) {
    parent.postMessage(e.name, "*");
  }
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
  "source_name": "html/browsers/windows/nested-browsing-contexts/resources/frameElement-sibling-accessor.html"
}
```
