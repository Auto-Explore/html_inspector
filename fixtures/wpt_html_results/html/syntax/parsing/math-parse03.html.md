# html/syntax/parsing/math-parse03.html

Counts:
- errors: 1
- warnings: 29
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/parsing/math-parse03.html",
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

<div id="log"></div>

<div>
<div><MATH id="m1"><Mtext/></math></div>
<div id="d1"><math><MI MATHVARIANT="BOLD" /></math></div>
<div id="d2"><math><semantics DEFINITIONurl="www.example.org/FOO"><mi>a</mi><annotation-xml><foo/><bar/></annotation-xml></semantics></math></div>
<div><math id="m3span-mtext"><mtext><Span>x</Span></mtext></math></div>
<div><math id="m3span-mi"><mi><Span>x</Span></mi></math></div>
<div><math id="m3span-mrow"><mi><Span>x</Span></mrow></math></div>
<div><math id="m3p-mtext"><mtext><P>x</P></mtext></math></div>
<div><math id="m3p-mi"><mi><P>x</P></mi></math></div>
<div id="d3p-mrow"><math><mrow><P>x</P><mi>y</mi></mrow></math></div>
<div><math id="m4"><mtext><Undefinedelement>x</Undefinedelement></mtext></math></div>
<div><math id="m5"><mtext><mi>x</mi></mtext></math></div>
<div><math><semantics><mi>x</mi>
                      <annotation-xml><p id="p6default">x</p></annotation-xml>
           </semantics></math></div>
<div><math><semantics><mi>x</mi>
                      <annotation-xml encoding=text/html><p id="p6texthtml">x</p></annotation-xml>
           </semantics></math></div>
<div><math><semantics><mi>x</mi>
                      <annotation-xml encoding=TEXT/HTML><p id="p6uctexthtml">x</p></annotation-xml>
           </semantics></math></div>
<div><math><semantics><mi>x</mi>
                      <annotation-xml encoding=application/xhtml+xml><p id="p6applicationxhtmlxml">x</p></annotation-xml>
           </semantics></math></div>
<div><math><semantics><mi>x</mi>
                      <annotation-xml encoding=foo><p id="p6foo">x</p></annotation-xml>
           </semantics></math></div>
</div>
<script>
test(function() {
assert_equals(document.getElementById("m1"),document.getElementsByTagName("math")[0]);
},"MATH element name should be lowercased");

test(function() {
assert_equals(document.getElementById("d1").firstChild.firstChild.nodeName,"mi");
assert_equals(document.getElementById("d1").firstChild.firstChild.namespaceURI, "http://www.w3.org/1998/Math/MathML");
assert_true(document.getElementById("d1").firstChild.firstChild.hasAttribute("mathvariant"));
assert_equals(document.getElementById("d1").firstChild.firstChild.getAttribute("mathvariant"),"BOLD")
},"MI element name and mathvariant attribute name should be lowercased, attribute value unchanged");

test(function() {
assert_true(document.getElementById("d2").firstChild.firstChild.hasAttribute("definitionURL"));
assert_equals(document.getElementById("d2").firstChild.firstChild.getAttribute("definitionURL"),"www.example.org/FOO")
},"DEFINITIONurl attribute markup should produce a definitionURL attribute, attribute value unchanged");

test(function() {
assert_equals(document.getElementById("m3span-mtext").firstChild.firstChild.nodeName,"SPAN");
assert_equals(document.getElementById("m3span-mtext").firstChild.firstChild.namespaceURI,"http://www.w3.org/1999/xhtml")
},"html Span in mtext produces SPAN nodename in XHTML namespace");

test(function() {
assert_equals(document.getElementById("m3span-mi").firstChild.firstChild.nodeName,"SPAN");
assert_equals(document.getElementById("m3span-mi").firstChild.firstChild.namespaceURI,"http://www.w3.org/1999/xhtml")
},"html Span in mi produces SPAN nodename in XHTML namespace");

test(function() {
assert_equals(document.getElementById("m3span-mrow").firstChild.firstChild.nodeName,"SPAN");
assert_equals(document.getElementById("m3span-mrow").firstChild.firstChild.namespaceURI,"http://www.w3.org/1999/xhtml")
},"html Span in mrow produces SPAN nodename in XHTML namespace");

