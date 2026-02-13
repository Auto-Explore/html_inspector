# html/rendering/replaced-elements/images/img-represents-nothing-style-mutation.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/images/img-represents-nothing-style-mutation.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>&lt;img> representing nothing and image related style mutation</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/rendering.html#images-3">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<img>
<script>
  test(t => {
    const img = document.querySelector('img');
    const boundsBefore = img.getBoundingClientRect();
    assert_equals(boundsBefore.width, 0);
    assert_equals(boundsBefore.height, 0);

    img.style.imageOrientation = 'none';

    const boundsAfter = img.getBoundingClientRect();
    assert_equals(boundsAfter.width, 0);
    assert_equals(boundsAfter.height, 0);
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
        "byte_end": 293,
        "byte_start": 288,
        "col": 1,
        "line": 6
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 293,
        "byte_start": 288,
        "col": 1,
        "line": 6
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
  "source_name": "html/rendering/replaced-elements/images/img-represents-nothing-style-mutation.html"
}
```
