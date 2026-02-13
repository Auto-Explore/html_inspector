# html/semantics/embedded-content/the-img-element/404-response-with-actual-image-data.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/404-response-with-actual-image-data.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<meta charset="utf-8">
<title>404 response with actual image data should be rendered and load event is fired</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<img id="img">

<script>
  async_test(t => {
    var img = document.getElementById("img");
    img.onload = t.step_func_done(e => {
      assert_equals(e.type, "load", "image.onload() called");
    });
    img.onerror = t.unreached_func("image.onerror() was not supposed to be called");
    img.src = "404-response-with-actual-image-data.py";
  }, "404 response with actual image data should be rendered and load event is fired");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 254,
        "byte_start": 240,
        "col": 1,
        "line": 7
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 254,
        "byte_start": 240,
        "col": 1,
        "line": 7
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
  "source_name": "html/semantics/embedded-content/the-img-element/404-response-with-actual-image-data.html"
}
```
