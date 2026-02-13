# html/editing/dnd/drop/events-input-same-element-manual.tentative.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/drop/events-input-same-element-manual.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>Selection drag and drop: events for &lt;input> (same element)</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body data-expected-events="
  b:drop:,
  b:beforeinput:deleteByDrag,
  b:input:deleteByDrag,
  b:beforeinput:insertFromDrop,
  b:textInput:,
  b:input:insertFromDrop">
<input id=b data-select="0,7" value="Drag me ...to here:">
<script src="/uievents/textInput/support/common.js"></script>
<script src="support/events.js"></script>
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
  "source_name": "html/editing/dnd/drop/events-input-same-element-manual.tentative.html"
}
```
