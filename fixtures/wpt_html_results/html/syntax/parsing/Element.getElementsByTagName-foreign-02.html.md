# html/syntax/parsing/Element.getElementsByTagName-foreign-02.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/parsing/Element.getElementsByTagName-foreign-02.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>getElementsByTagName and font</title>
<link rel="author" title="Ms2ger" href="mailto:ms2ger@gmail.com">
<link rel="help" href="https://dom.spec.whatwg.org/#dom-element-getelementsbytagname">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#parsing">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<div id="test">
<svg id="outer">
<foreignObject>
<font></font>
<svg><font/></svg>
</foreignObject>
</svg>
</div>
<script>
var HTML = "http://www.w3.org/1999/xhtml", SVG = "http://www.w3.org/2000/svg";
var wrapper = document.getElementById("outer");
test(function() {
  assert_equals(wrapper.getElementsByTagName("FONT").length, 1);
  assert_equals(wrapper.getElementsByTagName("FONT")[0].namespaceURI, HTML);
}, "Upper-case font")
test(function() {
  assert_equals(wrapper.getElementsByTagName("font").length, 2);
  assert_equals(wrapper.getElementsByTagName("font")[0].namespaceURI, HTML);
  assert_equals(wrapper.getElementsByTagName("font")[1].namespaceURI, SVG);
}, "Lower-case font")
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.element.font.obsolete",
      "message": "The “font” element is obsolete. Use CSS instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 469,
        "byte_start": 463,
        "col": 1,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.svg.element.font.missing_horiz_adv_x",
      "message": "Element “font” is missing required attribute “horiz-adv-x”.",
      "severity": "Warning",
      "span": {
        "byte_end": 489,
        "byte_start": 482,
        "col": 6,
        "line": 13
      }
    },
    {
      "category": "Html",
      "code": "html.svg.element.font.missing_missing_glyph",
      "message": "Element “font” is missing a required instance of child element “missing-glyph”.",
      "severity": "Warning",
      "span": {
        "byte_end": 489,
        "byte_start": 482,
        "col": 6,
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
  "source_name": "html/syntax/parsing/Element.getElementsByTagName-foreign-02.html"
}
```
