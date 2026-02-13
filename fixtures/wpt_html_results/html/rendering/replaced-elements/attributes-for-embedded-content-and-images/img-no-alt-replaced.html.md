# html/rendering/replaced-elements/attributes-for-embedded-content-and-images/img-no-alt-replaced.html

Counts:
- errors: 0
- warnings: 7
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/attributes-for-embedded-content-and-images/img-no-alt-replaced.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Images without alt attribute or with an empty alt attribute render as replaced elements regardless of src</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<link rel="author" href="mailto:emilio@crisal.io" title="Emilio Cobos Álvarez">
<link rel="author" href="mailto:yuzhehan@chromium.org" title="Yu Han">
<link rel="author" href="https://mozilla.org" title="Mozilla">
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1196668">
<link rel="help" href="https://crbug.com/753868">
<link ref="help" href="https://html.spec.whatwg.org/multipage/rendering.html#images-3">
<style>
  img {
    width: 100px;
    height: 150px;
  }
</style>
<img>
<img src="broken">
<img alt="">
<img alt>
<img src="broken" alt="">
<script>
const t = async_test("Images without alt attribute or with an empty alt attribute render as replaced elements regardless of src");
onload = t.step_func_done(function() {
  for (const img of document.querySelectorAll("img")) {
    assert_equals(img.offsetWidth, 100, `width: ${img.outerHTML}`);
    assert_equals(img.offsetHeight, 150, `height: ${img.outerHTML}`);
  }
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
        "byte_end": 673,
        "byte_start": 586,
        "col": 1,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 745,
        "byte_start": 740,
        "col": 1,
        "line": 17
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 745,
        "byte_start": 740,
        "col": 1,
        "line": 17
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 764,
        "byte_start": 746,
        "col": 1,
        "line": 18
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 777,
        "byte_start": 765,
        "col": 1,
        "line": 19
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 787,
        "byte_start": 778,
        "col": 1,
        "line": 20
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
  "source_name": "html/rendering/replaced-elements/attributes-for-embedded-content-and-images/img-no-alt-replaced.html"
}
```
