# html/dom/documents/dom-tree-accessors/nameditem-08.html

Counts:
- errors: 2
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/documents/dom-tree-accessors/nameditem-08.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>Named items: duplicate id attributes for object and img</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/dom.html#dom-document-nameditem">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<div id="test">
<div id=test2></div>
<object id=test2></object>

<div id=test3></div>
<img id=test3 name=non-empty>
</div>
<script>
test(function() {
  var object = document.querySelector("object");
  assert_equals(object.id, "test2");

  assert_true("test2" in document);
  assert_equals(document.test2, object);
}, "If there is a div and object with same id, the object should be returned");

test(function() {
  var img = document.querySelector("img");
  assert_equals(img.id, "test3");

  assert_true("test3" in document);
  assert_equals(document.test3, img);
}, "If there is a div and img with same id, the img should be returned");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.id.duplicate",
      "message": "Duplicate ID “test2”.",
      "severity": "Error",
      "span": {
        "byte_end": 385,
        "byte_start": 368,
        "col": 1,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.object.data.missing",
      "message": "Element “object” is missing required attribute “data”.",
      "severity": "Warning",
      "span": {
        "byte_end": 385,
        "byte_start": 368,
        "col": 1,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.id.duplicate",
      "message": "Duplicate ID “test3”.",
      "severity": "Error",
      "span": {
        "byte_end": 446,
        "byte_start": 417,
        "col": 1,
        "line": 13
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 446,
        "byte_start": 417,
        "col": 1,
        "line": 13
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 446,
        "byte_start": 417,
        "col": 1,
        "line": 13
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
  "source_name": "html/dom/documents/dom-tree-accessors/nameditem-08.html"
}
```
