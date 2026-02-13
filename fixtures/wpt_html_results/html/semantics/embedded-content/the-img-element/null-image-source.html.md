# html/semantics/embedded-content/the-img-element/null-image-source.html

Counts:
- errors: 0
- warnings: 7
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/null-image-source.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>Null image source check for src, srcset and picture parent</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id=log></div>
<img id="src_id" src="">
<img id="srcset_id" srcset="">
<picture><img id="parent_picture_id"></picture>
<script>
async_test(function(t) {
    img = document.getElementById('src_id');
    img.onerror = t.step_func(function(e) {
        assert_equals(e.type, "error", "null image source check failed");
        t.done();
    });
}, "img with empty src");

async_test(function(t) {
    img = document.getElementById('srcset_id');
    img.onerror = t.unreached_func("empty srcset fires an error");
    t.step_timeout(function() { t.done(); }, 2000);
}, "img with empty srcset");

async_test(function(t) {
    img = document.getElementById('parent_picture_id');
    img.onerror = t.unreached_func("null img with picture parent fires an error");
    t.step_timeout(function() { t.done(); }, 2000);
}, "img with picture parent");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.img.src.empty",
      "message": "Bad value “” for attribute “src” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 262,
        "byte_start": 238,
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
        "byte_end": 262,
        "byte_start": 238,
        "col": 1,
        "line": 7
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 293,
        "byte_start": 263,
        "col": 1,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 293,
        "byte_start": 263,
        "col": 1,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 331,
        "byte_start": 303,
        "col": 10,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 331,
        "byte_start": 303,
        "col": 10,
        "line": 9
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
  "source_name": "html/semantics/embedded-content/the-img-element/null-image-source.html"
}
```
