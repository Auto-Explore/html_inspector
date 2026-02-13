# html/semantics/embedded-content/the-img-element/source-relevant-mutations.html

Counts:
- errors: 0
- warnings: 18
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/source-relevant-mutations.html",
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
  <source id=s1 media=all srcset="data:,s1">
  <source id=s2 media=all srcset="data:,s2">
  <source id=s3 media=all srcset="data:,s3">
  <img id=img1 src="data:,img1">
  <img id=img2 src="data:,img2">
  <source id=s4 media=all srcset="data:,s4">
  <source id=s5 media=all srcset="data:,s5">
  <source       media=all srcset="data:,s6">
</picture>
<script>

const picture = document.querySelector("picture");
const img1 = document.getElementById("img1");
const img2 = document.getElementById("img2");
const source1 = document.getElementById("s1");
const source2 = document.getElementById("s2");
const source3 = document.getElementById("s3");
const source4 = document.getElementById("s4");
const source5 = document.getElementById("s5");

promise_test(async () => {
  // Wait for the relevant mutation microtask to run.
  await Promise.resolve();
  assert_equals(img1.currentSrc, "data:,s1", "[img1] First source is correctly chosen");
  assert_equals(img2.currentSrc, "data:,s1", "[img2] First source is correctly chosen");

  // Triggers a relevant mutation for *all* image children under the parent
  // `<picture>` element to update their source.
  source1.remove();
  await Promise.resolve();
  assert_equals(img1.currentSrc, "data:,s2",
      "[img1] Second source is chosen after first is removed");
  assert_equals(img2.currentSrc, "data:,s2",
      "[img2] Second source is chosen after first is removed");


  source2.remove();
  await Promise.resolve();
  assert_equals(img1.currentSrc, "data:,s3",
      "[img1] Third is chosen after first is removed");
  assert_equals(img2.currentSrc, "data:,s3",
      "[img2] Third is chosen after first is removed");


  document.body.moveBefore(source3, null);
  await Promise.resolve();
  assert_equals(img1.currentSrc, "data:,img1",
      "[img1] Img src attribute is chosen after third source is moved");
  assert_equals(img2.currentSrc, "data:,img2",
      "[img2] Img src attribute is chosen after third source is moved");


  source4.remove();
  await Promise.resolve();
  assert_equals(img1.currentSrc, "data:,img1",
      "[img1] Img src attribute is unchanged after source is removed from after the img");
  assert_equals(img2.currentSrc, "data:,img2",
      "[img2] Img src attribute is unchanged after source is removed from after the img");

  document.body.moveBefore(source5, null);
  await Promise.resolve();
  assert_equals(img1.currentSrc, "data:,img1",
      "[img1] Img src attribute is unchanged after source is moved from after the img");
  assert_equals(img2.currentSrc, "data:,img2",
      "[img2] Img src attribute is unchanged after source is moved from after the img");

  picture.prepend(source2);
  await Promise.resolve();
  assert_equals(img1.currentSrc, "data:,s2",
      "[img1] Second source is chosen after inserted before the img element");
  assert_equals(img2.currentSrc, "data:,s2",
      "[img2] Second source is chosen after inserted before the img element");

  picture.prepend(source1);
  await Promise.resolve();
  assert_equals(img1.currentSrc, "data:,s1",
      "[img1] First source is chosen after inserted before the img element");
  assert_equals(img2.currentSrc, "data:,s1",
      "[img2] First source is chosen after inserted before the img element");
}, "Neither the removing, moving, nor insertion steps for the source element " +
   "track the previous 'next sibling' pointer when triggering relevant " +
   "mutations for remaining child image elements of its old parent");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “data:,s1” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 240,
        "byte_start": 198,
        "col": 3,
        "line": 6
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “data:,s2” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 285,
        "byte_start": 243,
        "col": 3,
        "line": 7
      }
    },
    {
      "category": "Html",
      "code": "html.picture.source.media_all.disallowed",
      "message": "Value of “media” attribute here must not be “all”.",
      "severity": "Warning",
      "span": {
        "byte_end": 240,
        "byte_start": 198,
        "col": 3,
        "line": 6
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “data:,s3” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 330,
        "byte_start": 288,
        "col": 3,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.picture.source.media_all.disallowed",
      "message": "Value of “media” attribute here must not be “all”.",
      "severity": "Warning",
      "span": {
        "byte_end": 285,
        "byte_start": 243,
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
        "byte_end": 363,
        "byte_start": 333,
        "col": 3,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 396,
        "byte_start": 366,
        "col": 3,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.picture.child.img.disallowed_multiple",
      "message": "Element “img” not allowed as child of “picture” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 396,
        "byte_start": 366,
        "col": 3,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.picture.child.source.disallowed_after_img",
      "message": "Element “source” not allowed as child of “picture” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 441,
        "byte_start": 399,
        "col": 3,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “data:,s4” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 441,
        "byte_start": 399,
        "col": 3,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.picture.source.media_all.disallowed",
      "message": "Value of “media” attribute here must not be “all”.",
      "severity": "Warning",
      "span": {
        "byte_end": 330,
        "byte_start": 288,
        "col": 3,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.picture.child.source.disallowed_after_img",
      "message": "Element “source” not allowed as child of “picture” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 486,
        "byte_start": 444,
        "col": 3,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “data:,s5” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 486,
        "byte_start": 444,
        "col": 3,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.picture.source.media_all.disallowed",
      "message": "Value of “media” attribute here must not be “all”.",
      "severity": "Warning",
      "span": {
        "byte_end": 441,
        "byte_start": 399,
        "col": 3,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.picture.child.source.disallowed_after_img",
      "message": "Element “source” not allowed as child of “picture” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 531,
        "byte_start": 489,
        "col": 3,
        "line": 13
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “data:,s6” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 531,
        "byte_start": 489,
        "col": 3,
        "line": 13
      }
    },
    {
      "category": "Html",
      "code": "html.picture.source.media_all.disallowed",
      "message": "Value of “media” attribute here must not be “all”.",
      "severity": "Warning",
      "span": {
        "byte_end": 486,
        "byte_start": 444,
        "col": 3,
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
  "source_name": "html/semantics/embedded-content/the-img-element/source-relevant-mutations.html"
}
```
