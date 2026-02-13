# html/dom/render-blocking/element-render-blocking-038.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/render-blocking/element-render-blocking-038.html",
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
<title>link rel=expect: only connected elements are eligible</title>

<link rel=expect href="#second" blocking="render">
<script>
async_test((t) => {
  requestAnimationFrame(() => {
    t.step(() => assert_true(!!document.getElementById("second"), "The second element should be there"));
    t.step(() => assert_false(!!document.getElementById("last"), "The last element should not be there yet, even though it's created (in a shadow root)"));
    t.done();
  });
}, "");
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
  <div id="second">
    <custom-element>
      <template shadowrootmode="open">
        <div id="last">
          Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vestibulum augue
          nibh, venenatis a ligula in, tempus pharetra urna. Fusce semper, velit
          tincidunt lobortis sollicitudin, sapien velit fermentum odio, ultricies
          tempor mi est eget ipsum egestas.
        </div>
      </template>
    </custom-element>
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
        "byte_end": 327,
        "byte_start": 277,
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
  "source_name": "html/dom/render-blocking/element-render-blocking-038.html"
}
```
