# html/semantics/embedded-content/the-img-element/sizes/implicit-sizes-ignores-width.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/sizes/implicit-sizes-ignores-width.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta name="viewport" content="width=device-width,initial-scale=1,minimum-scale=1">
<title>Implicit sizes ignores width</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<style>
img {width: auto;}
</style>
<img srcset="../srcset/resources/image.png 100w" sizes="400px" id="sizes">
<img srcset="../srcset/resources/image.png 100w" width="400" id="width">
<script>
setup({explicit_done:true});
onload = () => {
  test(() => {
    assert_equals(document.getElementById("sizes").width, 400);
    assert_equals(document.getElementById("width").width, window.innerWidth);
    done();
  });
};
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
        "byte_end": 360,
        "byte_start": 286,
        "col": 1,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 433,
        "byte_start": 361,
        "col": 1,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 433,
        "byte_start": 361,
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
  "source_name": "html/semantics/embedded-content/the-img-element/sizes/implicit-sizes-ignores-width.html"
}
```
