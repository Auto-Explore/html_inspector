# html/semantics/scripting-1/the-script-element/script-noembed-noframes-iframe.xhtml

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/script-noembed-noframes-iframe.xhtml",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<html xmlns="http://www.w3.org/1999/xhtml">
<head>
<title>Script inside noembed, noframes and iframe</title>
<link rel="author" title="Ms2ger" href="mailto:ms2ger@gmail.com"/>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
</head>
<body>
<div id="log"></div>
<script>
var run = [];
</script>
<div id="test">
<noembed>
<script>
run.push("noembed");
</script>
</noembed>
<noframes>
<script>
run.push("noframes");
</script>
</noframes>
<iframe>
<script>
run.push("iframe");
</script>
</iframe>
</div>
<script>
test(function() {
  assert_array_equals(run, ["noembed", "noframes", "iframe"], "Haven't run.");
});
</script>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.element.noframes.obsolete",
      "message": "The “noframes” element is obsolete. Use the “iframe” element and CSS instead, or use server-side includes.",
      "severity": "Warning",
      "span": {
        "byte_end": 438,
        "byte_start": 428,
        "col": 1,
        "line": 19
      }
    },
    {
      "category": "Html",
      "code": "html.iframe.text.disallowed",
      "message": "Text not allowed in “iframe” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 540,
        "byte_start": 500,
        "col": 9,
        "line": 24
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
  "source_name": "html/semantics/scripting-1/the-script-element/script-noembed-noframes-iframe.xhtml"
}
```
