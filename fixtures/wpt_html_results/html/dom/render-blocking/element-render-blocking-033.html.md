# html/dom/render-blocking/element-render-blocking-033.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/render-blocking/element-render-blocking-033.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta name="timeout" content="long">
<head>
<meta charset="utf8">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="support/utils.js"></script>
<title>link rel=expect should support percent encoding</title>

<link rel=expect href="#se%F0%9F%98%8Fcond" blocking="render">
<script>
async_test((t) => {
  requestAnimationFrame(() => {
    t.step(() => assert_true(!!document.getElementById("se😏cond")));
    t.step(() => assert_false(!!document.getElementById("last")), "the second element should already unblock rendering");
    t.done();
  });
}, "blocking defers frames until full parsing");
</script>
</head>
<body>
  <div id="first">
    Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vestibulum augue
    nibh, venenatis a ligula in, tempus pharetra urna. Fusce semper, velit
    tincidunt lobortis sollicitudin, sapien velit fermentum odio, ultricies
    tempor mi est eget ipsum egestas.
  </div>
  <script>
    generateParserDelay();
  </script>
  <div id="se😏cond">
    Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vestibulum augue
    nibh, venenatis a ligula in, tempus pharetra urna. Fusce semper, velit
    tincidunt lobortis sollicitudin, sapien velit fermentum odio, ultricies
    tempor mi est eget ipsum egestas.
  </div>
  <script>
    generateParserDelay();
  </script>
  <div id="last">
    Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vestibulum augue
    nibh, venenatis a ligula in, tempus pharetra urna. Fusce semper, velit
    tincidunt lobortis sollicitudin, sapien velit fermentum odio, ultricies
    tempor mi est eget ipsum egestas.
  </div>
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
        "byte_end": 355,
        "byte_start": 293,
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
  "source_name": "html/dom/render-blocking/element-render-blocking-033.html"
}
```
