# html/semantics/embedded-content/the-img-element/responsive-image-select-print.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/responsive-image-select-print.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Test print result of responsive image</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/images.html#srcset-attribute">
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1803094">
<link rel="author" href="https://mozilla.org" title="Mozilla">
<link rel="match" href="responsive-image-select-print-ref.html">
<style>
  :root {
    print-color-adjust: exact;
  }
  @page {
    size: 300px;
    margin: 0;
  }
</style>
<body>
  <picture>
    <source width="200" srcset="./resources/red.png 1w, ./resources/green.png 200w">
    <img>
  </picture>
</body>
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
        "byte_end": 589,
        "byte_start": 584,
        "col": 5,
        "line": 19
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 589,
        "byte_start": 584,
        "col": 5,
        "line": 19
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 579,
        "byte_start": 499,
        "col": 5,
        "line": 18
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
  "source_name": "html/semantics/embedded-content/the-img-element/responsive-image-select-print.html"
}
```
