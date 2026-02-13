# html/semantics/embedded-content/the-img-element/srcset/srcset-media-dynamic.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/srcset/srcset-media-dynamic.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>source element in picture handles dynamic media change correctly.</title>
<link rel="author" href="mailto:emilio@crisal.io" title="Emilio Cobos Álvarez">
<link rel="author" href="https://mozilla.org" title="Mozilla">
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1523627">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<picture id="pic">
  <source srcset="data:,a">
</picture>
<script>
let t = async_test("Dynamic media change is handled correctly");

let pic = document.getElementById("pic");
// Something that will never match.
pic.querySelector("source").setAttribute("media", "not all");

let img = document.createElement("img");
img.src = "data:,b";
pic.appendChild(img);

onload = t.step_func_done(function() {
  assert_equals(img.currentSrc, "data:,b");
});
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “data:,a” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 471,
        "byte_start": 446,
        "col": 3,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.picture.missing_img",
      "message": "Element “picture” is missing a required instance of child element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 482,
        "byte_start": 472,
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
  "source_name": "html/semantics/embedded-content/the-img-element/srcset/srcset-media-dynamic.html"
}
```
