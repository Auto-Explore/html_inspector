# html/canvas/offscreen/manual/the-offscreen-canvas/offscreencanvas.transfer.lowlatency.nocrash.html

Counts:
- errors: 4
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/offscreen/manual/the-offscreen-canvas/offscreencanvas.transfer.lowlatency.nocrash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<title>Transfer Low-Latency Canvas</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/html/canvas/resources/canvas-tests.js"></script>
<body></body>

<script>
test(function(t) {
  var ctx = document.createElement("canvas").getContext('2d', {desynchronized:true});
  assert_throws_dom("InvalidStateError", () => { ctx.canvas.transferControlToOffscreen(); });
}, "Tests that transferring a low latency canvas does not cause a crash. See crbug.com/1255153");
</script>
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
        "byte_end": 7,
        "byte_start": 0,
        "col": 1,
        "line": 1
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_start_tag",
      "message": "Stray start tag “script”.",
      "severity": "Error",
      "span": {
        "byte_end": 235,
        "byte_start": 227,
        "col": 1,
        "line": 7
      }
    },
    {
      "category": "Html",
      "code": "html.parser.nonspace_after_body",
      "message": "Non-space character after body.",
      "severity": "Error",
      "span": {
        "byte_end": 533,
        "byte_start": 235,
        "col": 9,
        "line": 7
      }
    },
    {
      "category": "Html",
      "code": "html.parser.end_tag_after_body_closed",
      "message": "Saw an end tag after “body” had been closed.",
      "severity": "Error",
      "span": {
        "byte_end": 542,
        "byte_start": 533,
        "col": 1,
        "line": 12
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
  "source_name": "html/canvas/offscreen/manual/the-offscreen-canvas/offscreencanvas.transfer.lowlatency.nocrash.html"
}
```
