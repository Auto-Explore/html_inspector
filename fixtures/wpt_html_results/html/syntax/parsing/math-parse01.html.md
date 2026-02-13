# html/syntax/parsing/math-parse01.html

Counts:
- errors: 0
- warnings: 5
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/parsing/math-parse01.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<title>math in html: parsing</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
</head>
<body>
<h1>math in html: parsing</h1>

<div id="log" style="display:block"></div>

<div style="display:none">
<div><math id="m1"><mtext/></math></div>
<div id="d1"><math><mrow/><mi/></math></div>
<div id="d2"><math><mrow><mrow><mn>1</mn></mrow><mi>a</mi></mrow></math></div>
<div id="d3">&lang;&rang;</div>
<div id="d4">&Kopf;</div>
<div id="d5"><math><semantics><mi>a</mi><annotation-xml><foo/><bar/></annotation-xml></semantics></math></div>
<div id="d6"><math><semantics><mi>a</mi><annotation-xml encoding="text/html"><div></div></annotation-xml></semantics><mn/></math>
</div>


<script>

test(function() {
assert_equals(document.getElementById("m1"),document.getElementsByTagName("math")[0]);
},"The id attribute should be recognised on math elements");

test(function() {
assert_equals(document.getElementById("d1").firstChild.nodeName,"math")
},"The node name should be math");

test(function() {
assert_equals(document.getElementById("d1").firstChild.namespaceURI ,"http://www.w3.org/1998/Math/MathML")
},"math should be in MathML Namespace");

test(function() {
assert_equals(document.getElementById("d1").firstChild.childNodes.length ,2)
},"Math has 2 children (empty tag syntax)");

test(function() {
assert_equals(document.getElementById("d2").firstChild.childNodes.length ,1)
},"Nested mrow elements should be parsed correctly");

test(function() {
assert_equals(document.getElementById("d3").firstChild.nodeValue ,"\u27E8\u27E9")
},"Testing rang and lang entity code points");

test(function() {
assert_equals(document.getElementById("d4").firstChild.nodeValue ,"\uD835\uDD42")
},"Testing Kopf (Plane 1) entity code point");

test(function() {
assert_equals(document.getElementById("d5").firstChild.firstChild.childNodes[1].childNodes.length ,2)
},"Empty element tags in annotation-xml parsed as per XML.");

test(function() {
assert_equals(document.getElementById("d6").firstChild.childNodes.length ,2)
},"html tags allowed in annotation-xml/@encoding='text/html'.");

</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “foo” not allowed as child of “annotation-xml” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 576,
        "byte_start": 570,
        "col": 57,
        "line": 19
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “foo” element is a completely-unknown element that is not allowed anywhere in any MathML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 576,
        "byte_start": 570,
        "col": 57,
        "line": 19
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “bar” not allowed as child of “annotation-xml” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 582,
        "byte_start": 576,
        "col": 63,
        "line": 19
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “bar” element is a completely-unknown element that is not allowed anywhere in any MathML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 582,
        "byte_start": 576,
        "col": 63,
        "line": 19
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
  "source_name": "html/syntax/parsing/math-parse01.html"
}
```
