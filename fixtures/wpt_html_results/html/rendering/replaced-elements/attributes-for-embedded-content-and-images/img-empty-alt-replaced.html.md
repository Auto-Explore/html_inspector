# html/rendering/replaced-elements/attributes-for-embedded-content-and-images/img-empty-alt-replaced.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/attributes-for-embedded-content-and-images/img-empty-alt-replaced.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Images with an empty alt attribute have an intrinsic size of zero</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<style>
  img {
    width: 50px;
    height: auto;
  }
</style>
<img src="broken">
<img src="broken" alt="non-empty">
<img src="broken" alt="">
<script>
const t = async_test("Images with an empty alt attribute have an intrinsic size of zero");
onload = t.step_func_done(function() {
  for (const img of document.querySelectorAll("img")) {
    const alt = img.getAttribute("alt");
    const shouldTakeUpSpace = alt == null || alt.length > 0;
    (shouldTakeUpSpace ? assert_not_equals : assert_equals)(img.offsetHeight, 0, img.outerHTML);
  }
});
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 285,
        "byte_start": 267,
        "col": 1,
        "line": 11
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
  "source_name": "html/rendering/replaced-elements/attributes-for-embedded-content-and-images/img-empty-alt-replaced.html"
}
```
