# html/dom/documents/dom-tree-accessors/nameditem-06.html

Counts:
- errors: 1
- warnings: 27
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/documents/dom-tree-accessors/nameditem-06.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>Named items: imgs</title>
<link rel="author" title="Ms2ger" href="mailto:ms2ger@gmail.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-document-nameditem">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<div id="test">
<img name=test1>

<img name=test2>
<img name=test2>

<img id=test3>

<img id=test4>
<img id=test4 name="">

<img name=test5>
<img id=test5>

<img id=test6>
<img name=test6>

<img name="test7">

<img>

<img name="test9">
</div>
<script>
test(function() {
  var img = document.getElementsByTagName("img")[0];
  assert_equals(img.name, "test1");

  assert_true("test1" in document, '"test1" in document should be true');
  assert_equals(document.test1, img);
}, "If there is one img, it should be returned (name)");

test(function() {
  var img1 = document.getElementsByTagName("img")[1];
  assert_equals(img1.name, "test2");
  var img2 = document.getElementsByTagName("img")[2];
  assert_equals(img2.name, "test2");

  assert_true("test2" in document, '"test2" in document should be true');
  var collection = document.test2;
  assert_class_string(collection, "HTMLCollection", "collection should be an HTMLCollection");
  assert_array_equals(collection, [img1, img2]);
}, "If there are two imgs, a collection should be returned. (name)");

test(function() {
  var img = document.getElementsByTagName("img")[3];
  assert_equals(img.id, "test3");

  assert_false("test3" in document, '"test3" in document should be false');
  assert_equals(document.test3, undefined);
}, "If there is one img, it should not be returned (id)");

test(function() {
  var img1 = document.getElementsByTagName("img")[4];
  assert_equals(img1.id, "test4");
  var img2 = document.getElementsByTagName("img")[5];
  assert_equals(img2.id, "test4");

  assert_false("test4" in document, '"test4" in document should be false');
  assert_equals(document.test4, undefined);
}, "If there are two imgs, nothing should be returned. (id)");

test(function() {
  var img1 = document.getElementsByTagName("img")[6];
  assert_equals(img1.name, "test5");
  var img2 = document.getElementsByTagName("img")[7];
  assert_equals(img2.id, "test5");

  assert_true("test5" in document, '"test5" in document should be true');
  assert_equals(document.test5, img1);
}, "If there are two imgs, the one with a name should be returned. (name and id)");

test(function() {
  var img1 = document.getElementsByTagName("img")[8];
  assert_equals(img1.id, "test6");
  var img2 = document.getElementsByTagName("img")[9];
  assert_equals(img2.name, "test6");

  assert_true("test6" in document, '"test6" in document should be true');
  assert_equals(document.test6, img2);
}, "If there are two imgs, the one with a name should be returned. (id and name)");

test(function() {
  var img = document.getElementsByTagName("img")[10];
  assert_equals(img.name, "test7");

  assert_true("test7" in document, 'test7 in document should be true');
  assert_equals(document["test7"], img);
  assert_equals(document.test7, img);

  img.removeAttribute("name");
  assert_false("test7" in document, 'test7 in document should be false');
  assert_equals(document["test7"], undefined);
  assert_equals(document.test7, undefined);
}, "Dynamically removing the name attribute from img elements, should not be accessible.");

test(function() {
  var img = document.getElementsByTagName("img")[11];
  img.setAttribute("name", "test8a");

  assert_true("test8a" in document, 'test8a in document should be true');
  assert_equals(document["test8a"], img);
  assert_equals(document.test8a, img);

  img.setAttribute("name", "test8b");
  assert_false("test8a" in document, 'test8a in document should be false');
  assert_equals(document["test8a"], undefined);
  assert_equals(document.test8a, undefined);
  assert_true("test8b" in document, 'test8b in document should be true');
  assert_equals(document["test8b"], img);
  assert_equals(document.test8b, img);
}, "Dynamically updating the name attribute from img elements, should be accessible by its name.");

test(function() {
  var img = document.getElementsByTagName("img")[12];
  assert_equals(img.name, "test9");

  assert_true("test9" in document, 'test9 in document should be true');
  assert_equals(document["test9"], img);
  assert_equals(document.test9, img);

  img.remove();
  assert_false("test9" in document, 'test9 in document should be false');
  assert_equals(document["test9"], undefined);
  assert_equals(document.test9, undefined);
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
        "byte_end": 383,
        "byte_start": 367,
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
        "byte_end": 383,
        "byte_start": 367,
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
        "byte_end": 401,
        "byte_start": 385,
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
        "byte_end": 401,
        "byte_start": 385,
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
        "byte_end": 418,
        "byte_start": 402,
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
        "byte_end": 418,
        "byte_start": 402,
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
        "byte_end": 434,
        "byte_start": 420,
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
        "byte_end": 434,
        "byte_start": 420,
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
        "byte_end": 450,
        "byte_start": 436,
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
        "byte_end": 450,
        "byte_start": 436,
        "col": 1,
        "line": 17
      }
    },
    {
      "category": "Html",
      "code": "html.id.duplicate",
      "message": "Duplicate ID “test4”.",
      "severity": "Error",
      "span": {
        "byte_end": 473,
        "byte_start": 451,
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
        "byte_end": 473,
        "byte_start": 451,
        "col": 1,
        "line": 18
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 473,
        "byte_start": 451,
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
        "byte_end": 491,
        "byte_start": 475,
        "col": 1,
        "line": 20
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 491,
        "byte_start": 475,
        "col": 1,
        "line": 20
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 506,
        "byte_start": 492,
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
        "byte_end": 506,
        "byte_start": 492,
        "col": 1,
        "line": 21
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 522,
        "byte_start": 508,
        "col": 1,
        "line": 23
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 522,
        "byte_start": 508,
        "col": 1,
        "line": 23
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 539,
        "byte_start": 523,
        "col": 1,
        "line": 24
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 539,
        "byte_start": 523,
        "col": 1,
        "line": 24
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 559,
        "byte_start": 541,
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
        "byte_end": 559,
        "byte_start": 541,
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
        "byte_end": 566,
        "byte_start": 561,
        "col": 1,
        "line": 28
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 566,
        "byte_start": 561,
        "col": 1,
        "line": 28
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 586,
        "byte_start": 568,
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
        "byte_end": 586,
        "byte_start": 568,
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
  "source_name": "html/dom/documents/dom-tree-accessors/nameditem-06.html"
}
```
