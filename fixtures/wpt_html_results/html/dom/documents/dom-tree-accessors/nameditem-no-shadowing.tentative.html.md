# html/dom/documents/dom-tree-accessors/nameditem-no-shadowing.tentative.html

Counts:
- errors: 0
- warnings: 17
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/documents/dom-tree-accessors/nameditem-no-shadowing.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>Named items: property names don't shadow</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-document-nameditem">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<!-- interface Document -->
<img name="constructor">
<img name="currentScript">
<img name="forms">
<img name="onreadystatechange">

<!-- interface Node -->
<img name="firstChild">
<img name="isSameNode">

<!-- Object.prototype -->
<img name="__proto__">

<img name="foobar">

<script id="this-script">
test(function() {
  assert_equals(document.constructor, HTMLDocument);
}, "document.constructor is not shadowed");

test(function() {
  assert_equals(document.currentScript, document.getElementById("this-script"));
}, "document.currentScript is not shadowed");

test(function() {
  assert_true(document.forms instanceof HTMLCollection);
  assert_equals(document.forms.length, 0);
}, "document.forms is not shadowed");

test(function() {
  assert_equals(document.onreadystatechange, null);
}, "document.onreadystatechange is not shadowed");

test(function() {
  assert_true(document.firstChild instanceof DocumentType);
  assert_equals(document.firstChild, document.childNodes[0]);
}, "document.firstChild is not shadowed");

test(function() {
  assert_true(document.isSameNode instanceof Function);
  assert_true(document.isSameNode(document));
}, "document.isSameNode() is not shadowed");

test(function() {
  assert_equals(document.__proto__, HTMLDocument.prototype);
}, "document.__proto__ is not shadowed");

test(function() {
  assert_true(document.foobar instanceof HTMLImageElement);
}, "document.foobar works (sanity check)");
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
        "byte_end": 340,
        "byte_start": 316,
        "col": 1,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 340,
        "byte_start": 316,
        "col": 1,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 367,
        "byte_start": 341,
        "col": 1,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 367,
        "byte_start": 341,
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
        "byte_end": 386,
        "byte_start": 368,
        "col": 1,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 386,
        "byte_start": 368,
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
        "byte_end": 418,
        "byte_start": 387,
        "col": 1,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 418,
        "byte_start": 387,
        "col": 1,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 467,
        "byte_start": 444,
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
        "byte_end": 467,
        "byte_start": 444,
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
        "byte_end": 491,
        "byte_start": 468,
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
        "byte_end": 491,
        "byte_start": 468,
        "col": 1,
        "line": 16
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 541,
        "byte_start": 519,
        "col": 1,
        "line": 19
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 541,
        "byte_start": 519,
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
        "byte_end": 562,
        "byte_start": 543,
        "col": 1,
        "line": 21
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 562,
        "byte_start": 543,
        "col": 1,
        "line": 21
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
  "source_name": "html/dom/documents/dom-tree-accessors/nameditem-no-shadowing.tentative.html"
}
```
