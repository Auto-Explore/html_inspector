# html/dom/documents/dom-tree-accessors/nameditem-01.html

Counts:
- errors: 1
- warnings: 15
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/documents/dom-tree-accessors/nameditem-01.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>Named items: img id &amp; name</title>
<link rel="author" title="Ms2ger" href="mailto:ms2ger@gmail.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-document-nameditem">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<div id="test">
<img id="a" name="b">
<img id="test" name="test">
<img id="test2a" name="test2b">
<img id="test3a" name="test3b">
<img id="test4a" name="test4b">
<img id="test5a" name="test5b">
<img id="test6a" name="test6b">
</div>
<script>
test(function() {
  let img = document.getElementsByTagName("img")[0];
  assert_equals(document.a, img);
  assert_equals(document['a'], img);
  assert_equals(document.b, img);
  assert_equals(document['b'], img);
}, "img elements that have a name and id attribute, should be accessible by both values.");

test(function() {
  let img = document.getElementsByTagName("img")[1];
  assert_equals(document.test, img);
  assert_equals(document['test'], img);
}, "img elements that have a name and id attribute with same value.");

test(function() {
  let img = document.getElementsByTagName("img")[2];
  assert_equals(document.test2a, img);
  assert_equals(document['test2a'], img);
  assert_equals(document.test2b, img);
  assert_equals(document['test2b'], img);

  img.removeAttribute("name");
  assert_equals(document.test2a, undefined);
  assert_equals(document['test2a'], undefined);
  assert_equals(document.test2b, undefined);
  assert_equals(document['test2b'], undefined);
}, "Dynamically removing the name attribute from img elements, should not be accessible.");

test(function() {
  let img = document.getElementsByTagName("img")[3];
  assert_equals(document.test3a, img);
  assert_equals(document['test3a'], img);
  assert_equals(document.test3b, img);
  assert_equals(document['test3b'], img);

  img.removeAttribute("id");
  assert_equals(document.test3a, undefined);
  assert_equals(document['test3a'], undefined);
  assert_equals(document.test3b, img);
  assert_equals(document['test3b'], img);
}, "Dynamically removing the id attribute from img elements, should still be accessible by name value.");

test(function() {
  let img = document.getElementsByTagName("img")[4];
  assert_equals(document.test4a, img);
  assert_equals(document['test4a'], img);
  assert_equals(document.test4b, img);
  assert_equals(document['test4b'], img);

  img.name = 'test4a';
  assert_equals(document.test4a, img);
  assert_equals(document['test4a'], img);
  assert_equals(document.test4b, undefined);
  assert_equals(document['test4b'], undefined);

  img.name = 'test4c';
  assert_equals(document.test4a, img);
  assert_equals(document['test4a'], img);
  assert_equals(document.test4b, undefined);
  assert_equals(document['test4b'], undefined);
  assert_equals(document.test4c, img);
  assert_equals(document['test4c'], img);
}, "Dynamically updating the name attribute from img elements, should be accessible by values.");

test(function() {
  let img = document.getElementsByTagName("img")[5];
  assert_equals(document.test5a, img);
  assert_equals(document['test5a'], img);
  assert_equals(document.test5b, img);
  assert_equals(document['test5b'], img);

  img.id = 'test5b';
  assert_equals(document.test5a, undefined);
  assert_equals(document['test5a'], undefined);
  assert_equals(document.test5b, img);
  assert_equals(document['test5b'], img);

  img.id = 'test5c';
  assert_equals(document.test5a, undefined);
  assert_equals(document['test5a'], undefined);
  assert_equals(document.test5b, img);
  assert_equals(document['test5b'], img);
  assert_equals(document.test5c, img);
  assert_equals(document['test5c'], img);
}, "Dynamically updating the id attribute from img elements, should be accessible by values.");

test(function() {
  let img = document.getElementsByTagName("img")[6];
  assert_equals(document.test6a, img);
  assert_equals(document['test6a'], img);
  assert_equals(document.test6b, img);
  assert_equals(document['test6b'], img);

  img.remove();
  assert_equals(document.test6a, undefined);
  assert_equals(document['test6a'], undefined);
  assert_equals(document.test6b, undefined);
  assert_equals(document['test6b'], undefined);
}, "img elements that is removed, should not be accessible.");
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
        "byte_end": 401,
        "byte_start": 380,
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
        "byte_end": 401,
        "byte_start": 380,
        "col": 1,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.id.duplicate",
      "message": "Duplicate ID “test”.",
      "severity": "Error",
      "span": {
        "byte_end": 429,
        "byte_start": 402,
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
        "byte_end": 429,
        "byte_start": 402,
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
        "byte_end": 429,
        "byte_start": 402,
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
        "byte_end": 461,
        "byte_start": 430,
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
        "byte_end": 461,
        "byte_start": 430,
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
        "byte_end": 493,
        "byte_start": 462,
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
        "byte_end": 493,
        "byte_start": 462,
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
        "byte_end": 525,
        "byte_start": 494,
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
        "byte_end": 525,
        "byte_start": 494,
        "col": 1,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 557,
        "byte_start": 526,
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
        "byte_end": 557,
        "byte_start": 526,
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
        "byte_end": 589,
        "byte_start": 558,
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
        "byte_end": 589,
        "byte_start": 558,
        "col": 1,
        "line": 16
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
  "source_name": "html/dom/documents/dom-tree-accessors/nameditem-01.html"
}
```
