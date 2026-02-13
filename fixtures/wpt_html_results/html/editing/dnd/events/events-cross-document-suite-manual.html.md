# html/editing/dnd/events/events-cross-document-suite-manual.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/events/events-cross-document-suite-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>drag &amp; drop - event sequence for cross-document drag</title>
<script type="text/javascript" src="/resources/testharness.js"></script>
<script type="text/javascript" src="/resources/testharnessreport.js"></script>
<script>
var events = new Array();
</script>

<frameset cols="308,*" frameborder="no" border="0">
  <frame src="events-cross-document-suite-HELPER-1.html">
  <frame src="events-cross-document-suite-HELPER-2.html">
</frameset>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 151,
        "byte_start": 88,
        "col": 1,
        "line": 3
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 230,
        "byte_start": 161,
        "col": 1,
        "line": 4
      }
    },
    {
      "category": "Html",
      "code": "html.element.frameset.obsolete",
      "message": "The “frameset” element is obsolete. Use the “iframe” element and CSS instead, or use server-side includes.",
      "severity": "Warning",
      "span": {
        "byte_end": 337,
        "byte_start": 286,
        "col": 1,
        "line": 9
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
  "source_name": "html/editing/dnd/events/events-cross-document-suite-manual.html"
}
```
