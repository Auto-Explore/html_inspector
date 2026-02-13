# html/semantics/embedded-content/the-img-element/nonexistent-image.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/nonexistent-image.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<meta charset="utf-8">
<title>Loading an nonexisting image should fail; triggering appropriate events</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<img>

<script>
  async_test(function(t) {
    var img = document.querySelector("img");
    img.onload = this.step_func_done(function() {
      assert_unreached("image.onload() was not supposed to be called");
    });
    img.onerror = this.step_func_done(function(e) {
      assert_equals(e.type, "error", "image.onerror() called");
      t.done();
    });
    img.src = "404";
  });
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
        "byte_end": 238,
        "byte_start": 233,
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
        "byte_end": 238,
        "byte_start": 233,
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
  "source_name": "html/semantics/embedded-content/the-img-element/nonexistent-image.html"
}
```
