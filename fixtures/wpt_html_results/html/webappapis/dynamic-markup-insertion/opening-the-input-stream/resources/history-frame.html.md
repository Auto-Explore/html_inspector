# html/webappapis/dynamic-markup-insertion/opening-the-input-stream/resources/history-frame.html

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/dynamic-markup-insertion/opening-the-input-stream/resources/history-frame.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<script>
function queueTest() {
  // The timeout is necessary to avoid the parser still being active when
  // `document.open()` is called and becoming a no-op.
  //
  // We also cannot use setTimeout(..., 0), as the parser is terminated in a
  // task with DOM manipulation task source while the timeout is run in a task
  // on the timer task source. The order is therefore not guaranteed. Let's
  // play it safer and use some actual timeout.
  setTimeout(() => {
    document.open();
    document.write("<p>New content</p>");
    document.close();
    opener.onDocumentOpen();
  }, 200);
}
</script>
<body onload="opener.onFrameLoaded(); queueTest();">
<p>Old content</p>
</body>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.start_tag_without_doctype",
      "message": "Start tag seen without seeing a doctype first. Expected “<!DOCTYPE html>”.",
      "severity": "Error",
      "span": {
        "byte_end": 8,
        "byte_start": 0,
        "col": 1,
        "line": 1
      }
    },
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
  "source_name": "html/webappapis/dynamic-markup-insertion/opening-the-input-stream/resources/history-frame.html"
}
```
