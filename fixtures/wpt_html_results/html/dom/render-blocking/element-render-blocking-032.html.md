# html/dom/render-blocking/element-render-blocking-032.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/render-blocking/element-render-blocking-032.html",
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
<title>&lt;a name&gt; should only unblock when finished parsing children</title>

<link rel=expect href="#fold" blocking="render">
<script>
async_test((t) => {
  requestAnimationFrame(() => {
    t.step(() => assert_true(!!document.getElementsByName("second")));
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
  <a name="fold">
    Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vestibulum augue
    nibh, venenatis a ligula in, tempus pharetra urna. Fusce semper, velit
    tincidunt lobortis sollicitudin, sapien velit fermentum odio, ultricies
    tempor mi est eget ipsum egestas.
    <script>
      generateParserDelay();
    </script>
    <div id="second">
      Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vestibulum augue
      nibh, venenatis a ligula in, tempus pharetra urna. Fusce semper, velit
      tincidunt lobortis sollicitudin, sapien velit fermentum odio, ultricies
      tempor mi est eget ipsum egestas.
    </div>
  </a>
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
        "byte_end": 337,
        "byte_start": 289,
        "col": 1,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.a.name.obsolete",
      "message": "The “name” attribute on the “a” element is obsolete. Consider putting an “id” attribute on the nearest container instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 1048,
        "byte_start": 1033,
        "col": 3,
        "line": 30
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
  "source_name": "html/dom/render-blocking/element-render-blocking-032.html"
}
```
