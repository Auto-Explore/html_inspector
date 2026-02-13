# html/browsers/browsing-the-web/navigating-across-documents/resources/multi-globals-subframe-1.sub.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/navigating-across-documents/resources/multi-globals-subframe-1.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Multi-globals test outer subframe</title>

<script>
"use strict";
document.domain = "{{hosts[][]}}";
</script>

<iframe src="http://{{hosts[][]}}:{{ports[http][0]}}/html/browsers/browsing-the-web/navigating-across-documents/resources/multi-globals-subframe-2.sub.html"></iframe>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.iframe.src.invalid",
      "message": "Bad value “http://{{hosts[][]}}:{{ports[http][0]}}/html/browsers/browsing-the-web/navigating-across-documents/resources/multi-globals-subframe-2.sub.html” for attribute “src” on element “iframe”.",
      "severity": "Warning",
      "span": {
        "byte_end": 315,
        "byte_start": 158,
        "col": 1,
        "line": 10
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/browsers/browsing-the-web/navigating-across-documents/resources/multi-globals-subframe-1.sub.html"
}
```
