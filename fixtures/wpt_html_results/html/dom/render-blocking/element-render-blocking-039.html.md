# html/dom/render-blocking/element-render-blocking-039.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/render-blocking/element-render-blocking-039.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta name="timeout" content="long">
<head>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="support/utils.js"></script>
<title>link rel=expect: render blocking stops waiting when the document finishes loading</title>

<link rel=expect href="#invalid" blocking="render">
<script>
const startTime = performance.now();

async_test((t) => {
  requestAnimationFrame(() => {
    t.step(() => assert_less_than(performance.now() - startTime, 3000, "requestAnimationFrame should not have been delayed by the link that didn't find its target after load completes"));
    t.done();
  });
}, "");
</script>
</head>
<body>
  <div id="first"></div>
  <div id="last"></div>
</body>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.link.blocking.requires_stylesheet",
      "message": "A “link” element with a “blocking” attribute must have a “rel” attribute whose value is “stylesheet”.",
      "severity": "Warning",
      "span": {
        "byte_end": 356,
        "byte_start": 305,
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
  "source_name": "html/dom/render-blocking/element-render-blocking-039.html"
}
```
