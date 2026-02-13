# html/rendering/replaced-elements/attributes-for-embedded-content-and-images/img-title-only-w-sizing.html

Counts:
- errors: 0
- warnings: 7
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/attributes-for-embedded-content-and-images/img-title-only-w-sizing.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Images with only title should be treated as a replaced element</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<link rel="author" href="mailto:yuzhehan@chromium.org" title="Yu Han">
<link rel="help" href="https://crbug.com/958250">
<link ref="help" href="https://html.spec.whatwg.org/multipage/rendering.html#images-3">
<style>
  .title-only {
    width: 100px;
    height: 150px;
  }
</style>
<img class="title-only" title="title">
<img width="100" height="150px" title="title">
<script>
async_test(t => {
  onload = t.step_func_done(function() {
    for (const img of document.querySelectorAll("img")) {
      assert_equals(img.offsetWidth, 100, `width: ${img.outerHTML}`);
      assert_equals(img.offsetHeight, 150, `height: ${img.outerHTML}`);
    }
  });
});
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.link.missing_rel_or_itemprop_or_property",
      "message": "Element “link” is missing one or more of the following attributes: “itemprop”, “property”, “rel”.",
      "severity": "Warning",
      "span": {
        "byte_end": 408,
        "byte_start": 321,
        "col": 1,
        "line": 7
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 521,
        "byte_start": 483,
        "col": 1,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 521,
        "byte_start": 483,
        "col": 1,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.img.height.bad_value",
      "message": "Bad value “150px” for attribute “height” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 568,
        "byte_start": 522,
        "col": 1,
        "line": 15
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 568,
        "byte_start": 522,
        "col": 1,
        "line": 15
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 568,
        "byte_start": 522,
        "col": 1,
        "line": 15
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
  "source_name": "html/rendering/replaced-elements/attributes-for-embedded-content-and-images/img-title-only-w-sizing.html"
}
```
