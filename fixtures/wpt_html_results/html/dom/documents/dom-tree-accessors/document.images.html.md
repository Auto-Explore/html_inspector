# html/dom/documents/dom-tree-accessors/document.images.html

Counts:
- errors: 1
- warnings: 25
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/documents/dom-tree-accessors/document.images.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>Document.images</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<div id=log></div>
<div id=test>
<img>
<img id=x><img name=y><img id=z1 name=z2>
<img id=a><img id=a>
<img name=b><img name=b>
<img id=><img name=>
<input type=image name=input>
</div>
<script>
function assert_all(aAssertFunc, aCollection) {
  for (var i = 0; i < aCollection.length; ++i) {
    aAssertFunc(aCollection[i]);
  }
}

var XHTML = "http://www.w3.org/1999/xhtml";
var div, images, c;

setup(function() {
  div = document.getElementById("test");
  var foreign =
    div.appendChild(document.createElementNS("http://example.org", "img"));
  foreign.setAttribute("id", "f");

  images = [].slice.call(div.getElementsByTagNameNS(XHTML, "img"));

  c = document.images;
});

test(function() {
  assert_equals(c.length, 10);
  assert_array_equals(c, images);

  assert_all(function (aElement) {
    assert_equals(aElement.namespaceURI, XHTML);
  }, c);
}, "document.images should contain all HTML img elements");

test(function() {
  assert_equals(c.x, images[1]);
  assert_equals(c.namedItem("x"), images[1]);
  assert_true("x" in c, '"x" in c');
}, "img with id");

test(function() {
  assert_equals(c.y, images[2]);
  assert_equals(c.namedItem("y"), images[2]);
  assert_true("y" in c, '"y" in c');
}, "img with name");

test(function() {
  assert_equals(c.z1, images[3]);
  assert_equals(c.namedItem("z1"), images[3]);
  assert_true("z1" in c, '"z1" in c');
  assert_equals(c.z2, images[3]);
  assert_equals(c.namedItem("z2"), images[3]);
  assert_true("z2" in c, '"z2" in c');
}, "img with id and name");

test(function() {
  assert_equals(c.a, images[4]);
  assert_equals(c.namedItem("a"), images[4]);
  assert_true("a" in c, '"a" in c');
}, "Two img elements with the same id");

test(function() {
  assert_equals(c.b, images[6]);
  assert_equals(c.namedItem("b"), images[6]);
  assert_true("b" in c, '"b" in c');
}, "Two img elements with the same name");

test(function() {
  assert_equals(c.c, undefined);
  assert_equals(c.namedItem("c"), null);
  assert_false("c" in c, '"c" in c');
}, "Unknown name should not be in the collection");

test(function() {
  assert_equals(c.f, undefined);
  assert_equals(c.namedItem("f"), null);
  assert_false("f" in c, '"f" in c');
}, "Foreign element should not be in the collection");

test(function() {
  assert_equals(c.input, undefined);
  assert_equals(c.namedItem("input"), null);
  assert_false("input" in c, '"input" in c');
  var input = div.getElementsByTagName("input")[0];
  assert_all(function (aElement) {
    assert_not_equals(aElement.namespaceURI, input);
  }, c);
}, "Input elements should not be in the collection");

test(function() {
  assert_equals(c[""], undefined);
  assert_equals(c.namedItem(""), null);
  assert_false("" in c, '"" in c');
}, "The empty string should not be in the collections");

test(function() {
  var div = document.getElementById("test");
  var imgs = document.images;
  assert_true(imgs instanceof HTMLCollection);
  assert_equals(imgs.length, 10);

  var img = document.createElement("img");
  div.appendChild(img);
  assert_equals(imgs.length, 11);

  div.removeChild(img);
  assert_equals(imgs.length, 10);
}, "Document.images should be a live collection");
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
        "byte_end": 208,
        "byte_start": 203,
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
        "byte_end": 208,
        "byte_start": 203,
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
        "byte_end": 219,
        "byte_start": 209,
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
        "byte_end": 219,
        "byte_start": 209,
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
        "byte_end": 231,
        "byte_start": 219,
        "col": 11,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 231,
        "byte_start": 219,
        "col": 11,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 250,
        "byte_start": 231,
        "col": 23,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 250,
        "byte_start": 231,
        "col": 23,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 261,
        "byte_start": 251,
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
        "byte_end": 261,
        "byte_start": 251,
        "col": 1,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.id.duplicate",
      "message": "Duplicate ID “a”.",
      "severity": "Error",
      "span": {
        "byte_end": 271,
        "byte_start": 261,
        "col": 11,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 271,
        "byte_start": 261,
        "col": 11,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 271,
        "byte_start": 261,
        "col": 11,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 284,
        "byte_start": 272,
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
        "byte_end": 284,
        "byte_start": 272,
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
        "byte_end": 296,
        "byte_start": 284,
        "col": 13,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 296,
        "byte_start": 284,
        "col": 13,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.attr_value_missing",
      "message": "Attribute value missing.",
      "severity": "Warning",
      "span": {
        "byte_end": 306,
        "byte_start": 297,
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
        "byte_end": 306,
        "byte_start": 297,
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
        "byte_end": 306,
        "byte_start": 297,
        "col": 1,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.id.invalid",
      "message": "Bad value “” for attribute “id” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 306,
        "byte_start": 297,
        "col": 1,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.attr_value_missing",
      "message": "Attribute value missing.",
      "severity": "Warning",
      "span": {
        "byte_end": 317,
        "byte_start": 306,
        "col": 10,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 317,
        "byte_start": 306,
        "col": 10,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 317,
        "byte_start": 306,
        "col": 10,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.input.image.alt.missing",
      "message": "Element “input” is missing required attribute “alt”.",
      "severity": "Warning",
      "span": {
        "byte_end": 347,
        "byte_start": 318,
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
  "source_name": "html/dom/documents/dom-tree-accessors/document.images.html"
}
```
