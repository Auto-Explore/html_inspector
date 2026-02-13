# html/rendering/replaced-elements/attributes-for-embedded-content-and-images/content-aspect-ratio.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/attributes-for-embedded-content-and-images/content-aspect-ratio.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>div with content style's width and height attributes are not used to infer aspect-ratio</title>
<link rel="help" href="https://bugs.webkit.org/show_bug.cgi?id=201641#c22">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<style>
  video {
    width: 100%;
    max-width: 100px;
    height: auto;
  }
</style>
<body>
<script>
// Create and append a div with content style and immediately check the height.
let t = test(function() {
  var div = document.createElement("div");
  div.setAttribute("style", "content: url('/images/blue.png')");
  div.setAttribute("width", "250");
  div.setAttribute("height", "100");
  document.body.appendChild(div);
  assert_equals(getComputedStyle(div).height, "0px");
}, "div with content style's width and height attributes are not used to infer aspect-ratio");

</script>
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
  "source_name": "html/rendering/replaced-elements/attributes-for-embedded-content-and-images/content-aspect-ratio.html"
}
```