test(function() {
assert_equals(document.getElementById("m3p-mtext").firstChild.firstChild.nodeName,"P");
assert_equals(document.getElementById("m3p-mtext").firstChild.firstChild.namespaceURI,"http://www.w3.org/1999/xhtml")
},"html P in mtext produces P nodename in XHTML namespace");

test(function() {
assert_equals(document.getElementById("m3p-mi").firstChild.firstChild.nodeName,"P");
assert_equals(document.getElementById("m3p-mi").firstChild.firstChild.namespaceURI,"http://www.w3.org/1999/xhtml")
},"html P in mi produces P nodename in XHTML namespace");

test(function() {
assert_equals(document.getElementById("d3p-mrow").childNodes.length ,3)
},"html P in mrow terminates the math: mrow,P,MI children of div");

test(function() {
assert_equals(document.getElementById("d3p-mrow").firstChild.childNodes.length ,1)
},"html P in mrow terminates the math: mrow child of math");

test(function() {
assert_equals(document.getElementById("d3p-mrow").firstChild.firstChild.childNodes.length ,0)
},"html P in mrow terminates the math: mrow empty");

test(function() {
assert_equals(document.getElementById("d3p-mrow").childNodes[0].nodeName,"math");
assert_equals(document.getElementById("d3p-mrow").childNodes[1].nodeName,"P");
assert_equals(document.getElementById("d3p-mrow").childNodes[2].nodeName,"MI");
},"html P in mrow terminates the math: math,P,MI children of div");

test(function() {
assert_equals(document.getElementById("m4").firstChild.firstChild.nodeName,"UNDEFINEDELEMENT");
assert_equals(document.getElementById("m4").firstChild.firstChild.namespaceURI,"http://www.w3.org/1999/xhtml")
},"Undefinedelement in mtext produces UNDEFINEDELEMENT nodename in XHTML namespace");

test(function() {
assert_equals(document.getElementById("m5").firstChild.firstChild.nodeName,"MI");
assert_equals(document.getElementById("m5").firstChild.firstChild.namespaceURI,"http://www.w3.org/1999/xhtml")
},"mi in mtext produces MI nodename in XHTML namespace");

test(function() {
assert_equals(document.getElementById("p6default").parentNode.nodeName,"DIV")
},"p in annotation-xml moves to be child of DIV");

test(function() {
assert_equals(document.getElementById("p6texthtml").parentNode.nodeName,"annotation-xml")
},"p in annotation-xml encoding=text/html stays as child of annotation-xml");

test(function() {
assert_equals(document.getElementById("p6uctexthtml").parentNode.nodeName,"annotation-xml")
},"p in annotation-xml encoding=TEXT/HTML stays as child of annotation-xml");

test(function() {
assert_equals(document.getElementById("p6applicationxhtmlxml").parentNode.nodeName,"annotation-xml")
},"p in annotation-xml encoding=application/xhtml+xml stays as child of annotation-xml");

