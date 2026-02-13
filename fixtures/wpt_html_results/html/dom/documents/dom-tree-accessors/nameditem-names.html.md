# html/dom/documents/dom-tree-accessors/nameditem-names.html

Counts:
- errors: 2
- warnings: 15
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/documents/dom-tree-accessors/nameditem-names.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>Named items: supported property names</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-document-nameditem">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<embed name="exposed_embed">
  <embed name="not_exposed_embed">
  </embed>
</embed>
<form name="form">
</form>
<iframe name="iframe">
</iframe>
<img name="img">
<object name="exposed_object_with_name">
  <object name="not_exposed_object_with_name">
  </object>
</object>
<object id="exposed_object_with_id">
  <object id="not_exposed_object_with_id">
  </object>
</object>
<img name="img_with_id" id="img_id">
<img id="img_with_just_id">
<template id="template">
  <img name="img_in_template">
</template>
<img name="42">
<script>
var names = Object.getOwnPropertyNames(document);

test(function() {
  assert_true(names.includes("exposed_embed"))
}, "An embed name appears in a document's property names if the embed is exposed.");

test(function() {
  assert_false(names.includes("not_exposed_embed"))
}, "An embed name does not appears in a document's property names if the embed is inside another embed.");

test(function() {
  assert_true(names.includes("form"))
}, "A form name appears in a document's property names.");

test(function() {
  assert_true(names.includes("iframe"))
}, "An iframe name appears in a document's property names.");

test(function() {
  assert_true(names.includes("img"))
}, "An img name appears in a document's property names when the img has no id.");

test(function() {
  assert_true(names.includes("exposed_object_with_name"))
}, "An object name appears in a document's property names if the object is exposed.");

test(function() {
  assert_true(names.includes("exposed_object_with_id"))
}, "An object id appears in a document's property names if the object is exposed.");

test(function() {
  assert_false(names.includes("not_exposed_object_with_name"))
}, "An object name does not appear in a document's property names if the object is inside another object.");

test(function() {
  assert_false(names.includes("not_exposed_object_with_id"))
}, "An object id does not appear in a document's property names if the object is inside another object.");

test(function() {
  assert_true(names.includes("img_with_id"))
}, "An img name appears in a document's property names when the img has an id.");

test(function() {
  assert_true(names.includes("img_id"))
}, "An img id appears in a document's property names when the img has a name.");

test(function() {
  assert_false(names.includes("img_with_just_id"))
}, "An img id does not appear in a document's property names when the img has no name.");

test(function() {
  assert_true(names.includes("42"))
}, "A document's property names can include integer strings.");

test(function() {
  assert_false(names.includes("template"))
}, "A template name does not appear in a document's property names.");

test(function() {
  assert_false(names.includes("img_in_template"))
}, "An img name does not appear in a document's property names when the img is in a template's document fragment.");

test(function() {
  var form_index = names.indexOf("form");
  assert_equals(names.indexOf("iframe"), form_index + 1);
  assert_equals(names.indexOf("img"), form_index + 2);
  assert_greater_than(names.indexOf("img_id"), names.indexOf("img"));
  assert_greater_than(names.indexOf("42"), names.indexOf("img_id"));
}, "A document's property names appear in tree order.");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “embed”.",
      "severity": "Error",
      "span": {
        "byte_end": 379,
        "byte_start": 371,
        "col": 3,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “embed”.",
      "severity": "Error",
      "span": {
        "byte_end": 388,
        "byte_start": 380,
        "col": 1,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 465,
        "byte_start": 449,
        "col": 1,
        "line": 16
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 465,
        "byte_start": 449,
        "col": 1,
        "line": 16
      }
    },
    {
      "category": "Html",
      "code": "html.object.data.missing",
      "message": "Element “object” is missing required attribute “data”.",
      "severity": "Warning",
      "span": {
        "byte_end": 506,
        "byte_start": 466,
        "col": 1,
        "line": 17
      }
    },
    {
      "category": "Html",
      "code": "html.object.data.missing",
      "message": "Element “object” is missing required attribute “data”.",
      "severity": "Warning",
      "span": {
        "byte_end": 553,
        "byte_start": 509,
        "col": 3,
        "line": 18
      }
    },
    {
      "category": "Html",
      "code": "html.object.data.missing",
      "message": "Element “object” is missing required attribute “data”.",
      "severity": "Warning",
      "span": {
        "byte_end": 612,
        "byte_start": 576,
        "col": 1,
        "line": 21
      }
    },
    {
      "category": "Html",
      "code": "html.object.data.missing",
      "message": "Element “object” is missing required attribute “data”.",
      "severity": "Warning",
      "span": {
        "byte_end": 655,
        "byte_start": 615,
        "col": 3,
        "line": 22
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 714,
        "byte_start": 678,
        "col": 1,
        "line": 25
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 714,
        "byte_start": 678,
        "col": 1,
        "line": 25
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 742,
        "byte_start": 715,
        "col": 1,
        "line": 26
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 742,
        "byte_start": 715,
        "col": 1,
        "line": 26
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 798,
        "byte_start": 770,
        "col": 3,
        "line": 28
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 798,
        "byte_start": 770,
        "col": 3,
        "line": 28
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 826,
        "byte_start": 811,
        "col": 1,
        "line": 30
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 826,
        "byte_start": 811,
        "col": 1,
        "line": 30
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
  "source_name": "html/dom/documents/dom-tree-accessors/nameditem-names.html"
}
```
