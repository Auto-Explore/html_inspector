# html/semantics/embedded-content/the-img-element/img-picture-ancestor.html

Counts:
- errors: 0
- warnings: 8
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/img-picture-ancestor.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>img should only look at a parent picture element</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<picture>
  <source media="not all" srcset="data:,a">
  <source media="all" srcset="data:,b">
  <img src="data:,c">
  <picture>
    <source media="not all" srcset="data:,e">
    <source media="all" srcset="data:,f">
    <img src="data:,g">
  </picture>
</picture>
<script>
const picture1 = document.querySelector("picture");
const picture2 = document.querySelector("picture > picture");
const img1 = document.querySelector("picture > img");
const img2 = document.querySelector("picture > picture > img");

const div = document.createElement("div");

const imgInsideDiv = document.createElement("img");
imgInsideDiv.src = "data:,d";
div.append(imgInsideDiv);

test(function() {
  assert_equals(img1.currentSrc, "data:,b");
}, "currentSrc of img in normally parented picture is correct");

test(function() {
  assert_equals(img2.currentSrc, "data:,f");
}, "currentSrc of img in nested picture element is correct");

async_test(function(t) {
  picture1.append(div);
  queueMicrotask(t.step_func(function() {
    assert_equals(imgInsideDiv.currentSrc, "data:,d");
    t.done();
  }));
}, "currentSrc of img with picture ancestor but non-picture parent is correct");

async_test(function(t) {
  picture2.remove();
  queueMicrotask(t.step_func(function() {
    assert_equals(img2.currentSrc, "data:,f");
    t.done();
  }));
}, "currentSrc of img in nested picture element remains correct when the inner picture is removed from the document");
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
        "byte_end": 239,
        "byte_start": 198,
        "col": 3,
        "line": 6
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “data:,b” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 279,
        "byte_start": 242,
        "col": 3,
        "line": 7
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 301,
        "byte_start": 282,
        "col": 3,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.picture.child.picture.disallowed",
      "message": "Element “picture” not allowed as child of “picture” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 313,
        "byte_start": 304,
        "col": 3,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “data:,e” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 359,
        "byte_start": 318,
        "col": 5,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “data:,f” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 401,
        "byte_start": 364,
        "col": 5,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 425,
        "byte_start": 406,
        "col": 5,
        "line": 12
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
  "source_name": "html/semantics/embedded-content/the-img-element/img-picture-ancestor.html"
}
```