test(function() {
assert_equals(document.getElementById("p6foo").parentNode.nodeName,"DIV")
},"p in annotation-xml encoding=foo moves to be child of DIV");
</script>
</body>
</html>
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
        "byte_end": 445,
        "byte_start": 439,
        "col": 93,
        "line": 16
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “foo” element is a completely-unknown element that is not allowed anywhere in any MathML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 445,
        "byte_start": 439,
        "col": 93,
        "line": 16
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “bar” not allowed as child of “annotation-xml” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 451,
        "byte_start": 445,
        "col": 99,
        "line": 16
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “bar” element is a completely-unknown element that is not allowed anywhere in any MathML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 451,
        "byte_start": 445,
        "col": 99,
        "line": 16
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “span” not allowed as child of “mtext” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 536,
        "byte_start": 530,
        "col": 37,
        "line": 17
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “span” element is a completely-unknown element that is not allowed anywhere in any MathML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 536,
        "byte_start": 530,
        "col": 37,
        "line": 17
      }
    },
    {
      "category": "Html",
      "code": "html.mathml.html_start_tag_in_foreign_namespace",
      "message": "HTML start tag “span” in a foreign namespace context.",
      "severity": "Warning",
      "span": {
        "byte_end": 536,
        "byte_start": 530,
        "col": 37,
        "line": 17
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “mrow”.",
      "severity": "Error",
      "span": {
        "byte_end": 682,
        "byte_start": 675,
        "col": 47,
        "line": 19
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “span” not allowed as child of “mi” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 602,
        "byte_start": 596,
        "col": 31,
        "line": 18
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “span” element is a completely-unknown element that is not allowed anywhere in any MathML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 602,
        "byte_start": 596,
        "col": 31,
        "line": 18
      }
    },
    {
      "category": "Html",
      "code": "html.mathml.html_start_tag_in_foreign_namespace",
      "message": "HTML start tag “span” in a foreign namespace context.",
      "severity": "Warning",
      "span": {
        "byte_end": 602,
        "byte_start": 596,
        "col": 31,
        "line": 18
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “span” not allowed as child of “mi” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 667,
        "byte_start": 661,
        "col": 33,
        "line": 19
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “span” element is a completely-unknown element that is not allowed anywhere in any MathML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 667,
        "byte_start": 661,
        "col": 33,
        "line": 19
      }
    },
    {
      "category": "Html",
      "code": "html.mathml.html_start_tag_in_foreign_namespace",
      "message": "HTML start tag “span” in a foreign namespace context.",
      "severity": "Warning",
      "span": {
        "byte_end": 667,
        "byte_start": 661,
        "col": 33,
        "line": 19
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “p” not allowed as child of “mtext” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 732,
        "byte_start": 729,
        "col": 34,
        "line": 20
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “p” element is a completely-unknown element that is not allowed anywhere in any MathML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 732,
        "byte_start": 729,
        "col": 34,
        "line": 20
      }
    },
    {
      "category": "Html",
      "code": "html.mathml.html_start_tag_in_foreign_namespace",
      "message": "HTML start tag “p” in a foreign namespace context.",
      "severity": "Warning",
      "span": {
        "byte_end": 732,
        "byte_start": 729,
        "col": 34,
        "line": 20
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “p” not allowed as child of “mi” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 789,
        "byte_start": 786,
        "col": 28,
        "line": 21
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “p” element is a completely-unknown element that is not allowed anywhere in any MathML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 789,
        "byte_start": 786,
        "col": 28,
        "line": 21
      }
    },
    {
      "category": "Html",
      "code": "html.mathml.html_start_tag_in_foreign_namespace",
      "message": "HTML start tag “p” in a foreign namespace context.",
      "severity": "Warning",
      "span": {
        "byte_end": 789,
        "byte_start": 786,
        "col": 28,
        "line": 21
      }
    },
    {
      "category": "Html",
      "code": "html.mathml.html_start_tag_in_foreign_namespace",
      "message": "HTML start tag “p” in a foreign namespace context.",
      "severity": "Warning",
      "span": {
        "byte_end": 847,
        "byte_start": 844,
        "col": 32,
        "line": 22
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “mi” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 856,
        "byte_start": 852,
        "col": 40,
        "line": 22
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “mi” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 856,
        "byte_start": 852,
        "col": 40,
        "line": 22
      }
    },
    {
      "category": "Html",
      "code": "html.mathml.element.outside_math",
      "message": "Element “mi” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 856,
        "byte_start": 852,
        "col": 40,
        "line": 22
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “undefinedelement” not allowed as child of “mtext” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 927,
        "byte_start": 909,
        "col": 27,
        "line": 23
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “undefinedelement” element is a completely-unknown element that is not allowed anywhere in any MathML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 927,
        "byte_start": 909,
        "col": 27,
        "line": 23
      }
    },
    {
      "category": "Html",
      "code": "html.mathml.html_start_tag_in_foreign_namespace",
      "message": "HTML start tag “undefinedelement” in a foreign namespace context.",
      "severity": "Warning",
      "span": {
        "byte_end": 927,
        "byte_start": 909,
        "col": 27,
        "line": 23
      }
    },
    {
      "category": "Html",
      "code": "html.mathml.html_start_tag_in_foreign_namespace",
      "message": "HTML start tag “p” in a foreign namespace context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1116,
        "byte_start": 1098,
        "col": 39,
        "line": 26
      }
    },
    {
      "category": "Html",
      "code": "html.mathml.html_start_tag_in_foreign_namespace",
      "message": "HTML start tag “p” in a foreign namespace context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1806,
        "byte_start": 1792,
        "col": 52,
        "line": 38
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
  "source_name": "html/syntax/parsing/math-parse03.html"
}
```
